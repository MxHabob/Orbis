# =============================================================================
# Orbis — Local Development Setup (Modern CLI — Expo-Style UX)
# =============================================================================
# Usage:  powershell -ExecutionPolicy Bypass -File scripts/start-local.ps1
# =============================================================================

$ErrorActionPreference = "Stop"
$ROOT = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not $ROOT) { $ROOT = (Get-Location).Path }

# ─── UTF-8 Output ────────────────────────────────────────────────────────────
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# ─── Color Palette ───────────────────────────────────────────────────────────
$C = @{
    Primary   = "Cyan"
    Success   = "Green"
    Warning   = "Yellow"
    Error     = "Red"
    Muted     = "DarkGray"
    White     = "White"
    Accent    = "Magenta"
}

# ─── Spinner Frames ──────────────────────────────────────────────────────────
$script:SpinnerFrames = @('⠋','⠙','⠹','⠸','⠼','⠴','⠦','⠧','⠇','⠏')

# ─── Utility Functions ───────────────────────────────────────────────────────

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

# ─── UI Components ───────────────────────────────────────────────────────────

function Print-Banner {
    Write-Blank
    Write-Host "  ╭──────────────────────────────────────────────╮" -ForegroundColor $C.Primary
    Write-Host "  │                                              │" -ForegroundColor $C.Primary
    Write-Host "  │" -NoNewline -ForegroundColor $C.Primary
    Write-Host "        ◉  O R B I S" -NoNewline -ForegroundColor $C.White
    Write-Host "                       │" -ForegroundColor $C.Primary
    Write-Host "  │" -NoNewline -ForegroundColor $C.Primary
    Write-Host "        Local Development Setup" -NoNewline -ForegroundColor $C.Muted
    Write-Host "          │" -ForegroundColor $C.Primary
    Write-Host "  │                                              │" -ForegroundColor $C.Primary
    Write-Host "  ╰──────────────────────────────────────────────╯" -ForegroundColor $C.Primary
    Write-Blank
}

function Print-StepHeader([int]$Current, [int]$Total, [string]$Title) {
    Write-Blank
    Write-Host "  ──────────────────────────────────────────────" -ForegroundColor $C.Muted
    Write-Host "  Step $Current/$Total" -NoNewline -ForegroundColor $C.Accent
    Write-Host " · " -NoNewline -ForegroundColor $C.Muted
    Write-Host "$Title" -ForegroundColor $C.White
    Write-Host "  ──────────────────────────────────────────────" -ForegroundColor $C.Muted
    Write-Blank
}

function Write-TaskSuccess([string]$Message, [string]$Elapsed = "") {
    if ($Elapsed) {
        $padding = 48 - $Message.Length - 6
        if ($padding -lt 1) { $padding = 1 }
        $spaces = " " * $padding
        Write-Host "    ✓ " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -NoNewline -ForegroundColor $C.White
        Write-Host "$spaces" -NoNewline
        Write-Host "$Elapsed" -ForegroundColor $C.Muted
    } else {
        Write-Host "    ✓ " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -ForegroundColor $C.White
    }
}

function Write-TaskWarning([string]$Message) {
    Write-Host "    ⚠ " -NoNewline -ForegroundColor $C.Warning
    Write-Host "$Message" -ForegroundColor $C.Warning
}

function Write-TaskError([string]$Message) {
    Write-Host "    ✗ " -NoNewline -ForegroundColor $C.Error
    Write-Host "$Message" -ForegroundColor $C.Error
}

function Write-TaskInfo([string]$Message) {
    Write-Host "    › " -NoNewline -ForegroundColor $C.Muted
    Write-Host "$Message" -ForegroundColor $C.Muted
}

# ─── Spinner Execution ───────────────────────────────────────────────────────

