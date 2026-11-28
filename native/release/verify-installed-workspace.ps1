[CmdletBinding()]
param([Parameter(Mandatory)][string]$InstallRoot, [Parameter(Mandatory)][string]$VerifiedPortableRoot)
$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted') {
    throw 'Installed Workbench acceptance requires a disposable GitHub-hosted Windows runner.'
}
Add-Type -AssemblyName UIAutomationClient,UIAutomationTypes

function Get-InstalledWorkspacePaths([string]$Installed, [string]$Verified) {
    $root = (Resolve-Path -LiteralPath $Installed).ProviderPath
    $portable = (Resolve-Path -LiteralPath $Verified).ProviderPath
    if (Test-Path -LiteralPath (Join-Path $root 'inta-portable.marker')) {
        throw 'The installed Workbench fixture may not be a portable copy.'
    }
    foreach ($relative in @('INTA Desktop.exe', 'base/inta-desktop.exe')) {
        $actual = Get-FileHash -LiteralPath (Join-Path $root $relative) -Algorithm SHA256
        $expected = Get-FileHash -LiteralPath (Join-Path $portable $relative) -Algorithm SHA256
        if ($actual.Hash -cne $expected.Hash) { throw "Installed $relative did not match the independently verified package." }
    }
    @{
        Launcher = (Resolve-Path -LiteralPath (Join-Path $root 'INTA Desktop.exe')).ProviderPath
        Workbench = (Resolve-Path -LiteralPath (Join-Path $root 'base/inta-desktop.exe')).ProviderPath
        NavigationMarker = Join-Path $root 'base/state/maintenance-view.v1'
    }
}

function Wait-InstalledChild([Diagnostics.Process]$Launch, [string]$ExpectedPath) {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    do {
        $Launch.Refresh()
        if ($Launch.HasExited -and $Launch.ExitCode -ne 0) {
            throw "Installed launcher reported exit code $($Launch.ExitCode)."
        }
        # WMI supplies the parent relation only; check the same-user child image
        # through its Process object, without relying on WMI ExecutablePath privilege.
        $rows = @(Get-CimInstance -ClassName Win32_Process -Property ProcessId -Filter ("ParentProcessId = {0} AND Name = 'inta-desktop.exe'" -f $Launch.Id))
        if ($rows.Count -gt 1) { throw 'The installed launcher started more than one Workbench process.' }
        if ($rows.Count -eq 1) {
            $child = [Diagnostics.Process]::GetProcessById([int]$rows[0].ProcessId)
            try {
                if (-not [string]::Equals($child.MainModule.FileName, $ExpectedPath, [StringComparison]::OrdinalIgnoreCase) -or
                    $child.StartTime -lt $Launch.StartTime) {
                    throw 'The inspected Workbench process does not own the verified installed executable.'
                }
                return $child
            } catch { $child.Dispose(); throw }
        }
        Start-Sleep -Milliseconds 150
    } while ($timer.ElapsedMilliseconds -lt 30000)
    $title = if ($Launch.HasExited) { "exited $($Launch.ExitCode)" } else { "window '$($Launch.MainWindowTitle)'" }
    throw "Installed launcher produced no verified Workbench child (launcher $title)."
}

function Wait-InstalledUi([Diagnostics.Process]$Owned, [scriptblock]$Read, [string]$Description) {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    do {
        $Owned.Refresh()
        if ($Owned.HasExited) { throw "Installed Workbench exited with $($Owned.ExitCode) before $Description." }
        $match = & $Read
        if ($match) { return $match }
        Start-Sleep -Milliseconds 150
    } while ($timer.ElapsedMilliseconds -lt 30000)
    $observed = if ($null -ne $window) {
        try {
            $head = Find-InstalledElement $window ([Windows.Automation.AutomationElement]::AutomationIdProperty) 'WorkspaceHeading'
            $status = Find-InstalledElement $window ([Windows.Automation.AutomationElement]::AutomationIdProperty) 'WorkspaceStatus'
            " Heading='$($head.Current.Name)', status='$($status.Current.Name)'."
        } catch { " UI Automation diagnostics unavailable: $($_.Exception.GetType().Name)." }
    } else { '' }
    throw "Installed Workbench did not show $Description within 30 seconds.$observed"
}

