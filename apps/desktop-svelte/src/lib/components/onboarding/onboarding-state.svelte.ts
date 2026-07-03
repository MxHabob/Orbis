// ── Onboarding State Store ───────────────────────────────────────────
// Svelte 5 Runes-based state management for the onboarding wizard.
// Follows the same class-based pattern as auth.svelte.ts.

import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { INSTITUTION_PROFILES } from "./onboarding-templates";
import type {
  Room,
  GradeLevel,
  TimeSlot,
  Subject,
  CycleEntry,
  ClassEntry,
  SubjectEntry,
  OnboardingPayload,
} from "./onboarding-types";

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
  return "YER";
}

// ── Default Data ─────────────────────────────────────────────────────

const DEFAULT_GRADE_LEVELS: GradeLevel[] = [
  {
    level: "kg1",
    name: "KG1",
    name_ar: "روضة أولى",
    color: "#f43f5e",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "kg2",
    name: "KG2",
    name_ar: "روضة ثانية",
    color: "#ec4899",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade1",
    name: "Grade 1",
    name_ar: "الصف الأول",
    color: "#d946ef",
    enabled: true,
    divisions: ["أ", "ب"],
  },
  {
    level: "grade2",
    name: "Grade 2",
    name_ar: "الصف الثاني",
    color: "#a855f7",
    enabled: true,
    divisions: ["أ", "ب"],
  },
  {
    level: "grade3",
    name: "Grade 3",
    name_ar: "الصف الثالث",
    color: "#6366f1",
    enabled: true,
    divisions: ["أ", "ب"],
  },
  {
    level: "grade4",
    name: "Grade 4",
    name_ar: "الصف الرابع",
    color: "#3b82f6",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade5",
    name: "Grade 5",
    name_ar: "الصف الخامس",
    color: "#0ea5e9",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade6",
    name: "Grade 6",
    name_ar: "الصف السادس",
    color: "#06b6d4",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade7",
    name: "Grade 7",
    name_ar: "الصف السابع",
    color: "#14b8a6",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade8",
    name: "Grade 8",
    name_ar: "الصف الثامن",
    color: "#10b981",
    enabled: true,
    divisions: ["أ"],
  },
  {
    level: "grade9",
    name: "Grade 9",
    name_ar: "الصف التاسع",
    color: "#22c55e",
    enabled: true,
    divisions: ["أ"],
  },
];

const DEFAULT_TIME_SLOTS: TimeSlot[] = [
  {
    name: "1st Period",
    name_ar: "الحصة الأولى",
    start_time: "08:00",
    end_time: "08:45",
  },
  {
    name: "2nd Period",
    name_ar: "الحصة الثانية",
    start_time: "08:45",
    end_time: "09:30",
  },
  {
    name: "3rd Period",
    name_ar: "الحصة الثالثة",
    start_time: "09:45",
    end_time: "10:30",
  },
  {
    name: "4th Period",
    name_ar: "الحصة الرابعة",
    start_time: "10:30",
    end_time: "11:15",
  },
  {
    name: "5th Period",
    name_ar: "الحصة الخامسة",
    start_time: "11:30",
    end_time: "12:15",
  },
];

const DEFAULT_SUBJECTS: Subject[] = [
  {
    code: "QUR",
    name: "Holy Quran",
    name_ar: "القرآن الكريم",
    color: "#10b981",
    icon: "book-open",
    sort_order: 1,
    enabled: true,
  },
  {
    code: "ISL",
    name: "Islamic Studies",
    name_ar: "الدراسات الإسلامية",
    color: "#0d9488",
    icon: "bookmark",
    sort_order: 2,
    enabled: true,
  },
  {
    code: "ARB",
    name: "Arabic Language",
    name_ar: "اللغة العربية",
    color: "#f59e0b",
    icon: "pen-tool",
    sort_order: 3,
    enabled: true,
  },
  {
    code: "MTH",
    name: "Mathematics",
    name_ar: "الرياضيات",
    color: "#3b82f6",
    icon: "binary",
    sort_order: 4,
    enabled: true,
  },
  {
    code: "SCI",
    name: "Science",
    name_ar: "العلوم",
    color: "#8b5cf6",
    icon: "flask-conical",
    sort_order: 5,
    enabled: true,
  },
  {
    code: "ENG",
    name: "English Language",
    name_ar: "اللغة الإنجليزية",
    color: "#6366f1",
    icon: "languages",
    sort_order: 6,
    enabled: true,
  },
  {
    code: "SOC",
    name: "Social Studies",
    name_ar: "الدراسات الاجتماعية",
    color: "#f97316",
    icon: "globe",
    sort_order: 7,
    enabled: true,
  },
];

const BUILDINGS = ["المبنى 1", "المبنى 2"];

