# Post-Bootstrap State & Initial System Configuration Audit

**Project:** Orbis ERP / SIS  
**Document:** Post-Bootstrap State Audit (`docs/post_bootstrap_state_audit.md`)  
**Status:** Canonical Reference Architecture Audit  
**Date:** 2026-10-01  

---

## 1. Executive Summary

The Orbis platform operates on a clear lifecycle:
1. **Connection & Database Setup:** Engine checks, database creation, SQLite/PostgreSQL migrations.
2. **Bootstrap (System Foundation):** Creates the single tenant organization identity, root super-administrator credentials, baseline IAM security roles and complete permissions, baseline accounting currencies and standard Chart of Accounts (COA) scaffold, and default organization settings.
3. **Post-Bootstrap Onboarding (Institution Setup):** An application-orchestrated, blueprint-aware, dependency-directed task engine that guides the administrator through organization profile completion, campuses/branches, academic calendar (year, terms, monthly periods), academic structure (levels/grades, classes, sections, subjects), grading & assessment policies, attendance & conduct policies, fee templates & installment policies, document requirements, and integration readiness.
4. **Normal Operational Workflows:** Daily school operations (student enrollment, grading input, lesson scheduling, attendance tracking, payment processing, invoice generation, report printing).

This audit reviews what the current codebase creates at Bootstrap, identifies foundation gaps, clarifies domain boundaries, and establishes the blueprint-driven Onboarding system architecture.

---

## 2. Audit of Current Bootstrap Creation

### 2.1 Inventory of Created Records and Technical Entities

| Created Record / Artifact | Current Layer | Target Classification | Audit Finding / Verdict |
| :--- | :--- | :--- | :--- |
| **Organization Root Record** (`organizations`) | `BootstrapService::create_organization` | `2. Bootstrap Foundation Data` | **Valid.** Creates organization with slug, timezone, locale, branding color, and verified status. |
| **Organization Settings** (`organization_settings`) | `BootstrapService::initialize_settings` | `3. Organization Configuration` | **Partial.** Currently sets `default_grading_system` and `max_class_size_default`, but does not populate full default finance/notifications/autopilot policy structure consistently. |
| **System IAM Roles** (`roles`) | `BootstrapService::create_system_roles` | `2. Bootstrap Foundation Data` | **Valid.** Creates 8 core system roles (`super_admin`, `org_admin`, `principal`, `teacher`, `guardian`, `student`, `accountant`, `registrar`). |
| **System IAM Permissions** (`role_permissions`) | `BootstrapService::create_admin_permissions` | `2. Bootstrap Foundation Data` | **Partial.** Super_admin and org_admin receive 33 resource permission sets, but need to be aligned with the full domain permission enum (including all sub-actions). |
| **Initial Administrator Person** (`persons`) | `BootstrapService::create_administrator` | `2. Bootstrap Foundation Data` | **Valid.** Creates Person record with first name, last name, email, phone, and optional Arabic names. |
| **Initial Administrator User** (`users`) | `BootstrapService::create_administrator` | `2. Bootstrap Foundation Data` | **Valid.** Creates active User with Argon2/PBKDF2 hashed password, pre-verified email. |
| **Initial Administrator Identity** (`user_identities`) | `BootstrapService::create_administrator` | `2. Bootstrap Foundation Data` | **Valid.** Creates password identity provider binding. |
| **Admin Role Assignment** (`user_role_assignments`) | `BootstrapService::assign_admin_role` | `2. Bootstrap Foundation Data` | **Valid.** Binds user to `super_admin` and `org_admin` roles at organization scope. |
| **System Reference Currencies** (`currencies`) | Not initialized during Bootstrap | `1. System Reference Data` | **Gap.** Currencies table remains empty until manual insertion. Bootstrap must seed standard ISO currencies (SAR, USD, AED, YER, EUR, GBP, KWD, BHD, QAR, OMR) and bind base currency. |
| **Chart of Accounts Foundation** (`accounts`) | Not initialized during Bootstrap | `2. Bootstrap Foundation Data` | **Gap.** Standard account categories (Assets 1000, Current Assets 1100, Cash 1110, Student Receivables 1120, Liabilities 2000, Equity 3000, Tuition Revenue 4000, Discounts 4100, Operating Expense 5000) must be seeded so Fee Templates and Payments have valid accounting links. |
| **Academic Years & Periods** | Not in Bootstrap | `4. Onboarding Configuration` | **Correctly Omitted.** Academic Years and Periods belong strictly to Onboarding / Institution Setup. |
| **Classes & Subjects** | Not in Bootstrap | `4. Onboarding Configuration` | **Correctly Omitted.** Belongs to Onboarding / Institution Setup. |
| **Fee Templates & Discounts** | Not in Bootstrap | `4. Onboarding Configuration` | **Correctly Omitted.** Belongs to Onboarding / Institution Setup. |
| **Students & Enrollments** | Not in Bootstrap | `5. User-Created Operational Data` | **Correctly Omitted.** Operational data must NEVER be faked during Bootstrap or Onboarding. |
| **Report & Certificate Templates** | Seeding script in catalog | `1. System Reference Data` / `4. Onboarding` | **Refined.** System visual templates are reference definitions; institution custom templates are configured in Onboarding. |

