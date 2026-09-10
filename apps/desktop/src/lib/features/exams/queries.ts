/**
 * Exams domain queries and mutations.
 * Wraps OrbisApiClient with TanStack Query for automatic caching and state updates.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import type {
  ExamSessionDto,
  ExamScheduleDto,
  CreateExamSessionRequest,
  ScheduleExamRequest,
} from "@orbis/sdk";

/** Get all exam sessions for a period. */
export function createExamSessionsQuery(periodId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: queryKeys.exam.sessions(periodId()!),
    queryFn: () => api.listExamSessions(periodId()!),
    enabled: !!periodId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Get a single exam session. */
export function createExamSessionQuery(sessionId: string | (() => string)) {
  const getSessionId =
    typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery(() => ({
    queryKey: queryKeys.exam.session(getSessionId()),
    queryFn: () => api.getExamSession(getSessionId()),
    enabled: !!getSessionId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Get exam schedules for a session. */
export function createExamSubjectsQuery(sessionId: string | (() => string)) {
  const getSessionId =
    typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery(() => ({
    queryKey: queryKeys.exam.schedules(getSessionId()),
    queryFn: () => api.listExamSchedules(getSessionId()),
    enabled: !!getSessionId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Create exam session mutation. */
export function createSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { data: CreateExamSessionRequest }) =>
      api.createExamSession(variables.data),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.all,
      });
    },
  }));
}

/** Schedule exam mutation. */
export function addExamSubjectMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { data: ScheduleExamRequest }) =>
      api.scheduleExam(variables.data),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.schedules(variables.data.exam_session_id),
      });
    },
  }));
}
