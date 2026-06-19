<script lang="ts">
import { Progress as ProgressPrimitive } from "bits-ui";
import { cn, type WithoutChildrenOrChild } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  max = 100,
  value,
  ...restProps
}: WithoutChildrenOrChild<ProgressPrimitive.RootProps> = $props();
</script>

<ProgressPrimitive.Root
	bind:ref
	data-slot="progress"
	class={cn("bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden shadow-[inset_0_1px_2px_0_hsl(267_30%_30%/0.08)] dark:shadow-[inset_0_1px_2px_0_hsl(0_0%_0%/0.3)]", className)}
	{value}
	{max}
	{...restProps}
>
	<div
		data-slot="progress-indicator"
		class="size-full flex-1 rounded-full transition-all"
		style="transform: translateX(-{100 - (100 * (value ?? 0)) / (max ?? 1)}%); background: linear-gradient(90deg, hsl(248 50% 56%), hsl(80 85% 62%));"
	></div>
</ProgressPrimitive.Root>
