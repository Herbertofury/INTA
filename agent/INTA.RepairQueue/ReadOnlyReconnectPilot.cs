namespace Inta.RepairQueue;

public sealed record DiagnosticSnapshot(string Status, int ItemCount);
public sealed record DiagnosticAttempt(QueueState State, bool Claimed, DiagnosticSnapshot? Observation);

/// Local test worker for one reviewed inventory action. No server, pairing enrollment,
/// shell, download, elevation, or dispatch of system changes lives here.
public sealed class ReadOnlyReconnectPilot
{
    private const string CatalogId = "powercfg:power-plans";
    private const string TargetId = "this-device";
    private readonly string reviewedExecutableHash;
    private readonly Func<DiagnosticSnapshot> readNativePowerPlans;

    public ReadOnlyReconnectPilot(string reviewedExecutableHash, Func<DiagnosticSnapshot> trustedAdapter)
    {
        if (reviewedExecutableHash.Length != 64 || reviewedExecutableHash.Any(c => c is not (>= '0' and <= '9') and not (>= 'a' and <= 'f')))
            throw new ArgumentException("An exact reviewed local adapter pin is required");
        this.reviewedExecutableHash = reviewedExecutableHash;
        this.readNativePowerPlans = trustedAdapter;
    }

    public DiagnosticAttempt RunNext(LocalRepairJournal journal,
        SignedPlanApprovalVerifier.VerifiedPlanApproval approval, DeviceIdentity device, Func<DateTimeOffset> clock)
    {
        var now = clock();
        var (state, index) = journal.Observe(now);
        if (state != QueueState.Waiting || approval.Plan.Device != device || !journal.Matches(approval)
            || (uint)index >= approval.Plan.Steps.Count)
            return new DiagnosticAttempt(state, false, null);

        RepairStep expected = approval.Plan.Steps[index];
        bool supported = expected.Kind == RepairKind.ReadOnlyDiagnostic && expected.Effect == Effect.ReadOnly
            && expected.CatalogId == CatalogId && expected.SelectedTarget == TargetId
            && expected.ContentSha256 == reviewedExecutableHash;
        // A missing, changed or unbound adapter becomes NeedsReview; it cannot run.
        StepLease? lease = journal.Claim(device, now, supported ? reviewedExecutableHash : "");
        if (lease is null)
            return new DiagnosticAttempt(journal.Observe(clock()).State, false, null);

        if (lease.Step != expected || lease.Index != index || lease.JobId != approval.Plan.Id)
        {
            // A caller cannot accidentally bind a lease from a different local queue.
            journal.Finish(device, lease.Token, false, clock());
            return new DiagnosticAttempt(journal.Observe(clock()).State, true, null);
        }

        DiagnosticSnapshot? snapshot = null;
        try
        {
            var value = readNativePowerPlans();
            if (value.Status is "ready" or "incomplete" or "unsupported" or "unavailable" or "unstable"
                && value.ItemCount is >= 0 and <= 256)
                snapshot = value;
        }
        catch (Exception error) when (error is IOException or InvalidDataException or InvalidOperationException)
        {
            // Report failure without recording potentially sensitive exception text.
        }
        journal.Finish(device, lease.Token, snapshot?.Status == "ready", clock());
        return new DiagnosticAttempt(journal.Observe(clock()).State, true, snapshot);
    }
}
