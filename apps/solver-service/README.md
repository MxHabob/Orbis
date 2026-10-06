# Orbis Solver

High-performance dedicated scheduling and constraint optimization engine powered by Timefold Solver and Quarkus.

---

## 🏗️ Architecture & Philosophy

```
Desktop / Web App ──(App Store Configuration)──► Multi-tenant DB
                                                        │
api-rust (HTTP Orchestrator / Client Adapter) ──────────┘
      │
      ├── (HTTP / JSON) ──► apps/solver-service (Zero-dependency Standalone Executable)
      └── SQLite/Postgres (Persistence & Cycle State)
```

1. **Zero Logic Duplication**: Rust acts purely as the multi-tenant orchestrator and HTTP client adapter (`RemoteTimefoldSolver` / `SchedulingSolverPort`).
2. **Multi-Tenant Integration**: Configured directly through `@orbis/app-store` (`solver-service` app) without static environment variables or server restarts.
3. **High-Performance Constraint Engine**: Timefold Solver running in an isolated, lightweight process.
4. **Standalone Native Binary**: Can be compiled to a native `.exe` executable via GraalVM / Mandrel, requiring **zero Docker** and **zero JVM** on deployment machines.

---

## 🚀 Running & Building Standalone Native Binary (.exe)

### Option 1: Standalone Windows Native Executable (No Docker, No JVM)

```bash
cd apps/solver-service
mvn package -Dnative -DskipTests
```
The output binary will be created at:
`apps/solver-service/target/orbis-solver-<Orbis-version>-runner.exe`

Double-click or run from command line:
```cmd
.\target\orbis-solver-<Orbis-version>-runner.exe
```
- **Instant startup**: < 0.05 seconds
- **Memory footprint**: ~25-35 MB RAM
- **Dependencies**: None.

### Option 2: Local Development (Hot Reload)

```bash
cd apps/solver-service
mvn quarkus:dev
```
Service starts on `http://localhost:8081`.

---

## 🔌 App Store Integration

Configure the solver via the UI at **AI Center > Integrations > Timetable & Schedule Solver**:
- **Endpoint URL**: `http://localhost:8081` (or your remote cluster URL)
- **Solving Timeout**: `30` seconds

---

## 📡 API Endpoints

| Method | Path | Description |
|---|---|---|
| `POST` | `/solve` | Submit timetable optimization problem (Async `202 Accepted`) |
| `GET` | `/solve/{jobId}` | Poll timetable solving progress and solution |
| `DELETE` | `/solve/{jobId}` | Terminate/cancel active timetable solving job |
| `POST` | `/solve/exam` | Submit exam schedule optimization problem |
| `GET` | `/solve/exam/{jobId}` | Poll exam schedule solving progress and solution |
| `DELETE` | `/solve/exam/{jobId}` | Terminate/cancel active exam solving job |
| `GET` | `/health` | Liveness & readiness health check |
