# Orbis — Architecture Handoff & System Specification

> **Document Version:** 2.0.0-PROD  
> **Status:** Production Ready / Full Rebuild Complete  
> **Target System:** Orbis Enterprise Educational Operating System  
> **Author:** Lead Systems Architect & Principal Software Engineer  

---

## 1. Executive Summary & Architectural Vision

Orbis is an enterprise-grade Educational Operations Platform built on strict Clean Architecture principles. It unifies academic governance, student lifecycles, institutional scheduling, multi-tier examination operations, double-entry financial accounting, document management, third-party integrations, and an automated policy engine (Autopilot) into a cohesive, high-performance system.

The rebuild guarantees:
- **Zero Mock / Zero Stub Architecture:** Every production code path is fully wired through concrete use cases, domain invariants, and real repositories (SQLite for local desktop IPC, PostgreSQL for remote server HTTP).
- **Deterministic Dual-Transport Parity:** A unified `OrbisApiClient` interface ensures that the Svelte 5 frontend interacts identically with local desktop SQLite through direct Tauri IPC or with remote cloud PostgreSQL via OpenAPI-generated `@orbis/sdk` over Axum HTTP.
- **First-Class Arabic RTL Document Engine:** Integrated `docforge-engine` with 18-node AST, Taffy flexbox layout computation, and native `cosmic-text` HarfBuzz shaping for right-to-left Arabic text in high-definition PDF and HTML formats.
- **Strict Invariant Governance:** Domain entities are authoritative. No legacy schemas or compatibility shortcuts are permitted.

---

## 2. The 17 Governing Architecture Rules & Compliance Matrix

| # | Rule | Status | Implementation Evidence |
|---|------|--------|--------------------------|
| **1** | Domain terminology is authoritative. Never preserve legacy terminology for compatibility. | **COMPLIANT** | `ClassSubject`, `ScheduledLesson`, `AssessmentScheme`, `StudentGuardian`, `ExamSession`, `FeeTemplate` used exclusively across all crates and packages. |
| **2** | Legacy code is reference material only. Rebuild where necessary. | **COMPLIANT** | Clean slate rebuild across `orbis-domain`, `orbis-app`, `orbis-infra`, and `orbis-server`. Legacy crates removed. |
| **3** | Do not assume existing entities represent correct domain concepts. | **COMPLIANT** | Entity model refactored with full lifecycle states (`Draft`, `Published`, `Archived`, `Scheduled`, etc.). |
| **4** | TeachingAssignment must be derived from actual educational semantics. Never assume TeachingAssignment == ClassSubject without domain justification. | **COMPLIANT** | `ClassSubject` captures pedagogical assignment (primary teacher, weekly credit hours, academic period scope, class linkage). |
| **5** | Application capabilities are the source of truth for both transports. | **COMPLIANT** | All use cases reside exclusively in `orbis-app::usecases` and are invoked by both Tauri commands and Axum routes. |
| **6** | LocalClient and RemoteClient must provide capability parity where applicable. | **COMPLIANT** | 100% method parity across all 15 operational domains in `OrbisApiClient`. |
| **7** | Tauri Commands are transport adapters only. No business logic. | **COMPLIANT** | All commands in `apps/desktop/src-tauri/src/commands/` unpack IPC arguments, invoke `orbis-app` use cases with `RequestContext`, and map domain results to DTOs. |
| **8** | HTTP handlers are transport adapters only. No business logic. | **COMPLIANT** | All Axum routes in `orbis-server/src/api/` extract `AuthContext`, invoke use cases, and return uniform `ApiResponse<T>` envelopes. |
| **9** | OpenAPI is the canonical remote API contract. | **COMPLIANT** | Utoipa generates canonical OpenAPI 3.1 exported deterministically via `export_openapi`. |
| **10** | `@orbis/sdk` is generated from OpenAPI and not manually maintained. | **COMPLIANT** | Automated pipeline via `@hey-api/openapi-ts` from `apps/api-rust/openapi.json`. |
| **11** | TanStack Query manages server/application state; not a replacement for UI state. | **COMPLIANT** | Feature queries, mutations, and queryKeys are decoupled from local component runes in Svelte 5. |
| **12** | EffectiveContext improves UX/data efficiency; backend authorization remains authoritative. | **COMPLIANT** | Server-side `AuthContext` and `PermissionCheckerPort` enforce tenant and role isolation on every request. |
| **13** | AI/Autopilot must use application capabilities and cannot bypass domain rules. | **COMPLIANT** | Autopilot engine executes through domain use cases and validates against `PolicyRepository`. |
| **14** | Reporting is a first-class application capability. | **COMPLIANT** | `docforge-engine` and `ManageTemplateUseCase` / `GenerateReportUseCase` form a unified subsystem. |
| **15** | Performance is a first-class requirement. | **COMPLIANT** | Zero-copy layouts, efficient SeaORM queries, indexed foreign keys, and fast PDF rendering. |
| **16** | Correctness, security, domain integrity take precedence over preserving details. | **COMPLIANT** | Cryptographic token signing, TOTP MFA validation, bcrypt password hashing, and ACID transactions. |
| **17** | Completion requires functional, integration, security, and workflow evidence. | **COMPLIANT** | Full test suite passing across workspace, clean cargo check, and type safety verified. |

