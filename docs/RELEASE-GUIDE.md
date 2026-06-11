# 🚀 Orbis Release Guide

This guide explains how to publish a new version of Orbis. The release process is fully automated via GitHub Actions — you just need to commit, tag, and push.

---

## How It Works

The CI/CD pipeline ([release.yml](file:///c:/Users/YZLPN3/Downloads/new/Orbis/.github/workflows/release.yml)) is triggered by **pushing a tag** that matches the pattern `v*.*.*` (e.g., `v1.0.1`, `v2.0.0`).

When triggered, the pipeline automatically:

1. **Lints & Typechecks** the codebase
2. **Builds 3 Docker images** (API, Dashboard, Worker)
3. **Pushes them to GHCR** with two tags each: the version tag + `stable`
4. **Builds the Tauri desktop app** for Windows & macOS and creates a draft GitHub Release

---

## Step-by-Step Release Process

### 1. Make sure all changes are committed

```powershell
git status
```

If there are uncommitted changes, stage and commit them:

```powershell
git add -A
git commit -m "fix: description of changes"
```

### 2. Push to main

```powershell
git push origin main
```

### 3. Choose the version number

Follow [Semantic Versioning](https://semver.org/):

| Type | When to use | Example |
|------|-------------|---------|
| **Patch** (`x.x.+1`) | Bug fixes, typo corrections, small tweaks | `v1.0.0` → `v1.0.1` |
| **Minor** (`x.+1.0`) | New features, non-breaking changes | `v1.0.1` → `v1.1.0` |
| **Major** (`+1.0.0`) | Breaking changes, major rewrites | `v1.1.0` → `v2.0.0` |

To check the latest existing version:

```powershell
git tag --sort=-creatordate -l
```

### 4. Create the tag

```powershell
git tag -a v1.0.2 -m "v1.0.2: Brief description of what changed"
```

> Replace `v1.0.2` with your chosen version number.

### 5. Push the tag

```powershell
git push origin v1.0.2
```

This triggers the GitHub Actions pipeline automatically.

### 6. Monitor the pipeline

Go to: **https://github.com/MxHabob/Orbis/actions**

You will see the pipeline running with two jobs:
- **Lint & Typecheck** — runs first
- **Build & Publish to GHCR** — runs after tests pass
- **Build & Release Desktop App** — runs in parallel on Windows & macOS

### 7. Finalize the GitHub Release (optional)

The desktop build creates a **draft release** on GitHub. To publish it:

1. Go to **https://github.com/MxHabob/Orbis/releases**
2. Find the draft release for your version
3. Edit the release notes if needed
4. Click **Publish release**

---

## Updating a School Server to the New Version

After the pipeline finishes and images are pushed to GHCR, school servers can update using the CLI:

```powershell
powershell -ExecutionPolicy Bypass -File start-school-server.ps1 update
```

Or manually:

```powershell
docker compose pull
docker compose up -d
```

> By default, school servers pull the `stable` tag, which is automatically updated with every release.

---

## Quick Reference (Copy-Paste)

Here's the full release flow in one block:

```powershell
# 1. Commit your changes
git add -A
git commit -m "fix: description of changes"

# 2. Push to main
git push origin main

# 3. Tag the release
git tag -a v1.0.2 -m "v1.0.2: Brief description"

# 4. Push the tag (triggers CI/CD)
git push origin v1.0.2
```

---

## What Gets Built

Each release produces the following artifacts on GHCR:

| Image | Tags | Source |
|-------|------|--------|
| `ghcr.io/<username>/orbis-api` | `v1.0.2`, `stable` | `apps/api/Dockerfile` |
| `ghcr.io/<username>/orbis-dashboard` | `v1.0.2`, `stable` | `apps/dashboard/Dockerfile` |
| `ghcr.io/<username>/orbis-worker` | `v1.0.2`, `stable` | `packages/jobs/Dockerfile` |

Plus a **Tauri desktop installer** for Windows (`.msi`) and macOS (`.dmg`) attached to the GitHub Release.

---

## Troubleshooting

### Pipeline fails at Lint & Typecheck

Lint warnings are allowed (`continue-on-error: true`), but **typecheck errors will block the release**. Fix them before tagging:

```powershell
bun run typecheck
```

### Pipeline fails at Docker build

Check the GitHub Actions logs for the specific build error. Common issues:
- Missing environment variables in Dockerfile
- Build context issues (make sure you're building from the repo root)

### Wrong tag pushed

If you pushed a wrong tag, delete it and re-tag:

```powershell
# Delete locally
git tag -d v1.0.2

# Delete from remote
git push origin --delete v1.0.2

# Re-tag with correct commit
git tag -a v1.0.2 -m "v1.0.2: Corrected description"
git push origin v1.0.2
```

### Need to re-run a failed pipeline

Go to the failed workflow run on GitHub Actions and click **Re-run all jobs**.
