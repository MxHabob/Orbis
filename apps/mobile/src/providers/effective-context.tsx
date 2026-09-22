/**
 * @module @orbis/mobile/providers/effective-context
 *
 * High-Performance, Declarative Effective Context Provider for Orbis Mobile.
 *
 * Architectural Principles:
 * 1. Zero `useEffect` — state is derived purely and declaratively from the authoritative
 *    backend IAM session (`authCtx`), eliminating cascading renders and state lag.
 * 2. Dynamic RBAC / ABAC — role and permission verification is evaluated dynamically
 *    via `hasRole`, `hasAnyRole`, `hasPermission`, and `can`, avoiding hardcoded role assumptions.
 * 3. Reactive Context Scopes — instantaneous resolution of active period, dependents,
 *    and academic targets with persistent preferences.
 */

import React, {
  createContext,
  useContext,
  useState,
  useMemo,
  useCallback,
} from "react";
import { useAuth } from "@/lib/api";
import { setSecureItem } from "@/lib/api/storage";
import type { LinkedDependentDto } from "@/lib/api/types";

export type { LinkedDependentDto };

export interface EffectiveContextType {
  // ── Authoritative IAM / RBAC Matrix ──────────────────────────────
  effectiveRole: string;
  roles: string[];
  permissions: string[];
  hasRole: (role: string) => boolean;
  hasAnyRole: (roles: string[]) => boolean;
  hasPermission: (permission: string) => boolean;
  hasAnyPermission: (permissions: string[]) => boolean;
  hasAllPermissions: (permissions: string[]) => boolean;
  can: (action: string, domain?: string) => boolean;

  // ── Academic Scope ───────────────────────────────────────────────
  activePeriodId: string | null;
  setActivePeriodId: (periodId: string | null) => void;

  // ── Dependent / Student Scope ────────────────────────────────────
  dependents: LinkedDependentDto[];
  selectedDependentId: string | null;
  selectedDependent: LinkedDependentDto | null;
  setSelectedDependentId: (id: string | null) => void;
  targetStudentId: string | null;

  // ── Instructional / Class Scope ──────────────────────────────────
  selectedClassId: string | null;
  setSelectedClassId: (id: string | null) => void;
  selectedSubjectId: string | null;
  setSelectedSubjectId: (id: string | null) => void;
}

const EffectiveContext = createContext<EffectiveContextType | undefined>(
  undefined,
);

const STORAGE_KEYS_CONTEXT = {
  SELECTED_DEPENDENT_ID: "orbis_selected_dependent_id",
  SELECTED_CLASS_ID: "orbis_selected_class_id",
  SELECTED_SUBJECT_ID: "orbis_selected_subject_id",
} as const;

