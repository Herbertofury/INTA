using System.Text.RegularExpressions;
namespace INTA.LocalAgent;
public sealed class PackageManagers(ProcessRunner runner)
{
    private static readonly string[] WindowsManagers = ["winget", "choco", "scoop", "uniget"];
    private static readonly string[] LinuxManagers = ["apt", "dnf", "pacman", "flatpak", "snap"];
    private static readonly string[] MacManagers = ["brew"];
    public string[] Detect()
    {
        var candidates = OperatingSystem.IsWindows() ? WindowsManagers : OperatingSystem.IsMacOS() ? MacManagers : LinuxManagers;
        return candidates.Where(Available).ToArray();
    }
    public async Task<IReadOnlyList<ManagerPreview>> PreviewAsync(string[] managers, CancellationToken ct = default)
    {
        var results = new List<ManagerPreview>();
        foreach (var manager in managers)
        {
            var (file, args) = PreviewCommand(manager);
            if (file is null) continue;
            try { var result = await runner.RunAsync(file, args, TimeSpan.FromMinutes(3), ct); results.Add(new(manager, file + " " + string.Join(' ', args), result.ExitCode, result.Output, DateTimeOffset.UtcNow)); }
            catch (Exception ex) { results.Add(new(manager, file + " " + string.Join(' ', args), -1, ex.Message, DateTimeOffset.UtcNow)); }
        }
        return results;
    }
    public async Task<(int ExitCode, string Output)> ExecuteAsync(OperationRequest request, CancellationToken ct = default)
    {
        var package = request.Action == "update-all" ? "*" : ValidateId(request.PackageId);
        var (file, args) = BuildCommand(request.Manager, request.Action, package);
        return await runner.RunAsync(file, args, TimeSpan.FromMinutes(60), ct);
    }

    public static string ValidateId(string value)
    {
        if (string.IsNullOrWhiteSpace(value) || value.Length > 180 || value.StartsWith('-') ||
            !Regex.IsMatch(value, @"^[A-Za-z0-9][A-Za-z0-9._+@:/-]*$"))
            throw new ArgumentException("Enter the exact package ID shown by the selected package manager. Shell syntax and command-line switches are not accepted.");
        return value;
    }
    public async Task<(bool Verified, string Output, bool NeedsReview)> VerifyAsync(OperationRequest request, CancellationToken ct = default)
    {
        if (request.Action == "update-all")
        {
            var previews = await PreviewAsync([request.Manager], ct);
            var preview = previews.FirstOrDefault();
            return (false, preview?.Output ?? "No post-update preview was available.", true);
        }
        var package = ValidateId(request.PackageId);
        if (request.Action == "update")
        {
            var previews = await PreviewAsync([request.Manager], ct);
            var preview = previews.FirstOrDefault();
            var stillPending = preview is not null && preview.ExitCode == 0 && preview.Output.Contains(package, StringComparison.OrdinalIgnoreCase);
            return (preview is not null && preview.ExitCode == 0 && !stillPending,
                preview?.Output ?? "No post-update preview was available.", preview is null || preview.ExitCode != 0);
        }
        var (file, args) = VerificationCommand(request.Manager, package);
        if (file is null) return (false, "This package manager does not expose a reliable post-operation verification query.", true);
        var result = await runner.RunAsync(file, args, TimeSpan.FromMinutes(3), ct);
        var present = result.ExitCode == 0 && result.Output.Contains(package, StringComparison.OrdinalIgnoreCase);
        var verified = request.Action == "repair" ? present : request.Action == "uninstall" && result.ExitCode != 0;
        return (verified, result.Output, false);
    }

    private static (string? File, string[] Args) VerificationCommand(string manager, string package) => manager switch
    {
        "winget" => ("winget", ["list", "--id", package, "--exact", "--disable-interactivity"]),
        "choco" => ("choco", ["list", "--limit-output", "--exact", package]),
        "scoop" => ("scoop", ["list", package]),
        "apt" => ("dpkg-query", ["-W", "-f=$" + "{Version}", package]),
        "dnf" => ("rpm", ["-q", package]),
        "pacman" => ("pacman", ["-Q", package]),
        "brew" => ("brew", ["list", "--versions", package]),
        "flatpak" => ("flatpak", ["info", package]),
        "snap" => ("snap", ["list", package]),
        _ => (null, [])
    };

