import React, { createContext, useCallback, useContext, useMemo, useState } from 'react';

import { orbisSdk } from './sdk';
import { clearStoredSession, getStoredSession, setStoredSession, type StoredSession } from './secure-token-store';

interface SessionPayload {
  accessToken?: string;
  refreshToken?: string;
  expiresAt?: string;
  refreshExpiresAt?: string;
}

interface ApiSessionEnvelope {
  session?: SessionPayload;
  data?: {
    session?: SessionPayload;
  };
}

function normalizeSession(value: SessionPayload | null | undefined): StoredSession | null {
  if (!value?.accessToken || !value.refreshToken) {
    return null;
  }

  return {
    accessToken: value.accessToken,
    refreshToken: value.refreshToken,
    expiresAt: value.expiresAt,
    refreshExpiresAt: value.refreshExpiresAt,
  };
}

export type AuthState =
  | { status: 'loading'; user: null }
  | { status: 'authenticated'; user: Record<string, unknown> }
  | { status: 'anonymous'; user: null };

interface AuthContextValue {
  authState: AuthState;
  login: (identifier: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  restore: () => Promise<void>;
  refreshSession: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [authState, setAuthState] = useState<AuthState>({ status: 'loading', user: null });

  const restore = useCallback(async () => {
    const session = await getStoredSession();
    if (!session?.accessToken) {
      setAuthState({ status: 'anonymous', user: null });
      return;
    }

    try {
      const me = await orbisSdk.get('/api/auth/me');
      setAuthState({ status: 'authenticated', user: me as Record<string, unknown> });
    } catch {
      try {
        const refreshed = (await orbisSdk.post('/api/auth/refresh', {
          refresh_token: session.refreshToken,
        })) as ApiSessionEnvelope;
        const nextSession = normalizeSession(refreshed.session ?? refreshed.data?.session ?? null);
        if (!nextSession) {
          await clearStoredSession();
          setAuthState({ status: 'anonymous', user: null });
          return;
        }
        await setStoredSession(nextSession);
        const me = await orbisSdk.get('/api/auth/me');
        setAuthState({ status: 'authenticated', user: me as Record<string, unknown> });
      } catch {
        await clearStoredSession();
        setAuthState({ status: 'anonymous', user: null });
      }
    }
  }, []);

  const login = useCallback(async (identifier: string, password: string) => {
    const result = (await orbisSdk.post('/api/auth/login', { identifier, password })) as ApiSessionEnvelope;
    const payload = normalizeSession(result.session ?? result.data?.session ?? null);
    if (!payload) {
      throw new Error('Invalid login response');
    }

    await setStoredSession(payload);
    const me = await orbisSdk.get('/api/auth/me');
    setAuthState({ status: 'authenticated', user: me as Record<string, unknown> });
  }, []);

  const logout = useCallback(async () => {
    try {
      const session = await getStoredSession();
      if (session?.accessToken) {
        await orbisSdk.post('/api/auth/logout', { token: session.accessToken });
      }
    } finally {
      await clearStoredSession();
      setAuthState({ status: 'anonymous', user: null });
    }
  }, []);

  const refreshSession = useCallback(async () => {
    const session = await getStoredSession();
    if (!session?.refreshToken) {
      setAuthState({ status: 'anonymous', user: null });
      return;
    }

    try {
      const refreshed = (await orbisSdk.post('/api/auth/refresh', {
        refresh_token: session.refreshToken,
      })) as ApiSessionEnvelope;
      const nextSession = normalizeSession(refreshed.session ?? refreshed.data?.session ?? null);
      if (!nextSession) {
        await clearStoredSession();
        setAuthState({ status: 'anonymous', user: null });
        return;
      }

      await setStoredSession(nextSession);
      const me = await orbisSdk.get('/api/auth/me');
      setAuthState({ status: 'authenticated', user: me as Record<string, unknown> });
    } catch {
      await clearStoredSession();
      setAuthState({ status: 'anonymous', user: null });
    }
  }, []);

  const value = useMemo<AuthContextValue>(
    () => ({ authState, login, logout, restore, refreshSession }),
    [authState, login, logout, restore, refreshSession],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used inside AuthProvider');
  }
  return context;
}