export function EffectiveContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { user, effectiveContext: authCtx } = useAuth();

  // Authoritative data streams derived directly from session state
  const effectiveRole = useMemo<string>(
    () => authCtx?.effective_role ?? "guest",
    [authCtx?.effective_role],
  );

  const roles = useMemo<string[]>(
    () => authCtx?.roles ?? [],
    [authCtx?.roles],
  );

  const permissions = useMemo<string[]>(
    () => authCtx?.permissions ?? [],
    [authCtx?.permissions],
  );

  const dependents = useMemo<LinkedDependentDto[]>(
    () => authCtx?.dependents ?? [],
    [authCtx?.dependents],
  );

  // User-selected operational scopes (declarative initial states without useEffect)
  const [activePeriodOverride, setActivePeriodOverride] = useState<string | null>(null);
  const [selectedDependentIdState, setSelectedDependentIdState] = useState<string | null>(null);
  const [selectedClassId, setSelectedClassIdState] = useState<string | null>(null);
  const [selectedSubjectId, setSelectedSubjectIdState] = useState<string | null>(null);

  // Active period resolution (override or backend default)
  const activePeriodId = useMemo<string | null>(
    () => activePeriodOverride ?? authCtx?.active_period_id ?? null,
    [activePeriodOverride, authCtx?.active_period_id],
  );

  // Active dependent resolution (pure derivation)
  const selectedDependent = useMemo<LinkedDependentDto | null>(() => {
    if (dependents.length === 0) return null;
    if (selectedDependentIdState) {
      const match = dependents.find((d) => d.student_id === selectedDependentIdState);
      if (match) return match;
    }
    return dependents[0] ?? null;
  }, [dependents, selectedDependentIdState]);

  const selectedDependentId = useMemo<string | null>(
    () => selectedDependent?.student_id ?? null,
    [selectedDependent],
  );

  // Target student identifier for academic & student-specific queries
  const targetStudentId = useMemo<string | null>(() => {
    if (dependents.length > 0) {
      return selectedDependent?.student_id ?? null;
    }
    return user?.id ?? null;
  }, [dependents, selectedDependent, user?.id]);

  // Setters with persistent caching
  const setSelectedDependentId = useCallback((id: string | null) => {
    setSelectedDependentIdState(id);
    if (id) {
      setSecureItem(STORAGE_KEYS_CONTEXT.SELECTED_DEPENDENT_ID, id).catch(() => {});
    }
  }, []);

  const setSelectedClassId = useCallback((id: string | null) => {
    setSelectedClassIdState(id);
    if (id) {
      setSecureItem(STORAGE_KEYS_CONTEXT.SELECTED_CLASS_ID, id).catch(() => {});
    }
  }, []);

  const setSelectedSubjectId = useCallback((id: string | null) => {
    setSelectedSubjectIdState(id);
    if (id) {
      setSecureItem(STORAGE_KEYS_CONTEXT.SELECTED_SUBJECT_ID, id).catch(() => {});
    }
  }, []);

  // Dynamic IAM evaluation engines
  const hasRole = useCallback(
    (roleToCheck: string): boolean => {
      const normalized = roleToCheck.toLowerCase();
      return (
        effectiveRole.toLowerCase() === normalized ||
        roles.some((r) => r.toLowerCase() === normalized)
      );
    },
    [effectiveRole, roles],
  );

  const hasAnyRole = useCallback(
    (rolesToCheck: string[]): boolean => {
      const lowercased = rolesToCheck.map((r) => r.toLowerCase());
      return (
        lowercased.includes(effectiveRole.toLowerCase()) ||
        roles.some((r) => lowercased.includes(r.toLowerCase()))
      );
    },
    [effectiveRole, roles],
  );

  const hasPermission = useCallback(
    (permission: string): boolean => {
      if (permissions.length === 0) return false;
      if (permissions.includes("*") || permissions.includes("admin")) return true;
      return permissions.includes(permission);
    },
    [permissions],
  );

  const hasAnyPermission = useCallback(
    (required: string[]): boolean => {
      if (permissions.length === 0) return false;
      if (permissions.includes("*") || permissions.includes("admin")) return true;
      return required.some((p) => permissions.includes(p));
    },
    [permissions],
  );

  const hasAllPermissions = useCallback(
    (required: string[]): boolean => {
      if (permissions.length === 0) return false;
      if (permissions.includes("*") || permissions.includes("admin")) return true;
      return required.every((p) => permissions.includes(p));
    },
    [permissions],
  );

  const can = useCallback(
    (action: string, domain?: string): boolean => {
      const key = domain ? `${domain}:${action}` : action;
      return hasPermission(key);
    },
    [hasPermission],
  );

  const contextValue = useMemo<EffectiveContextType>(
    () => ({
      effectiveRole,
      roles,
      permissions,
      hasRole,
      hasAnyRole,
      hasPermission,
      hasAnyPermission,
      hasAllPermissions,
      can,
      activePeriodId,
      setActivePeriodId: setActivePeriodOverride,
      dependents,
      selectedDependentId,
      selectedDependent,
      setSelectedDependentId,
      selectedClassId,
      setSelectedClassId,
      selectedSubjectId,
      setSelectedSubjectId,
      targetStudentId,
    }),
    [
      effectiveRole,
      roles,
      permissions,
      hasRole,
      hasAnyRole,
      hasPermission,
      hasAnyPermission,
      hasAllPermissions,
      can,
      activePeriodId,
      dependents,
      selectedDependentId,
      selectedDependent,
      setSelectedDependentId,
      selectedClassId,
      setSelectedClassId,
      selectedSubjectId,
      setSelectedSubjectId,
      targetStudentId,
    ],
  );

  return (
    <EffectiveContext.Provider value={contextValue}>
      {children}
    </EffectiveContext.Provider>
  );
}

export function useEffectiveContext(): EffectiveContextType {
  const ctx = useContext(EffectiveContext);
  if (!ctx) {
    throw new Error(
      "useEffectiveContext must be used within an EffectiveContextProvider",
    );
  }
  return ctx;
}