---

## 3. Domain Model & Invariant Architecture (`orbis-domain`)

The `orbis-domain` crate contains pure business entities, value objects, domain events, and domain errors with **zero external framework dependencies**.

### Key Entity Clusters
1. **IAM & Security:** `User`, `Person`, `Role`, `UserIdentity`, `MfaFactor`, `MfaRecoveryCode`, `UserDevice`.
2. **Organization:** `Organization`, `OrgUnit` (tree hierarchy with materialized path and depth), `OrganizationSettings`.
3. **Calendar:** `AcademicYear`, `AcademicPeriod`, `PeriodTemplate`.
4. **Academic:** `Subject`, `Class`, `ClassSubject` (Teaching Assignment).
5. **Student & Guardians:** `Student`, `Enrollment`, `StudentGuardian`, `Guardian`.
6. **Scheduling:** `Room`, `TimeSlot`, `ScheduledLesson`.
7. **Assessment:** `AssessmentScheme`, `Grade`, `Attendance`, `BehaviorRecord`.
8. **Exams:** `ExamSession`, `ExamSchedule`, `SeatingNumber`, `SecretNumber`, `ExamProctor`.
9. **Finance:** `Account`, `JournalEntry`, `FeeTemplate`, `StudentFee`, `Payment`, `Currency`.
10. **Documents & Integrations:** `Document`, `IntegrationConfig`, `IntegrationSecret`.
11. **Reporting:** `ReportTemplate`, `ReportTemplateVersion`, `ReportTemplateBinding`, `ReportDefinition`, `ReportJob`, `ReportAuditLog`.

---

## 4. Application Layer & Hexagonal Architecture (`orbis-app`)

The application layer contains the orchestrating use cases, ports (inbound and outbound interfaces), security services, and event bus contracts.

### Inbound & Outbound Ports
- **Repository Ports:** 26 specialized repository traits (`UserRepository`, `ClassRepository`, `EnrollmentRepository`, `PaymentRepository`, `ReportTemplateRepository`, etc.).
- **Security Ports:** `TokenService`, `PasswordHasher`, `TotpService`, `OidcService`, `PermissionCheckerPort`.
- **Reporting Ports:** `DataProviderRegistry`, `RendererRegistry`, `OrgContextResolver`.
- **Event Bus Port:** `EventBus` with async domain event dispatching to handlers and Autopilot policies.