// ── Store Class ──────────────────────────────────────────────────────

class OnboardingState {
  // Wizard navigation
  activeStep = $state(1);
  isLoading = $state(false);
  errorMessage = $state<string | null>(null);
  totalSteps = 8;
  completedSteps = $state([false, false, false, false, false, false, false, false]);

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
  semester1Name = $state("Semester 1");
  semester1NameAr = $state("الفصل الدراسي الأول");
  semester1Start = $state("2026-06-21");
  semester1End = $state("2026-10-21");
  semester2Name = $state("Semester 2");
  semester2NameAr = $state("الفصل الدراسي الثاني");
  semester2Start = $state("2026-10-22");
  semester2End = $state("2027-02-21");

  // Step 3: Buildings & Rooms
  rooms = $state<Room[]>([]);
  activeBuildingTab = $state("المبنى 1");
  buildings = BUILDINGS;

  // Step 4: Grade Levels & Classes
  gradeLevels = $state<GradeLevel[]>(structuredClone(DEFAULT_GRADE_LEVELS));
  divisionInput = $state<Record<string, string>>({});

  // Step 5: Time Slots
  timeSlots = $state<TimeSlot[]>(structuredClone(DEFAULT_TIME_SLOTS));

  // Step 6: Subjects
  subjects = $state<Subject[]>(structuredClone(DEFAULT_SUBJECTS));

  // ── Centralized Terminology Mappings ──────────────────────────────
  terminology = $derived(
    INSTITUTION_PROFILES[this.institutionType]?.terminology || INSTITUTION_PROFILES.school.terminology
  );

  stageLabel = $derived(getLocale() === "ar" ? this.terminology.stage_ar : this.terminology.stage);
  classroomLabel = $derived(getLocale() === "ar" ? this.terminology.classroom_ar : this.terminology.classroom);
  divisionLabel = $derived(getLocale() === "ar" ? this.terminology.division_ar : this.terminology.division);
  subjectLabel = $derived(getLocale() === "ar" ? this.terminology.subject_ar : this.terminology.subject);
  calendarLabel = $derived(getLocale() === "ar" ? this.terminology.calendar_ar : this.terminology.calendar);
  localizedLanguageName = $derived(this.locale.startsWith("ar") ? m.onboarding_lang_ar() : m.onboarding_lang_en());
  classroomsLabelPlural = $derived(getLocale() === "ar" ? this.terminology.classroom_ar : this.terminology.classroom + "s");
  stagesLabelPlural = $derived(getLocale() === "ar" ? this.terminology.stage_ar : this.terminology.stage + "s");

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
      this.gradeLevels = profile.stages.map(s => ({
        level: s.level,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        enabled: true,
        divisions: [...s.divisions]
      }));
      
      // Default time slots
      this.timeSlots = profile.timeSlots.map(t => ({
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time,
        end_time: t.end_time
      }));
      
      // Default subjects
      this.subjects = profile.subjects.map(s => ({
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        icon: s.icon,
        sort_order: s.sort_order,
        enabled: true
      }));

      // Prepopulate default fees to 0
      const fees: Record<string, number> = {};
      profile.stages.forEach(s => {
        fees[s.level] = 0;
      });
      this.registrationFees = fees;

      // Dynamic room list prefix mapping
      const roomsList: Room[] = [];
      const bld1Name = getLocale() === "ar" ? "المبنى 1" : "Building 1";
      const bld2Name = getLocale() === "ar" ? "المبنى 2" : "Building 2";
      
      profile.stages.slice(0, 5).forEach((stg, sIdx) => {
        roomsList.push({
          id: `room-b1-f1-${sIdx + 1}`,
          name: `${profile.terminology.classroom} 1-0${sIdx + 1}`,
          name_ar: `${profile.terminology.classroom_ar} 1-0${sIdx + 1}`,
          capacity: 30,
          room_type: "classroom",
          building: bld1Name,
          floor: 1,
        });
      });
      this.rooms = roomsList;
      this.buildings = [bld1Name, bld2Name];
      this.activeBuildingTab = bld1Name;
      
