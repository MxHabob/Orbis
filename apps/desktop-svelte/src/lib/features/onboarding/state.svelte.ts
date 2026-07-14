import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { INSTITUTION_PROFILES } from "./templates";
import type { OnboardingPayload, PeriodTemplateDef, TemplateNode, PeriodEntry, ClassEntry, SubjectEntry } from "./types";
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
  completedSteps = $state([false, false, false, false, false, false, false, false, false]);

  // Sub-states
  profile = new ProfileInfoState();
  calendar = new CalendarInfoState();
  facilities = new FacilitiesInfoState();
  academics = new AcademicsInfoState();
  finance = new FinanceInfoState();

  // ٤?٤? Centralized Terminology Mappings ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
  terminology = $derived(
    INSTITUTION_PROFILES[this.profile.institutionType]?.terminology ||
      INSTITUTION_PROFILES.school.terminology,
  );

  stageLabel = $derived(getLocale() === "ar" ? this.terminology.stage_ar : this.terminology.stage);
  classroomLabel = $derived(getLocale() === "ar" ? this.terminology.classroom_ar : this.terminology.classroom);
  divisionLabel = $derived(getLocale() === "ar" ? this.terminology.division_ar : this.terminology.division);
  subjectLabel = $derived(getLocale() === "ar" ? this.terminology.subject_ar : this.terminology.subject);
  calendarLabel = $derived(getLocale() === "ar" ? this.terminology.calendar_ar : this.terminology.calendar);
  localizedLanguageName = $derived(this.profile.locale.startsWith("ar") ? m.onboarding_lang_ar() : m.onboarding_lang_en());
  classroomsLabelPlural = $derived(getLocale() === "ar" ? this.terminology.classroom_ar : this.terminology.classroom + "s");
  stagesLabelPlural = $derived(getLocale() === "ar" ? this.terminology.stage_ar : this.terminology.stage + "s");

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
      const bldName = this.facilities.buildings.length > 0 ? this.facilities.buildings[0] : getDefaultBuildings()[0];
      
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

  // ٤?٤? Derived Values ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
  progressValue = $derived(this.activeStep === 0 ? 0 : (this.activeStep / this.totalSteps) * 100);
  tasksCompletedCount = $derived(this.completedSteps.filter(Boolean).length);

  // ٤?٤? Persistence ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
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
    try {
      const { commands } = await import("$lib/core/ipc");
      const org = await commands.organization.getOrganization();
      if (org) {
        this.profile.orgName = org.name;
        this.profile.orgNameAr = org.name_ar || org.name;
        this.profile.rootUnitName = org.name;
        this.profile.rootUnitNameAr = org.name_ar || org.name;
        this.profile.timezone = org.timezone || this.profile.timezone;
        this.profile.locale = org.locale || this.profile.locale;
        this.profile.currency = org.currency || this.profile.currency;
        await this.loadTemplates(orgId);
        this.save();
      }
    } catch (e) {
      console.error("Failed to load existing organization:", e);
    }
  }

  // ٤?٤? Navigation ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
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

  // ٤?٤? Step 3: Room & Building Management ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
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
    this.facilities.rooms = this.facilities.rooms.filter((r) => r.building !== name);
    if (this.facilities.activeBuildingTab === name) {
      this.facilities.activeBuildingTab = this.facilities.buildings[0];
    }
    this.save();
    return true;
  }

  addDefaultRoomsToBuilding(bldName: string) {
    const otherRooms = this.facilities.rooms.filter((r) => r.building !== bldName);
    const terminology = INSTITUTION_PROFILES[this.profile.institutionType]?.terminology || INSTITUTION_PROFILES.school.terminology;
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
        name_ar: `?�?�?�?�?? ?�???�?�???�?� 1-${i}`,
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
        name_ar: `?�?�?�?�?? ?�???�?�?�?�?�?� 1-${i}`,
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

  addRoom(name: string, nameAr: string, capacity: number, roomType: string, floor: number) {
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

  // ٤?٤? Step 4: Grade/Division Management ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
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

  // ٤?٤? Step 5: Time Slot Management ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
  async loadTemplates(orgId: string) {
    try {
      const { commands } = await import("$lib/core/ipc");
      this.calendar.periodTemplates = await commands.platform.getPeriodTemplates();
      if (this.calendar.periodTemplates.length > 0 && !this.calendar.selectedTemplateId) {
        this.calendar.selectedTemplateId = this.calendar.periodTemplates[0].id;
      }
    } catch (e) {
      console.error("Failed to load period templates:", e);
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
    const durationDays = Math.round((templateNode.weight / 100) * parentDurationDays);
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
        const childDurationDays = Math.round((child.weight / 100) * durationDays);
        const childEntries = this.generatePeriodsFromTemplate(
          child,
          id,
          orgId,
          cycleStartMs,
          totalCycleDays,
          currentChildStartMs,
          durationDays,
          level + 1,
          childIdx + 1
        );
        allEntries = allEntries.concat(childEntries);
        currentChildStartMs += childDurationDays * 24 * 60 * 60 * 1000;
      });
    }
    return allEntries;
  }

  addTimeSlot(name: string, nameAr: string, startTime: string, endTime: string) {
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
    this.academics.timeSlots = this.academics.timeSlots.filter((_, i) => i !== idx);
    toast.success(m.onboarding_toast_period_deleted());
  }

  // ٤?٤? Step 6: Subject Management ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
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

  // ٤?٤? Build Final Payload ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
  buildPayload(orgId: string): OnboardingPayload {
    const periodPeriodId = `period-1447-${}`;
    const startMs = new Date(this.calendar.startDate).getTime();
    const endMs = new Date(this.calendar.endDate).getTime();
    const totalDays = Math.ceil((endMs - startMs) / (1000 * 60 * 60 * 24));

    let periods: PeriodEntry[] = [{
      id: periodPeriodId,
      parent_id: null,
      name: this.calendar.academicYearName,
      name_ar: this.calendar.academicYearNameAr,
      start_date: this.calendar.startDate,
      end_date: this.calendar.endDate,
      period_type: "year",
      weight: null,
    }];

    const template = this.calendar.periodTemplates.find((t) => t.id === this.calendar.selectedTemplateId);
    
    if (template && template.structure) {
      let currentTermStartMs = startMs;
      template.structure.forEach((term, idx) => {
        const termDurationDays = Math.round((term.weight / 100) * totalDays);
        const termEntries = this.generatePeriodsFromTemplate(
          term, periodPeriodId, orgId, startMs, totalDays, currentTermStartMs, totalDays, 1, idx + 1
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
          id: `class-${gl.level}-${div}-${}`,
          name: `${gl.name} - ${div}`,
          name_ar: `${gl.name_ar} - ?????�?� ${div}`,
          level: gl.level,
          division: div,
          color: gl.color,
        });
      }
    }

    const activeSubjects: SubjectEntry[] = this.academics.subjects
      .filter((s) => s.enabled)
      .map((s, idx) => ({
        id: `subject-${s.code.toLowerCase()}-${}`,
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
      root_unit_id: `unit-${}`,
      root_unit_name: this.profile.rootUnitName,
      root_unit_name_ar: this.profile.rootUnitNameAr,
      periods,
      time_slots: this.academics.timeSlots,
      rooms: this.facilities.rooms.map((r) => ({
        ...r,
        id: `room-${r.id}-${}`,
      })),
      subjects: activeSubjects,
      classes,
      registration_fees: $state.snapshot(this.finance.registrationFees),
    };
  }

  // ٤?٤? Complete Setup ٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?٤?
  async completeSetup(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      if (!orgId) throw new Error(m.onboarding_err_active_org());

      const payload = this.buildPayload(orgId);
      const { commands } = await import("$lib/core/ipc");
      await commands.platform.initializeSchoolSetup(payload);

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

      const org = await commands.organization.getOrganization();
      if (org) {
        const updateRequest = {
          name: org.name,
          logo: org.logo || null,
          primary_color: org.primary_color,
          currency: this.profile.currency,
          timezone: this.profile.timezone,
          locale: this.profile.locale,
          date_format: org.date_format,
          hijri_enabled: org.hijri_enabled,
          settings: JSON.stringify(settingsObj),
        };
        await commands.organization.updateOrganization(updateRequest);
      }

      for (const gl of this.academics.gradeLevels) {
        if (!gl.enabled) continue;
        const amount = this.finance.registrationFees[gl.level] || 0;
        if (amount > 0) {
          const req = {
            name: `${gl.name} Registration Fee`,
            name_ar: `?????�?� ?�???�?�?? ${gl.name_ar}`,
            code: `REG_${gl.level.toUpperCase()}`,
            category: "tuition",
            default_amount: Number(amount),
            revenue_account_id: `${}_tuition_revenue`,
            receivable_account_id: `${}_student_receivable_control`,
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
            console.error(`Failed to create fee template for ${gl.level}:`, feeErr);
          }
        }
      }

      localStorage.removeItem("orbis_onboarding_state");
      toast.success(m.onboarding_toast_success());
      return true;
    } catch (err: any) {
      console.error("Setup initialization failed:", err);
      this.errorMessage = err?.message || String(err) || m.onboarding_err_unexpected();
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
      description: locale === "ar" ? profile.description_ar : profile.description,
    };
  }
}

export const onboardingState = new OnboardingState();
