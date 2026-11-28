param([Parameter(Mandatory)][string]$Executable)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient,UIAutomationTypes
$app = (Resolve-Path -LiteralPath $Executable).Path
$p = Start-Process -FilePath $app -WorkingDirectory (Split-Path $app) -PassThru
$condition = [Windows.Automation.PropertyCondition]
$scope = [Windows.Automation.TreeScope]::Descendants
function Wait-For([scriptblock]$Read, [string]$Description) {
    $until = [DateTime]::UtcNow.AddSeconds(30)
    do {
        $p.Refresh()
        if ($p.HasExited) { throw "Desktop exited before $Description with $($p.ExitCode)" }
        $value = & $Read
        if ($value) { return $value }
        Start-Sleep -Milliseconds 150
    } while ([DateTime]::UtcNow -lt $until)
    throw "Timed out waiting for $Description."
}
try {
    $window = Wait-For {
        if ($p.MainWindowHandle -ne [IntPtr]::Zero) {
            [Windows.Automation.AutomationElement]::FromHandle($p.MainWindowHandle)
        }
    } 'the original workspace'
    if ($window.Current.Name -ne 'INTA Desktop') { throw "Unexpected window: $($window.Current.Name)" }
    function Named([string]$Name) {
        $window.FindFirst($scope,$condition::new([Windows.Automation.AutomationElement]::NameProperty,$Name))
    }
    $null = Wait-For { Named 'Technician directory' } 'the directory on first launch'
    $search = Named 'Search all tools, playbooks and methods'
    if (!$search) { throw 'Global search missing' }
    $search.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern).SetValue('7-Zip')
    $null = Wait-For { Named 'Review installation for 7-Zip' } 'installation review'
    $null = Wait-For { Named 'Review launch for 7-Zip' } 'launch review'
    foreach ($name in @('Appearance Studio','Open offline hub','Scenarios','Local AI','Managed toolbox','Rescue media')) {
        if (!(Named $name)) { throw "Missing workspace feature: $name" }
    }
    $diag = Named 'Diagnostics'
    $diag.GetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern).Select()
    $null = Wait-For { Named 'Capability diagnostics' } 'native diagnostics entry point'
    'Original directory, search, tool action controls and diagnostic navigation passed.'
} finally {
    if (!$p.HasExited) {
        [void]$p.CloseMainWindow()
        if (!$p.WaitForExit(6000)) { $p.Kill() }
    }
    $p.Dispose()
}
