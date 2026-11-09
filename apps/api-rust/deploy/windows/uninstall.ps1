#requires -RunAsAdministrator
$ErrorActionPreference = "Stop"

$installDirectory = Join-Path $env:ProgramFiles "Orbis\API"
$serviceExecutable = Join-Path $installDirectory "orbis-api-service.exe"
$serviceName = "OrbisApi"

$existingService = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($null -ne $existingService) {
  if ($existingService.Status -ne "Stopped") {
    & $serviceExecutable stop
    if ($LASTEXITCODE -ne 0) {
      throw "Could not stop the Orbis API service."
    }
  }
  & $serviceExecutable uninstall
  if ($LASTEXITCODE -ne 0) {
    throw "Could not remove the Orbis API service registration."
  }
}

foreach ($file in @(
  "orbis-server.exe",
  "orbis-api-service.exe",
  "orbis-api-service.xml",
  "VERSION"
)) {
  $path = Join-Path $installDirectory $file
  if (Test-Path -LiteralPath $path) {
    Remove-Item -LiteralPath $path
  }
}

if ((Test-Path -LiteralPath $installDirectory) -and
    -not (Get-ChildItem -LiteralPath $installDirectory -Force)) {
  Remove-Item -LiteralPath $installDirectory
}

Write-Output "Orbis API service and executable removed."
Write-Output "Configuration, logs, and license data were preserved in $env:ProgramData\Orbis."
