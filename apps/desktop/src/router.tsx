import { lazy, Suspense, type ComponentType } from "react";
import { Routes, Route } from "react-router-dom";
import { AppErrorBoundary } from "@/components/app-error-boundary";
import { AuthGuard } from "@/components/auth-guard";
import LoginPage from "@/pages/login";

function lazyLayout<T extends Record<string, ComponentType<object>>>(
  factory: () => Promise<T>,
  name: keyof T,
) {
  const Lazy = lazy(() =>
    factory().then((mod) => ({ default: mod[name] as ComponentType<object> })),
  );

  return function LayoutRoute() {
    return (
      <Suspense fallback={<PageSkeleton />}>
        <Lazy />
      </Suspense>
    );
  };
}

const SidebarLayout = lazyLayout(
  () => import("@/layouts/sidebar-layout"),
  "SidebarLayout",
);
const EditorLayout = lazyLayout(
  () => import("@/layouts/editor-layout"),
  "EditorLayout",
);
const AccountLayout = lazyLayout(
  () => import("@/layouts/account-layout"),
  "AccountLayout",
);
const SettingsLayout = lazyLayout(
  () => import("@/layouts/settings-layout"),
  "SettingsLayout",
);

const OverviewPage = lazy(() => import("@/pages/overview"));
const GradesPage = lazy(() => import("@/pages/grades"));
const ReportsPage = lazy(() => import("@/pages/reports"));
const ReportDetailPage = lazy(() => import("@/pages/report-detail"));
const VaultPage = lazy(() => import("@/pages/vault"));
const AccountPage = lazy(() => import("@/pages/account"));
const SupportPage = lazy(() => import("@/pages/support"));
const SettingsGeneralPage = lazy(() => import("@/pages/settings/general"));
const SettingsClassesPage = lazy(() => import("@/pages/settings/classes"));
const SettingsSubjectsPage = lazy(() => import("@/pages/settings/subjects"));
const SettingsStaffPage = lazy(() => import("@/pages/settings/staff"));
const SettingsPromotionsPage = lazy(() => import("@/pages/settings/promotions"));
const SettingsControlPage = lazy(() => import("@/pages/settings/control"));
const SettingsNotificationsPage = lazy(
  () => import("@/pages/settings/notifications"),
);
const SettingsAccountsPage = lazy(() => import("@/pages/settings/accounts"));
const DocumentEditorPage = lazy(() => import("@/pages/editor/edit"));
const DocumentCreatePage = lazy(() => import("@/pages/editor/create"));
const DesktopSearchPage = lazy(() => import("@/pages/desktop/search"));
const NotFoundPage = lazy(() => import("@/pages/not-found"));

function PageSkeleton() {
  return (
    <div
      className="flex items-center justify-center bg-background"
      style={{ minHeight: "100vh" }}
    >
      <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" />
    </div>
  );
}

export function AppRouter() {
  return (
    <AppErrorBoundary>
      <Suspense fallback={<PageSkeleton />}>
        <Routes>
        <Route path="/login" element={<LoginPage />} />

        <Route path="/desktop/search" element={<DesktopSearchPage />} />

        <Route element={<AuthGuard />}>
          <Route element={<EditorLayout />}>
            <Route path="/editor/:id" element={<DocumentEditorPage />} />
            <Route path="/editor/create" element={<DocumentCreatePage />} />
          </Route>
        </Route>

        <Route element={<AuthGuard />}>
          <Route element={<SidebarLayout />}>
            <Route index element={<OverviewPage />} />
            <Route path="/grades" element={<GradesPage />} />
            <Route path="/reports" element={<ReportsPage />} />
            <Route path="/reports/:id" element={<ReportDetailPage />} />
            <Route path="/vault" element={<VaultPage />} />

            <Route element={<AccountLayout />}>
              <Route path="/account" element={<AccountPage />} />
              <Route path="/account/support" element={<SupportPage />} />
            </Route>

            <Route element={<SettingsLayout />}>
              <Route path="/settings" element={<SettingsGeneralPage />} />
              <Route path="/settings/classes" element={<SettingsClassesPage />} />
              <Route
                path="/settings/subjects"
                element={<SettingsSubjectsPage />}
              />
              <Route path="/settings/staff" element={<SettingsStaffPage />} />
              <Route
                path="/settings/promotions"
                element={<SettingsPromotionsPage />}
              />
              <Route
                path="/settings/control"
                element={<SettingsControlPage />}
              />
              <Route
                path="/settings/notifications"
                element={<SettingsNotificationsPage />}
              />
              <Route
                path="/settings/accounts"
                element={<SettingsAccountsPage />}
              />
            </Route>
          </Route>
        </Route>

        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </Suspense>
    </AppErrorBoundary>
  );
}
