<script lang="ts">
import { commands } from "$lib/core/ipc";
import WelcomeSection from "./welcome-section.svelte";
import QuickActions from "./quick-actions.svelte";
import WidgetCards from "./widget-cards.svelte";
import DesktopBanner from "./desktop-banner.svelte";
import OverviewSkeleton from "./overview-skeleton.svelte";
import * as m from "$lib/paraglide/messages";

let { orgId }: { orgId: string } = $props();

let stats = $state<any>(null);
let isLoading = $state(true);
let error = $state<string | null>(null);

async function fetchStats() {
  isLoading = true;
  error = null;
  try {
    stats = await commands.organization.getDashboardStats(orgId);
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    console.error("Error loading dashboard stats:", e);
  } finally {
    isLoading = false;
  }
}

// Fetch statistics whenever the organization ID changes
$effect(() => {
  if (orgId) {
    fetchStats();
  }
});
</script>

<div class="flex flex-col justify-center min-h-[calc(100vh-120px)] max-w-3xl mx-auto w-full select-none">
	{#if isLoading}
		<div class="flex flex-col items-center text-center pt-6 pb-10 w-full">
			<WelcomeSection type="greeting" {orgId} />
			<div class="mt-0 w-full">
				<OverviewSkeleton type="summary"/>
			</div>
		</div>
		<QuickActions {orgId} />
		<OverviewSkeleton type="cards" />
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-sm">{m.error_load_overview()}</h3>
			<p class="text-xs text-muted-foreground">{error}</p>
			<button onclick={fetchStats} class="text-xs underline cursor-pointer mt-2 hover:text-foreground">{m.error_retry()}</button>
		</div>
	{:else}
		<div class="flex flex-col items-center text-center pt-6 pb-10 w-full">
			<WelcomeSection type="both" {stats} {orgId} />
		</div>
		<QuickActions {orgId} />
		<WidgetCards {stats} {orgId} />
		<DesktopBanner />
	{/if}
</div>

