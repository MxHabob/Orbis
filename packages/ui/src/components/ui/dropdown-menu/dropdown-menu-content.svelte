<script lang="ts">
import { cn, type WithoutChildrenOrChild } from "../../../utils.js";
import {
  useDirectionAwareSide,
  type PositionSide,
} from "../../../utils/direction.svelte.js";
import DropdownMenuPortal from "./dropdown-menu-portal.svelte";
import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
import type { ComponentProps } from "svelte";

let {
  ref = $bindable(null),
  sideOffset = 4,
  side = "bottom",
  align = "start",
  portalProps,
  class: className,
  ...restProps
}: Omit<DropdownMenuPrimitive.ContentProps, "side"> & {
  side?: PositionSide;
  portalProps?: WithoutChildrenOrChild<
    ComponentProps<typeof DropdownMenuPortal>
  >;
} = $props();

const resolvedSide = useDirectionAwareSide(() => side);
</script>

<DropdownMenuPortal {...portalProps}>
	<DropdownMenuPrimitive.Content
		bind:ref
		data-slot="dropdown-menu-content"
		{sideOffset}
		side={resolvedSide()}
		{align}
		class={cn(
			"bg-popover border text-popover-foreground text-start data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 min-w-32 rounded-md p-1 shadow-md duration-75 z-50 w-[var(--bits-dropdown-menu-anchor-width)] overflow-x-hidden overflow-y-auto outline-none data-closed:overflow-hidden",
			className
		)}
		{...restProps}
	/>
</DropdownMenuPortal>
