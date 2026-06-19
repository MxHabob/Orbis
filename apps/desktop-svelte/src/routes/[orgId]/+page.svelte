<!--
  Overview / Dashboard page.
  Renders the Midday-style widgets dashboard.
-->
<script lang="ts">
	import OverviewView from '$lib/components/widgets/index.svelte';
	import type { LayoutData } from './$types';

	let { data }: { data: LayoutData } = $props();
</script>

<svelte:head>
	<title>Dashboard | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background px-4">
	<OverviewView orgId={data.orgId} />
</div>

