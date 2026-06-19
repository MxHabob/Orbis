<script lang="ts">
	import { X } from "lucide-svelte";
	import { onMount } from "svelte";
	import { fly } from "svelte/transition";

	let dismissed = $state(true); // Default to true to prevent hydration mismatch/flash
	let mounted = $state(false);

	onMount(() => {
		const saved = localStorage.getItem("orbis_desktop_banner_dismissed");
		dismissed = saved === "true";
		mounted = true;
	});

	function handleDismiss(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		dismissed = true;
		localStorage.setItem("orbis_desktop_banner_dismissed", "true");
	}
</script>

{#if mounted && !dismissed}
	<div
		in:fly={{ y: 40, opacity: 0, duration: 500, delay: 500 }}
		out:fly={{ y: 20, opacity: 0, duration: 300 }}
		class="fixed bottom-2.5 right-4 z-50 group"
	>
		<div class="relative border border-border bg-background shadow-lg w-[300px] overflow-hidden rounded-xl">
			<button
				type="button"
				onclick={handleDismiss}
				class="absolute top-2.5 right-2.5 z-10 text-muted-foreground hover:text-foreground p-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 focus:outline-none"
			>
				<X size={14} />
			</button>

			<div class="relative h-[100px] flex items-center justify-center border-b border-border bg-muted/10">
				<!-- Light mode dot pattern grid -->
				<div
					class="absolute inset-0 dark:hidden pointer-events-none"
					style="background-image: radial-gradient(circle, rgba(0,0,0,0.08) 1px, transparent 1px); background-size: 12px 12px;"
				></div>
				<!-- Dark mode dot pattern grid -->
				<div
					class="absolute inset-0 hidden dark:block pointer-events-none"
					style="background-image: radial-gradient(circle, rgba(255,255,255,0.07) 1px, transparent 1px); background-size: 12px 12px;"
				></div>

				<div class="relative z-10 flex items-center justify-center gap-3 w-full px-4">
					<!-- OS platforms row -->
					<div class="flex items-center justify-center size-9 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary transition-all duration-300 shadow-sm cursor-help" title="macOS compatibility">
						<svg class="size-5 fill-current" viewBox="0 0 170 170">
							<path d="M150.37 130.25c-2.45 5.66-5.35 10.87-8.71 15.66-4.58 6.53-8.33 11.05-11.22 13.56-4.48 4.12-9.28 6.23-14.42 6.35-3.69 0-8.14-1.05-13.32-3.18-5.19-2.12-9.97-3.17-14.34-3.17-4.58 0-9.49 1.05-14.75 3.17-5.26 2.13-9.5 3.24-12.74 3.35-4.37.13-9.13-1.9-14.3-6.08-3.35-2.75-7.18-7.39-11.53-13.93-7.61-11.14-13.38-23.74-17.32-37.76-4.17-14.86-6.27-29.13-6.27-42.83 0-15.53 3.77-28.56 11.3-39.09 7.53-10.53 17.26-15.8 29.17-15.8 5.7 0 11.75 1.48 18.13 4.43 6.38 2.95 10.42 4.42 12.11 4.42 1.34 0 5.48-1.4 12.42-4.21 6.93-2.82 13.11-4.17 18.52-4.07 15.86.37 28.16 6.13 36.92 17.3-13.32 8.12-19.86 19.38-19.64 33.78.22 11.53 4.54 21.2 12.98 28.98 8.44 7.78 18.17 11.9 29.17 12.35-.9 3.74-2.37 8.6-4.4 14.54zM119.03 26.54c0-7.83 2.8-15.11 8.41-21.84 5.61-6.73 12.43-10.48 20.46-11.25.13 1.13.2 2.06.2 2.8 0 7.61-2.91 14.86-8.71 21.75-5.8 6.89-12.82 10.55-21.05 11-1.34-1.33-1.31-2.46-1.31-2.46z"/>
						</svg>
					</div>
					<div class="flex items-center justify-center size-9 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary transition-all duration-300 shadow-sm cursor-help" title="Windows compatibility">
						<svg class="size-4.5 fill-current" viewBox="0 0 88 88">
							<path d="M0 12.402l35.687-4.86.016 34.46-35.703.094zm0 32.553l35.687.094-.016 34.703-35.671-4.906zm38.86-37.766l49.14-7.189v44.82l-49.14.328zm0 40.812l49.14.32v45.242l-49.14-7.078z"/>
						</svg>
					</div>
					<div class="flex items-center justify-center size-9 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary transition-all duration-300 shadow-sm cursor-help" title="Linux support">
						<svg class="size-5 stroke-current fill-none" viewBox="0 0 24 24" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<rect width="20" height="16" x="2" y="4" rx="2"/>
							<path d="m7 8 3 3-3 3M13 12h4"/>
						</svg>
					</div>
					<div class="flex items-center justify-center size-9 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary transition-all duration-300 shadow-sm cursor-help" title="Web sync enabled">
						<svg class="size-5 stroke-current fill-none" viewBox="0 0 24 24" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="12" cy="12" r="10"/>
							<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20M2 12h20"/>
						</svg>
					</div>
				</div>
			</div>

			<div class="block p-4 select-none">
				<h3 class="text-xs font-semibold mb-1 tracking-wide">
					Orbis Native Shell Enabled
				</h3>
				<p class="text-[10px] text-muted-foreground leading-normal">
					Supercharged desktop environment. Keyboard shortcuts (⌘K) and local database queries are fully operational.
				</p>
			</div>
		</div>
	</div>
{/if}
