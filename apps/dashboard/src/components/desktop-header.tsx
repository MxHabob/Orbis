"use client";

import { isDesktopApp } from "@orbis/desktop-client/platform";
import { usePathname } from "next/navigation";
import { DesktopTrafficLight } from "./desktop-traffic-light";

export function DesktopHeader() {
  const pathname = usePathname();

  if (!isDesktopApp() || pathname.includes("/search")) {
    return null;
  }

  // This is used to make the header draggable on macOS
  return (
    <div
      data-tauri-drag-region
      className="absolute top-0 left-0 right-0 h-10 z-[51] border-radius-[10px] overflow-hidden"
    >
      <div className="flex">
        <DesktopTrafficLight />
      </div>
    </div>
  );
}
