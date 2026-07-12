// ── Onboarding Types ─────────────────────────────────────────────────
// Shared TypeScript interfaces for the onboarding wizard.

export interface Room {
  id: string;
  name: string;
  name_ar: string;
  capacity: number;
  room_type: string;
  building: string;
  floor: number;
}

export interface GradeLevel {
  level: string;
  name: string;
  name_ar: string;
  color: string;
  enabled: boolean;
  divisions: string[];
}

export interface TimeSlot {
  name: string;
  name_ar: string;
  start_time: string;
  end_time: string;
}

export interface Subject {
  code: string;
  name: string;
  name_ar: string;
  color: string;
  icon: string;
  sort_order: number;
  enabled: boolean;
}

export interface OnboardingStep {
  id: number;
  labelKey: string;
  labelEn: string;
  icon: any;
}

export interface PeriodEntry {
  id: string;
  parent_id: string | null;
  name: string;
  name_ar: string;
  start_date: string;
  end_date: string;
  period_type: string;
  weight: number | null;
}

export interface ClassEntry {
  id: string;
  name: string;
  name_ar: string;
  level: string;
  division: string;
  color: string;
}

export interface SubjectEntry {
  id: string;
  code: string;
  name: string;
  name_ar: string;
  color: string;
  icon: string;
  sort_order: number;
}

export interface OnboardingPayload {
  org_name: string;
  org_name_ar: string;
  timezone: string;
  locale: string;
  currency: string;
  root_unit_id: string;
  root_unit_name: string;
  root_unit_name_ar: string;
  periods: PeriodEntry[];
  time_slots: TimeSlot[];
  rooms: Room[];
  subjects: SubjectEntry[];
  classes: ClassEntry[];
}
