<script lang="ts">
import { Switch as SwitchPrimitive } from "bits-ui";
import { cn, type WithoutChildrenOrChild } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  checked = $bindable(false),
  size = "default",
  ...restProps
}: WithoutChildrenOrChild<SwitchPrimitive.RootProps> & {
  size?: "sm" | "default";
} = $props();
</script>

<SwitchPrimitive.Root
	bind:ref
	bind:checked
	data-slot="switch"
	data-size={size}
	class={cn(
		"border-border bg-muted/40 data-checked:bg-primary data-checked:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 shrink-0 rounded-[6px] border focus-visible:ring-3 aria-invalid:ring-3 peer group/switch relative inline-flex items-center justify-start transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50",
		"data-[size=default]:h-6 data-[size=default]:w-11 data-[size=default]:p-0",
		"data-[size=sm]:h-4 data-[size=sm]:w-7 data-[size=sm]:p-0",
		className
	)}
	{...restProps}
>
	<SwitchPrimitive.Thumb
		data-slot="switch-thumb"
		class="bg-background border border-border data-checked:border-primary absolute left-0 top-1/2 -translate-y-1/2 rounded-lg pointer-events-none flex items-center justify-center ring-0 shadow-sm transition-transform group-data-[size=default]/switch:h-full group-data-[size=default]/switch:w-5.75 group-data-[size=default]/switch:data-checked:translate-x-5 group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:h-full group-data-[size=sm]/switch:w-3.5 group-data-[size=sm]/switch:data-checked:translate-x-3 group-data-[size=sm]/switch:data-unchecked:translate-x-0"
	>
		{#if checked}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="3"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-2.5 text-primary"
			>
				<polyline points="20 6 9 17 4 12" />
			</svg>
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="3"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-2.5 text-muted-foreground/80"
			>
				<line x1="18" y1="6" x2="6" y2="18" />
				<line x1="6" y1="6" x2="18" y2="18" />
			</svg>
		{/if}
	</SwitchPrimitive.Thumb>
</SwitchPrimitive.Root>

