<!--
@component SettingsFormSkeleton
Skeleton matching the standard settings sub-page layout:
page header + card section with form fields.
Used during initial data load of any settings page.
-->
<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
	<div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
		<!-- Page Header -->
		<div class="flex flex-col gap-2 text-start">
			<Skeleton class="h-6 w-48" />
			<Skeleton class="h-3.5 w-72" />
		</div>

		<!-- Card Section -->
		<div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
			<!-- Section Title -->
			<div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
				<Skeleton class="h-4 w-36" />
			</div>

			<!-- Form Grid -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				{#each { length: 6 } as _}
					<div class="flex flex-col gap-2">
						<Skeleton class="h-3.5 w-24" />
						<Skeleton class="h-9 w-full rounded-md" />
					</div>
				{/each}
			</div>
		</div>

		<!-- Second Card Section -->
		<div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
			<div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
				<Skeleton class="h-4 w-32" />
			</div>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				{#each { length: 4 } as _}
					<div class="flex flex-col gap-2">
						<Skeleton class="h-3.5 w-20" />
						<Skeleton class="h-9 w-full rounded-md" />
					</div>
				{/each}
			</div>
		</div>

		<!-- Action Buttons -->
		<div class="flex justify-end gap-3 pb-8">
			<Skeleton class="h-9 w-28 rounded-md" />
		</div>
	</div>
</div>
