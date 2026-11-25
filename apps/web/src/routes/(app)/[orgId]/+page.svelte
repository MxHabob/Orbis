<!--
  Overview / Dashboard page.
  Renders the canonical DashboardPage feature component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { DashboardPage } from "$lib/features/dashboard";
import type { LayoutData } from "./$types";

let { data }: { data: LayoutData } = $props();
</script>

<svelte:head>
	<title>{m.common_page_title_dashboard()}</title>
</svelte:head>

<div class="h-full w-full min-h-0 flex-1">
	<DashboardPage orgId={data.orgId} />
</div>
