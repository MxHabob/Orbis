// ── Exams Store ──────────────────────────────────────────────────────
// Svelte 5 Runes-based exam sessions and related UI state.
// No server data is stored directly in $state lists.

export interface ExamSession {
  id: string;
  org_id: string;
  period_id: string;
  name: string;
  name_ar?: string | null;
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
  // UI-only selection state
  activeSession = $state<ExamSession | null>(null);

  // Filters
  searchQuery = $state("");
  statusFilter = $state<string>("all");

  /**
   * Helper to filter session lists using the store's current filters.
   */
  filterSessions(sessions: ExamSession[]): ExamSession[] {
    return sessions.filter((s) => {
      const matchesSearch =
        !this.searchQuery ||
        (s.name || "").toLowerCase().includes(this.searchQuery.toLowerCase()) ||
        (s.name_ar || "")
          .toLowerCase()
          .includes(this.searchQuery.toLowerCase());

      const matchesStatus =
        this.statusFilter === "all" || s.status === this.statusFilter;

      return matchesSearch && matchesStatus;
    });
  }

  setActiveSession(session: ExamSession | null) {
    this.activeSession = session;
  }

  reset() {
    this.activeSession = null;
    this.searchQuery = "";
    this.statusFilter = "all";
  }
}

export const examsStore = new ExamsStore();
