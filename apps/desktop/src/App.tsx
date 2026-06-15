import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter } from "react-router-dom";
import { NuqsAdapter } from "nuqs/adapters/react-router/v6";
import { I18nProvider } from "@/i18n/provider";
import { ThemeProvider } from "@/components/theme-provider";
import { DesktopShell } from "@/components/desktop-shell";

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
            <ThemeProvider defaultTheme="system">
              <DesktopShell />
            </ThemeProvider>
          </I18nProvider>
        </NuqsAdapter>
      </BrowserRouter>
    </QueryClientProvider>
  );
}
