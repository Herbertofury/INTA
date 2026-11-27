using System.Net;
using System.Security.Cryptography;
using System.Runtime.InteropServices;
using INTA.LocalAgent;

if (args.Length > 0 && args[0] == "--storage-preview")
{
    Environment.ExitCode = StoragePreview.Run(args.AsSpan(1), Console.Out, Console.Error);
    return;
}

if (args.Length > 0 && args[0] == "--power-plans")
{
    Environment.ExitCode = PowerPlans.Run(args.AsSpan(1), Console.Out, Console.Error);
    return;
}

if (args.Length > 0 && args[0] == "--startup-inventory")
{
    Environment.ExitCode = StartupInventory.Run(args.AsSpan(1), Console.Out, Console.Error);
    return;
}

const string Version = "0.1.0";
const int Port = 47821;
var allowedOrigins = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
{
    "https://herbertofury.github.io",
    "https://intacomputerservices.com",
    "https://www.intacomputerservices.com",
    "http://localhost:5173",
    "http://127.0.0.1:5173"
};
var token = Convert.ToBase64String(RandomNumberGenerator.GetBytes(32));
var builder = WebApplication.CreateBuilder(args);
builder.Logging.ClearProviders();
builder.WebHost.ConfigureKestrel(options =>
{
    options.Listen(IPAddress.Loopback, Port);
    options.Listen(IPAddress.IPv6Loopback, Port);
});
var app = builder.Build();
var runner = new ProcessRunner();
var managers = new PackageManagers(runner);
var inventory = new Inventory(managers);
var store = new OperationStore();

app.Use(async (context, next) =>
{
    var remote = context.Connection.RemoteIpAddress;
    if (remote is null || !IPAddress.IsLoopback(remote)) { context.Response.StatusCode = 403; return; }
    var host = context.Request.Host.Host;
    if (host is not ("127.0.0.1" or "localhost" or "::1")) { context.Response.StatusCode = 403; return; }
    var origin = context.Request.Headers.Origin.ToString();
    context.Response.Headers.CacheControl = "no-store";
    if (!allowedOrigins.Contains(origin)) { context.Response.StatusCode = 403; return; }
    context.Response.Headers.AccessControlAllowOrigin = origin;
    context.Response.Headers.AccessControlAllowMethods = "GET, POST, OPTIONS";
    context.Response.Headers.AccessControlAllowHeaders = "Authorization, Content-Type";
    context.Response.Headers.AccessControlMaxAge = "600";
    context.Response.Headers.Append("Vary", "Origin");
    context.Response.Headers.Append("Access-Control-Allow-Private-Network", "true");
    if (HttpMethods.IsOptions(context.Request.Method)) { context.Response.StatusCode = 204; return; }
    if (context.Request.Path != "/api/v1/pair" && context.Request.Headers.Authorization != $"Bearer {token}")
    { context.Response.StatusCode = 401; return; }
    await next();
});

app.MapGet("/api/v1/health", () => Results.Ok(new { status = "ready", version = Version, platform = RuntimeInformation.OSDescription }));
app.MapPost("/api/v1/pair", () => Results.Ok(new PairResponse(token, Version, RuntimeInformation.OSDescription)));
app.MapGet("/api/v1/status", () => Results.Ok(new AgentStatus(Version, RuntimeInformation.OSDescription,
    Environment.MachineName, IsElevated(), managers.Detect(),
    ["inventory", "update-preview", "update", "repair", "uninstall", "operation-history"])));
app.MapGet("/api/v1/inventory", async (CancellationToken ct) => Results.Ok(await inventory.CollectAsync(ct)));
app.MapGet("/api/v1/operations", () => Results.Ok(store.Recent()));
app.MapGet("/api/v1/operations/{id}", (string id) => store.Get(id) is { } record ? Results.Ok(record) : Results.NotFound());

