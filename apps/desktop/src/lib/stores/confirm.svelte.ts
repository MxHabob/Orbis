// ── Global Confirm Store ─────────────────────────────────────────────
// Svelte 5 Runes-based store for managing imperative global confirmation dialogs.
// This allows any part of the app to await a user confirmation seamlessly.

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
};

class ConfirmStore {
  isOpen = $state(false);
  options = $state<ConfirmOptions | null>(null);
  private resolvePromise: ((value: boolean) => void) | null = null;

  prompt(options: ConfirmOptions): Promise<boolean> {
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
}

export const confirmStore = new ConfirmStore();
