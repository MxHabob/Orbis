<script lang="ts">
import {
  cn,
  type WithElementRef,
  type WithoutChildren,
} from "../../../utils.js";
import type { HTMLAttributes } from "svelte/elements";

let {
  ref = $bindable(null),
  class: className,
  animate = true,
  ...restProps
}: WithoutChildren<WithElementRef<HTMLAttributes<HTMLDivElement>>> & {
  animate?: boolean;
} = $props();
</script>

<div
	bind:this={ref}
	data-slot="skeleton"
	class={cn(
		"relative overflow-hidden",
		"bg-gradient-to-r from-transparent via-primary/10 to-transparent dark:via-primary/10",
		"bg-[length:200%_100%]",
		"rounded-none",
		animate && "animate-shimmer",
		className
	)}
	{...restProps}
></div>
