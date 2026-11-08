<script lang="ts">
import { Select as SelectPrimitive } from "bits-ui";
import SelectPortal from "./select-portal.svelte";
import SelectScrollUpButton from "./select-scroll-up-button.svelte";
import SelectScrollDownButton from "./select-scroll-down-button.svelte";
import { cn, type WithoutChild } from "../../../utils.js";
import {
  useDirectionAwareSide,
  type PositionSide,
} from "../../../utils/direction.svelte.js";
import type { ComponentProps } from "svelte";
import type { WithoutChildrenOrChild } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  sideOffset = 4,
  side = "bottom",
  portalProps,
  children,
  preventScroll = true,
  ...restProps
}: Omit<WithoutChild<SelectPrimitive.ContentProps>, "side"> & {
  side?: PositionSide;
  portalProps?: WithoutChildrenOrChild<ComponentProps<typeof SelectPortal>>;
} = $props();

const resolvedSide = useDirectionAwareSide(() => side);
</script>

<SelectPortal {...portalProps}>
	<SelectPrimitive.Content
		bind:ref
		{sideOffset}
		side={resolvedSide()}
		{preventScroll}
		data-slot="select-content"
		class={cn(
			"bg-popover border text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 min-w-36 rounded-lg shadow-md duration-100 relative isolate z-50 overflow-x-hidden overflow-y-auto",
			className
		)}
		{...restProps}
	>
		<SelectScrollUpButton />
		<SelectPrimitive.Viewport
			class={cn(
				"h-[var(--bits-select-anchor-height)] w-full min-w-[var(--bits-select-anchor-width)] scroll-my-1"
			)}
		>
			{@render children?.()}
		</SelectPrimitive.Viewport>
		<SelectScrollDownButton />
	</SelectPrimitive.Content>
</SelectPortal>
