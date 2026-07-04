<!--
@component GenericListSkeleton
Generic skeleton for simple list/card pages.
Shows a header area + configurable number of list items.
-->
<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";

let { rows = 8 }: { rows?: number } = $props();
</script>

<div class="h-full w-full flex flex-col gap-4 p-6">
	<!-- Header -->
	<div class="flex items-center justify-between shrink-0">
		<Skeleton class="h-6 w-40" />
		<Skeleton class="h-9 w-32 rounded-md" />
	</div>

	<!-- List Items -->
	<div class="flex flex-col gap-2">
		{#each { length: rows } as _, i}
			<div
				class="flex items-center gap-3 p-4 bg-card border border-border rounded-lg"
				style:opacity={1 - i * 0.06}
			>
				<Skeleton class="h-8 w-8 rounded-full shrink-0" />
				<div class="flex flex-col gap-1.5 flex-1">
					<Skeleton class="h-4 w-40" />
					<Skeleton class="h-3 w-56" />
				</div>
				<Skeleton class="h-6 w-16 rounded-full" />
			</div>
		{/each}
	</div>
</div>
