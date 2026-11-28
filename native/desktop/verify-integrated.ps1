param([Parameter(Mandatory)][string]$Payload, [Parameter(Mandatory)][string]$StateRoot,
      [Parameter(Mandatory)][string]$ExampleManifest)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$dir = (Resolve-Path -LiteralPath $Payload).Path
$state = [IO.Path]::GetFullPath($StateRoot)
if ([IO.Directory]::Exists($state) -or [IO.File]::Exists($state)) { throw 'Choose a new disposable state directory.' }
$null = [IO.Directory]::CreateDirectory($state)
$env:INTA_DATA_DIR = $state
$source = (Resolve-Path -LiteralPath $ExampleManifest).Path
$app = Join-Path $dir 'inta-desktop.exe'
$helper = Join-Path $dir 'inta-views.exe'
foreach ($name in 'inta-desktop.exe','inta.exe','inta-capabilities.exe','inta_optics.dll','inta-views.exe') {
    if (!(Test-Path -LiteralPath (Join-Path $dir $name) -PathType Leaf)) { throw "Missing payload file $name" }
}
$existing = @(Get-Process -Name inta-views -ErrorAction SilentlyContinue | Where-Object Path -eq $helper | ForEach-Object Id)
$service = [Windows.Automation.AutomationElement]
$scope = [Windows.Automation.TreeScope]::Descendants
function Until([scriptblock]$check,[string]$why){
    $ends=[DateTime]::UtcNow.AddSeconds(28)
    do { $v=&$check; if($null -ne $v -and $v -ne $false){return $v}; Start-Sleep -Milliseconds 140 } while([DateTime]::UtcNow -lt $ends)
    throw $why
}
function Named($window,[string]$name){ $window.FindFirst($scope,[Windows.Automation.PropertyCondition]::new($service::NameProperty,$name)) }
function Invoke-ItemControl($root,[string]$name){$c=Until {Named $root $name} "Missing $name"; $c.GetCurrentPattern([Windows.Automation.InvokePattern]::Pattern).Invoke()}
function Toggle($root,[string]$name){$c=Until {Named $root $name} "Missing $name"; $c.GetCurrentPattern([Windows.Automation.TogglePattern]::Pattern).Toggle()}
function Section($root,[string]$name){
    $matches=$root.FindAll($scope,[Windows.Automation.PropertyCondition]::new($service::NameProperty,$name));
    foreach($item in $matches){$pattern=$null;if($item.TryGetCurrentPattern([Windows.Automation.SelectionItemPattern]::Pattern,[ref]$pattern)){$pattern.Select();return}}
    throw "Missing workspace navigation section $name"
}
function View([string]$title){
    $process=Until {Get-Process -Name inta-views -ErrorAction SilentlyContinue | Where-Object {$_.Path -eq $helper -and $_.Id -notin $existing -and $_.MainWindowTitle -like "$title * INTA Desktop"}|Select-Object -First 1} "Original workspace failed to open $title"
    $service::FromHandle($process.MainWindowHandle)
}
function Close-View($element){$proc=Get-Process -Id $element.Current.ProcessId; $null=$proc.CloseMainWindow();if(!$proc.WaitForExit(7000)){throw 'Feature view did not close.'}}
function Text-Matches($root,[string]$pattern){
    $all=$root.FindAll($scope,[Windows.Automation.PropertyCondition]::new($service::ControlTypeProperty,[Windows.Automation.ControlType]::Text));
    foreach($c in $all){if($c.Current.Name -match $pattern){return $true}}return $false
}
function Read-Appearance([string]$path){
    $f=[IO.File]::Open($path,[IO.FileMode]::Open,[IO.FileAccess]::Read,([IO.FileShare]::ReadWrite -bor [IO.FileShare]::Delete));
    try{$reader=[IO.StreamReader]::new($f);try{$reader.ReadToEnd()|ConvertFrom-Json}finally{$reader.Dispose()}}finally{$f.Dispose()}
}
function Result($root,[string]$text){$null=Until {Text-Matches $root $text} "Missing observed result $text";Invoke-ItemControl $root 'Close result';$null=Until {!(Named $root 'Close result')} 'Result did not close.'}

