import { DesktopHeader } from "@/components/desktop-header";
import { DesktopProvider } from "@/components/desktop-provider";
import { AppRouter } from "@/router";
import { Toaster } from "@orbis/ui/toaster";

/** Shell layout: desktop chrome + routing — no extra context providers. */
export function DesktopShell() {
  return (
    <>
      <DesktopHeader />
      <DesktopProvider />
      <AppRouter />
      <Toaster />
    </>
  );
}
