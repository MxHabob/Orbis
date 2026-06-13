# =============================================================================
# Orbis School Server - Modern CLI Manager (Expo-Style UX)
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
Set-Location $PSScriptRoot

# --- Global Settings ---------------------------------------------------------
$script:LanIp = "localhost"
$script:TotalStopwatch = $null

# --- ANSI / Unicode Helpers --------------------------------------------------
# Ensure UTF-8 output for box-drawing and Unicode symbols
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# --- Color Palette -----------------------------------------------------------
$C = @{
    Primary   = "Cyan"
    Success   = "Green"
    Warning   = "Yellow"
    Error     = "Red"
    Muted     = "DarkGray"
    White     = "White"
    Accent    = "Magenta"
}

# --- Spinner Frames (Braille dots - same style as Expo/Ora) ------------------
$script:SpinnerFrames = @('+','x','+','x','+','x','+','x','+','x')

# --- Utility Functions -------------------------------------------------------

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

function Format-Elapsed([System.Diagnostics.Stopwatch]$sw) {
    $totalSec = $sw.Elapsed.TotalSeconds
    if ($totalSec -lt 1) {
        return "$([math]::Round($totalSec * 1000))ms"
    } elseif ($totalSec -lt 60) {
        return "$([math]::Round($totalSec, 1))s"
    } else {
        $min = [math]::Floor($totalSec / 60)
        $sec = [math]::Round($totalSec % 60)
        return "${min}m ${sec}s"
    }
}

function Write-Blank { Write-Host "" }

# --- UI Components -----------------------------------------------------------

function Print-Banner {
    Write-Blank

    # Get OS Name
    $osName = "Windows"
    try {
        $osName = (Get-CimInstance Win32_OperatingSystem).Caption
        $osName = $osName -replace "Microsoft ", ""
    } catch {
        try {
            $osName = [System.Environment]::OSVersion.VersionString
        } catch {}
    }

    # Get Shell
    $shell = "PowerShell " + $PSVersionTable.PSVersion.ToString()

    # Get LAN IP
    $lanIp = $script:LanIp

    # Get Status
    $statusText = "Stopped"
    $statusColor = $C.Warning
    if ($null -ne (Get-Command docker -ErrorAction SilentlyContinue)) {
        $oldEA = $ErrorActionPreference
        $ErrorActionPreference = "SilentlyContinue"
        try {
            docker info >$null 2>&1
            if ($LASTEXITCODE -eq 0) {
                $running = docker compose ps --services --filter "status=running" 2>&1
                if ($running -and $running.Count -gt 0) {
                    $statusText = "Running ($($running.Count) services)"
                    $statusColor = $C.Success
                } else {
                    $statusText = "Online (Idle)"
                    $statusColor = $C.Success
                }
            } else {
                $statusText = "Docker Offline"
                $statusColor = $C.Warning
            }
        } catch {
            $statusText = "Error Checking"
            $statusColor = $C.Error
        }
        $ErrorActionPreference = $oldEA
    } else {
        $statusText = "Docker Missing"
        $statusColor = $C.Error
    }

    $cwd = (Get-Location).Path
    if ($cwd.Length -gt 30) {
        $cwd = "..." + $cwd.Substring($cwd.Length - 27)
    }

    # Logo rows (simple ASCII block style for encoding safety)
    $logo = @(
        "  ORBIS   ___  ____  ____ ___ ____  ",
        "         / _ \|  _ \| __ )_ _/ ___| ",
        "        | | | | |_) |  _ \| |\___ \ ",
        "        | |_| |  _ <| |_) | | ___) |",
        "         \___/|_| \_\____/___|____/ ",
        "                                     "
    )

    # Info rows
    $info = @(
        @{ Label = "Platform"; Value = "Orbis School Server"; Color = $C.White },
        @{ Label = "OS      "; Value = $osName; Color = $C.White },
        @{ Label = "Shell   "; Value = $shell; Color = $C.White },
        @{ Label = "LAN IP  "; Value = $lanIp; Color = $C.Primary },
        @{ Label = "Status  "; Value = $statusText; Color = $statusColor },
        @{ Label = "Location"; Value = $cwd; Color = $C.Muted }
    )

    for ($i = 0; $i -lt 6; $i++) {
        Write-Host " $($logo[$i])" -NoNewline -ForegroundColor $C.Primary
        Write-Host "   " -NoNewline
        Write-Host "$($info[$i].Label) : " -NoNewline -ForegroundColor $C.Muted
        Write-Host "$($info[$i].Value)" -ForegroundColor $info[$i].Color
    }

    Write-Blank
}