    private static (string? File, string[] Args) PreviewCommand(string manager) => manager switch
    {
        "winget" => ("winget", ["upgrade", "--accept-source-agreements", "--disable-interactivity"]),
        "choco" => ("choco", ["outdated"]),
        "scoop" => ("scoop", ["status"]),
        "uniget" => ("uniget.exe", ["package", "updates", "--manager", "winget"]),
        "apt" => ("apt", ["list", "--upgradable"]),
        "dnf" => ("dnf", ["check-update"]),
        "pacman" => ("checkupdates", []),
        "flatpak" => ("flatpak", ["remote-ls", "--updates"]),
        "snap" => ("snap", ["refresh", "--list"]),
        "brew" => ("brew", ["outdated", "--verbose"]),
        _ => (null, [])
    };
    private static (string File, string[] Args) BuildCommand(string manager, string action, string package) => (manager, action) switch
    {
        ("winget", "update-all") => ("winget", ["upgrade", "--all", "--interactive", "--accept-source-agreements"]),
        ("winget", "update") => ("winget", ["upgrade", "--id", package, "--exact", "--interactive", "--accept-source-agreements"]),
        ("winget", "repair") => ("winget", ["repair", "--id", package, "--exact", "--interactive"]),
        ("winget", "uninstall") => ("winget", ["uninstall", "--id", package, "--exact", "--interactive"]),
        ("choco", "update-all") => ("choco", ["upgrade", "all", "--yes"]),
        ("choco", "update") => ("choco", ["upgrade", package, "--yes"]),
        ("choco", "repair") => ("choco", ["install", package, "--force", "--yes"]),
        ("choco", "uninstall") => ("choco", ["uninstall", package, "--confirm"]),
        ("scoop", "update-all") => ("scoop", ["update", "*"]),
        ("scoop", "update") => ("scoop", ["update", package]),
        ("scoop", "repair") => ("scoop", ["reinstall", package]),
        ("scoop", "uninstall") => ("scoop", ["uninstall", package]),
        ("apt", "update-all") => ("apt-get", ["upgrade", "-y"]),
        ("apt", "update") => ("apt-get", ["install", "--only-upgrade", "-y", package]),
        ("apt", "repair") => ("apt-get", ["install", "--reinstall", "-y", package]),
        ("apt", "uninstall") => ("apt-get", ["remove", "-y", package]),
        ("dnf", "update-all") => ("dnf", ["upgrade", "-y"]),
        ("dnf", "update") => ("dnf", ["upgrade", "-y", package]),
        ("dnf", "repair") => ("dnf", ["reinstall", "-y", package]),
        ("dnf", "uninstall") => ("dnf", ["remove", "-y", package]),
        ("pacman", "update-all") => ("pacman", ["-Syu", "--noconfirm"]),
        ("pacman", "update") => ("pacman", ["-S", "--needed", "--noconfirm", package]),
        ("pacman", "repair") => ("pacman", ["-S", "--noconfirm", package]),
        ("pacman", "uninstall") => ("pacman", ["-R", "--noconfirm", package]),
        ("brew", "update-all") => ("brew", ["upgrade"]),
        ("brew", "update") => ("brew", ["upgrade", package]),
        ("brew", "repair") => ("brew", ["reinstall", package]),
        ("brew", "uninstall") => ("brew", ["uninstall", package]),
        ("flatpak", "update-all") => ("flatpak", ["update", "-y"]),
        ("flatpak", "update") => ("flatpak", ["update", "-y", package]),
        ("flatpak", "repair") => ("flatpak", ["repair"]),
        ("flatpak", "uninstall") => ("flatpak", ["uninstall", package]),
        ("snap", "update-all") => ("snap", ["refresh"]),
        ("snap", "update") => ("snap", ["refresh", package]),
        ("snap", "uninstall") => ("snap", ["remove", package]),
        ("uniget", "update-all") => ("uniget.exe", ["package", "update-all", "--wait", "true", "--timeout", "3600"]),
        _ => throw new ArgumentException($"The {manager} manager does not support the {action} operation.")
    };
    private static string ValidatePackageId(string value)
    {
        if (string.IsNullOrWhiteSpace(value) || value.Length > 180 || value.StartsWith('-') || !Regex.IsMatch(value, @"^[A-Za-z0-9][A-Za-z0-9._+@:/-]*$"))
            throw new ArgumentException("Enter the exact package ID shown by the selected package manager. Shell syntax and command-line switches are not accepted.");
        return value;
    }
    private static bool Available(string name)
    {
        if (name == "uniget" && !OperatingSystem.IsWindows()) return false;
        var path = Environment.GetEnvironmentVariable("PATH") ?? "";
        var extensions = OperatingSystem.IsWindows() ? (Environment.GetEnvironmentVariable("PATHEXT") ?? ".EXE;.CMD;.BAT").Split(';') : [""];
        return path.Split(Path.PathSeparator).Any(folder => extensions.Any(ext => File.Exists(Path.Combine(folder, name + ext))));
    }
}
