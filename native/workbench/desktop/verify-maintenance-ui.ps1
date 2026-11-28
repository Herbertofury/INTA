[CmdletBinding()]
param([Parameter(Mandatory)][string]$Executable)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
# Run against a disposable extracted portable release; the app may save its local selection.
$exe = (Resolve-Path -LiteralPath $Executable).Path
$app = Start-Process -FilePath $exe -WorkingDirectory ([IO.Path]::GetDirectoryName($exe)) -PassThru
function Wait-Until([scriptblock]$Probe, [string]$Failure) {
    $until = [DateTime]::UtcNow.AddSeconds(20)
    do {
        $value = & $Probe
        if ($null -ne $value -and $value -ne $false) { return $value }
        Start-Sleep -Milliseconds 180
    } while ([DateTime]::UtcNow -lt $until)
    throw $Failure
}
try {
    $window = Wait-Until {
        $app.Refresh()
        if ($app.HasExited) { throw 'INTA Desktop exited before opening.' }
        if ($app.MainWindowHandle -ne 0) { [Windows.Automation.AutomationElement]::FromHandle($app.MainWindowHandle) }
    } 'The shipping WPF window was not available.'
    if ($window.Current.Name -ne 'INTA Desktop') { throw 'Unexpected Workbench window.' }
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
    'Shipping WPF route, keyboard-accessible selection, scoped filter and Technician Desk UI checks passed. No maintenance action ran.'
}
finally {
    $app.Refresh()
    if (-not $app.HasExited) {
        $null = $app.CloseMainWindow()
        if (-not $app.WaitForExit(5000)) { Stop-Process -Id $app.Id }
    }
}