function Print-StepHeader([int]$Current, [int]$Total, [string]$Title) {
    Write-Blank
    Write-Host "  ──────────────────────────────────────────────" -ForegroundColor $C.Muted
    Write-Host "  Step $Current/$Total" -NoNewline -ForegroundColor $C.Accent
    Write-Host " - " -NoNewline -ForegroundColor $C.Muted
    Write-Host "$Title" -ForegroundColor $C.White
    Write-Host "  ──────────────────────────────────────────────" -ForegroundColor $C.Muted
    Write-Blank
}

function Write-TaskSuccess([string]$Message, [string]$Elapsed = "") {
    if ($Elapsed) {
        $padding = 48 - $Message.Length - 6
        if ($padding -lt 1) { $padding = 1 }
        $spaces = " " * $padding
        Write-Host "    [OK] " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -NoNewline -ForegroundColor $C.White
        Write-Host "$spaces" -NoNewline
        Write-Host "$Elapsed" -ForegroundColor $C.Muted
    } else {
        Write-Host "    [OK] " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -ForegroundColor $C.White
    }
}

function Write-TaskWarning([string]$Message) {
    Write-Host "    [!!] " -NoNewline -ForegroundColor $C.Warning
    Write-Host "$Message" -ForegroundColor $C.Warning
}

function Write-TaskError([string]$Message) {
    Write-Host "    [XX] " -NoNewline -ForegroundColor $C.Error
    Write-Host "$Message" -ForegroundColor $C.Error
}

function Write-TaskInfo([string]$Message) {
    Write-Host "    [..] " -NoNewline -ForegroundColor $C.Muted
    Write-Host "$Message" -ForegroundColor $C.Muted
}

# --- Spinner Execution -------------------------------------------------------
# Run a scriptblock in a background job while showing an animated spinner.
# The background job is initialized in $PSScriptRoot so docker compose can
# find docker-compose.yml and .env relative paths correctly.
# Returns: @{ Success = $bool; Output = $string; Elapsed = $string }

function Invoke-WithSpinner {
    param(
        [string]$Message,
        [scriptblock]$Action
    )

    $currentPath = $PSScriptRoot
    $initBlock = ([scriptblock]::Create("Set-Location '$currentPath'"))
    $job = Start-Job -InitializationScript $initBlock -ScriptBlock $Action
    $frameIdx = 0
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

    # Animate spinner while job is running
    while ($job.State -eq 'Running') {
        $frame = $script:SpinnerFrames[$frameIdx % $script:SpinnerFrames.Count]
        $elapsed = Format-Elapsed $sw
        $padding = 48 - $Message.Length - 6
        if ($padding -lt 1) { $padding = 1 }
        $spaces = " " * $padding
        Write-Host "`r    $frame " -NoNewline -ForegroundColor $C.Primary
        Write-Host "$Message" -NoNewline -ForegroundColor $C.White
        Write-Host "$spaces" -NoNewline
        Write-Host "$elapsed  " -NoNewline -ForegroundColor $C.Muted
        Start-Sleep -Milliseconds 100
        $frameIdx++
    }

    $sw.Stop()
    $elapsed = Format-Elapsed $sw
    $padding = 48 - $Message.Length - 6
    if ($padding -lt 1) { $padding = 1 }
    $spaces = " " * $padding

    # Collect output and determine success
    $output = ""
    $jobFailed = $false
    try {
        $output = Receive-Job $job -ErrorAction SilentlyContinue 2>&1 | Out-String
    } catch {
        $output = $_.ToString()
        $jobFailed = $true
    }

    if ($job.State -eq 'Failed' -or ($job.ChildJobs -and $job.ChildJobs[0].Error.Count -gt 0)) {
        $jobFailed = $true
        # Capture error details from child jobs for diagnostics
        if ($job.ChildJobs -and $job.ChildJobs[0].Error.Count -gt 0) {
            $errorDetails = $job.ChildJobs[0].Error | ForEach-Object { $_.ToString() }
            if (-not $output) {
                $output = $errorDetails -join "`n"
            }
        }
    }
    Remove-Job $job -Force -ErrorAction SilentlyContinue

    if (-not $jobFailed) {
        Write-Host "`r    [OK] " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -NoNewline -ForegroundColor $C.White
        Write-Host "$spaces" -NoNewline
        Write-Host "$elapsed  " -ForegroundColor $C.Muted
    } else {
        Write-Host "`r    [XX] " -NoNewline -ForegroundColor $C.Error
        Write-Host "$Message" -NoNewline -ForegroundColor $C.Error
        Write-Host "$spaces" -NoNewline
        Write-Host "$elapsed  " -ForegroundColor $C.Muted
    }

    return @{
        Success  = (-not $jobFailed)
        Output   = $output.Trim()
        Elapsed  = $elapsed
    }
}

