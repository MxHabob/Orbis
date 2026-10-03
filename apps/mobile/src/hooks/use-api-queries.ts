/**
 * @module @orbis/mobile/hooks/use-api-queries
 *
 * Centralized, performance-optimized React Query hooks using the unified `api` client.
 * Eliminates manual data fetching, providing instant cache,
 * background revalidation, and type-safe query results.
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
 * Helper to safely extract response data from SDK calls
 */
function extractData<T>(res: any): T {
  return res?.data !== undefined ? res.data : res;
}

/**
 * Fetch Current Organization Metadata
 */
export function useCurrentOrgQuery() {
  return useQuery({
    queryKey: queryKeys.org.current(),
    queryFn: async (): Promise<OrgDto | null> => {
      try {
        const res = await api.getCurrentOrg({ throwOnError: true });
        return extractData<OrgDto>(res);
      } catch {
        return null;
      }
    },
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
        const res = await api.getDashboard({ throwOnError: true });
        return extractData<DashboardCountsDto>(res);
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
        const res = await api.getCurrentContext({ throwOnError: true });
        return extractData<AcademicContextResponse>(res);
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
        const res = await api.listClasses({ throwOnError: true });
        return extractData<ClassDto[]>(res) ?? [];
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
        const res = await api.getClass({ path: { id: classId }, throwOnError: true });
        return extractData<ClassDto>(res);
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
        const res = await api.listEnrollments({
          query: { class_id: filter?.classId, student_id: filter?.studentId } as any,
          throwOnError: true,
        });
        return extractData<EnrollmentDto[]>(res) ?? [];
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
        const res = await api.listGrades({
          query: { student_id: filter?.studentId, class_subject_id: filter?.classSubjectId } as any,
          throwOnError: true,
        });
        return extractData<GradeDto[]>(res) ?? [];
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
        const res = await api.listAttendance({
          query: { class_id: filter?.classId, student_id: filter?.studentId, date: filter?.date } as any,
          throwOnError: true,
        });
        return extractData<AttendanceDto[]>(res) ?? [];
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
        const res = await api.listLessons({
          query: { period_id: sessionId } as any,
          throwOnError: true,
        });
        return extractData<ScheduledLessonDto[]>(res) ?? [];
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
        const res = await api.listFees({
          query: { student_id: studentId } as any,
          throwOnError: true,
        });
        return extractData<StudentFeeDto[]>(res) ?? [];
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
        const res = await api.listGuardians({
          query: { student_id: studentId },
          throwOnError: true,
        });
        return extractData<GuardianDto[]>(res) ?? [];
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
        const res = await api.getUser({
          path: { id: userId },
          throwOnError: true,
        });
        return extractData<UserDto>(res);
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
        const res = await api.getSettings({ throwOnError: true });
        return extractData<OrganizationSettingsDto>(res);
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
      const res = await api.markAttendance({ body: variables, throwOnError: true });
      return extractData<AttendanceDto>(res);
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
      const res = await api.recordGrade({ body: variables, throwOnError: true });
      return extractData<GradeDto>(res);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}
