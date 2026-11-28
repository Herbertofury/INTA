param([Parameter(Mandatory)][string]$Core, [Parameter(Mandatory)][string]$Model)
$ErrorActionPreference = 'Stop'
$cli = (Resolve-Path -LiteralPath $Core).Path
$models = (& $cli assistant models | ConvertFrom-Json).models
$selected = @($models | Where-Object name -eq $Model)
if ($LASTEXITCODE -or $selected.Count -ne 1) { throw 'The selected local text model must already be prepared and serving.' }
$plan = & $cli ai ask $Model $selected[0].digest 'My computer is running extremely slowly. Find out why.' | ConvertFrom-Json
if ($LASTEXITCODE -or $plan.state -ne 'awaiting-review' -or $plan.executed -ne $false -or $plan.plan.id -ne 'slow-pc') {
    throw 'The local planner did not select the matching performance inspection for review.'
}
$sources = @($plan.knowledge.citations | Where-Object { $_.source -and $_.location -and $_.passage -and $_.sourceTextSha256 -match '^[a-f0-9]{64}$' })
if (!$sources.Count -or !($sources | Where-Object { $_.source -like '*slow-pc*' -or $_.location -like 'job/slow-pc/*' })) {
    throw 'The reviewed plan is missing relevant offline sources.'
}
$null = & $cli ai run-plan slow-pc ('0' * 64) --approve 2>$null
if ($LASTEXITCODE -eq 0) { throw 'A different plan digest was accepted.' }
$result = & $cli ai run-plan slow-pc $plan.planSha256 --approve | ConvertFrom-Json
if ($LASTEXITCODE -or $result.status -ne 'complete' -or $result.id -ne 'slow-pc') { throw 'Approved inspection failed.' }
$history = & $cli scenario checkpoints | ConvertFrom-Json
if ($LASTEXITCODE -or !($history | Where-Object runId -eq $result.runId)) { throw 'Completed inspection missing after restart.' }
[ordered]@{model=$Model; modelDigest=$selected[0].digest; scenario=$result.id; executedDuringPlanning=$plan.executed;
    alteredPlanRejected=$true; checkpointFound=$true; stages=$result.stages} | ConvertTo-Json
