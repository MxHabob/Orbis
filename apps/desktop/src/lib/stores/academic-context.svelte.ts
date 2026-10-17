/**
 * Academic Context Store — centralized academic state for the entire app.
 *
 * Unified academic year + period model adhering to Orbis Target Architecture.
 */
import type {
  academicCalendarDto,
  AcademicPeriodDto,
  AcademicContextResponse,
} from "@orbis/sdk";
import { api } from "$lib/api/client";
import { extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";

class AcademicContextStore {
  currentYear = $state<academicCalendarDto | null>(null);
  allYears = $state<academicCalendarDto[]>([]);
  periods = $state<AcademicPeriodDto[]>([]);
  yearStatus = $state<string>("active");
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

  get isReadOnly(): boolean {
    return this._isReadOnly || this.currentYear?.status === "archived";
  }
  get hasYears(): boolean {
    return this.allYears.length > 0;
  }
  get yearLabel(): string {
    return this.currentYear?.name ?? "";
  }
  get yearLabelAr(): string {
    return this.currentYear?.name_ar ?? this.currentYear?.name ?? "";
  }
  get effectivePeriodId(): string {
    return this._selectedPeriodId || this.activePeriod?.id || "";
  }

  /** First active period or first period in list */
  get activePeriod(): AcademicPeriodDto | null {
    return (
      this.periods.find((p) => p.status === "active") ?? this.periods[0] ?? null
    );
  }

  /** All periods for current academic context */
  get allPeriods(): AcademicPeriodDto[] {
    return this.periods;
  }

  async resolveContext(orgId: string) {
    this.orgId = orgId;
    this.isLoading = true;
    try {
      const ctx: AcademicContextResponse = await api.getAcademicContext();
      this.currentYear = ctx.current_year ?? null;
      this.allYears = ctx.all_years ?? [];
      this.periods = ctx.periods ?? [];
      this.yearStatus = ctx.current_year?.status ?? "active";
      this._isReadOnly = ctx.current_year?.status === "archived";

      // If current_year wasn't returned as active, check all_years
      if (!this.currentYear && this.allYears.length > 0) {
        this.currentYear =
          this.allYears.find((y) => y.status === "active" || y.is_active) ??
          this.allYears[0] ??
          null;
      }

      // If all_years was empty, fallback to listacademicCalendars
      if (this.allYears.length === 0) {
        try {
          const years = await api.listacademicCalendars();
          this.allYears = years;
          this.currentYear =
            years.find((y) => y.status === "active" || y.is_active) ??
            years[0] ??
            null;
        } catch {
          // ignore fallback error
        }
      }

      // If periods was empty, fallback to listAcademicPeriods
      if (this.periods.length === 0) {
        try {
          this.periods = await api.listAcademicPeriods();
        } catch {
          // ignore fallback error
        }
      }

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

  async loadCurrentYear() {
    try {
      const ctx = await api.getAcademicContext();
      this.currentYear = ctx.current_year ?? null;
    } catch (e: unknown) {
      console.warn("Failed to load current year:", extractErrorMessage(e));
    }
  }

  async loadYears() {
    try {
      this.allYears = await api.listacademicCalendars();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async switchYear(yearId: string) {
    if (this.isSwitching) return;
    this.isSwitching = true;
    try {
      const year = await api.getacademicCalendar(yearId);
      this.currentYear = year;
      const periods = await api.listAcademicPeriods();
      this.periods = periods.filter((p) => p.year_id === yearId);
      if (this.activePeriod) {
        this.selectedPeriodId = this.activePeriod.id;
      }
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