# --- Error Display -----------------------------------------------------------

function Show-ErrorBox {
    param(
        [string]$Title,
        [string]$Details,
        [string[]]$Fix
    )

    Write-Blank
    Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Error
    Write-Host "  |" -NoNewline -ForegroundColor $C.Error
    Write-Host "  [XX]  $Title" -NoNewline -ForegroundColor $C.Error
    $tp = 46 - $Title.Length - 7
    if ($tp -lt 1) { $tp = 1 }
    Write-Host "$(" " * $tp)|" -ForegroundColor $C.Error
    Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Error
    Write-Blank

    if ($Details) {
        Write-Host "    What went wrong:" -ForegroundColor $C.Warning
        Write-Host "      $Details" -ForegroundColor $C.White
        Write-Blank
    }

    if ($Fix -and $Fix.Count -gt 0) {
        Write-Host "    How to fix:" -ForegroundColor $C.Success
        foreach ($step in $Fix) {
            Write-Host "      $step" -ForegroundColor $C.White
        }
        Write-Blank
    }
}

# --- Success Summary ---------------------------------------------------------

function Show-SuccessSummary {
    param([string]$TotalElapsed)

    Write-Blank
    Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Success
    Write-Host "  |                                              |" -ForegroundColor $C.Success
    Write-Host "  |" -NoNewline -ForegroundColor $C.Success
    Write-Host "    [OK]  ORBIS SERVER IS RUNNING" -NoNewline -ForegroundColor $C.Success
    Write-Host "            |" -ForegroundColor $C.Success
    Write-Host "  |                                              |" -ForegroundColor $C.Success
    Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Success
    Write-Blank

    Write-Host "    Local Access" -ForegroundColor $C.White
    Write-Host "    +-- Dashboard    " -NoNewline -ForegroundColor $C.Muted
    Write-Host "http://localhost:3001" -ForegroundColor $C.Primary
    Write-Host "    +-- API          " -NoNewline -ForegroundColor $C.Muted
    Write-Host "http://localhost:3002" -ForegroundColor $C.Primary
    Write-Host "    +-- Storage      " -NoNewline -ForegroundColor $C.Muted
    Write-Host "http://localhost:9001" -ForegroundColor $C.Primary
    Write-Blank

    if ($script:LanIp -ne "localhost") {
        Write-Host "    Network Access (LAN)" -ForegroundColor $C.White
        Write-Host "    +-- Dashboard    " -NoNewline -ForegroundColor $C.Muted
        Write-Host "http://$($script:LanIp):3001" -ForegroundColor $C.Primary
        Write-Blank
    }

    if ($TotalElapsed) {
        Write-Host "    Total startup time: " -NoNewline -ForegroundColor $C.Muted
        Write-Host "$TotalElapsed" -ForegroundColor $C.Accent
        Write-Blank
    }
}

# =============================================================================
# CORE FUNCTIONS
# =============================================================================

# --- Prerequisites Check -----------------------------------------------------

