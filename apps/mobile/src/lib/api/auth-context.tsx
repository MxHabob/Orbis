/**
 * @module @orbis/mobile-api/auth-context
 *
 * React Context and Hook for authentication, session lifecycle,
 * and user state management in the Orbis Mobile Application.
 */

import type {
  EffectiveContextDto,
  LoginResponse,
  MfaChallengeRequest,
  MfaVerifyEnrollmentRequest,
  UserDto,
} from "@orbis/sdk";
import type React from "react";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { api, configureApiBaseUrl } from "./client";
import {
  deleteSecureItem,
  getSecureItem,
  getInstitutionSession,
  STORAGE_KEYS,
  setSecureItem,
  setInstitutionSession,
  type InstitutionSessionSnapshot,
} from "./storage";
import { ApiRequestError, OrbisError } from "./types";
import {
  notifySessionEnded,
  subscribeToSessionInvalidation,
} from "./session-events";
import { onlineManager } from "@/lib/network/online-manager";
import {
  getSavedInstitutions,
  saveSelectedInstitution,
  type DirectoryInstitution,
} from "./institution-discovery";

export interface AuthState {
  user: UserDto | null;
  effectiveContext: EffectiveContextDto | null;
  token: string | null;
  isInitializing: boolean;
  isLoading: boolean;
  error: string | null;
  isSetupComplete: boolean | null;
  isAuthenticated: boolean;
  orgId: string;
  userId: string;
  needsOnboarding: boolean;
  institutions: DirectoryInstitution[];
  selectedInstitution: DirectoryInstitution | null;
}

export interface AuthContextType extends AuthState {
  login: (email: string, password: string) => Promise<LoginResponse>;
  loginWithGoogle: (idToken: string) => Promise<LoginResponse>;
  mfaChallenge: (req: MfaChallengeRequest) => Promise<LoginResponse>;
  mfaVerifyEnrollment: (req: MfaVerifyEnrollmentRequest) => Promise<boolean>;
  setSession: (session: {
    token: string;
    user: UserDto;
    effective_context?: EffectiveContextDto | null;
  }) => Promise<void>;
  logout: () => Promise<void>;
  restoreSession: () => Promise<void>;
  refreshSessionData: () => Promise<void>;
  checkSetupStatus: () => Promise<boolean | null>;
  clearError: () => void;
  selectInstitution: (institution: DirectoryInstitution) => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

function hasData(value: unknown): value is { data: unknown } {
  return typeof value === "object" && value !== null && "data" in value;
}

function unwrapApiData<T>(response: unknown): T {
  const sdkData = hasData(response) ? response.data : response;
  const apiData = hasData(sdkData) ? sdkData.data : sdkData;
  return apiData as T;
}

function isUserDto(value: unknown): value is UserDto {
  if (typeof value !== "object" || value === null) return false;

  const user = value as Record<string, unknown>;
  return (
    typeof user.id === "string" &&
    typeof user.org_id === "string" &&
    typeof user.person_id === "string" &&
    typeof user.full_name === "string" &&
    typeof user.status === "string" &&
    typeof user.created_at === "string" &&
    typeof user.updated_at === "string" &&
    typeof user.is_active === "boolean" &&
    typeof user.is_email_verified === "boolean" &&
    typeof user.is_mfa_enabled === "boolean" &&
    typeof user.is_phone_verified === "boolean"
  );
}

function isEffectiveContextDto(value: unknown): value is EffectiveContextDto {
  if (typeof value !== "object" || value === null) return false;

  const context = value as Record<string, unknown>;
  return (
    typeof context.effective_role === "string" &&
    Array.isArray(context.roles) &&
    context.roles.every((role) => typeof role === "string") &&
    Array.isArray(context.permissions) &&
    context.permissions.every((permission) => typeof permission === "string") &&
    Array.isArray(context.dependents) &&
    context.dependents.every(
      (dependent) =>
        typeof dependent === "object" &&
        dependent !== null &&
        "student_id" in dependent &&
        typeof dependent.student_id === "string",
    ) &&
    Array.isArray(context.teaching_classes) &&
    context.teaching_classes.every(
      (teachingClass) =>
        typeof teachingClass === "object" &&
        teachingClass !== null &&
        "section_id" in teachingClass &&
        typeof teachingClass.section_id === "string",
    )
  );
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserDto | null>(null);
  const userRef = useRef(user);
  useEffect(() => {
    userRef.current = user;
  }, [user]);
  const [effectiveContext, setEffectiveContext] =
    useState<EffectiveContextDto | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [isInitializing, setIsInitializing] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isSetupComplete, setIsSetupComplete] = useState<boolean | null>(null);
  const [institutions, setInstitutions] = useState<DirectoryInstitution[]>([]);
  const [selectedInstitution, setSelectedInstitution] =
    useState<DirectoryInstitution | null>(null);
  const selectedInstitutionRef = useRef(selectedInstitution);
  useEffect(() => {
    selectedInstitutionRef.current = selectedInstitution;
  }, [selectedInstitution]);

