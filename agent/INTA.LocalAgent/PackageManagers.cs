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
        var package = ValidatePackageId(request.PackageId);
        var (file, args) = BuildCommand(request.Manager, request.Action, package);
        return await runner.RunAsync(file, args, TimeSpan.FromMinutes(60), ct);
    }
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
        ("winget", "update") => ("winget", ["upgrade", "--id", package, "--exact", "--interactive", "--accept-source-agreements"]),
        ("winget", "repair") => ("winget", ["repair", "--id", package, "--exact", "--interactive"]),
        ("winget", "uninstall") => ("winget", ["uninstall", "--id", package, "--exact", "--interactive"]),
        ("choco", "update") => ("choco", ["upgrade", package, "--yes"]),
        ("choco", "repair") => ("choco", ["install", package, "--force", "--yes"]),
        ("choco", "uninstall") => ("choco", ["uninstall", package, "--confirm"]),
        ("scoop", "update") => ("scoop", ["update", package]),
        ("scoop", "repair") => ("scoop", ["reinstall", package]),
        ("scoop", "uninstall") => ("scoop", ["uninstall", package]),
        ("apt", "update") => ("apt-get", ["install", "--only-upgrade", "-y", package]),
        ("apt", "repair") => ("apt-get", ["install", "--reinstall", "-y", package]),
        ("apt", "uninstall") => ("apt-get", ["remove", "-y", package]),
        ("dnf", "update") => ("dnf", ["upgrade", "-y", package]),
        ("dnf", "repair") => ("dnf", ["reinstall", "-y", package]),
        ("dnf", "uninstall") => ("dnf", ["remove", "-y", package]),
        ("pacman", "update") => ("pacman", ["-S", "--needed", "--noconfirm", package]),
        ("pacman", "repair") => ("pacman", ["-S", "--noconfirm", package]),
        ("pacman", "uninstall") => ("pacman", ["-R", "--noconfirm", package]),
        ("brew", "update") => ("brew", ["upgrade", package]),
        ("brew", "repair") => ("brew", ["reinstall", package]),
        ("brew", "uninstall") => ("brew", ["uninstall", package]),
        ("flatpak", "update") => ("flatpak", ["update", "-y", package]),
        ("flatpak", "repair") => ("flatpak", ["repair"]),
        ("flatpak", "uninstall") => ("flatpak", ["uninstall", package]),
        ("snap", "update") => ("snap", ["refresh", package]),
        ("snap", "uninstall") => ("snap", ["remove", package]),
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
