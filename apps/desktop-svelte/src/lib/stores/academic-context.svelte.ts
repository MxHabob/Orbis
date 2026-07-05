/**
 * Academic Context Store — centralized academic state for the entire app.
 *
 * Replaces the two separate stores:
 * - `academic-year.svelte.ts` (year-level state)
 * - `features/academics/store.svelte.ts` (cycle/class/subject selection)
 *
 * This store combines:
 * 1. Backend-resolved context (year, active cycle, all cycles, phase)
 * 2. UI selections (cycle override, class, subject) persisted in localStorage
 *
 * All modules should import `academicContext` and use `effectiveCycleId`
 * instead of manually asking for the cycle.
 */

import type {
  AcademicYear,
  AcademicYearInfo,
  AcademicContextResponse,
  AcademicCycleContext,
  AcademicCycleSummary,
  CyclePhase,
} from "@orbis/desktop-client/types";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

class AcademicContextStore {
  // ── Backend-resolved state ──────────────────────────────────────

  /** Current year from registry. */
  currentYear = $state<AcademicYear | null>(null);

  /** Auto-resolved active cycle (by date or is_current flag). */
  activeCycle = $state<AcademicCycleContext | null>(null);

  /** All cycles in the current year. */
  allCycles = $state<AcademicCycleSummary[]>([]);

  /** All available years for the organization. */
  years = $state<AcademicYearInfo[]>([]);

  /** Year status from backend. */
  yearStatus = $state<"active" | "archived" | "promoting">("active");

  /** Whether the current year is read-only (from backend). */
  private _isReadOnly = $state(false);

  // ── UI state ────────────────────────────────────────────────────

  /** Whether the app is currently switching years. */
  isSwitching = $state(false);

  /** Whether the context is loading. */
  isLoading = $state(false);

  /** Whether initial context has been loaded. */
  isInitialized = $state(false);

  // ── UI selections (persisted in localStorage) ───────────────────

  private _selectedCycleId = $state<string>(
    localStorage.getItem("orbis_academic_cycle_id") ?? "",
  );
  private _selectedClassId = $state<string>(
    localStorage.getItem("orbis_academic_class_id") ?? "",
  );
  private _selectedSubjectId = $state<string>(
    localStorage.getItem("orbis_academic_subject_id") ?? "",
  );

  // ── Getters/setters for UI selections ───────────────────────────

  get selectedCycleId(): string {
    return this._selectedCycleId;
  }
  set selectedCycleId(val: string) {
    this._selectedCycleId = val;
    localStorage.setItem("orbis_academic_cycle_id", val);
  }

  get selectedClassId(): string {
    return this._selectedClassId;
  }
  set selectedClassId(val: string) {
    this._selectedClassId = val;
    localStorage.setItem("orbis_academic_class_id", val);
  }

  get selectedSubjectId(): string {
    return this._selectedSubjectId;
  }
  set selectedSubjectId(val: string) {
    this._selectedSubjectId = val;
    localStorage.setItem("orbis_academic_subject_id", val);
  }

  // ── Derived ─────────────────────────────────────────────────────

  /** Whether the current year is archived (read-only mode). */
  isReadOnly = $derived(this._isReadOnly || this.currentYear?.status === "archived");

  /** Whether any years exist (for first-launch experience). */
  hasYears = $derived(this.years.length > 0);

  /** Whether enrollment is open in the active cycle. */
  canEnroll = $derived(
    (this.activeCycle?.is_enrollment_open ?? false) && !this.isReadOnly,
  );

  /** Whether grade entry is open in the active cycle. */
  canEditGrades = $derived(
    (this.activeCycle?.is_grade_entry_open ?? false) && !this.isReadOnly,
  );

  /** Whether the active cycle is locked. */
  isLocked = $derived(this.activeCycle?.is_locked ?? false);

  /** Current lifecycle phase of the active cycle. */
  cyclePhase = $derived<CyclePhase>(this.activeCycle?.phase ?? "active");

  /** Display label for the current year. */
  yearLabel = $derived(this.currentYear?.name ?? "");

  /** Display label (Arabic) for the current year. */
  yearLabelAr = $derived(
    this.currentYear?.name_ar ?? this.currentYear?.name ?? "",
  );

  /** Status badge text. */
  statusBadge = $derived<"active" | "archived" | "promoting">(this.yearStatus);

  /**
   * The cycle ID that all modules should use.
   * If user manually selected a cycle, use that; otherwise use the auto-resolved one.
   */
  effectiveCycleId = $derived(
    this._selectedCycleId || this.activeCycle?.id || "",
  );

  // ── Actions ─────────────────────────────────────────────────────

  /**
   * Resolve the full academic context from the backend.
   * Should be called once on app init (after login/org is known).
   */
  async resolveContext(orgId: string) {
    this.isLoading = true;
    try {
      const ctx: AcademicContextResponse =
        await commands.academicContext.getContext(orgId);

      // Apply backend-resolved state
      this.activeCycle = ctx.active_cycle;
      this.allCycles = ctx.all_cycles;
      this.yearStatus = ctx.year_status;
      this._isReadOnly = ctx.is_read_only;

      // Sync year info
      if (ctx.year_id) {
        this.currentYear = await commands.academicYears.getCurrent();
      }

      // If no manual selection and we have an auto-resolved cycle, use it
      if (!this._selectedCycleId && ctx.active_cycle) {
        // Don't persist — this is automatic
        this._selectedCycleId = ctx.active_cycle.id;
      }

      this.isInitialized = true;
    } catch (e: unknown) {
      console.warn("Failed to resolve academic context:", extractErrorMessage(e));
    } finally {
      this.isLoading = false;
    }
  }

  /** Load the current year from the backend. Fire-and-forget safe. */
  async loadCurrentYear() {
    try {
      this.currentYear = await commands.academicYears.getCurrent();
    } catch (e: unknown) {
      console.warn("Failed to load current year:", extractErrorMessage(e));
    }
  }

  /** Load all years for an organization. */
  async loadYears(orgId: string) {
    try {
      this.years = await commands.academicYears.list(orgId);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  /** Switch to a different academic year. */
  async switchYear(yearId: string) {
    if (this.isSwitching) return;
    this.isSwitching = true;
    try {
      await commands.academicYears.switch(yearId);
      await this.loadCurrentYear();
      toast.success(`Switched to ${this.yearLabel}`);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      this.isSwitching = false;
    }
  }

  /** Reset all UI selections. */
  reset() {
    this.selectedCycleId = "";
    this.selectedClassId = "";
    this.selectedSubjectId = "";
  }
}

export const academicContext = new AcademicContextStore();

/**
 * @deprecated Use `academicContext` instead.
 * Backward-compatible alias for `academicContext`.
 */
export const academicYearStore = academicContext;
