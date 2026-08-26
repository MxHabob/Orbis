# Final Architecture Review: Classes Domain

This document provides a deep, backend-first verification of the academic workflow. It strictly defines the boundaries between Class-level responsibilities and ClassSubject-level responsibilities, and redesigns the frontend to map perfectly to the backend source of truth.

---

## 1. The Domain Reality: Class vs. ClassSubject

### The Class Level (`/academics/classes/[classId]`)
*   **Backend Meaning**: A container for a group of enrolled students (`Enrollment`) assigned to a specific `period_id`.
*   **Responsibility**: Homeroom tracking, aggregate attendance, global schedule, and roster management.
*   **Ownership**: Homeroom Teacher / Admin.
*   **What it does NOT own**: It does not own grading, assessment schemes, or specific teacher assignments.

### The ClassSubject Level (`/academics/classes/[classId]/curriculum/[classSubjectId]`)
*   **Backend Meaning**: The fundamental "Execution Hub". It is the intersection of a `Class`, a `Subject` (from the catalog), a `Teacher`, and a `Period`. 
*   **Responsibility**: Assessment configuration (weights/limits), raw grade entry, and calculation of `TermResult` for that specific subject.
*   **Ownership**: The Subject Teacher.

---

## 2. Resolving the `ClassSubject` UX and Sub-routes

The user requested an evaluation of whether `assessment-scheme`, `gradebook`, and `term-results` should be separate pages or merged into a single workspace.

**Verdict**: They **must remain as separate SvelteKit sub-routes**, but presented visually as lightweight tabs within the `ClassSubject` layout. 
*   **Why?** The data models for these three workflows are entirely disjointed:
    *   **Gradebook**: A matrix of raw `ComponentScore` entries (e.g., Homework 1, Quiz 2).
    *   **Term Results**: A finalized, calculated projection (`coursework_score`, `exam_score`, `percentage`, `grade_label`).
    *   **Assessment Scheme**: Configuration of weights (`AssessmentSchemeComponent`).
*   Merging them into a single `.svelte` file would create a monolithic, unmaintainable component. The SvelteKit nested routing (`+layout.svelte` wrapping the tabs) is architecturally correct.
*   **However**, the *current implementation* of these sub-routes is fundamentally flawed and must be rewritten.

---

## 3. The End-to-End Information Architecture

### Level 1: Global Catalogs
*   `/settings/subjects`: Master `Subject` catalog. (Source of truth for syllabus).
*   `/settings/users`: Master `Teacher` directory.

### Level 2: The Class Workspace (`/academics/classes/[classId]`)
1.  **Overview**: 
    *   **Purpose**: High-level health and alerts.
    *   **Data**: `class::get`, `attendance::class_metrics` (scoped to `class_id`).
    *   **Do NOT fetch**: Global grades, global enrollments.
2.  **Students**: 
    *   **Purpose**: Roster management.
    *   **Data**: `class::list_enriched_students(class_id)`. *(⚠️ Missing Backend Requirement: Must implement this to avoid fetching all org enrollments)*.
3.  **Curriculum (New)**: 
    *   **Purpose**: Replaces the old `Subjects` and `Teachers` tabs. Maps a Teacher to a Subject for this Class.
    *   **Data**: `class_subject::list(class_id, period_id)`.
    *   **Navigation**: Clicking a row drills down into Level 3.

### Level 3: The ClassSubject Workspace (`/academics/classes/[classId]/curriculum/[classSubjectId]`)
This workspace uses a `+layout.svelte` header containing the Subject Name, Teacher, and Period.

1.  **Gradebook (`/gradebook`)**: 
    *   **Purpose**: Enter raw scores.
    *   **Source of Truth**: `grade::matrix(period_id, class_subject_id)`.
    *   **Current Flaw**: The frontend currently fetches all global enrollments and manually maps scores!
    *   **Fix**: Completely replace with the `OrbisGrid` component bound directly to the backend's `GradeMatrixResponse`. No manual mapping needed. Cell edits call `grade::upsert_cell`.
2.  **Term Results (`/term-results`)**: 
    *   **Purpose**: Calculate and approve final grades.
    *   **Source of Truth**: `assessment::listTermResults(term_id, class_subject_id)`.
    *   **Current Flaw**: Fetches global enrollments just to map student names. 
    *   **Fix**: The backend must include student names in the `TermResult` projection, or the page must fetch the scoped `list_enriched_students`.
    *   **Mutation**: `assessment::calculateTermResultsBatch()`.
3.  **Assessment Scheme (`/assessment-scheme`)**: 
    *   **Purpose**: Configure weights.
    *   **Source of Truth**: `assessment::listComponentsForClassSubject()`.

---

## 4. Summary of Required Execution Steps

1.  **Backend Updates**: 
    *   Add `list_enriched_students(class_id)` to `svc-academic`.
    *   Ensure `class_metrics` can be scoped to a single `class_id`.
2.  **Frontend Consolidation**: Delete `classes/[classId]/subjects` and `classes/[classId]/teachers` list views. Merge them into `classes/[classId]/curriculum`.
3.  **Frontend Route Renaming**: Move `classes/[classId]/subjects/[classSubjectId]` to `classes/[classId]/curriculum/[classSubjectId]`.
4.  **Gradebook Rewrite**: Rip out the manual matrix logic in the gradebook tab and bind it directly to the `grade.matrix` backend command.
5.  **Term Results Rewrite**: Remove the global `listEnrollments()` query from the term results tab.

## User Feedback Required
> [!IMPORTANT]
> This final deep review strictly isolates the `ClassSubject` execution hub from the `Class` homeroom, validating the SvelteKit sub-route structure but condemning the current data-fetching implementation. 
> 
> **Are you ready to proceed with the execution phase based on this architecture?**
