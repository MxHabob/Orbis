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
	dir="ltr"
	class={cn(
		"border-border bg-muted/40 data-checked:bg-primary data-checked:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 shrink-0 rounded-[6px] border focus-visible:ring-3 aria-invalid:ring-3 peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50",
		"data-unchecked:shadow-[inset_0_2px_4px_0_oklch(0.3_0.05_280_/_0.06),_inset_0_1px_2px_0_oklch(0.3_0.05_280_/_0.04)] dark:data-unchecked:shadow-[inset_0_3px_6px_0_oklch(0_0_0_/_0.35),_inset_0_1px_2px_0_oklch(0_0_0_/_0.2)]",
		"data-[size=default]:h-6 data-[size=default]:w-11 data-[size=default]:p-0",
		"data-[size=sm]:h-4 data-[size=sm]:w-7 data-[size=sm]:p-0",
		className
	)}
	{...restProps}
>
	<SwitchPrimitive.Thumb
		data-slot="switch-thumb"
		class="bg-background border border-border data-checked:border-primary rounded-[4px] pointer-events-none flex items-center justify-center ring-0 transition-all group-data-[size=default]/switch:h-full group-data-[size=default]/switch:w-[23px] group-data-[size=default]/switch:data-checked:translate-x-[20px] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:h-full group-data-[size=sm]/switch:w-[14px] group-data-[size=sm]/switch:data-checked:translate-x-[12px] group-data-[size=sm]/switch:data-unchecked:translate-x-0 shadow-[0_2px_0_0_var(--color-border),_0_1px_3px_0_oklch(0_0_0_/_0.08)] dark:shadow-[0_2px_0_0_var(--color-border),_0_1px_3px_0_oklch(0_0_0_/_0.3)] data-checked:shadow-[0_2px_0_0_oklch(0.35_0.1_280),_0_1px_3px_0_oklch(0_0_0_/_0.2)] dark:data-checked:shadow-[0_2px_0_0_oklch(0.3_0.1_280),_0_1px_3px_0_oklch(0_0_0_/_0.5)]"
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

