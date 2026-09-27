// ── Onboarding State ──────────────────────────────────────────────────
// Central orchestrator for the 10-step academic onboarding wizard.

import { AcademicsInfoState } from "./state/academics-info.svelte";
import { CalendarInfoState } from "./state/calendar-info.svelte";
import { FacilitiesInfoState } from "./state/facilities-info.svelte";
import { FinanceInfoState } from "./state/finance-info.svelte";
import { ProfileInfoState } from "./state/profile-info.svelte";

import { api } from "$lib/api";
import type { CompleteOrganizationOnboardingRequest } from "$lib/api/types";
import { getLocale } from "$lib/paraglide/runtime";
import { authStore } from "$lib/stores/auth.svelte";
import { orgContext } from "$lib/stores/org-context.svelte";
import { systemStore } from "$lib/stores/system.svelte";
import { INSTITUTION_PROFILES, type InstitutionProfile } from "./templates";
import type { Room, TimeSlot } from "./types";

const STORAGE_KEY = "orbis_onboarding_state_v3";

export class OnboardingState {
  currentStep = $state(0);
  isLoading = $state(false);
  errorMessage = $state<string | null>(null);

  profile = new ProfileInfoState();
  calendar = new CalendarInfoState();
  academics = new AcademicsInfoState();
  facilities = new FacilitiesInfoState();
  finance = new FinanceInfoState();

  readonly totalSteps = 9;

  constructor() {
    this.init();
  }

  // ── Derived Properties & Labels ─────────────────────────────────────

  get isFirstStep() {
    return this.currentStep === 0;
  }

  get isLastStep() {
    return this.currentStep === this.totalSteps - 1;
  }

  get progressPercentage() {
    return Math.round(((this.currentStep + 1) / this.totalSteps) * 100);
  }

  get isArabic() {
    return getLocale().startsWith("ar");
  }

  get currentProfile(): InstitutionProfile {
    return (
      INSTITUTION_PROFILES[this.profile.institutionType] ||
      INSTITUTION_PROFILES["school"]
    );
  }

  get stageLabel(): string {
    return this.isArabic
      ? this.currentProfile.terminology.stage_ar
      : this.currentProfile.terminology.stage;
  }

  get stagesLabelPlural(): string {
    return this.isArabic ? "مراحل" : "Stages";
  }

  get classroomLabel(): string {
    return this.isArabic
      ? this.currentProfile.terminology.classroom_ar
      : this.currentProfile.terminology.classroom;
  }

  get classroomsLabelPlural(): string {
    return this.isArabic ? "قاعات" : "Classrooms";
  }

  get divisionLabel(): string {
    return this.isArabic
      ? this.currentProfile.terminology.division_ar
      : this.currentProfile.terminology.division;
  }

  get subjectLabel(): string {
    return this.isArabic
      ? this.currentProfile.terminology.subject_ar
      : this.currentProfile.terminology.subject;
  }

  get calendarLabel(): string {
    return this.isArabic
      ? this.currentProfile.terminology.calendar_ar
      : this.currentProfile.terminology.calendar;
  }

  get localizedLanguageName(): string {
    return this.profile.locale.startsWith("ar") ? "العربية" : "English";
  }

  localizedProfile(type: string) {
    const prof = INSTITUTION_PROFILES[type] || INSTITUTION_PROFILES["school"];
    return {
      name: this.isArabic ? prof.name_ar : prof.name,
      description: this.isArabic ? prof.description_ar : prof.description,
    };
  }

  // ── Navigation ──────────────────────────────────────────────────────

  nextStep() {
    if (this.currentStep < this.totalSteps - 1) {
      this.currentStep += 1;
      this.save();
    }
  }

  prevStep() {
    if (this.currentStep > 0) {
      this.currentStep -= 1;
      this.save();
    }
  }

  goToStep(step: number) {
    if (step >= 0 && step < this.totalSteps) {
      this.currentStep = step;
      this.save();
    }
  }

  // ── Preset & Profile Selection ──────────────────────────────────────

