<!--
  Reports / Analytics dashboard.
-->
<script lang="ts">
import MetricsView from "$lib/components/metrics/metrics-view.svelte";
import type { PageData } from "./$types";
import * as m from "$lib/paraglide/messages";

let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>{m.page_title_reports()}</title>
</svelte:head>

<div class="h-full w-full overflow-auto">
	<div class="max-w-6xl mx-auto w-full">
		<MetricsView orgId={data.orgId} />
	</div>
</div>
