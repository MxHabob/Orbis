<script lang="ts">
import type { HTMLTableAttributes } from "svelte/elements";
import { cn, type WithElementRef } from "../../../utils.js";

type Props = WithElementRef<HTMLTableAttributes> & {
  containerClass?: string;
  stickyHeader?: boolean;
};

let {
  ref = $bindable(null),
  class: className,
  children,
  containerClass,
  stickyHeader = false,
  ...restProps
}: Props = $props();
</script>

<div data-slot="table-container" class={cn("relative w-full min-w-0 max-w-full overflow-auto", containerClass)}>
	<table bind:this={ref} data-slot="table" class={cn("w-full caption-bottom text-sm", stickyHeader && "[&>[data-slot=table-header]]:sticky [&>[data-slot=table-header]]:top-0 [&>[data-slot=table-header]]:z-10 [&>[data-slot=table-header]]:bg-card", className)} {...restProps}>
		{@render children?.()}
	</table>
</div>
