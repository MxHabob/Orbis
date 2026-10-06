<!--
  @component DashboardPage
  Canonical feature page for Organization Overview / Dashboard.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createDashboardStatsQuery } from "./queries";
import { extractErrorMessage } from "$lib/api";
import WelcomeSection from "./components/welcome-section.svelte";
import QuickActions from "./components/quick-actions.svelte";
import WidgetCards from "./components/widget-cards.svelte";
import DesktopBanner from "./components/desktop-banner.svelte";
import { DashboardSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";

let { orgId }: { orgId: string } = $props();

const statsQuery = createDashboardStatsQuery(() => orgId);
const stats = $derived(statsQuery.data ?? null);
const isLoading = $derived(statsQuery.isLoading);
const error = $derived(statsQuery.error ? extractErrorMessage(statsQuery.error) : null);
</script>

<div class="h-full min-h-0 flex-1 overflow-y-auto">
	<div class="mx-auto flex min-h-full w-full max-w-3xl flex-col gap-6 px-4 pb-8 pt-14 sm:px-6">
		{#if isLoading}
			<DashboardSkeleton />
		{:else if error}
			<ErrorState
				title={m.common_error_load_overview()}
				message={error}
				onRetry={() => statsQuery.refetch()}
			/>
		{:else}
			<div class="flex w-full flex-col items-center pb-2 pt-2 text-center">
				<WelcomeSection type="both" {stats} {orgId} />
			</div>
			<QuickActions {orgId} />
			<WidgetCards {stats} {orgId} />
			<DesktopBanner />
		{/if}
	</div>
</div>
