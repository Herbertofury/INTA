using Inta.RepairQueue;
using System.Security.Cryptography;

static void Must(bool condition, string message) { if (!condition) throw new Exception(message); }
static void Reject(Action action, string message)
{
    try { action(); } catch (ArgumentException) { return; }
    throw new Exception(message);
}

const string Request = "synthetic-case-75";
var device = new DeviceIdentity("offline-test-device", "pairing-v3");
var other = new DeviceIdentity("not-this-device", "pairing-v3");
var old = device with { PairingEpoch = "pairing-v2" };
var now = DateTimeOffset.FromUnixTimeSeconds(1_790_000_000).AddMilliseconds(250);
string Hash(char c) => new(c, 64);
CheckupRecipe Recipe(string check, string target, string catalog, int order, string? resource = null) =>
    new(check, target, catalog, "v3", Hash('a'), resource ?? target, order, TimeSpan.FromMinutes(5), true, CheckupDisruption.None);
CheckupFinding Found(string id, CheckupRecipe rule, int age = 100) =>
    new(id, device, rule.CheckId, rule.TargetId, CheckupObservation.VerifiedIssue, now.AddSeconds(-age), "synthetic-observer-v1",
        Hash('b'), CheckupPolicy.Permitted, CheckupRollback.Available);
CheckupDecision Decision(CheckupPreview p, string id) => p.Decisions.Single(x => x.FindingId == id);

var early = Recipe("expected-setting", "setting-x", "compliance:setting-x", 10);
var late = Recipe("failed-security-update", "patch-y", "compliance:patch-y", 20);
var alias = early with { CheckId = "same-setting-alias" };
var unavailableRollback = Recipe("inventory-version", "app-z", "compliance:app-z", 30);
var restartRule = Recipe("restart-only", "service-z", "compliance:service-z", 40) with { Disruption = CheckupDisruption.Restart };
var noRecipe = Recipe("not-reviewed", "unreviewed", "compliance:never-add", 0);
var staleRule = Recipe("stale", "stale", "compliance:stale", 0);
var unknownRule = Recipe("unknown", "unknown", "compliance:unknown", 0);
var futureRule = Recipe("future", "future", "compliance:future", 0);
var policyRule = Recipe("policy", "policy", "compliance:policy", 0);
var clearedRule = Recipe("cleared", "cleared", "compliance:cleared", 0);
var ageBoundary = Recipe("boundary", "boundary", "compliance:boundary", 0);
var planner = new SecurityCheckupPlanner([early, late, alias, restartRule, unavailableRollback,
    staleRule, unknownRule, futureRule, policyRule, clearedRule, ageBoundary]);
var first = Found("alpha", early, 120);
var duplicate = Found("beta", alias, 80);
var findings = new CheckupFinding[]
{
    Found("later", late), duplicate, first, first,
    Found("stale", staleRule, 301), Found("unknown", unknownRule) with { Observation = CheckupObservation.Unknown },
    Found("future", futureRule, -1), Found("unreviewed", noRecipe),
    Found("policy", policyRule) with { Policy = CheckupPolicy.ManagedConflict },
    Found("rollback", unavailableRollback) with { Rollback = CheckupRollback.Unavailable },
    Found("restart", restartRule), Found("clear", clearedRule) with { Observation = CheckupObservation.NoIssue },
    Found("boundary", ageBoundary, 299) with { ObservedAt = now.AddMinutes(-5).AddMilliseconds(200) },
    Found("old-pair", early) with { Device = old }, Found("other-pc", late) with { Device = other },
    first with { Device = other, ObservedStateSha256 = Hash('d') }
};

var all = planner.Preview(device, findings, now, Request);
var reordered = planner.Preview(device, findings.Reverse(), now, Request);
var plan = all.Plan!;
Must(plan.Steps.Count == 2 && plan.Device == device && all.Steps[0].Recipe == early, "safe ordered plan");
Must(plan.Steps.Select(x => x.CatalogId).SequenceEqual([early.CatalogId, late.CatalogId]), "reviewed order");
Must(plan.Steps.All(x => x.Kind == RepairKind.VerifiedFix && x.Effect == Effect.ChangesDevice && x.ContentSha256 == Hash('a')),
    "only the reviewed artifact hash reaches the existing VerifiedFix queue");
