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
            try
            {
                (file, args) = ResolveCommand(manager, file, args);
                var result = await runner.RunAsync(file, args, TimeSpan.FromMinutes(3), ct);
                results.Add(new(manager, file + " " + string.Join(' ', args), result.ExitCode, result.Output, DateTimeOffset.UtcNow));
            }
            catch (Exception ex) { results.Add(new(manager, file + " " + string.Join(' ', args), -1, ex.Message, DateTimeOffset.UtcNow)); }
        }
        return results;
    }
    public async Task<(int ExitCode, string Output)> ExecuteAsync(OperationRequest request, CancellationToken ct = default)
    {
        var package = request.Action == "update-all" ? "*" : ValidateId(request.PackageId);
        var output = new List<string>();
        if (request.Manager == "apt" && request.Action is "update" or "update-all")
        {
            var refresh = await runner.RunAsync("apt-get", ["update"], TimeSpan.FromMinutes(15), ct);
            output.Add("===== apt package index refresh =====" + Environment.NewLine + refresh.Output);
            if (refresh.ExitCode != 0) return (refresh.ExitCode, string.Join(Environment.NewLine, output));
        }
        if (request.Manager == "brew" && request.Action is "update" or "update-all")
        {
            var refresh = await runner.RunAsync("brew", ["update"], TimeSpan.FromMinutes(20), ct);
            output.Add("===== Homebrew metadata refresh =====" + Environment.NewLine + refresh.Output);
            if (refresh.ExitCode != 0) return (refresh.ExitCode, string.Join(Environment.NewLine, output));
        }
        if (request.Manager == "scoop" && request.Action is "update" or "update-all")
        {
            var refresh = await runner.RunAsync("scoop", ["update"], TimeSpan.FromMinutes(10), ct);
            output.Add("===== Scoop bucket refresh =====" + Environment.NewLine + refresh.Output);
            if (refresh.ExitCode != 0) return (refresh.ExitCode, string.Join(Environment.NewLine, output));
        }
        var (file, args) = BuildCommand(request.Manager, request.Action, package);
        var result = await runner.RunAsync(file, args, TimeSpan.FromMinutes(60), ct);
        output.Add(result.Output);
        return (result.ExitCode, string.Join(Environment.NewLine, output));
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
            var previewSucceeded = preview is not null && (preview.ExitCode == 0 ||
                (request.Manager == "dnf" && preview.ExitCode == 100) ||
                (request.Manager == "pacman" && preview.ExitCode == 2));
            var stillPending = previewSucceeded && preview!.Output.Contains(package, StringComparison.OrdinalIgnoreCase);
            return (previewSucceeded && !stillPending,
                preview?.Output ?? "No post-update preview was available.", !previewSucceeded);
        }
        var (file, args) = VerificationCommand(request.Manager, package);
        if (file is null) return (false, "This package manager does not expose a reliable post-operation verification query.", true);
        (file, args) = ResolveCommand(request.Manager, file, args);
        var result = await runner.RunAsync(file, args, TimeSpan.FromMinutes(3), ct);
        var present = result.ExitCode == 0 && result.Output.Contains(package, StringComparison.OrdinalIgnoreCase);
        var absentMarker = result.Output.Contains("not installed", StringComparison.OrdinalIgnoreCase) ||
            result.Output.Contains("no installed package", StringComparison.OrdinalIgnoreCase) ||
            result.Output.Contains("no matching package", StringComparison.OrdinalIgnoreCase);
        var verified = request.Action == "repair" ? present :
            request.Action == "uninstall" && (absentMarker || (result.ExitCode != 0 && string.IsNullOrWhiteSpace(result.Output)));
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
        ("winget", "update-all") => ("winget", ["upgrade", "--all", "--silent", "--accept-source-agreements", "--accept-package-agreements"]),
        ("winget", "update") => ("winget", ["upgrade", "--id", package, "--exact", "--silent", "--accept-source-agreements", "--accept-package-agreements"]),
        ("winget", "repair") => ("winget", ["repair", "--id", package, "--exact", "--silent"]),
        ("winget", "uninstall") => ("winget", ["uninstall", "--id", package, "--exact", "--silent"]),
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
