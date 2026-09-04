# Architecture Handoff: SDK & API Integration

## Completed Work
- **Phase 1 (OpenAPI):** Configured `utoipa` inside `orbis-server`. Explicitly separated DTOs (`CreateOrgRequest`, `OrgResponse`, etc.) from Domain entities. Exposed `/api-docs/openapi.json`. Built `export_openapi.rs` script for CI/CD deterministic generation.
- **Phase 2 (SDK Generation):** Rebuilt `packages/sdk` utilizing `@hey-api/openapi-ts` strictly as an HTTP/Types client. 
- **Phase 3 (Legacy Cleanup):** Hard-deleted `packages/sdk`, `sdk-core`, `sdk-desktop`.
- **Phase 4 (Desktop Transport Abstraction):** Created `apps/desktop/src/lib/api/client.ts` containing the Dual-Mode `ApiClient` (LocalClient using Tauri IPC vs RemoteClient using Fetch SDK).

## Remaining Work
- **Phase 5 (Web/Mobile & Validation):** TypeScript compilation (`bun run check`) currently fails in `apps/desktop`. 
  - **Reason:** Legacy Svelte components heavily rely on Domain Types (`User`, `AcademicYear`, `Enrollment`, `ExamSession`) from the now-deleted `sdk-core`. Because these endpoints aren't implemented in Axum yet, their schemas are missing from the new SDK. 
  - **Resolution Needed:** We must decide how to bridge the missing types without violating the "No placeholder schemas" rule.

## Architecture
- **Local Desktop:** Svelte -> `LocalClient` -> Tauri IPC -> `Application Core` -> SQLite (No HTTP overhead).
- **Remote / Mobile:** App -> `RemoteClient` -> `@orbis/sdk` -> HTTP Axum -> PostgreSQL.
- **OpenAPI:** Acts purely as the HTTP Contract. Does not infect the Domain layer.
