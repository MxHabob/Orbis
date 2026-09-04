import { createOrbisClient } from "@orbis/sdk";
import type { SessionTokens, TokenStore } from "@orbis/sdk";
import { clearStoredSession, getStoredSession, setStoredSession } from "@/lib/secure-token-store";

const secureTokenStore: TokenStore = {
  async get(): Promise<SessionTokens | null> {
    const session = await getStoredSession();
    if (!session) return null;
    return {
      accessToken: session.accessToken,
      refreshToken: session.refreshToken,
      expiresAt: session.expiresAt,
      refreshExpiresAt: session.refreshExpiresAt,
    };
  },
  async set(session: SessionTokens): Promise<void> {
    await setStoredSession({
      accessToken: session.accessToken,
      refreshToken: session.refreshToken,
      expiresAt: session.expiresAt,
      refreshExpiresAt: session.refreshExpiresAt,
    });
  },
  async clear(): Promise<void> {
    await clearStoredSession();
  },
};

export const orbisSdk = createOrbisClient({
  baseUrl: "http://localhost:3002",
  tokenStore: secureTokenStore,
  refreshSession: async () => {
    const current = await secureTokenStore.get();
    if (!current?.refreshToken) return null;

    try {
      const result = await fetch("http://localhost:3002/api/auth/refresh", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        body: JSON.stringify({ refresh_token: current.refreshToken }),
      });

      const payload = await result.json();
      if (!result.ok || !payload?.success || !payload?.data?.session) {
        await secureTokenStore.clear();
        return null;
      }

      const next = payload.data.session as SessionTokens;
      await secureTokenStore.set(next);
      return next;
    } catch {
      await secureTokenStore.clear();
      return null;
    }
  },
});

export async function loginMobile(identifier: string, password: string) {
  return orbisSdk.post("/api/auth/login", {
    identifier,
    password,
  });
}
