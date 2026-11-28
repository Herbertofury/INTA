using System.Globalization;
using System.Security.Cryptography;
using System.Text;

namespace Inta.RepairQueue;

public enum CheckupObservation { Unknown = 0, NoIssue = 1, VerifiedIssue = 2 }
public enum CheckupPolicy { Unknown = 0, Permitted = 1, ManagedConflict = 2 }
public enum CheckupRollback { Unknown = 0, Available = 1, Unavailable = 2 }
public enum CheckupDisruption { None = 0, Restart = 1, ConnectionInterruption = 2, ServiceInterruption = 3 }
public enum CheckupDisposition
{
    Included, MergedIntoStep, NotSelected, WrongDevice, UnknownEvidence, FutureEvidence, StaleEvidence,
    NoIssue, NoReviewedRecipe, PolicyUnknown, ManagedPolicyConflict, RollbackUnavailable,
    SeparateDisruptionReview, ConflictingEvidence, ConflictingRecipe, PlanLimit
}

public sealed record CheckupFinding(string Id, DeviceIdentity Device, string CheckId, string TargetId,
    CheckupObservation Observation, DateTimeOffset? ObservedAt, string? EvidenceSourceId, string? ObservedStateSha256,
    CheckupPolicy Policy, CheckupRollback Rollback);

/// A recipe is loaded from a trusted, reviewed local catalog, not inferred from a finding.
public sealed record CheckupRecipe(string CheckId, string TargetId, string CatalogId, string Version,
    string ContentSha256, string ResourceId, int Order, TimeSpan MaximumEvidenceAge,
    bool RequiresRollback, CheckupDisruption Disruption);

public sealed record CheckupDecision(string FindingId, CheckupDisposition Disposition, int? StepIndex,
    CheckupDisruption Disruption = CheckupDisruption.None);

public sealed record CheckupPreviewStep(CheckupRecipe Recipe, IReadOnlyList<CheckupFinding> Observations);
public sealed record CheckupPreview(RepairPlan? Plan, IReadOnlyList<CheckupPreviewStep> Steps,
    IReadOnlyList<CheckupDecision> Decisions);

/// Pure draft planner: it does not authenticate enrollment, sign approval, observe a device, or run a repair.
/// The caller supplies the exact already-authenticated enrolled device and a consistent set of observations
/// for either selection mode. An approved plan still needs an executor that independently rechecks its evidence.
public sealed class SecurityCheckupPlanner
{
    private const int MaximumPlanSteps = 24; // RepairPlan v1 bound.
    private static readonly TimeSpan MaximumDraftAge = TimeSpan.FromMinutes(15);
    private readonly Dictionary<(string Check, string Target), CheckupRecipe> recipes = new();
    private readonly Dictionary<(string Catalog, string Target), CheckupRecipe> recipesByStep = new();

    public SecurityCheckupPlanner(IEnumerable<CheckupRecipe> reviewedCatalog)
    {
        ArgumentNullException.ThrowIfNull(reviewedCatalog);
        foreach (var recipe in reviewedCatalog)
        {
            ArgumentNullException.ThrowIfNull(recipe);
            if (!Identifier(recipe.CheckId) || !Identifier(recipe.TargetId) || !Identifier(recipe.CatalogId)
                || !Identifier(recipe.Version) || !Identifier(recipe.ResourceId) || !Hash(recipe.ContentSha256)
                || !Enum.IsDefined(recipe.Disruption) || recipe.Order < 0
                || recipe.MaximumEvidenceAge < TimeSpan.FromSeconds(1)
                || recipe.MaximumEvidenceAge > TimeSpan.FromDays(1))
                throw new ArgumentException("Invalid reviewed Security Checkup recipe.", nameof(reviewedCatalog));
            var step = (recipe.CatalogId, recipe.TargetId);
            if (recipesByStep.TryGetValue(step, out var existing) && (existing with { CheckId = recipe.CheckId }) != recipe)
                throw new ArgumentException("Conflicting reviews for one repair step.", nameof(reviewedCatalog));
            recipesByStep[step] = recipe;
            if (!recipes.TryAdd((recipe.CheckId, recipe.TargetId), recipe))
                throw new ArgumentException("Multiple reviewed recipes for one check and target.", nameof(reviewedCatalog));
        }
    }

