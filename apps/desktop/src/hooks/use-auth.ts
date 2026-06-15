import { useQuery } from "@tanstack/react-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import {
  clearSession,
  getAccessToken,
  setSession,
} from "@/lib/session";

export function useAuth() {
  const token = getAccessToken();
  const savedUser =
    typeof localStorage !== "undefined"
      ? localStorage.getItem("session_user")
      : null;
  let initialUser = null;
  if (savedUser) {
    try {
      initialUser = JSON.parse(savedUser);
    } catch {
      initialUser = null;
    }
  }

  const { data: session, isPending: isLoading } = useQuery({
    queryKey: queryKeys.auth.session(),
    queryFn: async () => {
      if (!token) return null;
      try {
        const user = await commands.auth.validateToken(token);
        localStorage.setItem("session_user", JSON.stringify(user));
        return { user };
      } catch {
        clearSession();
        return null;
      }
    },
    initialData: initialUser ? { user: initialUser } : undefined,
    staleTime: 60_000,
    retry: false,
  });

  const logout = async () => {
    const t = getAccessToken();
    if (t) {
      try {
        await commands.auth.logout(t);
      } catch {
        /* ignore */
      }
    }
    clearSession();
    window.location.href = "/login";
  };

  const user = session?.user ?? null;

  return {
    user,
    isLoading: isLoading && !session,
    isAuthenticated: !!user,
    logout,
  };
}

export const authClient = {
  useSession: () => {
    const { user, isLoading } = useAuth();
    return {
      data: user ? { user } : null,
      isPending: isLoading,
      error: null,
    };
  },
  signIn: {
    email: async (params: { email: string; password: string }) => {
      try {
        const response = await commands.auth.login(
          params.email,
          params.password,
        );
        if (response?.access_token) {
          setSession(
            response.user as Record<string, unknown>,
            response.access_token,
          );
          return { data: response, error: null };
        }
        return { data: null, error: new Error("Authentication failed") };
      } catch (err) {
        return {
          data: null,
          error: err instanceof Error ? err : new Error(String(err)),
        };
      }
    },
    social: async () => ({
      data: null,
      error: new Error("Social sign in not supported in desktop client"),
    }),
    emailOtp: async () => ({
      data: null,
      error: new Error("OTP sign in not supported in desktop client"),
    }),
    magicLink: async () => ({
      data: null,
      error: new Error("Magic link sign in not supported in desktop client"),
    }),
  },
  signOut: async () => {
    const token = getAccessToken();
    if (token) {
      try {
        await commands.auth.logout(token);
      } catch {
        /* ignore */
      }
    }
    clearSession();
    return { error: null };
  },
};
