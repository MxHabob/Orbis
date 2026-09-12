/**
 * @module @orbis/mobile/providers/effective-context
 *
 * Provides shared context for the authenticated user's active role,
 * selected dependent (child for parent), active class (for teacher),
 * and current academic period.
 */

import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  useMemo,
  useCallback,
} from "react";
import { useAuth } from "@/lib/api";
import { getSecureItem, setSecureItem } from "@/lib/api/storage";

export type EffectiveRole = "student" | "parent" | "teacher" | "admin";

export interface LinkedChild {
  id: string;
  name: string;
  studentNumber?: string;
  className?: string;
  gradeLevel?: string;
  avatarUrl?: string;
  isPrimary?: boolean;
  isAuthorizedPickup?: boolean;
  isEmergencyContact?: boolean;
  isPayer?: boolean;
}

export interface EffectiveContextType {
  role: EffectiveRole;
  setRole: (role: EffectiveRole) => void;
  roles: string[];
  permissions: string[];
  hasPermission: (permission: string) => boolean;
  selectedChildId: string | null;
  setSelectedChildId: (id: string | null) => void;
  selectedChild: LinkedChild | null;
  childrenList: LinkedChild[];
  setChildrenList: (list: LinkedChild[]) => void;
  selectedClassId: string | null;
  setSelectedClassId: (id: string | null) => void;
  activePeriodId: string | null;
  setActivePeriodId: (id: string | null) => void;
  isStudent: boolean;
  isParent: boolean;
  isTeacher: boolean;
  isAdmin: boolean;
  targetStudentId: string | null; // Returns current user ID if student, or selectedChildId if parent
}

const EffectiveContext = createContext<EffectiveContextType | undefined>(
  undefined,
);

const STORAGE_KEYS_CONTEXT = {
  SELECTED_CHILD_ID: "orbis_selected_child_id",
  OVERRIDE_ROLE: "orbis_effective_role",
};

