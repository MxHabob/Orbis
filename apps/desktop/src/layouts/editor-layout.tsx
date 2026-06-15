import { Outlet } from "react-router-dom";
import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";

/**
 * Full-screen editor layout — replaces Next.js (editor)/layout.tsx
 * Preserves exact same DOM structure and CSS classes.
 */
export function EditorLayout() {
  return (
    <div className="fixed inset-0 overflow-hidden bg-background">
      <Outlet />
      <GlobalSheetsProvider />
    </div>
  );
}
