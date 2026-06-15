import { useQuery } from "@tanstack/react-query";
import { api } from "./tauri-api";

export const authClient = {
  useSession: () => {
    const token = typeof window !== "undefined" ? localStorage.getItem("access_token") : null;
    const savedUser = typeof window !== "undefined" ? localStorage.getItem("session_user") : null;
    
    let initialUser = null;
    if (savedUser) {
      try {
        initialUser = JSON.parse(savedUser);
      } catch (_) {}
    }

    const query = useQuery({
      queryKey: ["auth", "session"],
      queryFn: async () => {
        if (!token) return null;
        try {
          const user = await api.auth.validateToken(token);
          if (user) {
            localStorage.setItem("session_user", JSON.stringify(user));
            return { user };
          }
          return null;
        } catch (err) {
          localStorage.removeItem("access_token");
          localStorage.removeItem("session_user");
          return null;
        }
      },
      initialData: initialUser ? { user: initialUser } : undefined,
      staleTime: 60 * 1000,
      retry: false,
    });

    return {
      data: query.data ?? null,
      isPending: query.isPending && !query.data,
      error: query.error,
    };
  },

  signIn: {
    email: async (params: { email: string; password: string; callbackURL?: string }) => {
      try {
        const response = await api.auth.login(params.email, params.password);
        if (response && response.access_token) {
          localStorage.setItem("access_token", response.access_token);
          localStorage.setItem("session_user", JSON.stringify(response.user));
          return { data: response, error: null };
        }
        return { data: null, error: new Error("Authentication failed") };
      } catch (err: any) {
        return { data: null, error: err instanceof Error ? err : new Error(String(err)) };
      }
    },
    social: async (_params: { provider: string; callbackURL?: string }) => {
      return { data: null, error: new Error("Social sign in not supported in desktop client") };
    },
    emailOtp: async (_params: any) => {
      return { data: null, error: new Error("OTP sign in not supported in desktop client") };
    },
    magicLink: async (_params: any) => {
      return { data: null, error: new Error("Magic link sign in not supported in desktop client") };
    }
  },

  signOut: async () => {
    const token = localStorage.getItem("access_token");
    if (token) {
      try {
        await api.auth.logout(token);
      } catch (_) {}
    }
    localStorage.removeItem("access_token");
    localStorage.removeItem("session_user");
    return { error: null };
  }
};
