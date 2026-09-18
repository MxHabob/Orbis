import type { AiToolResult, AiProposedAction, AiStoredSession, AiStoredMessage } from "$lib/api/types";
import { api } from "$lib/api";

export interface ActionWithStatus extends AiProposedAction {
  status?: "pending" | "executed" | "rejected";
  executedAt?: number;
  resultMessage?: string;
}

export interface AiChatMessage {
  id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  timestamp: number;
  executedTools?: AiToolResult[];
  proposedActions?: ActionWithStatus[];
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

export async function loadFileSessions(orgId: string): Promise<AiChatSession[]> {
  try {
    const rawSessions = await api.aiLoadSessions(orgId || "default");
    return rawSessions.map((s) => ({
      id: s.id,
      title: s.title,
      orgId: s.org_id,
      createdAt: s.created_at,
      updatedAt: s.updated_at,
      context: s.context || null,
      messages: s.messages.map((m) => ({
        id: m.id,
        role: m.role as any,
        content: m.content,
        timestamp: m.timestamp,
        executedTools: m.executed_tools,
        proposedActions: m.proposed_actions as any,
      })),
    }));
  } catch (e) {
    console.error("Failed to load file AI sessions:", e);
    return [];
  }
}

export async function saveFileSession(session: AiChatSession): Promise<boolean> {
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
        proposed_actions: m.proposedActions as any,
      })),
    };
    return await api.aiSaveSession(stored);
  } catch (e) {
    console.error("Failed to save file AI session:", e);
    return false;
  }
}

export async function deleteFileSession(orgId: string, sessionId: string): Promise<boolean> {
  try {
    return await api.aiDeleteSession(orgId || "default", sessionId);
  } catch (e) {
    console.error("Failed to delete file AI session:", e);
    return false;
  }
}

export async function saveFileContext(orgId: string, context: Record<string, unknown>): Promise<boolean> {
  try {
    return await api.aiSaveContext(orgId || "default", context);
  } catch (e) {
    console.error("Failed to save file AI context:", e);
    return false;
  }
}

export async function loadFileContext(orgId: string): Promise<Record<string, unknown> | null> {
  try {
    return await api.aiLoadContext(orgId || "default");
  } catch (e) {
    console.error("Failed to load file AI context:", e);
    return null;
  }
}

export function createNewLocalSession(orgId: string, title: string = "محادثة جديدة"): AiChatSession {
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
