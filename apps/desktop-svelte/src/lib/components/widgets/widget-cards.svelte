<script lang="ts">
	let { stats, orgId }: { stats: any; orgId: string } = $props();

	const studentVal = $derived(String(stats?.total_students ?? 0));
	const studentDetail = $derived(`across ${stats?.total_classes ?? 0} active classes`);

	const attendanceVal = $derived(`${stats?.attendance_overview?.present_rate ?? 0}%`);
	const attendanceDetail = $derived(
		`Late: ${stats?.attendance_overview?.late_rate ?? 0}% | Absent: ${stats?.attendance_overview?.absent_rate ?? 0}%`
	);

	const classesVal = $derived(String(stats?.total_classes ?? 0));
	const classesDetail = $derived(`Managed by ${stats?.total_teachers ?? 0} teachers`);

	const behaviorPoints = $derived(stats?.behavior_summary?.net_points ?? 0);
	const behaviorVal = $derived(`${behaviorPoints > 0 ? "+" : ""}${behaviorPoints}`);
	const behaviorDetail = $derived(`${stats?.behavior_summary?.critical_count ?? 0} critical incidents`);

	const staffVal = $derived(String(stats?.total_staff ?? 0));
	const staffDetail = $derived("Supporting administrative roles");

	const enrollmentVal = $derived(`${stats?.enrollment_rate ?? 0}%`);
	const enrollmentDetail = $derived(`Total enrolled: ${stats?.total_enrollments ?? 0}`);
</script>

{#snippet card(label: string, href: string, value: string, detail?: string)}
	<a
		{href}
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
	{@render card("Total Students", `/${orgId}/students`, studentVal, studentDetail)}
	{@render card("Attendance Rate", `/${orgId}/reports`, attendanceVal, attendanceDetail)}
	{@render card("Active Classes", `/${orgId}/academics`, classesVal, classesDetail)}
	{@render card("Behavior Points", `/${orgId}/students`, behaviorVal, behaviorDetail)}
	{@render card("Staff Members", `/${orgId}/settings`, staffVal, staffDetail)}
	{@render card("Enrollment Rate", `/${orgId}/students`, enrollmentVal, enrollmentDetail)}
</div>
