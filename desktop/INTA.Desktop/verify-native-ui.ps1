param([Parameter(Mandatory=$true)][string]$Executable, [switch]$InstalledPackage)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$sourceExe = (Resolve-Path -LiteralPath $Executable).Path
$testRoot = $null
$exe = $sourceExe
$process = $null

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
    if (-not $InstalledPackage) {
        $testRoot = Join-Path ([IO.Path]::GetTempPath()) ('inta-preview-ui-'+[Guid]::NewGuid().ToString('N'))
        $null = [IO.Directory]::CreateDirectory($testRoot)
        $exe = Join-Path $testRoot 'INTA.Desktop.exe'
        [IO.File]::Copy($sourceExe, $exe)
    }
    $process = Start-Process -FilePath $exe -WorkingDirectory ([IO.Path]::GetDirectoryName($exe)) -PassThru
    $window = Wait-Until {
        $process.Refresh()
        if ($process.HasExited) { throw 'The native Desktop program exited before opening its window.' }
        if ($process.MainWindowHandle -ne 0) {
            [Windows.Automation.AutomationElement]::FromHandle($process.MainWindowHandle)
        }
    } 'A real INTA Desktop window did not become available.'
    if ($window.Current.Name -notlike 'INTA Desktop * Maintenance') { throw 'Unexpected native window title.' }
    function Find-Name([string]$Name) {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::NameProperty, $Name)
        $window.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
    }
    function Find-Id([string]$Id) {
        $condition = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::AutomationIdProperty, $Id)
        $window.FindFirst([Windows.Automation.TreeScope]::Descendants, $condition)
    }
    function Select-Radio([Windows.Automation.AutomationElement]$Element) {
        $pattern = $null
        if ($Element.TryGetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern, [ref]$pattern)) {
            $pattern.Select()
        } else {
            $Element.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
        }
    }
    $categories = [ordered]@{
        'deep-cleanup'='Deep Cleanup'; 'large-file-cleanup'='Large File Cleanup'; 'duplicate-cleanup'='Duplicate Cleanup'
        'disk-space-analysis'='Disk Space Analysis'; 'privacy-cleanup'='Privacy Cleanup'
        'app-uninstaller'='App Uninstaller'; 'startup-items'='Startup Items'
        'system-optimization'='System Optimization'; 'system-maintenance'='System Maintenance'
    }
    function Open-Category([string]$Id) {
        $row = Wait-Until { Find-Id $Id } ("Missing category " + $Id)
        if (-not $row.Current.IsKeyboardFocusable) { throw ("Category cannot receive keyboard focus: " + $Id) }
        Select-Radio $row
        $heading = Wait-Until { Find-Id ("heading-" + $Id) } ("The category did not open: " + $Id)
        if ($heading.Current.Name -ne $categories[$Id]) { throw ("Incorrect category heading: " + $Id) }
        $selected = $null
        if ($row.TryGetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern, [ref]$selected) -and -not $selected.Current.IsSelected) {
            throw ("Navigation did not expose the selected state: " + $Id)
        }
    }
    Open-Category 'startup-items'
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
    Open-Category 'system-optimization'
    $powerGrid = Wait-Until { Find-Name 'Installed Windows power plans' } 'Missing native Power plans grid.'
    $powerGridPattern = $powerGrid.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern)
    $null = Wait-Until {
        $labelType = [Windows.Automation.PropertyCondition]::new(
            [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::Text)
        $labels = $window.FindAll([Windows.Automation.TreeScope]::Descendants, $labelType)
        foreach ($label in $labels) {
            if ($label.Current.Name -like ('Active plan: '+$before+'*') -and $powerGridPattern.Current.RowCount -gt 0) {
                return $true
            }
        }
    } 'Native Power plans did not show the same active scheme as Windows.'
    if ((Active-Plan) -ne $before) { throw 'The Windows active scheme changed during observation.' }
    $favorite = Wait-Until { Find-Name 'Favorite Tweaks' } 'Favorite Tweaks must be nested under Optimization.'
    Select-Radio $favorite
    $null = Wait-Until { Find-Name 'Taskbar End task' } 'Favorite Tweaks content was not restored inside Optimization.'
    $catalogFilter = Wait-Until { Find-Name 'Search tools in this category' } 'Missing tool catalog search.'
    $recommended = Wait-Until { Find-Name 'INTA Recommended' } 'Missing recommended filter.'
    $toggle = $recommended.GetCurrentPattern([Windows.Automation.TogglePattern]::Pattern)
    $toggle.Toggle()
    $null = Wait-Until { -not (Find-Name 'Taskbar End task') -and (Find-Id 'heading-system-optimization') } 'Recommended filter changed the category or retained a nonmatching operation.'
    $catalogFilter.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern).SetValue('impossible-inta-native-tool')
    $null = Wait-Until { Find-Name "No tools match. Clear filters to see this category's available paths." } 'Search and Recommended did not combine.'
    $clear = Wait-Until { Find-Name 'Clear filters' } 'Missing reset for category filters.'
    $clear.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()
    $null = Wait-Until { Find-Name 'Taskbar End task' } 'Clearing filters did not restore the previously selected operation.'
    Open-Category 'startup-items'
    $null = Wait-Until { (Find-Name 'Refresh').Current.IsEnabled -and $gridPattern.Current.RowCount -eq $allRows } 'Returning did not preserve the Startup inventory.'
    'The embedded Power plans view showed the current Windows plan without changing the active scheme.'
    $fixture = Join-Path ([IO.Path]::GetTempPath()) ('inta-storage-ui-'+[Guid]::NewGuid().ToString('N'))
    $null = [IO.Directory]::CreateDirectory($fixture)
    try {
        $old = Join-Path $fixture 'old.test'
        $recent = Join-Path $fixture 'recent.test'
        [IO.File]::WriteAllBytes($old, [byte[]](1,2,3,4))
        [IO.File]::WriteAllBytes($recent, [byte[]](5,6,7,8,9,10,11,12,13))
        [IO.File]::SetLastWriteTimeUtc($old, [DateTime]::UtcNow.AddDays(-8))
        $oldTime = [IO.File]::GetLastWriteTimeUtc($old)
        $bytes = @(Get-FileHash -LiteralPath $old, $recent | ForEach-Object Hash)
        Open-Category 'disk-space-analysis'
        function Find-StorageControl([string]$Name) { Find-Name $Name }
        function Storage-Status([string]$Pattern) {
            $condition = [Windows.Automation.PropertyCondition]::new(
                [Windows.Automation.AutomationElement]::ControlTypeProperty, [Windows.Automation.ControlType]::Text)
            $labels = $window.FindAll([Windows.Automation.TreeScope]::Descendants, $condition)
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
        $allocation = Wait-Until { Find-StorageControl 'Include observed disk allocation' } 'Missing allocation opt-in.'
        $allocationToggle = $allocation.GetCurrentPattern([Windows.Automation.TogglePattern]::Pattern)
        if ($allocationToggle.Current.ToggleState -ne [Windows.Automation.ToggleState]::Off) {
            throw 'Allocation must be off by default.'
        }
        $edit.SetValue($fixture)
        $scan.Invoke()
        $null = Wait-Until { (Storage-Status 'Preview complete: 2 files, 13 logical bytes.*') -and $rows.Current.RowCount -eq 8 -and $preview.Current.IsEnabled } 'Storage totals did not match the synthetic fixture.'
        if ([IO.File]::GetLastWriteTimeUtc($old) -ne $oldTime -or
            (Compare-Object $bytes @(Get-FileHash -LiteralPath $old, $recent | ForEach-Object Hash))) { throw 'The read-only scan modified the fixture.' }
        $allocationToggle.Toggle()
        $scan.Invoke()
        $null = Wait-Until { (Storage-Status 'Preview complete: 2 files, 13 logical bytes.*') -and $rows.Current.RowCount -eq 14 -and $preview.Current.IsEnabled } 'Opt-in allocation totals were not shown.'
        if ([IO.File]::GetLastWriteTimeUtc($old) -ne $oldTime -or
            (Compare-Object $bytes @(Get-FileHash -LiteralPath $old, $recent | ForEach-Object Hash))) { throw 'Allocation scan modified the fixture.' }
        $edit.SetValue([IO.Path]::GetPathRoot($fixture))
        $scan.Invoke()
        $null = Wait-Until { (Storage-Status '*not a drive root or link*') -and $rows.Current.RowCount -eq 0 -and $preview.Current.IsEnabled } 'The drive-root request was not rejected before scanning.'
    } finally {
        Remove-Item -LiteralPath $fixture -Recurse -Force
    }
    'The real Storage preview matched the scoped fixture, left its files unchanged, and refused a whole-drive scan.'
    function Registered-Snapshot {
        $sub = 'Software\Microsoft\Windows\CurrentVersion\Uninstall'
        if ([Environment]::Is64BitOperatingSystem) {
            $scopes = ,@('CurrentUser','Default') + ,@('LocalMachine','Registry64') + ,@('LocalMachine','Registry32')
        } else { $scopes = ,@('CurrentUser','Default') + ,@('LocalMachine','Default') }
        $evidence = [Collections.Generic.List[string]]::new()
        foreach ($scope in $scopes) {
            $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey($scope[0], $scope[1])
            try {
                $parent = $base.OpenSubKey($sub, $false)
                if ($null -eq $parent) { continue }
                try {
                    foreach ($name in ($parent.GetSubKeyNames() | Select-Object -First 4096)) {
                        $app = $parent.OpenSubKey($name, $false)
                        if ($null -eq $app) { continue }
                        try {
                            $title = $app.GetValue('DisplayName', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
                            $system = $app.GetValue('SystemComponent')
                            if ($title -is [string] -and -not [string]::IsNullOrWhiteSpace($title) -and -not ($system -is [int] -and $system -eq 1)) {
                                $version = $app.GetValue('DisplayVersion', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
                                $publisher = $app.GetValue('Publisher', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
                                $evidence.Add((@($scope[0],$scope[1],$name,$title,$version,$publisher) -join [char]0))
                            }
                        } finally { $app.Dispose() }
                    }
                } finally { $parent.Dispose() }
            } finally { $base.Dispose() }
        }
        return $evidence.ToArray() | Sort-Object -CaseSensitive
    }

    $registrationsBefore = @(Registered-Snapshot)
    $expectedPrograms = $registrationsBefore.Count
    if ($expectedPrograms -lt 1) { throw 'Windows must supply at least one desktop-program registry fixture.' }
    Open-Category 'app-uninstaller'
    function Find-ProgramsControl([string]$Name) { Find-Name $Name }
    $programGrid = Wait-Until { Find-ProgramsControl 'Desktop program registrations' } 'Missing native programs grid.'
    $programFilter = Wait-Until { Find-ProgramsControl 'Search desktop programs' } 'Missing accessible program search.'
    $programRows = $programGrid.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern)
    $null = Wait-Until { $programRows.Current.RowCount -eq $expectedPrograms } 'Desktop program list did not match the independent registry count.'
    $searchPrograms = $programFilter.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern)
    $searchPrograms.SetValue('inta-test-no-program-'+[Guid]::NewGuid().ToString('N'))
    $null = Wait-Until { $programRows.Current.RowCount -eq 0 } 'Installed programs did not filter.'
    $searchPrograms.SetValue('')
    $null = Wait-Until { $programRows.Current.RowCount -eq $expectedPrograms } 'Clearing the program search did not restore entries.'
    if (Compare-Object -ReferenceObject $registrationsBefore -DifferenceObject @(Registered-Snapshot) -CaseSensitive) { throw 'Desktop program registry observations changed after the read-only preview.' }
    'Native Installed programs matched the independent registry count and passed search; entries were not changed.'
    foreach ($id in @('deep-cleanup', 'large-file-cleanup', 'duplicate-cleanup', 'privacy-cleanup', 'system-maintenance')) {
        Open-Category $id
        $null = Wait-Until { Find-Name 'What to review' } ("Missing honest evidence review: " + $id)
        if (Find-Name 'Preview storage') { throw ('A non-native category exposed a duplicate primary operation: ' + $id) }
    }
    Open-Category 'disk-space-analysis'
    $kept = Wait-Until { Find-Name 'Storage preview totals' } 'Storage did not return to the canonical workspace.'
    if ($kept.GetCurrentPattern([Windows.Automation.GridPattern]::Pattern).Current.RowCount -ne 0) { throw 'Scoped preview state was not preserved.' }

    if (-not $InstalledPackage) {
        # Restart the disposable test binary from the same package directory, without
        # touching the operator's installation, and prove navigation restoration.
        Open-Category 'system-maintenance'
        if (-not $process.CloseMainWindow() -or -not $process.WaitForExit(8000)) { throw 'Preview did not close cleanly.' }
        $process = Start-Process -FilePath $exe -WorkingDirectory ([IO.Path]::GetDirectoryName($exe)) -PassThru
        $window = Wait-Until {
            $process.Refresh()
            if ($process.HasExited) { throw 'The preview exited on relaunch.' }
            if ($process.MainWindowHandle -ne 0) { [Windows.Automation.AutomationElement]::FromHandle($process.MainWindowHandle) }
        } 'The preview did not reopen.'
        $null = Wait-Until { Find-Id 'heading-system-maintenance' } 'The last category was not restored on relaunch.'
        'Portable navigation restoration passed in the disposable application folder.'
    }
    'Nine canonical routes, recommendation + search, and preserved data passed.'

}
finally {
    if ($null -ne $process) {
        $process.Refresh()
        if (-not $process.HasExited) {
            $null = $process.CloseMainWindow()
            if (-not $process.WaitForExit(4000)) {
                Stop-Process -Id $process.Id
                $process.WaitForExit()
            }
        }
    }
    if ($null -ne $testRoot) { Remove-Item -LiteralPath $testRoot -Recurse -Force }
}
