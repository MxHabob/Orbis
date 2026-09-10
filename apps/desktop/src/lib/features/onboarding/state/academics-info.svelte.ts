import type { GradeLevel, TimeSlot, Subject } from "../types";

export class AcademicsInfoState {
  gradeLevels = $state<GradeLevel[]>([]);
  divisionInput = $state<Record<string, string>>({});
  workingDays = $state<number[]>([1, 2, 3, 4, 5]);
  timeSlots = $state<TimeSlot[]>([]);
  subjects = $state<Subject[]>([]);

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
    if (data.gradeLevels) {
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
    }
    if (data.divisionInput) this.divisionInput = data.divisionInput;
    if (data.workingDays) this.workingDays = data.workingDays;
    if (data.timeSlots) {
      this.timeSlots = data.timeSlots.map((slot: TimeSlot) => ({
        ...slot,
        is_break: slot.is_break ?? false,
        applicable_days: [...(slot.applicable_days ?? this.workingDays)],
      }));
    }
    if (data.subjects) this.subjects = data.subjects;
  }
}
