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
import MulverseIntroDialog from "./components/mulverse-intro-dialog.svelte";
import { DashboardSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { authStore, effectiveContext } from "$lib/stores";

let { orgId }: { orgId: string } = $props();

const { stats: statsQuery, access } = createDashboardStatsQuery(
  () => orgId,
  () => authStore.userId,
  () => effectiveContext.activeOrgUnitId,
);
const stats = $derived(statsQuery.data ?? null);
const isLoading = $derived(access.isLoading || statsQuery.isLoading);
const error = $derived(
  access.error || statsQuery.error
    ? extractErrorMessage(access.error ?? statsQuery.error)
    : null,
);
</script>

<div class="h-full min-h-0 flex-1 overflow-y-auto [container-type:size]">
	<div class="mx-auto flex w-full max-w-3xl flex-col gap-[clamp(0.75rem,2.7cqh,1.5rem)] px-4 pt-[clamp(0.75rem,calc(12cqh-2.75rem),3.5rem)] pb-[clamp(0.75rem,3cqh,2rem)] sm:px-6">
		{#if isLoading}
			<DashboardSkeleton />
		{:else if error}
			<ErrorState
				title={m.common_error_load_overview()}
				message={error}
				onRetry={() => access.isError ? access.refetch() : statsQuery.refetch()}
			/>
		{:else}
			<div class="flex w-full flex-col items-center text-center">
				<WelcomeSection type="greeting" {stats} {orgId} />
			</div>
			<QuickActions {orgId} />
			<WidgetCards {stats} {orgId} />
		{/if}
	</div>
</div>
{#if !isLoading && !error}
	<DesktopBanner />
	<MulverseIntroDialog
		{orgId}
		hasOperationalActivity={Boolean(
			stats &&
				(stats.active_sections > 0 ||
					stats.total_students > 0 ||
					stats.total_employees > 0),
		)}
	/>
{/if}
