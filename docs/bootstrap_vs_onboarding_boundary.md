# Bootstrap vs. Onboarding Boundary Specification

**Project:** Orbis ERP / SIS  
**Document:** Bootstrap vs. Onboarding Boundary (`docs/bootstrap_vs_onboarding_boundary.md`)  
**Status:** Canonical Architecture Specification  
**Date:** 2026-10-01  

---

## 1. Architectural Principle & Separation of Concerns

A core flaw in previous systems was blurring the line between **technical foundation creation** and **business onboarding / institution configuration**, leading to either:
1. Bloated, non-resumable bootstrap routines attempting to pre-create hardcoded academic years, classes, and fake demo data.
2. Incomplete bootstrap routines leaving the database in an invalid state lacking currencies, basic accounting accounts, or complete security permissions.

Orbis establishes a strict, non-negotiable architectural boundary:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. BOOTSTRAP (Zero-to-One Technical Foundation)                             │
│ - Executed BEFORE authentication (unauthenticated API / local IPC).        │
│ - Idempotent (409 Conflict if organization + administrator already exist).  │
│ - Creates tenant organization root entity.                                  │
│ - Seeds system reference currencies (ISO standard).                         │
│ - Creates standard double-entry Chart of Accounts scaffold.                 │
│ - Seeds IAM system roles with complete permission sets.                     │
│ - Creates root administrator person, user, and password identity.           │
│ - Stores default organization settings based on selected blueprint.         │
│ - Transitions system to READY_WITH_SETUP state.                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 2. ONBOARDING (Guided Institution Configuration Engine)                     │
│ - Executed AFTER authentication (authenticated with admin permissions).     │
│ - Non-blocking, resumable, idempotent, permission-aware, blueprint-aware.   │
│ - Orchestrates existing application and domain services.                    │
│ - Evaluates real backend state to calculate task completion dynamically.    │
│ - Configures organization profile, branding, contacts, branches, rooms.     │
│ - Configures academic calendar (Year, Terms, Monthly Periods).              │
│ - Configures academic structure (Stages, Grades, Classes, Subjects).        │
│ - Configures assessment schemes & 100-pt monthly/term grading policies.      │
│ - Configures fee templates, installments, discount policies.                │
│ - Configures document requirements, student registration admission rules.   │
│ - Connects integrations (SMTP/Email, SMS, Payment Gateways, DocForge).      │
│ - NEVER fakes completion or invents mock operational data.                 │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 3. NORMAL OPERATION (Day-to-Day SIS & ERP Management)                       │
│ - Student Admissions, Class Rosters, Homeroom assignments.                 │
│ - Teacher Gradebook Entry, Behavior records, Daily Attendance.             │
│ - Fee Invoicing, Payment Collection, Double-Entry Ledger Postings.          │
│ - Master Visual Certificate & Report generation via DocForge.               │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Detailed Responsibility Matrix

| Domain Capability | Bootstrap Layer | Onboarding Subsystem | Normal Application |
| :--- | :--- | :--- | :--- |
| **Organization Identity** | Creates Organization ID, slug, basic name, timezone, locale | Enriches profile (Arabic name, legal tax ID, contact phone, logo, address) | Updates profile settings as needed |
| **Campuses & Branches** | — | Creates Org Units, Campuses, Buildings, Floors, Classrooms | Adds new rooms/facilities |
| **IAM Security** | Creates system roles (`super_admin`, `org_admin`, `principal`, `teacher`, `guardian`, `student`, `accountant`, `registrar`) and root admin user | Configures custom roles, employee role assignments, group memberships | Day-to-day user provisioning & access control |
| **Currencies** | Seeds standard ISO currencies (`SAR`, `USD`, `AED`, `YER`, `EUR`, `GBP`, `KWD`, etc.) & base currency | Sets active operating currencies, exchange rates if multi-currency enabled | Updates exchange rates periodically |
| **Chart of Accounts** | Seeds standard account ledger scaffold (`1000 Assets`, `1110 Cash`, `1120 Receivables`, `2000 Liabilities`, `3000 Equity`, `4000 Tuition`, `4100 Discounts`, `5000 Expenses`) | Maps institution cashboxes, bank accounts, specific student ledger accounts | Posts daily journal entries & audits balances |
| **Academic Calendar** | — | Creates Academic Year, Terms, and Monthly Periods / Exam windows matching blueprint | Daily attendance, term locking, academic promotion |
| **Academic Structure** | — | Creates Stages/Levels, Grades, Classes/Sections, Subjects, and Teaching Assignments | Day-to-day roster management, transfers |
| **Assessment Schemes** | — | Creates 100-pt Monthly breakdown (homework, oral, attendance, test) + Term Exam formulas | Teacher grade entry, exam scheduling, gradebook locking |
| **Academic Policy** | — | Configures pass/fail thresholds, honors thresholds, ranking scopes, rounding rules | Calculates GPA, ranks, and report cards |
| **Fee Templates & Rules**| — | Configures Tuition, Transport, Book fee templates with Grade targeting, discounts, and installment plans | Generates student invoices, collects payments |
| **Student Readiness** | — | Configures registration numbering series, capacity rules, guardian requirements | Registers real students, admits students |
| **Document Archive** | — | Configures mandatory document requirements (birth cert, national ID, vaccine card) | Uploads, verifies, and archives student documents |
| **Reports & Templates** | Reference catalog available | Previews and binds institution report templates and DocForge settings | Generates and exports official transcripts, certificates |
| **Integrations** | — | Configures Email SMTP/SES, SMS gateway, Payment Gateway, DocForge engine | Sends automated alerts and receives webhook notifications |

---

## 3. Anti-Patterns Explicitly Prohibited

1. **No Fake Data:** Neither Bootstrap nor Onboarding shall generate fake student profiles, fake grades, fake attendance logs, or fake payment records to give the illusion of an active institution.
2. **No Client-Side Completion Simulation:** Desktop UI cannot mark a task "Complete" in localStorage or frontend store. Completion is calculated on the backend from real persisted records satisfying domain invariants.
3. **No Direct Database Bypass:** Onboarding actions must execute through authoritative application & domain services (`CalendarService`, `AcademicService`, `FinanceService`, `AssessmentService`, `OrganizationService`, `FolderService`), ensuring all domain events, validation constraints, and double-entry rules fire cleanly.
4. **No Rigid One-Session Requirement:** The administrator is free to complete setup tasks in multiple sessions, exit the application, restart, and pick up exactly where they left off.
