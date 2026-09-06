import { toast } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { api } from "$lib/api";
import { INSTITUTION_PROFILES, DEFAULT_PERIOD_TEMPLATES } from "./templates";

import type {
  OnboardingPayload,
  PeriodTemplateDef,
  TemplateNode,
  PeriodEntry,
  ClassEntry,
  SubjectEntry,
  Room,
} from "./types";

import { ProfileInfoState } from "./state/profile-info.svelte";
import { CalendarInfoState } from "./state/calendar-info.svelte";
import { FacilitiesInfoState } from "./state/facilities-info.svelte";
import { AcademicsInfoState } from "./state/academics-info.svelte";
import { FinanceInfoState } from "./state/finance-info.svelte";

function formatTime(totalMinutes: number): string {
  const hours = Math.floor(totalMinutes / 60).toString().padStart(2, "0");
  const minutes = (totalMinutes % 60).toString().padStart(2, "0");
  return `${hours}:${minutes}`;
}

class OnboardingState {
  // Wizard navigation
  activeStep = $state(0);
  isLoading = $state(false);
  errorMessage = $state<string | null>(null);
  totalSteps = 9;
  completedSteps = $state([
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);

  // Sub-states
  profile = new ProfileInfoState();
  calendar = new CalendarInfoState();
  facilities = new FacilitiesInfoState();
  academics = new AcademicsInfoState();
  finance = new FinanceInfoState();

  // ظ¤?ظ¤? Centralized Terminology Mappings ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  terminology = $derived(
    INSTITUTION_PROFILES[this.profile.institutionType]?.terminology ||
      INSTITUTION_PROFILES.school.terminology,
  );

  stageLabel = $derived(
    getLocale() === "ar" ? this.terminology.stage_ar : this.terminology.stage,
  );
  classroomLabel = $derived(
    getLocale() === "ar"
      ? this.terminology.classroom_ar
      : this.terminology.classroom,
  );
  divisionLabel = $derived(
    getLocale() === "ar"
      ? this.terminology.division_ar
      : this.terminology.division,
  );
  subjectLabel = $derived(
    getLocale() === "ar"
      ? this.terminology.subject_ar
      : this.terminology.subject,
  );
  calendarLabel = $derived(
    getLocale() === "ar"
      ? this.terminology.calendar_ar
      : this.terminology.calendar,
  );
  localizedLanguageName = $derived(
    this.profile.locale.startsWith("ar")
      ? m.onboarding_lang_ar()
      : m.onboarding_lang_en(),
  );
  classroomsLabelPlural = $derived(
    getLocale() === "ar"
      ? this.terminology.classroom_ar
      : this.terminology.classroom + "s",
  );
  stagesLabelPlural = $derived(
    getLocale() === "ar"
      ? this.terminology.stage_ar
      : this.terminology.stage + "s",
  );

  selectInstitutionType(type: string) {
    this.profile.institutionType = type;
    const profile = INSTITUTION_PROFILES[type];
    if (profile) {
      this.profile.studentNumberPattern = profile.numberingPattern;
      this.profile.activeFields = { ...profile.activeFields };
      this.profile.requiredFields = { ...profile.requiredFields };
      this.profile.preventDuplicates = { ...profile.preventDuplicates };
      this.profile.requiredDocumentsList = [...profile.requiredDocuments];

      // Default academic stages
      this.academics.gradeLevels = profile.stages.map((s) => ({
        level: s.level,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        enabled: true,
        divisions: [...s.divisions],
      }));

      // Default time slots
      this.academics.workingDays = [1, 2, 3, 4, 5];
      const defaultLessonNames = [
        ["1st Period", "الحصة الأولى"],
        ["2nd Period", "الحصة الثانية"],
        ["3rd Period", "الحصة الثالثة"],
        ["4th Period", "الحصة الرابعة"],
        ["5th Period", "الحصة الخامسة"],
        ["6th Period", "الحصة السادسة"],
        ["7th Period", "الحصة السابعة"],
      ];
      const defaultSlots = [];
      let currentMinutes = 8 * 60;
      for (let index = 0; index < 7; index++) {
        if (index === 3) {
          defaultSlots.push({
            name: "Break",
            name_ar: "استراحة",
            start_time: formatTime(currentMinutes),
            end_time: formatTime(currentMinutes + 20),
            is_break: true,
            break_type: "recess" as const,
            applicable_days: [...this.academics.workingDays],
          });
          currentMinutes += 20;
        }
        defaultSlots.push({
          name: defaultLessonNames[index][0],
          name_ar: defaultLessonNames[index][1],
          start_time: formatTime(currentMinutes),
          end_time: formatTime(currentMinutes + 40),
          is_break: false,
          applicable_days: [...this.academics.workingDays],
        });
        currentMinutes += 40;
      }
      this.academics.timeSlots = defaultSlots;

      // Default subjects
      this.academics.subjects = profile.subjects.map((s) => ({
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        icon: s.icon,
        sort_order: s.sort_order,
        enabled: true,
      }));

      // Prepopulate default fees to 0
      const fees: Record<string, number> = {};
      profile.stages.forEach((s) => {
        fees[s.level] = 0;
      });
      this.finance.registrationFees = fees;

      this.save();
    }
  }

