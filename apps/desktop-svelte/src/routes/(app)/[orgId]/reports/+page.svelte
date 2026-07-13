<!--
  Reports / Analytics dashboard.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import MetricsView from "$lib/features/metrics/metrics-view.svelte";
import type { PageData } from "./$types";

let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>{m.common_page_title_reports()}</title>
</svelte:head>

<div class="h-full w-full overflow-auto mx-4">
	<div class="mx-auto w-full">
		<MetricsView orgId={data.orgId} />
	</div>
</div>
