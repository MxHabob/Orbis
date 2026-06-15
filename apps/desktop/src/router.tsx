import { lazy, Suspense, type ComponentType, type ReactNode } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
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

// ── Page Components (lazy-loaded for optimal performance) ──────────────────
const OverviewPage = lazy(() => import("@/pages/overview"));
const GradesPage = lazy(() => import("@/pages/grades"));
const CertificatesPage = lazy(() => import("@/pages/certificates"));
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
const CertificateEditorPage = lazy(
  () => import("@/pages/certificates/editor"),
);
const CertificateCreatePage = lazy(
  () => import("@/pages/certificates/create"),
);
const ExportCertificatePage = lazy(
  () => import("@/pages/export/certificate"),
);
const ExportCertificateBulkPage = lazy(
  () => import("@/pages/export/certificate-bulk"),
);
const DesktopSearchPage = lazy(() => import("@/pages/desktop/search"));
const CheckoutSuccessPage = lazy(
  () => import("@/pages/desktop/checkout-success"),
);
const NotFoundPage = lazy(() => import("@/pages/not-found"));

// ── Loading Fallback ───────────────────────────────────────────────────────
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

// ── Router ─────────────────────────────────────────────────────────────────
export function AppRouter() {
  return (
    <AppErrorBoundary>
      <Suspense fallback={<PageSkeleton />}>
        <Routes>
        {/* ── Public Routes ─────────────────────────────── */}
        <Route path="/login" element={<LoginPage />} />

        {/* ── Export Routes (no sidebar, auth-guarded) ──── */}
        <Route element={<AuthGuard />}>
          <Route path="/export/certificate" element={<ExportCertificatePage />} />
          <Route
            path="/export/certificate/bulk"
            element={<ExportCertificateBulkPage />}
          />
        </Route>

        {/* ── Desktop-specific Routes ──────────────────── */}
        <Route path="/desktop/search" element={<DesktopSearchPage />} />
        <Route element={<AuthGuard />}>
          <Route
            path="/desktop/checkout/success"
            element={<CheckoutSuccessPage />}
          />
        </Route>

        {/* ── Editor Layout (full-screen, no sidebar) ──── */}
        <Route element={<AuthGuard />}>
          <Route element={<EditorLayout />}>
            <Route
              path="/certificates/editor/:id"
              element={<CertificateEditorPage />}
            />
            <Route
              path="/certificates/create"
              element={<CertificateCreatePage />}
            />
          </Route>
        </Route>

        {/* ── Sidebar Layout (main app) ────────────────── */}
        <Route element={<AuthGuard />}>
          <Route element={<SidebarLayout />}>
            {/* Overview */}
            <Route index element={<OverviewPage />} />

            {/* Grades */}
            <Route path="/grades" element={<GradesPage />} />

            {/* Certificates */}
            <Route path="/certificates" element={<CertificatesPage />} />

            {/* Reports */}
            <Route path="/reports" element={<ReportsPage />} />
            <Route path="/reports/:id" element={<ReportDetailPage />} />

            {/* Vault */}
            <Route path="/vault" element={<VaultPage />} />

            {/* Account */}
            <Route element={<AccountLayout />}>
              <Route path="/account" element={<AccountPage />} />
              <Route path="/account/support" element={<SupportPage />} />
            </Route>

            {/* Settings */}
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

        {/* ── Fallback ─────────────────────────────────── */}
        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </Suspense>
    </AppErrorBoundary>
  );
}
