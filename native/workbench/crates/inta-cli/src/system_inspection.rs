//! Read-only local provider queries. These strings never contain user input.
use anyhow::{Result, bail};
use serde_json::Value;
pub fn run(kind: &str) -> Result<Value> {
    let query = match kind {
        "applications" => {
            r#"
$path='Software\Microsoft\Windows\CurrentVersion\Uninstall'
$seen=@{}; $denied=0
foreach ($hive in 'LocalMachine','CurrentUser') {
  foreach ($view in 'Registry64','Registry32') {
    $base=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]$hive,[Microsoft.Win32.RegistryView]$view)
    try {
      $entries=$base.OpenSubKey($path)
      if (!$entries) { continue }
      try { foreach ($name in $entries.GetSubKeyNames()) {
        try {
          $key=$entries.OpenSubKey($name)
          if (!$key) { continue }
          try {
            $title=[string]$key.GetValue('DisplayName')
            if (!$title -or $key.GetValue('SystemComponent') -eq 1) { continue }
            $version=[string]$key.GetValue('DisplayVersion'); $publisher=[string]$key.GetValue('Publisher')
            $id=$hive+'|'+$name+'|'+$title+'|'+$version
            if (!$seen.ContainsKey($id)) { $seen[$id]=[pscustomobject]@{application=$title;version=$version;publisher=$publisher;scope=$hive} }
          } finally { $key.Close() }
        } catch { $denied++ }
      } } finally { $entries.Close() }
    } finally { $base.Close() }
  }
}
[pscustomobject]@{source='Windows uninstall registry';registeredApplications=$seen.Count;entriesUnavailable=$denied;displayLimit=2000;scope='Visible registered applications. Portable apps and Store-only registrations may be absent.'}
$seen.Values | Sort-Object application,version,scope | Select-Object -First 2000
"#
        }
        "updates" => {
            r#"
$reboot=Test-Path -LiteralPath 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Component Based Servicing\RebootPending'
$wu=Test-Path -LiteralPath 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Auto Update\RebootRequired'
$session=Get-Item -LiteralPath 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager'
$rename=$session.GetValue('PendingFileRenameOperations')
[pscustomobject]@{source='Windows servicing registry';componentServicingRebootPending=$reboot;windowsUpdateRebootRequired=$wu;pendingFileRenames=[bool]$rename;scope='Local indicators. No online check for available updates.'}
try {
  $fixes=@(Get-CimInstance Win32_QuickFixEngineering)
  [pscustomobject]@{source='Win32_QuickFixEngineering';status='observed';reportedHotfixes=$fixes.Count;scope='Component-based servicing records only.'}
} catch { [pscustomobject]@{source='Win32_QuickFixEngineering';status='unavailable'} }
"#
        }
        "power" => {
            r#"
try {
  $plan=Get-CimInstance -Namespace 'root\cimv2\power' -ClassName Win32_PowerPlan -Filter 'IsActive=True'
  if (!$plan) {throw 'unavailable'}
  [pscustomobject]@{source='Active power plan';status='observed';name=$plan.ElementName}
} catch {
  $scheme=& ([IO.Path]::Combine([Environment]::SystemDirectory,'powercfg.exe')) /getactivescheme
  if ($LASTEXITCODE -eq 0 -and $scheme) { [pscustomobject]@{source='PowerCfg active scheme';status='observed';activeScheme=[string]$scheme} }
  else { [pscustomobject]@{source='Active power plan';status='unavailable'} }
}
try {
  $cells=@(Get-CimInstance Win32_Battery)
  [pscustomobject]@{source='Win32_Battery';status='observed';batteryCount=$cells.Count}
  foreach($cell in $cells){[pscustomobject]@{source='Battery';chargeRemainingPercent=$cell.EstimatedChargeRemaining;batteryStatusCode=$cell.BatteryStatus}}
} catch { [pscustomobject]@{source='Win32_Battery';status='unavailable'} }
"#
        }
        _ => bail!("Unknown system inspection"),
    };
    crate::inspect::run_query(kind, query)
}
