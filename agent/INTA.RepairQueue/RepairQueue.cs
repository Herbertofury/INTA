using System.Globalization;
using System.Security.Cryptography;
using System.Text;
using System.Text.RegularExpressions;

namespace Inta.RepairQueue;

public enum RepairKind { UninstallApplication = 1, VerifiedFix = 2, ReviewedPowerShell = 3, MalwareScan = 4, Optimize = 5, ReadOnlyDiagnostic = 6 }
public enum Effect { ReadOnly = 1, ChangesDevice = 2 }
public enum QueueState { Waiting = 0, Running = 1, Succeeded = 2, Failed = 3, NeedsReview = 4, Canceled = 5, Expired = 6 }
public sealed record DeviceIdentity(string DeviceId, string PairingEpoch);
public sealed record RepairStep(RepairKind Kind, Effect Effect, string CatalogId, string SelectedTarget, string ContentSha256);
public sealed record PlanApproval(string TechnicianId, DeviceIdentity Device, string PlanDigest);
public sealed record StepLease(string JobId, Guid Token, int Index, RepairStep Step, DateTimeOffset Deadline);

/// The remote boundary must authenticate device identity and approval before constructing a ledger.
public sealed class RepairPlan
{
    private static readonly Regex Identifier = new(@"\A[A-Za-z0-9][A-Za-z0-9._:-]{0,95}\z", RegexOptions.CultureInvariant | RegexOptions.NonBacktracking);
    private static readonly Regex Hash = new(@"\A[0-9a-f]{64}\z", RegexOptions.CultureInvariant | RegexOptions.NonBacktracking);
    public string Id { get; }
    public DeviceIdentity Device { get; }
    public DateTimeOffset ExpiresAt { get; }
    public IReadOnlyList<RepairStep> Steps { get; }
    public string Digest { get; }

    public RepairPlan(string id, DeviceIdentity device, DateTimeOffset expiresAt, IReadOnlyList<RepairStep> steps)
    {
        if (!ValidIdentifier(id) || !ValidIdentifier(device.DeviceId) || !ValidIdentifier(device.PairingEpoch)
                || steps.Count is < 1 or > 24 || expiresAt != DateTimeOffset.FromUnixTimeSeconds(expiresAt.ToUnixTimeSeconds()))
            throw new ArgumentException("Invalid plan identity, count, or expiration");
        Id = id;
        Device = device;
        ExpiresAt = expiresAt.ToUniversalTime();
        RepairStep[] copy = steps.ToArray();
        foreach (RepairStep step in copy)
        {
            if (!Enum.IsDefined(step.Kind) || !Enum.IsDefined(step.Effect) || !ValidIdentifier(step.CatalogId)
                    || !ValidIdentifier(step.SelectedTarget) || !Hash.IsMatch(step.ContentSha256)
                    || step.Effect != (step.Kind is RepairKind.MalwareScan or RepairKind.ReadOnlyDiagnostic ? Effect.ReadOnly : Effect.ChangesDevice))
                throw new ArgumentException("Step must pin a reviewed operation and its effect");
        }
        Steps = Array.AsReadOnly(copy);
        // Identifiers reject control characters. Numeric protocol tags are explicit and do not use locale.
        var lines = new List<string> { "inta-reconnect-plan:v1", Id, Device.DeviceId, Device.PairingEpoch,
            Number(ExpiresAt.ToUnixTimeSeconds()), Number(copy.Length) };
        foreach (RepairStep step in copy)
        {
            lines.Add(Number((int)step.Kind)); lines.Add(Number((int)step.Effect));
            lines.Add(step.CatalogId); lines.Add(step.SelectedTarget); lines.Add(step.ContentSha256);
        }
        Digest = Convert.ToHexStringLower(SHA256.HashData(Encoding.UTF8.GetBytes(string.Join('\n', lines) + "\n")));
    }

    internal static bool ValidIdentifier(string value) => Identifier.IsMatch(value);
    private static string Number(long value) => value.ToString(CultureInfo.InvariantCulture);
}