function Confirm-Prerequisites {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $null = docker info 2>&1
        $dockerExit = $LASTEXITCODE
    } catch {
        $dockerExit = 1
    }
    $ErrorActionPreference = $oldEA
    $sw.Stop()

    if ($dockerExit -ne 0) {
        Write-TaskError "Docker is not running"
        Show-ErrorBox `
            "Docker Desktop Required" `
            "Docker engine is not running or not installed." `
            @(
                "1. Install Docker Desktop from docker.com/products/docker-desktop",
                "2. Start Docker Desktop and wait for it to initialize",
                "3. Re-run this script"
            )
        return $false
    }
    Write-TaskSuccess "Docker engine is running" (Format-Elapsed $sw)
    return $true
}

# --- LAN IP Detection --------------------------------------------------------

function Detect-LanIp {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $detected = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
        $_.IPAddress -match "^192\.168\." -or
        $_.IPAddress -match "^10\." -or
        $_.IPAddress -match "^172\.(1[6-9]|2[0-9]|3[0-1])\."
    } | Select-Object -First 1).IPAddress

    $sw.Stop()

    if (-not $detected) {
        $script:LanIp = "localhost"
        Write-TaskWarning "No LAN IP found - using localhost"
    } else {
        $script:LanIp = $detected
        Write-TaskSuccess "LAN IP detected: $script:LanIp" (Format-Elapsed $sw)
    }
}

# --- Environment Configuration -----------------------------------------------

function Initialize-Environment {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

    if (-not (Test-Path ".env")) {
        Write-TaskInfo "Creating .env from template..."
        Copy-Item ".env.example" ".env"

        $bytes1 = New-Object Byte[] 32
        $bytes2 = New-Object Byte[] 32
        [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes1)
        [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes2)
        $authSecret = [System.BitConverter]::ToString($bytes1).Replace("-", "").ToLower()
        $encryptSecret = [System.BitConverter]::ToString($bytes2).Replace("-", "").ToLower()

        $year = (Get-Date).Year
        $dbUser = "orbis_user_" + (Get-RandomString 8)
        $dbPassword = Get-RandomPassword 24
        $dbName = "orbis_db_" + $year + "_" + (Get-RandomString 8)

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
        Write-TaskSuccess "Secrets auto-generated" ""
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
            $newLines += $line
        }
    }
    Set-Content -Path ".env" -Value $newLines

    $sw.Stop()
    Write-TaskSuccess "Environment configured" (Format-Elapsed $sw)
}

# --- Get .env Value ----------------------------------------------------------

function Get-EnvValue($key) {
    if (Test-Path ".env") {
        $line = Get-Content ".env" | Where-Object { $_.StartsWith("$key=") }
        if ($line) {
            return $line.Substring($key.Length + 1).Trim()
        }
    }
    return $null
}

# =============================================================================
# SERVER ACTIONS
# =============================================================================

# --- Start Server ------------------------------------------------------------

function Start-Server {
    $script:TotalStopwatch = [System.Diagnostics.Stopwatch]::StartNew()

    # -- Step 1: Prerequisites ------------------------------------------------
    Print-StepHeader 1 4 "Checking prerequisites"

    if (-not (Confirm-Prerequisites)) { return }
    Detect-LanIp

    # -- Step 2: Environment --------------------------------------------------
    Print-StepHeader 2 4 "Configuring environment"

    Initialize-Environment

    # Check GHCR_USERNAME
    $ghcrUser = Get-EnvValue "GHCR_USERNAME"
    if (-not $ghcrUser -or $ghcrUser -eq "your_github_username_here") {
        Write-TaskWarning "GHCR_USERNAME not set - edit .env with the correct username"
    } else {
        Write-TaskSuccess "Registry: ghcr.io/$ghcrUser"
    }

    # -- Step 3: Pull Images --------------------------------------------------
    Print-StepHeader 3 4 "Pulling container images"

    $pullResult = Invoke-WithSpinner "Pulling latest images from registry" {
        $ErrorActionPreference = "Continue"
        $output = docker compose pull --quiet 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose pull failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if (-not $pullResult.Success) {
        $output = $pullResult.Output
        if ($output -match "403|Forbidden|unauthorized|denied") {
            Show-ErrorBox `
                "Registry Authentication Failed" `
                "Access denied to private container registry." `
                @(
                    "1. Run: docker login ghcr.io -u YOUR_GITHUB_USERNAME",
                    "2. Enter your Personal Access Token (PAT) with read:packages scope",
                    "3. Re-run this script"
                )
        } elseif ($output -match "not found|manifest unknown") {
            Show-ErrorBox `
                "Container Image Not Found" `
                "The specified version could not be found in the registry." `
                @(
                    "1. Check ORBIS_VERSION in your .env file",
                    "2. Verify the images exist at ghcr.io/$ghcrUser",
                    "3. Try setting ORBIS_VERSION=stable"
                )
        } else {
            Show-ErrorBox `
                "Image Pull Failed" `
                "Could not download container images." `
                @(
                    "Check your internet connection and try again.",
                    "Run: docker compose pull  (for detailed output)"
                )
            if ($output) {
                Write-TaskInfo "Details: $output"
            }
        }
        return
    }

    # -- Step 4: Start Services -----------------------------------------------
    Print-StepHeader 4 4 "Starting services"

    $startResult = Invoke-WithSpinner "Starting Docker containers" {
        $ErrorActionPreference = "Continue"
        $output = docker compose up -d 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose up failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if (-not $startResult.Success) {
        $output = $startResult.Output
        if ($output -match "port is already allocated|address already in use") {
            Show-ErrorBox `
                "Port Conflict" `
                "One or more required ports are already in use." `
                @(
                    "Check which ports are busy: netstat -ano | findstr :3001",
                    "Stop conflicting services or change ports in docker-compose.yml",
                    "Then re-run this script"
                )
        } else {
            Show-ErrorBox `
                "Failed to Start Services" `
                "Docker Compose could not start all containers." `
                @(
                    "Run: docker compose up -d  (for detailed output)",
                    "Check logs: docker compose logs"
                )
            if ($output) {
                Write-TaskInfo "Details: $output"
            }
        }
        return
    }

    # Wait for health checks
    $healthResult = Invoke-WithSpinner "Waiting for services to be healthy" {
        Start-Sleep -Seconds 6
        # Quick health check
        $maxWait = 30
        $waited = 0
        while ($waited -lt $maxWait) {
            try {
                $pg = docker exec orbis-postgres pg_isready -U orbis 2>&1
                $rd = docker exec orbis-redis redis-cli ping 2>&1
                if (($pg -match "accepting") -and ($rd -match "PONG")) {
                    return "healthy"
                }
            } catch {}
            Start-Sleep -Seconds 2
            $waited += 2
        }
        return "timeout"
    }

    if ($healthResult.Output -match "timeout") {
        Write-TaskWarning "Services may still be initializing - check status in a moment"
    }

    # -- Summary --------------------------------------------------------------
    $script:TotalStopwatch.Stop()
    Show-SuccessSummary (Format-Elapsed $script:TotalStopwatch)
}

