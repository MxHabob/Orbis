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
	class={cn("bg-muted h-1 rounded-full relative flex w-full items-center overflow-x-hidden shadow-[inset_0_1px_2px_0_oklch(0.3_0.03_285_/_0.06)] dark:shadow-[inset_0_1px_2px_0_oklch(0_0_0_/_0.25)]", className)}
	{value}
	{max}
	{...restProps}
>
	<div
		data-slot="progress-indicator"
		class="size-full flex-1 rounded-full transition-all"
		style="transform: translateX(-{100 - (100 * (value ?? 0)) / (max ?? 1)}%); background: linear-gradient(90deg, var(--color-primary), var(--color-brand-lime));"
	></div>
</ProgressPrimitive.Root>