/// Pure single-process queue model. A production host needs durable atomic transitions
/// and authenticated requests; this model deliberately does not invoke tools or a shell.
public sealed class RepairLedger
{
    private readonly object gate = new();
    private readonly RepairPlan plan;
    private readonly TimeSpan leaseLength;
    private readonly DateTimeOffset approvedUntil;
    private bool stopAfterCurrent;
    private int current;
    private QueueState state = QueueState.Waiting;
    private StepLease? active;

    internal RepairLedger(RepairPlan plan, PlanApproval approval, TimeSpan leaseLength)
        : this(plan, approval, leaseLength, plan.ExpiresAt) { }

    public RepairLedger(SignedPlanApprovalVerifier.VerifiedPlanApproval approval, TimeSpan leaseLength)
        : this(approval.Plan, approval.Proof, leaseLength, approval.ExpiresAt) { }

    private RepairLedger(RepairPlan plan, PlanApproval approval, TimeSpan leaseLength, DateTimeOffset approvedUntil)
    {
        if (!RepairPlan.ValidIdentifier(approval.TechnicianId) || approval.Device != plan.Device
            || !string.Equals(approval.PlanDigest, plan.Digest, StringComparison.Ordinal)
            || leaseLength <= TimeSpan.Zero || leaseLength > TimeSpan.FromMinutes(30))
            throw new ArgumentException("Approval must refer to this exact device, pairing and plan");
        this.plan = plan;
        this.leaseLength = leaseLength;
        this.approvedUntil = approvedUntil;
    }

    internal (QueueState State, int Completed) Snapshot()
    {
        lock (gate) return (state, current);
    }

    internal void Restore(QueueState stored, int count)
    {
        lock (gate)
        {
            if (!Enum.IsDefined(stored) || count < 0 || count > plan.Steps.Count
                || (stored == QueueState.Succeeded) != (count == plan.Steps.Count))
                throw new InvalidDataException("Invalid repair journal state");
            current = count;
            // A recovered process cannot tell whether an in-flight system action already ran.
            state = stored == QueueState.Running ? QueueState.NeedsReview : stored;
        }
    }

    public (QueueState State, int Completed) Observe(DateTimeOffset now)
    {
        lock (gate) { Refresh(now); return (state, current); }
    }

    public StepLease? Claim(DeviceIdentity caller, DateTimeOffset now, string observedCatalogSha256)
    {
        lock (gate)
        {
            if (caller != plan.Device) return null;
            Refresh(now);
            if (state != QueueState.Waiting) return null;
            RepairStep step = plan.Steps[current];
            if (!string.Equals(observedCatalogSha256, step.ContentSha256, StringComparison.Ordinal))
            {
                state = QueueState.NeedsReview;
                return null;
            }
            TimeSpan available = approvedUntil - now;
            var deadline = now + (available < leaseLength ? available : leaseLength);
            active = new(plan.Id, Guid.NewGuid(), current, step, deadline);
            state = QueueState.Running;
            return active;
        }
    }

    public bool Finish(DeviceIdentity caller, Guid token, bool succeeded, DateTimeOffset now)
    {
        lock (gate)
        {
            if (caller != plan.Device) return false;
            Refresh(now);
            if (state != QueueState.Running || active?.Token != token) return false;
            active = null;
            if (!succeeded) { state = QueueState.Failed; return true; }
            current++;
            state = current == plan.Steps.Count ? QueueState.Succeeded
                : stopAfterCurrent ? QueueState.Canceled : QueueState.Waiting;
            return true;
        }
    }

    public void Cancel(DateTimeOffset now)
    {
        lock (gate)
        {
            Refresh(now);
            if (state == QueueState.Waiting) state = QueueState.Canceled;
            else if (state == QueueState.Running) stopAfterCurrent = true;
        }
    }

    private void Refresh(DateTimeOffset now)
    {
        if (state == QueueState.Running && active is { } lease && now >= lease.Deadline)
        {
            active = null;
            state = QueueState.NeedsReview;
        }
        else if (state == QueueState.Waiting && now >= approvedUntil) state = QueueState.Expired;
    }
}
