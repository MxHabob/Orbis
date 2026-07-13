import type { GradeLevel, TimeSlot, Subject } from "../types";

export class AcademicsInfoState {
  gradeLevels = $state<GradeLevel[]>([]);
  divisionInput = $state<Record<string, string>>({});
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
      timeSlots: this.timeSlots,
      subjects: this.subjects,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.gradeLevels) this.gradeLevels = data.gradeLevels;
    if (data.divisionInput) this.divisionInput = data.divisionInput;
    if (data.timeSlots) this.timeSlots = data.timeSlots;
    if (data.subjects) this.subjects = data.subjects;
  }
}
