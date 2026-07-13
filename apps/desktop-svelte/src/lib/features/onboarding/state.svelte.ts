// ── Onboarding State Store ───────────────────────────────────────────
// Svelte 5 Runes-based state management for the onboarding wizard.
// Follows the same class-based pattern as auth.svelte.ts.

import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { INSTITUTION_PROFILES } from "./templates";
import type {
  Room,
  GradeLevel,
  TimeSlot,
  Subject,
  PeriodEntry,
  ClassEntry,
  SubjectEntry,
  OnboardingPayload,
  PeriodTemplateDef,
  TemplateNode,
} from "./types";

function detectTimezone(): string {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "Asia/Aden";
  } catch {
    return "Asia/Aden";
  }
}

function detectLocale(): string {
  try {
    const lang =
      navigator.language ||
      (navigator.languages && navigator.languages[0]) ||
      "ar";
    const cleanLang = lang.split("-")[0]?.toLowerCase();
    return cleanLang === "en" || cleanLang === "ar" ? cleanLang : "ar";
  } catch {
    return "ar";
  }
}

function detectCurrency(timezone: string): string {
  if (timezone.includes("Riyadh") || timezone.includes("Saudi")) return "SAR";
  if (timezone.includes("Dubai") || timezone.includes("Uae")) return "AED";
  if (timezone.includes("Cairo") || timezone.includes("Egypt")) return "EGP";
  if (timezone.includes("Aden") || timezone.includes("Yemen")) return "YER";
  return detectLocale() === "en" ? "USD" : "SAR"; // Smarter fallback based on locale
}

function getDefaultBuildings(): string[] {
  return detectLocale() === "ar" ? ["المبنى الرئيسي"] : ["Main Building"];
}

// ── Default Data ─────────────────────────────────────────────────────

const DEFAULT_GRADE_LEVELS: GradeLevel[] = INSTITUTION_PROFILES.school.stages.map((s) => ({
  level: s.level,
  name: s.name,
  name_ar: s.name_ar,
  color: s.color,
  enabled: true,
  divisions: [...s.divisions],
}));

const DEFAULT_TIME_SLOTS: TimeSlot[] = INSTITUTION_PROFILES.school.timeSlots.map((t) => ({
  name: t.name,
  name_ar: t.name_ar,
  start_time: t.start_time,
  end_time: t.end_time,
}));

const DEFAULT_SUBJECTS: Subject[] = INSTITUTION_PROFILES.school.subjects.map((s) => ({
  code: s.code,
  name: s.name,
  name_ar: s.name_ar,
  color: s.color,
  icon: s.icon,
  sort_order: s.sort_order,
  enabled: true,
}));

// ── Store Class ──────────────────────────────────────────────────────

class OnboardingState {
  // Wizard navigation
  activeStep = $state(0);
  isLoading = $state(false);
  errorMessage = $state<string | null>(null);
  totalSteps = 9;
  completedSteps = $state([false, false, false, false, false, false, false, false, false]);

  // Central Configuration Defaults
  institutionType = $state("school");
  studentNumberPattern = $state("STU-{YEAR}-{SEQ}");
  activeFields = $state<Record<string, boolean>>({});
  requiredFields = $state<Record<string, boolean>>({});
  preventDuplicates = $state<Record<string, boolean>>({});
  requiredDocumentsList = $state<string[]>([]);

  // Step 8: Registration Fees Setup
  registrationFees = $state<Record<string, number>>({});

  // School General Info (pre-populated by CLI / loadExistingOrg)
  orgName = $state("Sanaa Model School");
  orgNameAr = $state("مدرسة صنعاء النموذجية");
  rootUnitName = $state("Sanaa Model School");
  rootUnitNameAr = $state("مدرسة صنعاء النموذجية");
  timezone = $state(detectTimezone());
  locale = $state(detectLocale());
  currency = $state(detectCurrency(detectTimezone()));

  // Step 2: Academic Calendar
  academicYearName = $state("Academic Year 1447 Hijri");
  academicYearNameAr = $state("العام الدراسي 1447 هـ");
  startDate = $state("2026-06-21");
  endDate = $state("2027-02-21");
  selectedTemplateId = $state("");
  periodTemplates = $state<PeriodTemplateDef[]>([]);

  // Step 3: Buildings & Rooms
  rooms = $state<Room[]>([]);
  activeBuildingTab = $state(getDefaultBuildings()[0]);
  buildings = $state<string[]>(getDefaultBuildings());

