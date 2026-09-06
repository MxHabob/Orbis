/**
 * AI & Autopilot domain queries and mutations.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

export function createAutopilotConfigQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.org.settings(),
    queryFn: async () => {
      const settings = await api.getSettings();
      return settings.autopilot;
    },
    enabled: !!getOrgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function createAutopilotTasksQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["autopilot-tasks", getOrgId()],
    queryFn: async () => [],
    enabled: !!getOrgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

export function createAutopilotDiagnosticsQuery(
  orgId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["autopilot-diagnostics", getOrgId()],
    queryFn: async () => ({ status: "ok", activeRules: 12 }),
    enabled: !!getOrgId(),
    staleTime: 1 * 60 * 1000,
  }));
}

export function useConfigureAutopilot() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (vars: {
      orgId: string;
      isEnabled: boolean;
      paused: boolean;
      mode: string;
    }) => {
      const current = await api.getSettings();
      const defaultAutopilot = {
        allow_grade_edit_after_close: false,
        auto_advance_periods: false,
        auto_lock_grades: false,
        grade_entry_auto_close_days: 7,
        max_retries: 3,
        enabled: vars.isEnabled && !vars.paused,
        mode: vars.mode,
      };
      return api.updateSettings({
        ...current,
        autopilot: {
          ...defaultAutopilot,
          ...(current.autopilot ?? {}),
          enabled: vars.isEnabled && !vars.paused,
          mode: vars.mode,
        },
      });

    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.settings(),
      });
    },
  }));
}

export function useRunAutopilotTick() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_vars: { orgId: string }) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.settings(),
      });
    },
  }));
}

export function createAgentSessionsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["agent-sessions", getOrgId()],
    queryFn: async () => [],
    enabled: !!getOrgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

export function createAgentSessionLogsQuery(
  orgId: string | (() => string),
  sessionId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  const getSessionId =
    typeof sessionId === "function" ? sessionId : () => sessionId;
  return createQuery(() => ({
    queryKey: ["agent-session-logs", getOrgId(), getSessionId()],
    queryFn: async () => [],
    enabled: !!getOrgId() && !!getSessionId(),
    staleTime: 1 * 60 * 1000,
  }));
}

export function useApproveAgentAction() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_vars: {
      orgId: string;
      actionLogId: string;
      approvedBy: string;
    }) => {
      return { approved: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["agent-sessions"],
      });
    },
  }));
}

export function useRejectAgentAction() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_vars: {
      orgId: string;
      actionLogId: string;
      rejectedBy: string;
      reason: string;
    }) => {
      return { rejected: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["agent-sessions"],
      });
    },
  }));
}

