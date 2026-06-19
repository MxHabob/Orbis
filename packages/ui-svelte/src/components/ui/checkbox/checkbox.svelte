<script lang="ts">
	import { Checkbox as CheckboxPrimitive } from "bits-ui";
	import { cn, type WithoutChildrenOrChild } from "../../../utils.js";
	import { Check as CheckIcon, Minus as MinusIcon } from "lucide-svelte";

	let {
		ref = $bindable(null),
		checked = $bindable(false),
		indeterminate = $bindable(false),
		class: className,
		...restProps
	}: WithoutChildrenOrChild<CheckboxPrimitive.RootProps> = $props();
</script>

<CheckboxPrimitive.Root
	bind:ref
	data-slot="checkbox"
	class={cn(
		"border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-all group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50",
		"shadow-[inset_0_1px_3px_0_hsl(267_30%_30%/0.1)] dark:shadow-[inset_0_1px_3px_0_hsl(0_0%_0%/0.35)]",
		"data-checked:shadow-[0_0_6px_-1px_hsl(248_50%_56%/0.35)] dark:data-checked:shadow-[0_0_8px_-1px_hsl(248_50%_56%/0.45)]",
		className
	)}
	bind:checked
	bind:indeterminate
	{...restProps}
>
	{#snippet children({ checked, indeterminate })}
		<div
			data-slot="checkbox-indicator"
			class="[&>svg]:size-3.5 grid place-content-center text-current transition-none"
		>
			{#if checked}
				<CheckIcon  />
			{:else if indeterminate}
				<MinusIcon  />
			{/if}
		</div>
	{/snippet}
</CheckboxPrimitive.Root>