  const isAuthenticated = useMemo(() => !!token && !!user, [token, user]);
  const orgId = useMemo(() => user?.org_id ?? "", [user]);
  const userId = useMemo(() => user?.id ?? "", [user]);
  const needsOnboarding = useMemo(
    () => isAuthenticated && isSetupComplete === false,
    [isAuthenticated, isSetupComplete],
  );

  const clearError = useCallback(() => setError(null), []);

  const checkSetupStatus = useCallback(async (): Promise<boolean | null> => {
    try {
      const res = await api.getCurrentOrg({ throwOnError: true });
      const org = unwrapApiData<{ id?: string } | null>(res);
      const complete = !!org && !!org.id;
      setIsSetupComplete(complete);
      await setSecureItem(
        STORAGE_KEYS.SETUP_COMPLETE,
        complete ? "true" : "false",
      );
      return complete;
    } catch (error) {
      console.warn(
        "[Auth] Unable to determine organization setup status.",
        error,
      );
      return null;
    }
  }, []);

  const setSession = useCallback(
    async (res: {
      token: string;
      user: UserDto;
      effective_context?: EffectiveContextDto | null;
    }) => {
      if (
        res.effective_context &&
        !isEffectiveContextDto(res.effective_context)
      ) {
        throw new Error(
          "The login response included an invalid effective context.",
        );
      }

      if (
        user &&
        (user.id !== res.user.id || user.org_id !== res.user.org_id)
      ) {
        notifySessionEnded({ orgId: user.org_id, userId: user.id });
      }

      setToken(res.token);
      setUser(res.user);
      setEffectiveContext(res.effective_context ?? null);
      setIsSetupComplete(null);
      setError(null);

      const writes: Promise<void>[] = [
        setSecureItem(STORAGE_KEYS.TOKEN, res.token),
        setSecureItem(STORAGE_KEYS.USER, JSON.stringify(res.user)),
      ];

      if (res.effective_context) {
        writes.push(
          setSecureItem(
            STORAGE_KEYS.EFFECTIVE_CONTEXT,
            JSON.stringify(res.effective_context),
          ),
        );
      } else {
        writes.push(deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT));
      }

      await Promise.all(writes);
      const setupComplete = await checkSetupStatus();
      if (selectedInstitution) {
        await setInstitutionSession(selectedInstitution.deployment_id, {
          token: res.token,
          user: JSON.stringify(res.user),
          effectiveContext: res.effective_context
            ? JSON.stringify(res.effective_context)
            : null,
          setupComplete:
            setupComplete === null ? null : setupComplete ? "true" : "false",
        });
      }
    },
    [checkSetupStatus, selectedInstitution, user],
  );

  const login = useCallback(
    async (email: string, password: string): Promise<LoginResponse> => {
      setIsLoading(true);
      setError(null);
      try {
        const res = await api.login({
          body: { email, password },
          throwOnError: true,
        });
        const data = unwrapApiData<LoginResponse>(res);
        if (data?.token && data?.user) {
          await setSession({
            token: data.token,
            user: data.user,
            effective_context: data.effective_context,
          });
        }
        return data;
      } catch (err) {
        const orbisErr = OrbisError.fromUnknown(err);
        setError(orbisErr.message);
        throw orbisErr;
      } finally {
        setIsLoading(false);
      }
    },
    [setSession],
  );

  const loginWithGoogle = useCallback(
    async (idToken: string): Promise<LoginResponse> => {
      setIsLoading(true);
      setError(null);
      try {
        const res = await api.googleLogin({
          body: { id_token: idToken },
          throwOnError: true,
        });
        const data = unwrapApiData<LoginResponse>(res);
        if (data?.token && data?.user) {
          await setSession({
            token: data.token,
            user: data.user,
            effective_context: data.effective_context,
          });
        }
        return data;
      } catch (err) {
        const orbisErr = OrbisError.fromUnknown(err);
        setError(orbisErr.message);
        throw orbisErr;
      } finally {
        setIsLoading(false);
      }
    },
    [setSession],
  );

  const mfaChallenge = useCallback(
    async (req: MfaChallengeRequest): Promise<LoginResponse> => {
      setIsLoading(true);
      setError(null);
      try {
        const res = await api.mfaChallenge({
          body: req as any,
          throwOnError: true,
        });
        const data = unwrapApiData<LoginResponse>(res);
        if (data?.token && data?.user) {
          await setSession({
            token: data.token,
            user: data.user,
            effective_context: data.effective_context,
          });
        }
        return data;
      } catch (err) {
        const orbisErr = OrbisError.fromUnknown(err);
        setError(orbisErr.message);
        throw orbisErr;
      } finally {
        setIsLoading(false);
      }
    },
    [setSession],
  );

  const mfaVerifyEnrollment = useCallback(
    async (req: MfaVerifyEnrollmentRequest): Promise<boolean> => {
      const res = await api.mfaVerifyEnrollment({
        body: req,
        throwOnError: true,
      });
      return unwrapApiData<boolean>(res);
    },
    [],
  );

  const logout = useCallback(async () => {
    const currentUser = userRef.current;
    if (currentUser) {
      notifySessionEnded({
        orgId: currentUser.org_id,
        userId: currentUser.id,
      });
    }

    setUser(null);
    setEffectiveContext(null);
    setToken(null);
    setIsSetupComplete(null);

    const removals: Promise<void>[] = [
      deleteSecureItem(STORAGE_KEYS.TOKEN),
      deleteSecureItem(STORAGE_KEYS.USER),
      deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
      deleteSecureItem(STORAGE_KEYS.SETUP_COMPLETE),
    ];
    if (selectedInstitutionRef.current) {
      const sessionsRaw = await getSecureItem(
        STORAGE_KEYS.INSTITUTION_SESSIONS,
      );
      if (sessionsRaw) {
        const sessions: Record<string, InstitutionSessionSnapshot> =
          JSON.parse(sessionsRaw);
        delete sessions[selectedInstitutionRef.current.deployment_id];
        removals.push(
          setSecureItem(
            STORAGE_KEYS.INSTITUTION_SESSIONS,
            JSON.stringify(sessions),
          ),
        );
      }
    }
    await Promise.all(removals);
  }, []);

  const restoreSession = useCallback(async () => {
    setIsInitializing(true);
    try {
      const [
        savedToken,
        savedUserStr,
        savedCtxStr,
        savedSetup,
        savedApiBaseUrl,
        savedInstitutions,
        activeInstitutionId,
      ] = await Promise.all([
        getSecureItem(STORAGE_KEYS.TOKEN),
        getSecureItem(STORAGE_KEYS.USER),
        getSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
        getSecureItem(STORAGE_KEYS.SETUP_COMPLETE),
        getSecureItem(STORAGE_KEYS.API_BASE_URL),
        getSavedInstitutions(),
        getSecureItem(STORAGE_KEYS.ACTIVE_INSTITUTION_ID),
      ]);

      setInstitutions(savedInstitutions);
      const activeInstitution =
        savedInstitutions.find(
          (institution) => institution.deployment_id === activeInstitutionId,
        ) ?? null;
      selectedInstitutionRef.current = activeInstitution;
      setSelectedInstitution(activeInstitution);

      if (savedApiBaseUrl) {
        configureApiBaseUrl(savedApiBaseUrl);
      }
      if (savedToken) {
        setToken(savedToken);
        let restoredUser: UserDto | null = null;

        if (savedUserStr) {
          try {
            const cachedUser: unknown = JSON.parse(savedUserStr);
            if (!isUserDto(cachedUser)) {
              throw new Error("Cached user data has an invalid shape.");
            }
            restoredUser = cachedUser;
            setUser(cachedUser);
          } catch (error) {
            console.warn("[Auth] Discarding invalid cached user:", error);
            await deleteSecureItem(STORAGE_KEYS.USER);
          }
        }

        if (savedCtxStr) {
          try {
            const cachedContext: unknown = JSON.parse(savedCtxStr);
            if (!isEffectiveContextDto(cachedContext)) {
              throw new Error("Cached effective context has an invalid shape.");
            }
            setEffectiveContext(cachedContext);
          } catch (error) {
            console.warn("[Auth] Discarding invalid cached context:", error);
            await deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT);
          }
        }

        if (savedSetup !== null) {
          if (savedSetup === "true" || savedSetup === "false") {
            setIsSetupComplete(savedSetup === "true");
          }
        }

        if (onlineManager.isOnline()) {
          try {
            const res = await api.getSession({ throwOnError: true });
            const session = unwrapApiData<{
              user?: UserDto;
              effective_context?: EffectiveContextDto;
            }>(res);
            if (isUserDto(session?.user)) {
              restoredUser = session.user;
              setUser(session.user);
              await setSecureItem(
                STORAGE_KEYS.USER,
                JSON.stringify(session.user),
              );
            } else if (!restoredUser) {
              console.warn(
                "[Auth] Session response did not contain a valid user.",
              );
              await logout();
            }
            if (isEffectiveContextDto(session?.effective_context)) {
              setEffectiveContext(session.effective_context);
              await setSecureItem(
                STORAGE_KEYS.EFFECTIVE_CONTEXT,
                JSON.stringify(session.effective_context),
              );
            }
          } catch (sessionErr) {
            console.warn("[Auth] Session validation notice:", sessionErr);
            const isInvalidToken =
              (sessionErr instanceof ApiRequestError &&
                (sessionErr.status === 401 || sessionErr.status === 403)) ||
              (sessionErr instanceof Error &&
                (sessionErr.message.toLowerCase().includes("unauthorized") ||
                  sessionErr.message
                    .toLowerCase()
                    .includes("invalid or expired token") ||
                  sessionErr.message
                    .toLowerCase()
                    .includes("invalidsignature")));

            if (isInvalidToken || !restoredUser) {
              await logout();
            }
          }
        } else if (!restoredUser) {
          await logout();
        }
      }
    } catch (err) {
      console.warn("[Auth] Restore session error:", err);
    } finally {
      setIsInitializing(false);
    }
  }, [logout]);

  const selectInstitution = useCallback(
    async (institution: DirectoryInstitution) => {
      const previousInstitution = selectedInstitutionRef.current;
      if (previousInstitution && token && user) {
        const sessionsRaw = await getSecureItem(
          STORAGE_KEYS.INSTITUTION_SESSIONS,
        );
        const sessions: Record<string, InstitutionSessionSnapshot> = sessionsRaw
          ? JSON.parse(sessionsRaw)
          : {};
        sessions[previousInstitution.deployment_id] = {
          token,
          user: JSON.stringify(user),
          effectiveContext: effectiveContext
            ? JSON.stringify(effectiveContext)
            : null,
          setupComplete:
            isSetupComplete === null
              ? null
              : isSetupComplete
                ? "true"
                : "false",
        };
        await setSecureItem(
          STORAGE_KEYS.INSTITUTION_SESSIONS,
          JSON.stringify(sessions),
        );
      }

      const savedInstitutions = await saveSelectedInstitution(institution);
      const snapshot = await getInstitutionSession(institution.deployment_id);
      configureApiBaseUrl(institution.endpoint_url);
      if (
        previousInstitution &&
        previousInstitution.deployment_id !== institution.deployment_id &&
        user
      ) {
        notifySessionEnded({ orgId: user.org_id, userId: user.id });
      }

      const cachedUser: unknown = snapshot ? JSON.parse(snapshot.user) : null;
      const restoredUser = isUserDto(cachedUser) ? cachedUser : null;
      const cachedContext: unknown = snapshot?.effectiveContext
        ? JSON.parse(snapshot.effectiveContext)
        : null;
      const restoredContext = isEffectiveContextDto(cachedContext)
        ? cachedContext
        : null;

      setInstitutions(savedInstitutions);
      selectedInstitutionRef.current = institution;
      setSelectedInstitution(institution);
      setToken(restoredUser && snapshot ? snapshot.token : null);
      setUser(restoredUser);
      setEffectiveContext(restoredContext);
      setIsSetupComplete(
        snapshot?.setupComplete === "true"
          ? true
          : snapshot?.setupComplete === "false"
            ? false
            : null,
      );
      setError(null);

      await Promise.all([
        restoredUser && snapshot
          ? setSecureItem(STORAGE_KEYS.TOKEN, snapshot.token)
          : deleteSecureItem(STORAGE_KEYS.TOKEN),
        restoredUser
          ? setSecureItem(STORAGE_KEYS.USER, JSON.stringify(restoredUser))
          : deleteSecureItem(STORAGE_KEYS.USER),
        restoredContext
          ? setSecureItem(
              STORAGE_KEYS.EFFECTIVE_CONTEXT,
              JSON.stringify(restoredContext),
            )
          : deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
        snapshot?.setupComplete
          ? setSecureItem(STORAGE_KEYS.SETUP_COMPLETE, snapshot.setupComplete)
          : deleteSecureItem(STORAGE_KEYS.SETUP_COMPLETE),
      ]);

      await restoreSession();
    },
    [effectiveContext, isSetupComplete, restoreSession, token, user],
  );

  const refreshSessionData = useCallback(async () => {
    const res = await api.getSession({ throwOnError: true });
    const session = unwrapApiData<{
      user?: UserDto;
      effective_context?: EffectiveContextDto | null;
    }>(res);
    if (!isUserDto(session?.user)) {
      throw new Error("The session response did not include a user.");
    }

    setUser(session.user);
    const writes = [
      setSecureItem(STORAGE_KEYS.USER, JSON.stringify(session.user)),
    ];
    if ("effective_context" in session) {
      const nextContext = session.effective_context ?? null;
      if (nextContext && !isEffectiveContextDto(nextContext)) {
        throw new Error(
          "The session response included an invalid effective context.",
        );
      }
      setEffectiveContext(nextContext);
      writes.push(
        nextContext
          ? setSecureItem(
              STORAGE_KEYS.EFFECTIVE_CONTEXT,
              JSON.stringify(nextContext),
            )
          : deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
      );
    }
    await Promise.all(writes);
    if (selectedInstitutionRef.current && token) {
      const currentContext =
        "effective_context" in session
          ? (session.effective_context ?? null)
          : null;
      await setInstitutionSession(
        selectedInstitutionRef.current.deployment_id,
        {
          token,
          user: JSON.stringify(session.user),
          effectiveContext: currentContext
            ? JSON.stringify(currentContext)
            : null,
          setupComplete:
            isSetupComplete === null
              ? null
              : isSetupComplete
                ? "true"
                : "false",
        },
      );
    }
  }, [isSetupComplete, token]);

  useEffect(() => {
    return subscribeToSessionInvalidation(() => {
      void logout().catch((error: unknown) => {
        console.error("[Auth] Failed to clear an invalidated session.", error);
      });
    });
  }, [logout]);

  useEffect(() => {
    void Promise.resolve().then(restoreSession);
  }, [restoreSession]);

  const value = useMemo<AuthContextType>(
    () => ({
      user,
      effectiveContext,
      token,
      isInitializing,
      isLoading,
      error,
      isSetupComplete,
      isAuthenticated,
      orgId,
      userId,
      needsOnboarding,
      institutions,
      selectedInstitution,
      login,
      loginWithGoogle,
      mfaChallenge,
      mfaVerifyEnrollment,
      setSession,
      logout,
      restoreSession,
      refreshSessionData,
      checkSetupStatus,
      clearError,
      selectInstitution,
    }),
    [
      user,
      effectiveContext,
      token,
      isInitializing,
      isLoading,
      error,
      isSetupComplete,
      isAuthenticated,
      orgId,
      userId,
      needsOnboarding,
      institutions,
      selectedInstitution,
      login,
      loginWithGoogle,
      mfaChallenge,
      mfaVerifyEnrollment,
      setSession,
      logout,
      restoreSession,
      refreshSessionData,
      checkSetupStatus,
      clearError,
      selectInstitution,
    ],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
