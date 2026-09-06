/**
 * Effective Context Store — Centralized Context-Aware state for the unified Desktop App.
 *
 * Implements:
 * 1. Active Academic Context (Academic Year + Period)
 * 2. Active Effective Scope (Org / Branch / Class / Teacher Assignment)
 * 3. Permission & Role Checking
 */

import { authStore } from './auth.svelte';
import { academicContext } from './academic-context.svelte';
import type { UserDto, OrgUnitDto } from '$lib/api/types';

export type ScopeLevel = 'organization' | 'branch' | 'class' | 'self';

export interface ScopeFilter {
  orgId: string;
  orgUnitId?: string | null;
  classId?: string | null;
  periodId?: string | null;
}

class EffectiveContextStore {
  // ── State ───────────────────────────────────────────────────
  activeOrgUnitId = $state<string | null>(
    localStorage.getItem('orbis_active_org_unit_id') || null
  );
  activeClassId = $state<string | null>(
    localStorage.getItem('orbis_active_class_id') || null
  );
  userPermissions = $state<Set<string>>(new Set());

  // ── Derived ─────────────────────────────────────────────────
  currentUser = $derived<UserDto | null>(authStore.user);
  currentOrgId = $derived<string>(authStore.orgId);
  currentPeriodId = $derived<string>(academicContext.effectivePeriodId);
  currentYearId = $derived<string>(academicContext.currentYear?.id || '');

  /** Computes the active scope level */
  scopeLevel = $derived.by<ScopeLevel>(() => {
    if (this.activeClassId) return 'class';
    if (this.activeOrgUnitId) return 'branch';
    return 'organization';
  });

  /** Returns full filter parameters for queries based on effective scope */
  effectiveScopeFilter = $derived.by<ScopeFilter>(() => ({
    orgId: this.currentOrgId,
    orgUnitId: this.activeOrgUnitId,
    classId: this.activeClassId,
    periodId: this.currentPeriodId || null,
  }));

  // ── Methods ─────────────────────────────────────────────────
  setOrgUnit(unitId: string | null) {
    this.activeOrgUnitId = unitId;
    if (unitId) {
      localStorage.setItem('orbis_active_org_unit_id', unitId);
    } else {
      localStorage.removeItem('orbis_active_org_unit_id');
    }
  }

  setClass(classId: string | null) {
    this.activeClassId = classId;
    if (classId) {
      localStorage.setItem('orbis_active_class_id', classId);
    } else {
      localStorage.removeItem('orbis_active_class_id');
    }
  }

  setPermissions(perms: string[]) {
    this.userPermissions = new Set(perms);
  }

  /** Checks if the user has a specific permission */
  hasPermission(permission: string): boolean {
    if (this.userPermissions.has('*') || this.userPermissions.has('admin')) {
      return true;
    }
    return this.userPermissions.has(permission);
  }

  /** Check if a resource belongs to the current effective scope */
  matchesScope(resource: { orgId?: string; orgUnitId?: string | null; classId?: string | null }): boolean {
    if (resource.orgId && resource.orgId !== this.currentOrgId) {
      return false;
    }
    if (this.activeOrgUnitId && resource.orgUnitId && resource.orgUnitId !== this.activeOrgUnitId) {
      return false;
    }
    if (this.activeClassId && resource.classId && resource.classId !== this.activeClassId) {
      return false;
    }
    return true;
  }

  reset() {
    this.activeOrgUnitId = null;
    this.activeClassId = null;
    this.userPermissions = new Set();
    localStorage.removeItem('orbis_active_org_unit_id');
    localStorage.removeItem('orbis_active_class_id');
  }
}

export const effectiveContext = new EffectiveContextStore();