  selectInstitutionType(type: string) {
    const prof = INSTITUTION_PROFILES[type];
    if (!prof) return;

    this.profile.institutionType = type;
    this.profile.studentNumberPattern = prof.numberingPattern;
    this.profile.activeFields = { ...prof.activeFields };
    this.profile.requiredFields = { ...prof.requiredFields };
    this.profile.preventDuplicates = { ...prof.preventDuplicates };
    this.profile.requiredDocumentsList = [...prof.requiredDocuments];

    // Populate Grade Levels
    this.academics.gradeLevels = prof.stages.map((stage) => ({
      level: stage.level,
      name: stage.name,
      name_ar: stage.name_ar,
      color: stage.color,
      enabled: true,
      divisions: [...stage.divisions],
    }));

    // Populate Time Slots
    this.academics.timeSlots = prof.timeSlots.map((slot) => ({
      name: slot.name,
      name_ar: slot.name_ar,
      start_time: slot.start_time,
      end_time: slot.end_time,
      is_break: slot.is_break ?? false,
      break_type: slot.break_type,
      applicable_days: slot.applicable_days
        ? [...slot.applicable_days]
        : [...this.academics.workingDays],
    }));

    // Populate Subjects
    this.academics.subjects = prof.subjects.map((sub) => ({
      code: sub.code,
      name: sub.name,
      name_ar: sub.name_ar,
      color: sub.color,
      icon: sub.icon,
      sort_order: sub.sort_order,
      enabled: sub.enabled !== undefined ? sub.enabled : true,
    }));

    // Reset and re-populate rooms for current buildings
    this.facilities.rooms = [];
    for (const bld of this.facilities.buildings) {
      this.addDefaultRoomsToBuilding(bld);
    }

    this.save();
  }

  // ── Facilities Mutations ───────────────────────────────────────────

  addBuilding(name: string) {
    if (!name.trim()) return;
    const clean = name.trim();
    if (!this.facilities.buildings.includes(clean)) {
      this.facilities.buildings.push(clean);
      this.facilities.activeBuildingTab = clean;
      this.addDefaultRoomsToBuilding(clean);
      this.save();
    }
  }

  removeBuilding(name: string) {
    if (this.facilities.buildings.length <= 1) return;
    this.facilities.buildings = this.facilities.buildings.filter(
      (b) => b !== name
    );
    this.facilities.rooms = this.facilities.rooms.filter(
      (r) => r.building !== name
    );
    this.facilities.activeBuildingTab = this.facilities.buildings[0] || "";
    this.save();
  }

