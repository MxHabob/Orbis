/**
 * Attendance domain queries and mutations.
 * Provides TanStack Query hooks for attendance fetching and marking.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { api, queryKeys, type MarkAttendanceRequest } from "$lib/api";

/** List attendance records for a specific class and date */
export function createAttendanceQuery(
  classId: () => string,
  date: () => string,
) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.attendance(classId(), date()),
    queryFn: () => api.listAttendance(classId(), date()),
    enabled: !!classId() && !!date(),
    staleTime: 60 * 1000,
  }));
}

/** Mark attendance for a single student */
export function createMarkAttendanceMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: MarkAttendanceRequest) => api.markAttendance(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.assessment.attendance(vars.section_id, vars.date),
      });
    },
  }));
}
