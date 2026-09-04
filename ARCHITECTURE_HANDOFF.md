# Orbis Architecture Handoff

## 1. Current Architecture (Legacy)
- 21 scattered micro-crates (`svc-iam`, `svc-finance`, etc.) with tight coupling.
- God Object `ServiceContext` containing 15 dependencies.
- Manual raw SQL migrations (`schema_migrations.rs`, `schema_baseline.sql`) causing schema drift between SQLite and PostgreSQL.
- Hardcoded integrations without a scalable Port/Adapter pattern.

## 2. Final Architecture Decision
- **Style:** Clean Hexagonal Modular Monolith.
- **Crates:** `orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`, `desktop/src-tauri`.
- **Database:** `sea-orm` (2.x) with `sea-orm-migration` as the single source of truth for both SQLite and PostgreSQL.
- **Integrations:** DB-backed Provider Registry + Adapter Pattern.
- **Transactions:** Outbox pattern for ensuring at-least-once domain event delivery within DB transactions.
- **Desktop Strategy:** Tauri IPC for zero-latency local mode, lazy startup, background DB migration.

## 3. Current Phase
Phase 2: Domain & Application Core

## 4. Completed Work
- Exhaustive Architecture Review and Blueprint creation.
- Phase 1: Project Skeleton & Migration Foundation.
  - Old crates moved to `crates-legacy`.
  - 4 core crates created.
  - `sea-orm-migration` initialized in `orbis-infra`.

## 5. Files Created
- `ARCHITECTURE_HANDOFF.md`

## 6. Files Modified
*(To be updated)*

## 7. Files Deleted/Moved
- Moved `apps/api-rust/crates/*` to `apps/api-rust/crates-legacy/*`

## 8. Database/Migration State
- Wiping old manual SQL scripts. Setting up `sea-orm-migration`.

## 9. Integration State
- Design stage. DB models to be defined in Phase 4.

## 10. Tests Passed
- N/A

## 11. Tests Remaining
- All unit and integration tests for new crates.

## 12. Known Issues
- None yet.

## 13. Architectural Decisions
- No domain code relies on HTTP or SeaORM.
- UI does not block on DB migrations during startup.

## 14. Decisions explicitly rejected
- *Microservices:* Too complex for local desktop deployment.
- *Sidecar HTTP API for Desktop:* IPC is much faster and reduces OS process overhead.
- *RLS as sole security:* Defense-in-depth is preferred (App-level checks + optional RLS).

## 15. Next exact steps
1. Initialize the 4 main crates.
2. Setup `sea-orm-migration` in `orbis-infra`.

## 16. Commands to continue
`cargo check` from `apps/api-rust`.

## 17. Important constraints for the next agent
- **DO NOT** use raw SQL unless `SeaQuery` absolutely cannot handle it.
- **DO NOT** add `sea-orm` to `orbis-app` or `orbis-domain`.
- Always check official SeaORM 2.x documentation for migrations.
