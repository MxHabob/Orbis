# =============================================================================
# Orbis — Local Setup & Start Script (Windows PowerShell)
# =============================================================================
# Usage:  powershell -ExecutionPolicy Bypass -File scripts/start-local.ps1
# =============================================================================

$ErrorActionPreference = "Stop"
$ROOT = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not $ROOT) { $ROOT = (Get-Location).Path }

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "          Orbis School System — Local Setup" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# ─── Step 1: Check Docker ────────────────────────────────────────────────────
Write-Host "[1/6] Checking Docker..." -ForegroundColor Yellow
try {
    $dockerVersion = docker --version 2>&1
    Write-Host "  ✓ Docker found: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Docker not found! Please install Docker Desktop." -ForegroundColor Red
    Write-Host "  Download: https://www.docker.com/products/docker-desktop/" -ForegroundColor Gray
    exit 1
}

# Check if Docker daemon is running
try {
    docker info 2>&1 | Out-Null
    Write-Host "  ✓ Docker daemon is running" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Docker daemon is not running! Please start Docker Desktop." -ForegroundColor Red
    exit 1
}

# ─── Step 2: Check Bun ───────────────────────────────────────────────────────
Write-Host "[2/6] Checking Bun..." -ForegroundColor Yellow
try {
    $bunVersion = bun --version 2>&1
    Write-Host "  ✓ Bun found: v$bunVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Bun not found! Installing..." -ForegroundColor Red
    powershell -c "irm bun.sh/install.ps1 | iex"
}

# ─── Step 3: Setup .env ──────────────────────────────────────────────────────
Write-Host "[3/6] Setting up environment..." -ForegroundColor Yellow
$envFile = Join-Path $ROOT ".env"
$envLocalFile = Join-Path $ROOT ".env.local"

if (-not (Test-Path $envFile)) {
    if (Test-Path $envLocalFile) {
        Copy-Item $envLocalFile $envFile
        Write-Host "  ✓ Created .env from .env.local" -ForegroundColor Green
    } else {
        Write-Host "  ✗ No .env.local found!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✓ .env already exists" -ForegroundColor Green
}

# ─── Step 4: Start Docker Services ───────────────────────────────────────────
Write-Host "[4/6] Starting Docker services (PostgreSQL + Redis + RustFS)..." -ForegroundColor Yellow
$composeFile = Join-Path $ROOT "docker-compose.local.yml"

docker compose -f $composeFile up -d 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }

# Wait for services to be healthy
Write-Host "  Waiting for services to be ready..." -ForegroundColor Gray
$maxWait = 30
$waited = 0
while ($waited -lt $maxWait) {
    $pgReady = docker exec orbis-postgres pg_isready -U orbis -d orbis_db 2>&1
    $redisReady = docker exec orbis-redis redis-cli ping 2>&1
    
    if ($pgReady -match "accepting" -and $redisReady -match "PONG") {
        break
    }
    Start-Sleep -Seconds 2
    $waited += 2
    Write-Host "  ... waiting ($waited s)" -ForegroundColor Gray
}

if ($waited -ge $maxWait) {
    Write-Host "  ⚠ Services may not be fully ready, continuing anyway..." -ForegroundColor Yellow
} else {
    Write-Host "  ✓ PostgreSQL ready" -ForegroundColor Green
    Write-Host "  ✓ Redis ready" -ForegroundColor Green
}

# ─── Step 5: Create RustFS Buckets ───────────────────────────────────────────
Write-Host "[5/6] Setting up storage buckets..." -ForegroundColor Yellow

# Wait a bit for RustFS to be ready
Start-Sleep -Seconds 3

# Create buckets using the RustFS/MinIO client (mc)
# We use curl as a simple alternative
$buckets = @("vault", "avatars", "exports")
foreach ($bucket in $buckets) {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:9000/$bucket" -Method Head -ErrorAction SilentlyContinue 2>&1
        Write-Host "  ✓ Bucket '$bucket' exists" -ForegroundColor Green
    } catch {
        try {
            Invoke-WebRequest -Uri "http://localhost:9000/$bucket" -Method Put -Headers @{
                "Authorization" = "AWS rustfsadmin:rustfs_local_2026"
            } -ErrorAction SilentlyContinue 2>&1 | Out-Null
            Write-Host "  ✓ Bucket '$bucket' created" -ForegroundColor Green
        } catch {
            Write-Host "  ⚠ Could not create bucket '$bucket' — create it manually via http://localhost:9001" -ForegroundColor Yellow
        }
    }
}

# ─── Step 6: Install Dependencies & Run Migrations ──────────────────────────
Write-Host "[6/6] Installing dependencies & running migrations..." -ForegroundColor Yellow

Set-Location $ROOT

# Install dependencies
Write-Host "  Installing packages..." -ForegroundColor Gray
bun install 2>&1 | Select-Object -Last 3 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }

# Run database migrations
Write-Host "  Running database migrations..." -ForegroundColor Gray
try {
    $env:DATABASE_URL = "postgresql://orbis:orbis_local_2026@localhost:5432/orbis_db"
    Push-Location (Join-Path $ROOT "packages/db")
    bunx drizzle-kit push 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    Pop-Location
    Write-Host "  ✓ Migrations completed" -ForegroundColor Green
} catch {
    Write-Host "  ⚠ Migration warning: $_" -ForegroundColor Yellow
}

# Run seed
Write-Host "  Seeding database..." -ForegroundColor Gray
try {
    bun run seed 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    Write-Host "  ✓ Seed completed" -ForegroundColor Green
} catch {
    Write-Host "  ⚠ Seed warning (may already be seeded): $_" -ForegroundColor Yellow
}

# ─── Get LAN IP ──────────────────────────────────────────────────────────────
$lanIP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { 
    $_.InterfaceAlias -notmatch "Loopback" -and 
    $_.IPAddress -notmatch "^169\." -and
    $_.IPAddress -ne "127.0.0.1" 
} | Select-Object -First 1).IPAddress

# ─── Print Success ───────────────────────────────────────────────────────────
Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "          ✅ Orbis is ready!" -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Dashboard:  http://localhost:3001" -ForegroundColor Cyan
Write-Host "  API:        http://localhost:3002" -ForegroundColor Cyan
Write-Host "  Storage:    http://localhost:9001  (RustFS Console)" -ForegroundColor Cyan
Write-Host ""
if ($lanIP) {
    Write-Host "  ─── School Network Access ───" -ForegroundColor Yellow
    Write-Host "  Dashboard:  http://${lanIP}:3001" -ForegroundColor Cyan
    Write-Host "  API:        http://${lanIP}:3002" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  💡 Other devices on the school network can access" -ForegroundColor Gray
    Write-Host "     the dashboard at: http://${lanIP}:3001" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  💡 Add this to .env for full LAN support:" -ForegroundColor Gray
    Write-Host "     SERVER_LAN_IP=$lanIP" -ForegroundColor White
}
Write-Host ""
Write-Host "  To start the app, run:" -ForegroundColor Yellow
Write-Host "    bun run dev" -ForegroundColor White
Write-Host ""
Write-Host "  To stop Docker services:" -ForegroundColor Yellow
Write-Host "    docker compose -f docker-compose.local.yml down" -ForegroundColor White
Write-Host ""
