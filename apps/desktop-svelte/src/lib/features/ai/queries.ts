/**
 * AI domain queries and mutations.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createAutopilotConfigQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-config", orgId],
    queryFn: () => commands.platform.getAutopilotStatus(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

export function createAutopilotTasksQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-tasks", orgId],
    queryFn: () => commands.platform.getAutopilotTasks(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

export function createAutopilotDiagnosticsQuery(orgId: string) {
  return createQuery({
    queryKey: ["autopilot-diagnostics", orgId],
    queryFn: () => commands.platform.getAutopilotDiagnostics(orgId),
    enabled: !!orgId,
    staleTime: 1 * 60 * 1000,
  });
}

export function useConfigureAutopilot() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; isEnabled: boolean; paused: boolean; mode: string }) =>
      commands.platform.configureAutopilot(vars.orgId, vars.isEnabled, vars.paused, vars.mode),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["autopilot-config", vars.orgId] });
    },
  });
}

export function useRunAutopilotTick() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string }) => commands.platform.runAutopilotTick(vars.orgId),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["autopilot-config", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["autopilot-tasks", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["autopilot-diagnostics", vars.orgId] });
    },
  });
}

export function createAgentSessionsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery({
    get queryKey() { return ["agent-sessions", getOrgId()]; },
    queryFn: () => commands.platform.listAgentSessions(getOrgId()),
    get enabled() { return !!getOrgId(); },
    staleTime: 2 * 60 * 1000,
  });
}

export function createAgentSessionLogsQuery(
  orgId: string | (() => string),
  sessionId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  const getSessionId = typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery({
    get queryKey() { return ["agent-session-logs", getOrgId(), getSessionId()]; },
    queryFn: () => commands.platform.getAgentSessionLogs(getOrgId(), getSessionId()),
    get enabled() { return !!getOrgId() && !!getSessionId(); },
    staleTime: 1 * 60 * 1000,
  });
}

export function useApproveAgentAction() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; actionLogId: string; approvedBy: string }) =>
      commands.platform.approveAgentAction(vars.orgId, vars.actionLogId, vars.approvedBy),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["agent-session-logs", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["agent-sessions", vars.orgId] });
    },
  });
}

export function useRejectAgentAction() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (vars: { orgId: string; actionLogId: string; rejectedBy: string; reason: string }) =>
      commands.platform.rejectAgentAction(vars.orgId, vars.actionLogId, vars.rejectedBy, vars.reason),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ["agent-session-logs", vars.orgId] });
      queryClient.invalidateQueries({ queryKey: ["agent-sessions", vars.orgId] });
    },
  });
}