  addDefaultRoomsToBuilding(building: string) {
    const existing = this.facilities.rooms.filter((r) => r.building === building);
    if (existing.length > 0) return;

    const instType = this.profile.institutionType || "school";
    if (instType === "university") {
      for (let i = 1; i <= 6; i++) {
        this.facilities.rooms.push({
          id: `room-${building}-${Date.now()}-${i}`,
          name: `Lecture Hall ${i}`,
          name_ar: `قاعة المحاضرات ${i}`,
          capacity: 60,
          room_type: "lecture_hall",
          building,
          floor: i <= 3 ? 1 : 2,
        });
      }
      this.facilities.rooms.push(
        {
          id: `lab-sci-${building}-${Date.now()}`,
          name: `Advanced Research Lab`,
          name_ar: `معمل البحوث المتقدمة`,
          capacity: 30,
          room_type: "science_lab",
          building,
          floor: 1,
        },
        {
          id: `lab-comp-${building}-${Date.now()}`,
          name: `Computer & AI Center`,
          name_ar: `مركز الحاسوب والذكاء الاصطناعي`,
          capacity: 40,
          room_type: "computer_lab",
          building,
          floor: 2,
        },
        {
          id: `lib-${building}-${Date.now()}`,
          name: `University Central Library`,
          name_ar: `المكتبة الجامعية المركزية`,
          capacity: 80,
          room_type: "library",
          building,
          floor: 1,
        },
        {
          id: `aud-${building}-${Date.now()}`,
          name: `Grand Auditorium`,
          name_ar: `المدرج الرئيسي للمؤتمرات`,
          capacity: 150,
          room_type: "activity_hall",
          building,
          floor: 1,
        },
        {
          id: `vroom-${building}-${Date.now()}`,
          name: `Virtual Lecture Hall (Online)`,
          name_ar: `قاعة المحاضرات الافتراضية (عن بُعد)`,
          capacity: 300,
          room_type: "virtual_classroom",
          building,
          floor: 0,
        }
      );
    } else if (instType === "institute") {
      for (let i = 1; i <= 4; i++) {
        this.facilities.rooms.push({
          id: `room-${building}-${Date.now()}-${i}`,
          name: `Training Room ${i}`,
          name_ar: `قاعة التدريب ${i}`,
          capacity: 25,
          room_type: "classroom",
          building,
          floor: 1,
        });
      }
      this.facilities.rooms.push(
        {
          id: `lab-comp-${building}-${Date.now()}`,
          name: `IT & Multimedia Lab`,
          name_ar: `معمل الحاسب والوسائط`,
          capacity: 25,
          room_type: "computer_lab",
          building,
          floor: 1,
        },
        {
          id: `lab-lang-${building}-${Date.now()}`,
          name: `Languages Lab`,
          name_ar: `مختبر اللغات الحديثة`,
          capacity: 25,
          room_type: "workshop",
          building,
          floor: 1,
        },
        {
          id: `vroom-${building}-${Date.now()}`,
          name: `Virtual Classroom (Online)`,
          name_ar: `القاعة الافتراضية للتدريب`,
          capacity: 100,
          room_type: "virtual_classroom",
          building,
          floor: 0,
        }
      );
    } else if (instType === "training_center") {
      for (let i = 1; i <= 3; i++) {
        this.facilities.rooms.push({
          id: `room-${building}-${Date.now()}-${i}`,
          name: `Workshop Hall ${i}`,
          name_ar: `قاعة ورش العمل ${i}`,
          capacity: 30,
          room_type: "workshop",
          building,
          floor: 1,
        });
      }
      this.facilities.rooms.push(
        {
          id: `lab-comp-${building}-${Date.now()}`,
          name: `Executive Tech Lab`,
          name_ar: `معمل التدريب التقني التنفيذي`,
          capacity: 25,
          room_type: "computer_lab",
          building,
          floor: 1,
        },
        {
          id: `vroom-${building}-${Date.now()}`,
          name: `Virtual Webinar Room`,
          name_ar: `قاعة الندوات الافتراضية (ويبينار)`,
          capacity: 200,
          room_type: "virtual_classroom",
          building,
          floor: 0,
        }
      );
    } else {
      // Default: Home
      for (let i = 1; i <= 8; i++) {
        this.facilities.rooms.push({
          id: `room-${building}-${Date.now()}-${i}`,
          name: `Classroom ${i}`,
          name_ar: `الفصل ${i}`,
          capacity: 30,
          room_type: "classroom",
          building,
          floor: i <= 4 ? 1 : 2,
        });
      }
      this.facilities.rooms.push(
        {
          id: `lab-sci-${building}-${Date.now()}`,
          name: `Science Lab`,
          name_ar: `المختبر العلمي`,
          capacity: 25,
          room_type: "science_lab",
          building,
          floor: 1,
        },
        {
          id: `lab-comp-${building}-${Date.now()}`,
          name: `Computer Lab`,
          name_ar: `معمل الحاسوب`,
          capacity: 25,
          room_type: "computer_lab",
          building,
          floor: 2,
        },
        {
          id: `lib-${building}-${Date.now()}`,
          name: `Library & Learning Resources`,
          name_ar: `المكتبة ومصادر التعلم`,
          capacity: 40,
          room_type: "library",
          building,
          floor: 1,
        },
        {
          id: `act-${building}-${Date.now()}`,
          name: `Activity Hall`,
          name_ar: `قاعة الأنشطة والفعاليات`,
          capacity: 60,
          room_type: "activity_hall",
          building,
          floor: 1,
        },
        {
          id: `vroom-${building}-${Date.now()}`,
          name: `Virtual Classroom (Online)`,
          name_ar: `القاعة الافتراضية (تعليم عن بُعد)`,
          capacity: 100,
          room_type: "virtual_classroom",
          building,
          floor: 0,
        }
      );
    }
    this.save();
  }

