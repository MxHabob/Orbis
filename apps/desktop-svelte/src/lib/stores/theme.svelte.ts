// ── Theme Store ──────────────────────────────────────────────────────
// Svelte 5 Runes-based theme management (light/dark/system).

type Theme = "light" | "dark" | "system";

class ThemeStore {
	current = $state<Theme>("system");

	resolved = $derived<"light" | "dark">(() => {
		if (this.current !== "system") return this.current;
		if (typeof window === "undefined") return "light";
		return window.matchMedia("(prefers-color-scheme: dark)").matches
			? "dark"
			: "light";
	});

	constructor() {
		this.restore();
	}

	set(theme: Theme) {
		this.current = theme;
		localStorage.setItem("orbis_theme", theme);
		this.applyToDOM();
	}

	toggle() {
		const resolved =
			this.current === "system"
				? window.matchMedia("(prefers-color-scheme: dark)").matches
					? "dark"
					: "light"
				: this.current;
		this.set(resolved === "dark" ? "light" : "dark");
	}

	restore() {
		try {
			const saved = localStorage.getItem("orbis_theme") as Theme | null;
			if (saved) {
				this.current = saved;
			}
		} catch {
			// SSR or localStorage unavailable
		}
	}

	applyToDOM() {
		if (typeof document === "undefined") return;
		const resolved =
			this.current === "system"
				? window.matchMedia("(prefers-color-scheme: dark)").matches
					? "dark"
					: "light"
				: this.current;
		document.documentElement.classList.toggle("dark", resolved === "dark");
	}
}

export const themeStore = new ThemeStore();