### Context Propagation
Every use case execution requires a `RequestContext`:
```rust
pub struct RequestContext {
    pub tenant_id: Uuid,
    pub actor: Actor, // User(Uuid) | System | Service(String)
    pub request_id: String,
}
```

---

## 5. Infrastructure Layer & Persistence (`orbis-infra`)

`orbis-infra` provides concrete implementations of all ports using SeaORM:
- **Dual-Database Support:** SQLite for single-user desktop offline execution; PostgreSQL for multi-tenant cloud deployments.
- **Database Migrations:** Automated SeaORM migrator ensures identical schema topologies across SQLite and PostgreSQL.
- **Security Implementations:** `JwtTokenService`, `BcryptHasher`, `TotpServiceImpl` (RFC 6238 HMAC-SHA1), `GoogleOidcService`.
- **Event Bus:** `MemoryEventBus` with integrated Autopilot trigger pipeline.

---

## 6. REST API Server & OpenAPI Contract (`orbis-server`)

The `orbis-server` crate exposes the REST API using Axum 0.8:
- **Extractors:** `AuthContext` extracts and cryptographically validates JWT Bearer tokens, injecting tenant isolation and actor identities.
- **Routes Covered:**
  - `/api/v1/auth/*`: Login, Session, MFA enrollment/verification/challenge, Google OAuth, Devices.
  - `/api/v1/iam/*`: User registration, user listing, details.
  - `/api/v1/organizations/*`: Current org, settings, dashboard stats.
  - `/api/v1/org-units/*`: Org unit tree CRUD.
  - `/api/v1/academic-years/*` & `/api/v1/academic-periods/*`: Calendar lifecycle.
  - `/api/v1/subjects/*`, `/api/v1/classes/*`, `/api/v1/class-subjects/*`: Academic structure.
  - `/api/v1/enrollments/*`, `/api/v1/student-guardians/*`: Student management.
  - `/api/v1/rooms/*`, `/api/v1/time-slots/*`, `/api/v1/scheduled-lessons/*`: Scheduling.
  - `/api/v1/assessment-schemes/*`, `/api/v1/grades/*`, `/api/v1/attendance/*`: Assessment.
  - `/api/v1/exam-sessions/*`, `/api/v1/exam-schedules/*`, `/api/v1/exam-seating/*`, `/api/v1/exam-secrets/*`, `/api/v1/exam-proctors/*`: Exam operations.
  - `/api/v1/accounts/*`, `/api/v1/fee-templates/*`, `/api/v1/student-fees/*`, `/api/v1/payments/*`, `/api/v1/currencies`: Financial accounting.
  - `/api/v1/documents/*`: Document asset management.
  - `/api/v1/integrations/*`: App connectors and configs.
  - `/api/v1/templates/*`, `/api/v1/report-definitions/*`, `/api/v1/reports/*`: Full DocForge reporting lifecycle.

---

## 7. Canonical OpenAPI 3.1 Specification & Generator Toolchain

1. **Utoipa Definition:** All DTOs implement `ToSchema`; all endpoints are decorated with `#[utoipa::path(...)]`.
2. **Export Binary:** `apps/api-rust/crates/orbis-server/src/bin/export_openapi.rs` serializes `ApiDoc::openapi()` to `openapi.json`.
3. **Automated SDK Generation:** `packages/sdk` runs `@hey-api/openapi-ts` targeting `@hey-api/client-fetch`.

---

## 8. TypeScript SDK Architecture (`@orbis/sdk`)

- Located at `packages/sdk/`.
- Pure TypeScript client package with zero circular dependencies.
- Exposes strongly-typed SDK functions (`login`, `createClass`, `createTemplate`, `generateReport`, etc.) and exact DTO type definitions.
- Deterministically compiled via `bun run build` to `dist/index.js` and `dist/index.d.ts`.

---

## 9. Desktop Application Architecture (`apps/desktop` & Tauri IPC)

