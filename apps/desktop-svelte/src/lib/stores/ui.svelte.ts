// ── UI Store ─────────────────────────────────────────────────────────
// Svelte 5 Runes-based UI state for layout management.
// Manages sidebar, panel widths, and global loading state.

class UIStore {
	globalLoading = $state(false);

	// Panel resize preferences are persisted via Resizer component
	// to localStorage with `resizer:{persistId}` keys.

	setGlobalLoading(loading: boolean) {
		this.globalLoading = loading;
	}
}

export const uiStore = new UIStore();
