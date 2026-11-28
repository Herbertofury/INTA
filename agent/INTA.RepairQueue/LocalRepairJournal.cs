using System.Security.Cryptography;
using System.Text;

namespace Inta.RepairQueue;

/// File-backed acceptance implementation for an already-authorized local worker.
/// The caller owns a private, non-linked directory. No public API, execution, or elevation.
public sealed class LocalRepairJournal : IDisposable
{
    private const int Magic = 0x014e5441; // container format v1
    private readonly object gate = new();
    private readonly FileStream owner;
    private readonly RepairLedger ledger;
    private readonly string path;
    private readonly byte[] digest;
    private bool blocked;

    internal LocalRepairJournal(string directory, RepairPlan plan, PlanApproval proof, TimeSpan leaseLength)
        : this(directory, plan, plan.Digest, new RepairLedger(plan, proof, leaseLength)) { }

    public LocalRepairJournal(string directory, SignedPlanApprovalVerifier.VerifiedPlanApproval approval, TimeSpan leaseLength)
        : this(directory, approval.Plan, approval.JournalBinding, new RepairLedger(approval, leaseLength)) { }

    private LocalRepairJournal(string directory, RepairPlan plan, string binding, RepairLedger checkedLedger)
    {
        ledger = checkedLedger;
        digest = Convert.FromHexString(binding);
        path = RecordPath(directory, plan.Id);
        owner = new(path + ".lock", FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None);
        try
        {
            (QueueState State, int Count) stored = (QueueState.Waiting, 0);
            try
            {
                using var input = new BinaryReader(new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read));
                if (input.ReadInt32() != Magic || !CryptographicOperations.FixedTimeEquals(input.ReadBytes(digest.Length), digest))
                    throw new InvalidDataException("Journal does not belong to this exact approval");
                stored = ((QueueState)input.ReadInt32(), input.ReadInt32());
                if (input.BaseStream.ReadByte() != -1) throw new InvalidDataException("Unexpected journal data");
            }
            catch (FileNotFoundException) { Publish(); return; }
            ledger.Restore(stored.State, stored.Count);
            if (stored.State == QueueState.Running) Publish();
        }
        catch { owner.Dispose(); throw; }
    }

    internal static string RecordPath(string directory, string id)
        => Path.Combine(directory, Convert.ToHexStringLower(SHA256.HashData(Encoding.UTF8.GetBytes(id))) + ".intajob");

    internal bool Matches(SignedPlanApprovalVerifier.VerifiedPlanApproval proof)
        => CryptographicOperations.FixedTimeEquals(digest, Convert.FromHexString(proof.JournalBinding));

    public (QueueState State, int Completed) Observe(DateTimeOffset now) => Update(() => ledger.Observe(now));

    public StepLease? Claim(DeviceIdentity device, DateTimeOffset now, string observedHash)
        => Update(() => ledger.Claim(device, now, observedHash));

    public bool Finish(DeviceIdentity device, Guid token, bool succeeded, DateTimeOffset now)
        => Update(() => ledger.Finish(device, token, succeeded, now));

    public void Cancel(DateTimeOffset now) => Update(() => { ledger.Cancel(now); return true; });

    private T Update<T>(Func<T> operation)
    {
        lock (gate)
        {
            if (blocked) throw new InvalidOperationException("Journal not writable or closed");
            var before = ledger.Snapshot();
            T result = operation();
            if (before != ledger.Snapshot())
            {
                try { Publish(); }
                catch { blocked = true; throw; }
            }
            return result;
        }
    }

    private void Publish()
    {
        string staged = path + ".stage-" + Guid.NewGuid().ToString("N");
        bool ownedStage = false;
        try
        {
            var snapshot = ledger.Snapshot();
            using (var stream = new FileStream(staged, FileMode.CreateNew, FileAccess.Write, FileShare.None))
            {
                ownedStage = true;
                using var writer = new BinaryWriter(stream);
                writer.Write(Magic); writer.Write(digest);
                writer.Write((int)snapshot.State); writer.Write(snapshot.Completed);
                writer.Flush(); stream.Flush(flushToDisk: true);
            }
            File.Move(staged, path, overwrite: true);
        }
        finally { if (ownedStage) File.Delete(staged); }
    }

    public void Dispose()
    {
        lock (gate) { blocked = true; owner.Dispose(); }
    }
}
