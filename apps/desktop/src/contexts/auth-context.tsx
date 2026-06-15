import { createContext, useContext, ReactNode } from "react";
import { authClient } from "@/lib/auth-client";

interface AuthContextType {
  user: any | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const { data: session, isPending: isLoading } = authClient.useSession();

  const logout = async () => {
    await authClient.signOut();
  };

  const user = session?.user ?? null;
  const isAuthenticated = !!user;

  return (
    <AuthContext.Provider value={{ user, isLoading, isAuthenticated, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
