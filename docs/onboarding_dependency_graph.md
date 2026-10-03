# Onboarding Dependency Graph & Evaluation Engine

**Project:** Orbis ERP / SIS  
**Document:** Onboarding Dependency Graph (`docs/onboarding_dependency_graph.md`)  
**Status:** Canonical Reference Specification  
**Date:** 2026-10-01  

---

## 1. Directed Acyclic Graph (DAG) Visualization

The Orbis onboarding task graph enforces operational dependencies so administrators cannot jump ahead into operations that lack foundational prerequisite configuration.

```mermaid
flowchart TD
    subgraph Organization ["1. Organization Layer"]
        ORG_PROFILE["org_profile\n(Org Profile & Branding)"]
        ORG_UNITS["org_units\n(Campuses & Branches)"]
        FACILITIES["facilities_rooms\n(Rooms & Facilities)"]
        
        ORG_PROFILE --> ORG_UNITS
        ORG_UNITS --> FACILITIES
    end

    subgraph Calendar ["2. Calendar Layer"]
        ACADEMIC_YEAR["academic_year\n(Active Academic Year)"]
        ACADEMIC_PERIODS["academic_periods\n(Terms & Monthly Periods)"]
        
        ORG_UNITS --> ACADEMIC_YEAR
        ACADEMIC_YEAR --> ACADEMIC_PERIODS
    end

    subgraph Academics ["3. Academic Structure"]
        SUBJECTS["academic_subjects\n(Subjects & Courses)"]
        CLASSES["academic_classes\n(Classes & Sections)"]
        TEACHING["teaching_assignments\n(Teacher Assignments)"]
        
        ORG_UNITS --> SUBJECTS
        ACADEMIC_PERIODS --> CLASSES
        SUBJECTS --> CLASSES
        CLASSES --> TEACHING
    end

    subgraph Assessment ["4. Grading & Policy"]
        SCHEMES["assessment_schemes\n(Assessment Schemes)"]
        POLICY["academic_policy\n(Pass/Fail & Honors Criteria)"]
        
        SUBJECTS --> SCHEMES
        SCHEMES --> POLICY
    end

    subgraph Finance ["5. Finance Configuration"]
        COA["chart_of_accounts\n(Chart of Accounts & Ledger)"]
        FEE_TEMPLATES["fee_templates\n(Tuition & Fee Templates)"]
        DISCOUNT_TEMPLATES["discount_templates\n(Discounts & Scholarships)"]
        
        ORG_UNITS --> COA
        COA --> FEE_TEMPLATES
        ACADEMIC_YEAR --> FEE_TEMPLATES
        FEE_TEMPLATES --> DISCOUNT_TEMPLATES
    end

    subgraph Students ["6. Student Readiness"]
        REG_POLICY["student_registration_policy\n(Admission Policy & Windows)"]
        DOC_REQ["document_requirements\n(Document Requirements)"]
        
        CLASSES --> REG_POLICY
        FEE_TEMPLATES --> REG_POLICY
        REG_POLICY --> DOC_REQ
    end

    subgraph Extensions ["7. Templates & Integrations"]
        TEMPLATES["system_templates\n(Report & Certificate ASTs)"]
        INTEGRATIONS["integrations_setup\n(Gateways & Notifications)"]
        
        ORG_PROFILE --> TEMPLATES
        ORG_PROFILE --> INTEGRATIONS
    end
```

---

## 2. Dynamic Status Resolution Algorithm

For every task $T$ in the catalog:

1. **Check Applicability:** If $T$ is not applicable to the active `InstitutionType` / `InstitutionBlueprint`, return status `NOT_APPLICABLE`.
2. **Check Unmet Dependencies:** For each prerequisite task ID $P \in \text{dependencies}(T)$:
   - Evaluate completion of $P$.
   - If $P$ is not completed, $T$ is flagged as `BLOCKED`, and a localized `blocking_reason` is attached (e.g. *"Waiting for Academic Year to be created"*).
3. **Evaluate Backend Database Invariants:**
   - Execute precise, optimized count and predicate queries against real database tables (`academic_years`, `academic_periods`, `classes`, `subjects`, `assessment_schemes`, `fee_templates`, `accounts`, `rooms`, `documents`).
   - If the criteria are fully met, return `COMPLETED` with `progress = 100`.
   - If partial records exist (e.g. Year created but no terms), return `IN_PROGRESS` with fractional progress.
   - If no records exist and dependencies are met, return `NOT_STARTED` (or `OPTIONAL` if optional).

---

## 3. Readiness Calculation & Readiness State

An institution transitions to `READY_FOR_OPERATIONS` when:
$$\sum_{T \in \text{Required Tasks}} (\text{status}(T) == \text{COMPLETED}) = |\text{Required Tasks}|$$

Overall progress is computed authoritatively as:
$$\text{Progress} = \left\lfloor \frac{\sum_{T} \text{progress}(T)}{|\text{Applicable Tasks}|} \right\rfloor$$

$$\text{Required Progress} = \left\lfloor \frac{\sum_{T \in \text{Required}} \text{progress}(T)}{|\text{Required Tasks}|} \right\rfloor$$
