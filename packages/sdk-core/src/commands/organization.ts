/**
 * Organization command interfaces — mirrors svc-organization/ops.
 */
import type { Organization } from "../types";

export interface OrganizationCommands {
  getMainOrganization(): Promise<Organization>;
  getOrganization(orgId: string): Promise<Organization>;
  updateOrganization(
    orgId: string,
    data: Partial<Organization>,
  ): Promise<Organization>;
  getSettings(orgId: string): Promise<Record<string, unknown>>;
  updateSettings(
    orgId: string,
    settings: Record<string, unknown>,
  ): Promise<void>;
  getDashboardStats(orgId: string): Promise<Record<string, unknown>>;
  registerStudent(
    orgId: string,
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
}
