/**
 * AI & Autopilot domain queries and mutations.
 * Clean, production-grade integration with backend AI Platform & Autopilot Engine.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import type {
  AiChatRequest,
  AiChatResponse,
  AiToolDefinition,
} from "$lib/api/types";

// ── AI Chat & Tools ───────────────────────────────────────────

export function createAiToolsQuery() {
  return createQuery(() => ({
    queryKey: ["ai", "tools"],
    queryFn: async (): Promise<AiToolDefinition[]> => {
      return api.aiListTools();
    },
    staleTime: 10 * 60 * 1000,
  }));
}

export function useAiChat() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (req: AiChatRequest): Promise<AiChatResponse> => {
      return api.aiChat(req);
    },
    onSuccess: () => {
      // Invalidate related domain queries if consequential actions were executed
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

// ── Autopilot Configuration & Engine ──────────────────────────

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

export function createAutopilotTasksQuery(
  orgId: string | (() => string),
  params?: () => { status?: string; task_type?: string; limit?: number; offset?: number },
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.autopilot.tasks(params?.()),
    queryFn: async () => {
      const res = await api.autopilotListTasks(params?.());
      return res.items;
    },
    enabled: !!getOrgId(),
    staleTime: 10 * 1000,
    refetchInterval: 30 * 1000,
  }));
}

export function createAutopilotDiagnosticsQuery(
  orgId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.autopilot.diagnostics(),
    queryFn: async () => {
      return api.autopilotGetDiagnostics();
    },
    enabled: !!getOrgId(),
    staleTime: 30 * 1000,
    refetchInterval: 60 * 1000,
  }));
}

export function useRunAutopilotTick() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (vars?: { orgId?: string; dryRun?: boolean }) => {
      return api.autopilotRunTick(vars?.dryRun ?? false);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.autopilot.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.finance.all });
    },
  }));
}

export function useExecuteAutopilotTask() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (taskId: string) => {
      return api.autopilotExecuteTask(taskId);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.autopilot.all });
    },
  }));
}

export function useRetryAutopilotTask() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (taskId: string) => {
      return api.autopilotRetryTask(taskId);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.autopilot.all });
    },
  }));
}

export function useDismissAutopilotTask() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (taskId: string) => {
      return api.autopilotDismissTask(taskId);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.autopilot.all });
    },
  }));
}

export function useConfigureAutopilot() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (vars: {
      orgId: string;
      isEnabled: boolean;
      paused?: boolean;
      mode: string;
      auto_advance_periods?: boolean;
      auto_lock_grades?: boolean;
      auto_open_enrollment?: boolean;
      auto_close_enrollment?: boolean;
      auto_record_attendance?: boolean;
      auto_fee_assignment?: boolean;
      auto_grade_entry_reminder?: boolean;
      auto_period_lock?: boolean;
      auto_promotion?: boolean;
      auto_exam_scheduling?: boolean;
      auto_report_generation?: boolean;
    }) => {
      const current = await api.getSettings();
      const defaultAutopilot = {
        allow_grade_edit_after_close: false,
        auto_advance_periods: false,
        auto_lock_grades: false,
        auto_open_enrollment: false,
        auto_close_enrollment: true,
        auto_record_attendance: true,
        auto_fee_assignment: true,
        auto_grade_entry_reminder: true,
        auto_period_lock: false,
        auto_promotion: false,
        auto_exam_scheduling: false,
        auto_report_generation: true,
        grace_days: 3,
        notify_before_deadline_hours: 24,
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
          ...vars,
          enabled: vars.isEnabled && !vars.paused,
          mode: vars.mode,
        },
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.settings(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.autopilot.all,
      });
    },
  }));
}
