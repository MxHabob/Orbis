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

export function createAutopilotDiagnosticsQuery(
  orgId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["autopilot-diagnostics", getOrgId()],
    queryFn: async () => {
      const settings = await api.getSettings();
      const isEnabled = settings.autopilot?.enabled ?? false;
      return {
        status: isEnabled ? "active" : "standby",
        mode: settings.autopilot?.mode ?? "advisory",
        autoLockGrades: settings.autopilot?.auto_lock_grades ?? false,
      };
    },
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
