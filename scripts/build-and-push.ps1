# =============================================================================
# Orbis — Build & Push Docker Images to GitHub Container Registry (GHCR)
# =============================================================================
# Usage: 
#   powershell -ExecutionPolicy Bypass -File scripts/build-and-push.ps1 -GithubUsername "your-github-username"
# =============================================================================

param (
    [Parameter(Mandatory=$true)]
    [string]$GithubUsername
)

$ErrorActionPreference = "Stop"

Write-Host "=====================================================================" -ForegroundColor Cyan
Write-Host "🚀 Starting Orbis Production Build and Push to GHCR" -ForegroundColor Cyan
Write-Host "   GitHub Username: $GithubUsername" -ForegroundColor Cyan
Write-Host "=====================================================================" -ForegroundColor Cyan

# 1. Prompt Docker Login to GHCR if needed
Write-Host "`n🔑 Step 1: Checking GHCR authentication..." -ForegroundColor Yellow
Write-Host "If you are not logged in, please run: 'docker login ghcr.io' first using your GitHub PAT." -ForegroundColor Gray

# 2. Build API Image
Write-Host "`n📦 Step 2: Building API image..." -ForegroundColor Yellow
$ApiTag = "ghcr.io/$($GithubUsername.ToLower())/orbis-api:latest"
docker build -t $ApiTag -f apps/api/Dockerfile .

# 3. Build Dashboard Image (Next.js)
Write-Host "`n📦 Step 3: Building Dashboard image..." -ForegroundColor Yellow
$DashboardTag = "ghcr.io/$($GithubUsername.ToLower())/orbis-dashboard:latest"
docker build -t $DashboardTag -f apps/dashboard/Dockerfile .

# 4. Build Worker Image
Write-Host "`n📦 Step 4: Building Worker image (Background Jobs)..." -ForegroundColor Yellow
$WorkerTag = "ghcr.io/$($GithubUsername.ToLower())/orbis-worker:latest"
docker build -t $WorkerTag -f packages/jobs/Dockerfile .

# 5. Push Images to GHCR
Write-Host "`n📤 Step 5: Pushing images to GitHub Container Registry..." -ForegroundColor Yellow
Write-Host "Pushing API..." -ForegroundColor Gray
docker push $ApiTag

Write-Host "Pushing Dashboard..." -ForegroundColor Gray
docker push $DashboardTag

Write-Host "Pushing Worker..." -ForegroundColor Gray
docker push $WorkerTag

Write-Host "`n=====================================================================" -ForegroundColor Green
Write-Host "✅ All Orbis Docker images built and pushed successfully!" -ForegroundColor Green
Write-Host "   API:       $ApiTag" -ForegroundColor Green
Write-Host "   Dashboard: $DashboardTag" -ForegroundColor Green
Write-Host "   Worker:    $WorkerTag" -ForegroundColor Green
Write-Host "=====================================================================" -ForegroundColor Green
Write-Host "Now you can distribute the 'dist/' folder to the schools." -ForegroundColor Gray