  addRoom(
    name: string,
    name_ar: string,
    capacity: number,
    room_type: string,
    floor: number
  ): boolean {
    if (!name.trim() || !name_ar.trim() || capacity <= 0) return false;
    const building = this.facilities.activeBuildingTab;
    if (!building) return false;

    const newRoom: Room = {
      id: `room-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`,
      name: name.trim(),
      name_ar: name_ar.trim(),
      capacity: Number(capacity),
      room_type,
      building,
      floor: Number(floor) || 0,
    };

    this.facilities.rooms.push(newRoom);
    this.save();
    return true;
  }

  removeRoom(id: string) {
    this.facilities.rooms = this.facilities.rooms.filter((r) => r.id !== id);
    this.save();
  }

  // ── Time Slots Mutations ────────────────────────────────────────────

  addTimeSlot(
    name: string,
    name_ar: string,
    start_time: string,
    end_time: string,
    options?: { isBreak?: boolean; breakType?: any }
  ): boolean {
    if (!name.trim() || !name_ar.trim()) return false;

    const newSlot: TimeSlot = {
      name: name.trim(),
      name_ar: name_ar.trim(),
      start_time,
      end_time,
      is_break: options?.isBreak ?? false,
      break_type: options?.breakType,
      applicable_days: [...this.academics.workingDays],
    };

    this.academics.timeSlots.push(newSlot);
    this.save();
    return true;
  }

  removeTimeSlot(index: number) {
    if (index >= 0 && index < this.academics.timeSlots.length) {
      this.academics.timeSlots.splice(index, 1);
      this.save();
    }
  }

  // ── Academics / Grades Mutations ────────────────────────────────────

  addDivision(level: string) {
    const gl = this.academics.gradeLevels.find((l) => l.level === level);
    if (!gl) return;

    const rawInput = this.academics.divisionInput[level]?.trim() || "";
    if (!rawInput) {
      // Auto-assign next letter
      const alphabet = ["A", "B", "C", "D", "E", "F", "G"];
      const nextLetter = alphabet.find((l) => !gl.divisions.includes(l)) || `${gl.divisions.length + 1}`;
      gl.divisions.push(nextLetter);
    } else {
      const parts = rawInput.split(/[,،\s]+/).filter(Boolean);
      for (const part of parts) {
        if (!gl.divisions.includes(part)) {
          gl.divisions.push(part);
        }
      }
      this.academics.divisionInput[level] = "";
    }
    this.save();
  }

  addNamedDivision(level: string, name: string) {
    const gl = this.academics.gradeLevels.find((l) => l.level === level);
    if (!gl) return;
    if (!gl.divisions.includes(name)) {
      gl.divisions.push(name);
      this.save();
    }
  }

  renameDivision(level: string, divisionIndex: number, newName: string) {
    const gl = this.academics.gradeLevels.find((l) => l.level === level);
    if (!gl) return;
    if (divisionIndex >= 0 && divisionIndex < gl.divisions.length && newName.trim()) {
      gl.divisions[divisionIndex] = newName.trim();
      this.save();
    }
  }

  removeDivision(level: string, div: string) {
    const gl = this.academics.gradeLevels.find((l) => l.level === level);
    if (!gl) return;
    gl.divisions = gl.divisions.filter((d) => d !== div);
    this.save();
  }

  // ── Subjects Mutations ──────────────────────────────────────────────

  addSubject(code: string, name: string, name_ar?: string, color = "#f59e0b"): boolean {
    if (!code.trim() || !name.trim()) return false;
    const cleanCode = code.trim().toUpperCase();

    if (this.academics.subjects.some((s) => s.code === cleanCode)) {
      return false;
    }

    this.academics.subjects.push({
      code: cleanCode,
      name: name.trim(),
      name_ar: name_ar?.trim() || name.trim(),
      color,
      icon: "BookOpen",
      sort_order: this.academics.subjects.length + 1,
      enabled: true,
    });

    this.save();
    return true;
  }

  // ── Persistence & Initialization ────────────────────────────────────