      this.save();
    }
  }

  // ── Derived Values ───────────────────────────────────────────────

  progressValue = $derived((this.activeStep / this.totalSteps) * 100);
  totalCapacity = $derived(this.rooms.reduce((acc, r) => acc + r.capacity, 0));
  enabledStages = $derived(this.gradeLevels.filter((l) => l.enabled).length);
  totalDivisions = $derived(
    this.gradeLevels
      .filter((l) => l.enabled)
      .reduce((acc, l) => acc + l.divisions.length, 0),
  );
  enabledSubjects = $derived(this.subjects.filter((s) => s.enabled).length);
  totalSlots = $derived(this.timeSlots.length);
  bld1Rooms = $derived(
    this.rooms.filter((r) => r.building === (getLocale() === "ar" ? "المبنى 1" : "Building 1")).length,
  );
  bld2Rooms = $derived(
    this.rooms.filter((r) => r.building === (getLocale() === "ar" ? "المبنى 2" : "Building 2")).length,
  );
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
        semester1Name: this.semester1Name,
        semester1NameAr: this.semester1NameAr,
        semester1Start: this.semester1Start,
        semester1End: this.semester1End,
        semester2Name: this.semester2Name,
        semester2NameAr: this.semester2NameAr,
        semester2Start: this.semester2Start,
        semester2End: this.semester2End,
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
        if (data.semester1Name) this.semester1Name = data.semester1Name;
        if (data.semester1NameAr) this.semester1NameAr = data.semester1NameAr;
        if (data.semester1Start) this.semester1Start = data.semester1Start;
        if (data.semester1End) this.semester1End = data.semester1End;
        if (data.semester2Name) this.semester2Name = data.semester2Name;
        if (data.semester2NameAr) this.semester2NameAr = data.semester2NameAr;
        if (data.semester2Start) this.semester2Start = data.semester2Start;
        if (data.semester2End) this.semester2End = data.semester2End;
        if (data.timeSlots) this.timeSlots = data.timeSlots;
        if (data.subjects) this.subjects = data.subjects;
        if (data.rooms) this.rooms = data.rooms;
        if (data.institutionType) this.institutionType = data.institutionType;
        if (data.studentNumberPattern) this.studentNumberPattern = data.studentNumberPattern;
        if (data.activeFields) this.activeFields = data.activeFields;
        if (data.requiredFields) this.requiredFields = data.requiredFields;
        if (data.preventDuplicates) this.preventDuplicates = data.preventDuplicates;
        if (data.requiredDocumentsList) this.requiredDocumentsList = data.requiredDocumentsList;
        if (data.gradeLevels) this.gradeLevels = data.gradeLevels;
      }
    } catch (e) {
      console.error("Failed to load onboarding state:", e);
    }
  }

  async loadExistingOrg(orgId: string) {
    try {
      const { commands } = await import("$lib/services/ipc");
      const org = await commands.users.getOrganization(orgId);
      if (org) {
        this.orgName = org.name;
        this.orgNameAr = org.name_ar || org.name;
        this.rootUnitName = org.name;
        this.rootUnitNameAr = org.name_ar || org.name;
        this.timezone = org.timezone || detectTimezone();
        this.locale = org.locale || detectLocale();
        this.currency = org.currency || detectCurrency(this.timezone);
        this.save();
      }
    } catch (e) {
      console.error("Failed to load existing organization:", e);
    }
  }

  // ── Navigation ─────────────────────────────────────────────────

  goNext() {
    if (this.activeStep < this.totalSteps) {
      this.completedSteps[this.activeStep - 1] = true;
      this.activeStep += 1;
      this.save();
    }
  }

  goPrev() {
    if (this.activeStep > 1) {
      this.activeStep -= 1;
      this.save();
    }
  }

  goToStep(step: number) {
    if (!this.isLoading && step >= 1 && step <= this.totalSteps) {
      this.activeStep = step;
      this.save();
    }
  }

  // ── Step 3: Room Management ────────────────────────────────────

  resetRoomsToDefault() {
    const list: Room[] = [];
    // Building 1 Classrooms
    for (let i = 1; i <= 10; i++) {
      list.push({
        id: `room-b1-f1-${i}`,
        name: `Classroom 1-0${i}`,
        name_ar: `فصل 1-0${i}`,
        capacity: 30,
        room_type: "classroom",
        building: "المبنى 1",
        floor: 1,
      });
    }
    for (let i = 11; i <= 20; i++) {
      list.push({
        id: `room-b1-f2-${i}`,
        name: `Classroom 1-${i}`,
        name_ar: `فصل 1-${i}`,
        capacity: 30,
        room_type: "classroom",
        building: "المبنى 1",
        floor: 2,
      });
    }
    // Building 1 Computer labs
    for (let i = 1; i <= 3; i++) {
      list.push({
        id: `room-b1-lab-comp-${i}`,
        name: `Computer Lab 1-${i}`,
        name_ar: `مختبر الحاسوب 1-${i}`,
        capacity: 25,
        room_type: "computer_lab",
        building: "المبنى 1",
        floor: 2,
      });
    }
    // Building 1 Chemistry labs
    for (let i = 1; i <= 2; i++) {
      list.push({
        id: `room-b1-lab-chem-${i}`,
        name: `Chemistry Lab 1-${i}`,
        name_ar: `مختبر الكيمياء 1-${i}`,
        capacity: 25,
        room_type: "chemistry_lab",
        building: "المبنى 1",
        floor: 1,
      });
    }
    // Building 2
    for (let floor = 1; floor <= 3; floor++) {
      for (let room = 1; room <= 7; room++) {
        const roomNum = `${floor}-0${room}`;
        list.push({
          id: `room-b2-f${floor}-${room}`,
          name: `Classroom 2-${roomNum}`,
          name_ar: `فصل 2-${roomNum}`,
          capacity: 30,
          room_type: "classroom",
          building: "المبنى 2",
          floor,
        });
      }
    }
    this.rooms = list;
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

  // ── Cycle Generation Helpers ───────────────────────────────────

  private generateMonthsForSemester(
    parentCycleId: string,
    namePrefix: string,
    namePrefixAr: string,
    startStr: string,
    endStr: string,
  ): CycleEntry[] {
    const start = new Date(startStr);
    const end = new Date(endStr);
    const totalDays = Math.ceil(
      (end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24),
    );
    const daysPerMonth = Math.floor(totalDays / 4);

    const months: CycleEntry[] = [];
    const monthNamesAr = ["الأول", "الثاني", "الثالث", "الرابع (النهائي)"];
    const monthNamesEn = ["Month 1", "Month 2", "Month 3", "Month 4 (Final)"];

    let currentStart = new Date(start);
    for (let i = 0; i < 4; i++) {
      const mStart = new Date(currentStart);
      const mEnd = new Date(currentStart);
      if (i === 3) {
        mEnd.setTime(end.getTime());
      } else {
        mEnd.setDate(mEnd.getDate() + daysPerMonth - 1);
      }

      months.push({
        id: `${parentCycleId}-m${i + 1}`,
        parent_id: parentCycleId,
        name: `${namePrefix} - ${monthNamesEn[i]}`,
        name_ar: `${namePrefixAr} - الشهر ${monthNamesAr[i]}`,
        start_date: mStart.toISOString().split("T")[0],
        end_date: mEnd.toISOString().split("T")[0],
        cycle_type: "month",
        weight: i === 3 ? 30 : 20,
      });

      currentStart = new Date(mEnd);
      currentStart.setDate(currentStart.getDate() + 1);
    }
    return months;
  }

  // ── Build Final Payload ────────────────────────────────────────

  buildPayload(orgId: string): OnboardingPayload {
    const cycleYearId = `cycle-1447-${orgId}`;

    const cycles: CycleEntry[] = [
      {
        id: cycleYearId,
        parent_id: null,
        name: this.academicYearName,
        name_ar: this.academicYearNameAr,
        start_date: this.startDate,
        end_date: this.endDate,
        cycle_type: "year",
        weight: null,
      },
      {
        id: `${cycleYearId}-s1`,
        parent_id: cycleYearId,
        name: this.semester1Name,
        name_ar: this.semester1NameAr,
        start_date: this.semester1Start,
        end_date: this.semester1End,
        cycle_type: "term",
        weight: 50,
      },
      {
        id: `${cycleYearId}-s2`,
        parent_id: cycleYearId,
        name: this.semester2Name,
        name_ar: this.semester2NameAr,
        start_date: this.semester2Start,
        end_date: this.semester2End,
        cycle_type: "term",
        weight: 50,
      },
      ...this.generateMonthsForSemester(
        `${cycleYearId}-s1`,
        this.semester1Name,
        this.semester1NameAr,
        this.semester1Start,
        this.semester1End,
      ),
      ...this.generateMonthsForSemester(
        `${cycleYearId}-s2`,
        this.semester2Name,
        this.semester2NameAr,
        this.semester2Start,
        this.semester2End,
      ),
    ];

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
      cycles,
      time_slots: this.timeSlots,
      rooms: this.rooms.map((r) => ({
        ...r,
        id: `room-${r.id}-${orgId}`,
      })),
      subjects: activeSubjects,
      classes,
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

      const { commands } = await import("$lib/services/ipc");
      await commands.academics.initializeSchoolSetup(orgId, payload);

      // Save registration configuration to organization settings
      const settingsObj = {
        registration_config: {
          institution_type: this.institutionType,
          student_number_pattern: this.studentNumberPattern,
          academic_stages: this.gradeLevels.filter(gl => gl.enabled).map(gl => gl.level),
          required_fields: this.requiredFields,
          active_fields: this.activeFields,
          prevent_duplicates: this.preventDuplicates,
          required_documents: this.requiredDocumentsList,
        }
      };

      const org = await commands.users.getOrganization(orgId);
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
        await commands.users.updateOrganization(orgId, updateRequest);
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
            is_mandatory: 1,
            is_active: 1,
          };
          try {
            await commands.finance.createFeeTemplate(orgId, req);
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
      description: locale === "ar" ? profile.description_ar : profile.description
    };
  }
}

export const onboardingState = new OnboardingState();