---

## 3. Classification of All System State

Every entity in the Orbis ecosystem is strictly classified into one of the 6 architectural tiers:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. System Reference Data (ISO Currencies, Global Templates) │
├─────────────────────────────────────────────────────────────┤
│ 2. Bootstrap Foundation Data (Org, Root Admin, IAM Roles)   │
├─────────────────────────────────────────────────────────────┤
│ 3. Organization Configuration (Timezone, Locale, Settings)  │
├─────────────────────────────────────────────────────────────┤
│ 4. Onboarding Configuration (Years, Periods, Grades, Fees)  │
├─────────────────────────────────────────────────────────────┤
│ 5. User-Created Operational Data (Students, Grades, Payments)│
├─────────────────────────────────────────────────────────────┤
│ 6. Test/Fixture Data (Mock suites, Isolated Unit Fixtures)  │
└─────────────────────────────────────────────────────────────┘
```

1. **System Reference Data:**
   - Universal ISO Currencies (`SAR`, `USD`, `AED`, `YER`, `EUR`, `GBP`, `KWD`, `BHD`, `QAR`, `OMR`).
   - Master Report Definitions (`report_catalog.rs`).
   - System Template ASTs (`template_catalog.rs`).
   - Standard Document Category Taxonomies.

2. **Bootstrap Foundation Data:**
   - Root Organization entity (`Organization`).
   - System IAM Roles (`super_admin`, `org_admin`, `principal`, `teacher`, `guardian`, `student`, `accountant`, `registrar`).
   - Root Administrator Person, User, and UserIdentity records.
   - Core Chart of Accounts Ledger Foundation (`1000 Assets`, `1110 Cash`, `1120 Student Receivables`, `2000 Liabilities`, `3000 Equity`, `4000 Tuition Revenue`, `4100 Discounts`, `5000 Expenses`).

3. **Organization Configuration:**
   - Organization Profile (Legal name, Arabic name, Logo, Phone, Address, Tax ID).
   - Organization Structure (Main Campus, Branches, Org Units, Buildings, Rooms).
   - Organization Base Policy Settings (`OrganizationSettings`: Academic, Enrollment, Finance, Notifications, Autopilot).

4. **Onboarding Configuration (Institution-Specific Setup):**
   - Active Academic Year and Date Windows.
   - Academic Terms & Monthly Period Breakdown (matching School vs University blueprint).
   - Academic Stages, Grade Levels, Classroom Sections, and Curriculum Subjects.
   - Assessment Schemes (e.g. School 100-point monthly breakdown: Homework 20, Oral 20, Attendance 20, Written Test 40 + Term Exam).
   - Academic Policy (Pass thresholds, Honors thresholds, Tie policies, Rounding modes).
   - Fee Templates (Tuition, Transport, Books) with Grade/Class targeted scopes, Installment rules, and Discount templates.
   - Student Registration & Document Requirements (Mandatory birth certificates, immunization records, civil IDs).
   - Integrations (Email SMTP/SES, SMS Twilio/Unifonic, WhatsApp, Payment Gateway Mada/Stripe, DocForge, Solver).

5. **User-Created Operational Data (Normal Operations):**
   - Student Applications, Admissions, and Enrollments.
   - Class Section Rosters and Homeroom assignments.
   - Daily Attendance Marks and Behavior Logs.
   - Teacher Gradebook Entries and Assessment Scores.
   - Payments, Receipts, and Double-entry Journal Ledger Lines.
   - Scheduled Timetable Lessons.

6. **Test / Fixture Data:**
   - Mock payloads used exclusively in isolated automated test harnesses.
   - NEVER persisted into real user workspaces or fresh production databases.

---

## 4. Gaps and Weaknesses in Current Implementation

### 4.1 Missing Foundation Areas in Current Bootstrap
1. **Currencies Initialization:** While `OrganizationSettings.finance.currency` is set to e.g. `"SAR"`, no actual rows were inserted into the `currencies` table during Bootstrap, causing queries to `currency_repo.find_by_org()` to fail or return partial data if not pre-seeded.
2. **Standard Chart of Accounts (COA) Scaffold:** If an institution attempts to create Fee Templates or process payments immediately after onboarding, accounts such as Cash (`1110`), Tuition Revenue (`4000`), and Accounts Receivable (`1120`) did not exist unless manually created.
3. **Complete IAM Permissions Mapping:** Several new permission resources (`Employee*`, `Group*`, `Autopilot*`, `Storage*`, `Document*`) were missing from earlier migration backfills.

### 4.2 Inadequate Onboarding State Derivation (Previous System Service)
- Previously, `SystemService::get_readiness()` only evaluated 3 hardcoded booleans (`has_academic_year`, `has_classes`, `has_subjects`).
- It had no concept of task dependencies, blueprint variations (School vs University vs Institute), blocked states, progress tracking, category grouping, or deep verification of fee templates, assessment policies, grading formulas, and document requirements.

### 4.3 Desktop & Backend Contract Parity
- Local IPC (`src-tauri/src/commands`) and Remote HTTP (`api-rust/crates/orbis-server`) must expose the exact same typed onboarding contracts.
- Frontend Desktop must not use hand-crafted unvalidated DTOs or simulate completion client-side. Completion must be 100% derived from backend database verification queries.

---

## 5. Blueprint-Specific Intent for Onboarding

### 5.1 School Blueprint (`k12_school`)
- **Structure:** Academic Year $\rightarrow$ 2 Terms $\rightarrow$ 3 Monthly Periods per Term + Term Final Exam.
- **Assessment Policy:** 
  - Monthly Period Grade = $/100$ (e.g. Homework: 20%, Oral: 20%, Attendance: 20%, Written: 40%).
  - Term Grade = Average of Monthly Periods ($50\%$) + Term Exam ($50\%$) $= 100\%$.
  - Annual Grade = Average of Term 1 and Term 2 $= 100\%$.
- **Classroom Organization:** Homeroom classes, division sections (e.g. Grade 1-A, Grade 1-B), Primary guardian tracking, Student numbering series.

### 5.2 University Blueprint (`university`)
- **Structure:** Academic Year $\rightarrow$ Fall Semester, Spring Semester, Summer Session.
- **Assessment Policy:** GPA / Credit-Hour weighting (Scale 4.0 or 5.0), Course prerequisites, Direct course enrollment per section.
- **Organization:** Faculties, Academic Departments, Degree Programs, Majors, Course Sections.

### 5.3 Vocational / Training Institute Blueprint (`vocational_institute`)
- **Structure:** Training Cohorts, Rolling Modular Tracks, Certificate Programs.
- **Assessment Policy:** Pass/Fail competencies, Contact hour requirements, Module completion criteria.

---

## 6. Audit Sign-off

With this audit established, the implementation proceeds with:
1. Enhancing `BootstrapService` with currency seeding, standard COA scaffold, and complete IAM role permissions.
2. Implementing the strongly-typed `OnboardingService` in `orbis-app` with full task graph, dependency resolution, blueprint differentiation, and database-backed completion verifications.
3. Exposing unified API routes & Tauri commands.
4. Building the professional Onboarding Hub & Setup Center in Desktop.
5. Providing full documentation and verification tests.
