namespace INTA.LocalAgent;
public sealed record AgentStatus(string Version, string Platform, string Hostname, bool Elevated, string[] Managers, string[] Capabilities);
public sealed record InstalledApp(string Name, string? Version, string? Publisher, string Manager, string? PackageId, string Scope);
public sealed record ManagerPreview(string Manager, string Command, int ExitCode, string Output, DateTimeOffset CheckedAt);
public sealed record InventoryResult(DateTimeOffset CollectedAt, string Platform, IReadOnlyList<InstalledApp> Applications, IReadOnlyList<string> Managers, IReadOnlyList<ManagerPreview> UpdatePreviews);
public sealed record OperationRequest(string Manager, string Action, string PackageId, bool Confirmed);
public sealed record BatchRequest(IReadOnlyList<OperationRequest> Operations, bool Confirmed);
public sealed record OperationRecord(string Id, string Manager, string Action, string PackageId, string State, DateTimeOffset StartedAt, DateTimeOffset? FinishedAt, int? ExitCode, string Output, bool RebootLikely, string? Error);
public sealed record PairResponse(string Token, string Version, string Platform);
