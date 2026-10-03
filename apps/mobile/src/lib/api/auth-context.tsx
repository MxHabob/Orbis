/**
 * @module @orbis/mobile-api/auth-context
 *
 * React Context and Hook for authentication, session lifecycle,
 * and user state management in the Orbis Mobile Application.
 */

import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useMemo,
  useCallback,
} from "react";
import { api } from "./client";
import {
  getSecureItem,
  setSecureItem,
  deleteSecureItem,
  STORAGE_KEYS,
} from "./storage";
import type {
  EffectiveContextDto,
  LoginResponse,
  MfaChallengeRequest,
  MfaVerifyEnrollmentRequest,
  UserDto,
} from "@orbis/sdk";
import { OrbisError, ApiRequestError } from "./types";


export interface AuthState {
  user: UserDto | null;
  effectiveContext: EffectiveContextDto | null;
  token: string | null;
  isLoading: boolean;
  error: string | null;
  isSetupComplete: boolean | null;
  isAuthenticated: boolean;
  orgId: string;
  userId: string;
  needsOnboarding: boolean;
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
  checkSetupStatus: () => Promise<boolean>;
  clearError: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserDto | null>(null);
  const [effectiveContext, setEffectiveContext] =
    useState<EffectiveContextDto | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isSetupComplete, setIsSetupComplete] = useState<boolean | null>(null);

  const isAuthenticated = useMemo(() => !!token && !!user, [token, user]);
  const orgId = useMemo(() => user?.org_id ?? "", [user]);
  const userId = useMemo(() => user?.id ?? "", [user]);
  const needsOnboarding = useMemo(
    () => isAuthenticated && isSetupComplete === false,
    [isAuthenticated, isSetupComplete],
  );

  const clearError = useCallback(() => setError(null), []);

  const checkSetupStatus = useCallback(async (): Promise<boolean> => {
    try {
      const res = await api.getCurrentOrg({ throwOnError: true });
      const org = (res as any).data ?? res;
      const complete = !!org && !!org.id;
      setIsSetupComplete(complete);
      await setSecureItem(
        STORAGE_KEYS.SETUP_COMPLETE,
        complete ? "true" : "false",
      );
      return complete;
    } catch {
      // If unable to fetch org, retain existing or default to true
      const complete = true;
      setIsSetupComplete(complete);
      return complete;
    }
  }, []);

  const setSession = useCallback(
    async (res: {
      token: string;
      user: UserDto;
      effective_context?: EffectiveContextDto | null;
    }) => {
      setToken(res.token);
      setUser(res.user);
      if (res.effective_context) {
        setEffectiveContext(res.effective_context);
      }
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
      }

      await Promise.all(writes);
      await checkSetupStatus();
    },
    [checkSetupStatus],
  );

  const login = useCallback(
    async (email: string, password: string): Promise<LoginResponse> => {
      setIsLoading(true);
      setError(null);
      try {
        const res = await api.login({ body: { email, password }, throwOnError: true });
        const data = (res as any).data ?? res;
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
        const res = await api.googleLogin({ body: { id_token: idToken }, throwOnError: true });
        const data = (res as any).data ?? res;
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
        const res = await api.mfaChallenge({ body: req as any, throwOnError: true });
        const data = (res as any).data ?? res;
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
      const res = await api.mfaVerifyEnrollment({ body: req, throwOnError: true });
      return (res as any).data ?? res;
    },
    [],
  );

  const logout = useCallback(async () => {
    setUser(null);
    setEffectiveContext(null);
    setToken(null);
    setIsSetupComplete(null);

    await Promise.all([
      deleteSecureItem(STORAGE_KEYS.TOKEN),
      deleteSecureItem(STORAGE_KEYS.USER),
      deleteSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
      deleteSecureItem(STORAGE_KEYS.SETUP_COMPLETE),
    ]);
  }, []);

  const restoreSession = useCallback(async () => {
    setIsLoading(true);
    try {
      const [savedToken, savedUserStr, savedCtxStr, savedSetup] =
        await Promise.all([
          getSecureItem(STORAGE_KEYS.TOKEN),
          getSecureItem(STORAGE_KEYS.USER),
          getSecureItem(STORAGE_KEYS.EFFECTIVE_CONTEXT),
          getSecureItem(STORAGE_KEYS.SETUP_COMPLETE),
        ]);

      if (savedToken && savedUserStr) {
        try {
          const parsedUser = JSON.parse(savedUserStr) as UserDto;
          setToken(savedToken);
          setUser(parsedUser);
          if (savedCtxStr) {
            try {
              const parsedCtx = JSON.parse(savedCtxStr) as EffectiveContextDto;
              setEffectiveContext(parsedCtx);
            } catch {}
          }

          if (savedSetup !== null) {
            setIsSetupComplete(savedSetup === "true");
          }

          // Revalidate session with backend in background
          const res = await api.getSession({ throwOnError: true });
          const session = (res as any).data ?? res;
          if (session?.user) {
            setUser(session.user);
            await setSecureItem(
              STORAGE_KEYS.USER,
              JSON.stringify(session.user),
            );
          }
          if (session?.effective_context) {
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
                sessionErr.message.toLowerCase().includes("invalidsignature")));

          if (isInvalidToken) {
            await logout();
          }
        }
      }
    } catch (err) {
      console.warn("[Auth] Restore session error:", err);
    } finally {
      setIsLoading(false);
    }
  }, [logout]);

  useEffect(() => {
    restoreSession();
  }, [restoreSession]);

  const value = useMemo<AuthContextType>(
    () => ({
      user,
      effectiveContext,
      token,
      isLoading,
      error,
      isSetupComplete,
      isAuthenticated,
      orgId,
      userId,
      needsOnboarding,
      login,
      loginWithGoogle,
      mfaChallenge,
      mfaVerifyEnrollment,
      setSession,
      logout,
      restoreSession,
      checkSetupStatus,
      clearError,
    }),
    [
      user,
      effectiveContext,
      token,
      isLoading,
      error,
      isSetupComplete,
      isAuthenticated,
      orgId,
      userId,
      needsOnboarding,
      login,
      loginWithGoogle,
      mfaChallenge,
      mfaVerifyEnrollment,
      setSession,
      logout,
      restoreSession,
      checkSetupStatus,
      clearError,
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
