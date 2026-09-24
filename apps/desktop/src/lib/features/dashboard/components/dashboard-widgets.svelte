<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createDashboardStatsQuery } from "../queries";
import { extractErrorMessage } from "$lib/api";
import WelcomeSection from "./welcome-section.svelte";
import QuickActions from "./quick-actions.svelte";
import WidgetCards from "./widget-cards.svelte";
import DesktopBanner from "./desktop-banner.svelte";

import { DashboardSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { SetupProgress } from "$lib/features/setup-cards";

let { orgId }: { orgId: string } = $props();

const statsQuery = createDashboardStatsQuery(() => orgId);
const stats = $derived(statsQuery.data ?? null);
const isLoading = $derived(statsQuery.isLoading);
const error = $derived(statsQuery.error ? extractErrorMessage(statsQuery.error) : null);
</script>

<div class="flex flex-col min-h-[calc(100vh-120px)] max-w-6xl mx-auto w-full px-4 sm:px-6 py-6 gap-6">
	{#if isLoading}
		<DashboardSkeleton />
	{:else if error}
		<ErrorState 
			title={m.common_error_load_overview()} 
			message={error} 
			onRetry={() => statsQuery.refetch()} 
		/>
	{:else}
		<div class="flex flex-col items-center text-center pt-2 pb-6 w-full">
			<WelcomeSection type="both" {stats} {orgId} />
		</div>
		<SetupProgress {orgId} />
		<QuickActions {orgId} />
		<WidgetCards {stats} {orgId} />
		<DesktopBanner />
	{/if}
</div>
