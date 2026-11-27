<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatPercent } from "$lib/core/i18n";
import type { DashboardCountsDto } from "$lib/api/types";
import { createOrgUnitsQuery } from "$lib/features/settings/queries";

let { stats, orgId }: { stats: DashboardCountsDto | null; orgId: string } =
  $props();

const studentVal = $derived(formatNumber(stats?.total_students ?? 0));
const sectionsVal = $derived(formatNumber(stats?.active_sections ?? 0));
const staffVal = $derived(formatNumber(stats?.total_employees ?? 0));

const unitsQuery = createOrgUnitsQuery(() => orgId);
const branchVal = $derived(
  unitsQuery.data
    ? formatNumber(
        unitsQuery.data.filter(
          (unit) => unit.unit_type === "branch" && unit.is_active,
        ).length,
      )
    : "—",
);
const branchDetail = $derived(
  unitsQuery.data
    ? undefined
    : unitsQuery.isError
      ? m.common_loading_error()
      : m.common_loading(),
);

const enrollmentVal = $derived(formatPercent(stats?.enrollment_rate ?? 0));
const enrollmentDetail = $derived(
  m.common_card_detail_total_enrolled({ count: stats?.enrolled_students ?? 0 }),
);
</script>

{#snippet card(label: string, href: string, value: string, detail?: string)}
	<a
		href={localizeHref(href)}
		class="card-tactile flex min-w-0 flex-col justify-between cursor-pointer p-[clamp(0.75rem,3.7cqh,1.25rem)]"
	>
		<span class="text-xs text-muted-foreground font-sans">{label}</span>
		<div class="mt-2">
			<span class="text-xl font-semibold tracking-tight font-sans break-words">{value}</span>
			{#if detail}
				<div class="text-[10px] text-muted-foreground mt-1 font-sans">{detail}</div>
			{/if}
		</div>
	</a>
{/snippet}

<div class="grid grid-cols-1 @min-[32rem]/workspace:grid-cols-2 @min-[48rem]/workspace:grid-cols-3 gap-3 w-full">
	{@render card(m.common_promo_total(), `/${orgId}/students`, studentVal)}
	{@render card(m.common_card_active_classes(), `/${orgId}/academics/sections`, sectionsVal)}
	{@render card(m.common_card_staff_members(), `/${orgId}/employees`, staffVal)}
	{@render card(m.common_card_enrollment_rate(), `/${orgId}/students`, enrollmentVal, enrollmentDetail)}
	{@render card(m.settings_nav_subjects(), `/${orgId}/settings/subjects`, formatNumber(stats?.total_subjects ?? 0))}
	{@render card(m.settings_ui_branch_active_count(), `/${orgId}/settings/branches`, branchVal, branchDetail)}
</div>
