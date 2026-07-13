/**
 * Exams domain queries and mutations.
 * Wraps IPC commands with TanStack Query for automatic caching and state updates.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type { ExamSession, ExamSubject } from "@orbis/sdk-core";

/** Get all exam sessions for a period. */
export function createExamSessionsQuery(periodId?: string) {
  return createQuery({
    queryKey: ["exam-sessions", periodId || "all"],
    queryFn: () =>
      commands.exam.listSessions(periodId!) as Promise<ExamSession[]>,
    enabled: !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Get a single exam session. */
export function createExamSessionQuery(sessionId: string) {
  return createQuery({
    queryKey: ["exam-session", sessionId],
    queryFn: () => commands.exam.getSession(sessionId) as Promise<ExamSession>,
    enabled: !!sessionId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Get exam subjects. */
export function createExamSubjectsQuery(sessionId: string) {
  return createQuery({
    queryKey: ["exam-subjects", sessionId],
    queryFn: () =>
      commands.exam.listExamSubjects(sessionId) as Promise<ExamSubject[]>,
    enabled: !!sessionId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Create exam session mutation. */
export function createSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      data: {
        period_id: string;
        name: string;
        name_ar?: string;
        session_type: string;
        start_date: string;
        end_date: string;
      };
    }) => commands.exam.createSession(variables.data) as Promise<ExamSession>,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["exam-sessions"],
      });
    },
  });
}

/** Update exam session mutation. */
export function updateSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      sessionId: string;
      updates: {
        name?: string;
        status?: string;
        start_date?: string;
        end_date?: string;
      };
    }) =>
      commands.exam.updateSession(
        variables.sessionId,
        variables.updates,
      ) as Promise<ExamSession>,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["exam-sessions"],
      });
      queryClient.invalidateQueries({
        queryKey: ["exam-session", variables.sessionId],
      });
    },
  });
}

/** Add exam subject mutation. */
export function addExamSubjectMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      sessionId: string;
      data: {
        session_id: string;
        subject_id: string;
        max_score: number;
        class_subject_id?: string;
        duration_minutes?: number;
        exam_type?: string;
      };
    }) =>
      commands.exam.createExamSubject(variables.data) as Promise<ExamSubject>,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["exam-subjects", variables.sessionId],
      });
    },
  });
}
