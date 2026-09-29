import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import type {
  ExamSessionDto,
  ExamScheduleDto,
  SeatingDto,
  SecretDto,
  ProctorDto,
  CreateExamSessionRequest,
  UpdateExamSessionRequest,
  ScheduleExamRequest,
  AssignSeatingRequest,
  GenerateSeatingRequest,
  AssignSecretRequest,
  GenerateSecretsRequest,
  AssignProctorRequest,
} from "$lib/api/types";

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

/** Get seating numbers for a session. */
export function createExamSeatingQuery(sessionId: string | (() => string)) {
  const getSessionId =
    typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery(() => ({
    queryKey: queryKeys.exam.seating(getSessionId()),
    queryFn: () => api.listSeating(getSessionId()),
    enabled: !!getSessionId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Get secret numbers for a schedule. */
export function createExamSecretsQuery(scheduleId: string | (() => string)) {
  const getScheduleId =
    typeof scheduleId === "function" ? scheduleId : () => scheduleId;
  return createQuery(() => ({
    queryKey: queryKeys.exam.secrets(getScheduleId()),
    queryFn: () => api.listSecrets(getScheduleId()),
    enabled: !!getScheduleId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Get proctor assignments for a schedule. */
export function createExamProctorsQuery(scheduleId: string | (() => string)) {
  const getScheduleId =
    typeof scheduleId === "function" ? scheduleId : () => scheduleId;
  return createQuery(() => ({
    queryKey: queryKeys.exam.proctors(getScheduleId()),
    queryFn: () => api.listProctors(getScheduleId()),
    enabled: !!getScheduleId(),
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

/** Update exam session mutation. */
export function createUpdateSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; data: UpdateExamSessionRequest }) =>
      api.updateExamSession(variables.id, variables.data),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.session(variables.id),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.all,
      });
    },
  }));
}

/** Delete exam session mutation. */
export function createDeleteSessionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteExamSession(id),
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

/** Delete exam schedule mutation. */
export function createDeleteScheduleMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; sessionId: string }) =>
      api.deleteExamSchedule(variables.id),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.schedules(variables.sessionId),
      });
    },
  }));
}

/** Generate seating mutation. */
export function createGenerateSeatingMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (data: GenerateSeatingRequest) => api.generateSeating(data),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.seating(variables.exam_session_id),
      });
    },
  }));
}

/** Delete seating mutation. */
export function createDeleteSeatingMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; sessionId: string }) =>
      api.deleteSeating(variables.id),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.seating(variables.sessionId),
      });
    },
  }));
}

/** Generate secrets mutation. */
export function createGenerateSecretsMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (data: GenerateSecretsRequest) => api.generateSecrets(data),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.secrets(variables.exam_schedule_id),
      });
    },
  }));
}

/** Delete secret mutation. */
export function createDeleteSecretMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; scheduleId: string }) =>
      api.deleteSecret(variables.id),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.secrets(variables.scheduleId),
      });
    },
  }));
}

/** Assign proctor mutation. */
export function createAssignProctorMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (data: AssignProctorRequest) => api.assignProctor(data),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.proctors(variables.exam_schedule_id),
      });
    },
  }));
}

/** Delete proctor mutation. */
export function createDeleteProctorMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (variables: { id: string; scheduleId: string }) =>
      api.deleteProctor(variables.id),
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.exam.proctors(variables.scheduleId),
      });
    },
  }));
}
