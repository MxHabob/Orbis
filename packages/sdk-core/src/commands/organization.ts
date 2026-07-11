/**
 * Organization command interfaces — mirrors svc-organization/ops.
 */
import type { Organization } from "../types";

export interface OrganizationCommands {
  getOrganization(orgId: string): Promise<Organization>;
  updateOrganization(orgId: string, data: Partial<Organization>): Promise<Organization>;
  getSettings(orgId: string): Promise<Record<string, unknown>>;
  updateSettings(orgId: string, settings: Record<string, unknown>): Promise<void>;
  getDashboardStats(orgId: string): Promise<Record<string, unknown>>;
}
