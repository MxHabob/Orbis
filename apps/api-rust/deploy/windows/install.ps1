#requires -RunAsAdministrator
$ErrorActionPreference = "Stop"

$packageDirectory = $PSScriptRoot
$installDirectory = Join-Path $env:ProgramFiles "Orbis\API"
$dataDirectory = Join-Path $env:ProgramData "Orbis"
$licenseDirectory = Join-Path $dataDirectory "license"
$logsDirectory = Join-Path $dataDirectory "logs"
$configPath = Join-Path $dataDirectory "server.json"
$serviceExecutable = Join-Path $installDirectory "orbis-api-service.exe"
$serviceName = "OrbisApi"

foreach ($file in @(
  "orbis-server.exe",
  "orbis-api-service.exe",
  "orbis-api-service.xml",
  "VERSION",
  "server.example.json"
)) {
  if (-not (Test-Path -LiteralPath (Join-Path $packageDirectory $file) -PathType Leaf)) {
    throw "Required package file is missing: $file"
  }
}

New-Item -ItemType Directory -Force -Path $installDirectory, $dataDirectory, $licenseDirectory, $logsDirectory | Out-Null

$existingService = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($null -ne $existingService) {
  if ($existingService.Status -ne "Stopped") {
    & $serviceExecutable stop
    if ($LASTEXITCODE -ne 0) {
      throw "Could not stop the existing Orbis API service."
    }
  }
  & $serviceExecutable uninstall
  if ($LASTEXITCODE -ne 0) {
    throw "Could not remove the existing Orbis API service registration."
  }
}

Copy-Item -LiteralPath (Join-Path $packageDirectory "orbis-server.exe") -Destination $installDirectory -Force
Copy-Item -LiteralPath (Join-Path $packageDirectory "orbis-api-service.exe") -Destination $installDirectory -Force
Copy-Item -LiteralPath (Join-Path $packageDirectory "orbis-api-service.xml") -Destination $installDirectory -Force
Copy-Item -LiteralPath (Join-Path $packageDirectory "VERSION") -Destination $installDirectory -Force

if (-not (Test-Path -LiteralPath $configPath)) {
  Copy-Item -LiteralPath (Join-Path $packageDirectory "server.example.json") -Destination $configPath
}

& icacls.exe $dataDirectory /inheritance:r /grant:r `
  "NT AUTHORITY\SYSTEM:(OI)(CI)F" `
  "BUILTIN\Administrators:(OI)(CI)F" `
  "NT AUTHORITY\LOCAL SERVICE:(OI)(CI)RX"
if ($LASTEXITCODE -ne 0) {
  throw "Could not secure the Orbis data directory permissions."
}

& icacls.exe $licenseDirectory /grant:r "NT AUTHORITY\LOCAL SERVICE:(OI)(CI)M"
if ($LASTEXITCODE -ne 0) {
  throw "Could not grant the Orbis service access to its license data directory."
}

& icacls.exe $logsDirectory /grant:r "NT AUTHORITY\LOCAL SERVICE:(OI)(CI)M"
if ($LASTEXITCODE -ne 0) {
  throw "Could not grant the Orbis service access to its log directory."
}

& icacls.exe $configPath /inheritance:r /grant:r `
  "NT AUTHORITY\SYSTEM:F" `
  "BUILTIN\Administrators:F" `
  "NT AUTHORITY\LOCAL SERVICE:R"
if ($LASTEXITCODE -ne 0) {
  throw "Could not secure the Orbis database configuration file."
}

& $serviceExecutable install
if ($LASTEXITCODE -ne 0) {
  throw "Could not register the Orbis API service."
}

$version = (Get-Content -LiteralPath (Join-Path $installDirectory "VERSION") -Raw).Trim()
Write-Output "Orbis API $version installed."
Write-Output "Configure $configPath with the existing Orbis database URL."
Write-Output "After the database schema and migrations are ready, start the service with:"
Write-Output "  Start-Service -Name $serviceName"
