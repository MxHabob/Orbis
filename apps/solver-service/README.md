# Orbis Timetable Solver (Timefold Sidecar)

Isolated Quarkus + Timefold service for automatic school timetable generation.

## Architecture

```
api-rust (orchestrator)  ──HTTP──►  solver-service (compute)
     │                                    │
     └── SQLite (jobs + schedule)         └── SolverManager thread pool
```

- **Async-first**: API returns `202 Accepted` immediately; solving runs in background.
- **Resource isolation**: JVM limited to 768MB / 2 CPUs (see `Dockerfile` and `docker-compose.local.yml`).
- **No logic duplication**: Hard constraints mirror `find_lesson_conflicts` in Rust; validation reuses `check_schedule_conflicts` on apply.

## Quick Start

### With Docker (recommended)

```bash
docker compose -f docker-compose.local.yml up -d solver-service
```

### Local development (requires JDK 17 + Maven)

```bash
cd apps/solver-service
mvn quarkus:dev
```

Service listens on `http://localhost:8081`.

### Configure API

```env
SOLVER_SERVICE_URL=http://localhost:8081
```

## API Contract

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/solve` | Submit problem → `202 Accepted` |
| `GET` | `/solve/{jobId}` | Poll job status + best solution |
| `DELETE` | `/solve/{jobId}` | Cancel early termination |
| `GET` | `/health` | Health check |

Rust API wrappers (authenticated):

| Method | Path |
|--------|------|
| `POST` | `/api/schedule/solve` |
| `GET` | `/api/schedule/solve` |
| `GET` | `/api/schedule/solve/{id}` |
| `DELETE` | `/api/schedule/solve/{id}` |
| `POST` | `/api/schedule/solve/{id}/apply` |

## Solve Modes

- **full** — generates all `weekly_hours` assignments; apply replaces existing cycle schedule.
- **incremental** — fills only missing hours; pins existing lessons.

## Production Build

```bash
mvn -B package -DskipTests
docker build -t orbis-solver:2.0.0 .
```