  async init(orgDto?: any) {
    // 1. If stored in localStorage, load it
    this.load();

    // 2. If organization info is present from passed orgDto or systemStore or API
    if (orgDto) {
      if (orgDto.name) this.profile.orgName = orgDto.name;
      if (orgDto.name_ar) this.profile.orgNameAr = orgDto.name_ar;
      const curr = orgDto.base_currency_code || orgDto.baseCurrencyCode || orgDto.currency;
      if (curr) this.profile.currency = curr;
      if (orgDto.timezone) this.profile.timezone = orgDto.timezone;
      if (orgDto.locale) this.profile.locale = orgDto.locale;
      const type = orgDto.org_type || orgDto.institution_type || orgDto.type;
      if (type) this.profile.institutionType = type;
    } else if (systemStore.orgReadiness) {
      const r = systemStore.orgReadiness;
      if (r.org_name && !this.profile.orgName) this.profile.orgName = r.org_name;
      const type = (r as any).org_type || (r as any).institution_type || (r as any).type;
      if (type && !this.profile.institutionType) this.profile.institutionType = type;
    }

    if (authStore.isAuthenticated) {
      try {
        const org: any = await api.getCurrentOrg();
        if (org) {
          if (org.name) this.profile.orgName = org.name;
          if (org.name_ar) this.profile.orgNameAr = org.name_ar;
          const curr = org.base_currency_code || org.baseCurrencyCode || org.currency;
          if (curr) this.profile.currency = curr;
          if (org.timezone) this.profile.timezone = org.timezone;
          if (org.locale) this.profile.locale = org.locale;
          const type = org.org_type || org.institution_type || org.type;
          if (type && !this.profile.institutionType) {
            this.profile.institutionType = type;
          }
        }
      } catch {
        // Silently continue with defaults
      }
    }

    // Also sync from orgContext if already initialized
    if (orgContext.isInitialized && orgContext.baseCurrency) {
      this.profile.currency = orgContext.baseCurrency;
      if (orgContext.name && !this.profile.orgName) this.profile.orgName = orgContext.name;
      if (orgContext.nameAr && !this.profile.orgNameAr) this.profile.orgNameAr = orgContext.nameAr;
      if (orgContext.timezone && !this.profile.timezone) this.profile.timezone = orgContext.timezone;
    }

    // 3. Ensure complete profile is applied if anything is missing
    if (
      this.academics.gradeLevels.length < 10 ||
      this.academics.timeSlots.length < 7 ||
      this.academics.subjects.length < 18
    ) {
      this.selectInstitutionType(this.profile.institutionType || "school");
    }

    this.save();
  }