# --- Stop Server -------------------------------------------------------------

function Stop-Server {
    Print-StepHeader 1 1 "Stopping server"

    $result = Invoke-WithSpinner "Stopping all containers" {
        $ErrorActionPreference = "Continue"
        $output = docker compose down 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose down failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if ($result.Success) {
        Write-Blank
        Write-Host "    [OK] " -NoNewline -ForegroundColor $C.Success
        Write-Host "Server stopped successfully" -ForegroundColor $C.White
        Write-Blank
    } else {
        Write-TaskError "Could not stop containers cleanly"
        Write-TaskInfo "Run: docker compose down  (for detailed output)"
        if ($result.Output) {
            Write-TaskInfo "Details: $($result.Output)"
        }
        Write-Blank
    }
}

# --- Restart Server ----------------------------------------------------------

function Restart-Server {
    $script:TotalStopwatch = [System.Diagnostics.Stopwatch]::StartNew()

    Print-StepHeader 1 2 "Stopping current services"

    $stopResult = Invoke-WithSpinner "Stopping containers" {
        $ErrorActionPreference = "Continue"
        $output = docker compose down 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose down failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if (-not $stopResult.Success) {
        Write-TaskWarning "Could not stop containers cleanly - attempting restart anyway"
    }

    Print-StepHeader 2 2 "Starting services"

    $startResult = Invoke-WithSpinner "Starting Docker containers" {
        $ErrorActionPreference = "Continue"
        $output = docker compose up -d 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose up failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if ($startResult.Success) {
        # Quick wait for initialization
        $healthResult = Invoke-WithSpinner "Waiting for services to be healthy" {
            Start-Sleep -Seconds 5
            $maxWait = 20
            $waited = 0
            while ($waited -lt $maxWait) {
                try {
                    $pg = docker exec orbis-postgres pg_isready -U orbis 2>&1
                    $rd = docker exec orbis-redis redis-cli ping 2>&1
                    if (($pg -match "accepting") -and ($rd -match "PONG")) {
                        return "healthy"
                    }
                } catch {}
                Start-Sleep -Seconds 2
                $waited += 2
            }
            return "timeout"
        }

        Detect-LanIp
        $script:TotalStopwatch.Stop()
        Show-SuccessSummary (Format-Elapsed $script:TotalStopwatch)
    } else {
        Show-ErrorBox `
            "Restart Failed" `
            "Could not restart Docker containers." `
            @(
                "Run: docker compose up -d  (for detailed output)",
                "Check logs: docker compose logs"
            )
        if ($startResult.Output) {
            Write-TaskInfo "Details: $($startResult.Output)"
        }
    }
}

# --- Update Server -----------------------------------------------------------

function Update-Server {
    $script:TotalStopwatch = [System.Diagnostics.Stopwatch]::StartNew()

    # -- Step 1: Prerequisites ------------------------------------------------
    Print-StepHeader 1 3 "Checking prerequisites"

    if (-not (Confirm-Prerequisites)) { return }

    $currentVersion = Get-EnvValue "ORBIS_VERSION"
    if (-not $currentVersion) { $currentVersion = "stable" }
    Write-TaskInfo "Current version: $currentVersion"

    # -- Version Prompt -------------------------------------------------------
    Write-Blank
    Write-Host "    Enter new version " -NoNewline -ForegroundColor $C.White
    Write-Host "[press Enter for '$currentVersion']" -NoNewline -ForegroundColor $C.Muted
    Write-Host ": " -NoNewline -ForegroundColor $C.White
    $newVersion = Read-Host

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
        Write-TaskSuccess "Version updated to $newVersion"
        $currentVersion = $newVersion
    } else {
        Write-TaskInfo "Keeping version: $currentVersion"
    }

    # -- Step 2: Pull Images --------------------------------------------------
    Print-StepHeader 2 3 "Pulling updated images"

    $pullResult = Invoke-WithSpinner "Pulling version '$currentVersion' from registry" {
        $ErrorActionPreference = "Continue"
        $output = docker compose pull --quiet 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose pull failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if (-not $pullResult.Success) {
        $output = $pullResult.Output
        if ($output -match "403|Forbidden|unauthorized|denied") {
            Show-ErrorBox `
                "Registry Authentication Failed" `
                "Access denied when pulling updated images." `
                @(
                    "1. Run: docker login ghcr.io -u YOUR_GITHUB_USERNAME",
                    "2. Enter your Personal Access Token (PAT)",
                    "3. Re-run: start-school-server.ps1 update"
                )
        } else {
            Show-ErrorBox `
                "Pull Failed" `
                "Could not download updated images." `
                @(
                    "Check your internet connection and try again.",
                    "Verify version '$currentVersion' exists in the registry."
                )
            if ($output) {
                Write-TaskInfo "Details: $output"
            }
        }
        return
    }

    # -- Step 3: Redeploy -----------------------------------------------------
    Print-StepHeader 3 3 "Redeploying services"

    $deployResult = Invoke-WithSpinner "Re-creating containers with updated images" {
        $ErrorActionPreference = "Continue"
        $output = docker compose up -d --remove-orphans 2>&1
        if ($LASTEXITCODE -ne 0) { throw "Docker compose up failed (exit code $LASTEXITCODE): $output" }
        $output
    }

    if ($deployResult.Success) {
        $script:TotalStopwatch.Stop()

        Write-Blank
        Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Success
        Write-Host "  |" -NoNewline -ForegroundColor $C.Success
        Write-Host "    [OK]  UPDATE COMPLETED SUCCESSFULLY" -NoNewline -ForegroundColor $C.Success
        Write-Host "       |" -ForegroundColor $C.Success
        Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Success
        Write-Blank
        Write-Host "    Version: " -NoNewline -ForegroundColor $C.Muted
        Write-Host "$currentVersion" -ForegroundColor $C.Accent
        Write-Host "    Total time: " -NoNewline -ForegroundColor $C.Muted
        Write-Host (Format-Elapsed $script:TotalStopwatch) -ForegroundColor $C.Accent
        Write-Blank
    } else {
        Show-ErrorBox `
            "Redeploy Failed" `
            "Containers could not be updated." `
            @(
                "Run: docker compose up -d --remove-orphans  (for detailed output)",
                "Check logs: docker compose logs"
            )
        if ($deployResult.Output) {
            Write-TaskInfo "Details: $($deployResult.Output)"
        }
    }
}

# --- Show Status -------------------------------------------------------------

function Show-Status {
    Print-StepHeader 1 2 "Container health"

    if (-not (Confirm-Prerequisites)) { return }

    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    # Get container status
    $containers = docker compose ps --format "table {{.Name}}\t{{.Status}}" 2>&1
    $ErrorActionPreference = $oldEA

    if ($containers) {
        foreach ($containerLine in $containers) {
            $lineStr = $containerLine.ToString().Trim()

            # Skip the header line
            if ($lineStr -match "^NAME" -or -not $lineStr) { continue }

            # Split by whitespace (tab-separated output)
            $parts = $lineStr -split '\s{2,}', 2
            if ($parts.Count -ge 2) {
                $name = $parts[0].Trim()
                $status = $parts[1].Trim()

                # Determine icon based on status
                if ($status -match "Up|running|healthy") {
                    Write-Host "    [*] " -NoNewline -ForegroundColor $C.Success
                    Write-Host "$name" -NoNewline -ForegroundColor $C.White
                    $sp = 30 - $name.Length
                    if ($sp -lt 1) { $sp = 1 }
                    Write-Host "$(" " * $sp)" -NoNewline
                    Write-Host "$status" -ForegroundColor $C.Muted
                } elseif ($status -match "Exit|stopped|dead") {
                    Write-Host "    [o] " -NoNewline -ForegroundColor $C.Error
                    Write-Host "$name" -NoNewline -ForegroundColor $C.Error
                    $sp = 30 - $name.Length
                    if ($sp -lt 1) { $sp = 1 }
                    Write-Host "$(" " * $sp)" -NoNewline
                    Write-Host "$status" -ForegroundColor $C.Muted
                } else {
                    Write-Host "    [~] " -NoNewline -ForegroundColor $C.Warning
                    Write-Host "$name" -NoNewline -ForegroundColor $C.Warning
                    $sp = 30 - $name.Length
                    if ($sp -lt 1) { $sp = 1 }
                    Write-Host "$(" " * $sp)" -NoNewline
                    Write-Host "$status" -ForegroundColor $C.Muted
                }
            }
        }
    } else {
        Write-TaskWarning "No containers found - server may not be running"
    }

    # -- Step 2: Endpoint Tests -----------------------------------------------
    Print-StepHeader 2 2 "Endpoint health"

    # Dashboard
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:3001" -Method Head -TimeoutSec 3 -ErrorAction Stop -UseBasicParsing
        Write-TaskSuccess "Dashboard (3001): Online - Status $($resp.StatusCode)"
    } catch {
        Write-TaskError "Dashboard (3001): Offline"
    }

    # API
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 3 -ErrorAction Stop -UseBasicParsing
        Write-TaskSuccess "API Health (3002): Online"
    } catch {
        Write-TaskError "API Health (3002): Offline"
    }

    # Storage
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:9001" -Method Head -TimeoutSec 3 -ErrorAction Stop -UseBasicParsing
        Write-TaskSuccess "Storage Console (9001): Online"
    } catch {
        Write-TaskError "Storage Console (9001): Offline"
    }

    Write-Blank
}

