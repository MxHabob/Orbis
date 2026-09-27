# Orbis — Enterprise Educational Operating System (Agent Execution Contract)

> **Document Version:** 2.0.0-PROD  
> **Workspace Root:** `d:/projects/Orbis`  
> **Master Execution Plan:** `docs/ORBIS_ERP_TRANSFORMATION_EXECUTION_PLAN.md`  

---

## 1. Executive Instructions for AI Coding Agents

Every agent starting a session in this repository MUST:
1. **Read this AGENTS.md file first.**
2. **Read the current CONTINUATION STATE at the bottom of this file.**
3. **Inspect the existing implementation before writing any code.**
4. **Follow the Cardinal Law: CONSOLIDATION BEFORE ADDITION.**
5. **Never create mock data, placeholder buttons, empty handlers, or parallel duplicate components.**
6. **Never use `as any` casts to hide architectural mismatches.**
7. **Perform a Legacy Removal Pass after modifying any vertical section.**
8. **Verify the Section Completion Contract before declaring any section complete.**
9. **Update the CONTINUATION STATE section at the bottom of this file before ending the turn.**

---

## 2. Technology Stack & Workspace Structure

- **Backend:** Rust Clean Architecture (`apps/api-rust/crates/`):
  - `orbis-domain`: Pure domain entities, value objects, invariants.
  - `orbis-app`: Use cases, inbound/outbound ports, security services.
  - `orbis-infra`: Concrete repositories via SeaORM (SQLite for desktop; PostgreSQL for cloud).
  - `orbis-server`: Axum 0.8 REST API exposing canonical OpenAPI 3.1.
  - `docforge-engine`: Standalone 18-node AST vector reporting engine with cosmic-text RTL Arabic HarfBuzz shaping.
- **Frontend / Desktop:** SvelteKit 2 + Svelte 5 Runes + Tauri 2.0 (`apps/desktop/`):
  - `apps/desktop/src-tauri`: Native desktop wrapper and IPC command handlers calling `orbis-app` use cases.
  - `packages/ui`: Shared UI components.
  - `packages/svelte-grid`: High-performance virtualized canvas/DOM hybrid grid (`OrbisGrid`).
  - `packages/sdk`: Pure TypeScript client generated deterministically via `@hey-api/openapi-ts`.

---

## 3. CONTINUATION STATE

- **Current Status:** `AI_SIDEBAR_AND_COCKPIT_REFINED`
- **Master Reference Documents:** [orbis_ai_architecture_discovery.md](file:///c:/Users/User/.gemini/antigravity-ide/brain/3e2fb622-6955-4230-8ce1-9809fa99c255/orbis_ai_architecture_discovery.md), [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md)
- **Accomplished:**
  - Audited and reinforced AI Orchestration architecture, consolidating 100% on existing Clean Architecture domain models (`StudentService`, `AcademicService`, `AssessmentService`, `FinanceService`, `SchedulingService`, `ReportService`, `OrganizationService`, `AutopilotService`) without creating any unnecessary database tables or duplicate schemas.
  - Implemented strict runtime permission validation on all AI tools (`PermissionCheckerPort::check`) before execution or consequential action queuing, preventing unauthorized data access or bypass.
  - Built typed `UserAiContext` resolver resolving authenticated user identity, active roles, effective permissions, and scope paths.
  - Built typed `AcademicAiContext` and `UiContext` resolvers connecting real-time academic years, active periods, selected classes, subjects, route locations, and table filters directly into the AI system prompt.
  - Redesigned [provider-model-picker.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/ai/components/provider-model-picker.svelte) to use nested DropdownMenu (`DropdownMenuSub`, `DropdownMenuSubTrigger`, `DropdownMenuSubContent`, `DropdownMenuPortal`) matching [org-sidebar-switcher.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/shell/org-sidebar-switcher.svelte).
  - Streamlined [chat-message-list.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/ai/components/chat-message-list.svelte) by removing the inner header title box from assistant responses, providing a clean, elegant, minimal response view with hover actions.
  - Eliminated hardcoded tool execution switches (`handleApproveAction`/`handleRejectAction`), routing all consequential operations directly through the backend Autopilot task lifecycle.
  - Removed manual `isRtl` variables in JS for CSS classes, adopting pure CSS logical properties (`dir`, `ms-`, `me-`, `ps-`, `pe-`, `inset-x-`, `border-s`, `border-e`, `rtl:rotate-180`) respecting `<html lang="%paraglide.lang%" dir="%paraglide.dir%">`.
  - Added 5 high-impact institutional AI tools:
    1. `analyze_at_risk_students`: Early warning signal detector evaluating attendance drop rates, failing grades, and overdue fees.
    2. `generate_exam_paper`: Pedagogical assessment generator applying Bloom's taxonomy with model answer keys and allocated marks.
    3. `analyze_class_performance`: Deep academic analytics computing GPA distribution curves, pass rates, and grade spectrums.
    4. `analyze_financial_health`: Institutional collection efficiency, tuition aging analysis, and revenue health forecasting.
    5. `draft_parent_notice`: Personalized, respectful official school letters and parent notices.
  - Verified 0 errors across `bun x tsc --noEmit` (clean), `cargo check --workspace` in `apps/api-rust` (clean), `cargo check` in `apps/desktop/src-tauri` (clean), and production static build `bun run build` (clean).
- **Next Target Module:** `EXAMS` / `FINANCE` domain workflows and DocForge template integrations.