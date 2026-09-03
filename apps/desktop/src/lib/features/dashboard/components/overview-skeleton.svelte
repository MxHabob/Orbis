<script lang="ts">
import { Skeleton } from "@orbis/ui";

let { type = "summary" }: { type?: "summary" | "cards" } = $props();
</script>

{#if type === "summary"}
	<div class="flex flex-col items-center max-w-lg w-full gap-3">
		<div class="h-10 flex items-center justify-center w-full">
			<Skeleton class="h-[14px] w-[280px]" />
		</div>
		<div class="flex items-center gap-1.5 justify-center w-full">
			{#each Array(5) as _}
				<div class="h-[2px] w-4 rounded-full bg-primary/10"></div>
			{/each}
		</div>
	</div>
{:else if type === "cards"}
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 w-full">
		{#each Array(6) as _}
			<div class="border border-border p-5 min-h-[110px] flex flex-col justify-between rounded-xl bg-card/50">
				<Skeleton class="h-[14px] w-[80px]" />
				<Skeleton class="h-[24px] w-[120px] mt-3" />
			</div>
		{/each}
	</div>
{/if}
