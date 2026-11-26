<script lang="ts">
import { Popover as PopoverPrimitive } from "bits-ui";
import PopoverPortal from "./popover-portal.svelte";
import { cn, type WithoutChildrenOrChild } from "../../../utils.js";
import {
  useDirectionAwareSide,
  type PositionSide,
} from "../../../utils/direction.svelte.js";
import type { ComponentProps } from "svelte";

let {
  ref = $bindable(null),
  class: className,
  sideOffset = 4,
  side = "bottom",
  align = "center",
  portalProps,
  ...restProps
}: Omit<PopoverPrimitive.ContentProps, "side"> & {
  side?: PositionSide;
  portalProps?: WithoutChildrenOrChild<ComponentProps<typeof PopoverPortal>>;
} = $props();

const resolvedSide = useDirectionAwareSide(() => side);
</script>

<PopoverPortal {...portalProps}>
	<PopoverPrimitive.Content
		bind:ref
		data-slot="popover-content"
		{sideOffset}
		side={resolvedSide()}
		{align}
		class={cn(
			"bg-popover border text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 flex flex-col gap-2.5 rounded-md p-2.5 text-sm shadow-md duration-75 z-50 w-72 origin-(--transform-origin) outline-hidden",
			className
		)}
		{...restProps}
	/>
</PopoverPortal>
