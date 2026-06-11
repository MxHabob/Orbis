# =============================================================================
# Orbis School Server - Startup, Update, and Management CLI
# =============================================================================
# Usage: powershell -ExecutionPolicy Bypass -File start-school-server.ps1 [Action]
# Supported Actions: start, stop, restart, update, status, config, menu
# =============================================================================

param (
    [Parameter(Position=0)]
    [ValidateSet("start", "stop", "restart", "update", "status", "config", "menu")]
    [string]$Action = "menu"
)

$ErrorActionPreference = "Stop"

# Global script settings
$LanIp = "localhost"

# Helper to generate a random string of lowercase letters and numbers
function Get-RandomString($length) {
    $chars = "abcdefghijklmnopqrstuvwxyz0123456789"
    $bytes = New-Object Byte[] $length
    [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
    $result = ""
    for ($i = 0; $i -lt $length; $i++) {
        $result += $chars[$bytes[$i] % $chars.Length]
    }
    return $result
}

# Helper to generate a secure random password with mixed case and digits
function Get-RandomPassword($length) {
    $chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    $bytes = New-Object Byte[] $length
    [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
    $result = ""
    for ($i = 0; $i -lt $length; $i++) {
        $result += $chars[$bytes[$i] % $chars.Length]
    }
    return $result
}

# Print Banner
function Print-Banner {
    Write-Host ""
    Write-Host "    .d88b.  d8888b. d8888b. d888888b .d8888." -ForegroundColor Cyan
    Write-Host "   .8P  Y8. 88  '8D 88  '8D   '88'   88'  YP" -ForegroundColor Cyan
    Write-Host "   88    88 88oobY' 88oooY'    88    '8bo.  " -ForegroundColor Cyan
    Write-Host "   88    88 88'8b   88~~~b.    88      'Y8b." -ForegroundColor Cyan
    Write-Host "   '8b  d8' 88 '88. 88   8D   .88.   db   8D" -ForegroundColor Cyan
    Write-Host "    'Y88P'  88   YD Y8888P' Y888888P '8888Y'" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "   ORBIS School Management Platform CLI" -ForegroundColor White
    Write-Host "   Secure - Scalable - Intelligent [2026 Edition]" -ForegroundColor Gray
    Write-Host ""
}

# Check System Prerequisites (Docker)
function Confirm-Prerequisites {
    Write-Host " [*] Checking System Prerequisites..." -ForegroundColor Yellow
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $null = docker info 2>&1
        $dockerExit = $LASTEXITCODE
    } catch {
        $dockerExit = 1
    }
    $ErrorActionPreference = $oldEA

    if ($dockerExit -ne 0) {
        Write-Host "  [FAIL] Docker is not running. Start Docker Desktop and retry." -ForegroundColor Red
        return $false
    }
    Write-Host "  [OK] Docker engine is running." -ForegroundColor Green
    return $true
}

# Detect Local Network IP
function Detect-LanIp {
    Write-Host " [*] Detecting Local Network IP..." -ForegroundColor Yellow
    $detected = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
        $_.IPAddress -match "^192\.168\." -or
        $_.IPAddress -match "^10\." -or
        $_.IPAddress -match "^172\.(1[6-9]|2[0-9]|3[0-1])\."
    } | Select-Object -First 1).IPAddress

    if (-not $detected) {
        $script:LanIp = "localhost"
        Write-Host "  [WARN] No LAN IP found. Using 'localhost'." -ForegroundColor Yellow
    } else {
        $script:LanIp = $detected
        Write-Host "  [OK] LAN IP: $script:LanIp" -ForegroundColor Green
    }
}

