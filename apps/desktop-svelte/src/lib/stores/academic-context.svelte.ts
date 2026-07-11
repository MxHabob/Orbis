/**
 * Academic Context Store — centralized academic state for the entire app.
 */
import type {
  AcademicYear,
  AcademicYearInfo,
  AcademicContextResponse,
  AcademicCycleContext,
  AcademicCycleSummary,
  CyclePhase,
} from "@orbis/sdk-core";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

class AcademicContextStore {
  currentYear = $state<AcademicYear | null>(null);
  activeCycle = $state<AcademicCycleContext | null>(null);
  allCycles = $state<AcademicCycleSummary[]>([]);
  years = $state<AcademicYearInfo[]>([]);
  yearStatus = $state<"active" | "archived" | "promoting">("active");
  private _isReadOnly = $state(false);
  isSwitching = $state(false);
  isLoading = $state(false);
  isInitialized = $state(false);

  private _selectedCycleId = $state<string>(localStorage.getItem("orbis_academic_cycle_id") ?? "");
  private _selectedClassId = $state<string>(localStorage.getItem("orbis_academic_class_id") ?? "");
  private _selectedSubjectId = $state<string>(localStorage.getItem("orbis_academic_subject_id") ?? "");

  get selectedCycleId(): string { return this._selectedCycleId; }
  set selectedCycleId(val: string) { this._selectedCycleId = val; localStorage.setItem("orbis_academic_cycle_id", val); }
  get selectedClassId(): string { return this._selectedClassId; }
  set selectedClassId(val: string) { this._selectedClassId = val; localStorage.setItem("orbis_academic_class_id", val); }
  get selectedSubjectId(): string { return this._selectedSubjectId; }
  set selectedSubjectId(val: string) { this._selectedSubjectId = val; localStorage.setItem("orbis_academic_subject_id", val); }

  isReadOnly = $derived(this._isReadOnly || this.currentYear?.status === "archived");
  hasYears = $derived(this.years.length > 0);
  canEnroll = $derived((this.activeCycle?.is_enrollment_open ?? false) && !this.isReadOnly);
  canEditGrades = $derived((this.activeCycle?.is_grade_entry_open ?? false) && !this.isReadOnly);
  isLocked = $derived(this.activeCycle?.is_locked ?? false);
  cyclePhase = $derived<CyclePhase>(this.activeCycle?.phase ?? "active");
  yearLabel = $derived(this.currentYear?.name ?? "");
  yearLabelAr = $derived(this.currentYear?.name_ar ?? this.currentYear?.name ?? "");
  statusBadge = $derived<"active" | "archived" | "promoting">(this.yearStatus);
  effectiveCycleId = $derived(this._selectedCycleId || this.activeCycle?.id || "");

  async resolveContext(orgId: string) {
    this.isLoading = true;
    try {
      const ctx: AcademicContextResponse = await commands.platform.getAcademicContext(orgId);
      this.activeCycle = ctx.active_cycle;
      this.allCycles = ctx.all_cycles;
      this.yearStatus = ctx.year_status;
      this._isReadOnly = ctx.is_read_only;
      if (ctx.year_id) {
        this.currentYear = await commands.platform.getCurrentYear();
      }
      if (!this._selectedCycleId && ctx.active_cycle) {
        this._selectedCycleId = ctx.active_cycle.id;
      }
      this.isInitialized = true;
    } catch (e: unknown) {
      console.warn("Failed to resolve academic context:", extractErrorMessage(e));
    } finally {
      this.isLoading = false;
    }
  }

  async loadCurrentYear() {
    try {
      this.currentYear = await commands.platform.getCurrentYear();
    } catch (e: unknown) {
      console.warn("Failed to load current year:", extractErrorMessage(e));
    }
  }

  async loadYears(orgId: string) {
    try {
      this.years = await commands.platform.listYears(orgId);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async switchYear(yearId: string) {
    if (this.isSwitching) return;
    this.isSwitching = true;
    try {
      await commands.platform.switchYear(yearId);
      await this.loadCurrentYear();
      toast.success(`Switched to ${this.yearLabel}`);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      this.isSwitching = false;
    }
  }

  reset() {
    this.selectedCycleId = "";
    this.selectedClassId = "";
    this.selectedSubjectId = "";
  }
}

export const academicContext = new AcademicContextStore();