  save() {
    try {
      if (typeof window === "undefined" || !window.localStorage) return;
      const data = {
        orgId: authStore.orgId,
        currentStep: this.currentStep,
        profile: this.profile.serialize(),
        calendar: this.calendar.serialize(),
        academics: this.academics.serialize(),
        facilities: this.facilities.serialize(),
        finance: this.finance.serialize(),
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
    } catch (e) {
      console.warn("Failed to persist onboarding state:", e);
    }
  }

  load(): boolean {
    try {
      if (typeof window === "undefined" || !window.localStorage) return false;
      const raw = localStorage.getItem(STORAGE_KEY);
      if (!raw) return false;
      const data = JSON.parse(raw);

      // If stored state is from a different org_id, don't load stale cache
      if (data.orgId && authStore.orgId && data.orgId !== authStore.orgId) {
        return false;
      }

      if (data.profile) this.profile.deserialize(data.profile);
      if (data.calendar) this.calendar.deserialize(data.calendar);
      if (data.academics) this.academics.deserialize(data.academics);
      if (data.facilities) this.facilities.deserialize(data.facilities);
      if (data.finance) this.finance.deserialize(data.finance);
      if (typeof data.currentStep === "number") {
        this.currentStep = Math.min(Math.max(0, data.currentStep), this.totalSteps - 1);
      }
      return true;
    } catch (e) {
      console.warn("Failed to load onboarding state:", e);
      return false;
    }
  }

  reset() {
    try {
      if (typeof window !== "undefined" && window.localStorage) {
        localStorage.removeItem(STORAGE_KEY);
      }
      this.currentStep = 0;
      this.errorMessage = null;
      this.profile = new ProfileInfoState();
      this.calendar = new CalendarInfoState();
      this.academics = new AcademicsInfoState();
      this.facilities = new FacilitiesInfoState();
      this.finance = new FinanceInfoState();
      this.selectInstitutionType("school");
    } catch (e) {
      console.warn("Failed to reset onboarding state:", e);
    }
  }

  // ── Final Setup & Backend Bootstrap ─────────────────────────────────

  async completeSetup(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.errorMessage = null;

    const targetOrgId = orgId || authStore.orgId;
    if (!targetOrgId) {
      this.errorMessage = "No active organization ID found.";
      this.isLoading = false;
      return false;
    }

    try {
      // 1. Periods generation from selected template
      const selectedTpl = this.calendar.periodTemplates.find(
        (t) => t.id === this.calendar.selectedTemplateId
      );

      let periods: Array<{
        id?: string;
        parentId?: string;
        name: string;
        nameAr?: string;
        periodType: string;
        startDate: string;
        endDate: string;
      }> = [];

      const startMs = new Date(this.calendar.startDate).getTime();
      const endMs = new Date(this.calendar.endDate).getTime();
      const totalDuration = Math.max(endMs - startMs, 86400000);

      if (selectedTpl?.structure && selectedTpl.structure.length > 0) {
        const numTerms = selectedTpl.structure.length;
        const termDuration = totalDuration / numTerms;

        for (let i = 0; i < numTerms; i++) {
          const item = selectedTpl.structure[i];
          const termStart = new Date(startMs + i * termDuration);
          const termEnd = new Date(startMs + (i + 1) * termDuration - 86400000);
          const termId = `term_${i + 1}`;

          const termStartStr = termStart.toISOString().split("T")[0];
          const termEndStr = termEnd.toISOString().split("T")[0];

          periods.push({
            id: termId,
            name: item.name || `Period ${i + 1}`,
            nameAr: item.name_ar || `الفصل ${i + 1}`,
            periodType: item.type || "semester",
            startDate: termStartStr,
            endDate: termEndStr,
          });

          // Also generate monthly assessment sub-periods linked to this parent term
          if (Array.isArray(item.children) && item.children.length > 0) {
            const numChildren = item.children.length;
            const childDuration = (termEnd.getTime() - termStart.getTime()) / numChildren;

            for (let j = 0; j < numChildren; j++) {
              const child = item.children[j];
              const childStart = new Date(termStart.getTime() + j * childDuration);
              const childEnd = new Date(termStart.getTime() + (j + 1) * childDuration - 86400000);

              periods.push({
                id: `${termId}_sub_${j + 1}`,
                parentId: termId,
                name: `${item.name || `Term ${i + 1}`} - ${child.name || `Month ${j + 1}`}`,
                nameAr: `${item.name_ar || `الفصل ${i + 1}`} - ${child.name_ar || `الشهر ${j + 1}`}`,
                periodType: child.type || "monthly",
                startDate: childStart.toISOString().split("T")[0],
                endDate: childEnd.toISOString().split("T")[0],
              });
            }
          }
        }
      } else {
        periods = [
          {
            id: "sem_1",
            name: "First Semester",
            nameAr: "الفصل الدراسي الأول",
            periodType: "semester",
            startDate: this.calendar.startDate,
            endDate: this.calendar.endDate,
          },
        ];
      }

      // 2. Classes generation from enabled grade levels & divisions
      const enabledGrades = this.academics.gradeLevels.filter((gl) => gl.enabled);
      const classes: Array<{
        name: string;
        nameAr?: string;
        level: string;
        division?: string;
      }> = [];

      for (const gl of enabledGrades) {
        if (gl.divisions.length > 0) {
          for (const div of gl.divisions) {
            classes.push({
              name: `${gl.name} - ${div}`,
              nameAr: `${gl.name_ar} - ${div}`,
              level: gl.name,
              division: div,
            });
          }
        } else {
          classes.push({
            name: gl.name,
            nameAr: gl.name_ar,
            level: gl.name,
          });
        }
      }

      if (classes.length === 0) {
        classes.push({
          name: "Grade 1 - A",
          nameAr: "الصف الأول - أ",
          level: "Grade 1",
          division: "A",
        });
      }

      // 3. Subjects generation from enabled subjects
      const enabledSubjects = this.academics.subjects.filter((s) => s.enabled);
      const subjects: Array<{
        code: string;
        name: string;
        nameAr?: string;
        isCore: boolean;
      }> = (enabledSubjects.length > 0 ? enabledSubjects : this.academics.subjects).map(
        (s) => ({
          code: s.code,
          name: s.name,
          nameAr: s.name_ar,
          isCore: true,
        })
      );

      // 4. Time slots generation
      const timeSlots = this.academics.timeSlots.map((ts) => ({
        name: ts.name,
        nameAr: ts.name_ar,
        startTime: ts.start_time,
        endTime: ts.end_time,
        isBreak: ts.is_break,
        breakType: ts.break_type,
      }));

      // 5. Facilities generation
      const facilities = {
        buildings: this.facilities.buildings || [],
        rooms: (this.facilities.rooms || []).map((rm) => ({
          name: rm.name,
          roomType: rm.room_type,
          capacity: rm.capacity,
          building: rm.building,
        })),
      };

      // 6. Finance configuration & fee templates
      const feeTemplates = Object.entries(this.finance.registrationFees).map(([levelCode, amount]) => {
        const gradeLevelObj = this.academics.gradeLevels.find((gl) => gl.level === levelCode);
        const nameAr = gradeLevelObj?.name_ar ? `رسوم ${gradeLevelObj.name_ar}` : `رسوم ${levelCode}`;
        const nameEn = gradeLevelObj?.name ? `${gradeLevelObj.name} Tuition Fee` : `${levelCode} Fee`;

        return {
          code: `FEE_${levelCode.toUpperCase()}`,
          name: nameEn,
          nameAr,
          category: "tuition",
          defaultAmount: amount,
          applicableLevels: [levelCode],
          frequency: "annual",
          isMandatory: true,
        };
      });

      const finance = {
        initializeChartOfAccounts: true,
        taxRate: 0.15,
        feeTemplates,
        discountTemplates: [],
      };

      // 7. Profile & Registration configuration
      const profile = {
        institutionType: this.profile.institutionType,
        name: this.profile.orgName || (this.isArabic ? "مؤسسة أوربس التعليمية" : "Orbis Educational Institution"),
        nameAr: this.profile.orgNameAr || undefined,
        baseCurrencyCode: this.profile.currency || "SAR",
        timezone: this.profile.timezone || "Asia/Riyadh",
        locale: this.profile.locale || "ar-SA",
        studentNumberPattern: this.profile.studentNumberPattern,
        requiredFields: this.profile.requiredFields,
        activeFields: this.profile.activeFields,
        preventDuplicates: this.profile.preventDuplicates,
        requiredDocuments: this.profile.requiredDocumentsList,
      };

      // 8. Complete Atomic Onboarding Request
      const payload: CompleteOrganizationOnboardingRequest = {
        orgId: targetOrgId,
        profile,
        calendar: {
          academicYearName: this.calendar.academicYearName || "Academic Year 2026/2027",
          academicYearNameAr: this.calendar.academicYearNameAr || undefined,
          startDate: this.calendar.startDate,
          endDate: this.calendar.endDate,
          periods,
        },
        academics: {
          classes,
          subjects,
          timeSlots,
          workingDays: this.academics.workingDays,
        },
        facilities,
        finance,
      };

      // 9. BarsArrowUp single atomic request to authoritative Onboarding Engine
      const result = await systemStore.completeOnboarding(payload);
      if (!result.success) {
        throw new Error("Onboarding completion failed");
      }

      // 10. Refresh active organization context
      await orgContext.refresh();

      // 11. Clean up local persistence
      if (typeof window !== "undefined" && window.localStorage) {
        localStorage.removeItem(STORAGE_KEY);
      }

      return true;
    } catch (e) {
      this.errorMessage = e instanceof Error ? e.message : String(e);
      return false;
    } finally {
      this.isLoading = false;
    }
  }
}

export const onboardingState = new OnboardingState();