    /// A null selection previews Fix All; otherwise preview Fix One. Both classify the same observations.
    /// Checkup evidence includes fingerprints of the exact normalized check target, not commands or raw output.
    public CheckupPreview Preview(DeviceIdentity enrolledDevice, IEnumerable<CheckupFinding> observations,
        DateTimeOffset now, string requestId, string? selectedFindingId = null)
    {
        ArgumentNullException.ThrowIfNull(enrolledDevice);
        ArgumentNullException.ThrowIfNull(observations);
        if (!Identifier(enrolledDevice.DeviceId) || !Identifier(enrolledDevice.PairingEpoch) || !Identifier(requestId)
            || selectedFindingId is not null && !Identifier(selectedFindingId))
            throw new ArgumentException("An exact enrolled device, pairing and request are required.");

        var input = new Dictionary<string, CheckupFinding>(StringComparer.Ordinal);
        var collidingIds = new HashSet<string>(StringComparer.Ordinal);
        var foreignIds = new HashSet<string>(StringComparer.Ordinal);
        // Every collision must still be able to block its own same-device repair step.
        var collisionSteps = new HashSet<(string Catalog, string Target)>();
        foreach (var finding in observations)
        {
            ArgumentNullException.ThrowIfNull(finding);
            if (!Identifier(finding.Id) || !Identifier(finding.CheckId) || !Identifier(finding.TargetId))
                throw new ArgumentException("Invalid observed identity.", nameof(observations));
            if (finding.Device != enrolledDevice)
            {
                foreignIds.Add(finding.Id);
                continue;
            }
            if (!input.TryAdd(finding.Id, finding) && input[finding.Id] != finding)
            {
                collidingIds.Add(finding.Id);
                foreach (var conflicting in new[] { input[finding.Id], finding })
                    if (recipes.TryGetValue((conflicting.CheckId, conflicting.TargetId), out var recipe))
                        collisionSteps.Add((recipe.CatalogId, recipe.TargetId));
            }
        }

        var decisions = new Dictionary<string, CheckupDecision>(StringComparer.Ordinal);
        foreach (var id in foreignIds)
            if (!input.ContainsKey(id)) decisions[id] = new(id, CheckupDisposition.WrongDevice, null);
        var competing = new HashSet<(string Catalog, string Target)>(collisionSteps);
        var uncertainResources = new HashSet<(string Catalog, string Target)>(collisionSteps);
        var eligible = new Dictionary<(string Catalog, string Target), List<(CheckupFinding Evidence, CheckupRecipe Recipe)>>();
        foreach (var finding in input.Values)
        {
            if (collidingIds.Contains(finding.Id))
            {
                decisions[finding.Id] = new(finding.Id, CheckupDisposition.ConflictingEvidence, null);
                continue;
            }

            recipes.TryGetValue((finding.CheckId, finding.TargetId), out var recipe);
            var disposition = Classify(finding, recipe, now);
            if (disposition != CheckupDisposition.Included)
            {
                decisions[finding.Id] = new(finding.Id, disposition, null, recipe?.Disruption ?? CheckupDisruption.None);
                // Do not let an otherwise eligible alias override a policy or evidence veto for the same step.
                if (recipe is not null)
                {
                    var veto = (recipe.CatalogId, recipe.TargetId);
                    competing.Add(veto);
                    if (disposition != CheckupDisposition.NoIssue) uncertainResources.Add(veto);
                }
                continue;
            }
            var exactRecipe = recipe!;
            var key = (exactRecipe.CatalogId, exactRecipe.TargetId);
            if (!eligible.TryGetValue(key, out var group))
                eligible[key] = group = new();
            group.Add((finding, exactRecipe));
        }

        var safe = new List<CheckupPreviewStep>();
        foreach (var (key, group) in eligible)
        {
            var first = group[0];
            if (competing.Contains(key)
                || group.Any(item => item.Evidence.ObservedStateSha256 != first.Evidence.ObservedStateSha256))
            {
                foreach (var (finding, _) in group)
                    decisions[finding.Id] = new(finding.Id, CheckupDisposition.ConflictingEvidence, null);
                continue;
            }

            // Stable representative and stored observation order regardless of input order.
            var ordered = group.OrderBy(item => item.Evidence.Id, StringComparer.Ordinal).ToArray();
            safe.Add(new(ordered[0].Recipe, Array.AsReadOnly(ordered.Select(item => item.Evidence).ToArray())));
        }

        // Different recipes cannot silently write to the same reviewed exclusive resource.
        // Include vetoed/disruptive recipes too when their same-device observations are not clear.
        var conflictedResources = eligible.Values.Select(group => group[0].Recipe)
            .Concat(uncertainResources.Select(key => recipesByStep[key]))
            .GroupBy(recipe => recipe.ResourceId, StringComparer.Ordinal)
            .Where(group => group.Select(r => (r.CatalogId, r.TargetId)).Distinct().Skip(1).Any())
            .Select(group => group.Key).ToHashSet(StringComparer.Ordinal);

        var steps = new List<CheckupPreviewStep>();
        foreach (var step in safe.OrderBy(step => step.Recipe.Order)
                     .ThenBy(step => step.Recipe.CatalogId, StringComparer.Ordinal)
                     .ThenBy(step => step.Recipe.TargetId, StringComparer.Ordinal))
        {
            if (conflictedResources.Contains(step.Recipe.ResourceId))
            {
                foreach (var finding in step.Observations)
                    decisions[finding.Id] = new(finding.Id, CheckupDisposition.ConflictingRecipe, null);
                continue;
            }
            var selected = selectedFindingId is null ? step.Observations[0]
                : step.Observations.FirstOrDefault(finding => finding.Id == selectedFindingId);
            if (selected is null)
            {
                foreach (var finding in step.Observations)
                    decisions[finding.Id] = new(finding.Id, CheckupDisposition.NotSelected, null);
                continue;
            }
            foreach (var finding in step.Observations)
                decisions[finding.Id] = new(finding.Id, finding.Id == selected.Id
                    ? CheckupDisposition.Included : CheckupDisposition.MergedIntoStep, steps.Count);
            steps.Add(step);
        }

        if (steps.Count > MaximumPlanSteps)
        {
            foreach (var decision in decisions.Values.ToArray())
                if (decision.StepIndex is not null)
                    decisions[decision.FindingId] = decision with { Disposition = CheckupDisposition.PlanLimit, StepIndex = null };
            steps.Clear();
        }

        RepairPlan? plan = null;
        if (steps.Count != 0)
        {
            var expiry = steps.SelectMany(step => step.Observations.Select(finding =>
                    WholeSecondDeadline(finding.ObservedAt!.Value, step.Recipe.MaximumEvidenceAge)))
                .Append(WholeSecondDeadline(now, MaximumDraftAge)).Min();
            var id = DraftId(requestId, steps);
            plan = new(id, enrolledDevice, expiry, steps.Select(step =>
                new RepairStep(RepairKind.VerifiedFix, Effect.ChangesDevice, step.Recipe.CatalogId,
                    step.Recipe.TargetId, step.Recipe.ContentSha256)).ToArray());
        }
        return new(plan, steps.AsReadOnly(), Array.AsReadOnly(decisions.Values
            .OrderBy(decision => decision.FindingId, StringComparer.Ordinal).ToArray()));
    }

