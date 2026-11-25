<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatPercent } from "$lib/core/i18n";
import type { DashboardCountsDto } from "$lib/api/types";

let { stats, orgId }: { stats: DashboardCountsDto | null; orgId: string } =
  $props();

const studentVal = $derived(formatNumber(stats?.total_students ?? 0));
const sectionsVal = $derived(formatNumber(stats?.active_sections ?? 0));
const studentDetail = $derived(
  m.common_card_detail_total_enrolled({ count: stats?.enrolled_students ?? 0 }),
);

const staffVal = $derived(formatNumber(stats?.total_employees ?? 0));
const staffDetail = $derived(m.common_card_detail_admin_roles());

const enrollmentVal = $derived(formatPercent(stats?.enrollment_rate ?? 0));
const enrollmentDetail = $derived(
  m.common_card_detail_total_enrolled({ count: stats?.enrolled_students ?? 0 }),
);
</script>

{#snippet card(label: string, href: string, value: string, detail?: string)}
	<a
		href={localizeHref(href)}
		class="card-tactile h-full flex flex-col justify-between cursor-pointer group min-h-[110px] !p-5"
	>
		<span class="text-xs text-muted-foreground font-sans">{label}</span>
		<div class="mt-3">
			<span class="text-xl font-semibold tracking-tight font-sans">{value}</span>
			{#if detail}
				<div class="text-[10px] text-muted-foreground mt-1 font-sans">{detail}</div>
			{/if}
		</div>
	</a>
{/snippet}

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 w-full">
	{@render card(m.common_promo_total(), `/${orgId}/students`, studentVal, studentDetail)}
	{@render card(m.common_card_active_classes(), `/${orgId}/academics/sections`, sectionsVal)}
	{@render card(m.common_card_staff_members(), `/${orgId}/settings`, staffVal, staffDetail)}
	{@render card(m.common_card_enrollment_rate(), `/${orgId}/students`, enrollmentVal, enrollmentDetail)}
</div>