app.MapPost("/api/v1/operations", async (OperationRequest request) =>
{
    if (!request.Confirmed) return Results.BadRequest(new { error = "This action changes the computer. Confirm once in INTA before continuing." });
    if (request.Action is not ("update" or "update-all" or "repair" or "uninstall" or "rollback")) return Results.BadRequest(new { error = "Unsupported operation." });
    if (request.Action == "rollback" && request.Manager is not ("snap" or "windows-update")) return Results.BadRequest(new { error = "This agent version supports native rollback for Snap and exact Windows KB updates only." });
    if (request.Manager == "windows-update" && (request.Action is "update" or "update-all")) return Results.BadRequest(new { error = "Use the supported Windows Update Settings interface to check and install operating-system updates." });
    if (!managers.Detect().Contains(request.Manager, StringComparer.OrdinalIgnoreCase)) return Results.BadRequest(new { error = "That package manager is not installed on this computer." });
    if (request.Manager == "snap" && request.Action == "repair") return Results.BadRequest(new { error = "Snap does not expose a generic package repair operation." });
    if (request.Manager == "uniget" && request.Action != "update-all") return Results.BadRequest(new { error = "UniGetUI currently supports manager-wide update only in this agent version." });
    if (request.Action != "update-all" && !(request.Manager == "windows-update" && request.Action == "repair"))
    {
        try { _ = PackageManagers.ValidateId(request.PackageId); }
        catch (ArgumentException ex) { return Results.BadRequest(new { error = ex.Message }); }
    }
    var id = Guid.NewGuid().ToString("N");
    var record = new OperationRecord(id, request.Manager, request.Action, request.PackageId, "queued", DateTimeOffset.UtcNow, null, null, "", false, null);
    await store.SaveAsync(record);
    _ = Task.Run(() => ExecuteOperation(request with { Confirmed = true }, record));
    return Results.Accepted($"/api/v1/operations/{id}", record);
});

app.MapPost("/api/v1/operations/batch", async (BatchRequest request) =>
{
    if (!request.Confirmed) return Results.BadRequest(new { error = "Confirm once to run this update batch." });
    if (request.Operations is null || request.Operations.Count is < 1 or > 30 ||
        request.Operations.Any(x => x.Action != "update" || !managers.Detect().Contains(x.Manager, StringComparer.OrdinalIgnoreCase)))
        return Results.BadRequest(new { error = "A batch accepts 1–30 updates from installed package managers only." });
    try { foreach (var item in request.Operations) _ = PackageManagers.ValidateId(item.PackageId); }
    catch (ArgumentException ex) { return Results.BadRequest(new { error = ex.Message }); }
    var ids = new List<string>();
    foreach (var item in request.Operations)
    {
        var id = Guid.NewGuid().ToString("N");
        var record = new OperationRecord(id, item.Manager, "update", item.PackageId, "queued", DateTimeOffset.UtcNow, null, null, "", false, null);
        await store.SaveAsync(record);
        ids.Add(id);
        _ = Task.Run(() => ExecuteOperation(item with { Confirmed = true }, record));
    }
    return Results.Accepted("/api/v1/operations", ids.Select(id => store.Get(id)));
});

app.Run();

async Task ExecuteOperation(OperationRequest request, OperationRecord initial)
{
    var gate = store.LockFor(request.Manager);
    await gate.WaitAsync();
    var current = initial with { State = "running" };
    await store.SaveAsync(current);
    try
    {
        var result = await managers.ExecuteAsync(request);
        var succeeded = result.ExitCode == 0 || (request.Manager == "windows-update" && result.ExitCode == 3010);
        var reboot = OperatingSystem.IsWindows() && (PendingReboot() || result.ExitCode == 3010);
        var state = succeeded ? "completed-review" : "failed";
        var output = result.Output;
        if (succeeded)
        {
            var verification = await managers.VerifyAsync(request);
            output += Environment.NewLine + Environment.NewLine + "===== INTA POST-CHECK =====" + Environment.NewLine + verification.Output;
            state = reboot ? "awaiting-reboot" : verification.Verified ? "verified" : "completed-review";
        }
        current = current with { State = state, FinishedAt = DateTimeOffset.UtcNow,
            ExitCode = result.ExitCode, Output = output, RebootLikely = reboot,
            Error = succeeded && state == "completed-review" ? "Operation exited successfully. Review the post-check output; this manager does not expose a definitive verification for this operation." : null };
    }
    catch (Exception ex)
    {
        current = current with { State = ex is TimeoutException ? "still-running-check-installer" : "failed",
            FinishedAt = DateTimeOffset.UtcNow, Error = ex.Message };
    }
    finally { await store.SaveAsync(current); gate.Release(); }
}

static bool IsElevated()
{
    if (!OperatingSystem.IsWindows()) return Environment.UserName == "root";
    try
    {
        using var identity = System.Security.Principal.WindowsIdentity.GetCurrent();
        return new System.Security.Principal.WindowsPrincipal(identity).IsInRole(System.Security.Principal.WindowsBuiltInRole.Administrator);
    }
    catch { return false; }
}

static bool PendingReboot()
{
    if (!OperatingSystem.IsWindows()) return false;
    try
    {
        using var key = Microsoft.Win32.Registry.LocalMachine.OpenSubKey(@"SOFTWARE\Microsoft\Windows\CurrentVersion\Component Based Servicing\RebootPending");
        return key is not null;
    }
    catch { return false; }
}
