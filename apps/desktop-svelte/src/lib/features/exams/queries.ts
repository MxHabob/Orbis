/**
 * Exams domain queries and mutations.
 * Wraps IPC commands with TanStack Query for automatic caching and state updates.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type { ExamSession, ExamSubject } from "./store.svelte";

/** Get all exam sessions for an organization. */
export function createExamSessionsQuery(orgId: string, cycleId?: string) {
  return createQuery({
    queryKey: ["exam-sessions", orgId, cycleId || "all"],
    queryFn: () => commands.exams.getSessions(orgId, cycleId) as Promise<ExamSession[]>,
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Get a single exam session. */
export function createExamSessionQuery(orgId: string, sessionId: string) {
  return createQuery({
    queryKey: ["exam-session", orgId, sessionId],
    queryFn: () => commands.exams.getSession(orgId, sessionId) as Promise<ExamSession>,
    enabled: !!orgId && !!sessionId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Get exam subjects. */
export function createExamSubjectsQuery(orgId: string, sessionId: string) {
  return createQuery({
    queryKey: ["exam-subjects", orgId, sessionId],
    queryFn: () => commands.exams.getSubjects(orgId, sessionId) as Promise<ExamSubject[]>,
    enabled: !!orgId && !!sessionId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Create exam session mutation. */
export function createSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      orgId: string;
      userId: string;
      data: {
        name: string;
        name_ar: string;
        session_type: string;
        start_date: string;
        end_date: string;
        cycle_id?: string;
      };
    }) =>
      commands.exams.createSession(variables.orgId, variables.userId, {
        name: variables.data.name,
        name_ar: variables.data.name_ar,
        type: variables.data.session_type,
        start_date: variables.data.start_date,
        end_date: variables.data.end_date,
        cycle_id: variables.data.cycle_id || "",
      }) as Promise<ExamSession>,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({ queryKey: ["exam-sessions", variables.orgId] });
    },
  });
}

/** Update exam session mutation. */
export function updateSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      orgId: string;
      sessionId: string;
      updates: Record<string, unknown>;
    }) =>
      commands.exams.updateSession(
        variables.orgId,
        variables.sessionId,
        variables.updates,
      ) as Promise<ExamSession>,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({ queryKey: ["exam-sessions", variables.orgId] });
      queryClient.invalidateQueries({ queryKey: ["exam-session", variables.orgId, variables.sessionId] });
    },
  });
}

/** Add exam subject mutation. */
export function addExamSubjectMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (variables: {
      orgId: string;
      sessionId: string;
      request: Record<string, unknown>;
    }) =>
      commands.exams.createSubject(variables.orgId, variables.request) as Promise<ExamSubject>,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({ queryKey: ["exam-subjects", variables.orgId, variables.sessionId] });
    },
  });
}
