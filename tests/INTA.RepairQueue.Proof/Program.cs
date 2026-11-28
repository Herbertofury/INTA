using Inta.RepairQueue;
using System.Security.Cryptography;

static void Must(bool condition, string message) { if (!condition) throw new Exception(message); }
static void Invalid(Action action)
{
    try { action(); } catch (ArgumentException) { return; }
    throw new Exception("Expected rejection");
}

var phoneQueue = new DeviceIdentity("customer-pc-test", "pairing-v2");
var oldPairing = phoneQueue with { PairingEpoch = "pairing-v1" };
var now = DateTimeOffset.FromUnixTimeSeconds(1_790_000_000);
var scan = new RepairStep(RepairKind.MalwareScan, Effect.ReadOnly, "registered-clamav", "quick-scan", new('a', 64));
var uninstall = new RepairStep(RepairKind.UninstallApplication, Effect.ChangesDevice, "registered-uninstall", "app-id:example", new('b', 64));
var script = new RepairStep(RepairKind.ReviewedPowerShell, Effect.ChangesDevice, "signed-repair-template", "template-id:example", new('c', 64));
var steps = new RepairStep[] { scan, uninstall, script };
var plan = new RepairPlan("job-1", phoneQueue, now.AddMinutes(30), steps);
Must(plan.Digest == "5444e4707294bb02e8868ad4ce0af95b963d844796e0117f70897e9d803dd585", "approved plan digest must match the independently measured cross-client vector");
var approval = new PlanApproval("synthetic-tech", phoneQueue, plan.Digest);
RepairLedger Queue() => new(plan, approval, TimeSpan.FromMinutes(2));
var reordered = new RepairPlan("job-1", phoneQueue, plan.ExpiresAt, steps.Reverse().ToArray());
Must(plan.Digest != reordered.Digest, "step order must be approved");
Invalid(() => new RepairLedger(reordered, approval, TimeSpan.FromMinutes(2)));
Invalid(() => new RepairLedger(plan, approval with { Device = oldPairing }, TimeSpan.FromMinutes(2)));
Invalid(() => new RepairPlan("job-1", phoneQueue, plan.ExpiresAt, [script with { Effect = Effect.ReadOnly }]));
Invalid(() => new RepairPlan("job-1", phoneQueue, plan.ExpiresAt, [scan with { Effect = Effect.ChangesDevice }]));
Invalid(() => new RepairPlan("job-1", phoneQueue, plan.ExpiresAt, [script with { Kind = RepairKind.Optimize, Effect = Effect.ReadOnly }]));
Invalid(() => new RepairPlan("job-1", phoneQueue, plan.ExpiresAt, [script with { SelectedTarget = "not a command;tool" }]));
steps[0] = uninstall;
Must(plan.Steps[0] == scan, "approved input must not be mutable through the caller array");

var queue = Queue();
Must(queue.Observe(now).State == QueueState.Waiting, "offline has no work or client token");
Must(queue.Claim(oldPairing, now, scan.ContentSha256) == null, "old pairing must not claim");
var leases = Enumerable.Range(0, 12).AsParallel().Select(_ => queue.Claim(phoneQueue, now, scan.ContentSha256)).OfType<StepLease>().ToArray();
Must(leases.Length == 1 && leases[0].Index == 0, "exactly one claim for scan, even on replay");
Must(!queue.Finish(oldPairing, leases[0].Token, true, now), "old pairing cannot finish");
Must(queue.Finish(phoneQueue, leases[0].Token, true, now), "scan receipt advances");
Must(!queue.Finish(phoneQueue, leases[0].Token, true, now), "replayed receipt is not a completion");
var second = queue.Claim(phoneQueue, now, uninstall.ContentSha256)!;
Must(second.Index == 1 && second.Step.SelectedTarget == "app-id:example", "ordered pinned uninstall");
queue.Cancel(now);
Must(queue.Claim(phoneQueue, now, script.ContentSha256) == null, "cancel cannot claim another step");
Must(queue.Finish(phoneQueue, second.Token, true, now) && queue.Observe(now) == (QueueState.Canceled, 2), "cancel stops after acknowledged active step");

var ambiguous = Queue();
var uncertain = ambiguous.Claim(phoneQueue, now, scan.ContentSha256)!;
Must(ambiguous.Claim(phoneQueue, uncertain.Deadline, scan.ContentSha256) == null
     && ambiguous.Observe(uncertain.Deadline) == (QueueState.NeedsReview, 0)
     && !ambiguous.Finish(phoneQueue, uncertain.Token, true, uncertain.Deadline), "expired lease cannot be repeated or silently acknowledged");
var miss = Queue();
Must(miss.Claim(phoneQueue, now, uninstall.ContentSha256) == null
    && miss.Observe(now).State == QueueState.NeedsReview, "changed pinned package must stop");
