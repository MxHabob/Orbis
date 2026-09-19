import type { GradeLevel, TimeSlot, Subject } from "../types";
import { INSTITUTION_PROFILES } from "../templates";

export function createDefaultGradeLevels(type = "school"): GradeLevel[] {
  const prof = INSTITUTION_PROFILES[type] || INSTITUTION_PROFILES["school"];
  return prof.stages.map((stage) => ({
    level: stage.level,
    name: stage.name,
    name_ar: stage.name_ar,
    color: stage.color,
    enabled: true,
    divisions: [...stage.divisions],
  }));
}

export function createDefaultTimeSlots(type = "school"): TimeSlot[] {
  const prof = INSTITUTION_PROFILES[type] || INSTITUTION_PROFILES["school"];
  return prof.timeSlots.map((slot) => ({
    name: slot.name,
    name_ar: slot.name_ar,
    start_time: slot.start_time,
    end_time: slot.end_time,
    is_break: slot.is_break ?? false,
    break_type: slot.break_type,
    applicable_days: slot.applicable_days ? [...slot.applicable_days] : [1, 2, 3, 4, 5],
  }));
}

export function createDefaultSubjects(type = "school"): Subject[] {
  const prof = INSTITUTION_PROFILES[type] || INSTITUTION_PROFILES["school"];
  return prof.subjects.map((sub) => ({
    code: sub.code,
    name: sub.name,
    name_ar: sub.name_ar,
    color: sub.color,
    icon: sub.icon,
    sort_order: sub.sort_order,
    enabled: sub.enabled !== undefined ? sub.enabled : true,
  }));
}

export class AcademicsInfoState {
  gradeLevels = $state<GradeLevel[]>(createDefaultGradeLevels("school"));
  divisionInput = $state<Record<string, string>>({});
  workingDays = $state<number[]>([1, 2, 3, 4, 5]);
  timeSlots = $state<TimeSlot[]>(createDefaultTimeSlots("school"));
  subjects = $state<Subject[]>(createDefaultSubjects("school"));

  get enabledStages() {
    return this.gradeLevels.filter((l) => l.enabled).length;
  }

  get totalDivisions() {
    return this.gradeLevels
      .filter((l) => l.enabled)
      .reduce((acc, l) => acc + l.divisions.length, 0);
  }

  get enabledSubjects() {
    return this.subjects.filter((s) => s.enabled).length;
  }

  get totalSlots() {
    return this.timeSlots.length;
  }

  serialize() {
    return {
      gradeLevels: this.gradeLevels,
      divisionInput: this.divisionInput,
      workingDays: this.workingDays,
      timeSlots: this.timeSlots,
      subjects: this.subjects,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.gradeLevels && Array.isArray(data.gradeLevels) && data.gradeLevels.length >= 10) {
      const arabicDivisionLetters: Record<string, string> = {
        أ: "A",
        ب: "B",
        ج: "C",
        د: "D",
        هـ: "E",
      };
      this.gradeLevels = data.gradeLevels.map((gradeLevel: GradeLevel) => ({
        ...gradeLevel,
        divisions: gradeLevel.divisions.map(
          (division) => arabicDivisionLetters[division] || division,
        ),
      }));
    } else {
      this.gradeLevels = createDefaultGradeLevels("school");
    }

    if (data.divisionInput) this.divisionInput = data.divisionInput;
    if (data.workingDays) this.workingDays = data.workingDays;

    if (data.timeSlots && Array.isArray(data.timeSlots) && data.timeSlots.length >= 7) {
      this.timeSlots = data.timeSlots.map((slot: TimeSlot) => ({
        ...slot,
        is_break: slot.is_break ?? false,
        applicable_days: [...(slot.applicable_days ?? this.workingDays)],
      }));
    } else {
      this.timeSlots = createDefaultTimeSlots("school");
    }

    if (data.subjects && Array.isArray(data.subjects) && data.subjects.length >= 18) {
      this.subjects = data.subjects;
    } else {
      this.subjects = createDefaultSubjects("school");
    }
  }
}
