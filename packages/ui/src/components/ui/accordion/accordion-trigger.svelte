<script lang="ts">
import { Accordion as AccordionPrimitive } from "bits-ui";
import { cn, type WithoutChild } from "../../../utils.js";
import { ChevronDown, ChevronUp } from "@orbis/icons";

let {
  ref = $bindable(null),
  class: className,
  level = 3,
  children,
  ...restProps
}: WithoutChild<AccordionPrimitive.TriggerProps> & {
  level?: AccordionPrimitive.HeaderProps["level"];
} = $props();
</script>

<AccordionPrimitive.Header {level} class="flex w-full">
	<AccordionPrimitive.Trigger
		data-slot="accordion-trigger"
		bind:ref
		class={cn(
			"focus-visible:ring-ring/50 focus-visible:border-ring focus-visible:after:border-ring **:data-[slot=accordion-trigger-icon]:text-muted-foreground rounded-lg py-2.5 text-start text-sm font-medium hover:underline focus-visible:ring-3 **:data-[slot=accordion-trigger-icon]:size-4 group/accordion-trigger relative flex w-full flex-1 flex-row items-center justify-between border border-transparent transition-all outline-none disabled:pointer-events-none disabled:opacity-50",
			className
		)}
		{...restProps}
	>
		<span class="min-w-0 flex-1 text-start">
			{@render children?.()}
		</span>
		<ChevronDown data-slot="accordion-trigger-icon" class="cn-accordion-trigger-icon pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden" />
		<ChevronUp data-slot="accordion-trigger-icon" class="cn-accordion-trigger-icon pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline" />
	</AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
