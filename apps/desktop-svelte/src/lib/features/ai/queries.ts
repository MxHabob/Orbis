import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Autopilot config for an organization. */
export function createAutopilotConfigQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-config", orgId],
    queryFn: () => commands.ai.autopilot.getStatus(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Autopilot tasks. */
export function createAutopilotTasksQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-tasks", orgId],
    queryFn: () => commands.ai.autopilot.getTasks(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Autopilot diagnostics report. */
export function createAutopilotDiagnosticsQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-diagnostics", orgId],
    queryFn: () => commands.ai.autopilot.getDiagnostics(orgId),
    enabled: !!orgId,
    staleTime: 1 * 60 * 1000,
  });
}

/** Mutation to configure autopilot. */
export function useConfigureAutopilot() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; isEnabled: boolean; paused: boolean; mode: string }) =>
      commands.ai.autopilot.configure(vars.orgId, vars.isEnabled, vars.paused, vars.mode),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["autopilot-config", vars.orgId] });
    },
  });
}

/** Mutation to trigger manual autopilot tick. */
export function useRunAutopilotTick() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string }) =>
      commands.ai.autopilot.runTick(vars.orgId),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["autopilot-config", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["autopilot-tasks", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["autopilot-diagnostics", vars.orgId] });
    },
  });
}

/** Agent sessions list. */
export function createAgentSessionsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery({
    get queryKey() { return ["agent-sessions", getOrgId()]; },
    queryFn: () => commands.ai.agent.getSessions(getOrgId()),
    get enabled() { return !!getOrgId(); },
    staleTime: 2 * 60 * 1000,
  });
}

/** Agent session activity logs. */
export function createAgentSessionLogsQuery(
  orgId: string | (() => string),
  sessionId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  const getSessionId = typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery({
    get queryKey() { return ["agent-session-logs", getOrgId(), getSessionId()]; },
    queryFn: () => commands.ai.agent.getSessionLogs(getOrgId(), getSessionId()),
    get enabled() { return !!getOrgId() && !!getSessionId(); },
    staleTime: 1 * 60 * 1000,
  });
}

/** Mutation to approve agent action. */
export function useApproveAgentAction() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; actionLogId: string; approvedBy: string }) =>
      commands.ai.agent.approveAction(vars.orgId, vars.actionLogId, vars.approvedBy),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["agent-session-logs", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["agent-sessions", vars.orgId] });
    },
  });
}

/** Mutation to reject agent action. */
export function useRejectAgentAction() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; actionLogId: string; rejectedBy: string; reason: string }) =>
      commands.ai.agent.rejectAction(vars.orgId, vars.actionLogId, vars.rejectedBy, vars.reason),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["agent-session-logs", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["agent-sessions", vars.orgId] });
    },
  });
}
