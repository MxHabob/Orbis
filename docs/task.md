# Execution Tasks: Classes Domain Refactor

## Backend & IPC Phase
- `[x]` **Backend: Enriched Class Students**
  - Use `enrollment.enrich_enrollment_models` to provide a list of students for a given class that includes their user/person data (e.g. `student_name`).
- `[x]` **Backend: Class Metrics Scoping**
  - Support `class_id` filtering in `class_metrics` to prevent over-fetching on the Class Overview page.
- `[x]` **Backend: Term Results Enrichment**
  - Ensure `assessment.listTermResults` provides student names, or provide a way to easily join them without fetching the global roster.
- `[x]` **Frontend IPC Updates**
  - Expose new backend commands in the frontend IPC layer (`commands.ts`).
- `[x]` **Frontend: Class Level Restructure**
  - Remove `teachers` tab.
  - Rename `subjects` tab to `curriculum`.
  - Refactor `Classes/[classId]/+page.svelte` (Overview) to remove global fetching and the embedded grade matrix.
- `[x]` **Frontend: ClassSubject Level Restructure**
  - Rename `subjects/[classSubjectId]` directory to `curriculum/[classSubjectId]`.
  - Update any layout navigation pointing to it (e.g. `Classes/[classId]/+layout.svelte` should still point to `curriculum`).
- `[x]` **Frontend: Gradebook Rewrite**
  - Rewrite `[classSubjectId]/gradebook/+page.svelte` using the backend `grade.matrix()` command and `OrbisGrid`.
- `[x]` **Frontend: Term Results Rewrite**
  - Refactor `[classSubjectId]/term-results/+page.svelte` to eliminate global enrollment fetching.
  - Utilize the new `listTermResultsEnriched` API.
- `[x]` **End-to-End Verification**
  - Test the full flow from Classes directory down to Term Results approval.
