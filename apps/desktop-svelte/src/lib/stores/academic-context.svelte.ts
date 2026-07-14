import * as m from "$lib/paraglide/messages";
/**
 * Academic Context Store — centralized academic state for the entire app.
 *
 * Migrated from period-based to unified year+period model.
 */
import type {
  AcademicYear,
  AcademicYearInfo,
  AcademicContextResponse,
  AcademicPeriod,
} from "@orbis/sdk-core";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

class AcademicContextStore {
  currentYear = $state<AcademicYear | null>(null);
  allYears = $state<AcademicYearInfo[]>([]);
  periods = $state<AcademicPeriod[]>([]);
  yearStatus = $state<"planning" | "active" | "closing" | "archived">("active");
  private _isReadOnly = $state(false);
  isSwitching = $state(false);
  isLoading = $state(false);
  isInitialized = $state(false);
  orgId = $state<string>("");

  private _selectedPeriodId = $state<string>(
    localStorage.getItem("orbis_academic_period_id") ?? "",
  );
  private _selectedClassId = $state<string>(
    localStorage.getItem("orbis_academic_class_id") ?? "",
  );
  private _selectedSubjectId = $state<string>(
    localStorage.getItem("orbis_academic_subject_id") ?? "",
  );

  get selectedPeriodId(): string {
    return this._selectedPeriodId;
  }
  set selectedPeriodId(val: string) {
    this._selectedPeriodId = val;
    localStorage.setItem("orbis_academic_period_id", val);
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

  isReadOnly = $derived(
    this._isReadOnly || this.currentYear?.status === "archived",
  );
  hasYears = $derived(this.allYears.length > 0);
  canEnroll = $derived(
    this.activePeriod?.is_enrollment_open === 1 && !this.isReadOnly,
  );
  canEditGrades = $derived(
    this.activePeriod?.is_grade_entry_open === 1 && !this.isReadOnly,
  );
  isLocked = $derived(this.currentYear?.is_locked === 1);
  yearLabel = $derived(this.currentYear?.name ?? "");
  yearLabelAr = $derived(
    this.currentYear?.name_ar ?? this.currentYear?.name ?? "",
  );
  statusBadge = $derived<"planning" | "active" | "closing" | "archived">(
    this.yearStatus,
  );
  effectivePeriodId = $derived(
    this._selectedPeriodId || this.activePeriod?.id || "",
  );

  /** First active period or first period in list */
  get activePeriod(): AcademicPeriod | null {
    return (
      this.periods.find((p) => p.status === "active") ?? this.periods[0] ?? null
    );
  }

  /** Alias for periods — used by TitleBar and other shell components. */
  get allPeriods(): AcademicPeriod[] {
    return this.periods;
  }

  /**
   * Derived phase of the active period.
   * Computed from period status, enrollment, and grade-entry flags.
   */
  get periodPhase(): "pre_start" | "active" | "exams" | "grade_entry" | "closed" | null {
    const period = this.activePeriod;
    if (!period) return null;
    if (period.status === "closed" || period.status === "archived") return "closed";
    if (period.is_grade_entry_open === 1) return "grade_entry";
    if (period.exam_start && period.exam_end) {
      const now = new Date().toISOString().slice(0, 10);
      if (now >= period.exam_start && now <= period.exam_end) return "exams";
    }
    if (period.status === "active") return "active";
    return "pre_start";
  }

  async resolveContext(orgId: string) {
    this.orgId = orgId;
    this.isLoading = true;
    try {
      const current_year = await commands.platform.getCurrentYear();
      const all_years = await commands.platform.listYears(orgId);
      const ctx: AcademicContextResponse = {
        current_year,
        all_years: all_years || [],
        periods: all_years || [], // Assuming periods and years are the same in autopilot
        year_status: current_year?.status === "archived" ? "archived" : "active",
        is_read_only: current_year?.status === "archived",
      };
      this.currentYear = ctx.current_year ?? null;
      this.allYears = ctx.all_years ?? [];
      this.periods = ctx.periods ?? [];
      this.yearStatus = ctx.year_status ?? "active";
      this._isReadOnly = ctx.is_read_only ?? false;
      if (!this._selectedPeriodId && this.activePeriod) {
        this._selectedPeriodId = this.activePeriod.id;
      }
      this.isInitialized = true;
    } catch (e: unknown) {
      console.warn(
        "Failed to resolve academic context:",
        extractErrorMessage(e),
      );
    } finally {
      this.isLoading = false;
    }
  }

  async loadCurrentYear(orgId?: string) {
    try {
      const id = orgId || this.orgId;
      if (!id) return;
      this.currentYear = await commands.platform.getCurrentYear();
    } catch (e: unknown) {
      console.warn("Failed to load current year:", extractErrorMessage(e));
    }
  }

  async loadYears(orgId: string) {
    try {
      this.allYears = await commands.platform.listYears();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async switchYear(periodId: string) {
    if (this.isSwitching) return;
    this.isSwitching = true;
    try {
      await commands.platform.switchYear(periodId);
      await this.loadCurrentYear();
      toast.success(`Switched to ${this.yearLabel}`);
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      this.isSwitching = false;
    }
  }

  reset() {
    this.selectedPeriodId = "";
    this.selectedClassId = "";
    this.selectedSubjectId = "";
  }
}

export const academicContext = new AcademicContextStore();
