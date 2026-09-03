<script lang="ts" module>
	export type Side = "top" | "right" | "bottom" | "left";
</script>

<script lang="ts">
	import { Dialog as SheetPrimitive } from "bits-ui";
	import XIcon from '@lucide/svelte/icons/x';
	import { Button } from "../button/index.js";
	import { cn, type WithoutChildrenOrChild } from "../../../utils.js";
	import SheetOverlay from "./sheet-overlay.svelte";
	import type { Snippet } from "svelte";

	let {
		ref = $bindable(null),
		class: className,
		side = "right",
		showCloseButton = true,
		contentClass,
		portalProps,
		children,
		...restProps
	}: WithoutChildrenOrChild<SheetPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<SheetPrimitive.PortalProps>;
		side?: Side;
		showCloseButton?: boolean;
		contentClass?: string;
		children: Snippet;
	} = $props();
</script>

<SheetPrimitive.Portal {...portalProps}>
	<SheetOverlay />
	<SheetPrimitive.Content
		bind:ref
		data-slot="sheet-content"
		data-side={side}
		class={cn(
			"fixed z-50 gap-4 transition ease-in-out md:p-4 data-[side=bottom]:inset-x-0 data-[side=bottom]:bottom-0 data-[side=bottom]:data-closed:slide-out-to-bottom data-[side=bottom]:data-open:slide-in-from-bottom data-[side=left]:inset-y-0 data-[side=left]:left-0 data-[side=left]:h-full data-[side=left]:w-3/4 data-[side=left]:data-closed:slide-out-to-left data-[side=left]:data-open:slide-in-from-left data-[side=left]:sm:max-w-sm data-[side=right]:inset-y-0 data-[side=right]:right-0 data-[side=right]:h-full data-[side=right]:w-3/4 data-[side=right]:data-closed:slide-out-to-right data-[side=right]:data-open:slide-in-from-right data-[side=right]:sm:max-w-130 data-[side=top]:inset-x-0 data-[side=top]:top-0 data-[side=top]:data-closed:slide-out-to-top data-[side=top]:data-open:slide-in-from-top data-open:animate-in data-closed:animate-out data-closed:duration-200 data-open:duration-300",
			contentClass
		)}
		{...restProps}
	>
		<div class={cn("relative h-full w-full overflow-hidden border bg-background p-6 text-foreground", className)}>
			{@render children?.()}
			{#if showCloseButton}
				<SheetPrimitive.Close data-slot="sheet-close">
					{#snippet child({ props })}
						<Button variant="ghost" class="absolute top-4 inset-e-4" size="icon-sm" {...props}>
							<XIcon />
							<span class="sr-only">Close</span>
						</Button>
					{/snippet}
				</SheetPrimitive.Close>
			{/if}
		</div>
	</SheetPrimitive.Content>
</SheetPrimitive.Portal>