var failed = Queue();
var failedScan = failed.Claim(phoneQueue, now, scan.ContentSha256)!;
Must(failed.Finish(phoneQueue, failedScan.Token, false, now) && failed.Observe(now) == (QueueState.Failed, 0)
     && failed.Claim(phoneQueue, now, uninstall.ContentSha256) == null, "failure prevents later changes");
Must(Queue().Observe(plan.ExpiresAt).State == QueueState.Expired, "pending plan expires offline");
using var serviceKey = ECDsa.Create(ECCurve.NamedCurves.nistP256);
using var otherKey = ECDsa.Create(ECCurve.NamedCurves.nistP256);
var publicKey = serviceKey.ExportSubjectPublicKeyInfo();
var verifier = new SignedPlanApprovalVerifier("synthetic-tenant", "test-key-v1", publicKey);
var statement = new ApprovalStatement("synthetic-tenant", "test-key-v1", "synthetic-tech",
    plan.Device.DeviceId, plan.Device.PairingEpoch, plan.Digest, now.AddSeconds(45).ToUnixTimeSeconds());
byte[] Sign(ECDsa key, ApprovalStatement value) =>
    key.SignData(value.CanonicalBytes(), HashAlgorithmName.SHA256, DSASignatureFormat.IeeeP1363FixedFieldConcatenation);
var signature = Sign(serviceKey, statement);
var authenticated = verifier.Verify(plan, statement, signature, now);
var signedQueue = new RepairLedger(authenticated, TimeSpan.FromMinutes(2));
Must(signedQueue.Claim(phoneQueue, now, scan.ContentSha256)?.Deadline == now.AddSeconds(45), "signed approval limits the lease");
var offlineSignedQueue = new RepairLedger(authenticated, TimeSpan.FromMinutes(2));
Must(offlineSignedQueue.Claim(phoneQueue, now.AddSeconds(45), scan.ContentSha256) is null, "no execution after signed approval expires");
Invalid(() => verifier.Verify(plan, statement with { TenantId = "another-tenant" }, signature, now));
Invalid(() => verifier.Verify(plan, statement with { PairingEpoch = oldPairing.PairingEpoch }, signature, now));
Invalid(() => verifier.Verify(reordered, statement, signature, now));
Invalid(() => verifier.Verify(plan, statement, signature, now.AddSeconds(45)));
Invalid(() => verifier.Verify(plan, statement, signature[..63], now));
foreach (var checkedInput in new[] { Sign(otherKey, statement), (byte[])signature.Clone() })
{
    if (checkedInput.SequenceEqual(signature)) checkedInput[12] ^= 7;
    try { verifier.Verify(plan, statement, checkedInput, now); throw new Exception("invalid signature accepted"); }
    catch (CryptographicException) { }
}
try { verifier.Verify(plan, statement with { TechnicianId = "redacted-tech" }, signature, now); throw new Exception("modified approver accepted"); }
catch (CryptographicException) { }

var signedDirectory = Directory.CreateTempSubdirectory("inta-signed-journal-");
try
{
    using (var first = new LocalRepairJournal(signedDirectory.FullName, authenticated, TimeSpan.FromMinutes(2)))
        Must(first.Observe(now) == (QueueState.Waiting, 0), "signed journal starts pending");
    var renewed = statement with { ValidUntilUnixSeconds = now.AddMinutes(1).ToUnixTimeSeconds() };
    var differentApproval = verifier.Verify(plan, renewed, Sign(serviceKey, renewed), now);
    try
    {
        using var conflict = new LocalRepairJournal(signedDirectory.FullName, differentApproval, TimeSpan.FromMinutes(2));
        throw new Exception("journal resumed under a different approval");
    }
    catch (InvalidDataException) { }
    using var original = new LocalRepairJournal(signedDirectory.FullName, authenticated, TimeSpan.FromMinutes(2));
    Must(original.Claim(phoneQueue, now, scan.ContentSha256)?.Deadline == now.AddSeconds(45), "journal preserves signed expiry");
}
finally { signedDirectory.Delete(recursive: true); }