# Initialize and Configure .env
function Initialize-Environment {
    if (-not (Confirm-Prerequisites)) {
        Exit 1
    }
    Detect-LanIp

    Write-Host " [*] Configuring Environment..." -ForegroundColor Yellow

    if (-not (Test-Path ".env")) {
        Write-Host "  Creating .env from template..." -ForegroundColor Gray
        Copy-Item ".env.example" ".env"

        $bytes1 = New-Object Byte[] 32
        $bytes2 = New-Object Byte[] 32
        [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes1)
        [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes2)
        $authSecret = [System.BitConverter]::ToString($bytes1).Replace("-", "").ToLower()
        $encryptSecret = [System.BitConverter]::ToString($bytes2).Replace("-", "").ToLower()

        # Generate secure random database credentials with smart year prefix
        $year = (Get-Date).Year
        $dbUser = "orbis_user_" + (Get-RandomString 8)
        $dbPassword = Get-RandomPassword 24
        $dbName = "orbis_db_" + $year + "_" + (Get-RandomString 8)

        # Generate secure random S3 storage credentials
        $storageAccessKey = "orbis_storage_" + (Get-RandomString 8)
        $storageSecretKey = Get-RandomPassword 24

        $envContent = Get-Content ".env" -Raw
        $envContent = $envContent -replace "generate_a_secure_long_random_hex_string_here_auth", $authSecret
        $envContent = $envContent -replace "generate_a_secure_long_random_hex_string_here_encrypt", $encryptSecret
        $envContent = $envContent -replace "DB_USER=orbis", "DB_USER=$dbUser"
        $envContent = $envContent -replace "DB_PASSWORD=orbis_local_2026", "DB_PASSWORD=$dbPassword"
        $envContent = $envContent -replace "DB_NAME=orbis_db", "DB_NAME=$dbName"
        $envContent = $envContent -replace "STORAGE_ACCESS_KEY=rustfsadmin", "STORAGE_ACCESS_KEY=$storageAccessKey"
        $envContent = $envContent -replace "STORAGE_SECRET_KEY=rustfs_local_2026", "STORAGE_SECRET_KEY=$storageSecretKey"

        Set-Content -Path ".env" -Value $envContent
        Write-Host "  [OK] Secrets and secure database/storage credentials auto-generated." -ForegroundColor Green
    }

    # Update dynamic values in .env
    $envLines = Get-Content ".env"
    $newLines = @()

    foreach ($line in $envLines) {
        if ($line.StartsWith("SERVER_LAN_IP=")) {
            $newLines += "SERVER_LAN_IP=$script:LanIp"
        } elseif ($line.StartsWith("ALLOWED_API_ORIGINS=")) {
            $newLines += "ALLOWED_API_ORIGINS=http://localhost:3001,http://127.0.0.1:3001,http://$($script:LanIp):3001"
        } elseif ($line.StartsWith("BETTER_AUTH_URL=")) {
            $newLines += "BETTER_AUTH_URL=http://$($script:LanIp):3002"
        } elseif ($line.StartsWith("AUTH_TRUSTED_ORIGINS=")) {
            $newLines += "AUTH_TRUSTED_ORIGINS=http://localhost:3001,http://127.0.0.1:3001,http://$($script:LanIp):3001"
        } else {
            if ($line.StartsWith("GHCR_USERNAME=") -and $line -like "*your_github_username_here*") {
                Write-Host "  [WARN] GHCR_USERNAME not set. Edit .env with the correct username." -ForegroundColor Yellow
            }
            $newLines += $line
        }
    }
    Set-Content -Path ".env" -Value $newLines
    Write-Host "  [OK] Environment ready for $script:LanIp" -ForegroundColor Green
}

# Start Server
function Start-Server {
    Initialize-Environment

    Write-Host "`n [*] Starting Docker Containers..." -ForegroundColor Yellow
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $composeOutput = docker compose up -d 2>&1
    $composeExit = $LASTEXITCODE
    $ErrorActionPreference = $oldEA

    $composeOutput | ForEach-Object {
        $line = $_.ToString()
        if ($line -match "Error|403|failed") {
            Write-Host "  $line" -ForegroundColor Red
        } else {
            Write-Host "  $line" -ForegroundColor Gray
        }
    }

    if ($composeExit -ne 0) {
        Write-Host "`n  [FAIL] Docker Compose could not start all services." -ForegroundColor Red
        $authError = $composeOutput | Where-Object { $_ -match "403 Forbidden" -or $_ -match "failed to resolve" }
        if ($authError) {
            Write-Host "`n  Private registry authentication required. Run:" -ForegroundColor Yellow
            Write-Host "    docker login ghcr.io -u YOUR_GITHUB_USERNAME" -ForegroundColor Cyan
        }
        return
    }

    Write-Host "`n  Waiting for services to initialize..." -ForegroundColor Gray
    Start-Sleep -Seconds 5

    Write-Host "`n  =============================================================" -ForegroundColor Green
    Write-Host "  ORBIS SCHOOL SERVER IS RUNNING" -ForegroundColor Green
    Write-Host "  =============================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Local Access:" -ForegroundColor White
    Write-Host "    Dashboard:  http://localhost:3001" -ForegroundColor Cyan
    Write-Host "    API:        http://localhost:3002" -ForegroundColor Cyan
    Write-Host "    Storage:    http://localhost:9001  (rustfsadmin / rustfs_local_2026)" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  LAN Access:" -ForegroundColor White
    Write-Host "    Dashboard:  http://$($script:LanIp):3001" -ForegroundColor Cyan
    Write-Host "  =============================================================" -ForegroundColor Green
}

# Stop Server
function Stop-Server {
    Write-Host " [*] Stopping Docker Containers..." -ForegroundColor Yellow
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    docker compose down 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    $ErrorActionPreference = $oldEA
    Write-Host "  [OK] Server stopped successfully." -ForegroundColor Green
}

# Restart Server
function Restart-Server {
    Write-Host " [*] Restarting Orbis Server..." -ForegroundColor Yellow
    Stop-Server
    Start-Server
}

# Get current setting from .env
function Get-EnvValue($key) {
    if (Test-Path ".env") {
        $line = Get-Content ".env" | Where-Object { $_.StartsWith("$key=") }
        if ($line) {
            return $line.Substring($key.Length + 1).Trim()
        }
    }
    return $null
}

