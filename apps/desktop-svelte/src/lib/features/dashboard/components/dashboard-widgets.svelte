<script lang="ts">
import { commands } from "$lib/core/ipc";
import WelcomeSection from "./welcome-section.svelte";
import QuickActions from "./quick-actions.svelte";
import WidgetCards from "./widget-cards.svelte";
import DesktopBanner from "./desktop-banner.svelte";
import * as common from "$lib/paraglide/messages/common";
import { DashboardSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";

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
		<DashboardSkeleton />
	{:else if error}
		<ErrorState 
			title={common.error_load_overview()} 
			message={error} 
			onRetry={fetchStats} 
		/>
	{:else}
		<div class="flex flex-col items-center text-center pt-6 pb-10 w-full">
			<WelcomeSection type="both" {stats} {orgId} />
		</div>
		<QuickActions {orgId} />
		<WidgetCards {stats} {orgId} />
		<DesktopBanner />
	{/if}
</div>
