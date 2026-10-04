// ── Global Confirm Store ─────────────────────────────────────────────
// Svelte 5 Runes-based store for managing imperative global confirmation dialogs.
// This allows any part of the app to await a user confirmation seamlessly.
//
// Architecture:
// - Feature code calls `confirmStore.prompt(options)` and awaits a boolean.
// - For SIMPLE destructive confirmations, we use the native OS dialog via Tauri
//   when running inside the desktop shell. This gives a true native feel.
// - For COMPLEX confirmations (custom UI, forms, multi-step), or when Tauri
//   is unavailable, the GlobalConfirmDialog component renders the custom overlay.
// - Feature code does NOT know which path is taken. The abstraction owns that.

import * as m from "$lib/paraglide/messages";

export type ConfirmOptions = {
  title: string;
  description: string;
  confirmText: string;
  cancelText?: string;
  variant?:
    | "default"
    | "destructive"
    | "outline"
    | "secondary"
    | "ghost"
    | "link";
  requireMfa?: boolean; // Future expansion
  /**
   * When true, forces the custom Orbis dialog even if Tauri native is available.
   * Use for confirmations that need rich UI (previews, forms, etc.).
   */
  forceCustomDialog?: boolean;
};

/**
 * Detect whether Tauri IPC is available at runtime.
 * This allows the same code to work in both the Tauri desktop shell
 * and a plain browser dev environment.
 */
function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Attempt native OS confirmation via Tauri dialog plugin.
 * Returns the user's choice, or null if Tauri is not available.
 */
async function tryNativeConfirm(
  options: ConfirmOptions,
): Promise<boolean | null> {
  if (!isTauriAvailable()) return null;

  try {
    const { ask } = await import("@tauri-apps/plugin-dialog");
    const result = await ask(options.description, {
      title: options.title,
      kind: options.variant === "destructive" ? "warning" : "info",
      okLabel: options.confirmText,
      cancelLabel: options.cancelText,
    });
    return result;
  } catch {
    // Plugin not loaded or capability not granted — fall through to custom dialog
    return null;
  }
}

class ConfirmStore {
  isOpen = $state(false);
  options = $state<ConfirmOptions | null>(null);
  private resolvePromise: ((value: boolean) => void) | null = null;

  async prompt(options: ConfirmOptions): Promise<boolean> {
    // ── Native path ────────────────────────────────────────────
    // Use native OS dialog for simple confirmations when Tauri is available,
    // unless the caller explicitly wants the custom dialog.
    if (!options.forceCustomDialog && !options.requireMfa) {
      const nativeResult = await tryNativeConfirm(options);
      if (nativeResult !== null) {
        return nativeResult;
      }
    }

    // ── Custom dialog path ─────────────────────────────────────
    this.options = options;
    this.isOpen = true;

    return new Promise((resolve) => {
      this.resolvePromise = resolve;
    });
  }

  respond(result: boolean) {
    if (this.resolvePromise) {
      this.resolvePromise(result);
      this.resolvePromise = null;
    }
    this.isOpen = false;
  }

  /**
   * Prompts the user with a standardized version conflict (optimistic lock 409) dialog.
   * Returns true if the user confirmed reloading data.
   */
  async promptVersionConflict(message?: string): Promise<boolean> {
    return this.prompt({
      title: m.shared_ui_version_conflict_title(),
      description:
        message ||
        m.shared_ui_version_conflict_description(),
      confirmText: m.shared_ui_reload_latest_data(),
      cancelText: m.common_action_cancel(),
      variant: "default",
      forceCustomDialog: true,
    });
  }
}

export const confirmStore = new ConfirmStore();
