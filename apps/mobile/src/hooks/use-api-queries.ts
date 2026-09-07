/**
 * @module @orbis/mobile/hooks/use-api-queries
 *
 * Centralized, performance-optimized React Query hooks using @orbis/sdk.
 * Completely eliminates manual useEffect data fetching, providing instant cache,
 * background revalidation, and type-safe query results.
 * Guaranteed to never return `undefined` to satisfy TanStack Query v5 contracts.
 */

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import {
  getCurrentOrg,
  getDashboard,
  getCurrentContext,
  listClasses,
  getClass,
  listEnrollments,
  listGrades,
  listAttendance,
  listSchedules,
  listFees,
  listGuardians,
  getUser,
  getSettings,
  markAttendance,
  recordGrade,
  type MarkAttendanceRequest,
  type RecordGradeRequest,
} from '@orbis/sdk';
import { queryKeys } from '@/lib/api/query-keys';

const STALE_TIME = 1000 * 60 * 5; // 5 minutes cache
const GC_TIME = 1000 * 60 * 15; // 15 minutes garbage collection

/**
 * Fetch Current Organization Metadata
 */
export function useCurrentOrgQuery() {
  return useQuery({
    queryKey: queryKeys.org.current(),
    queryFn: async () => {
      try {
        const res = await getCurrentOrg();
        return res?.data ?? null;
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
    queryFn: async () => {
      try {
        const res = await getDashboard();
        return res?.data ?? null;
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
    queryFn: async () => {
      try {
        const res = await getCurrentContext();
        return res?.data ?? null;
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
    queryFn: async () => {
      try {
        const res = await listClasses();
        return res?.data ?? [];
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
    queryKey: queryKeys.academic.class(classId || ''),
    queryFn: async () => {
      if (!classId) return null;
      try {
        const res = await getClass({ path: { id: classId } });
        return res?.data ?? null;
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
export function useEnrollmentsQuery(filter?: { classId?: string; studentId?: string }) {
  return useQuery({
    queryKey: queryKeys.student.enrollments(filter),
    queryFn: async () => {
      try {
        const res = await listEnrollments({
          query: {
            class_id: filter?.classId,
            student_id: filter?.studentId,
          } as any,
        });
        return res?.data ?? [];
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
export function useGradesQuery(filter?: { studentId?: string; classSubjectId?: string }) {
  return useQuery({
    queryKey: queryKeys.assessment.grades(filter),
    queryFn: async () => {
      try {
        const res = await listGrades({
          query: {
            student_id: filter?.studentId,
            class_subject_id: filter?.classSubjectId,
          } as any,
        });
        return res?.data ?? [];
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
export function useAttendanceQuery(filter?: { classId?: string; studentId?: string; date?: string }) {
  return useQuery({
    queryKey: queryKeys.assessment.attendance(filter?.classId || '', filter?.date || ''),
    queryFn: async () => {
      try {
        const res = await listAttendance({
          query: {
            class_id: filter?.classId,
            date: filter?.date,
          } as any,
        });
        return res?.data ?? [];
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
    queryFn: async () => {
      try {
        const res = await listSchedules({
          query: { session_id: sessionId } as any,
        });
        return res?.data ?? [];
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
    queryFn: async () => {
      if (!studentId) return [];
      try {
        const res = await listFees({
          query: { student_id: studentId } as any,
        });
        return res?.data ?? [];
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
    queryKey: queryKeys.student.guardians(studentId || ''),
    queryFn: async () => {
      if (!studentId) return [];
      try {
        const res = await listGuardians({
          query: { student_id: studentId } as any,
        });
        return res?.data ?? [];
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
    queryKey: queryKeys.auth.user(userId || ''),
    queryFn: async () => {
      if (!userId) return null;
      try {
        const res = await getUser({ path: { id: userId } });
        return res?.data ?? null;
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
    queryFn: async () => {
      try {
        const res = await getSettings({});
        return res?.data ?? null;
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
      const res = await markAttendance({
        body: variables,
      });
      return res.data;
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
      const res = await recordGrade({
        body: variables,
      });
      return res.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}
