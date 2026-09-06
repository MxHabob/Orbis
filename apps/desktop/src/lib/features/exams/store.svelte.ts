// ── Exams Store ──────────────────────────────────────────────────────
// Svelte 5 Runes-based exam sessions and related UI state.
// No server data is stored directly in $state lists.

import type { ExamSessionDto } from "@orbis/sdk";

class ExamsStore {
  // UI-only selection state
  activeSession = $state<ExamSessionDto | null>(null);

  // Filters
  searchQuery = $state("");
  statusFilter = $state<string>("all");

  /**
   * Helper to filter session lists using the store's current filters.
   */
  filterSessions(sessions: ExamSessionDto[]): ExamSessionDto[] {
    return sessions.filter((s) => {
      const matchesSearch =
        !this.searchQuery ||
        (s.name || "").toLowerCase().includes(this.searchQuery.toLowerCase());

      const matchesStatus =
        this.statusFilter === "all" || s.status === this.statusFilter;

      return matchesSearch && matchesStatus;
    });
  }

  setActiveSession(session: ExamSessionDto | null) {
    this.activeSession = session;
  }

  reset() {
    this.activeSession = null;
    this.searchQuery = "";
    this.statusFilter = "all";
  }
}

export const examsStore = new ExamsStore();

