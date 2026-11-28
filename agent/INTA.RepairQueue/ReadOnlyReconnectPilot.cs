namespace Inta.RepairQueue;

public sealed record DiagnosticSnapshot(string Status, int ItemCount);
public sealed record DiagnosticAttempt(QueueState State, bool Claimed, DiagnosticSnapshot? Observation);

/// Local test worker for fixed read-only inventory actions. No server, pairing enrollment,
/// shell, download, elevation, or dispatch of system changes lives here.
public sealed class ReadOnlyReconnectPilot
{
    private const string PowerCatalogId = "powercfg:power-plans";
    private const string StartupCatalogId = "inta:startup-registration-count";
    private const string TargetId = "this-device";
    private readonly string reviewedExecutableHash;
    private readonly Func<DiagnosticSnapshot> readNativePowerPlans;
    private readonly Func<DiagnosticSnapshot>? countNativeStartupRegistrations;

    public ReadOnlyReconnectPilot(string reviewedExecutableHash, Func<DiagnosticSnapshot> trustedAdapter,
        Func<DiagnosticSnapshot>? trustedStartupCounter = null)
    {
        if (reviewedExecutableHash.Length != 64 || reviewedExecutableHash.Any(c => c is not (>= '0' and <= '9') and not (>= 'a' and <= 'f')))
            throw new ArgumentException("An exact reviewed local adapter pin is required");
        this.reviewedExecutableHash = reviewedExecutableHash;
        this.readNativePowerPlans = trustedAdapter;
        countNativeStartupRegistrations = trustedStartupCounter;
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
        bool bound = expected.Kind == RepairKind.ReadOnlyDiagnostic && expected.Effect == Effect.ReadOnly
            && expected.SelectedTarget == TargetId && expected.ContentSha256 == reviewedExecutableHash;
        Func<DiagnosticSnapshot>? adapter = bound ? expected.CatalogId switch
        {
            PowerCatalogId => readNativePowerPlans,
            StartupCatalogId => countNativeStartupRegistrations,
            _ => null,
        } : null;
        // A missing, changed or unbound adapter becomes NeedsReview; it cannot run.
        StepLease? lease = journal.Claim(device, now, adapter is not null ? reviewedExecutableHash : "");
        if (lease is null || adapter is null)
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
            var value = adapter();
            int maximum = expected.CatalogId == PowerCatalogId ? 256 : 6144;
            if (value.Status is "ready" or "incomplete" or "unsupported" or "unavailable" or "unstable"
                && value.ItemCount >= 0 && value.ItemCount <= maximum)
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
