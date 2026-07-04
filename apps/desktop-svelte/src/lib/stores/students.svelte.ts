// ── Students Store ───────────────────────────────────────────────────
// Svelte 5 Runes-based students, enrollments, and classes state.

import { commands } from "$lib/services/ipc";
import type { Enrollment, JsonRecord } from "$lib/services/ipc";
import { extractErrorMessage } from "$lib/utils/ipc-error";

class StudentsStore {
  // Core state
  list = $state<Enrollment[]>([]);
  classes = $state<JsonRecord[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);
  selectedId = $state<string | null>(null);

  // Filters state
  searchQuery = $state("");
  selectedClassId = $state<string | null>(null);
  statusFilter = $state<string>("active"); // active, withdrawn, all

  // Derived state (runes reactivity)
  filteredList = $derived.by(() => {
    return this.list.filter((student) => {
      // 1. Search Query Match
      let matchesSearch = true;
      if (this.searchQuery) {
        const query = this.searchQuery.toLowerCase().trim();
        const fullName = String(student.full_name || "").toLowerCase();
        const studentNumber = String(
          student.student_number || "",
        ).toLowerCase();
        const email = String(student.email || "").toLowerCase();
        matchesSearch =
          fullName.includes(query) ||
          studentNumber.includes(query) ||
          email.includes(query);
      }

      // 2. Class Match
      let matchesClass = true;
      if (this.selectedClassId) {
        matchesClass = student.class_id === this.selectedClassId;
      }

      // 3. Status Match
      let matchesStatus = true;
      if (this.statusFilter !== "all") {
        const status = String(student.status || "").toLowerCase();
        matchesStatus = status === this.statusFilter;
      }

      return matchesSearch && matchesClass && matchesStatus;
    });
  });

  // Load all enrollments and classes for the organization
  async load(orgId: string) {
    if (!orgId) return;
    this.isLoading = true;
    this.error = null;
    try {
      const [enrollmentData, classData] = await Promise.all([
        commands.enrollments.getAll(orgId),
        commands.classes.getAll(orgId),
      ]);
      this.list = enrollmentData;
      this.classes = classData;
    } catch (e) {
      this.error = extractErrorMessage(e);
      console.error("Error loading student/class data:", e);
    } finally {
      this.isLoading = false;
    }
  }

  // Create a new enrollment (student)
  async create(orgId: string, data: JsonRecord) {
    this.isLoading = true;
    try {
      const newEnrollment = await commands.enrollments.create(orgId, data);
      this.list = [newEnrollment, ...this.list];
      return newEnrollment;
    } catch (e) {
      this.error = extractErrorMessage(e);
      throw e;
    } finally {
      this.isLoading = false;
    }
  }

  // Optimistic UI Class Transfer
  async transfer(enrollmentId: string, newClassId: string) {
    const originalList = [...this.list];

    // Optimistically update local state
    const index = this.list.findIndex((s) => s.id === enrollmentId);
    if (index !== -1) {
      this.list[index] = { ...this.list[index], class_id: newClassId };
    }

    try {
      await commands.enrollments.transfer(enrollmentId, {
        new_class_id: newClassId,
      });
    } catch (e) {
      // Rollback on failure
      this.list = originalList;
      this.error = extractErrorMessage(e);
      throw e;
    }
  }

  // Optimistic UI Withdrawal
  async withdraw(enrollmentId: string, reason: string) {
    const originalList = [...this.list];

    // Optimistically update local state
    const index = this.list.findIndex((s) => s.id === enrollmentId);
    if (index !== -1) {
      this.list[index] = { ...this.list[index], status: "withdrawn" };
    }

    try {
      await commands.enrollments.withdraw(enrollmentId, { reason });
    } catch (e) {
      // Rollback on failure
      this.list = originalList;
      this.error = extractErrorMessage(e);
      throw e;
    }
  }
}

export const studentsStore = new StudentsStore();