# --- Show Config -------------------------------------------------------------

function Show-Config {
    Print-StepHeader 1 1 "Current configuration"

    if (-not (Test-Path ".env")) {
        Write-TaskWarning ".env file not found - run start to initialize"
        return
    }

    $lines = Get-Content ".env"
    $currentSection = ""

    foreach ($line in $lines) {
        $trimmed = $line.Trim()

        # Section headers
        if ($trimmed -match "^#\s*\d+\.\s*(.+)$") {
            $currentSection = $Matches[1].Trim()
            Write-Blank
            Write-Host "    $currentSection" -ForegroundColor $C.Accent
            Write-Host "    $("-" * $currentSection.Length)" -ForegroundColor $C.Muted
            continue
        }

        # Skip other comments and empty lines
        if (-not $trimmed -or $trimmed.StartsWith("#")) { continue }

        # Key=Value pairs
        if ($trimmed -match "^([^=]+)=(.*)$") {
            $key = $Matches[1]
            $value = $Matches[2]

            # Mask sensitive values
            if ($key -match "SECRET|PASSWORD|_KEY") {
                $maskedValue = "********"
            } else {
                $maskedValue = $value
            }

            Write-Host "      $key" -NoNewline -ForegroundColor $C.Muted
            Write-Host " = " -NoNewline -ForegroundColor $C.Muted
            Write-Host "$maskedValue" -ForegroundColor $C.White
        }
    }
    Write-Blank
}

