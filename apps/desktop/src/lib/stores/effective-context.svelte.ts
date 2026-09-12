/**
 * Effective Context Store — Centralized Context-Aware state for the unified Desktop App.
 *
 * Implements:
 * 1. Active Academic Context (Academic Year + Period)
 * 2. Active Effective Scope (Org / Branch / Class / Teacher Assignment)
 * 3. Permission & Role Checking
 */

import { authStore } from "./auth.svelte";
import { academicContext } from "./academic-context.svelte";
import type {
  UserDto,
  OrgUnitDto,
  EffectiveContextDto,
  LinkedDependentDto,
} from "$lib/api/types";

export type ScopeLevel = "organization" | "branch" | "class" | "self";

export interface ScopeFilter {
  orgId: string;
  orgUnitId?: string | null;
  classId?: string | null;
  periodId?: string | null;
}

class EffectiveContextStore {
  // ── State ───────────────────────────────────────────────────
  activeOrgUnitId = $state<string | null>(
    localStorage.getItem("orbis_active_org_unit_id") || null,
  );
  activeClassId = $state<string | null>(
    localStorage.getItem("orbis_active_class_id") || null,
  );
  effectiveRole = $state<string>("student");
  roles = $state<string[]>([]);
  dependents = $state<LinkedDependentDto[]>([]);
  selectedDependentId = $state<string | null>(
    localStorage.getItem("orbis_selected_dependent_id") || null,
  );
  userPermissions = $state<Set<string>>(new Set());

  // ── Derived ─────────────────────────────────────────────────
  currentUser = $derived<UserDto | null>(authStore.user);
  currentOrgId = $derived<string>(authStore.orgId);
  currentPeriodId = $derived<string>(academicContext.effectivePeriodId);
  currentYearId = $derived<string>(academicContext.currentYear?.id || "");

  selectedDependent = $derived<LinkedDependentDto | null>(
    this.dependents.find((d) => d.student_id === this.selectedDependentId) ||
      this.dependents[0] ||
      null,
  );

  /** Computes the active scope level */
  scopeLevel = $derived.by<ScopeLevel>(() => {
    if (this.activeClassId) return "class";
    if (this.activeOrgUnitId) return "branch";
    return "organization";
  });

  /** Returns full filter parameters for queries based on effective scope */
  effectiveScopeFilter = $derived.by<ScopeFilter>(() => ({
    orgId: this.currentOrgId,
    orgUnitId: this.activeOrgUnitId,
    classId: this.activeClassId,
    periodId: this.currentPeriodId || null,
  }));

  // ── Methods ─────────────────────────────────────────────────
  setEffectiveContext(ctx?: EffectiveContextDto | null) {
    if (!ctx) return;
    if (ctx.effective_role) {
      this.effectiveRole = ctx.effective_role;
    }
    if (ctx.roles) {
      this.roles = ctx.roles;
    }
    if (ctx.permissions) {
      this.setPermissions(ctx.permissions);
    }
    if (ctx.dependents) {
      this.dependents = ctx.dependents;
      if (
        !this.selectedDependentId &&
        ctx.dependents.length > 0 &&
        ctx.dependents[0]?.student_id
      ) {
        this.setSelectedDependent(ctx.dependents[0].student_id);
      }
    }
    if (ctx.active_period_id) {
      academicContext.selectedPeriodId = ctx.active_period_id;
    }
  }

  setSelectedDependent(dependentId: string | null) {
    this.selectedDependentId = dependentId;
    if (dependentId) {
      localStorage.setItem("orbis_selected_dependent_id", dependentId);
    } else {
      localStorage.removeItem("orbis_selected_dependent_id");
    }
  }

  setOrgUnit(unitId: string | null) {
    this.activeOrgUnitId = unitId;
    if (unitId) {
      localStorage.setItem("orbis_active_org_unit_id", unitId);
    } else {
      localStorage.removeItem("orbis_active_org_unit_id");
    }
  }

  setClass(classId: string | null) {
    this.activeClassId = classId;
    if (classId) {
      localStorage.setItem("orbis_active_class_id", classId);
    } else {
      localStorage.removeItem("orbis_active_class_id");
    }
  }

  setPermissions(perms: string[]) {
    this.userPermissions = new Set(perms);
  }

  /** Checks if the user has a specific permission */
  hasPermission(permission: string): boolean {
    if (this.userPermissions.has("*") || this.userPermissions.has("admin")) {
      return true;
    }
    return this.userPermissions.has(permission);
  }

  /** Check if a resource belongs to the current effective scope */
  matchesScope(resource: {
    orgId?: string;
    orgUnitId?: string | null;
    classId?: string | null;
  }): boolean {
    if (resource.orgId && resource.orgId !== this.currentOrgId) {
      return false;
    }
    if (
      this.activeOrgUnitId &&
      resource.orgUnitId &&
      resource.orgUnitId !== this.activeOrgUnitId
    ) {
      return false;
    }
    if (
      this.activeClassId &&
      resource.classId &&
      resource.classId !== this.activeClassId
    ) {
      return false;
    }
    return true;
  }

  reset() {
    this.activeOrgUnitId = null;
    this.activeClassId = null;
    this.effectiveRole = "student";
    this.roles = [];
    this.dependents = [];
    this.selectedDependentId = null;
    this.userPermissions = new Set();
    localStorage.removeItem("orbis_active_org_unit_id");
    localStorage.removeItem("orbis_active_class_id");
    localStorage.removeItem("orbis_selected_dependent_id");
  }
}

export const effectiveContext = new EffectiveContextStore();
