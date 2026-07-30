import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { INSTITUTION_PROFILES } from "./templates";
import type {
  OnboardingPayload,
  PeriodTemplateDef,
  TemplateNode,
  PeriodEntry,
  ClassEntry,
  SubjectEntry,
} from "./types";
import { getDefaultBuildings } from "./utils";

import { ProfileInfoState } from "./state/profile-info.svelte";
import { CalendarInfoState } from "./state/calendar-info.svelte";
import { FacilitiesInfoState } from "./state/facilities-info.svelte";
import { AcademicsInfoState } from "./state/academics-info.svelte";
import { FinanceInfoState } from "./state/finance-info.svelte";

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
      this.academics.timeSlots = profile.timeSlots.map((t) => ({
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time,
        end_time: t.end_time,
      }));

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

      // Dynamic room list prefix mapping
      const roomsList = [];
      const bldName =
        this.facilities.buildings.length > 0
          ? this.facilities.buildings[0]
          : getDefaultBuildings()[0];

      profile.stages.slice(0, 5).forEach((stg, sIdx) => {
        roomsList.push({
          id: `room-b1-f1-${sIdx + 1}`,
          name: `${profile.terminology.classroom} 1-0${sIdx + 1}`,
          name_ar: `${profile.terminology.classroom_ar} 1-0${sIdx + 1}`,
          capacity: 30,
          room_type: "classroom",
          building: bldName,
          floor: 1,
        });
      });
      this.facilities.rooms = roomsList;
      this.facilities.activeBuildingTab = bldName;

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
        completedSteps: this.completedSteps,
        activeStep: this.activeStep,
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
      const saved = localStorage.getItem("orbis_onboarding_state");
      if (saved) {
        const data = JSON.parse(saved);
        if (data.completedSteps) this.completedSteps = data.completedSteps;
        if (data.activeStep) this.activeStep = data.activeStep;

        // Handle migration from flat state to structured state
        if (data.profile) {
          this.profile.deserialize(data.profile);
          this.calendar.deserialize(data.calendar);
          this.facilities.deserialize(data.facilities);
          this.academics.deserialize(data.academics);
          this.finance.deserialize(data.finance);
        } else {
          // Fallback for old state format
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
      const { commands } = await import("$lib/core/ipc");
      const org = await commands.organization.getMain().catch(() => null) || await commands.organization.getOrganization().catch(() => null);
      if (org) {
        // Sync org profile from CLI-created data
        if (org.name) {
          this.profile.orgName = org.name;
          this.profile.rootUnitName = org.name;
        }
        if (org.name_ar) {
          this.profile.orgNameAr = org.name_ar;
          this.profile.rootUnitNameAr = org.name_ar;
        }
        if (org.timezone) this.profile.timezone = org.timezone;
        if (org.locale) this.profile.locale = org.locale;
        if (org.currency) this.profile.currency = org.currency;

        // Read institution type from org settings (set by CLI blueprint)
        let detectedType = "";
        if (org.settings) {
          try {
            const settings = typeof org.settings === 'string' ? JSON.parse(org.settings) : org.settings;
            if (settings?.registration_config?.institution_type) {
              detectedType = settings.registration_config.institution_type;
            }
          } catch { /* settings parsing is optional */ }
        }

        // Fallback: check period templates if not found in settings
        await this.loadTemplates(orgId).catch(() => {});
        if (!detectedType && this.calendar.periodTemplates && this.calendar.periodTemplates.length > 0) {
          const tmpl = this.calendar.periodTemplates.find((t: any) => t.is_default || t.isDefault) || this.calendar.periodTemplates[0];
          if (tmpl && (tmpl.institution_type || (tmpl as any).institutionType)) {
            detectedType = tmpl.institution_type || (tmpl as any).institutionType;
          }
        }

        if (detectedType) {
          // Normalize alias if needed ("training_center" <-> "institute")
          if (!INSTITUTION_PROFILES[detectedType]) {
            if (detectedType === "training_center" && INSTITUTION_PROFILES["institute"]) {
              detectedType = "institute";
            } else if (detectedType === "institute" && INSTITUTION_PROFILES["training_center"]) {
              detectedType = "training_center";
            }
          }
          if (INSTITUTION_PROFILES[detectedType]) {
            this.selectInstitutionType(detectedType);
          }
        } else if (!this.profile.institutionType) {
          this.selectInstitutionType("school");
        }

        this.save();
      }

      // Sync currency from finance module (CLI seeds currencies)
      const currencies = await commands.finance.listCurrencies().catch(() => []);
      if (currencies && currencies.length > 0) {
        const mainCurr = currencies.find((c: any) => c.isBase || c.is_base) || currencies[0];
        if (mainCurr && mainCurr.code) {
          this.profile.currency = mainCurr.code;
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
    if (this.facilities.buildings.length <= 1) return false;
    const idx = this.facilities.buildings.indexOf(name);
    if (idx === -1) return false;

    this.facilities.buildings.splice(idx, 1);
    this.facilities.rooms = this.facilities.rooms.filter(
      (r) => r.building !== name,
    );
    if (this.facilities.activeBuildingTab === name) {
      this.facilities.activeBuildingTab = this.facilities.buildings[0];
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
    for (let i = 11; i <= 20; i++) {
      defaultList.push({
        id: `room-${bldName}-${i}-${tsSuffix}`,
        name: `${terminology.classroom} 1-${i}`,
        name_ar: `${terminology.classroom_ar} 1-${i}`,
        capacity: 30,
        room_type: "classroom",
        building: bldName,
        floor: 2,
      });
    }
    for (let i = 1; i <= 3; i++) {
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
    for (let i = 1; i <= 2; i++) {
      defaultList.push({
        id: `room-${bldName}-lab-chem-${i}-${tsSuffix}`,
        name: `Chemistry Lab 1-${i}`,
        name_ar: `مختبر كيمياء 1-${i}`,
        capacity: 25,
        room_type: "chemistry_lab",
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
    if (!name || !nameAr) {
      toast.error(m.onboarding_toast_fill_room_fields());
      return false;
    }
    const id = `room-${Date.now()}`;
    this.facilities.rooms.push({
      id,
      name,
      name_ar: nameAr,
      capacity,
      room_type: roomType,
      building: this.facilities.activeBuildingTab,
      floor,
    });
    this.facilities.rooms = [...this.facilities.rooms];
    toast.success(m.onboarding_toast_room_added());
    return true;
  }

  removeRoom(id: string) {
    this.facilities.rooms = this.facilities.rooms.filter((r) => r.id !== id);
    toast.success(m.onboarding_toast_room_deleted());
  }

  // ظ¤?ظ¤? Step 4: Grade/Division Management ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  addDivision(level: string) {
    const text = this.academics.divisionInput[level]?.trim();
    if (!text) return;
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    if (gl && !gl.divisions.includes(text)) {
      gl.divisions.push(text);
      this.academics.gradeLevels = [...this.academics.gradeLevels];
      this.academics.divisionInput[level] = "";
      toast.success(m.onboarding_toast_division_added());
    }
  }

  removeDivision(level: string, div: string) {
    const gl = this.academics.gradeLevels.find((g) => g.level === level);
    if (gl) {
      gl.divisions = gl.divisions.filter((d) => d !== div);
      this.academics.gradeLevels = [...this.academics.gradeLevels];
      toast.success(m.onboarding_toast_division_deleted());
    }
  }

  // ظ¤?ظ¤? Step 5: Time Slot Management ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?ظ¤?
  async loadTemplates(orgId: string) {
    try {
      const { commands } = await import("$lib/core/ipc");
      const fetched = await (commands.organization?.getPeriodTemplates?.() || commands.calendar?.list?.() || Promise.resolve([])).catch(() => []);
      this.calendar.periodTemplates = fetched || [];
      if (
        this.calendar.periodTemplates.length > 0 &&
        !this.calendar.selectedTemplateId
      ) {
        this.calendar.selectedTemplateId = this.calendar.periodTemplates[0].id;
      }
    } catch (e) {
      console.warn("Failed to load period templates:", e);
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
  buildPayload(orgId: string): OnboardingPayload {
    // Use dynamic year-based ID instead of hardcoded Hijri 1447
    const yearStart = new Date(this.calendar.startDate).getFullYear();
    const periodPeriodId = `period-${yearStart}-${orgId}`;
    const startMs = new Date(this.calendar.startDate).getTime();
    const endMs = new Date(this.calendar.endDate).getTime();
    const totalDays = Math.ceil((endMs - startMs) / (1000 * 60 * 60 * 24));

    let periods: PeriodEntry[] = [
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
    ];

    const template = this.calendar.periodTemplates.find(
      (t) => t.id === this.calendar.selectedTemplateId,
    );

    if (template && template.structure) {
      let currentTermStartMs = startMs;
      template.structure.forEach((term, idx) => {
        const termDurationDays = Math.round((term.weight / 100) * totalDays);
        const termEntries = this.generatePeriodsFromTemplate(
          term,
          periodPeriodId,
          orgId,
          startMs,
          totalDays,
          currentTermStartMs,
          totalDays,
          1,
          idx + 1,
        );
        periods = periods.concat(termEntries);
        currentTermStartMs += termDurationDays * 24 * 60 * 60 * 1000;
      });
    }

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

    const activeSubjects: SubjectEntry[] = this.academics.subjects
      .filter((s) => s.enabled)
      .map((s, idx) => ({
        id: `subject-${s.code.toLowerCase()}-${orgId}`,
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        icon: s.icon,
        sort_order: idx + 1,
      }));

    return {
      org_name: this.profile.orgName,
      org_name_ar: this.profile.orgNameAr,
      timezone: this.profile.timezone,
      locale: this.profile.locale,
      currency: this.profile.currency,
      root_unit_id: `unit-${orgId}`,
      root_unit_name: this.profile.rootUnitName,
      root_unit_name_ar: this.profile.rootUnitNameAr,
      periods,
      time_slots: this.academics.timeSlots,
      rooms: this.facilities.rooms.map((r) => ({
        ...r,
        id: `room-${r.id}-${orgId}`,
      })),
      subjects: activeSubjects,
      classes,
      registration_fees: $state.snapshot(this.finance.registrationFees),
    };
  }

  // ── Complete Setup ─────────────────────────────────────────────────────
  async completeSetup(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      if (!orgId) throw new Error(m.onboarding_err_active_org());

      const payload = this.buildPayload(orgId);
      const { commands } = await import("$lib/core/ipc");

      // ── Step 1: Update Organization Base Currency, Locale, Timezone ──
      const chosenCurrency = this.profile.currency || "SAR";
      const settingsObj = {
        registration_config: {
          institution_type: this.profile.institutionType,
          student_number_pattern: this.profile.studentNumberPattern,
          academic_stages: this.academics.gradeLevels
            .filter((gl) => gl.enabled)
            .map((gl) => gl.level),
          required_fields: this.profile.requiredFields,
          active_fields: this.profile.activeFields,
          prevent_duplicates: this.profile.preventDuplicates,
          required_documents: this.profile.requiredDocumentsList,
        },
      };

      const org = await commands.organization.getOrganization().catch(() => null);
      if (org) {
        const updateRequest = {
          name: this.profile.orgName || org.name,
          logo: org.logo || null,
          primary_color: org.primary_color,
          currency: chosenCurrency, // Force user's selected base currency
          timezone: this.profile.timezone || org.timezone,
          locale: this.profile.locale || org.locale,
          date_format: org.date_format,
          hijri_enabled: org.hijri_enabled,
          settings: JSON.stringify(settingsObj),
        };
        await commands.organization.updateOrganization(updateRequest).catch(e => console.warn("Update org warning:", e));
      }

      // Currency is already created by the backend bootstrap_full().
      // Only ensure the base currency flag is set if needed.

      // ── Step 2: Create Organizational Structure (Org Units) ──────────
      let rootUnitId = "";
      try {
        const existingUnits = await commands.organization.listOrgUnits().catch(() => []);
        const root = existingUnits && existingUnits.find((u: any) => u.unit_type === "root" || u.unitType === "root" || !u.parent_id);
        if (root) {
          rootUnitId = root.id;
        } else {
          const created = await commands.organization.createOrgUnit({
            name: this.profile.rootUnitName || this.profile.orgName,
            name_ar: this.profile.rootUnitNameAr || this.profile.orgNameAr,
            unit_type: "root",
          });
          rootUnitId = created?.id || `unit-${orgId}`;
        }
      } catch (e) {
        console.warn("Root org unit creation warning:", e);
        rootUnitId = `unit-${orgId}`;
      }

      // Create Stage org units for enabled grade levels under Root Unit
      const stageUnitMap: Record<string, string> = {};
      for (const gl of this.academics.gradeLevels) {
        if (!gl.enabled) continue;
        try {
          const stageUnit = await commands.organization.createOrgUnit({
            name: gl.name,
            name_ar: gl.name_ar,
            parent_id: rootUnitId,
            unit_type: "stage",
            code: gl.level,
          });
          if (stageUnit?.id) {
            stageUnitMap[gl.level] = stageUnit.id;
          }
        } catch (e) {
          console.warn(`Stage unit for ${gl.level} warning:`, e);
          stageUnitMap[gl.level] = rootUnitId;
        }
      }

      // ── Step 3: Create Academic Year & Periods ────────────────────────
      let rootPeriodId = "";
      try {
        const rootPeriod = await commands.calendar.createAcademicPeriod({
          name: this.calendar.academicYearName || "Academic Year",
          name_ar: this.calendar.academicYearNameAr || "العام الدراسي",
          start_date: this.calendar.startDate,
          end_date: this.calendar.endDate,
          period_type: "year",
          parent_id: null,
        });
        rootPeriodId = rootPeriod?.id || "";
        console.log("Root academic period created successfully with ID:", rootPeriodId);
      } catch (e) {
        console.error("Root period creation failed:", e);
      }

      // Create child periods (Semesters / Quarters) under rootPeriodId
      const createdPeriodsMap: Record<string, string> = {};
      if (rootPeriodId) {
        createdPeriodsMap["root"] = rootPeriodId;
        if (payload.periods.length > 0 && payload.periods[0].id) {
          createdPeriodsMap[payload.periods[0].id] = rootPeriodId;
        }

        // Process child periods from payload
        for (const period of payload.periods.slice(1)) {
          try {
            let parentId = rootPeriodId;
            if (period.parent_id && createdPeriodsMap[period.parent_id]) {
              parentId = createdPeriodsMap[period.parent_id];
            }

            const created = await commands.calendar.createAcademicPeriod({
              parent_id: parentId,
              name: period.name,
              name_ar: period.name_ar,
              start_date: period.start_date,
              end_date: period.end_date,
              period_type: period.period_type,
              weight: period.weight ? Math.round(Number(period.weight)) : undefined,
            });
            if (created?.id && period.id) {
              createdPeriodsMap[period.id] = created.id;
            }
          } catch (e) {
            console.error(`Child period "${period.name}" creation failed:`, e);
          }
        }
      }

      const activePeriodId = rootPeriodId;
      if (!activePeriodId) {
        throw new Error("Failed to create root academic period in database");
      }

      // ── Step 4: Create Classes (الفصول) ───────────────────────────────
      for (const gl of this.academics.gradeLevels) {
        if (!gl.enabled) continue;
        const targetOrgUnitId = stageUnitMap[gl.level] || rootUnitId;

        for (const div of gl.divisions) {
          try {
            await commands.academic.createClass({
              org_unit_id: targetOrgUnitId,
              period_id: activePeriodId,
              name: `${gl.name} - ${div}`,
              name_ar: `${gl.name_ar} - الشعبة ${div}`,
              grade_level: gl.level,
              capacity: 30,
              division: div,
              color: gl.color,
            });
          } catch (e) {
            console.warn(`Class "${gl.name} - ${div}" creation warning:`, e);
          }
        }
      }

      // ── Step 5: Create Time Slots (الحصص اليومية) ─────────────────────
      for (let i = 0; i < this.academics.timeSlots.length; i++) {
        const slot = this.academics.timeSlots[i];
        const duration = calculateDurationMinutes(slot.start_time, slot.end_time);
        try {
          await commands.scheduling.createTimeSlot({
            period_id: activePeriodId,
            slot_number: i + 1,
            name: slot.name,
            name_ar: slot.name_ar,
            start_time: slot.start_time,
            end_time: slot.end_time,
            duration_minutes: duration,
          });
        } catch (e) {
          console.warn(`Time slot "${slot.name}" creation warning:`, e);
        }
      }

      // ── Step 6: Create Rooms (القاعات الدراسية) ──────────────────────
      for (const room of payload.rooms) {
        try {
          await commands.scheduling.createRoom({
            name: room.name,
            name_ar: room.name_ar,
            capacity: room.capacity || 30,
            room_type: room.room_type || "classroom",
            building: room.building,
            floor: room.floor,
          });
        } catch (e) {
          console.warn(`Room "${room.name}" creation warning:`, e);
        }
      }

      // ── Step 7: Create Subjects (المواد الدراسية) ─────────────────────
      for (const subj of payload.subjects) {
        try {
          await commands.academic.createSubject({
            code: subj.code,
            name: subj.name,
            name_ar: subj.name_ar,
          });
        } catch (e) {
          console.warn(`Subject "${subj.name}" creation warning:`, e);
        }
      }

      // ── Step 8: Create Fee Templates (الرسوم) ────────────────────────
      for (const gl of this.academics.gradeLevels) {
        if (!gl.enabled) continue;
        const amount = this.finance.registrationFees[gl.level] || 0;
        if (amount > 0) {
          const req = {
            name: `${gl.name} Registration Fee`,
            name_ar: `رسوم تسجيل ${gl.name_ar}`,
            code: `REG_${gl.level.toUpperCase()}`,
            category: "tuition",
            default_amount: Number(amount),
            revenue_account_id: `${orgId}_tuition_revenue`,
            receivable_account_id: `${orgId}_student_receivable_control`,
            applicable_levels: JSON.stringify({ levels: [gl.level] }),
            frequency: "once",
            installment_config: null,
            late_fee_rules: null,
            is_mandatory: true,
            is_active: true,
          };
          try {
            await commands.finance.createFeeTemplate(req as any);
          } catch (feeErr) {
            console.warn(`Fee template for ${gl.level} warning:`, feeErr);
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
