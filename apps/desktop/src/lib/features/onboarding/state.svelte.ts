// ── Onboarding State ──────────────────────────────────────────────────
// Streamlined initial setup state for Academic Year, Periods, Classes, and Subjects.

import { systemStore } from "$lib/stores/system.svelte";
import { authStore } from "$lib/stores/auth.svelte";
import type { InitialSetupRequest } from "$lib/api/types";

export interface PeriodItem {
  name: string;
  name_ar?: string;
  period_type: string;
  start_date: string;
  end_date: string;
}

export interface ClassItem {
  name: string;
  name_ar?: string;
  level: string;
  division?: string;
}

export interface SubjectItem {
  code: string;
  name: string;
  name_ar?: string;
  is_core: boolean;
}

class OnboardingState {
  currentStep = $state(0);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // Step 1: Academic Year & Periods
  academicYearName = $state("2026/2027");
  academicYearNameAr = $state("العام الدراسي 2026/2027");
  academicYearStart = $state("2026-09-01");
  academicYearEnd = $state("2027-06-30");

  periods = $state<PeriodItem[]>([
    {
      name: "First Semester",
      name_ar: "الفصل الدراسي الأول",
      period_type: "semester",
      start_date: "2026-09-01",
      end_date: "2027-01-15",
    },
    {
      name: "Second Semester",
      name_ar: "الفصل الدراسي الثاني",
      period_type: "semester",
      start_date: "2027-02-01",
      end_date: "2027-06-30",
    },
  ]);

  // Step 2: Classes
  classes = $state<ClassItem[]>([
    { name: "Grade 1 - A", name_ar: "الصف الأول - أ", level: "Grade 1", division: "A" },
    { name: "Grade 1 - B", name_ar: "الصف الأول - ب", level: "Grade 1", division: "B" },
    { name: "Grade 2 - A", name_ar: "الصف الثاني - أ", level: "Grade 2", division: "A" },
    { name: "Grade 2 - B", name_ar: "الصف الثاني - ب", level: "Grade 2", division: "B" },
  ]);

  // Step 3: Core Subjects
  subjects = $state<SubjectItem[]>([
    { code: "MATH101", name: "Mathematics", name_ar: "الرياضيات", is_core: true },
    { code: "SCI101", name: "Science", name_ar: "العلوم", is_core: true },
    { code: "ARB101", name: "Arabic Language", name_ar: "اللغة العربية", is_core: true },
    { code: "ENG101", name: "English Language", name_ar: "اللغة الإنجليزية", is_core: true },
    { code: "HIS101", name: "Social Studies", name_ar: "الدراسات الاجتماعية", is_core: false },
    { code: "ART101", name: "Art & Design", name_ar: "التربية الفنية", is_core: false },
  ]);

  addPeriod() {
    this.periods.push({
      name: `Period ${this.periods.length + 1}`,
      name_ar: `الفصل ${this.periods.length + 1}`,
      period_type: "semester",
      start_date: this.academicYearStart,
      end_date: this.academicYearEnd,
    });
  }

  removePeriod(index: number) {
    if (this.periods.length > 1) {
      this.periods.splice(index, 1);
    }
  }

  addClass(name: string, level: string, division?: string, name_ar?: string) {
    this.classes.push({ name, level, division, name_ar });
  }

  removeClass(index: number) {
    if (this.classes.length > 1) {
      this.classes.splice(index, 1);
    }
  }

  addSubject(code: string, name: string, name_ar?: string, is_core = true) {
    this.subjects.push({ code, name, name_ar, is_core });
  }

  removeSubject(index: number) {
    if (this.subjects.length > 1) {
      this.subjects.splice(index, 1);
    }
  }

  nextStep() {
    if (this.currentStep < 2) {
      this.currentStep += 1;
    }
  }

  prevStep() {
    if (this.currentStep > 0) {
      this.currentStep -= 1;
    }
  }

  async submit(orgId: string): Promise<boolean> {
    this.isLoading = true;
    this.error = null;
    try {
      const payload: InitialSetupRequest = {
        orgId: orgId || authStore.orgId,
        academicYearName: this.academicYearName,
        academicYearNameAr: this.academicYearNameAr || undefined,
        academicYearStart: this.academicYearStart,
        academicYearEnd: this.academicYearEnd,
        periods: this.periods.map((p) => ({
          name: p.name,
          name_ar: p.name_ar,
          period_type: p.period_type,
          start_date: p.start_date,
          end_date: p.end_date,
        })),
        classes: this.classes.map((c) => ({
          name: c.name,
          name_ar: c.name_ar,
          level: c.level,
          division: c.division,
        })),
        subjects: this.subjects.map((s) => ({
          code: s.code,
          name: s.name,
          name_ar: s.name_ar,
          is_core: s.is_core,
        })),
      };

      await systemStore.completeInitialSetup(payload);
      return true;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      return false;
    } finally {
      this.isLoading = false;
    }
  }
}

export const onboardingState = new OnboardingState();