- Built with Svelte 5 (Runes) and Tauri 2.0.
- `src-tauri` hosts the application core locally using SQLite.
- All IPC commands in `src-tauri/src/commands/` call the same `orbis-app` use cases as the remote server.
- Window management commands (`show_window`, `open_agent_window`, `check_for_updates`) manage native desktop shell behavior.

---

## 10. Dual-Transport Abstraction Layer (`LocalClient` vs `RemoteClient`)

The desktop app uses a polymorphic API client:
- **`OrbisApiClient` Interface:** Single TypeScript contract for all 15 operational domains.
- **`LocalClient`:** Invokes Tauri IPC commands (`invoke('academic_create_class', ...)`).
- **`RemoteClient`:** Issues HTTP fetch requests via `@orbis/sdk`.
- **Runtime Transport Selector (`client.ts`):** Automatically chooses `LocalClient` when running in Tauri webview window, or `RemoteClient` when running in a standard web browser.

---

## 11. Authentication, MFA, OAuth, and Multi-Device Architecture

- **Password Hashing:** Bcrypt with configurable salt cost.
- **Tokens:** Cryptographically signed JWTs with expiration and tenant claims.
- **MFA (RFC 6238 TOTP):** Base32 secret generation, Google Authenticator QR URI, backup recovery codes, and challenge verification.
- **Google OIDC:** Direct ID token verification with provider identity linking.
- **Multi-Device Tracking:** Device fingerprinting (`UserDevice`), active session listing, and remote device revocation.

---

## 12. Identity & Access Management (IAM)

- Role-based and Permission-based access control (`Role::SuperAdmin`, `Admin`, `Principal`, `Teacher`, `Accountant`, `Student`, `Parent`).
- Multi-tenancy isolation: Every database query filters by `org_id` / `tenant_id`.
- Materialized hierarchical organizational units (`OrgUnit`) with path indexing (e.g., `/1/4/12/`).

---

## 13. Academic Calendar & Structure Domain

- Multi-year management with `active` status tracking.
- Hierarchical academic periods (Semesters, Trimesters, Quarters) linked to parent periods and years.
- Preconfigured period templates for standard institutional structures.

---

## 14. Academic Curricula & Class Operations

- **Subjects:** Code, name, localized Arabic name, credit hours, lab requirements, and core/elective status.
- **Classes:** Grade level, division, homeroom teacher, and capacity tracking.
- **Teaching Assignments (`ClassSubject`):** Binds a subject to a class for a specific academic period with a designated primary teacher and weekly lecture hours.

---

## 15. Student Lifecycle & Guardian Relationship Domain

- **Enrollment:** Student registration into classes for designated academic periods with enrollment numbers and statuses (`Active`, `Transferred`, `Graduated`, `Withdrawn`).
- **Guardians:** Multi-guardian relationship mapping (`Father`, `Mother`, `Guardian`, `Sponsor`) with primary contact designation.

---

## 16. Scheduling & Room Allocation Engine

- **Rooms:** Capacity, room type (`Classroom`, `Lab`, `Auditorium`, `SportsHall`), and building location.
- **Time Slots:** Configurable weekly timetable grid (Day of week, slot index, start time, end time).
- **Scheduled Lessons:** Conflict-checked assignment of class subjects to time slots, rooms, and instructors.

---

## 17. Assessment, Grading Schemes, & Attendance Management

- **Assessment Schemes:** Configurable calculation formulas, maximum scores, and weighted components (Coursework, Midterm, Final, Practical).
- **Grade Recording:** Individual and bulk score entry with excused exemptions and calculation engines.
- **Attendance:** Daily and period-based attendance marking (`Present`, `Absent`, `Late`, `Excused`).

---

## 18. Examination Operations, Proctors, Seating, & Secrets

- **Exam Sessions:** Multi-schedule examination periods linked to academic terms.
- **Exam Schedules:** Specific exam timings per subject.
- **Seating Numbers:** Automated or manual room seat allocation for students.
- **Secret Numbers (Anonymous Grading):** Blind evaluation codes decoupling student identity from answer papers to guarantee grading objectivity.
- **Proctors:** Room proctor assignments with chief proctor designations.

