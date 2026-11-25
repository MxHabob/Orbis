<!--
@component StudentsTableSkeleton
Pixel-perfect skeleton matching the Students directory layout:
search bar + action buttons + table header + 12 data rows.
Used during initial data fetch to provide instant visual feedback.
-->
<script lang="ts">
import { Grid } from "@orbis/icons";

import { Skeleton } from "@orbis/ui/skeleton";
</script>

<div class="h-full w-full flex flex-col gap-4 p-6">
	<!-- Header: Search + Actions -->
	<div class="flex items-center justify-between shrink-0">
		<Skeleton class="h-9 w-72 rounded-md" />
		<div class="flex items-center gap-2">
			<Skeleton class="h-9 w-24 rounded-md" />
			<Skeleton class="h-9 w-24 rounded-md" />
			<Skeleton class="h-9 w-28 rounded-md" />
		</div>
	</div>

	<!-- Grid -->
	<div class="flex-1 min-h-0 bg-card border border-border rounded-lg overflow-hidden">
		<!-- Grid Header -->
		<div class="flex items-center gap-4 px-4 py-3 border-b border-border bg-muted/30">
			<Skeleton class="h-4 w-4 rounded" />
			<Skeleton class="h-3.5 w-28" />
			<Skeleton class="h-3.5 w-20" />
			<Skeleton class="h-3.5 w-40 flex-1" />
			<Skeleton class="h-3.5 w-24" />
			<Skeleton class="h-3.5 w-16" />
		</div>

		<!-- Grid Rows -->
		{#each { length: 12 } as _, i}
			<div
				class="flex items-center gap-4 px-4 py-3 border-b border-border/30"
				style:opacity={1 - i * 0.05}
			>
				<Skeleton class="h-4 w-4 rounded" />
				<Skeleton class="h-4 w-32" />
				<Skeleton class="h-4 w-20" />
				<Skeleton class="h-4 w-48 flex-1" />
				<Skeleton class="h-4 w-20" />
				<Skeleton class="h-4 w-16" />
			</div>
		{/each}
	</div>
</div>
