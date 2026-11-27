param([Parameter(Mandatory=$true)][string]$Executable)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$exe = (Resolve-Path -LiteralPath $Executable).Path
$process = Start-Process -FilePath $exe -PassThru

function Wait-Until([scriptblock]$Probe, [string]$Failure) {
    $until = [DateTime]::UtcNow.AddSeconds(22)
    do {
        $value = & $Probe
        if ($null -ne $value -and $value -ne $false) { return $value }
        Start-Sleep -Milliseconds 150
    } while ([DateTime]::UtcNow -lt $until)
    throw $Failure
}

try {
    $window = Wait-Until {
        $process.Refresh()
        if ($process.HasExited) { throw 'The native Desktop program exited before opening its window.' }
        if ($process.MainWindowHandle -ne 0) {
            [Windows.Automation.AutomationElement]::FromHandle($process.MainWindowHandle)
        }
    } 'A real INTA Desktop window did not become available.'
    if ($window.Current.Name -notlike 'INTA Desktop * Startup inventory') { throw 'Unexpected native window title.' }
    function Find-Name([string]$Name) {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::NameProperty, $Name)
        $window.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
    }
    $filter = Wait-Until { Find-Name 'Find a startup registration' } 'Missing accessible search field.'
    $refresh = Wait-Until { Find-Name 'Refresh' } 'Missing accessible refresh action.'
    $settings = Wait-Until { Find-Name 'Open Windows Startup Settings' } 'Missing accessible Settings action.'
    $grid = Wait-Until { Find-Name 'Read-only startup registrations' } 'Missing accessible native data grid.'
    $gridPattern = $grid.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern)
    $allRows = Wait-Until {
        if ($refresh.Current.IsEnabled -and $gridPattern.Current.RowCount -gt 0) { $gridPattern.Current.RowCount }
    } 'Real Windows inventory did not populate the grid.'
    if ($allRows -lt 6) { throw 'Expected one observation per available schema-2 source.' }
    $entry = $filter.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern)
    $entry.SetValue('inta-test-no-match-'+[Guid]::NewGuid().ToString('N'))
    $null = Wait-Until { $gridPattern.Current.RowCount -eq 0 } 'Search did not filter the native grid.'
    $entry.SetValue('')
    $null = Wait-Until { $gridPattern.Current.RowCount -eq $allRows } 'Clearing the search did not restore the observations.'
    $invoke = $refresh.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern)
    $invoke.Invoke()
    $null = Wait-Until {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::Text)
        $labels = $window.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)
        $updated = $false
        foreach ($label in $labels) { if ($label.Current.Name -like '*Scan 2*') { $updated = $true; break } }
        $updated -and $refresh.Current.IsEnabled -and $gridPattern.Current.RowCount -gt 0
    } 'The real refresh did not complete and populate the grid.' 
    if (-not $settings.Current.IsKeyboardFocusable) { throw 'Settings button is not keyboard focusable.' }
    # Do not print live registry/folder names. The manual follow-up verifies
    # the OS Settings handoff in an interactive Windows desktop.
    'Real Windows preview opened and populated. Native search, clear, refresh and accessible Settings action passed.'
}
finally {
    $process.Refresh()
    if (-not $process.HasExited) {
        $null = $process.CloseMainWindow()
        if (-not $process.WaitForExit(4000)) { Stop-Process -Id $process.Id }
    }
}
