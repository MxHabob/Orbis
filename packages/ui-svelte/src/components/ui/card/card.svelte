<script lang="ts">
import type { HTMLAttributes } from "svelte/elements";
import { cn, type WithElementRef } from "../../../utils.js";

let {
	ref = $bindable(null),
	class: className,
	children,
	size = "default",
	interactive = false,
	...restProps
}: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
	size?: "default" | "sm";
	interactive?: boolean;
} = $props();
</script>

<div
	bind:this={ref}
	data-slot="card"
	data-size={size}
	class={cn(interactive ? "card-tactile-interactive" : "card-tactile", "text-card-foreground gap-4 overflow-hidden py-4 text-sm has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl group/card flex flex-col", className)}
	{...restProps}
>
	{@render children?.()}
</div>
