import { BrowserRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { NuqsAdapter } from "nuqs/adapters/react-router/v6";
import { ThemeProvider } from "@/components/theme-provider";
import { AuthProvider } from "@/contexts/auth-context";
import { I18nProvider } from "@/i18n/provider";
import { AppRouter } from "./router";
import { Toaster } from "@orbis/ui/toaster";
import { DesktopProvider } from "@/components/desktop-provider";
import { DesktopHeader } from "@/components/desktop-header";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      retry: 1,
      refetchOnWindowFocus: false,
    },
    mutations: {
      retry: 0,
    },
  },
});

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <NuqsAdapter>
          <I18nProvider>
            <AuthProvider>
              <ThemeProvider
                // attribute="class"
                defaultTheme="system"
                // enableSystem
                // disableTransitionOnChange
              >
                <DesktopHeader />
                <DesktopProvider />
                <AppRouter />
                <Toaster />
              </ThemeProvider>
            </AuthProvider>
          </I18nProvider>
        </NuqsAdapter>
      </BrowserRouter>
    </QueryClientProvider>
  );
}
