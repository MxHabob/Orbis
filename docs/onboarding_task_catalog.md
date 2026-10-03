# Onboarding Setup Task Catalog

**Project:** Orbis ERP / SIS  
**Document:** Onboarding Task Catalog (`docs/onboarding_task_catalog.md`)  
**Status:** Canonical Reference Specification  
**Date:** 2026-10-01  

---

## 1. Catalog Architecture Overview

The Orbis Onboarding Engine generates an active DAG (Directed Acyclic Graph) of setup tasks for any given organization based on:
1. **Institution Blueprint:** `School` (`k12_school`), `University` (`university`), `Institute` (`vocational_institute`).
2. **Organization Capabilities & Enabled Modules:** e.g. Homeroom vs Credit-Hours, Transportation, Guardians.
3. **Current Persisted Database State:** Verified against domain repository queries.

---

## 2. Complete Setup Task Catalog

### Category 1: Organization & Identity (`organization`)

#### `org_profile` — Institution Profile & Branding
- **Category:** `Organization`
- **Required:** Yes
- **Applicability:** All Blueprints (`School`, `University`, `Institute`)
- **Dependencies:** None
- **Completion Condition:** Organization record contains non-empty name, timezone, locale, and at least one contact channel (phone/email) or address configured.
- **Route:** `/[orgId]/settings`
- **Action Type:** `navigate`
- **Permissions:** `organization:write`

#### `org_units` — Campuses, Branches & Facilities
- **Category:** `Organization`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `org_profile`
- **Completion Condition:** At least one active Org Unit (Main Campus/Branch) exists in `org_units`.
- **Route:** `/[orgId]/settings/branches`
- **Action Type:** `navigate`
- **Permissions:** `org_unit:write`

#### `facilities_rooms` — Buildings, Floors & Educational Spaces
- **Category:** `Organization`
- **Required:** Optional
- **Applicability:** All Blueprints
- **Dependencies:** `org_units`
- **Completion Condition:** At least one Room record exists in `rooms`.
- **Route:** `/[orgId]/settings/rooms`
- **Action Type:** `navigate`
- **Permissions:** `org_unit:write`

---

### Category 2: Academic Calendar & Cycles (`calendar`)

#### `academic_year` — Active Academic Year
- **Category:** `Calendar`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `org_units`
- **Completion Condition:** At least one Academic Year exists in `academic_years` with `status = 'active'` and valid `start_date < end_date`.
- **Route:** `/[orgId]/settings/periods`
- **Action Type:** `navigate`
- **Permissions:** `period:write`

#### `academic_periods` — Terms & Monthly Period Breakdown
- **Category:** `Calendar`
- **Required:** Yes
- **Applicability:** All Blueprints (Blueprint-specialized structure)
  - *School:* Terms (1 & 2) + Monthly Periods (1-3, 4-6) + Exam Windows.
  - *University:* Semesters (Fall, Spring, Summer) + Registration/Drop Windows.
  - *Institute:* Modular Training Cohort Windows.
- **Dependencies:** `academic_year`
- **Completion Condition:** At least 2 academic periods exist under the active year in `academic_periods`.
- **Route:** `/[orgId]/settings/periods`
- **Action Type:** `navigate`
- **Permissions:** `period:write`

---

### Category 3: Academic Structure (`academics`)

#### `academic_subjects` — Curriculum Subjects & Courses
- **Category:** `Academics`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `org_units`
- **Completion Condition:** At least one active Subject exists in `subjects` with non-empty code and name.
- **Route:** `/[orgId]/settings/subjects`
- **Action Type:** `navigate`
- **Permissions:** `subject:write`

#### `academic_classes` — Classrooms, Sections & Cohorts
- **Category:** `Academics`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `academic_periods`, `academic_subjects`
- **Completion Condition:** At least one active Class exists in `classes` linked to the active period and unit.
- **Route:** `/[orgId]/academics/classes`
- **Action Type:** `navigate`
- **Permissions:** `class:write`

#### `teaching_assignments` — Class Subject Teacher Assignments
- **Category:** `Academics`
- **Required:** Optional (Prerequisite for timetable)
- **Applicability:** All Blueprints
- **Dependencies:** `academic_classes`, `academic_subjects`
- **Completion Condition:** At least one `class_subjects` mapping exists linking class, subject, and teacher.
- **Route:** `/[orgId]/academics/classes`
- **Action Type:** `navigate`
- **Permissions:** `class:write`

