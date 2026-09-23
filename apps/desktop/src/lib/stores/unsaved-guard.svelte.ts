// ── Unsaved Changes Guard ────────────────────────────────────────────
// Provides navigation and window-close protection for forms with unsaved changes.
//
// Usage in a page/component:
//   import { useUnsavedGuard } from "$lib/stores/unsaved-guard.svelte";
//   const guard = useUnsavedGuard();
//   // When the form becomes dirty:
//   guard.setDirty(true);
//   // When the form is saved/reset:
//   guard.setDirty(false);
//
// The guard automatically:
//   - Blocks SvelteKit navigation (beforeNavigate) when dirty
//   - Blocks Tauri window close (onCloseRequested) when dirty
//   - Shows a confirmation dialog via confirmStore.prompt()
//   - Cleans up when the component is destroyed

import { beforeNavigate } from "$app/navigation";
import { onMount, onDestroy } from "svelte";
import { confirmStore } from "./confirm.svelte";

/**
 * Detect whether we are running inside a Tauri shell.
 */
function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Options for the unsaved changes guard.
 */
export interface UnsavedGuardOptions {
  /** Title shown in the confirmation dialog */
  title?: string;
  /** Description shown in the confirmation dialog */
  description?: string;
  /** Text for the "discard" button */
  discardText?: string;
  /** Text for the "stay" / cancel button */
  stayText?: string;
}

const DEFAULT_TITLE = "Unsaved Changes";
const DEFAULT_DESCRIPTION =
  "You have unsaved changes. Are you sure you want to leave? Your changes will be lost.";
const DEFAULT_DISCARD = "Discard Changes";
const DEFAULT_STAY = "Stay";

/**
 * Creates an unsaved-changes guard that blocks navigation and window close
 * when the form is dirty. Must be called during component initialization.
 */
export function useUnsavedGuard(options: UnsavedGuardOptions = {}) {
  let isDirty = $state(false);
  let unlisten: (() => void) | null = null;

  const title = options.title ?? DEFAULT_TITLE;
  const description = options.description ?? DEFAULT_DESCRIPTION;
  const discardText = options.discardText ?? DEFAULT_DISCARD;
  const stayText = options.stayText ?? DEFAULT_STAY;

  // ── SvelteKit navigation guard ─────────────────────────────────
  beforeNavigate(async (navigation) => {
    if (!isDirty) return;

    // Cancel the navigation first, then ask
    navigation.cancel();

    const shouldLeave = await confirmStore.prompt({
      title,
      description,
      confirmText: discardText,
      cancelText: stayText,
      variant: "destructive",
    });

    if (shouldLeave) {
      isDirty = false;
      // Re-trigger the navigation now that we've cleared dirty state
      if (navigation.to?.url) {
        // Use window.location for guaranteed navigation after cancel
        window.location.href = navigation.to.url.href;
      }
    }
  });

  // ── Tauri window close guard ───────────────────────────────────
  onMount(async () => {
    if (!isTauriAvailable()) return;

    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const win = getCurrentWindow();

      unlisten = await win.onCloseRequested(async (event) => {
        if (!isDirty) return; // Allow close

        event.preventDefault();

        const shouldClose = await confirmStore.prompt({
          title,
          description,
          confirmText: discardText,
          cancelText: stayText,
          variant: "destructive",
        });

        if (shouldClose) {
          isDirty = false;
          await win.close();
        }
      });
    } catch {
      // Tauri not available or capability not granted — silently skip
    }
  });

  onDestroy(() => {
    unlisten?.();
    unlisten = null;
  });

  return {
    get isDirty() {
      return isDirty;
    },
    setDirty(dirty: boolean) {
      isDirty = dirty;
    },
    /** Mark the form as clean (e.g., after a successful save) */
    markClean() {
      isDirty = false;
    },
    /** Mark the form as dirty (e.g., after any field change) */
    markDirty() {
      isDirty = true;
    },
  };
}
