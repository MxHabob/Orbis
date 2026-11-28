/**
 * Real-time SSE cache invalidation client for multi-user / multi-branch deployments.
 *
 * Connects to `GET /api/v1/events/stream` on the remote Orbis server and
 * selectively invalidates TanStack Query caches when another user mutates data.
 */

import { queryClient } from "./query-client";

const ENTITY_TO_QUERY_ROOTS: Record<string, readonly (readonly string[])[]> = {
  students: [["student"], ["org", "dashboard"]],
  employees: [["employee"], ["org", "dashboard"]],
  assessment: [["assessment"], ["progression"], ["student"]],
  exams: [["exam"]],
  finance: [["finance"], ["org", "dashboard"]],
  academic: [["academic"]],
  calendar: [["calendar"], ["academic"]],
  organization: [["org"]],
  scheduling: [["scheduling"]],
  documents: [["document"], ["folder"]],
  iam: [["auth"], ["group"]],
  notifications: [["notifications"]],
  reports: [["report"]],
  workflows: [["workflow"], ["finance"], ["assessment"]],
  system: [["org"]],
};

class RealtimeInvalidationClient {
  private abortController: AbortController | null = null;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private baseUrl: string | null = null;
  private token: string | null = null;
  private backoffMs = 1000;
  private active = false;

  configure(baseUrl: string | null, token: string | null): void {
    const normalized = baseUrl ? baseUrl.replace(/\/$/, "") : null;
    if (this.baseUrl === normalized && this.token === token && this.active) {
      return;
    }
    this.stop();
    this.baseUrl = normalized;
    this.token = token;
    if (this.baseUrl && this.token && typeof fetch === "function") {
      this.active = true;
      this.backoffMs = 1000;
      void this.connect();
    }
  }

  stop(): void {
    this.active = false;
    if (this.reconnectTimer !== null) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
  }

  private scheduleReconnect(): void {
    if (!this.active || this.reconnectTimer !== null) return;
    const delay = this.backoffMs;
    this.backoffMs = Math.min(this.backoffMs * 2, 30_000);
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      if (this.active) void this.connect();
    }, delay);
  }

  private async connect(): Promise<void> {
    if (!this.active || !this.baseUrl || !this.token) return;
    const controller = new AbortController();
    this.abortController = controller;

    try {
      const response = await fetch(`${this.baseUrl}/api/v1/events/stream`, {
        method: "GET",
        headers: {
          Accept: "text/event-stream",
          Authorization: `Bearer ${this.token}`,
        },
        signal: controller.signal,
      });

      if (!response.ok || !response.body) {
        if (response.status === 401 || response.status === 403) {
          this.stop();
          return;
        }
        this.scheduleReconnect();
        return;
      }

      this.backoffMs = 1000;
      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";

      while (this.active) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += decoder.decode(value, { stream: true });
        buffer = this.processBuffer(buffer);
      }
    } catch {
      // Network drop or abort; reconnect only if still active.
    } finally {
      if (this.abortController === controller) {
        this.abortController = null;
      }
      this.scheduleReconnect();
    }
  }

  private processBuffer(buffer: string): string {
    const frames = buffer.split(/\r?\n\r?\n/);
    const remainder = frames.pop() ?? "";
    for (const frame of frames) {
      this.handleFrame(frame);
    }
    return remainder;
  }

  private handleFrame(frame: string): void {
    let eventName = "message";
    const dataLines: string[] = [];

    for (const line of frame.split(/\r?\n/)) {
      if (line.startsWith("event:")) {
        eventName = line.slice(6).trim();
      } else if (line.startsWith("data:")) {
        dataLines.push(line.slice(5).trim());
      }
    }

    if (eventName !== "invalidate" || dataLines.length === 0) return;

    try {
      const payload = JSON.parse(dataLines.join("\n")) as {
        entity?: string;
      };
      const roots = payload.entity
        ? ENTITY_TO_QUERY_ROOTS[payload.entity]
        : undefined;
      if (!roots) return;

      for (const queryKey of roots) {
        void queryClient.invalidateQueries({
          queryKey: [...queryKey],
        });
      }
    } catch {
      // Ignore malformed event payloads.
    }
  }
}

export const realtimeSync = new RealtimeInvalidationClient();
