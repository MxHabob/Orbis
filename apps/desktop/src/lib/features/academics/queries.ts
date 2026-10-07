import { createQuery, createMutation } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type CalculateRankingsRequest,
  type RankingItemDto,
  type ClassDto,
  type SubjectDto,
  type AcademicPeriodDto,
  type AcademicYearDto,
  type ClassSubjectDto,
  type AssessmentSchemeDto,
  type TimeSlotDto,
  type ScheduledLessonDto,
  type PrerequisiteCheckResultDto,
  type RoomDto,
  type AttendanceDto,
  type GradeDto,
  type ReportDefinitionDto,
} from "$lib/api";

/** All academic years for current organization. */
export function createAcademicYearsQuery(orgId: () => string) {
  return createQuery<AcademicYearDto[]>(() => ({
    queryKey: queryKeys.calendar.years(orgId()),
    queryFn: () => api.listAcademicYears(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** All academic periods for current organization/year. */
export function createPeriodsQuery(orgId: () => string) {
  return createQuery<AcademicPeriodDto[]>(() => ({
    queryKey: queryKeys.calendar.periods(orgId()),
    queryFn: () => api.listAcademicPeriods(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Classes for an organization. */
export function createClassesQuery(orgId: () => string) {
  return createQuery<ClassDto[]>(() => ({
    queryKey: queryKeys.academic.classes(orgId()),
    queryFn: () => api.listClasses(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Single class by ID. */
export function createClassQuery(classId: () => string | undefined) {
  return createQuery<ClassDto>(() => ({
    queryKey: queryKeys.academic.class(classId() ?? ""),
    queryFn: () => api.getClass(classId()!),
    enabled: !!classId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Students enrolled in a specific class. */
export function createClassStudentsQuery(classId: () => string | undefined) {
  return createQuery<any[]>(() => ({
    queryKey: [...queryKeys.student.all, "class-students", classId() ?? ""],
    queryFn: async () => {
      const res = await api.listStudents();
      const currentClassId = classId();
      return (res.items || []).filter(
        (s: any) => String(s.current_class_id) === String(currentClassId),
      );
    },
    enabled: !!classId(),
    staleTime: 3 * 60 * 1000,
  }));
}

/** Attendance records for a class on a specific date. */
export function createClassAttendanceQuery(
  classId: () => string | undefined,
  date: () => string | undefined,
) {
  return createQuery<AttendanceDto[]>(() => ({
    queryKey: queryKeys.assessment.attendance(classId() ?? "", date() ?? ""),
    queryFn: () => api.listAttendance(classId()!, date()!),
    enabled: !!classId() && !!date(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Subjects catalog for an organization. */
export function createSubjectsQuery(orgId: () => string) {
  return createQuery<SubjectDto[]>(() => ({
    queryKey: queryKeys.academic.subjects(orgId()),
    queryFn: () => api.listSubjects(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Single subject by ID. */
export function createSubjectQuery(subjectId: () => string | undefined) {
  return createQuery<SubjectDto>(() => ({
    queryKey: queryKeys.academic.subject(subjectId() ?? ""),
    queryFn: () => api.getSubject(subjectId()!),
    enabled: !!subjectId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Class subjects / assignments for a specific class and period. */
export function createClassSubjectsQuery(
  classId: () => string | undefined,
  periodId: () => string | undefined,
) {
  return createQuery<ClassSubjectDto[]>(() => ({
    queryKey: queryKeys.academic.classSubjects(classId() ?? "", periodId() ?? ""),
    queryFn: () => api.listClassSubjects(classId()!, periodId()!),
    enabled: !!classId() && !!periodId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** All rooms available in the organization. */
export function createRoomsQuery(orgId: () => string) {
  return createQuery<RoomDto[]>(() => ({
    queryKey: queryKeys.scheduling.rooms(orgId()),
    queryFn: () => api.listRooms(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Assessments schemes for an organization. */
export function createAssessmentsQuery(
  params: () => {
    periodId?: string;
    classSubjectId?: string;
  },
) {
  return createQuery<AssessmentSchemeDto[]>(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!params().periodId,
    staleTime: 2 * 60 * 1000,
  }));
}

/** Grades for a class subject. */
export function createGradesQuery(classSubjectId: () => string | undefined) {
  return createQuery<GradeDto[]>(() => ({
    queryKey: queryKeys.assessment.grades({ classSubjectId: classSubjectId() }),
    queryFn: () => api.listGrades({ classSubjectId: classSubjectId() }),
    enabled: !!classSubjectId(),
    staleTime: 1 * 60 * 1000,
  }));
}

/** Time slots for an academic period. */
export function createTimeSlotsQuery(
  periodId: () => string | undefined,
) {
  return createQuery<TimeSlotDto[]>(() => ({
    queryKey: queryKeys.scheduling.timeSlots(periodId() ?? ""),
    queryFn: () => api.listTimeSlots(periodId()!),
    enabled: !!periodId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Class schedule lessons. */
export function createClassScheduleQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
  classId: () => string | undefined,
) {
  return createQuery<ScheduledLessonDto[]>(() => ({
    queryKey: queryKeys.scheduling.lessons({ periodId: periodId() }),
    queryFn: () => api.listScheduledLessons({ periodId: periodId() }),
    enabled: !!orgId() && !!periodId() && !!classId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Teacher schedule lessons. */
export function createTeacherScheduleQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
  teacherId: () => string | undefined,
) {
  return createQuery<ScheduledLessonDto[]>(() => ({
    queryKey: queryKeys.scheduling.lessons({ periodId: periodId() }),
    queryFn: () => api.listScheduledLessons({ periodId: periodId() }),
    enabled: !!orgId() && !!periodId() && !!teacherId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Check student prerequisites for a subject */
export function createPrerequisitesQuery(
  studentId: () => string,
  subjectId: () => string,
) {
  return createQuery<PrerequisiteCheckResultDto[]>(() => ({
    queryKey: queryKeys.academic.prerequisites(studentId(), subjectId()),
    queryFn: () => api.checkPrerequisites(studentId(), subjectId()),
    enabled: !!studentId() && !!subjectId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Report definitions for academics domain */
export function createReportDefinitionsQuery(domain: () => string = () => "academics") {
  return createQuery<ReportDefinitionDto[]>(() => ({
    queryKey: queryKeys.report.definitions(domain()),
    queryFn: () => api.listReportDefinitions(domain()),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Calculate student rankings using authoritative backend engine */
export function createCalculateRankingsMutation() {
  return createMutation(() => ({
    mutationFn: (params: CalculateRankingsRequest): Promise<RankingItemDto[]> => {
      return api.calculateRankings(params);
    },
  }));
}

/** Generate student certificates & grade sheets */
export function createGenerateReportMutation() {
  return createMutation(() => ({
    mutationFn: (params: {
      reportCode: string;
      format?: string;
      parameters?: Record<string, unknown>;
    }) => {
      return api.generateReport(
        params.reportCode,
        params.format || "PDF",
        params.parameters || {},
      );
    },
  }));
}
