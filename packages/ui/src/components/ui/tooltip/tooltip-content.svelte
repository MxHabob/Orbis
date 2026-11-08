<script lang="ts">
import { Tooltip as TooltipPrimitive } from "bits-ui";
import { cn } from "../../../utils.js";
import {
  useDirectionAwareSide,
  type PositionSide,
} from "../../../utils/direction.svelte.js";
import TooltipPortal from "./tooltip-portal.svelte";
import type { ComponentProps } from "svelte";
import type { WithoutChildrenOrChild } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  sideOffset = 0,
  side = "top",
  children,
  arrowClasses,
  portalProps,
  ...restProps
}: Omit<TooltipPrimitive.ContentProps, "side"> & {
  side?: PositionSide;
  arrowClasses?: string;
  portalProps?: WithoutChildrenOrChild<ComponentProps<typeof TooltipPortal>>;
} = $props();

const resolvedSide = useDirectionAwareSide(() => side);
</script>

<TooltipPortal {...portalProps}>
	<TooltipPrimitive.Content
		bind:ref
		data-slot="tooltip-content"
		{sideOffset}
		side={resolvedSide()}
		class={cn(
			"tooltip data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-[state=delayed-open]:animate-in data-[state=delayed-open]:fade-in-0 data-[state=delayed-open]:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 inline-flex items-center gap-1.5 has-data-[slot=kbd]:pr-1.5 **:data-[slot=kbd]:relative **:data-[slot=kbd]:isolate **:data-[slot=kbd]:z-50 **:data-[slot=kbd]:rounded-sm w-fit max-w-xs origin-(--bits-tooltip-content-transform-origin)",
			className
		)}
		{...restProps}
	>
		{@render children?.()}
		<TooltipPrimitive.Arrow>
			{#snippet child({ props })}
				<div
					class={cn(
						"size-2.5 translate-y-[calc(-50%-2px)] rotate-45 rounded-[2px] bg-popover z-50",
						"data-[side=top]:border-b data-[side=top]:border-r data-[side=top]:translate-x-1/2 data-[side=top]:translate-y-[calc(-50%+2px)]",
						"data-[side=bottom]:border-t data-[side=bottom]:border-l data-[side=bottom]:-translate-x-1/2 data-[side=bottom]:-translate-y-[calc(-50%+1px)]",
						"data-[side=right]:border-b data-[side=right]:border-l data-[side=right]:translate-x-[calc(50%+2px)] data-[side=right]:translate-y-1/2",
						"data-[side=left]:border-t data-[side=left]:border-r data-[side=left]:-translate-y-[calc(50%-3px)]",
						arrowClasses
					)}
					{...props}
				></div>
			{/snippet}
		</TooltipPrimitive.Arrow>
	</TooltipPrimitive.Content>
</TooltipPortal>
