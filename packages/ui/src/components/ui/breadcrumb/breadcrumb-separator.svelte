<script lang="ts">
import { cn, type WithElementRef } from "../../../utils.js";
import type { HTMLLiAttributes } from "svelte/elements";
import { ChevronRight } from "@orbis/icons";

let {
  ref = $bindable(null),
  class: className,
  children,
  ...restProps
}: WithElementRef<HTMLLiAttributes> = $props();
</script>

<li
	bind:this={ref}
	data-slot="breadcrumb-separator"
	role="presentation"
	aria-hidden="true"
	class={cn("[&>svg]:size-3.5", className)}
	{...restProps}
>
	{#if children}
		{@render children?.()}
	{:else}
		<ChevronRight  />
	{/if}
</li>