function Find-InstalledElement($Window, $Property, [string]$Value) {
    $Window.FindFirst([Windows.Automation.TreeScope]::Descendants,[Windows.Automation.PropertyCondition]::new($Property,$Value))
}

function Close-OwnedInstalledProcess([Diagnostics.Process]$Owned) {
    try {
        if ($Owned.HasExited) { return }
        $null = $Owned.CloseMainWindow()
        if ($Owned.WaitForExit(6000)) { return }
        try { $Owned.Kill() } catch [InvalidOperationException] { if ($Owned.HasExited) { return }; throw }
        if (-not $Owned.WaitForExit(4000)) { throw "Known fixture process $($Owned.Id) remained running." }
        throw "Known fixture process $($Owned.Id) needed forced termination."
    } finally { $Owned.Dispose() }
}

$expectedRoot = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'Programs/INTA Desktop'
if (-not [string]::Equals([IO.Path]::GetFullPath($InstallRoot), [IO.Path]::GetFullPath($expectedRoot), [StringComparison]::OrdinalIgnoreCase)) {
    throw 'The fixture must be the configured per-user MSI install path.'
}
$paths = Get-InstalledWorkspacePaths $InstallRoot $VerifiedPortableRoot
$pointer = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'INTA/desktop-development/active.txt'
if ((Test-Path -LiteralPath $pointer) -or (Test-Path -LiteralPath $paths.NavigationMarker)) {
    throw 'The runner is not a fresh installed fixture; a revision pointer or navigation marker already exists.'
}

$window = $null
$launcher = $null
$workbench = $null
try {
    $launcher = Start-Process -FilePath $paths.Launcher -WorkingDirectory $InstallRoot -PassThru
    $workbench = Wait-InstalledChild $launcher $paths.Workbench
    if (-not $launcher.WaitForExit(6000) -or $launcher.ExitCode -ne 0) {
        throw 'The installed launcher did not finish successfully after starting its Workbench child.'
    }
    $window = Wait-InstalledUi $workbench {
        if ($workbench.MainWindowHandle -ne [IntPtr]::Zero) {
            [Windows.Automation.AutomationElement]::FromHandle($workbench.MainWindowHandle)
        }
    } 'the native WPF window'
    if ($window.Current.Name -cne 'INTA Desktop') { throw "Unexpected installed Workbench window: $($window.Current.Name)" }
    $idProperty = [Windows.Automation.AutomationElement]::AutomationIdProperty
    $nameProperty = [Windows.Automation.AutomationElement]::NameProperty
    $null = Wait-InstalledUi $workbench {
        $heading = Find-InstalledElement $window $idProperty 'WorkspaceHeading'
        if ($null -ne $heading -and $heading.Current.Name -ceq 'Technician directory') { $heading }
    } 'the Technician directory'
    $search = Wait-InstalledUi $workbench { Find-InstalledElement $window $nameProperty 'Search all tools, playbooks and methods' } 'global search'
    $search.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern).SetValue('7-Zip')
    $null = Wait-InstalledUi $workbench { Find-InstalledElement $window $nameProperty 'Review installation for 7-Zip' } 'the 7-Zip installation review'
    $null = Wait-InstalledUi $workbench { Find-InstalledElement $window $nameProperty 'Review launch for 7-Zip' } 'the 7-Zip launch review'
    # Opening review controls is not required to prove startup; no tool is installed/launched.
} finally {
    try { if ($null -ne $workbench) { Close-OwnedInstalledProcess $workbench } }
    finally { if ($null -ne $launcher) { Close-OwnedInstalledProcess $launcher } }
}
if (Test-Path -LiteralPath $paths.NavigationMarker) { throw 'The read-only installed launch wrote a navigation marker into the package.' }
'Verified the per-user MSI launcher, exact installed WPF child, Technician directory, and 7-Zip search actions without invoking them.'
