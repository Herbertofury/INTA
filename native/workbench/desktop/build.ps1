[CmdletBinding()]
param([Parameter(Mandatory)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
$framework = Join-Path $env:WINDIR 'Microsoft.NET\Framework64\v4.0.30319'
$compiler = Join-Path $framework 'csc.exe'
if (!(Test-Path -LiteralPath $compiler)) { throw 'Install the Windows .NET Framework 4.x compiler to build INTA Desktop.' }
$output = [IO.Directory]::CreateDirectory($OutputDirectory).FullName
$refs = @('System.dll', 'System.Core.dll', 'System.Xaml.dll', 'System.Web.Extensions.dll', 'System.Windows.Forms.dll', 'WPF\WindowsBase.dll', 'WPF\PresentationCore.dll', 'WPF\PresentationFramework.dll', 'WPF\UIAutomationTypes.dll', 'WPF\UIAutomationProvider.dll')
$arguments = @('/nologo', '/codepage:65001', '/optimize+', '/warnaserror+', '/platform:x64', '/target:winexe', "/out:$output\inta-desktop.exe", "/win32manifest:$PSScriptRoot\app.manifest", "/resource:$PSScriptRoot\Workspace.xaml,Workspace.xaml")
$arguments += "/win32icon:$PSScriptRoot\inta.ico"
$arguments += "/resource:$(Join-Path $PSScriptRoot '..\..\..\assets\inta-brand\application-64.png'),INTA.Application.png"
$registry = Join-Path $output 'tool-mentions.json'
& python -I (Join-Path $PSScriptRoot 'mentions.py') $registry
if ($LASTEXITCODE) { throw 'Failed to compile the catalog mention registry.' }
$arguments += "/resource:$registry,ToolCatalog.json"
$arguments += $refs | ForEach-Object { "/reference:$(Join-Path $framework $_)" }
$arguments += (Get-ChildItem -LiteralPath $PSScriptRoot -Filter '*.cs' | Sort-Object Name).FullName
$arguments += (Join-Path $PSScriptRoot '..\..\desktop\interop\PrismField.cs')
$arguments += (Join-Path $PSScriptRoot '..\..\desktop\interop\FavoriteTweaks.cs')
& $compiler @arguments
if ($LASTEXITCODE -ne 0) { throw "Desktop compilation failed with exit $LASTEXITCODE." }
Get-Item -LiteralPath (Join-Path $output 'inta-desktop.exe')
