export * from "./queries";
export * from "./queries-calendar";
export * from "./queries-policy";
export * from "./queries-assessment";
export * from "./promotion-queries";

export { default as AcademicsOverview } from "./academics-overview.svelte";
export { default as ClassesDirectory } from "./classes-directory.svelte";
export { default as ClassDetails } from "./class-details.svelte";
export { default as ClassStudents } from "./class-students.svelte";
export { default as ClassCurriculum } from "./class-curriculum.svelte";
export { default as SectionSubjectDetails } from "./class-subject-details.svelte";
export { default as SectionSubjectResults } from "./class-subject-results.svelte";
export { default as AcademicsReports } from "./academics-reports.svelte";
export { default as ScheduleWorkspace } from "./schedule-workspace.svelte";

export { default as PromotionWizard } from "./promotion-wizard.svelte";
export { default as PromotionHistory } from "./promotion-history.svelte";
export { default as ArchivedBanner } from "./archived-banner.svelte";

export { default as ControlSidebar } from "./components/control-sidebar.svelte";
export { default as ControlGrid } from "./components/control-grid.svelte";
export { default as ClassSelect } from "./components/class-select.svelte";
export { default as SubjectSelect } from "./components/subject-select.svelte";
export { default as PeriodSelect } from "./components/period-select.svelte";
export { default as TeacherSelect } from "./components/teacher-select.svelte";

// Modern Timetable & Scheduling Components
export { default as ScheduleToolbar } from "./components/schedule-toolbar.svelte";
export { default as ScheduleMatrix } from "./components/schedule-matrix.svelte";
export { default as LessonSheet } from "./components/lesson-sheet.svelte";
export { default as TimeSlotSheet } from "./components/time-slot-sheet.svelte";
export { default as SolverDrawer } from "./components/solver-drawer.svelte";
export { default as ScheduleGeneratorStudio } from "./components/schedule-generator-studio.svelte";
export { default as ClassCreateSheet } from "./components/class-create-sheet.svelte";
export { default as ClassForm } from "./components/class-form.svelte";
export { default as SectionSubjectForm } from "./components/class-subject-form.svelte";
export { default as SectionSubjectSheet } from "./components/class-subject-sheet.svelte";
export { default as SubjectCreateSheet } from "./components/subject-create-sheet.svelte";
export { default as SubjectForm } from "./components/subject-form.svelte";
export { default as PeriodCreateSheet } from "./components/period-create-sheet.svelte";
export { default as PeriodForm } from "./components/period-form.svelte";
export { default as RoomCreateSheet } from "./components/room-create-sheet.svelte";
export { default as RoomForm } from "./components/room-form.svelte";
export { default as OrgUnitSheet } from "./components/org-unit-sheet.svelte";
export { default as OrgUnitForm } from "./components/org-unit-form.svelte";
export { default as AssessmentCreateSheet } from "./components/assessment-create-sheet.svelte";
export { default as AssessmentForm } from "./components/assessment-form.svelte";
export { default as TimeSlotForm } from "./components/time-slot-form.svelte";
export { default as TimeSlotManager } from "./components/time-slot-manager.svelte";


// Section aliases aligned with unified domain model
export { default as SectionsDirectory } from "./classes-directory.svelte";
export { default as SectionDetails } from "./class-details.svelte";
export { default as SectionStudents } from "./class-students.svelte";
export { default as SectionCurriculum } from "./class-curriculum.svelte";
export { default as SectionSelect } from "./components/class-select.svelte";
export { default as SectionCreateSheet } from "./components/class-create-sheet.svelte";
export { default as SectionForm } from "./components/class-form.svelte";
