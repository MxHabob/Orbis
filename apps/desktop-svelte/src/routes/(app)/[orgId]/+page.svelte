<!--
  Overview / Dashboard page.
  Renders the Midday-style widgets dashboard.
-->
<script lang="ts">
import OverviewView from "$lib/features/dashboard/components/dashboard-widgets.svelte";
import type { LayoutData } from "./$types";
import * as m from "$lib/paraglide/messages";

let { data }: { data: LayoutData } = $props();
</script>

<svelte:head>
	<title>{m.page_title_dashboard()}</title>
</svelte:head>

<div class="h-full w-full ">
	<OverviewView orgId={data.orgId} />
</div>