  // Step 4: Grade Levels & Classes
  gradeLevels = $state<GradeLevel[]>(structuredClone(DEFAULT_GRADE_LEVELS));
  divisionInput = $state<Record<string, string>>({});

  // Step 5: Time Slots
  timeSlots = $state<TimeSlot[]>(structuredClone(DEFAULT_TIME_SLOTS));

  // Step 6: Subjects
  subjects = $state<Subject[]>(structuredClone(DEFAULT_SUBJECTS));

  // ── Centralized Terminology Mappings ──────────────────────────────
  terminology = $derived(
    INSTITUTION_PROFILES[this.institutionType]?.terminology ||
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
    this.locale.startsWith("ar")
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
    this.institutionType = type;
    const profile = INSTITUTION_PROFILES[type];
    if (profile) {
      this.studentNumberPattern = profile.numberingPattern;
      this.activeFields = { ...profile.activeFields };
      this.requiredFields = { ...profile.requiredFields };
      this.preventDuplicates = { ...profile.preventDuplicates };
      this.requiredDocumentsList = [...profile.requiredDocuments];

      // Default academic stages
      this.gradeLevels = profile.stages.map((s) => ({
        level: s.level,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        enabled: true,
        divisions: [...s.divisions],
      }));

      // Default time slots
      this.timeSlots = profile.timeSlots.map((t) => ({
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time,
        end_time: t.end_time,
      }));

      // Default subjects
      this.subjects = profile.subjects.map((s) => ({
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
      this.registrationFees = fees;

      // Dynamic room list prefix mapping
      const roomsList: Room[] = [];
      const bldName = this.buildings.length > 0 ? this.buildings[0] : getDefaultBuildings()[0];
      
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
      this.rooms = roomsList;
      this.activeBuildingTab = bldName;

      this.save();
    }
  }

  // ── Derived Values ───────────────────────────────────────────────

  progressValue = $derived(this.activeStep === 0 ? 0 : (this.activeStep / this.totalSteps) * 100);
  totalCapacity = $derived(this.rooms.reduce((acc, r) => acc + r.capacity, 0));
  enabledStages = $derived(this.gradeLevels.filter((l) => l.enabled).length);
  totalDivisions = $derived(
    this.gradeLevels
      .filter((l) => l.enabled)
      .reduce((acc, l) => acc + l.divisions.length, 0),
  );
  enabledSubjects = $derived(this.subjects.filter((s) => s.enabled).length);
  totalSlots = $derived(this.timeSlots.length);
  tasksCompletedCount = $derived(this.completedSteps.filter(Boolean).length);

  // ── Persistence ────────────────────────────────────────────────
  save() {
    try {
      const data = {
        completedSteps: this.completedSteps,
        activeStep: this.activeStep,
        registrationFees: this.registrationFees,
        academicYearName: this.academicYearName,
        academicYearNameAr: this.academicYearNameAr,
        startDate: this.startDate,
        endDate: this.endDate,
        timeSlots: this.timeSlots,
        subjects: this.subjects,
        rooms: this.rooms,
        institutionType: this.institutionType,
        studentNumberPattern: this.studentNumberPattern,
        activeFields: this.activeFields,
        requiredFields: this.requiredFields,
        preventDuplicates: this.preventDuplicates,
        requiredDocumentsList: this.requiredDocumentsList,
        gradeLevels: this.gradeLevels,
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
        if (data.registrationFees)
          this.registrationFees = data.registrationFees;
        if (data.academicYearName)
          this.academicYearName = data.academicYearName;
        if (data.academicYearNameAr)
          this.academicYearNameAr = data.academicYearNameAr;
        if (data.startDate) this.startDate = data.startDate;
        if (data.endDate) this.endDate = data.endDate;
        if (data.timeSlots) this.timeSlots = data.timeSlots;
        if (data.subjects) this.subjects = data.subjects;
        if (data.rooms) this.rooms = data.rooms;
        if (data.institutionType) this.institutionType = data.institutionType;
        if (data.studentNumberPattern)
          this.studentNumberPattern = data.studentNumberPattern;
        if (data.activeFields) this.activeFields = data.activeFields;
        if (data.requiredFields) this.requiredFields = data.requiredFields;
        if (data.preventDuplicates)
          this.preventDuplicates = data.preventDuplicates;
        if (data.requiredDocumentsList)
          this.requiredDocumentsList = data.requiredDocumentsList;
        if (data.gradeLevels) this.gradeLevels = data.gradeLevels;
      }
    } catch (e) {
      console.error("Failed to load onboarding state:", e);
    }
  }

  async loadExistingOrg(orgId: string) {
    try {
      const { commands } = await import("$lib/core/ipc");
      const org = await commands.organization.getOrganization(orgId);
      if (org) {
        this.orgName = org.name;
        this.orgNameAr = org.name_ar || org.name;
        this.rootUnitName = org.name;
        this.rootUnitNameAr = org.name_ar || org.name;
        this.timezone = org.timezone || detectTimezone();
        this.locale = org.locale || detectLocale();
        this.currency = org.currency || detectCurrency(this.timezone);
        await this.loadTemplates(orgId);
        this.save();
      }
    } catch (e) {
      console.error("Failed to load existing organization:", e);
    }
  }

  // ── Navigation ─────────────────────────────────────────────────

  goNext() {
    if (this.activeStep < this.totalSteps) {
      if (this.activeStep > 0) {
        this.completedSteps[this.activeStep - 1] = true;
      }
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

  // ── Step 3: Room & Building Management ────────────────────────────────────

  addBuilding(name: string): boolean {
    const trimmed = name.trim();
    if (!trimmed || this.buildings.includes(trimmed)) return false;
    this.buildings.push(trimmed);
    this.activeBuildingTab = trimmed;
    this.save();
    return true;
  }

  removeBuilding(name: string): boolean {
    if (this.buildings.length <= 1) return false; // Must have at least one building
    const idx = this.buildings.indexOf(name);
    if (idx === -1) return false;
    
    this.buildings.splice(idx, 1);
    this.rooms = this.rooms.filter((r) => r.building !== name);
    if (this.activeBuildingTab === name) {
      this.activeBuildingTab = this.buildings[0];
    }
    this.save();
    return true;
  }

  addDefaultRoomsToBuilding(bldName: string) {
    const otherRooms = this.rooms.filter((r) => r.building !== bldName);
    const terminology = INSTITUTION_PROFILES[this.institutionType]?.terminology || INSTITUTION_PROFILES.school.terminology;
    const defaultList: Room[] = [];
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
        name_ar: `مختبر الحاسوب 1-${i}`,
        capacity: 25,
        room_type: "computer_lab",
        building: bldName,
        floor: 2,
      });
    }
    // Chemistry labs
    for (let i = 1; i <= 2; i++) {
      defaultList.push({
        id: `room-${bldName}-lab-chem-${i}-${tsSuffix}`,
        name: `Chemistry Lab 1-${i}`,
        name_ar: `مختبر الكيمياء 1-${i}`,
        capacity: 25,
        room_type: "chemistry_lab",
        building: bldName,
        floor: 1,
      });
    }
    this.rooms = [...otherRooms, ...defaultList];
    this.activeBuildingTab = bldName;
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
    this.rooms.push({
      id,
      name,
      name_ar: nameAr,
      capacity,
      room_type: roomType,
      building: this.activeBuildingTab,
      floor,
    });
    this.rooms = [...this.rooms];
    toast.success(m.onboarding_toast_room_added());
    return true;
  }

  removeRoom(id: string) {
    this.rooms = this.rooms.filter((r) => r.id !== id);
    toast.success(m.onboarding_toast_room_deleted());
  }

  // ── Step 4: Grade/Division Management ──────────────────────────

  addDivision(level: string) {
    const text = this.divisionInput[level]?.trim();
    if (!text) return;
    const gl = this.gradeLevels.find((g) => g.level === level);
    if (gl && !gl.divisions.includes(text)) {
      gl.divisions.push(text);
      this.gradeLevels = [...this.gradeLevels];
      this.divisionInput[level] = "";
      toast.success(m.onboarding_toast_division_added());
    }
  }

  removeDivision(level: string, div: string) {
    const gl = this.gradeLevels.find((g) => g.level === level);
    if (gl) {
      gl.divisions = gl.divisions.filter((d) => d !== div);
      this.gradeLevels = [...this.gradeLevels];
      toast.success(m.onboarding_toast_division_deleted());
    }
  }

  // ── Step 5: Time Slot Management ───────────────────────────────

  async loadTemplates(orgId: string) {
    try {
      const { commands } = await import("$lib/core/ipc");
      this.periodTemplates = await commands.platform.getPeriodTemplates(orgId);
      if (this.periodTemplates.length > 0 && !this.selectedTemplateId) {
        this.selectedTemplateId = this.periodTemplates[0].id;
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
    this.timeSlots.push({
      name,
      name_ar: nameAr,
      start_time: startTime,
      end_time: endTime,
    });
    this.timeSlots = [...this.timeSlots];
    toast.success(m.onboarding_toast_period_added());
    return true;
  }

  removeTimeSlot(idx: number) {
    this.timeSlots = this.timeSlots.filter((_, i) => i !== idx);
    toast.success(m.onboarding_toast_period_deleted());
  }

  // ── Step 6: Subject Management ─────────────────────────────────

  addSubject(code: string, name: string, nameAr: string, color: string) {
    if (!code || !name || !nameAr) {
      toast.error(m.onboarding_toast_fill_subject_fields());
      return false;
    }
    this.subjects.push({
      code: code.toUpperCase(),
      name,
      name_ar: nameAr,
      color,
      icon: "book",
      sort_order: this.subjects.length + 1,
      enabled: true,
    });
    this.subjects = [...this.subjects];
    toast.success(m.onboarding_toast_subject_added());
    return true;
  }


  // ── Build Final Payload ────────────────────────────────────────

  buildPayload(orgId: string): OnboardingPayload {
    const periodPeriodId = `period-1447-${orgId}`;

    const startMs = new Date(this.startDate).getTime();
    const endMs = new Date(this.endDate).getTime();
    const totalDays = Math.ceil((endMs - startMs) / (1000 * 60 * 60 * 24));

    let periods: PeriodEntry[] = [
      {
        id: periodPeriodId,
        parent_id: null,
        name: this.academicYearName,
        name_ar: this.academicYearNameAr,
        start_date: this.startDate,
        end_date: this.endDate,
        period_type: "year",
        weight: null,
      }
    ];

    const template = this.periodTemplates.find((t) => t.id === this.selectedTemplateId);
    
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
          idx + 1
        );
        periods = periods.concat(termEntries);
        currentTermStartMs += termDurationDays * 24 * 60 * 60 * 1000;
      });
    }

    const classes: ClassEntry[] = [];
    for (const gl of this.gradeLevels) {
      if (!gl.enabled) continue;
      for (const div of gl.divisions) {
        classes.push({
          id: `class-${gl.level}-${div}-${orgId}`,
          name: `${gl.name} - ${div}`,
          name_ar: `${gl.name_ar} - شعبة ${div}`,
          level: gl.level,
          division: div,
          color: gl.color,
        });
      }
    }

    const activeSubjects: SubjectEntry[] = this.subjects
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
      org_name: this.orgName,
      org_name_ar: this.orgNameAr,
      timezone: this.timezone,
      locale: this.locale,
      currency: this.currency,
      root_unit_id: `unit-${orgId}`,
      root_unit_name: this.rootUnitName,
      root_unit_name_ar: this.rootUnitNameAr,
      periods,
      time_slots: this.timeSlots,
      rooms: this.rooms.map((r) => ({
        ...r,
        id: `room-${r.id}-${orgId}`,
      })),
      subjects: activeSubjects,
      classes,
      registration_fees: $state.snapshot(this.registrationFees),
    };
  }

  // ── Complete Setup ─────────────────────────────────────────────

  async completeSetup(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      if (!orgId) {
        throw new Error(m.onboarding_err_active_org());
      }

      const payload = this.buildPayload(orgId);
      console.log("Submitting Onboarding Setup Payload:", payload);

      const { commands } = await import("$lib/core/ipc");
      await commands.platform.initializeSchoolSetup(orgId, payload);

      // Save registration configuration to organization settings
      const settingsObj = {
        registration_config: {
          institution_type: this.institutionType,
          student_number_pattern: this.studentNumberPattern,
          academic_stages: this.gradeLevels
            .filter((gl) => gl.enabled)
            .map((gl) => gl.level),
          required_fields: this.requiredFields,
          active_fields: this.activeFields,
          prevent_duplicates: this.preventDuplicates,
          required_documents: this.requiredDocumentsList,
        },
      };

      const org = await commands.organization.getOrganization(orgId);
      if (org) {
        const updateRequest = {
          name: org.name,
          logo: org.logo || null,
          primary_color: org.primary_color,
          currency: this.currency,
          timezone: this.timezone,
          locale: this.locale,
          date_format: org.date_format,
          hijri_enabled: org.hijri_enabled,
          settings: JSON.stringify(settingsObj),
        };
        await commands.organization.updateOrganization(orgId, updateRequest);
      }

      // Create fee templates for the specified registration fees
      for (const gl of this.gradeLevels) {
        if (!gl.enabled) continue;
        const amount = this.registrationFees[gl.level] || 0;
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
            console.error(
              `Failed to create fee template for ${gl.level}:`,
              feeErr,
            );
          }
        }
      }

      // Mark setup as complete in localStorage
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
