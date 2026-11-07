<script lang="ts">
import type { Snippet } from "svelte";
import { AdjustmentsHorizontal, XMark } from "@orbis/icons";
import { Button } from "../components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../components/ui/dialog";

let {
  open = $bindable(false),
  title,
  triggerLabel,
  count = 0,
  clearLabel,
  applyLabel,
  disabled = false,
  onClear,
  onApply,
  children,
}: {
  open?: boolean;
  title: string;
  triggerLabel: string;
  count?: number;
  clearLabel: string;
  applyLabel: string;
  disabled?: boolean;
  onClear: () => void;
  onApply?: () => void;
  children: Snippet;
} = $props();

function apply() {
  onApply?.();
  open = false;
}
</script>

<Dialog bind:open>
  <DialogTrigger>
    {#snippet child({ props })}
      <Button
        variant="outline"
        size="sm"
        class="relative h-8 gap-1.5 px-2.5 text-xs"
        {disabled}
        {...props}
      >
        <AdjustmentsHorizontal class="size-3.5" />
        {triggerLabel}
        {#if count > 0}
          <span
            class="inline-flex h-4 min-w-4 items-center justify-center rounded-sm bg-primary px-1 text-[10px] font-semibold leading-none text-primary-foreground"
          >{count}</span>
        {/if}
      </Button>
    {/snippet}
  </DialogTrigger>

  <DialogContent class="max-h-[80vh] max-w-3xl gap-0 overflow-hidden p-0">
    <DialogHeader class="gap-1 border-b border-border/60 px-5 py-4 pe-10">
      <DialogTitle class="text-sm font-semibold">{title}</DialogTitle>
    </DialogHeader>

    <div class="min-h-0 overflow-y-auto p-5">
      {@render children()}
    </div>

    <DialogFooter
      class="flex-row justify-between gap-2 border-t border-border/60 px-5 py-4"
    >
      <Button
        variant="ghost"
        size="sm"
        class="text-xs text-destructive hover:text-destructive"
        disabled={count === 0}
        onclick={onClear}
      >
        <XMark class="me-1 size-3.5" />
        {clearLabel}
      </Button>
      <Button size="sm" class="text-xs" onclick={apply}>
        {applyLabel}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
