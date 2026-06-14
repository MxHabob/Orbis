"use client";

import { Window } from "@orbis/desktop-client/core";

export function DesktopTrafficLight() {
  const getMainWindow = async () => {
    const mainWindow = await Window.getByLabel("main");
    if (!mainWindow) {
      throw new Error("Main window not found");
    }
    return mainWindow;
  };

  const handleClose = async () => {
    try {
      const window = await getMainWindow();
      // Hide the main window instead of closing it
      await window.hide();
    } catch (error) {
      console.error("Failed to hide main window:", error);
    }
  };

  const handleMinimize = async () => {
    try {
      const window = await getMainWindow();
      await window.minimize();
    } catch (error) {
      console.error("Failed to minimize main window:", error);
    }
  };

  const handleMaximize = async () => {
    try {
      const window = await getMainWindow();
      const isMaximized = await window.isMaximized();

      if (isMaximized) {
        await window.unmaximize();
      } else {
        await window.toggleMaximize();
      }
    } catch (error) {
      console.error("Failed to toggle maximize main window:", error);
    }
  };

  return (
    <div className="fixed top-[12px] left-[12px] flex items-center gap-[8px] z-[100]">
      {/* Close button */}
      <button
        type="button"
        onClick={handleClose}
        className="group relative w-3 h-3 flex items-center justify-center rounded-full bg-[#ff5f57] border border-[#e0443e] hover:bg-[#ff5f57] transition-all duration-200"
        aria-label="Close window"
      >
        <span className="opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-[8px] text-[#4c0000] font-bold">
          ✕
        </span>
      </button>

      {/* Minimize button */}
      <button
        type="button"
        onClick={handleMinimize}
        className="group relative w-3 h-3 flex items-center justify-center rounded-full bg-[#febc2e] border border-[#d8a024] hover:bg-[#febc2e] transition-all duration-200"
        aria-label="Minimize window"
      >
        <span className="opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-[10px] text-[#5c4000] font-bold mt-[-4px]">
          −
        </span>
      </button>

      {/* Maximize/Restore button */}
      <button
        type="button"
        onClick={handleMaximize}
        className="group relative w-3 h-3 flex items-center justify-center rounded-full bg-[#28c840] border border-[#24a030] hover:bg-[#28c840] transition-all duration-200"
        aria-label="Toggle maximize"
      >
        <span className="opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-[6px] text-[#003300] font-bold">
          ⇅
        </span>
      </button>
    </div>
  );
}
