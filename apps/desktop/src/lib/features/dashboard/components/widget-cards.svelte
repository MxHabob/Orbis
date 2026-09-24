<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatPercent } from "$lib/core/i18n";
import type { DashboardCountsDto } from "$lib/api/types";

let { stats, orgId }: { stats: DashboardCountsDto | null; orgId: string } = $props();

const studentVal = $derived(formatNumber(stats?.total_students ?? 0));
const studentDetail = $derived(
  m.common_card_detail_across_classes({ count: stats?.total_classes ?? 0 }),
);

const attendanceVal = $derived(
  formatPercent(stats?.attendance_overview?.present_rate ?? 0),
);
const attendanceDetail = $derived(
  m.common_card_detail_late_absent({
    lateRate: stats?.attendance_overview?.late_rate ?? 0,
    absentRate: stats?.attendance_overview?.absent_rate ?? 0,
  }),
);

const classesVal = $derived(formatNumber(stats?.total_classes ?? 0));
const classesDetail = $derived(
  m.common_card_detail_managed_by({ count: stats?.total_teachers ?? 0 }),
);

const behaviorPoints = $derived(stats?.behavior_summary?.net_points ?? 0);
const behaviorVal = $derived(
  `${behaviorPoints > 0 ? "+" : ""}${formatNumber(behaviorPoints)}`,
);
const behaviorDetail = $derived(
  m.common_card_detail_critical_incidents({
    count: stats?.behavior_summary?.critical_count ?? 0,
  }),
);

const staffVal = $derived(formatNumber(stats?.total_staff ?? 0));
const staffDetail = $derived(m.common_card_detail_admin_roles());

const enrollmentVal = $derived(formatPercent(stats?.enrollment_rate ?? 0));
const enrollmentDetail = $derived(
  m.common_card_detail_total_enrolled({ count: stats?.total_enrollments ?? 0 }),
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
	{@render card(m.common_card_attendance_rate(), `/${orgId}/reports`, attendanceVal, attendanceDetail)}
	{@render card(m.common_card_active_classes(), `/${orgId}/academics`, classesVal, classesDetail)}
	{@render card(m.common_card_behavior_points(), `/${orgId}/students`, behaviorVal, behaviorDetail)}
	{@render card(m.common_card_staff_members(), `/${orgId}/settings`, staffVal, staffDetail)}
	{@render card(m.common_card_enrollment_rate(), `/${orgId}/students`, enrollmentVal, enrollmentDetail)}
</div>

