# Orbis Lightweight Stack Deployer (Go + SQLite)
# Target environment: Windows PowerShell

Write-Host "🚀 Starting Orbis Lightweight Deployment (Go + SQLite)..." -ForegroundColor Indigo

# 1. Stop and remove existing container
Write-Host "🛑 Checking and stopping active Orbis containers..." -ForegroundColor Yellow
docker compose down --remove-orphans

# 2. Build the Docker image
Write-Host "📦 Building Go API backend image with CGO and SQLite support..." -ForegroundColor Cyan
docker compose build --no-cache

# 3. Spin up the containers
Write-Host "⚡ Starting Orbis Go API in background..." -ForegroundColor Green
docker compose up -d

# 4. Wait and verify health check
Write-Host "⏳ Waiting for API to boot and self-seed..." -ForegroundColor DarkYellow
Start-Sleep -Seconds 5

$healthUrl = "http://localhost:3002/health"
try {
    $response = Invoke-RestMethod -Uri $healthUrl -Method Get -TimeoutSec 5
    if ($response.status -eq "healthy") {
        Write-Host "✅ Orbis API is up and healthy!" -ForegroundColor Green
        Write-Host "   Engine: $($response.engine)" -ForegroundColor Gray
        Write-Host "   Version: $($response.version)" -ForegroundColor Gray
    } else {
        Write-Warning "⚠️ API responded, but status was: $($response.status)"
    }
} catch {
    Write-Error "❌ Could not contact API at $healthUrl. Check Docker logs: 'docker compose logs'"
}

# 5. Output testing details
Write-Host ""
Write-Host "🎉 Deployment finished!" -ForegroundColor Magenta
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host "💻 Dashboard SPA dev server: run 'bun run dev:dashboard-spa'" -ForegroundColor Cyan
Write-Host "🔗 Dashboard SPA Port: http://localhost:3001" -ForegroundColor Cyan
Write-Host "⚡ Go API Endpoint:     http://localhost:3002" -ForegroundColor Cyan
Write-Host "🔐 Seed Credentials:" -ForegroundColor Green
Write-Host "   Email:    admin@orbis.edu" -ForegroundColor Gray
Write-Host "   Password: admin123" -ForegroundColor Gray
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
