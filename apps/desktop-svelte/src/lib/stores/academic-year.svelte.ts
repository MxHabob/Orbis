/**
 * Academic Year Store — UI state for year management.
 * Uses Svelte 5 runes ($state, $derived).
 * Data fetching is handled via TanStack Query (see year-queries.ts).
 */

import type { AcademicYear, AcademicYearInfo } from "@orbis/desktop-client/types";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

class AcademicYearStore {
  /** The currently loaded year (from the backend). */
  currentYear = $state<AcademicYear | null>(null);

  /** All available years for the organization. */
  years = $state<AcademicYearInfo[]>([]);

  /** Whether the app is currently switching years. */
  isSwitching = $state(false);

  /** Loading state for initial fetch. */
  isLoading = $state(false);

  // ── Derived ───────────────────────────────────────────────────

  /** Whether the current year is archived (read-only mode). */
  isArchived = $derived(this.currentYear?.status === "archived");

  /** Display label for the current year. */
  yearLabel = $derived(this.currentYear?.name ?? "");

  /** Display label (Arabic) for the current year. */
  yearLabelAr = $derived(this.currentYear?.name_ar ?? this.currentYear?.name ?? "");

  /** Status badge text. */
  statusBadge = $derived<"active" | "archived" | "promoting">(
    (this.currentYear?.status as "active" | "archived" | "promoting") ?? "active",
  );

  /** Whether any years exist (for first-launch experience). */
  hasYears = $derived(this.years.length > 0);

  // ── Actions ───────────────────────────────────────────────────

  /** Load the current year from the backend. Fire-and-forget safe. */
  async loadCurrentYear() {
    try {
      this.currentYear = await commands.academicYears.getCurrent();
    } catch (e: unknown) {
      // Silently fail — registry may not be initialized yet
      console.warn("Failed to load current year:", extractErrorMessage(e));
    }
  }

  /** Load all years for an organization. */
  async loadYears(orgId: string) {
    this.isLoading = true;
    try {
      this.years = await commands.academicYears.list(orgId);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      this.isLoading = false;
    }
  }

  /** Switch to a different academic year. */
  async switchYear(yearId: string) {
    if (this.isSwitching) return;
    this.isSwitching = true;
    try {
      await commands.academicYears.switch(yearId);
      // Reload current year info
      await this.loadCurrentYear();
      toast.success(`Switched to ${this.yearLabel}`);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      this.isSwitching = false;
    }
  }
}

export const academicYearStore = new AcademicYearStore();