  // ظ¤?ظ¤? Derived Values ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  progressValue = $derived(
    this.activeStep === 0 ? 0 : (this.activeStep / this.totalSteps) * 100,
  );
  tasksCompletedCount = $derived(this.completedSteps.filter(Boolean).length);

  // ظ¤?ظ¤? Persistence ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  save() {
    try {
      const data = {
        activeStep: this.activeStep,
        completedSteps: this.completedSteps,
        profile: this.profile.serialize(),
        calendar: this.calendar.serialize(),
        facilities: this.facilities.serialize(),
        academics: this.academics.serialize(),
        finance: this.finance.serialize(),
      };
      localStorage.setItem("orbis_onboarding_state", JSON.stringify(data));
    } catch (e) {
      console.error("Failed to save onboarding state:", e);
    }
  }

  load() {
    try {
      const raw = localStorage.getItem("orbis_onboarding_state");
      if (raw) {
        const data = JSON.parse(raw);
        if (data && typeof data === "object") {
          this.activeStep = data.activeStep ?? 0;
          this.completedSteps = data.completedSteps ?? [
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
          ];
          this.profile.deserialize(data);
          this.calendar.deserialize(data);
          this.facilities.deserialize(data);
          this.academics.deserialize(data);
          this.finance.deserialize(data);
        }
      }
    } catch (e) {
      console.error("Failed to load onboarding state:", e);
    }
  }

  async loadExistingOrg(orgId: string) {
    if (!orgId) return;
    try {
      const org = await api.getCurrentOrg().catch(() => null);
      if (org) {
        // Sync org profile from CLI-created data
        if (org.name) {
          this.profile.orgName = org.name;
          this.profile.rootUnitName = org.name;
        }
        if (org.timezone) this.profile.timezone = org.timezone;
        if (org.locale) this.profile.locale = org.locale;

        // Fallback: check period templates if not found in settings
        await this.loadTemplates(orgId).catch(() => {});
        this.selectInstitutionType("school");

        const units = await api.listOrgUnits().catch(() => []);
        const rootUnit = (units || []).find(
          (unit: any) =>
            !unit.parent_id &&
            ["root", "organization"].includes(unit.unit_type || unit.unitType),
        );
        const buildingUnits = (units || []).filter((unit: any) => {
          const unitType = unit.unit_type || unit.unitType;
          const isRootChild = rootUnit?.id && unit.parent_id === rootUnit.id;
          return unitType === "building" || unitType === "branch" || unitType === "campus" || isRootChild;
        });
        if (buildingUnits.length > 0) {
          this.facilities.buildings = buildingUnits
            .map((unit: any) => String(unit.name || unit.name_ar || "").trim())
            .filter(Boolean);
          this.facilities.activeBuildingTab = this.facilities.buildings[0] || "";
        }

        const rooms = await api.listRooms().catch(() => []);
        if (rooms.length > 0) {
          this.facilities.rooms = (rooms || []).map((room: any): Room => ({
            id: String(room.id),
            name: String(room.name || room.name_ar || ""),
            name_ar: String(room.name_ar || room.name || ""),
            capacity: Number(room.capacity) || 0,
            room_type: String(room.room_type || "classroom"),
            building: String(room.building || ""),
            floor: Number(room.floor) || 0,
          }));
          for (const room of this.facilities.rooms) {
            if (room.building && !this.facilities.buildings.includes(room.building)) {
              this.facilities.buildings.push(room.building);
            }
          }
          if (!this.facilities.activeBuildingTab) {
            this.facilities.activeBuildingTab = this.facilities.buildings[0] || "";
          }
        }

        this.save();
      }

      // Sync currency from finance module (CLI seeds currencies)
      const currencies = await api.listCurrencies().catch(() => []);
      if (currencies && currencies.length > 0) {
        const mainCurr = currencies[0];
        if (mainCurr && mainCurr.id) {
          this.profile.currency = mainCurr.id;
        }
      }
    } catch (e) {
      console.warn("Failed to load existing organization:", e);
    }
  }

  // ظ¤?ظ¤? Navigation ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  goNext() {
    if (this.activeStep < this.totalSteps) {
      if (this.activeStep > 0) this.completedSteps[this.activeStep - 1] = true;
      this.activeStep += 1;
      this.save();
    }
  }

  goPrev() {
    if (this.activeStep > 0) {
      this.activeStep -= 1;
      this.save();
    }
  }

  goToStep(step: number) {
    if (!this.isLoading && step >= 0 && step <= this.totalSteps) {
      this.activeStep = step;
      this.save();
    }
  }

  // ظ¤?ظ¤? Step 3: Room & Building Management ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  addBuilding(name: string): boolean {
    const trimmed = name.trim();
    if (!trimmed || this.facilities.buildings.includes(trimmed)) return false;
    this.facilities.buildings.push(trimmed);
    this.facilities.activeBuildingTab = trimmed;
    this.save();
    return true;
  }

  removeBuilding(name: string): boolean {
    const idx = this.facilities.buildings.indexOf(name);
    if (idx === -1) return false;

    this.facilities.buildings.splice(idx, 1);
    this.facilities.rooms = this.facilities.rooms.filter(
      (r) => r.building !== name,
    );
    if (this.facilities.activeBuildingTab === name) {
      this.facilities.activeBuildingTab = this.facilities.buildings[0] || "";
    }
    this.save();
    return true;
  }

  addDefaultRoomsToBuilding(bldName: string) {
    const otherRooms = this.facilities.rooms.filter(
      (r) => r.building !== bldName,
    );
    const terminology =
      INSTITUTION_PROFILES[this.profile.institutionType]?.terminology ||
      INSTITUTION_PROFILES.school.terminology;
    const defaultList = [];
    const tsSuffix = Date.now();

    for (let i = 1; i <= 10; i++) {
      defaultList.push({
        id: `room-${bldName}-${i}-${tsSuffix}`,
        name: `${terminology.classroom} 1-0${i}`,
        name_ar: `${terminology.classroom_ar} 1-0${i}`,
        capacity: 30,
        room_type: "classroom",
        building: bldName,
        floor: 1,
      });
    }
    for (let i = 1; i <= 1; i++) {
      defaultList.push({
        id: `room-${bldName}-lab-comp-${i}-${tsSuffix}`,
        name: `Computer Lab 1-${i}`,
        name_ar: `مختبر حاسوب 1-${i}`,
        capacity: 25,
        room_type: "computer_lab",
        building: bldName,
        floor: 2,
      });
    }
    const additionalRooms = [
      ["lecture_hall", "Lecture Hall", "قاعة محاضرات", 60],
      ["science_lab", "Science Lab", "مختبر علمي", 25],
      ["workshop", "Workshop", "ورشة عمل", 25],
      ["library", "Library", "مكتبة", 40],
      ["studio", "Studio", "استوديو", 25],
      ["activity_hall", "Activity Hall", "صالة أنشطة", 60],
    ] as const;
    for (const [roomType, name, nameAr, capacity] of additionalRooms) {
      defaultList.push({
        id: `room-${bldName}-${roomType}-${tsSuffix}`,
        name,
        name_ar: nameAr,
        capacity,
        room_type: roomType,
        building: bldName,
        floor: 1,
      });
    }
    this.facilities.rooms = [...otherRooms, ...defaultList];
    this.facilities.activeBuildingTab = bldName;
    this.save();
  }

  addRoom(
    name: string,
    nameAr: string,
    capacity: number,
    roomType: string,
    floor: number,
  ) {
    const finalNameAr = (nameAr || name || "").trim();
    const finalNameEn = (name || nameAr || "").trim();
    if (!finalNameAr && !finalNameEn) {
      toast.error("يرجى إدخال اسم القاعة أو المرفق");
      return false;
    }
    const id = `room-${Date.now()}`;
    this.facilities.rooms.push({
      id,
      name: finalNameEn,
      name_ar: finalNameAr,
      capacity: Number(capacity) || 30,
      room_type: roomType || "classroom",
      building: this.facilities.activeBuildingTab,
      floor: Number(floor) || 0,
    });
    this.facilities.rooms = [...this.facilities.rooms];
    toast.success("تمت إضافة القاعة بنجاح");
    this.save();
    return true;
  }

  removeRoom(id: string) {
    this.facilities.rooms = this.facilities.rooms.filter((r) => r.id !== id);
    toast.success("تم حذف القاعة");
    this.save();
  }

  // ظ¤?ظ¤? Step 4: Grade/Division Management ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  addDivision(level: string) {
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    if (!gl) return;

    const input = this.academics.divisionInput[level]?.trim();
    const nextLetter = String.fromCharCode(65 + gl.divisions.length);
    const text = input ? input.replace(/\s+/g, " ") : nextLetter;
    if (!/^[A-Za-z][A-Za-z0-9 -]*$/.test(text) || gl.divisions.includes(text)) {
      toast.error(m.onboarding_toast_invalid_division());
      return;
    }

    gl.divisions.push(text);
    {
      this.academics.gradeLevels = [...this.academics.gradeLevels];
      this.academics.divisionInput[level] = "";
      toast.success(m.onboarding_toast_division_added());
    }
  }

  renameDivision(level: string, index: number, value: string) {
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    const trimmed = value.trim();
    const normalized = /^[A-Za-z][A-Za-z0-9 -]*$/.test(trimmed)
      ? trimmed.replace(/\s+/g, " ")
      : "";
    if (!gl || !normalized) return;
    if (gl.divisions.some((division, divisionIndex) => divisionIndex !== index && division === normalized)) {
      toast.error(m.onboarding_toast_invalid_division());
      return;
    }
    gl.divisions[index] = normalized;
    this.academics.gradeLevels = [...this.academics.gradeLevels];
    this.save();
  }

  addNamedDivision(level: string, name: "Boys" | "Girls") {
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    if (!gl || gl.divisions.includes(name)) return;
    gl.divisions.push(name);
    this.academics.gradeLevels = [...this.academics.gradeLevels];
    this.save();
  }

  removeDivision(level: string, div: string) {
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    if (gl) {
      gl.divisions = gl.divisions.filter((d) => d !== div);
      this.academics.gradeLevels = [...this.academics.gradeLevels];
      toast.success(m.onboarding_toast_division_deleted());
    }
  }

  // ── Step 5: Time Slot Management ──────────────────────────────────────
  async loadTemplates(_orgId?: string) {
    try {
      this.calendar.periodTemplates = DEFAULT_PERIOD_TEMPLATES;
      if (
        this.calendar.periodTemplates.length > 0 &&
        (!this.calendar.selectedTemplateId ||
          !this.calendar.periodTemplates.some((t) => t.id === this.calendar.selectedTemplateId))
      ) {
        this.calendar.selectedTemplateId = this.calendar.periodTemplates[0].id;
      }
    } catch (e) {
      console.warn("Failed to load period templates:", e);
      this.calendar.periodTemplates = DEFAULT_PERIOD_TEMPLATES;
      if (!this.calendar.selectedTemplateId) {
        this.calendar.selectedTemplateId = DEFAULT_PERIOD_TEMPLATES[0].id;
      }
    }
  }

  generatePeriodsFromTemplate(
    templateNode: TemplateNode,
    parentId: string,
    orgId: string,
    cycleStartMs: number,
    totalCycleDays: number,
    parentStartDateMs: number,
    parentDurationDays: number,
    level: number,
    index: number,
  ): PeriodEntry[] {
    const startMs = parentStartDateMs;
    const durationDays = Math.round(
      (templateNode.weight / 100) * parentDurationDays,
    );
    const endMs = startMs + durationDays * 24 * 60 * 60 * 1000;

    const startDate = new Date(startMs).toISOString().split("T")[0];
    const endDate = new Date(endMs).toISOString().split("T")[0];
    const id = `${parentId}-${templateNode.type}-${index}`;

    const entry: PeriodEntry = {
      id,
      parent_id: parentId,
      name: templateNode.name,
      name_ar: templateNode.name_ar,
      start_date: startDate,
      end_date: endDate,
      period_type: templateNode.type,
      weight: templateNode.weight,
    };

    let allEntries = [entry];

    if (templateNode.children && templateNode.children.length > 0) {
      let currentChildStartMs = startMs;
      templateNode.children.forEach((child, childIdx) => {
        const childDurationDays = Math.round(
          (child.weight / 100) * durationDays,
        );
        const childEntries = this.generatePeriodsFromTemplate(
          child,
          id,
          orgId,
          cycleStartMs,
          totalCycleDays,
          currentChildStartMs,
          durationDays,
          level + 1,
          childIdx + 1,
        );
        allEntries = allEntries.concat(childEntries);
        currentChildStartMs += childDurationDays * 24 * 60 * 60 * 1000;
      });
    }
    return allEntries;
  }

  addTimeSlot(
    name: string,
    nameAr: string,
    startTime: string,
    endTime: string,
    options: {
      isBreak?: boolean;
      breakType?: "recess" | "lunch" | "prayer" | "custom";
      applicableDays?: number[];
    } = {},
  ) {
    if (!name || !nameAr) {
      toast.error(m.onboarding_toast_fill_period_name());
      return false;
    }
    this.academics.timeSlots.push({
      name,
      name_ar: nameAr,
      start_time: startTime,
      end_time: endTime,
      is_break: options.isBreak ?? false,
      break_type: options.breakType,
      applicable_days: [...(options.applicableDays ?? this.academics.workingDays)],
    });
    this.academics.timeSlots = [...this.academics.timeSlots];
    toast.success(m.onboarding_toast_period_added());
    return true;
  }

  removeTimeSlot(idx: number) {
    this.academics.timeSlots = this.academics.timeSlots.filter(
      (_, i) => i !== idx,
    );
    toast.success(m.onboarding_toast_period_deleted());
  }

  // ظ¤?ظ¤? Step 6: Subject Management ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  addSubject(code: string, name: string, nameAr: string, color: string) {
    if (!code || !name || !nameAr) {
      toast.error(m.onboarding_toast_fill_subject_fields());
      return false;
    }
    this.academics.subjects.push({
      code: code.toUpperCase(),
      name,
      name_ar: nameAr,
      color,
      icon: "book",
      sort_order: this.academics.subjects.length + 1,
      enabled: true,
    });
    this.academics.subjects = [...this.academics.subjects];
    toast.success(m.onboarding_toast_subject_added());
    return true;
  }

  // ظ¤?ظ¤? Build Final Payload ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  // ── Build Final Payload ────────────────────────────────────────────────
  buildPayload(orgId: string): OnboardingPayload {
    const yearStart = new Date(this.calendar.startDate).getFullYear();
    const periodPeriodId = `period-${yearStart}-${orgId}`;

    const rooms: Room[] = this.facilities.rooms.map((r, i) => ({
      id: r.id || `room-${i}-${orgId}`,
      name: r.name,
      name_ar: r.name_ar,
      capacity: r.capacity || 30,
      room_type: r.room_type || "classroom",
      building: r.building,
      floor: r.floor,
    }));

    const classes: ClassEntry[] = [];
    for (const gl of this.academics.gradeLevels) {
      if (!gl.enabled) continue;
      for (const div of gl.divisions) {
        classes.push({
          id: `class-${gl.level}-${div}-${orgId}`,
          name: `${gl.name} - ${div}`,
          name_ar: `${gl.name_ar} - الشعبة ${div}`,
          level: gl.level,
          division: div,
          color: gl.color,
        });
      }
    }

    const subjects: SubjectEntry[] = this.academics.subjects
      .filter((s) => s.name.trim())
      .map((s, idx) => ({
        id: `subject-${s.code.toLowerCase()}-${orgId}`,
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color || "#4F46E5",
        icon: s.icon || "Book",
        sort_order: idx + 1,
      }));

    return {
      org_name: this.profile.orgName,
      org_name_ar: this.profile.orgNameAr,
      currency: this.profile.currency,
      timezone: this.profile.timezone,
      locale: this.profile.locale,
      root_unit_id: `unit-${orgId}`,
      root_unit_name: this.profile.rootUnitName,
      root_unit_name_ar: this.profile.rootUnitNameAr,
      periods: [
        {
          id: periodPeriodId,
          parent_id: null,
          name: this.calendar.academicYearName,
          name_ar: this.calendar.academicYearNameAr,
          start_date: this.calendar.startDate,
          end_date: this.calendar.endDate,
          period_type: "year",
          weight: null,
        },
      ],
      rooms,
      classes,
      subjects,
      time_slots: this.academics.timeSlots,
      registration_fees: this.finance.registrationFees,
    };
  }

  // ── Complete Setup ─────────────────────────────────────────────────────
  async completeSetup(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      if (!orgId) throw new Error(m.onboarding_err_active_org());

      const payload = this.buildPayload(orgId);

      // ── Step 1: Update Organization Base Currency, Locale, Timezone ──
      const org = await api.getCurrentOrg().catch(() => null);
      if (org) {
        const updateRequest = {
          name: this.profile.orgName || org.name,
          primary_color: org.primary_color,
          timezone: this.profile.timezone || org.timezone,
          locale: this.profile.locale || org.locale,
        };
        await api.updateCurrentOrg(updateRequest).catch((e: any) => console.warn("Update org warning:", e));
      }

      // ── Step 2: Create Organizational Structure (Org Units) ──────────
      let rootUnitId = "";
      let branchUnitId = "";
      let existingUnits: any[] = [];
      try {
        existingUnits = await api.listOrgUnits().catch(() => []);
        
        // 1. Root Organization Unit
        const root = existingUnits?.find(
          (u: any) =>
            (u.unit_type === "root" || u.unitType === "root") &&
            !u.parent_id,
        );
        if (root) {
          rootUnitId = root.id;
        } else {
          const created = await api.createOrgUnit({
            name: this.profile.rootUnitName || this.profile.orgName,
            unit_type: "root",
            data_isolation: "standard",
          });
          rootUnitId = created?.id || "";
          if (created) existingUnits = [...existingUnits, created];
        }
        if (!rootUnitId) {
          throw new Error(m.onboarding_err_root_unit_creation());
        }

        // 2. Main Branch Unit
        const existingBranch = existingUnits?.find(
          (u: any) =>
            u.parent_id === rootUnitId &&
            (u.unit_type === "branch" || u.unitType === "branch"),
        );
        if (existingBranch) {
          branchUnitId = existingBranch.id;
        } else {
          const createdBranch = await api.createOrgUnit({
            name: "Main Branch",
            parent_id: rootUnitId,
            unit_type: "branch",
            data_isolation: "standard",
          });
          branchUnitId = createdBranch?.id || rootUnitId;
          if (createdBranch) existingUnits = [...existingUnits, createdBranch];
        }
      } catch (e) {
        throw new Error(
          `${m.onboarding_err_root_unit_creation()}: ${e instanceof Error ? e.message : String(e)}`,
        );
      }

      // 3. Buildings under Branch
      const buildingUnitIds: Record<string, string> = {};
      for (const bldName of this.facilities.buildings) {
        if (!bldName || !bldName.trim()) continue;
        try {
          const normalizedBuildingName = bldName.trim().toLowerCase();
          const existingBuilding = existingUnits?.find(
            (unit: any) =>
              unit.parent_id === branchUnitId &&
              (unit.unit_type === "building" || unit.unitType === "building") &&
              String(unit.name || "").trim().toLowerCase() === normalizedBuildingName,
          );
          const building = existingBuilding || await api.createOrgUnit({
              name: bldName.trim(),
              parent_id: branchUnitId,
              unit_type: "building",
              data_isolation: "standard",
            });
          if (!building?.id) continue;
          buildingUnitIds[bldName] = building.id;
          if (!existingBuilding) existingUnits = [...existingUnits, building];
        } catch (e) {
          console.warn(`Building unit "${bldName}" creation warning:`, e);
        }
      }

      // ── Step 3: Create Academic Year & Periods ────────────────────────
      let rootYearId = "";
      let activePeriodId = "";
      try {
        const rootYear = await api.createAcademicYear({
          name: this.calendar.academicYearName || "Academic Year",
          start_date: this.calendar.startDate,
          end_date: this.calendar.endDate,
        });
        rootYearId = rootYear?.id || "";

        const rootPeriod = await api.createAcademicPeriod({
          year_id: rootYearId,
          name: "Semester 1",
          start_date: this.calendar.startDate,
          end_date: this.calendar.endDate,
          period_type: "semester",
        });
        activePeriodId = rootPeriod?.id || "";
      } catch (e) {
        console.error("Year/period creation failed:", e);
      }

      if (!activePeriodId) {
        activePeriodId = rootYearId;
      }

      // ── Step 4: Create Classes ────────────────────────────────────────
      const createdClassIds: string[] = [];
      const targetClassOrgUnitId = branchUnitId || rootUnitId;
      for (const gl of this.academics.gradeLevels) {
        if (!gl.enabled) continue;

        for (const div of gl.divisions) {
          try {
            const createdClass = await api.createClass({
              unit_id: targetClassOrgUnitId,
              period_id: activePeriodId,
              name: `${gl.name} - ${div}`,
              level: gl.level,
            });
            if (createdClass?.id) {
              createdClassIds.push(createdClass.id);
            }
          } catch (e) {
            console.warn(`Class "${gl.name} - ${div}" creation warning:`, e);
          }
        }
      }

      // ── Step 5: Create Time Slots ─────────────────────────────────────
      for (let i = 0; i < this.academics.timeSlots.length; i++) {
        const slot = this.academics.timeSlots[i];
        try {
          await api.createTimeSlot({
            period_id: activePeriodId,
            slot_index: i + 1,
            start_time: slot.start_time,
            end_time: slot.end_time,
            day_of_week: 1,
          });
        } catch (e) {
          console.warn(`Time slot "${slot.name}" creation warning:`, e);
        }
      }

      // ── Step 6: Create Rooms ──────────────────────────────────────────
      for (const room of payload.rooms) {
        try {
          await api.createRoom({
            name: room.name,
            capacity: room.capacity || 30,
            room_type: room.room_type || "classroom",
          });
        } catch (e) {
          console.warn(`Room "${room.name}" creation warning:`, e);
        }
      }

      // ── Step 7: Create Subjects ───────────────────────────────────────
      const createdSubjectIds: string[] = [];
      for (const subj of payload.subjects) {
        try {
          const createdSubj = await api.createSubject({
            code: subj.code,
            name: subj.name,
            is_core: true,
          });
          if (createdSubj?.id) {
            createdSubjectIds.push(createdSubj.id);
          }
        } catch (e) {
          console.warn(`Subject "${subj.name}" creation warning:`, e);
        }
      }

      // ── Step 7b: Auto-allocate Curriculum (class_subjects) ───────────
      for (const classId of createdClassIds) {
        for (const subjId of createdSubjectIds) {
          try {
            await api.assignClassSubject({
              class_id: classId,
              subject_id: subjId,
              period_id: activePeriodId,
              primary_teacher_id: "",
            });
          } catch (csErr) {
            console.warn("Class-subject allocation warning:", csErr);
          }
        }
      }

      localStorage.removeItem("orbis_onboarding_state");
      toast.success(m.onboarding_toast_success());
      return true;
    } catch (err: any) {
      console.error("Setup initialization failed:", err);
      this.errorMessage =
        err?.message || String(err) || m.onboarding_err_unexpected();
      toast.error(m.onboarding_toast_error());
      return false;
    } finally {
      this.isLoading = false;
    }
  }

  localizedProfile(profileId: string) {
    const profile = INSTITUTION_PROFILES[profileId];
    if (!profile) return { name: "", description: "" };
    const locale = getLocale();
    return {
      name: locale === "ar" ? profile.name_ar : profile.name,
      description:
        locale === "ar" ? profile.description_ar : profile.description,
    };
  }
}

export const onboardingState = new OnboardingState();

function calculateDurationMinutes(startTime: string, endTime: string): number {
  if (!startTime || !endTime) return 45;
  const [h1, m1] = startTime.split(":").map(Number);
  const [h2, m2] = endTime.split(":").map(Number);
  const startMins = (h1 || 0) * 60 + (m1 || 0);
  const endMins = (h2 || 0) * 60 + (m2 || 0);
  return endMins > startMins ? endMins - startMins : 45;
}
