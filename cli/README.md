# Orbis School Server — Deployment CLI (No Source Code Required)

This folder contains everything needed to deploy and run **Orbis** on a school's local server — fully offline, secure, and without any source code on the host machine. Pre-built Docker images are pulled from a private GitHub Container Registry (GHCR).

---

## Contents

| File | Purpose |
|------|---------|
| `docker-compose.yml` | Production Docker Compose with all 6 services |
| `start-school-server.ps1` | Interactive CLI for managing the server (start, stop, update, status) |
| `Run-Orbis-Server.bat` | One-click launcher with auto admin elevation (UAC) |
| `.env.example` | Template for environment variables and secrets |
| `README.md` | This file |

---

## Prerequisites

1. **Docker Desktop** (v4.x or later):
   - Download and install from [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/)
   - Ensure **WSL 2** or **Hyper-V** is enabled during installation.

2. **Docker Registry Authentication** (if images are private):
   - Orbis container images are hosted as **private packages** on GHCR to prevent unauthorized access. You must authenticate before the first run:
     1. Create a **Personal Access Token (classic)** from your GitHub account settings with the **`read:packages`** scope.
     2. Open PowerShell on the server and run:
        ```powershell
        docker login ghcr.io -u YOUR_GITHUB_USERNAME
        ```
     3. When prompted for a password, paste the **PAT** you created and press Enter.

---

## Quick Start (One Click)

1. Navigate to this folder.
2. Double-click **`Run-Orbis-Server.bat`**.
3. The system will automatically request Administrator privileges (UAC), then launch and configure everything.

**Alternative (Manual):**

1. Open PowerShell as **Administrator**.
2. Navigate to this folder.
3. Run:
   ```powershell
   powershell -ExecutionPolicy Bypass -File start-school-server.ps1
   ```

---

## What the Script Does

When you run the CLI for the first time, it performs the following steps automatically:

1. **Checks Prerequisites** — Verifies that Docker Desktop is installed and running.
2. **Generates Secure Secrets** — Creates a `.env` file from the template and generates unique, cryptographically secure passwords and encryption keys for your school (auth secrets, database credentials, and storage keys). Each school gets its own isolated credentials.
3. **Detects LAN IP** — Discovers the server's local network IP address (e.g., `192.168.1.100`) and automatically configures CORS, API URLs, and dashboard binding so that other devices on the school network can connect immediately.
4. **Pulls & Starts Containers** — Downloads the pre-built container images from GHCR and starts all 6 services in the background:
   - **PostgreSQL 17** — Primary database (optimized for school workloads)
   - **Redis 7** — Cache and background job queue (BullMQ)
   - **RustFS** — S3-compatible object storage (for documents, avatars, and exports)
   - **Orbis API** — Hono + tRPC backend server
   - **Orbis Dashboard** — Next.js admin panel
   - **Orbis Worker** — Background job processor
5. **Runs Migrations** — The API container automatically applies database migrations on first startup, creating all required tables and seeding initial data (grades 1–9, subjects, and academic terms).

---

## Access URLs

Once the server is running, you can access the system at:

### From the server machine:

| Service | URL |
|---------|-----|
| **Dashboard** | [http://localhost:3001](http://localhost:3001) |
| **API Docs** | [http://localhost:3002](http://localhost:3002) |
| **Storage Console** | [http://localhost:9001](http://localhost:9001) |

> Storage Console default credentials: `rustfsadmin` / `rustfs_local_2026`
> (These are replaced with secure random credentials after first setup)

### From other devices on the school network (teachers, admins):

Open a browser and navigate to the server's LAN IP with port `3001` (the script prints the exact URL for you):

```
http://192.168.1.100:3001
```

---

## CLI Management Commands

The `start-school-server.ps1` script provides an interactive menu with the following actions:

| Action | Command | Description |
|--------|---------|-------------|
| **Interactive Menu** | `start-school-server.ps1` | Opens the CLI menu (default) |
| **Start** | `start-school-server.ps1 start` | Initialize environment and start all services |
| **Stop** | `start-school-server.ps1 stop` | Gracefully stop all containers |
| **Restart** | `start-school-server.ps1 restart` | Stop and re-start all services |
| **Update** | `start-school-server.ps1 update` | Pull latest images and redeploy |
| **Status** | `start-school-server.ps1 status` | Show container health and test endpoints |
| **Config** | `start-school-server.ps1 config` | Display current `.env` settings (secrets are masked) |

### Direct Docker Commands

You can also manage the server directly with Docker Compose:

- **Stop the server:**
  ```powershell
  docker compose down
  ```
- **View logs (monitor the system):**
  ```powershell
  docker compose logs -f api
  ```
- **Update to a new version:**
  ```powershell
  docker compose pull
  docker compose up -d
  ```
- **Full reset (⚠️ deletes all data):**
  ```powershell
  docker compose down -v
  ```