# Update Server Version & Pull
function Update-Server {
    if (-not (Confirm-Prerequisites)) {
        Exit 1
    }

    Write-Host " [*] Starting Orbis Update Procedure..." -ForegroundColor Yellow
    
    $currentVersion = Get-EnvValue "ORBIS_VERSION"
    if (-not $currentVersion) { $currentVersion = "stable" }
    
    Write-Host "  Current Orbis Version: $currentVersion" -ForegroundColor Cyan
    $newVersion = Read-Host "  Enter new Orbis version to deploy [Or press Enter to keep '$currentVersion']"
    
    if ($newVersion) {
        $envLines = Get-Content ".env"
        $newLines = @()
        $found = $false
        foreach ($line in $envLines) {
            if ($line.StartsWith("ORBIS_VERSION=")) {
                $newLines += "ORBIS_VERSION=$newVersion"
                $found = $true
            } else {
                $newLines += $line
            }
        }
        if (-not $found) {
            $newLines += "ORBIS_VERSION=$newVersion"
        }
        Set-Content -Path ".env" -Value $newLines
        Write-Host "  [OK] Version updated to $newVersion in .env" -ForegroundColor Green
        $currentVersion = $newVersion
    }

    Write-Host "`n  Pulling latest Docker images for version '$currentVersion' from GHCR..." -ForegroundColor Gray
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    docker compose pull 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    
    Write-Host "`n  Re-creating containers with updated images..." -ForegroundColor Gray
    docker compose up -d --remove-orphans 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    $ErrorActionPreference = $oldEA

    Write-Host "`n  [OK] Update successfully completed!" -ForegroundColor Green
}

# Show Containers Status
function Show-Status {
    if (-not (Confirm-Prerequisites)) {
        Exit 1
    }
    Write-Host " [*] Checking Containers Status..." -ForegroundColor Yellow
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    docker compose ps
    $ErrorActionPreference = $oldEA

    Write-Host "`n [*] Testing local endpoints:" -ForegroundColor Yellow
    
    # Check Dashboard
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:3001" -Method Head -TimeoutSec 2 -ErrorAction Stop -UseBasicParsing
        Write-Host "  Dashboard (3001): ONLINE (Status: $($resp.StatusCode))" -ForegroundColor Green
    } catch {
        Write-Host "  Dashboard (3001): OFFLINE ($($_.Exception.Message))" -ForegroundColor Red
    }

    # Check API
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 2 -ErrorAction Stop -UseBasicParsing
        Write-Host "  API Health (3002): ONLINE ($($resp.Content.Trim()))" -ForegroundColor Green
    } catch {
        Write-Host "  API Health (3002): OFFLINE ($($_.Exception.Message))" -ForegroundColor Red
    }
}

# Show current configs
function Show-Config {
    Write-Host " [*] Current configurations (.env):" -ForegroundColor Yellow
    if (-not (Test-Path ".env")) {
        Write-Host "  [WARN] .env file does not exist yet. Run start action to initialize it." -ForegroundColor Yellow
        return
    }

    $lines = Get-Content ".env"
    foreach ($line in $lines) {
        $trimmed = $line.Trim()
        if ($trimmed -and -not $trimmed.StartsWith("#")) {
            # Hide sensitive keys
            if ($trimmed -match "SECRET|PASSWORD|_KEY") {
                $parts = $trimmed.Split("=", 2)
                Write-Host "  $($parts[0])=********" -ForegroundColor Gray
            } else {
                Write-Host "  $trimmed" -ForegroundColor Cyan
            }
        }
    }
}

# Interactive Menu Loop
function Show-InteractiveMenu {
    do {
        Clear-Host
        Print-Banner
        Write-Host "  =============================================================" -ForegroundColor Green
        Write-Host "                    ORBIS SERVER CLI MANAGER" -ForegroundColor Green
        Write-Host "  =============================================================" -ForegroundColor Green
        Write-Host "    1. Start Server (Check environment and launch)"
        Write-Host "    2. Stop Server (Gracefully halt services)"
        Write-Host "    3. Restart Server"
        Write-Host "    4. Update Server (Pull latest images & deploy)"
        Write-Host "    5. Check Status (Show health of containers)"
        Write-Host "    6. View Configuration (.env settings)"
        Write-Host "    7. Exit"
        Write-Host "  =============================================================" -ForegroundColor Green
        Write-Host ""
        
        $choice = Read-Host "  Select an option [1-7]"
        Write-Host ""
        
        switch ($choice) {
            "1" { Start-Server }
            "2" { Stop-Server }
            "3" { Restart-Server }
            "4" { Update-Server }
            "5" { Show-Status }
            "6" { Show-Config }
            "7" { Write-Host "  Goodbye!"; break }
            default { Write-Host "  [ERROR] Invalid option, try again." -ForegroundColor Red }
        }
        
        if ($choice -ne "7") {
            Write-Host ""
            Read-Host "  Press Enter to return to menu..."
        }
    } while ($choice -ne "7")
}

# Execution Dispatcher
Clear-Host
Print-Banner

switch ($Action) {
    "start"   { Start-Server }
    "stop"    { Stop-Server }
    "restart" { Restart-Server }
    "update"  { Update-Server }
    "status"  { Show-Status }
    "config"  { Show-Config }
    "menu"    { Show-InteractiveMenu }
}
