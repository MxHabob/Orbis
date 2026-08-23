<!--
@component HorizontalPagination
A simple, compact horizontal pagination layout with back and forward navigation buttons.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { cn } from "@orbis/ui-svelte/utils";
import { ChevronLeft, ChevronRight } from "@lucide/svelte";

let {
  canScrollLeft,
  canScrollRight,
  onScrollLeft,
  onScrollRight,
  class: className,
}: {
  canScrollLeft: boolean;
  canScrollRight: boolean;
  onScrollLeft: () => void;
  onScrollRight: () => void;
  class?: string;
} = $props();
</script>

<div class={cn("flex items-center gap-1.5", className)}>
  <Button
    variant="outline"
    size="sm"
    disabled={!canScrollLeft}
    class="h-7 w-7 p-0 flex items-center justify-center cursor-pointer transition-transform active:scale-95 shrink-0"
    onclick={onScrollLeft}
  >
    <ChevronLeft
      class={cn("h-4 w-4 transition-colors", canScrollLeft ? "text-primary" : "text-muted-foreground/50")}
    />
  </Button>
  <Button
    variant="outline"
    size="sm"
    disabled={!canScrollRight}
    class="h-7 w-7 p-0 flex items-center justify-center cursor-pointer transition-transform active:scale-95 shrink-0"
    onclick={onScrollRight}
  >
    <ChevronRight
      class={cn("h-4 w-4 transition-colors", canScrollRight ? "text-primary" : "text-muted-foreground/50")}
    />
  </Button>
</div>