    private static CheckupDisposition Classify(CheckupFinding finding,
        CheckupRecipe? recipe, DateTimeOffset now)
    {
        if (finding.Observation is not (CheckupObservation.VerifiedIssue or CheckupObservation.NoIssue)
            || finding.ObservedAt is not { } observed || !Identifier(finding.EvidenceSourceId)
            || !Hash(finding.ObservedStateSha256)) return CheckupDisposition.UnknownEvidence;
        if (observed > now) return CheckupDisposition.FutureEvidence;
        if (recipe is null) return CheckupDisposition.NoReviewedRecipe;
        if (WholeSecondDeadline(observed, recipe.MaximumEvidenceAge) <= now) return CheckupDisposition.StaleEvidence;
        if (finding.Observation == CheckupObservation.NoIssue) return CheckupDisposition.NoIssue;
        if (finding.Policy == CheckupPolicy.ManagedConflict) return CheckupDisposition.ManagedPolicyConflict;
        if (finding.Policy != CheckupPolicy.Permitted) return CheckupDisposition.PolicyUnknown;
        if (recipe.RequiresRollback && finding.Rollback != CheckupRollback.Available) return CheckupDisposition.RollbackUnavailable;
        if (recipe.Disruption != CheckupDisruption.None) return CheckupDisposition.SeparateDisruptionReview;
        return CheckupDisposition.Included;
    }

    private static string DraftId(string requestId, List<CheckupPreviewStep> steps)
    {
        // Existing approval signs the plan ID. Bind accepted evidence here without replacing ContentSha256,
        // which the queue reserves for the vetted recipe bytes. The versioned format is deliberately typed.
        var parts = new List<string> { "inta-security-checkup-evidence:v1", requestId, Number(steps.Count) };
        foreach (var step in steps)
        {
            var recipe = step.Recipe;
            parts.AddRange([recipe.CatalogId, recipe.TargetId, recipe.Version, recipe.ContentSha256, recipe.ResourceId,
                Number(recipe.Order), Number(recipe.MaximumEvidenceAge.Ticks), recipe.RequiresRollback ? "1" : "0",
                Number((int)recipe.Disruption), Number(step.Observations.Count)]);
            foreach (var item in step.Observations)
                parts.AddRange([item.Id, item.CheckId, item.EvidenceSourceId!, item.ObservedStateSha256!,
                    Number(item.ObservedAt!.Value.UtcTicks), Number((int)item.Policy), Number((int)item.Rollback)]);
        }
        return "checkup:" + Convert.ToHexStringLower(SHA256.HashData(Encoding.UTF8.GetBytes(string.Join('\n', parts) + "\n")));
    }

    private static string Number(long value) => value.ToString(CultureInfo.InvariantCulture);
    private static DateTimeOffset WholeSecondDeadline(DateTimeOffset value, TimeSpan duration)
    {
        long ticks = Math.Min(value.UtcTicks + duration.Ticks, DateTimeOffset.MaxValue.UtcTicks);
        return new DateTimeOffset(ticks - ticks % TimeSpan.TicksPerSecond, TimeSpan.Zero);
    }
    private static bool Identifier(string? value) => value is not null && RepairPlan.ValidIdentifier(value);
    private static bool Hash(string? value) => value is { Length: 64 }
        && value.All(character => character is >= '0' and <= '9' or >= 'a' and <= 'f');
}
