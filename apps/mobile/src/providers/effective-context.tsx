/**
 * @module @orbis/mobile/providers/effective-context
 *
 * High-Performance, Declarative Effective Context Provider for Orbis Mobile.
 *
 * Architectural Principles:
 * 1. Effective permissions and scopes are derived from the authoritative backend session;
 *    only user-selected scope preferences are restored asynchronously.
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
  useEffect,
} from "react";
import { useAuth } from "@/lib/api";
import {
  deleteSecureItem,
  getSecureItem,
  setSecureItem,
} from "@/lib/api/storage";
import { LinkedDependentDto } from "@orbis/sdk";

export interface EffectiveContextType {
  isReady: boolean;
  // ── Authoritative IAM / RBAC Matrix ──────────────────────────────
  effectiveRole: string;
  activeRole: string;
  setActiveRole: (role: string) => void;
  studentId: string | null;
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
  ACTIVE_PERIOD_ID: "orbis_active_period_id",
} as const;

function getScopedStorageKey(key: string, scopeId: string): string {
  return `${key}_${scopeId.replace(/:/g, "_")}`;
}

export function EffectiveContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const {
    effectiveContext: authCtx,
    isInitializing,
    orgId,
    userId,
  } = useAuth();

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
  const [activeRoleOverride, setActiveRoleOverride] = useState<string | null>(
    null,
  );
  const activeRole =
    (activeRoleOverride &&
    (roles.includes(activeRoleOverride) ||
      activeRoleOverride === effectiveRole)
      ? activeRoleOverride
      : null) ??
    (roles.includes(effectiveRole) ? effectiveRole : roles[0]) ??
    "guest";
  const studentId = authCtx?.student_id ?? null;

  const dependents = useMemo<LinkedDependentDto[]>(
    () => authCtx?.dependents ?? [],
    [authCtx?.dependents],
  );

  // Persisted scope choices are restored after the authenticated session resolves.
  const [activePeriodOverride, setActivePeriodOverride] = useState<string | null>(null);
  const [selectedDependentIdState, setSelectedDependentIdState] = useState<string | null>(null);
  const [selectedClassId, setSelectedClassIdState] = useState<string | null>(null);
  const [selectedSubjectId, setSelectedSubjectIdState] = useState<string | null>(null);
  const [loadedScope, setLoadedScope] = useState<string | null>(null);
  const scopeId = userId ? `${orgId}:${userId}` : "guest";
  const isReady = !isInitializing && loadedScope === scopeId;

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

  useEffect(() => {
    if (isInitializing) return;

    let cancelled = false;
    if (!userId) {
      setActiveRoleOverride(null);
      setActivePeriodOverride(null);
      setSelectedDependentIdState(null);
      setSelectedClassIdState(null);
      setSelectedSubjectIdState(null);
      setLoadedScope("guest");
      return () => {
        cancelled = true;
      };
    }

    const restoreScope = async () => {
      const scopedKey = (key: string) => getScopedStorageKey(key, scopeId);
      const [
        savedRole,
        savedPeriod,
        savedDependent,
        savedClass,
        savedSubject,
      ] = await Promise.all([
        getSecureItem(scopedKey("orbis_active_role")),
        getSecureItem(scopedKey(STORAGE_KEYS_CONTEXT.ACTIVE_PERIOD_ID)),
        getSecureItem(scopedKey(STORAGE_KEYS_CONTEXT.SELECTED_DEPENDENT_ID)),
        getSecureItem(scopedKey(STORAGE_KEYS_CONTEXT.SELECTED_CLASS_ID)),
        getSecureItem(scopedKey(STORAGE_KEYS_CONTEXT.SELECTED_SUBJECT_ID)),
      ]);

      if (cancelled) return;

      setActiveRoleOverride(
        savedRole &&
          (roles.includes(savedRole) || savedRole === effectiveRole)
          ? savedRole
          : null,
      );
      setActivePeriodOverride(savedPeriod);
      setSelectedDependentIdState(
        savedDependent &&
          dependents.some((dependent) => dependent.student_id === savedDependent)
          ? savedDependent
          : null,
      );
      const restoredClass =
        savedClass &&
        authCtx?.teaching_classes?.some(
          (classItem) => classItem.class_id === savedClass,
        )
          ? savedClass
          : null;
      setSelectedClassIdState(restoredClass);
      setSelectedSubjectIdState(restoredClass ? savedSubject : null);
      setLoadedScope(scopeId);
    };

    void restoreScope().catch((error: unknown) => {
      console.error(
        "[Orbis Mobile Context] Failed to restore saved scopes.",
        error,
      );
      if (!cancelled) setLoadedScope(scopeId);
    });

    return () => {
      cancelled = true;
    };
  }, [
    authCtx?.teaching_classes,
    dependents,
    effectiveRole,
    isInitializing,
    orgId,
    roles,
    scopeId,
    userId,
  ]);

  const persistScopeValue = useCallback(
    (key: string, value: string | null) => {
      if (!userId) return;

      const keyForUser = getScopedStorageKey(key, scopeId);
      const write = value
        ? setSecureItem(keyForUser, value)
        : deleteSecureItem(keyForUser);
      void write.catch((error: unknown) => {
        console.error(
          "[Orbis Mobile Context] Failed to save selected scope.",
          error,
        );
      });
    },
    [scopeId, userId],
  );

  const setActivePeriodId = useCallback(
    (periodId: string | null) => {
      setActivePeriodOverride(periodId);
      persistScopeValue(STORAGE_KEYS_CONTEXT.ACTIVE_PERIOD_ID, periodId);
    },
    [persistScopeValue],
  );

  const setActiveRole = useCallback(
    (role: string) => {
      if (roles.includes(role) || role === effectiveRole) {
        setActiveRoleOverride(role);
        persistScopeValue("orbis_active_role", role);
      }
    },
    [effectiveRole, persistScopeValue, roles],
  );

  // Target student identifier for academic & student-specific queries
  const targetStudentId = useMemo<string | null>(() => {
    if (["guardian", "parent", "family"].includes(activeRole.toLowerCase())) {
      return selectedDependent?.student_id ?? null;
    }
    return activeRole.toLowerCase().includes("student") ? studentId : null;
  }, [activeRole, selectedDependent, studentId]);

  // Setters with persistent caching
  const setSelectedDependentId = useCallback(
    (id: string | null) => {
      const validId =
        id && dependents.some((dependent) => dependent.student_id === id)
          ? id
          : null;
      setSelectedDependentIdState(validId);
      persistScopeValue(STORAGE_KEYS_CONTEXT.SELECTED_DEPENDENT_ID, validId);
    },
    [dependents, persistScopeValue],
  );

  const setSelectedClassId = useCallback(
    (id: string | null) => {
      const validId =
        id &&
        authCtx?.teaching_classes?.some(
          (classItem) => classItem.class_id === id,
        )
          ? id
          : null;
      setSelectedClassIdState(validId);
      persistScopeValue(STORAGE_KEYS_CONTEXT.SELECTED_CLASS_ID, validId);
      if (validId !== selectedClassId) {
        setSelectedSubjectIdState(null);
        persistScopeValue(STORAGE_KEYS_CONTEXT.SELECTED_SUBJECT_ID, null);
      }
    },
    [authCtx?.teaching_classes, persistScopeValue, selectedClassId],
  );

  const setSelectedSubjectId = useCallback(
    (id: string | null) => {
      setSelectedSubjectIdState(id);
      persistScopeValue(STORAGE_KEYS_CONTEXT.SELECTED_SUBJECT_ID, id);
    },
    [persistScopeValue],
  );

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
      isReady,
      effectiveRole,
      activeRole,
      setActiveRole,
      studentId,
      roles,
      permissions,
      hasRole,
      hasAnyRole,
      hasPermission,
      hasAnyPermission,
      hasAllPermissions,
      can,
      activePeriodId,
      setActivePeriodId,
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
      isReady,
      effectiveRole,
      activeRole,
      setActiveRole,
      studentId,
      roles,
      permissions,
      hasRole,
      hasAnyRole,
      hasPermission,
      hasAnyPermission,
      hasAllPermissions,
      can,
      activePeriodId,
      setActivePeriodId,
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
