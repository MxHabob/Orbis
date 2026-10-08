<script lang="ts">
import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
import { cn } from "../../../utils.js";
import { useDirectionAwareSide, type PositionSide } from "../../../utils/direction.svelte.js";

let {
  ref = $bindable(null),
  class: className,
  side = "inline-end",
  ...restProps
}: Omit<DropdownMenuPrimitive.SubContentProps, "side"> & {
  side?: PositionSide;
} = $props();

const resolvedSide = useDirectionAwareSide(() => side);
</script>

<DropdownMenuPrimitive.SubContent
	bind:ref
	data-slot="dropdown-menu-sub-content"
	side={resolvedSide()}
	class={cn("bg-popover border text-popover-foreground text-start data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 min-w-[96px] rounded-lg p-1 shadow-lg duration-100 w-auto", className)}
	{...restProps}
/>
