<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";
  import { cn, type WithElementRef } from "../../../utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    "data-slot": dataSlot = "kbd",
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLElement>> = $props();
</script>

<kbd
  bind:this={ref}
  data-slot={dataSlot}
  class={cn(
    "kbd-sunken inline-flex min-w-5 items-center justify-center rounded border border-input bg-background px-1.5 py-0.5 font-mono text-[9px] leading-none font-medium text-muted-foreground",
    className
  )}
  {...restProps}
>
  {@render children?.()}
</kbd>
