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
    $null = Wait-For { Named 'Review phone repair draft...' } 'the native Mobile draft review entry point'
    # Windows PowerShell wraps method arguments inside PSObject when invoking
    # reflection directly. Use a typed C# boundary with ordinary string/row values.
    Add-Type -TypeDefinition @'
using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Reflection;
public static class IntaPackagedMobileDraftCheck {
    private static IEnumerable Open(MethodInfo reader, string path) {
        return (IEnumerable)reader.Invoke(null, new object[] { path });
    }
    public static string[] Labels(MethodInfo reader, string path) {
        var result = new List<string>();
        foreach (object row in Open(reader, path)) {
            var field = row.GetType().GetField("Label", BindingFlags.NonPublic | BindingFlags.Instance);
            result.Add((string)field.GetValue(row));
        }
        return result.ToArray();
    }
    public static bool Refuses(MethodInfo reader, string path) {
        try { Open(reader, path); return false; }
        catch (TargetInvocationException error) {
            if (error.InnerException is InvalidDataException) return true;
            throw;
        }
    }
}
'@
    $binding = [Reflection.BindingFlags]'NonPublic,Static'
    $reader = [Reflection.Assembly]::LoadFile($app).GetType('Inta.Desktop.MobileDraftReader',$true)
    $read = $reader.GetMethod('ReadFile',$binding)
    $draftPath = Join-Path $env:RUNNER_TEMP ('intamobile-draft-' + [Guid]::NewGuid().ToString('N') + '.txt')
    $dash = [char]0x2014
    $report = "INTA repair plan draft`nNot approved. Nothing in this document runs or grants access.`n`n1. Run a malware scan $dash Review results`n2. Apply an optimization $dash Observe startup timing`n"
    try {
        [IO.File]::WriteAllText($draftPath,$report,([Text.UTF8Encoding]::new($false)))
        $original = (Get-FileHash -LiteralPath $draftPath -Algorithm SHA256).Hash
        $labels = @([IntaPackagedMobileDraftCheck]::Labels($read,[string]$draftPath))
        if ($labels.Count -ne 2 -or $labels[0] -cne 'Review results' -or $labels[1] -cne 'Observe startup timing') { throw 'The packaged reader lost draft order' }
        if ((Get-FileHash -LiteralPath $draftPath -Algorithm SHA256).Hash -cne $original) { throw 'Review changed the selected draft file' }
        [IO.File]::WriteAllText($draftPath,($report.Replace('2. Apply','3. Apply')),([Text.UTF8Encoding]::new($false)))
        if (![IntaPackagedMobileDraftCheck]::Refuses($read,[string]$draftPath)) { throw 'The packaged reader accepted a malformed order' }
    } finally {
        Remove-Item -LiteralPath $draftPath -ErrorAction SilentlyContinue
    }
    'Original directory, search, tool actions, diagnostic navigation, and strict Mobile draft readback passed.'
} finally {
    if (!$p.HasExited) {
        [void]$p.CloseMainWindow()
        if (!$p.WaitForExit(6000)) { $p.Kill() }
    }
    $p.Dispose()
}
