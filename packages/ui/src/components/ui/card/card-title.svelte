<script lang="ts">
import type { HTMLAttributes } from "svelte/elements";
import { cn, type WithElementRef } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  children,
  ...restProps
}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-title"
	class={cn("text-[11px] font-bold text-muted-foreground tracking-wider uppercase flex-1 leading-snug", className)}
	{...restProps}
>
	{@render children?.()}
</div>
