<script lang="ts">
	import type { Snippet } from "svelte";
	import type { ChartLayoutItem, ColSpan, ChartId } from "../utils/chart-types";

	let {
		layout = $bindable(),
		isEditing = false,
		children
	}: {
		layout: ChartLayoutItem[];
		isEditing?: boolean;
		children: Snippet<[ChartId, ColSpan]>;
	} = $props();

	const COL_SPAN_CLASS: Record<ColSpan, string> = {
		4: "col-span-12 md:col-span-6 lg:col-span-4",
		6: "col-span-12 md:col-span-6",
		8: "col-span-12 md:col-span-8",
		12: "col-span-12"
	};
</script>

<div class="grid grid-cols-12 gap-6 pb-20 select-none" data-metrics-grid>
	{#each layout as item (item.id)}
		<div class="{COL_SPAN_CLASS[item.colSpan]} transition-all duration-300">
			{@render children(item.id, item.colSpan)}
		</div>
	{/each}
</div>
