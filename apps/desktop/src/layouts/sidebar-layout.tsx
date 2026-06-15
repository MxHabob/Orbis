import { Outlet } from "react-router-dom";
import { ExportStatus } from "@/components/export-status";
import { Header } from "@/components/header";
import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";
import { Sidebar } from "@/components/sidebar";

/**
 * Main app layout with sidebar — replaces Next.js (sidebar)/layout.tsx
 * Preserves exact same DOM structure and CSS classes.
 */
export function SidebarLayout() {
  return (
    <div className="relative">
      <Sidebar />
      <div className="md:ms-[70px] pb-4">
        <Header />
        <div className="px-4 md:px-8">
          <Outlet />
        </div>
      </div>
      <ExportStatus />
      <GlobalSheetsProvider />
    </div>
  );
}
