// ── UI Store ─────────────────────────────────────────────────────────

import { page } from "$app/state";

export interface UiPreferenceScope {
  userId: string;
  organizationId: string;
}

export interface UiPreferenceValues {
  "exams.selectedPeriodId": string;
  "exams.sessionType": "final" | "midterm" | "quiz" | "practical";
  "exams.ai.preferredPeriod": "both" | "morning" | "afternoon";
  "exams.ai.maxExamsPerDay": 1 | 2;
  "exams.ai.defaultDurationMinutes": number;
  "exams.ai.separateDivisions": boolean;
  "exams.ai.separateGenders": boolean;
  "exams.ai.balanceHalls": boolean;
  "students.studyMode":
    | "regular"
    | "part_time"
    | "distance_learning"
    | "homeschooled"
    | "auditor"
    | "visiting"
    | "exchange"
    | "special_needs";
  "students.classId": string;
  "employees.employeeType": "teacher" | "admin" | "staff" | "other";
  "employees.employmentType":
    | "full_time"
    | "part_time"
    | "contract"
    | "hourly"
    | "volunteer";
  "employees.workShift": "morning" | "evening" | "flexible";
  "users.personaType": "employee" | "student" | "guardian" | "custom";
  "schedule.timeSlotStart": string;
  "schedule.timeSlotEnd": string;
}

export interface TimeSlotRangePreference {
  startTime: string;
  endTime: string;
}

type UiPreferenceKey = keyof UiPreferenceValues;

const DEFAULT_TIME_SLOT_RANGE: TimeSlotRangePreference = {
  startTime: "08:00",
  endTime: "08:45",
};

function getPreferenceStorageKey(
  scope: UiPreferenceScope,
  key: UiPreferenceKey,
): string {
  return `orbis:ui-preferences:v1:${encodeURIComponent(scope.userId)}:${encodeURIComponent(scope.organizationId)}:${key}`;
}

function isTimeOfDay(value: unknown): value is string {
  return typeof value === "string" && /^([01]\d|2[0-3]):[0-5]\d$/.test(value);
}

function getMinutes(value: string): number {
  const [hours, minutes] = value.split(":").map(Number);
  return hours * 60 + minutes;
}

class UIStore {
  private aiProviderModels = $state<
    Record<string, { provider: string; model: string }>
  >({});

  globalLoading = $state(false);
  _showAskOrbis = $state(false);
  showUpgradeDialog = $state(false);
  focusMode = $state(localStorage.getItem("orbis_focus_mode") === "true");
  showSearch = $state(false);
  showSystemClock = $state(
    localStorage.getItem("orbis_clock_open") !== "false",
  );

  get showAskOrbis() {
    try {
      return this._showAskOrbis || !!page.url.searchParams.get("ask_orbis");
    } catch {
      return this._showAskOrbis;
    }
  }

  setGlobalLoading(loading: boolean) {
    this.globalLoading = loading;
  }

  toggleAskOrbis() {
    this._showAskOrbis = !this.showAskOrbis;
  }

  setAskOrbis(show: boolean) {
    this._showAskOrbis = show;
  }

  toggleFocusMode() {
    this.focusMode = !this.focusMode;
    localStorage.setItem("orbis_focus_mode", String(this.focusMode));
  }

  toggleUpgradeDialog() {
    this.showUpgradeDialog = !this.showUpgradeDialog;
  }

  setUpgradeDialog(show: boolean) {
    this.showUpgradeDialog = show;
  }

  showActivationDialog = $state(false);

  setActivationDialog(show: boolean) {
    this.showActivationDialog = show;
  }

  toggleSearch() {
    this.showSearch = !this.showSearch;
  }

  setSearch(show: boolean) {
    this.showSearch = show;
  }

  setSystemClock(show: boolean) {
    this.showSystemClock = show;
    localStorage.setItem("orbis_clock_open", String(show));
  }

  getPreference<Key extends UiPreferenceKey>(
    scope: UiPreferenceScope,
    key: Key,
    fallback: UiPreferenceValues[Key],
    isValid: (value: unknown) => value is UiPreferenceValues[Key],
  ): UiPreferenceValues[Key] {
    if (
      typeof localStorage === "undefined" ||
      !scope.userId ||
      !scope.organizationId
    ) {
      return fallback;
    }

    try {
      const stored = localStorage.getItem(getPreferenceStorageKey(scope, key));
      if (stored === null) return fallback;

      const value: unknown = JSON.parse(stored);
      return isValid(value) ? value : fallback;
    } catch (error) {
      console.error(`Failed to restore UI preference "${key}":`, error);
      return fallback;
    }
  }

  setPreference<Key extends UiPreferenceKey>(
    scope: UiPreferenceScope,
    key: Key,
    value: UiPreferenceValues[Key],
  ): void {
    if (
      typeof localStorage === "undefined" ||
      !scope.userId ||
      !scope.organizationId
    ) {
      return;
    }

    try {
      localStorage.setItem(
        getPreferenceStorageKey(scope, key),
        JSON.stringify(value),
      );
    } catch (error) {
      console.error(`Failed to save UI preference "${key}":`, error);
    }
  }

  getTimeSlotRangePreference(
    scope: UiPreferenceScope,
  ): TimeSlotRangePreference {
    const startTime = this.getPreference(
      scope,
      "schedule.timeSlotStart",
      DEFAULT_TIME_SLOT_RANGE.startTime,
      isTimeOfDay,
    );
    const endTime = this.getPreference(
      scope,
      "schedule.timeSlotEnd",
      DEFAULT_TIME_SLOT_RANGE.endTime,
      isTimeOfDay,
    );

    return getMinutes(endTime) > getMinutes(startTime)
      ? { startTime, endTime }
      : DEFAULT_TIME_SLOT_RANGE;
  }

  setTimeSlotRangePreference(
    scope: UiPreferenceScope,
    range: TimeSlotRangePreference,
  ): void {
    if (
      !isTimeOfDay(range.startTime) ||
      !isTimeOfDay(range.endTime) ||
      getMinutes(range.endTime) <= getMinutes(range.startTime)
    ) {
      return;
    }

    this.setPreference(scope, "schedule.timeSlotStart", range.startTime);
    this.setPreference(scope, "schedule.timeSlotEnd", range.endTime);
  }

  getAiProviderModel(orgId: string): { provider: string; model: string } {
    const key = encodeURIComponent(orgId);
    const preferences = this.aiProviderModels[key];
    if (preferences) return preferences;

    return {
      provider: localStorage.getItem(`orbis_ai_provider_${key}`) ?? "",
      model: localStorage.getItem(`orbis_ai_model_${key}`) ?? "",
    };
  }

  setAiProviderModel(orgId: string, provider: string, model: string) {
    if (!orgId) return;
    const key = encodeURIComponent(orgId);
    this.aiProviderModels[key] = { provider, model };
    localStorage.setItem(`orbis_ai_provider_${key}`, provider);
    localStorage.setItem(`orbis_ai_model_${key}`, model);
  }
}

export const uiStore = new UIStore();
