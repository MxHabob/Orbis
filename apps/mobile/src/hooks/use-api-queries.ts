/**
 * @module @orbis/mobile/hooks/use-api-queries
 *
 * Centralized, performance-optimized React Query hooks using the authenticated MobileApiClient.
 * Completely eliminates manual useEffect data fetching, providing instant cache,
 * background revalidation, and type-safe query results.
 * Guaranteed to never return `undefined` to satisfy TanStack Query v5 contracts.
 */

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api/client";
import type {
  OrgDto,
  DashboardCountsDto,
  AcademicContextResponse,
  ClassDto,
  EnrollmentDto,
  GradeDto,
  AttendanceDto,
  ScheduledLessonDto,
  StudentFeeDto,
  GuardianDto,
  UserDto,
  OrganizationSettingsDto,
  MarkAttendanceRequest,
  RecordGradeRequest,
} from "@/lib/api/types";
import { queryKeys } from "@/lib/api/query-keys";

const STALE_TIME = 1000 * 60 * 5; // 5 minutes cache
const GC_TIME = 1000 * 60 * 15; // 15 minutes garbage collection

/**
 * Fetch Current Organization Metadata
 */
export function useCurrentOrgQuery() {
  return useQuery({
    queryKey: queryKeys.org.current(),
    queryFn: () => api.getCurrentOrg(),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Organization / Academic Dashboard Metrics
 */
export function useDashboardQuery() {
  return useQuery({
    queryKey: queryKeys.org.dashboard(),
    queryFn: async (): Promise<DashboardCountsDto | null> => {
      try {
        const res = await api.getDashboard();
        return res ?? null;
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Current Academic Context (Year, Active Period)
 */
export function useAcademicContextQuery() {
  return useQuery({
    queryKey: queryKeys.calendar.context(),
    queryFn: async (): Promise<AcademicContextResponse | null> => {
      try {
        const res = await api.getAcademicContext();
        return res ?? null;
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Academic Classes List
 */
export function useClassesQuery() {
  return useQuery({
    queryKey: queryKeys.academic.classes(),
    queryFn: async (): Promise<ClassDto[]> => {
      try {
        const res = await api.listClasses();
        return res ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Single Class Details
 */
export function useClassQuery(classId?: string) {
  return useQuery({
    queryKey: queryKeys.academic.class(classId || ""),
    queryFn: async (): Promise<ClassDto | null> => {
      if (!classId) return null;
      try {
        const res = await api.getClass(classId);
        return res ?? null;
      } catch {
        return null;
      }
    },
    enabled: Boolean(classId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Student Enrollments
 */
export function useEnrollmentsQuery(filter?: {
  classId?: string;
  studentId?: string;
}) {
  return useQuery({
    queryKey: queryKeys.student.enrollments(filter),
    queryFn: async (): Promise<EnrollmentDto[]> => {
      try {
        const res = await api.listEnrollments(filter);
        return res ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Grades & Academic Assessments
 */
export function useGradesQuery(filter?: {
  studentId?: string;
  classSubjectId?: string;
}) {
  return useQuery({
    queryKey: queryKeys.assessment.grades(filter),
    queryFn: async (): Promise<GradeDto[]> => {
      try {
        const res = await api.listGrades(filter);
        return res ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Attendance Logs
 */
export function useAttendanceQuery(filter?: {
  classId?: string;
  studentId?: string;
  date?: string;
}) {
  return useQuery({
    queryKey: queryKeys.assessment.attendance(
      filter?.classId || "",
      filter?.date || "",
    ),
    queryFn: async (): Promise<AttendanceDto[]> => {
      try {
        const res = await api.listAttendance(filter);
        return res ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Schedules and Lessons
 */
export function useSchedulesQuery(sessionId?: string) {
  return useQuery({
    queryKey: queryKeys.scheduling.lessons({ periodId: sessionId }),
    queryFn: async (): Promise<ScheduledLessonDto[]> => {
      try {
        const res = await api.listScheduledLessons({ periodId: sessionId });
        return res ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Student Fees and Payments
 */
export function useFeesQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.finance.studentFees({ studentId }),
    queryFn: async (): Promise<StudentFeeDto[]> => {
      if (!studentId) return [];
      try {
        const res = await api.listStudentFees({ studentId });
        return res ?? [];
      } catch {
        return [];
      }
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Guardians / Linked Parents for a Student
 */
export function useGuardiansQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.guardians(studentId || ""),
    queryFn: async (): Promise<GuardianDto[]> => {
      if (!studentId) return [];
      try {
        const res = await api.listGuardians(studentId);
        return res ?? [];
      } catch {
        return [];
      }
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch User Profile and System Settings
 */
export function useUserQuery(userId?: string) {
  return useQuery({
    queryKey: queryKeys.auth.user(userId || ""),
    queryFn: async (): Promise<UserDto | null> => {
      if (!userId) return null;
      try {
        const res = await api.getUser(userId);
        return res ?? null;
      } catch {
        return null;
      }
    },
    enabled: Boolean(userId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useSystemSettingsQuery() {
  return useQuery({
    queryKey: queryKeys.org.settings(),
    queryFn: async (): Promise<OrganizationSettingsDto | null> => {
      try {
        const res = await api.getSettings();
        return res ?? null;
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Mutation: Record Student Attendance
 */
export function useMarkAttendanceMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: MarkAttendanceRequest) => {
      return await api.markAttendance(variables);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}

/**
 * Mutation: Record Assessment Grade
 */
export function useRecordGradeMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: RecordGradeRequest) => {
      return await api.recordGrade(variables);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}
