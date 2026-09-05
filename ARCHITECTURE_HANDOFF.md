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

## Reporting Architecture & Desktop Template Engine Modernization

### Core Tenets
1. **Zero Legacy Crates**: `crates-legacy/docforge-orbis` and `crates-legacy/svc-report` have been completely removed from desktop dependencies.
2. **Preserved Core Engine**: `docforge-engine` (in `crates/docforge-engine`) provides the 18-node AST, Taffy flexbox layout engine, expression evaluator, and Krilla + cosmic-text PDF renderer with first-class Arabic/RTL text shaping. It is now a first-class member of the Cargo workspace.
3. **Hexagonal Integration**:
   - Template lifecycle management (Create, Read, Update, Delete, Publish, Archive, Version History, Version Reversion) runs through `ManageTemplateUseCase` in `orbis-app`.
   - Report generation runs through `GenerateReportUseCase` in `orbis-app` utilizing `DataProviderRegistry`, `RendererRegistry`, and `OrgContextResolver`.
   - IPC bridge in `apps/desktop/src-tauri/src/commands/template_cmds.rs` and `report_cmds.rs` exposes 15 typed Tauri commands.
4. **Transport Layer Abstraction**:
   - `apps/desktop/src/lib/api/types.ts`: Strongly-typed `TemplateDto`, `CreateTemplateRequest`, `UpdateTemplateRequest`, `TemplateVersionDto`, `TemplateBindingDto`, `ValidationResult`, `ReportDefinitionDto`.
   - `OrbisApiClient` interface updated with all template methods.
   - `LocalClient`: Maps to Tauri IPC `template_*` commands.
   - `RemoteClient`: Maps to REST `/api/v1/templates/*` endpoints.
   - `apps/desktop/src/lib/api/client.ts`: Exposes the singleton `api` proxy with automatic runtime transport detection.
5. **Frontend Modernization**:
   - Template List page (`/templates`): Uses `api.listTemplates()`, `api.createTemplate()`, `api.deleteTemplate()`, `api.publishTemplate()`, `api.archiveTemplate()`.
   - Template Editor page (`/templates/[templateId]`): Integrated with live PDF preview via `api.previewTemplate()`, saving via `api.updateTemplate()`, and robust Svelte 5 runes state.
   - Template Grid component: Supports both camelCase DTOs and localized names (`nameAr`/`name`).
   - Print Dialog & Exam Reports: Updated from legacy `commands.docforge.*` to unified `api` client.