---

### Category 4: Grading & Assessment Policies (`assessment`)

#### `assessment_schemes` — Monthly / Term Assessment Scheme
- **Category:** `Assessment`
- **Required:** Yes
- **Applicability:** All Blueprints (School 100-pt monthly formula vs University GPA scale)
- **Dependencies:** `academic_subjects`
- **Completion Condition:** At least one `AssessmentScheme` exists in `assessment_schemes` with valid component weightings.
- **Route:** `/[orgId]/settings/assessment-schemes`
- **Action Type:** `navigate`
- **Permissions:** `assessment:write`

#### `academic_policy` — Pass/Fail & Honors Criteria
- **Category:** `Assessment`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `assessment_schemes`
- **Completion Condition:** Organization settings or policy repository contains valid passing threshold ($> 0$), honors threshold, ranking scope, and rounding mode.
- **Route:** `/[orgId]/settings/academic-policy`
- **Action Type:** `navigate`
- **Permissions:** `organization:write`

---

### Category 5: Finance & Fee Configuration (`finance`)

#### `chart_of_accounts` — Accounting Ledger & Cashboxes
- **Category:** `Finance`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `org_units`
- **Completion Condition:** Core system accounts exist in `accounts` (`1000 Assets`, `1110 Cash`, `1120 Receivables`, `4000 Revenue`).
- **Route:** `/[orgId]/finance/accounts`
- **Action Type:** `navigate`
- **Permissions:** `finance:write`

#### `fee_templates` — Tuition & Mandatory Fee Templates
- **Category:** `Finance`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `chart_of_accounts`, `academic_year`
- **Completion Condition:** At least one active `FeeTemplate` exists in `fee_templates` with positive amount and target scope (Grade or All).
- **Route:** `/[orgId]/settings/fee-templates`
- **Action Type:** `navigate`
- **Permissions:** `finance:write`

#### `discount_templates` — Scholarships & Sibling Discounts
- **Category:** `Finance`
- **Required:** Optional
- **Applicability:** All Blueprints
- **Dependencies:** `fee_templates`
- **Completion Condition:** At least one `DiscountTemplate` exists in `discount_templates`.
- **Route:** `/[orgId]/settings/discount-templates`
- **Action Type:** `navigate`
- **Permissions:** `finance:write`

---

### Category 6: Student Readiness & Admission Policies (`students`)

#### `student_registration_policy` — Admission Windows & Numbering
- **Category:** `Students`
- **Required:** Yes
- **Applicability:** All Blueprints
- **Dependencies:** `academic_classes`, `fee_templates`
- **Completion Condition:** `OrganizationSettings.enrollment.registration_enabled` is set with valid `max_class_size_default > 0`.
- **Route:** `/[orgId]/settings/registration`
- **Action Type:** `navigate`
- **Permissions:** `organization:write`

#### `document_requirements` — Mandatory Student Documents
- **Category:** `Students`
- **Required:** Optional
- **Applicability:** All Blueprints
- **Dependencies:** `student_registration_policy`
- **Completion Condition:** At least one `DocumentRequirement` or folder category exists for student admissions.
- **Route:** `/[orgId]/settings/archive`
- **Action Type:** `navigate`
- **Permissions:** `document:write`

---

### Category 7: Reports, Templates & Documents (`templates`)

#### `system_templates` — Visual AST Certificate & ID Card Templates
- **Category:** `Templates`
- **Required:** Optional
- **Applicability:** All Blueprints
- **Dependencies:** `org_profile`
- **Completion Condition:** At least one published `report_template` exists in `report_templates` or DocForge engine initialized.
- **Route:** `/[orgId]/templates`
- **Action Type:** `navigate`
- **Permissions:** `template:write`

---

### Category 8: External Integrations (`integrations`)

#### `integrations_setup` — Messaging, Notifications & Gateways
- **Category:** `Integrations`
- **Required:** Optional
- **Applicability:** All Blueprints
- **Dependencies:** `org_profile`
- **Completion Condition:** Status of external integration providers (Email SMTP, SMS, WhatsApp, Payment Gateway, DocForge) is inspected and recorded.
- **Route:** `/[orgId]/settings/notifications`
- **Action Type:** `navigate`
- **Permissions:** `organization:write`
