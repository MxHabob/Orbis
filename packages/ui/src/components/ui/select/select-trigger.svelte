<script lang="ts">
import { Select as SelectPrimitive } from "bits-ui";
import { cn, type WithoutChild } from "../../../utils.js";
import { ChevronDown as ChevronDownIcon } from "@lucide/svelte";

let {
  ref = $bindable(null),
  class: className,
  children,
  size = "default",
  ...restProps
}: WithoutChild<SelectPrimitive.TriggerProps> & {
  size?: "sm" | "default";
} = $props();
</script>

<SelectPrimitive.Trigger
	bind:ref
	data-slot="select-trigger"
	data-size={size}
	class={cn(
		"btn-tactile bg-background border border-border data-placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 gap-1.5 rounded-lg py-2 pr-2 pl-2.5 text-sm select-none data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:flex *:data-[slot=select-value]:gap-1.5 [&_svg:not([class*='size-'])]:size-4 flex w-fit cursor-pointer items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:items-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
		className
	)}
	{...restProps}
>
	{@render children?.()}
	<ChevronDownIcon class="text-muted-foreground size-4 pointer-events-none" />
</SelectPrimitive.Trigger>
