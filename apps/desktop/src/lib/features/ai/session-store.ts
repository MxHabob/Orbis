import * as m from "$lib/paraglide/messages";
import type {
  AiToolResult,
  AiProposedAction,
  AiStoredSession,
  AiStoredMessage,
} from "$lib/api/types";
import { api } from "$lib/api";

export interface ActionWithStatus extends AiProposedAction {
  status?: "pending" | "processing" | "executed" | "rejected" | "failed";
  executedAt?: number;
  resultMessage?: string;
  reportJobId?: string;
  reportOutputFormat?: string;
  reportChecking?: boolean;
}

export interface UnpaidFeeUiRow {
  student_name: string;
  balance_due: string;
  status: string;
  due_date: string;
}

export type AiUiData = { type: "unpaid_fees"; items: UnpaidFeeUiRow[] };

export interface AiChatMessage {
  id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  timestamp: number;
  isError?: boolean;
  executedTools?: AiToolResult[];
  proposedActions?: ActionWithStatus[];
  uiData?: AiUiData[];
}

export function parseAiUiData(value: unknown): AiUiData | null {
  if (!value || typeof value !== "object" || !("type" in value)) return null;
  const candidate = value as Record<string, unknown>;
  if (candidate.type !== "unpaid_fees" || !Array.isArray(candidate.items)) {
    return null;
  }
  const items = candidate.items.filter(
    (item): item is UnpaidFeeUiRow =>
      !!item &&
      typeof item === "object" &&
      "student_name" in item &&
      typeof item.student_name === "string" &&
      "balance_due" in item &&
      typeof item.balance_due === "string" &&
      "status" in item &&
      typeof item.status === "string" &&
      "due_date" in item &&
      typeof item.due_date === "string",
  );
  if (items.length !== candidate.items.length) return null;
  return { type: "unpaid_fees", items };
}

export interface AiChatSession {
  id: string;
  title: string;
  orgId: string;
  createdAt: number;
  updatedAt: number;
  context?: Record<string, unknown> | null;
  messages: AiChatMessage[];
}

export async function loadFileSessions(
  orgId: string,
): Promise<AiChatSession[]> {
  try {
    const rawSessions = await api.aiLoadSessions(orgId || "default");
    return rawSessions.map((s) => ({
      id: s.id,
      title: s.title,
      orgId: s.org_id,
      createdAt: s.created_at,
      updatedAt: s.updated_at,
      context: (s.context as Record<string, unknown> | null) ?? null,
      messages: s.messages.map((m) => ({
        id: m.id,
        role: m.role as any,
        content: m.content,
        timestamp: m.timestamp,
        executedTools: m.executed_tools || undefined,
        proposedActions: m.proposed_actions as any,
      })),
    }));
  } catch (e) {
    console.error("Failed to load file AI sessions:", e);
    return [];
  }
}

export async function saveFileSession(
  session: AiChatSession,
): Promise<boolean> {
  try {
    const stored: AiStoredSession = {
      id: session.id,
      title: session.title,
      org_id: session.orgId,
      created_at: session.createdAt,
      updated_at: session.updatedAt,
      context: session.context || null,
      messages: session.messages.map((m) => ({
        id: m.id,
        role: m.role,
        content: m.content,
        timestamp: m.timestamp,
        executed_tools: m.executedTools,
        proposed_actions: m.proposedActions?.map(
          ({ reportChecking: _reportChecking, ...action }) => action,
        ),
      })),
    };
    return await api.aiSaveSession(stored);
  } catch (e) {
    console.error("Failed to save file AI session:", e);
    return false;
  }
}

export async function deleteFileSession(
  orgId: string,
  sessionId: string,
): Promise<boolean> {
  try {
    return await api.aiDeleteSession(orgId || "default", sessionId);
  } catch (e) {
    console.error("Failed to delete file AI session:", e);
    return false;
  }
}

export async function saveFileContext(
  orgId: string,
  context: Record<string, unknown>,
): Promise<boolean> {
  try {
    return await api.aiSaveContext(orgId || "default", context);
  } catch (e) {
    console.error("Failed to save file AI context:", e);
    return false;
  }
}

export async function loadFileContext(
  orgId: string,
): Promise<Record<string, unknown> | null> {
  try {
    return await api.aiLoadContext(orgId || "default");
  } catch (e) {
    console.error("Failed to load file AI context:", e);
    return null;
  }
}

export function createNewLocalSession(
  orgId: string,
  title: string = m.ai_ui_new_chat(),
): AiChatSession {
  return {
    id: `session_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`,
    title,
    orgId: orgId || "default",
    createdAt: Date.now(),
    updatedAt: Date.now(),
    context: null,
    messages: [],
  };
}
