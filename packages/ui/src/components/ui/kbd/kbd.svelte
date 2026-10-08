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
    "kbd-keycap inline-flex h-5 min-w-5 select-none items-center justify-center whitespace-nowrap rounded-[5px] border border-border/80 bg-muted/60 px-1.5 font-mono text-[10px] leading-none font-medium text-muted-foreground",
    className
  )}
  {...restProps}
>
  {@render children?.()}
</kbd>