var testDirectory = Directory.CreateTempSubdirectory("inta-repair-journal-");
LocalRepairJournal OpenJournal() => new(testDirectory.FullName, plan, approval, TimeSpan.FromMinutes(2));
try
{
    using (var before = OpenJournal())
    {
        var a = before.Claim(phoneQueue, now, scan.ContentSha256)!;
        Must(before.Finish(phoneQueue, a.Token, true, now), "receipt persisted");
        try { using var duplicate = OpenJournal(); throw new Exception("second worker was admitted"); }
        catch (IOException) { }
    }
    using (var reopened = OpenJournal())
    {
        Must(reopened.Observe(now) == (QueueState.Waiting, 1), "known completion survives restart");
        Must(reopened.Claim(phoneQueue, now, uninstall.ContentSha256)?.Index == 1, "exact next step survives restart");
    }
    using (var crash = OpenJournal())
    {
        Must(crash.Observe(now) == (QueueState.NeedsReview, 1), "unacknowledged uninstall quarantined after restart");
        Must(crash.Claim(phoneQueue, now, uninstall.ContentSha256) is null, "unacknowledged operation is never replayed");
    }
    string journal = LocalRepairJournal.RecordPath(testDirectory.FullName, plan.Id);
    Must(Path.GetFileName(LocalRepairJournal.RecordPath(testDirectory.FullName, "CON:job")).All(c => c != ':'), "logical IDs must not form Windows streams");
    var corrupt = File.ReadAllBytes(journal).Append((byte)127).ToArray();
    File.WriteAllBytes(journal, corrupt);
    try { using var invalid = OpenJournal(); throw new Exception("corrupt journal accepted"); }
    catch (InvalidDataException) { Must(File.ReadAllBytes(journal).SequenceEqual(corrupt), "malformed journal bytes preserved"); }
}
finally { testDirectory.Delete(recursive: true); }
Console.WriteLine("Signed ordered reconnect queue and restart persistence passed; no system action was executed.");

var readOnlyStep = new RepairStep(RepairKind.ReadOnlyDiagnostic, Effect.ReadOnly, "powercfg:power-plans",
    "this-device", new('d', 64));
var readOnlyPlan = new RepairPlan("read-one", phoneQueue, now.AddMinutes(5), [readOnlyStep]);
var readStatement = statement with { PlanSha256 = readOnlyPlan.Digest };
var readProof = verifier.Verify(readOnlyPlan, readStatement, Sign(serviceKey, readStatement), now);
var pilotRoot = Directory.CreateTempSubdirectory("inta-read-only-pilot-");
try
{
    static DiagnosticSnapshot ReadPower()
    {
        using var result = new StringWriter();
        using var errors = new StringWriter();
        if (INTA.LocalAgent.PowerPlans.Run([], result, errors) != 0)
            throw new InvalidDataException("Power Plans returned a nonzero result");
        using var report = System.Text.Json.JsonDocument.Parse(result.ToString());
        var root = report.RootElement;
        return new DiagnosticSnapshot(root.GetProperty("status").GetString()!, root.GetProperty("plans").GetArrayLength());
    }
    var calls = 0;
    var worker = new ReadOnlyReconnectPilot(readOnlyStep.ContentSha256, () => { calls++; return ReadPower(); });
    using (var ledger = new LocalRepairJournal(pilotRoot.FullName, readProof, TimeSpan.FromMinutes(1)))
    {
        Must(!worker.RunNext(ledger, readProof, oldPairing, () => now).Claimed && calls == 0, "wrong pairing gets no handler");
        var alternative = readStatement with { ValidUntilUnixSeconds = now.AddSeconds(40).ToUnixTimeSeconds() };
        var wrongProof = verifier.Verify(readOnlyPlan, alternative, Sign(serviceKey, alternative), now);
        Must(!worker.RunNext(ledger, wrongProof, phoneQueue, () => now).Claimed && calls == 0, "wrong exact signed proof gets no handler");

        var got = worker.RunNext(ledger, readProof, phoneQueue, () => now);
        Must(got.Claimed && calls == 1 && got.Observation is not null, "one signed lease reaches the real native backend");
        if (OperatingSystem.IsWindows())
            Must(got.State == QueueState.Succeeded && got.Observation?.Status == "ready",
                 "Windows must return its actual power inventory");
        else
            Must(got.State == QueueState.Failed && got.Observation?.Status == "unsupported",
                 "a real unsupported OS result cannot pass the plan");
        Must(!worker.RunNext(ledger, readProof, phoneQueue, () => now).Claimed && calls == 1, "never repeat the job");
    }
    var other = new RepairPlan("unknown-target", phoneQueue, now.AddMinutes(5), [readOnlyStep with { SelectedTarget = "another-pc" }]);
    var otherStatement = statement with { PlanSha256 = other.Digest };
    var otherApproval = verifier.Verify(other, otherStatement, Sign(serviceKey, otherStatement), now);
    using var blocked = new LocalRepairJournal(pilotRoot.FullName, otherApproval, TimeSpan.FromMinutes(1));
    Must(worker.RunNext(blocked, otherApproval, phoneQueue, () => now).State == QueueState.NeedsReview && calls == 1,
         "an unbound target cannot run or be accepted as safe");
    Console.WriteLine("Signed local read-only reconnect pilot passed; actual backend result: " + ReadPower().Status);
}
finally { pilotRoot.Delete(recursive: true); }