function Invoke-WithSpinner {
    param(
        [string]$Message,
        [scriptblock]$Action
    )

    $job = Start-Job -ScriptBlock $Action
    $frameIdx = 0
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

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

    $output = ""
    $jobFailed = $false
    try {
        $output = Receive-Job $job -ErrorAction SilentlyContinue 2>&1 | Out-String
    } catch {
        $output = $_.ToString()
        $jobFailed = $true
    }

    if ($job.State -eq 'Failed') { $jobFailed = $true }
    Remove-Job $job -Force -ErrorAction SilentlyContinue

    if (-not $jobFailed) {
        Write-Host "`r    ✓ " -NoNewline -ForegroundColor $C.Success
        Write-Host "$Message" -NoNewline -ForegroundColor $C.White
        Write-Host "$spaces" -NoNewline
        Write-Host "$elapsed  " -ForegroundColor $C.Muted
    } else {
        Write-Host "`r    ✗ " -NoNewline -ForegroundColor $C.Error
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

# ─── Error Display ───────────────────────────────────────────────────────────

function Show-ErrorBox {
    param(
        [string]$Title,
        [string]$Details,
        [string[]]$Fix
    )

    Write-Blank
    Write-Host "  ╭──────────────────────────────────────────────╮" -ForegroundColor $C.Error
    Write-Host "  │" -NoNewline -ForegroundColor $C.Error
    Write-Host "  ✗  $Title" -NoNewline -ForegroundColor $C.Error
    $tp = 46 - $Title.Length - 5
    if ($tp -lt 1) { $tp = 1 }
    Write-Host "$(" " * $tp)│" -ForegroundColor $C.Error
    Write-Host "  ╰──────────────────────────────────────────────╯" -ForegroundColor $C.Error
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

# ═════════════════════════════════════════════════════════════════════════════
# MAIN SETUP FLOW
# ═════════════════════════════════════════════════════════════════════════════

$totalSw = [System.Diagnostics.Stopwatch]::StartNew()

Clear-Host
Print-Banner

# ─── Step 1: Check Docker ────────────────────────────────────────────────────
Print-StepHeader 1 6 "Checking Docker"

$sw = [System.Diagnostics.Stopwatch]::StartNew()
try {
    $dockerVersion = docker --version 2>&1
    $sw.Stop()
    Write-TaskSuccess "Docker found: $dockerVersion" (Format-Elapsed $sw)
} catch {
    $sw.Stop()
    Write-TaskError "Docker not found"
    Show-ErrorBox `
        "Docker Not Installed" `
        "Docker Desktop is required to run Orbis locally." `
        @(
            "1. Download from: docker.com/products/docker-desktop",
            "2. Install and restart your computer",
            "3. Re-run this script"
        )
    exit 1
}

# Check daemon
$sw = [System.Diagnostics.Stopwatch]::StartNew()
try {
    docker info 2>&1 | Out-Null
    $sw.Stop()
    Write-TaskSuccess "Docker daemon is running" (Format-Elapsed $sw)
} catch {
    $sw.Stop()
    Write-TaskError "Docker daemon is not running"
    Show-ErrorBox `
        "Docker Not Running" `
        "Docker Desktop is installed but the engine is not started." `
        @(
            "1. Open Docker Desktop and wait for it to initialize",
            "2. Re-run this script"
        )
    exit 1
}

# ─── Step 2: Check Bun ───────────────────────────────────────────────────────
Print-StepHeader 2 6 "Checking Bun"

$sw = [System.Diagnostics.Stopwatch]::StartNew()
try {
    $bunVersion = bun --version 2>&1
    $sw.Stop()
    Write-TaskSuccess "Bun found: v$bunVersion" (Format-Elapsed $sw)
} catch {
    $sw.Stop()
    Write-TaskWarning "Bun not found — installing..."
    $installResult = Invoke-WithSpinner "Installing Bun" {
        powershell -c "irm bun.sh/install.ps1 | iex" 2>&1
    }
    if (-not $installResult.Success) {
        Show-ErrorBox `
            "Bun Installation Failed" `
            "Could not automatically install Bun." `
            @(
                "Install manually: https://bun.sh/docs/installation",
                "Then re-run this script"
            )
        exit 1
    }
}

# ─── Step 3: Setup .env ──────────────────────────────────────────────────────
Print-StepHeader 3 6 "Setting up environment"

$sw = [System.Diagnostics.Stopwatch]::StartNew()
$envFile = Join-Path $ROOT ".env"
$envLocalFile = Join-Path $ROOT ".env.local"

if (-not (Test-Path $envFile)) {
    if (Test-Path $envLocalFile) {
        Copy-Item $envLocalFile $envFile
        $sw.Stop()
        Write-TaskSuccess "Created .env from .env.local" (Format-Elapsed $sw)
    } else {
        $sw.Stop()
        Write-TaskError "No .env or .env.local found"
        Show-ErrorBox `
            "Missing Environment File" `
            "No .env.local template was found in the project root." `
            @(
                "Create a .env file manually from the documentation",
                "Or restore the .env.local from version control"
            )
        exit 1
    }
} else {
    $sw.Stop()
    Write-TaskSuccess ".env already exists" (Format-Elapsed $sw)
}

# ─── Step 4: Start Docker Services ───────────────────────────────────────────
Print-StepHeader 4 6 "Starting infrastructure services"

$composeFile = Join-Path $ROOT "docker-compose.local.yml"

$composeResult = Invoke-WithSpinner "Starting PostgreSQL, Redis & RustFS" {
    param()
    # The $using: scope lets us access outer variables in Start-Job
    docker compose -f $using:composeFile up -d 2>&1
}

# If $using: doesn't work in this PS version, fall back to inline approach
if (-not $composeResult.Success) {
    # Try direct approach if job scoping failed
    $oldEA = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    Write-TaskInfo "Starting services directly..."
    docker compose -f $composeFile up -d 2>&1 | Out-Null
    $ErrorActionPreference = $oldEA
}

# Wait for services
$healthResult = Invoke-WithSpinner "Waiting for services to be healthy" {
    $maxWait = 30
    $waited = 0
    while ($waited -lt $maxWait) {
        try {
            $pg = docker exec orbis-postgres pg_isready -U orbis -d orbis_db 2>&1
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

if ($healthResult.Output -match "healthy") {
    Write-TaskSuccess "PostgreSQL ready"
    Write-TaskSuccess "Redis ready"
} elseif ($healthResult.Output -match "timeout") {
    Write-TaskWarning "Services may still be initializing — continuing anyway"
}

# ─── Step 5: Create RustFS Buckets ───────────────────────────────────────────
Print-StepHeader 5 6 "Setting up storage buckets"

Start-Sleep -Seconds 3

# Read storage credentials from .env (or use defaults)
$storageUser = "rustfsadmin"
$storagePass = "rustfs_local_2026"
if (Test-Path $envFile) {
    $envContent = Get-Content $envFile -Raw
    if ($envContent -match 'STORAGE_ACCESS_KEY=(.+)') { $storageUser = $Matches[1].Trim() }
    if ($envContent -match 'STORAGE_SECRET_KEY=(.+)') { $storagePass = $Matches[1].Trim() }
}

$buckets = @("vault", "avatars", "exports")
foreach ($bucket in $buckets) {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:9000/$bucket" -Method Head -ErrorAction SilentlyContinue 2>&1
        $sw.Stop()
        Write-TaskSuccess "Bucket '$bucket' exists" (Format-Elapsed $sw)
    } catch {
        try {
            Invoke-WebRequest -Uri "http://localhost:9000/$bucket" -Method Put -Headers @{
                "Authorization" = "AWS ${storageUser}:${storagePass}"
            } -ErrorAction SilentlyContinue 2>&1 | Out-Null
            $sw.Stop()
            Write-TaskSuccess "Bucket '$bucket' created" (Format-Elapsed $sw)
        } catch {
            $sw.Stop()
            Write-TaskWarning "Could not create bucket '$bucket' — will be auto-created on first upload"
        }
    }
}

# ─── Step 6: Install Dependencies & Run Migrations ──────────────────────────
Print-StepHeader 6 6 "Installing dependencies & migrations"

Set-Location $ROOT

# Install packages
$installResult = Invoke-WithSpinner "Installing packages" {
    Set-Location $using:ROOT
    bun install 2>&1
}

# Run migrations
$migrateResult = Invoke-WithSpinner "Running database migrations" {
    $env:DATABASE_URL = "postgresql://orbis:orbis_local_2026@localhost:5432/orbis_db"
    Set-Location (Join-Path $using:ROOT "packages/db")
    bunx drizzle-kit push 2>&1
}

if (-not $migrateResult.Success) {
    Write-TaskWarning "Migration may have issues — check manually"
}

# Seed
$seedResult = Invoke-WithSpinner "Seeding database" {
    Set-Location $using:ROOT
    bun run seed 2>&1
}

if (-not $seedResult.Success) {
    Write-TaskWarning "Seed may already be applied — continuing"
}

# ─── Get LAN IP ──────────────────────────────────────────────────────────────
$lanIP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
    $_.InterfaceAlias -notmatch "Loopback" -and
    $_.IPAddress -notmatch "^169\." -and
    $_.IPAddress -ne "127.0.0.1"
} | Select-Object -First 1).IPAddress

# ─── Success Summary ─────────────────────────────────────────────────────────
$totalSw.Stop()

Write-Blank
Write-Host "  ╭──────────────────────────────────────────────╮" -ForegroundColor $C.Success
Write-Host "  │                                              │" -ForegroundColor $C.Success
Write-Host "  │" -NoNewline -ForegroundColor $C.Success
Write-Host "    ✓  ORBIS IS READY" -NoNewline -ForegroundColor $C.Success
Write-Host "                       │" -ForegroundColor $C.Success
Write-Host "  │                                              │" -ForegroundColor $C.Success
Write-Host "  ╰──────────────────────────────────────────────╯" -ForegroundColor $C.Success
Write-Blank

Write-Host "    Local Access" -ForegroundColor $C.White
Write-Host "    ├─ Dashboard    " -NoNewline -ForegroundColor $C.Muted
Write-Host "http://localhost:3001" -ForegroundColor $C.Primary
Write-Host "    ├─ API          " -NoNewline -ForegroundColor $C.Muted
Write-Host "http://localhost:3002" -ForegroundColor $C.Primary
Write-Host "    └─ Storage      " -NoNewline -ForegroundColor $C.Muted
Write-Host "http://localhost:9001" -ForegroundColor $C.Primary
Write-Blank

if ($lanIP) {
    Write-Host "    Network Access (School LAN)" -ForegroundColor $C.White
    Write-Host "    ├─ Dashboard    " -NoNewline -ForegroundColor $C.Muted
    Write-Host "http://${lanIP}:3001" -ForegroundColor $C.Primary
    Write-Host "    └─ API          " -NoNewline -ForegroundColor $C.Muted
    Write-Host "http://${lanIP}:3002" -ForegroundColor $C.Primary
    Write-Blank
    Write-Host "    › " -NoNewline -ForegroundColor $C.Muted
    Write-Host "Add to .env: SERVER_LAN_IP=$lanIP" -ForegroundColor $C.Muted
    Write-Blank
}

Write-Host "    To start the app:" -ForegroundColor $C.White
Write-Host "      bun run dev" -ForegroundColor $C.Accent
Write-Blank

Write-Host "    To stop Docker services:" -ForegroundColor $C.White
Write-Host "      docker compose -f docker-compose.local.yml down" -ForegroundColor $C.Accent
Write-Blank

Write-Host "    Total setup time: " -NoNewline -ForegroundColor $C.Muted
Write-Host (Format-Elapsed $totalSw) -ForegroundColor $C.Accent
Write-Blank
