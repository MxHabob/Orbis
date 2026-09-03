<script lang="ts">
import {
  cn,
  type WithElementRef,
  type WithoutChildren,
} from "../../../utils.js";
import type { HTMLTextareaAttributes } from "svelte/elements";

let {
  ref = $bindable(null),
  value = $bindable(),
  class: className,
  "data-slot": dataSlot = "textarea",
  ...restProps
}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();
</script>

<textarea
	bind:this={ref}
	data-slot={dataSlot}
	class={cn(
		"input-sunken dark:bg-input/30 disabled:bg-input/50 dark:disabled:bg-input/80 rounded-md px-2.5 py-2 text-sm placeholder:text-muted-foreground flex field-sizing-content min-h-16 w-full min-w-0 max-w-full break-all outline-none disabled:cursor-not-allowed disabled:opacity-50 resize-y box-border",
		className
	)}
	bind:value
	{...restProps}
></textarea>
