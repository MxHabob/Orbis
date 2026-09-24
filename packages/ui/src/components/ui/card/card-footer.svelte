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
	data-slot="card-footer"
	class={cn(
		"bg-muted/10 px-6 py-3 border-t border-border flex items-center justify-start gap-2 shrink-0",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
