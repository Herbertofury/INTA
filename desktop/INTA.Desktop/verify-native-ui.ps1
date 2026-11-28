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
    function Active-Plan {
        $cfg = Join-Path $env:SystemRoot 'System32\powercfg.exe'
        $output = & $cfg /getactivescheme | Out-String
        if ($LASTEXITCODE -ne 0) { throw 'Windows could not read its active power scheme.' }
        $pattern = '[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}'
        $guid = [Regex]::Match($output, $pattern, [Text.RegularExpressions.RegexOptions]::IgnoreCase)
        if (-not $guid.Success) { throw 'The active scheme could not be identified.' }
        $guid.Value
    }

    $before = Active-Plan
    $power = Wait-Until { Find-Name 'Open power plans' } 'Missing accessible Power plans action.'
    $power.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
    $powerWindow = Wait-Until {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::ProcessIdProperty, [int]$process.Id)
        $matches = [Windows.Automation.AutomationElement]::RootElement.FindAll(
            [Windows.Automation.TreeScope]::Children, $condition)
        foreach ($item in $matches) {
            if ($item.Current.Name -like 'INTA Desktop * Power plans') { return $item }
        }
        $matches = $window.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)
        foreach ($item in $matches) {
            if ($item.Current.Name -like 'INTA Desktop * Power plans') { return $item }
        }
    } 'The real native Power plans window did not launch.'
    try {
        $gridName = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::NameProperty, 'Installed Windows power plans')
        $powerGrid = Wait-Until {
            $powerWindow.FindFirst([Windows.Automation.TreeScope]::Descendants, $gridName)
        } 'Missing native Power plans grid.'
        $powerGridPattern = $powerGrid.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern)
        $null = Wait-Until {
            $labelType = [Windows.Automation.PropertyCondition]::new(
                [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::Text)
            $labels = $powerWindow.FindAll([Windows.Automation.TreeScope]::Descendants, $labelType)
            foreach ($label in $labels) {
                if ($label.Current.Name -like ('Active plan: '+$before+'*') -and $powerGridPattern.Current.RowCount -gt 0) {
                    return $true
                }
            }
        } 'Native Power plans did not show the same active scheme as Windows.'
        if ((Active-Plan) -ne $before) { throw 'The Windows active scheme changed during observation.' }
    } finally {
        $powerWindow.GetCurrentPattern([Windows.Automation.WindowPattern]::Pattern).Close()
    }
    $null = Wait-Until { $refresh.Current.IsEnabled } 'Could not return to the Startup inventory window.'
    'The real Power plans window showed the current Windows plan without changing the active scheme.'
    $fixture = Join-Path ([IO.Path]::GetTempPath()) ('inta-storage-ui-'+[Guid]::NewGuid().ToString('N'))
    $null = [IO.Directory]::CreateDirectory($fixture)
    $storageWindow = $null
    try {
        $old = Join-Path $fixture 'old.test'
        $recent = Join-Path $fixture 'recent.test'
        [IO.File]::WriteAllBytes($old, [byte[]](1,2,3,4))
        [IO.File]::WriteAllBytes($recent, [byte[]](5,6,7,8,9,10,11,12,13))
        [IO.File]::SetLastWriteTimeUtc($old, [DateTime]::UtcNow.AddDays(-8))
        $oldTime = [IO.File]::GetLastWriteTimeUtc($old)
        $bytes = @(Get-FileHash -LiteralPath $old, $recent | ForEach-Object Hash)
        $storage = Wait-Until { Find-Name 'Open storage preview' } 'Missing Storage preview action.'
        $storage.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
        $storageWindow = Wait-Until {
            $condition = [Windows.Automation.PropertyCondition]::new(
                [Windows.Automation.AutomationElement]::ProcessIdProperty, [int]$process.Id)
            $matches = [Windows.Automation.AutomationElement]::RootElement.FindAll([Windows.Automation.TreeScope]::Children, $condition)
            foreach ($item in $matches) { if ($item.Current.Name -like 'INTA Desktop * Storage preview') { return $item } }
            $matches = $window.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)
            foreach ($item in $matches) { if ($item.Current.Name -like 'INTA Desktop * Storage preview') { return $item } }
        } 'The native Storage preview window did not launch.'
        function Find-StorageControl([string]$Name) {
            $condition = [Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::NameProperty, $Name)
            $storageWindow.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
        }
        function Storage-Status([string]$Pattern) {
            $condition = [Windows.Automation.PropertyCondition]::new(
                [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::Text)
            $labels = $storageWindow.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)
            foreach ($label in $labels) { if ($label.Current.Name -like $Pattern) { return $true } }
            return $false
        }
        $pathInput = Wait-Until { Find-StorageControl 'Local folder path' } 'Missing accessible local-folder input.'
        $folderPicker = Wait-Until { Find-StorageControl 'Choose local folder' } 'Missing explicit folder picker.'
        if (-not $folderPicker.Current.IsKeyboardFocusable) { throw 'The folder picker is not keyboard focusable.' }
        $preview = Wait-Until { Find-StorageControl 'Preview storage' } 'Missing accessible storage scan action.'
        $totals = Wait-Until { Find-StorageControl 'Storage preview totals' } 'Missing native storage totals grid.'
        $rows = $totals.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern)
        if ($rows.Current.RowCount -ne 0) { throw 'Storage must not scan on startup.' }
        $edit = $pathInput.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern)
        $scan = $preview.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern)
        $edit.SetValue($fixture)
        $scan.Invoke()
        $null = Wait-Until { (Storage-Status 'Preview complete: 2 files, 13 bytes.*') -and $rows.Current.RowCount -eq 8 -and $preview.Current.IsEnabled } 'Storage totals did not match the synthetic fixture.'
        if ([IO.File]::GetLastWriteTimeUtc($old) -ne $oldTime -or
            (Compare-Object $bytes @(Get-FileHash -LiteralPath $old, $recent | ForEach-Object Hash))) { throw 'The read-only scan modified the fixture.' }
        $edit.SetValue([IO.Path]::GetPathRoot($fixture))
        $scan.Invoke()
        $null = Wait-Until { (Storage-Status '*not a drive root or link*') -and $rows.Current.RowCount -eq 0 -and $preview.Current.IsEnabled } 'The drive-root request was not rejected before scanning.'
    } finally {
        if ($null -ne $storageWindow) { $storageWindow.GetCurrentPattern([Windows.Automation.WindowPattern]::Pattern).Close() }
        Remove-Item -LiteralPath $fixture -Recurse -Force
    }
    'The real Storage preview matched the scoped fixture, left its files unchanged, and refused a whole-drive scan.'


}
finally {
    $process.Refresh()
    if (-not $process.HasExited) {
        $null = $process.CloseMainWindow()
        if (-not $process.WaitForExit(4000)) { Stop-Process -Id $process.Id }
    }
}
