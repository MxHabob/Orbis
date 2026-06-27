// ── Academic Store ───────────────────────────────────────────────────
// Svelte 5 Runes-based store for keeping the selected academic state (cycle, class, subject)
// persistent across route changes.

class AcademicStore {
  private _cycleId = $state<string>(localStorage.getItem("orbis_academic_cycle_id") ?? "");
  private _classId = $state<string>(localStorage.getItem("orbis_academic_class_id") ?? "");
  private _subjectId = $state<string>(localStorage.getItem("orbis_academic_subject_id") ?? "");

  get selectedCycleId(): string {
    return this._cycleId;
  }
  set selectedCycleId(val: string) {
    this._cycleId = val;
    localStorage.setItem("orbis_academic_cycle_id", val);
  }

  get selectedClassId(): string {
    return this._classId;
  }
  set selectedClassId(val: string) {
    this._classId = val;
    localStorage.setItem("orbis_academic_class_id", val);
  }

  get selectedSubjectId(): string {
    return this._subjectId;
  }
  set selectedSubjectId(val: string) {
    this._subjectId = val;
    localStorage.setItem("orbis_academic_subject_id", val);
  }

  reset() {
    this.selectedCycleId = "";
    this.selectedClassId = "";
    this.selectedSubjectId = "";
  }
}

export const academicStore = new AcademicStore();
