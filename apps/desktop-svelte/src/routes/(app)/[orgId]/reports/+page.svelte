<!--
  Reports / Analytics dashboard.
-->
<script lang="ts">
import MetricsView from "$lib/features/metrics/metrics-view.svelte";
import type { PageData } from "./$types";
import * as common from "$lib/paraglide/messages/common";

let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>{common.page_title_reports()}</title>
</svelte:head>

<div class="h-full w-full overflow-auto mx-4">
	<div class="mx-auto w-full">
		<MetricsView orgId={data.orgId} />
	</div>
</div>
