# Onboarding Subsystem Architecture

**Project:** Orbis ERP / SIS  
**Document:** Onboarding Subsystem Architecture (`docs/onboarding_architecture.md`)  
**Status:** Canonical Architecture Reference  
**Date:** 2026-10-01  

---

## 1. Architectural Layers & Component Interactions

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ DESKTOP CLIENT (Svelte 5 / TailwindCSS / Runes)                             │
│ - Onboarding Center Hub (/[orgId]/onboarding or Settings Dashboard Widget)  │
│ - Category filters, status tabs, search, progress gauge, task cards         │
│ - Direct navigation to authoritative admin screens                          │
│ - Single unified API interface (LocalClient IPC & RemoteClient HTTP)       │
└─────────────────────────────────────────────────────────────────────────────┘
                                      ▲
                                      │
                         (Tauri IPC / REST API)
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ ONBOARDING ORCHESTRATION LAYER (orbis-app::services::OnboardingService)     │
│ - Evaluates real backend state via canonical domain repositories.           │
│ - Resolves Blueprint metadata (School, University, Institute).              │
│ - Dynamically computes DAG task graph, blocking statuses, and progress.     │
│ - Provisions starter scaffolding on demand via canonical domain services.   │
│ - Pure application-level orchestrator — NO duplicate domain entities.      │
└─────────────────────────────────────────────────────────────────────────────┘
                                      ▲
                                      │
                 (Calls Canonical Domain Application Services)
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ CORE DOMAIN SERVICES & REPOSITORIES (orbis-app & orbis-infra)              │
│ - OrganizationService (Org, OrgUnit, Settings)                             │
│ - CalendarService (AcademicYear, AcademicPeriod, PeriodTemplate)           │
│ - AcademicService (Subject, Class, ClassSubject, CoursePrerequisites)       │
│ - AssessmentService (AssessmentScheme, Grade, Attendance, Behavior)        │
│ - FinanceService (Account, FeeTemplate, DiscountTemplate, Payment)         │
│ - DocumentService & FolderService (DocumentRequirement, ArchiveFolder)     │
│ - ReportService & TemplateEngine (ReportDefinitions, AST Templates)        │
└─────────────────────────────────────────────────────────────────────────────┘
                                      ▲
                                      │
                             (SeaORM SQL Queries)
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ DATABASE PERSISTENCE (SQLite / PostgreSQL)                                 │
│ - Real, normalized educational & enterprise ERP tables                      │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Key Architecture Invariants

1. **Idempotence & Crash Safety:**
   Every onboarding read and execution operation is strictly idempotent. If the user closes the application in the middle of configuring subjects or classes, restarting the app immediately evaluates the current database state, marks completed tasks as `COMPLETED`, unlocks dependent tasks, and points the user to the recommended next task.

2. **Local / Remote Parity:**
   The desktop client uses identical SDK and IPC signatures. Local desktop execution (Tauri command invoking Rust `AppState.onboarding_service`) and Remote execution (Axum HTTP route `GET /api/v1/onboarding/state` invoking Rust `AppState.onboarding_service`) return the exact same strongly-typed response JSON.

3. **No Redundant Domain Layer:**
   `OnboardingService` does not define its own tables for classes, years, or accounts. It delegates all validation and creation to existing domain repositories and services, keeping domain business rules in a single authoritative location.
