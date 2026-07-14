/**
 * Organization command interfaces — mirrors svc-organization/ops.
 */
import type { Organization, OrgUnit } from "../types";

export interface OrganizationCommands {
  getMainOrganization(): Promise<Organization>;
  getOrganization(): Promise<Organization>;
  updateOrganization(
    data: Partial<Organization>,
  ): Promise<Organization>;
  getSettings(): Promise<Record<string, unknown>>;
  updateSettings(
    settings: Record<string, unknown>,
  ): Promise<void>;
  getDashboardStats(): Promise<Record<string, unknown>>;
  registerStudent(
    data: {
      name: string;
      email?: string | null;
      phone?: string | null;
      password?: string | null;
      gender?: string | null;
      birth_date?: string | null;
      nationality?: string | null;
      national_id?: string | null;
      address?: string | null;
      class_id?: string | null;
      guardian_name?: string | null;
      guardian_phone?: string | null;
      guardian_relationship?: string | null;
    },
  ): Promise<{
    user_id: string;
    enrollment_id: string;
    student_number: string;
    fees_assigned: number;
    guardian_linked: boolean;
  }>;

  // ── OrgUnits ────────────────────────────────────────────────
  listOrgUnits(): Promise<OrgUnit[]>;
  createOrgUnit(data: {
    name: string;
    name_ar?: string;
    parent_id?: string;
  }): Promise<OrgUnit>;
  updateOrgUnit(
    unitId: string,
    data: { name: string; name_ar?: string },
  ): Promise<OrgUnit>;
  deleteOrgUnit(unitId: string): Promise<void>;
}
