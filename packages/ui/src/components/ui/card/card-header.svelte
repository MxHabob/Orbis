<script lang="ts">
import { cn, type WithElementRef } from "../../../utils.js";
import type { HTMLAttributes } from "svelte/elements";

let {
  ref = $bindable(null),
  class: className,
  children,
  ...restProps
}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-header"
	class={cn(
		"bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none flex items-center justify-between gap-2 shrink-0",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
