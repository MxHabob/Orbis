<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle,
  } from "@orbis/ui/dialog";
  import type { BulkPreviewSummary } from "../types";

  interface Props {
    open: boolean;
    summary: BulkPreviewSummary<any> | null;
    onclose: () => void;
    onconfirm: () => void;
  }

  let { open, summary, onclose, onconfirm }: Props = $props();
</script>

<Dialog
  {open}
  onOpenChange={(nextOpen) => {
    if (!nextOpen) onclose();
  }}
>
  {#if summary}
    <DialogContent class="max-w-sm gap-0 p-0 text-start">
      <DialogHeader class="px-5 py-4">
        <DialogTitle class="text-base font-semibold">
          {m.bulk_entry_preview_title()}
        </DialogTitle>
      </DialogHeader>

      <div class="border-y border-border/60 px-5 py-4">
        <p class="text-sm text-foreground">
          {m.bulk_entry_preview_valid_records()}:
          <span class="font-semibold tabular-nums">
            {summary.validRowsCount}
          </span>
        </p>
        {#if summary.errorRowsCount > 0}
          <p class="mt-1 text-xs text-destructive">
            {m.bulk_entry_preview_invalid_records()}:
            <span class="font-medium tabular-nums">
              {summary.errorRowsCount}
            </span>
          </p>
        {/if}
      </div>

      <DialogFooter class="flex-row justify-end px-5 py-3">
        <Button variant="outline" size="sm" onclick={onclose}>
          {m.bulk_entry_preview_back_to_edit()}
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={onconfirm}
          disabled={summary.validRowsCount === 0}
        >
          {m.bulk_entry_preview_confirm({ count: summary.validRowsCount })}
        </Button>
      </DialogFooter>
    </DialogContent>
  {/if}
</Dialog>
