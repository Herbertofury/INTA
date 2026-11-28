[CmdletBinding()]
param([Parameter(Mandatory)][string]$Executable)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
# Prefer a disposable extracted portable release's official launcher. A direct
# Workbench build remains useful for developer smoke tests, but it skips verification.
$exe = (Resolve-Path -LiteralPath $Executable).Path
$entryDirectory = [IO.Path]::GetDirectoryName($exe)
$outerLauncher = Join-Path (Split-Path $entryDirectory) 'INTA Desktop.exe'
if ([IO.Path]::GetFileName($exe) -ieq 'inta-desktop.exe' -and
    (Split-Path $entryDirectory -Leaf) -ieq 'base' -and
    (Test-Path -LiteralPath $outerLauncher -PathType Leaf)) {
    $exe = $outerLauncher
    $entryDirectory = [IO.Path]::GetDirectoryName($exe)
}
$throughLauncher = [IO.Path]::GetFileName($exe) -ieq 'INTA Desktop.exe'
$payloadDirectory = if ($throughLauncher) { Join-Path $entryDirectory 'base' } else { $entryDirectory }
$innerExe = Join-Path $payloadDirectory 'inta-desktop.exe'
$state = Join-Path ([IO.Path]::GetTempPath()) ('inta-maintenance-ui-' + [Guid]::NewGuid().ToString('N'))
$previousState = $env:INTA_DATA_DIR
$app = $null
function Wait-Until([scriptblock]$Probe, [string]$Failure) {
    $until = [DateTime]::UtcNow.AddSeconds(20)
    do {
        $value = & $Probe
        if ($null -ne $value -and $value -ne $false) { return $value }
        Start-Sleep -Milliseconds 180
    } while ([DateTime]::UtcNow -lt $until)
    throw $Failure
}
function Start-Workspace {
    $launcher = Start-Process -FilePath $exe -WorkingDirectory $entryDirectory -PassThru
    if (-not $throughLauncher) { return $launcher }
    try {
        $child = Wait-Until {
            $launcher.Refresh()
            if ($launcher.HasExited -and $launcher.ExitCode -ne 0) {
                throw "Official launcher failed (exit $($launcher.ExitCode))."
            }
            $process = Get-CimInstance Win32_Process -Filter "ParentProcessId = $($launcher.Id) AND Name = 'inta-desktop.exe'" |
                Where-Object { $_.ExecutablePath -ieq $innerExe } | Select-Object -First 1
            if ($null -ne $process) { Get-Process -Id $process.ProcessId }
        } 'Official launcher did not open the signed Workbench child.'
        if (-not $launcher.WaitForExit(5000) -or $launcher.ExitCode -ne 0) { throw 'Official launcher did not finish cleanly.' }
        $child
    } finally {
        if (-not $launcher.HasExited) { Stop-Process -Id $launcher.Id }
        $launcher.Dispose()
    }
}
function Get-WorkspaceWindow {
    $main = Wait-Until {
        $app.Refresh()
        if ($app.HasExited) { throw "INTA Desktop exited before opening (exit $($app.ExitCode))." }
        if ($app.MainWindowHandle -ne 0) { [Windows.Automation.AutomationElement]::FromHandle($app.MainWindowHandle) }
    } 'The shipping WPF window was not available.'
    if ($main.Current.Name -ne 'INTA Desktop') { throw "Unexpected Workbench window: $($main.Current.Name)" }
    $main
}
try {
    $env:INTA_DATA_DIR = $state
    $app = Start-Workspace
    $window = Get-WorkspaceWindow
    function Find-Name([string]$Name, $Root = $window) {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::NameProperty, $Name)
        $Root.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
    }
    function Open-Route([string]$Name) {
        $nav = Wait-Until { Find-Name 'Workspace navigation' } 'Missing accessible navigation.'
        $nameCondition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::NameProperty, $Name)
        $rowCondition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::ListItem)
        $row = Wait-Until {
            $nav.FindFirst([Windows.Automation.TreeScope]::Descendants,
                [Windows.Automation.AndCondition]::new($nameCondition, $rowCondition))
        } ('Missing route: ' + $Name)
        if (-not $row.Current.IsKeyboardFocusable) { throw ('Not keyboard focusable: ' + $Name) }
        $select = $row.GetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern)
        $select.Select()
        $null = Wait-Until { $select.Current.IsSelected } ('Selected state missing: ' + $Name)
    }
    function Select-TaskbarBookmark {
        $rows = Wait-Until { Find-Name 'Workspace results' } 'System Optimization results were unavailable.'
        $condition = [Windows.Automation.AndCondition]::new(
            [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::NameProperty, 'Taskbar End task'),
            [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::ListItem))
        $row = Wait-Until {
            $rows.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
        } 'The Taskbar End task card was unavailable.'
        $row.GetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern).Select()
    }
    $routes = [ordered]@{
        'Deep Cleanup'='Review cache files'; 'Large File Cleanup'='Largest files from folder analysis'
        'Duplicate Cleanup'='Equal-content candidates from folder analysis'; 'Disk Space Analysis'='Analyze a local folder'
        'Privacy Cleanup'='Privacy Cleanup isn''t available yet'; 'App Uninstaller'='Installed applications'
        'Startup Items'='Startup inventory'; 'System Optimization'='INTA Recommended'; 'System Maintenance'='Windows servicing'
    }
    foreach ($route in $routes.Keys) {
        Open-Route $route
        if ($route -ne 'Deep Cleanup') {
            $null = Wait-Until { Find-Name $routes[$route] } ('Route did not show its canonical view: ' + $route)
        } else {
            $null = Wait-Until { Find-Name 'Review rebuildable cache files' } 'Deep Cleanup did not show its cache review.'
        }
    }

    Open-Route 'Startup Items'
    $filter = Wait-Until { Find-Name 'INTA Recommended' } 'Missing local recommendation filter.'
    $filter.GetCurrentPattern([Windows.Automation.TogglePattern]::Pattern).Toggle()
    $query = Wait-Until { Find-Name 'Search tools in this maintenance view' } 'Missing maintenance-only search.'
    $query.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern).SetValue('no-possible-fixture-matches')
    $null = Wait-Until { Find-Name 'No tools match' } 'Search did not combine with recommendation filtering.'
    $heading = $window.FindFirst([Windows.Automation.TreeScope]::Descendants,
        [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::AutomationIdProperty, 'WorkspaceHeading'))
    if ($null -eq $heading -or $heading.Current.Name -ne 'Startup Items') { throw 'Filtering switched the selected route.' }
    $clear = Wait-Until { Find-Name 'Clear' } 'Missing clear action.'
    $clear.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
    $null = Wait-Until { Find-Name 'Startup inventory' } 'Clear did not restore Startup Items.'
    Open-Route 'Directory'
    $null = Wait-Until { Find-Name 'Search all tools, playbooks and methods' } 'The Technician Desk search no longer works.'
    # Toggle only the local bookmark, never the underlying Windows setting.
    Open-Route 'System Optimization'
    Select-TaskbarBookmark
    $unpin = Wait-Until { Find-Name 'Unpin from favorites' } 'The default Taskbar End task bookmark was not present.'
    $unpin.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
    $null = Wait-Until { Find-Name 'Pin to favorites' } 'The Taskbar End task card was not unpinned.'
    # Exercise the saved route on a literal relaunch of the same disposable package;
    # do not select the route in the reopened process to satisfy its startup proof.
    Open-Route 'Disk Space Analysis'
    $null = Wait-Until { Find-Name 'Analyze a local folder' } 'Disk Space Analysis did not render before closing.'
    if (-not $app.CloseMainWindow() -or -not $app.WaitForExit(6000)) { throw 'The portable fixture did not close cleanly.' }
    $mark = Join-Path $state 'maintenance-view.v1'
    if (-not (Test-Path -LiteralPath $mark -PathType Leaf) -or [IO.File]::ReadAllText($mark) -cne 'disk-space-analysis') {
        throw 'The fixture did not save the selected maintenance route outside the payload.'
    }
    $favoriteMark = Join-Path $state 'favorite-tweak-taskbar-end-task.v1'
    if (-not (Test-Path -LiteralPath $favoriteMark -PathType Leaf) -or [IO.File]::ReadAllText($favoriteMark) -cne '0') {
        throw 'The fixture did not save the bookmark outside the payload.'
    }
    foreach ($name in @('maintenance-view.v1', 'favorite-tweak-taskbar-end-task.v1')) {
        if (Test-Path -LiteralPath (Join-Path $payloadDirectory "state/$name")) { throw "A mutable marker entered the payload: $name" }
    }
    $app.Dispose()
    $app = $null
    $app = Start-Workspace
    $window = Get-WorkspaceWindow
    $null = Wait-Until {
        $restoredHeading = $window.FindFirst([Windows.Automation.TreeScope]::Descendants,
            [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::AutomationIdProperty, 'WorkspaceHeading'))
        if ($null -ne $restoredHeading -and $restoredHeading.Current.Name -ceq 'Disk Space Analysis') {
            Find-Name 'Analyze a local folder'
        }
    } 'The saved Disk Space Analysis route did not render after reopening the portable fixture.'
    Open-Route 'System Optimization'
    Select-TaskbarBookmark
    $null = Wait-Until { Find-Name 'Pin to favorites' } 'The saved unpinned bookmark was not restored after reopening.'
    $entryPoint = if ($throughLauncher) { 'official verified launcher' } else { 'direct WPF build (no signed launch verification)' }
    "WPF routes, scoped filter, Technician Desk, and isolated route and bookmark reopening passed through the $entryPoint. No maintenance action ran."
}
finally {
    if ($null -ne $app) {
        $app.Refresh()
        if (-not $app.HasExited) {
            $null = $app.CloseMainWindow()
            if (-not $app.WaitForExit(5000)) { Stop-Process -Id $app.Id }
        }
        $app.Dispose()
    }
    $env:INTA_DATA_DIR = $previousState
    if (Test-Path -LiteralPath $state) { Remove-Item -LiteralPath $state -Recurse -Force }
}
