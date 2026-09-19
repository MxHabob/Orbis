// ── Onboarding State ──────────────────────────────────────────────────
// Central orchestrator for the 10-step academic onboarding wizard.

import { ProfileInfoState } from "./state/profile-info.svelte";
import { CalendarInfoState } from "./state/calendar-info.svelte";
import { AcademicsInfoState } from "./state/academics-info.svelte";
import { FacilitiesInfoState } from "./state/facilities-info.svelte";
import { FinanceInfoState } from "./state/finance-info.svelte";

import { INSTITUTION_PROFILES, type InstitutionProfile } from "./templates";
import { getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import { authStore } from "$lib/stores/auth.svelte";
import { api } from "$lib/api";
import type { InitialSetupRequest } from "$lib/api/types";
import type { Room, TimeSlot } from "./types";

const STORAGE_KEY = "orbis_onboarding_state";

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
      // Default: School
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
      if (orgDto.currency) this.profile.currency = orgDto.currency;
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
          if (org.currency) this.profile.currency = org.currency;
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

    // 3. If no grade levels or subjects loaded yet, apply profile chosen in setup
    if (this.academics.gradeLevels.length === 0) {
      this.selectInstitutionType(this.profile.institutionType || "school");
    }
  }

  save() {
    try {
      if (typeof window === "undefined" || !window.localStorage) return;
      const data = {
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
        name: string;
        name_ar?: string;
        period_type: string;
        start_date: string;
        end_date: string;
      }> = [];

      if (selectedTpl?.structure && selectedTpl.structure.length > 0) {
        periods = selectedTpl.structure.map((item, idx) => ({
          name: item.name || `Period ${idx + 1}`,
          name_ar: item.name_ar || `الفصل ${idx + 1}`,
          period_type: item.type || "semester",
          start_date: this.calendar.startDate,
          end_date: this.calendar.endDate,
        }));
      } else {
        periods = [
          {
            name: "First Semester",
            name_ar: "الفصل الدراسي الأول",
            period_type: "semester",
            start_date: this.calendar.startDate,
            end_date: this.calendar.endDate,
          },
        ];
      }

      // 2. Classes generation from enabled grade levels & divisions
      const enabledGrades = this.academics.gradeLevels.filter((gl) => gl.enabled);
      const classes: Array<{
        name: string;
        name_ar?: string;
        level: string;
        division?: string;
      }> = [];

      for (const gl of enabledGrades) {
        if (gl.divisions.length > 0) {
          for (const div of gl.divisions) {
            classes.push({
              name: `${gl.name} - ${div}`,
              name_ar: `${gl.name_ar} - ${div}`,
              level: gl.name,
              division: div,
            });
          }
        } else {
          classes.push({
            name: gl.name,
            name_ar: gl.name_ar,
            level: gl.name,
          });
        }
      }

      if (classes.length === 0) {
        classes.push({
          name: "Grade 1 - A",
          name_ar: "الصف الأول - أ",
          level: "Grade 1",
          division: "A",
        });
      }

      // 3. Subjects generation from enabled subjects
      const enabledSubjects = this.academics.subjects.filter((s) => s.enabled);
      const subjects: Array<{
        code: string;
        name: string;
        name_ar?: string;
        is_core: boolean;
      }> = (enabledSubjects.length > 0 ? enabledSubjects : this.academics.subjects).map(
        (s) => ({
          code: s.code,
          name: s.name,
          name_ar: s.name_ar,
          is_core: true,
        })
      );

      // 4. Initial Setup Request
      const payload: InitialSetupRequest = {
        orgId: targetOrgId,
        academicYearName: this.calendar.academicYearName || "Academic Year 2026/2027",
        academicYearNameAr: this.calendar.academicYearNameAr || undefined,
        academicYearStart: this.calendar.startDate,
        academicYearEnd: this.calendar.endDate,
        periods,
        classes,
        subjects,
      };

      // 5. Complete core initial setup in backend
      await systemStore.completeInitialSetup(payload);

      // 6. Create Rooms in background (best effort)
      for (const rm of this.facilities.rooms) {
        try {
          await api.createRoom({
            name: rm.name,
            room_type: rm.room_type,
            capacity: rm.capacity,
          });
        } catch (e) {
          console.warn("Room creation note:", e);
        }
      }

      // 7. Clean up local persistence
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