export function EffectiveContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { user, effectiveContext: authCtx, isAuthenticated } = useAuth();

  // Derive initial role from effectiveContext or user fallback
  const initialRole = useMemo<EffectiveRole>(() => {
    if (authCtx?.effective_role) {
      const lower = authCtx.effective_role.toLowerCase();
      if (lower.includes("parent") || lower.includes("guardian"))
        return "parent";
      if (
        lower.includes("teacher") ||
        lower.includes("instructor") ||
        lower.includes("lecturer")
      )
        return "teacher";
      if (lower.includes("admin") || lower.includes("principal"))
        return "admin";
      return "student";
    }
    if (!user) return "student";
    const rawRole = (user as any).role || (user as any).user_type;
    if (typeof rawRole === "string") {
      const lower = rawRole.toLowerCase();
      if (lower.includes("parent") || lower.includes("guardian"))
        return "parent";
      if (
        lower.includes("teacher") ||
        lower.includes("instructor") ||
        lower.includes("lecturer")
      )
        return "teacher";
      if (lower.includes("admin") || lower.includes("principal"))
        return "admin";
    }
    return "student";
  }, [user, authCtx]);

  const [role, setRoleState] = useState<EffectiveRole>(initialRole);
  const [selectedChildId, setSelectedChildIdState] = useState<string | null>(
    null,
  );
  const [childrenList, setChildrenList] = useState<LinkedChild[]>([]);
  const [selectedClassId, setSelectedClassId] = useState<string | null>(null);
  const [activePeriodId, setActivePeriodId] = useState<string | null>(null);

  // Sync role and dependents from authCtx or user
  useEffect(() => {
    if (isAuthenticated) {
      if (authCtx?.dependents && authCtx.dependents.length > 0) {
        const mapped: LinkedChild[] = authCtx.dependents.map((d) => ({
          id: d.student_id,
          name:
            d.display_name ||
            `${d.first_name || ""} ${d.last_name || ""}`.trim() ||
            d.student_number,
          studentNumber: d.student_number,
          className: d.current_class_id ?? undefined,
          isPrimary: d.is_primary,
          isAuthorizedPickup: d.is_authorized_pickup,
          isEmergencyContact: d.is_emergency_contact,
          isPayer: d.is_payer,
        }));
        setChildrenList(mapped);
      } else if (user) {
        const userDependents =
          (user as any).dependents ||
          (user as any).children ||
          (user as any).students;
        if (Array.isArray(userDependents) && userDependents.length > 0) {
          setChildrenList(userDependents);
        }
      }

      if (authCtx?.active_period_id) {
        setActivePeriodId(authCtx.active_period_id);
      }

      getSecureItem(STORAGE_KEYS_CONTEXT.OVERRIDE_ROLE).then((storedRole) => {
        if (
          storedRole &&
          ["student", "parent", "teacher", "admin"].includes(storedRole)
        ) {
          setRoleState(storedRole as EffectiveRole);
        } else {
          setRoleState(initialRole);
        }
      });

      getSecureItem(STORAGE_KEYS_CONTEXT.SELECTED_CHILD_ID).then(
        (storedChildId) => {
          if (storedChildId) {
            setSelectedChildIdState(storedChildId);
          } else if (
            authCtx?.dependents &&
            authCtx.dependents.length > 0 &&
            authCtx.dependents[0]?.student_id
          ) {
            setSelectedChildIdState(authCtx.dependents[0].student_id);
          } else if (childrenList.length > 0) {
            setSelectedChildIdState(childrenList[0].id);
          }
        },
      );
    }
  }, [isAuthenticated, user, authCtx, initialRole]);

  const setRole = useCallback(async (newRole: EffectiveRole) => {
    setRoleState(newRole);
    await setSecureItem(STORAGE_KEYS_CONTEXT.OVERRIDE_ROLE, newRole);
  }, []);

  const setSelectedChildId = useCallback(async (id: string | null) => {
    setSelectedChildIdState(id);
    if (id) {
      await setSecureItem(STORAGE_KEYS_CONTEXT.SELECTED_CHILD_ID, id);
    }
  }, []);

  const selectedChild = useMemo(() => {
    if (!selectedChildId) return childrenList[0] || null;
    return (
      childrenList.find((c) => c.id === selectedChildId) ||
      childrenList[0] ||
      null
    );
  }, [selectedChildId, childrenList]);

  const isStudent = role === "student";
  const isParent = role === "parent";
  const isTeacher = role === "teacher";
  const isAdmin = role === "admin";

  const roles = useMemo(() => authCtx?.roles || [], [authCtx]);
  const permissions = useMemo(() => authCtx?.permissions || [], [authCtx]);

  const hasPermission = useCallback(
    (permission: string): boolean => {
      if (!permissions || permissions.length === 0) return false;
      if (permissions.includes("*") || permissions.includes("admin")) {
        return true;
      }
      return permissions.includes(permission);
    },
    [permissions],
  );

  const targetStudentId = useMemo(() => {
    if (isParent) return selectedChild?.id || selectedChildId || null;
    return user?.id || null;
  }, [isParent, selectedChild, selectedChildId, user]);

  const value = useMemo<EffectiveContextType>(
    () => ({
      role,
      setRole,
      roles,
      permissions,
      hasPermission,
      selectedChildId,
      setSelectedChildId,
      selectedChild,
      childrenList,
      setChildrenList,
      selectedClassId,
      setSelectedClassId,
      activePeriodId,
      setActivePeriodId,
      isStudent,
      isParent,
      isTeacher,
      isAdmin,
      targetStudentId,
    }),
    [
      role,
      setRole,
      roles,
      permissions,
      hasPermission,
      selectedChildId,
      setSelectedChildId,
      selectedChild,
      childrenList,
      selectedClassId,
      activePeriodId,
      isStudent,
      isParent,
      isTeacher,
      isAdmin,
      targetStudentId,
    ],
  );

  return (
    <EffectiveContext.Provider value={value}>
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