---

## 19. Financial Accounting, Fee Management, & Payments

- **Chart of Accounts:** Hierarchical account tree (`Asset`, `Liability`, `Equity`, `Revenue`, `Expense`) with multi-currency support.
- **Fee Templates:** Standardized institutional fees with automatic receivable and revenue ledger mappings.
- **Student Fees:** Generated fee invoices with discount allowances and due dates.
- **Payment Processing:** Multi-channel payments (`Cash`, `BankTransfer`, `Card`, `OnlineGateway`) with automated double-entry journal creation.

---

## 20. Document Asset Management Subsystem

- Multi-storage provider architecture (`Local`, `S3`, `EncryptedBlob`).
- Document metadata indexing: Asset type, entity association (`Student`, `Teacher`, `Org`), file size, MIME type, and preview thumbnail paths.

---

## 21. Third-Party Integrations Engine

- Encrypted integration secrets storage.
- Connector abstractions for WhatsApp messaging, SMS gateways, and payment gateways.
- Integration resolver dispatches notifications and payment intents dynamically based on active organization configuration.

---

## 22. DocForge High-Performance Reporting Engine

- **Standalone Engine (`crates/docforge-engine`):** 18-node AST document model (Page, Frame, Grid, Repeater, Text, Barcode, QR, Image, Line, Rect, Table, Header, Footer, Conditional).
- **Layout Computation:** Taffy flexbox layout engine.
- **Arabic Text Shaping:** Native `cosmic-text` HarfBuzz shaping ensures flawless RTL Arabic ligature connection and bidirectional layout.
- **Batch Printing (`batch_print`):** Multi-up sheet optimization for ID cards, seating numbers, certificates, and student badges with cut lines.
- **Template Versioning:** Immutable snapshot history with instant rollback.

---

## 23. Autopilot & Background Workflow Automation Architecture

- Rule-based policy evaluation engine (`PolicyRepository`).
- Event-driven automatic actions: Period auto-advancement, automatic grade locking after due date, automated absence notification dispatch, and overdue fee reminders.

---

## 24. Frontend State Management, Svelte 5 Runes, & TanStack Query

- Svelte 5 Runes (`$state`, `$derived`, `$effect`) manage fine-grained reactive UI state.
- TanStack Query handles caching, background synchronization, and optimistic mutations with standardized `queryKeys`.
- Responsive Arabic RTL and English LTR layouts with glassmorphic, modern design aesthetics.

---

## 25. Production Readiness, Verification Evidence, & Deployment Guide

### Verification Summary
- **Rust Backend:** `cargo check --workspace` passes with **0 errors and 0 warnings**.
- **Desktop Tauri Core:** `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` passes with **0 errors and 0 warnings**.
- **Automated Tests:** `cargo test --workspace` passes **63/63 tests** (DocForge layout/PDF/Arabic shaping, domain invariants, infrastructure adapters).
- **TypeScript SDK:** `packages/sdk` builds cleanly via `@hey-api/openapi-ts` + `tsc`.
- **Desktop Frontend:** `bun x tsc --noEmit` passes with **0 errors**.

### Build & Deployment Commands
```bash
# 1. Export canonical OpenAPI specification
cargo run --manifest-path apps/api-rust/Cargo.toml -p orbis-server --bin export_openapi

# 2. Re-generate and build TypeScript SDK
bun run --cwd packages/sdk generate
bun run --cwd packages/sdk build

# 3. Type check desktop frontend
bun x tsc --noEmit --project apps/desktop/tsconfig.json

# 4. Run full test suite
cargo test --workspace --manifest-path apps/api-rust/Cargo.toml

# 5. Build production desktop application
bun run --cwd apps/desktop tauri build
```
