// ── Students Store ───────────────────────────────────────────────────
// Svelte 5 Runes-based students filters and UI state.
// No server data is stored directly in $state.

import type { StudentDetailDto, EnrollmentDto } from "$lib/api/types";
import { matchStudent } from "$lib/shared/utils/search";

class StudentsStore {
  selectedId = $state<string | null>(null);

  // Filters state
  searchQuery = $state("");
  selectedClassId = $state<string | null>(null);
  statusFilter = $state<string>("active"); // active, inactive, withdrawn, graduated, suspended, all

  /**
   * Helper to filter a student list using the store's current filters.
   */
  filterList<T extends StudentDetailDto | EnrollmentDto>(list: T[]): T[] {
    return list.filter((item) => {
      // 1. Search Query Match
      if (this.searchQuery) {
        if (!matchStudent(item, this.searchQuery)) {
          return false;
        }
      }

      const isStudentDetail = "student" in item;
      const classId = isStudentDetail
        ? (item as StudentDetailDto).enrollment?.section_id
        : (item as EnrollmentDto).section_id;

      // 2. Class Match
      if (this.selectedClassId) {
        if (classId !== this.selectedClassId) {
          return false;
        }
      }

      // 3. Status Match
      if (this.statusFilter !== "all") {
        const rawStatus = isStudentDetail
          ? (item as StudentDetailDto).student?.status
          : (item as EnrollmentDto).status;
        const status = String(rawStatus || "").toLowerCase();
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
