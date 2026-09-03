<!--
  Overview / Dashboard page.
  Renders the Midday-style widgets dashboard.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import OverviewView from "$lib/features/dashboard/components/dashboard-widgets.svelte";
import type { LayoutData } from "./$types";

let { data }: { data: LayoutData } = $props();
</script>

<svelte:head>
	<title>{m.common_page_title_dashboard()}</title>
</svelte:head>

<div class="h-full w-full ">
	<OverviewView orgId={data.orgId} />
</div>