foreach($argsCase in @(@('--view','overview'),@('--view','forge','--run'))){$bad=Start-Process $helper -ArgumentList $argsCase -PassThru -WindowStyle Hidden;if(!$bad.WaitForExit(5000) -or $bad.ExitCode -ne 2){throw 'An invalid view invocation opened a window.'}}
$hostProcess=Start-Process -FilePath $app -WorkingDirectory $dir -PassThru
try {
    $root=Until {$hostProcess.Refresh();if($hostProcess.MainWindowHandle -ne 0){$service::FromHandle($hostProcess.MainWindowHandle)}} 'Primary workspace did not open'
    Invoke-ItemControl $root 'Appearance Studio';$studio=View 'Appearance Studio'
    Toggle $studio 'Deep Focus';Toggle $studio 'Reduce motion'
    $saved=Join-Path $state 'appearance.json'
    $null=Until {if(Test-Path -LiteralPath $saved){$s=Read-Appearance $saved; $s.preset -eq 2 -and $s.reduced_motion}} 'Isolated preferences were not saved'
    $scale=$studio.FindFirst($scope,[Windows.Automation.AndCondition]::new([Windows.Automation.PropertyCondition]::new($service::ControlTypeProperty,[Windows.Automation.ControlType]::Slider),[Windows.Automation.PropertyCondition]::new($service::NameProperty,'Interface scale')))
    $scale.GetCurrentPattern([Windows.Automation.RangeValuePattern]::Pattern).SetValue(125)
    $null=Until {(Read-Appearance $saved).zoom -eq 125} 'Interface scale did not persist'
    Close-View $studio; Invoke-ItemControl $root 'Appearance Studio';$studio=View 'Appearance Studio'
    if((Named $studio 'Deep Focus').GetCurrentPattern([Windows.Automation.TogglePattern]::Pattern).Current.ToggleState -ne [Windows.Automation.ToggleState]::On){throw 'Preset lost after reopening'}
    Invoke-ItemControl $studio 'Restore defaults';$null=Until {(Read-Appearance $saved).preset -eq 0} 'Restore failed';Close-View $studio
    Section $root 'Diagnostics';Invoke-ItemControl $root 'Capability diagnostics';$diag=View 'Diagnostics'; Invoke-ItemControl $diag 'Run operation';Result $diag 'availablePhysicalBytes';Close-View $diag
    Section $root 'Managed toolbox';Invoke-ItemControl $root 'CLI Forge';$forge=View 'Capability Forge'
    Invoke-ItemControl $forge ('Choose manifest'+[char]0x2026)
    $dialog=Until {
        $all=$service::RootElement.FindAll([Windows.Automation.TreeScope]::Children,[Windows.Automation.PropertyCondition]::new($service::ProcessIdProperty,$forge.Current.ProcessId));
        foreach($window in $all){if($window.Current.ClassName -eq '#32770'){return $window}}
        $forge.FindFirst($scope,[Windows.Automation.PropertyCondition]::new($service::ClassNameProperty,'#32770'))
    } 'Native file chooser did not open'
    $edit=$dialog.FindFirst($scope,[Windows.Automation.AndCondition]::new([Windows.Automation.PropertyCondition]::new($service::ControlTypeProperty,[Windows.Automation.ControlType]::Edit),[Windows.Automation.PropertyCondition]::new($service::AutomationIdProperty,'1148')))
    if(!$edit){$edit=$dialog.FindFirst($scope,[Windows.Automation.PropertyCondition]::new($service::NameProperty,'File name:'))}
    $edit.GetCurrentPattern([Windows.Automation.ValuePattern]::Pattern).SetValue($source);Invoke-ItemControl $dialog 'Open'
    Invoke-ItemControl $forge 'Verify manifest';Result $forge 'testsPassed'
    Invoke-ItemControl $forge ('Import manifest'+[char]0x2026);Invoke-ItemControl $forge 'Cancel'
    if(Test-Path (Join-Path $state 'forge')){throw 'Cancelled import created a store'}
    Invoke-ItemControl $forge ('Import manifest'+[char]0x2026);Invoke-ItemControl $forge 'Continue';Result $forge 'manifestSha256'
    Invoke-ItemControl $forge ('Review run'+[char]0x2026);Invoke-ItemControl $forge 'Continue';Result $forge 'testsPassed'; Close-View $forge
    if (!(Get-ChildItem -LiteralPath (Join-Path $state 'forge') -Recurse -File | Select-Object -First 1)){throw 'Imported capability missing'}
    'Original workspace opened all three native views. Isolated persistence, interface scaling, actual system memory, manifest verification, cancel, import, run, and closure passed.'
} finally {
    Get-Process -Name inta-views -ErrorAction SilentlyContinue | Where-Object {$_.Path -eq $helper -and $_.Id -notin $existing} |ForEach-Object {$null=$_.CloseMainWindow();if(!$_.WaitForExit(4000)){Stop-Process -Id $_.Id}}
    $hostProcess.Refresh();if(!$hostProcess.HasExited){$null=$hostProcess.CloseMainWindow();if(!$hostProcess.WaitForExit(4000)){Stop-Process -Id $hostProcess.Id}}
}
