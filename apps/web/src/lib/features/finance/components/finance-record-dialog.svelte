<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { Snippet } from "svelte";
import { Button } from "@orbis/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@orbis/ui/dialog";
import { Printer } from "@orbis/icons";

let {
  open = $bindable(false), title, fields, children, onprint,
}: {
  open?: boolean;
  title: string;
  fields: readonly { label: string; value?: string | null }[];
  children?: Snippet;
  onprint?: () => void;
} = $props();
</script>

<Dialog bind:open>
  <DialogContent class="flex max-h-[min(88dvh,46rem)] flex-col gap-0 overflow-hidden p-0 sm:max-w-2xl">
    <DialogHeader class="shrink-0 border-b border-border px-5 py-4 text-start">
      <DialogTitle>{title}</DialogTitle>
      <DialogDescription>{m.finance_ui_record_readonly()}</DialogDescription>
    </DialogHeader>

    <div class="min-h-0 flex-1 space-y-5 overflow-y-auto overscroll-contain p-5">
      <dl class="grid gap-4 sm:grid-cols-2">
        {#each fields as field}
          <div class="min-w-0 space-y-1">
            <dt class="text-xs text-muted-foreground">{field.label}</dt>
            <dd class="break-words text-sm font-medium text-foreground" dir="auto">{field.value || "—"}</dd>
          </div>
        {/each}
      </dl>
      {@render children?.()}
    </div>

    <DialogFooter class="shrink-0 border-t border-border px-5 py-3">
      <Button type="button" variant="outline" onclick={() => (open = false)}>{m.common_action_close()}</Button>
      {#if onprint}
        <Button type="button" onclick={onprint}><Printer size={16} />{m.finance_ui_print()}</Button>
      {/if}
    </DialogFooter>
  </DialogContent>
</Dialog>