Must(plan.ExpiresAt == DateTimeOffset.FromUnixTimeSeconds(now.ToUnixTimeSeconds() + 180), "expiry is whole seconds and bounded by oldest admitted observation");
Must(plan.Digest == reordered.Plan!.Digest && all.Decisions.SequenceEqual(reordered.Decisions), "stable under input order");
Must(Decision(all, "alpha") == new CheckupDecision("alpha", CheckupDisposition.Included, 0)
    && Decision(all, "beta") == new CheckupDecision("beta", CheckupDisposition.MergedIntoStep, 0)
    && all.Steps[0].Observations.Count == 2, "deduplicated with honest preview");
foreach (var (id, reason) in new[]
{
    ("stale", CheckupDisposition.StaleEvidence), ("boundary", CheckupDisposition.StaleEvidence),
    ("unknown", CheckupDisposition.UnknownEvidence), ("future", CheckupDisposition.FutureEvidence),
    ("unreviewed", CheckupDisposition.NoReviewedRecipe), ("policy", CheckupDisposition.ManagedPolicyConflict),
    ("rollback", CheckupDisposition.RollbackUnavailable), ("restart", CheckupDisposition.SeparateDisruptionReview),
    ("clear", CheckupDisposition.NoIssue), ("old-pair", CheckupDisposition.WrongDevice), ("other-pc", CheckupDisposition.WrongDevice)
})
{
    var one = planner.Preview(device, findings, now, Request, id);
    Must(Decision(all, id).Disposition == reason && Decision(one, id).Disposition == reason && one.Plan is null,
        $"Fix One cannot bypass the Fix All exclusion for {id}");
}
Must(Decision(all, "restart").Disruption == CheckupDisruption.Restart, "restart remains visible and separate");
var aliasOne = planner.Preview(device, findings, now, Request, "beta");
Must(aliasOne.Plan!.Steps.SequenceEqual(plan.Steps.Take(1))
    && Decision(aliasOne, "beta").Disposition == CheckupDisposition.Included
    && Decision(aliasOne, "later").Disposition == CheckupDisposition.NotSelected, "Fix One shares exact recipe and duplicate scope");
Must(planner.Preview(device, findings, now, Request, "not-present").Plan is null, "a stale selection cannot fall through to bulk");

var untrusted = new[]
{
    first with { Policy = CheckupPolicy.Unknown }, first with { ObservedStateSha256 = "not-a-hash" },
    first with { EvidenceSourceId = null }, first with { ObservedAt = null }, first with { Rollback = CheckupRollback.Unknown },
    first with { CheckId = "unreviewed-check" }, first with { TargetId = "unreviewed-target" },
    first with { Observation = (CheckupObservation)200 }
};
foreach (var unsupported in untrusted)
    Must(planner.Preview(device, [unsupported], now, Request).Plan is null, "missing or unreviewed input never auto-admitted");

var conflicts = new[]
{
    duplicate with { ObservedStateSha256 = Hash('c') },
    duplicate with { Policy = CheckupPolicy.ManagedConflict },
    duplicate with { Observation = CheckupObservation.NoIssue },
    duplicate with { Id = first.Id }
};
foreach (var conflict in conflicts)
{
    Must(planner.Preview(device, [first, conflict], now, Request).Plan is null, "conflicting observations are manual");
    Must(planner.Preview(device, [first, conflict], now, Request, first.Id).Plan is null, "selecting one does not erase the conflict");
}

