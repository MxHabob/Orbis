// ── UI Store ─────────────────────────────────────────────────────────
// Svelte 5 Runes-based UI state (sidebar, modals, loading).

class UIStore {
	sidebarOpen = $state(true);
	sidebarCollapsed = $state(false);
	globalLoading = $state(false);

	toggleSidebar() {
		this.sidebarOpen = !this.sidebarOpen;
	}

	collapseSidebar() {
		this.sidebarCollapsed = !this.sidebarCollapsed;
	}

	setGlobalLoading(loading: boolean) {
		this.globalLoading = loading;
	}
}

export const uiStore = new UIStore();
