// ── Students Store ───────────────────────────────────────────────────
// Svelte 5 Runes-based students filters and UI state.
// No server data is stored directly in $state.

import type { EnrollmentDto } from "$lib/api/types";
import { matchStudent } from "$lib/shared/utils/search";

class StudentsStore {
  selectedId = $state<string | null>(null);

  // Filters state
  searchQuery = $state("");
  selectedClassId = $state<string | null>(null);
  statusFilter = $state<string>("active"); // active, withdrawn, all

  /**
   * Helper to filter a student list using the store's current filters.
   */
  filterList(list: EnrollmentDto[]): EnrollmentDto[] {
    return list.filter((student) => {
      // 1. Search Query Match
      if (this.searchQuery) {
        if (!matchStudent(student, this.searchQuery)) {
          return false;
        }
      }

      // 2. Class Match
      if (this.selectedClassId) {
        if (student.class_id !== this.selectedClassId) {
          return false;
        }
      }

      // 3. Status Match
      if (this.statusFilter !== "all") {
        const status = String(student.status || "").toLowerCase();
        if (status !== this.statusFilter) {
          return false;
        }
      }

      return true;
    });
  }

  reset() {
    this.selectedId = null;
    this.searchQuery = "";
    this.selectedClassId = null;
    this.statusFilter = "active";
  }
}

export const studentsStore = new StudentsStore();