var otherRepair = Recipe("other-recipe", "another-exact-target", "compliance:related", 3, early.ResourceId);
var competing = new SecurityCheckupPlanner([early, otherRepair]);
var competitor = Found("competitor", otherRepair);
foreach (var contender in new[] { competitor, competitor with { Policy = CheckupPolicy.ManagedConflict } })
{
    var conflict = competing.Preview(device, [first, contender], now, Request, "alpha");
    Must(conflict.Plan is null && Decision(conflict, "alpha").Disposition == CheckupDisposition.ConflictingRecipe,
        "competing recipes on the same reviewed resource never arbitrate a write");
}
Must(competing.Preview(device, [first, competitor with { Observation = CheckupObservation.NoIssue }], now, Request).Plan!.Steps.Count == 1,
    "a known healthy, different recipe does not veto the unrelated eligible proposal");

var rules = Enumerable.Range(0, 25).Select(i => Recipe($"big-{i}", $"target-{i}", $"catalog-{i}", i)).ToArray();
var large = new SecurityCheckupPlanner(rules);
var source = rules.Select((recipe, i) => Found($"finding-{i}", recipe)).ToArray();
var oversized = large.Preview(device, source, now, Request);
Must(oversized.Plan is null && oversized.Decisions.All(x => x.Disposition == CheckupDisposition.PlanLimit),
    "no silently truncated or split signed plan");
Must(large.Preview(device, source.Take(24), now, Request).Plan!.Steps.Count == 24, "honor the existing plan's exact capacity");
Must(large.Preview(device, source, now, Request, "finding-7").Plan!.Steps.Count == 1, "allow a reviewed explicit single action");
var many = Enumerable.Range(0, 10_000).Select(i => first with { Id = $"synthetic-{i}" }).ToArray();
var manyPreview = planner.Preview(device, many, now, Request);
Must(manyPreview.Plan!.Steps.Count == 1 && manyPreview.Steps[0].Observations.Count == many.Length
    && manyPreview.Decisions.Count(x => x.Disposition == CheckupDisposition.Included) == 1, "large dedup fixture produces one exact step");

using var key = ECDsa.Create(ECCurve.NamedCurves.nistP256);
var verifier = new SignedPlanApprovalVerifier("synthetic-tenant", "synthetic-key", key.ExportSubjectPublicKeyInfo());
var approval = new ApprovalStatement("synthetic-tenant", "synthetic-key", "synthetic-tech", device.DeviceId,
    device.PairingEpoch, plan.Digest, plan.ExpiresAt.ToUnixTimeSeconds());
var signature = key.SignData(approval.CanonicalBytes(), HashAlgorithmName.SHA256, DSASignatureFormat.IeeeP1363FixedFieldConcatenation);
Must(verifier.Verify(plan, approval, signature, now).Plan.Digest == plan.Digest, "existing signer can review this exact draft");
foreach (var modification in new[]
{
    planner.Preview(device, [first with { ObservedStateSha256 = Hash('d') }, duplicate with { ObservedStateSha256 = Hash('d') }, findings[0]], now, Request),
    planner.Preview(device, [first with { EvidenceSourceId = "changed-evidence-source" }, duplicate, findings[0]], now, Request),
    planner.Preview(device, [first with { ObservedAt = first.ObservedAt!.Value.AddTicks(1) }, duplicate, findings[0]], now, Request),
    planner.Preview(device, [first, duplicate, findings[0]], now, "different-request"),
    planner.Preview(old, [first with { Device = old }, duplicate with { Device = old }, findings[0] with { Device = old }], now, Request)
})
{
    Must(modification.Plan!.Digest != plan.Digest, "changed precondition, source, instant, request or pairing changes signed identity");
    Reject(() => verifier.Verify(modification.Plan!, approval, signature, now), "changed draft cannot reuse the earlier approval");
}

Reject(() => new SecurityCheckupPlanner([early, early with { Version = "v4" }]), "multiple versions of one reviewed operation cannot silently win");
Reject(() => new SecurityCheckupPlanner([early with { ContentSha256 = "arbitrary" }]), "unversioned content cannot enter the reviewed catalog");
Reject(() => planner.Preview(device, findings, now, "not an opaque ID"), "invalid context cannot generate a plan");
Console.WriteLine("Security Checkup pure preview: exact device, shared eligibility, stable order, dedup, refusals, evidence-bound signed draft, and 10,000-row fixture passed. No device action executed.");
