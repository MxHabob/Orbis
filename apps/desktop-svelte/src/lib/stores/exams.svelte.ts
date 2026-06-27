// ── Exams Store ──────────────────────────────────────────────────────
// Svelte 5 Runes-based exam sessions and related state.
// Calls real IPC commands → Tauri → SQLite database.

import { commands } from "$lib/services/ipc";

export interface ExamSession {
  id: string;
  org_id: string;
  cycle_id: string;
  name: string;
  name_ar: string;
  session_type: string;
  status: string;
  start_date: string;
  end_date: string;
  subjects_count?: number;
  students_count?: number;
  [key: string]: unknown;
}

export interface ExamSubject {
  id: string;
  exam_session_id: string;
  subject_id: string;
  max_score: number;
  duration_minutes: number;
  exam_type: string;
  [key: string]: unknown;
}

class ExamsStore {
  // Core state
  sessions = $state<ExamSession[]>([]);
  activeSession = $state<ExamSession | null>(null);
  subjects = $state<ExamSubject[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // Filters
  searchQuery = $state("");
  statusFilter = $state<string>("all");

  // Derived
  filteredSessions = $derived.by(() => {
    return this.sessions.filter((s) => {
      const matchesSearch = !this.searchQuery ||
        (s.name || "").toLowerCase().includes(this.searchQuery.toLowerCase()) ||
        (s.name_ar || "").toLowerCase().includes(this.searchQuery.toLowerCase());

      const matchesStatus = this.statusFilter === "all" || s.status === this.statusFilter;

      return matchesSearch && matchesStatus;
    });
  });

  activeSessions = $derived(this.sessions.filter((s) => s.status === "active"));
  draftSessions = $derived(this.sessions.filter((s) => s.status === "draft"));
  completedSessions = $derived(this.sessions.filter((s) => s.status === "completed"));

  // ── Load Sessions from DB ───────────────────────────────────────────

  async loadSessions(orgId: string, cycleId?: string) {
    if (!orgId) return;
    this.isLoading = true;
    this.error = null;
    try {
      const list = await commands.exams.getSessions(orgId, cycleId);
      this.sessions = list as unknown as ExamSession[];
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      console.error("Failed to load exam sessions:", e);
    } finally {
      this.isLoading = false;
    }
  }

  // ── Create Session (saves to DB) ────────────────────────────────────

  async createSession(orgId: string, userId: string, data: {
    name: string;
    name_ar: string;
    session_type: string;
    start_date: string;
    end_date: string;
    cycle_id?: string;
  }): Promise<ExamSession | null> {
    try {
      const result = await commands.exams.createSession(orgId, userId, {
        name: data.name,
        name_ar: data.name_ar,
        type: data.session_type,
        start_date: data.start_date,
        end_date: data.end_date,
        cycle_id: data.cycle_id || "",
      });
      const session = result as unknown as ExamSession;
      this.sessions = [...this.sessions, session];
      return session;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  // ── Update Session ──────────────────────────────────────────────────

  async updateSession(orgId: string, sessionId: string, updates: Record<string, unknown>): Promise<ExamSession | null> {
    try {
      const result = await commands.exams.updateSession(orgId, sessionId, updates);
      const updated = result as unknown as ExamSession;
      this.sessions = this.sessions.map((s) => s.id === sessionId ? updated : s);
      if (this.activeSession?.id === sessionId) {
        this.activeSession = updated;
      }
      return updated;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  // ── Change Session Status ────────────────────────────────────────────
  // Status transitions go through updateSession with status field

  async changeStatus(orgId: string, sessionId: string, newStatus: string) {
    return this.updateSession(orgId, sessionId, { status: newStatus });
  }

  // ── Subjects ────────────────────────────────────────────────────────

  async loadSubjects(orgId: string, sessionId: string) {
    try {
      const list = await commands.exams.getSubjects(orgId, sessionId);
      this.subjects = list as unknown as ExamSubject[];
    } catch (e) {
      console.error("Failed to load subjects:", e);
    }
  }

  async addSubject(orgId: string, request: Record<string, unknown>) {
    try {
      const result = await commands.exams.createSubject(orgId, request);
      const subject = result as unknown as ExamSubject;
      this.subjects = [...this.subjects, subject];
      return subject;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  // ── Load Single Session ─────────────────────────────────────────────

  async loadSession(orgId: string, sessionId: string) {
    this.isLoading = true;
    try {
      const result = await commands.exams.getSession(orgId, sessionId);
      this.activeSession = result as unknown as ExamSession;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.isLoading = false;
    }
  }

  setActiveSession(session: ExamSession | null) {
    this.activeSession = session;
  }

  reset() {
    this.sessions = [];
    this.activeSession = null;
    this.subjects = [];
    this.isLoading = false;
    this.error = null;
    this.searchQuery = "";
    this.statusFilter = "all";
  }
}

export const examsStore = new ExamsStore();