# =============================================================================
# INTERACTIVE MENU
# =============================================================================

function Show-InteractiveMenu {
    do {
        Clear-Host
        Print-Banner

        Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Muted
        Write-Host "  |" -NoNewline -ForegroundColor $C.Muted
        Write-Host "        ORBIS SERVER MANAGER" -NoNewline -ForegroundColor $C.White
        Write-Host "                 |" -ForegroundColor $C.Muted
        Write-Host "  +----------------------------------------------+" -ForegroundColor $C.Muted
        Write-Blank

        # Menu items with text icons
        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "1" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  >>  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Start Server" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "2" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  []  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Stop Server" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "3" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  <>  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Restart Server" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "4" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  \/  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Update Server" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "5" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  ()  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Check Status" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "6" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  {}  " -NoNewline -ForegroundColor $C.Muted
        Write-Host "View Configuration" -ForegroundColor $C.White

        Write-Host "    [" -NoNewline -ForegroundColor $C.Muted
        Write-Host "7" -NoNewline -ForegroundColor $C.Primary
        Write-Host "]  x   " -NoNewline -ForegroundColor $C.Muted
        Write-Host "Exit" -ForegroundColor $C.Muted

        Write-Blank
        Write-Host "  ──────────────────────────────────────────────" -ForegroundColor $C.Muted
        Write-Host "    Select an option " -NoNewline -ForegroundColor $C.White
        Write-Host "[1-7]" -NoNewline -ForegroundColor $C.Accent
        Write-Host ": " -NoNewline -ForegroundColor $C.White
        $choice = Read-Host
        Write-Blank

        switch ($choice) {
            "1" { Start-Server }
            "2" { Stop-Server }
            "3" { Restart-Server }
            "4" { Update-Server }
            "5" { Show-Status }
            "6" { Show-Config }
            "7" {
                Write-Host "    (*) " -NoNewline -ForegroundColor $C.Primary
                Write-Host "Goodbye!" -ForegroundColor $C.White
                Write-Blank
                break
            }
            default {
                Write-TaskError "Invalid option - please select 1-7"
            }
        }

        if ($choice -ne "7") {
            Write-Blank
            Write-Host "    Press " -NoNewline -ForegroundColor $C.Muted
            Write-Host "Enter" -NoNewline -ForegroundColor $C.White
            Write-Host " to return to menu..." -NoNewline -ForegroundColor $C.Muted
            Read-Host
        }
    } while ($choice -ne "7")
}

# =============================================================================
# ENTRY POINT
# =============================================================================

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
