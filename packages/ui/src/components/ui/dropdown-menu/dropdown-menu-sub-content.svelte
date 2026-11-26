<script lang="ts">
import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
import { cn } from "../../../utils.js";
import {
  useDirectionAwareSide,
  type PositionSide,
} from "../../../utils/direction.svelte.js";

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
	class={cn("bg-popover border text-popover-foreground text-start data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 min-w-[96px] rounded-lg p-1 shadow-lg duration-75 w-auto", className)}
	{...restProps}
/>
