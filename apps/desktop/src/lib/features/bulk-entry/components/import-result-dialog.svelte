<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import type { BulkCommitResult } from "../types";
import ImportResultSummary from "./import-result-summary.svelte";

interface Props {
  open: boolean;
  result: BulkCommitResult | null;
  onclose: () => void;
}

let { open, result, onclose }: Props = $props();
</script>

<Dialog
  {open}
  onOpenChange={(nextOpen) => {
    if (!nextOpen) onclose();
  }}
>
  {#if result}
    <DialogContent
      class="max-h-[calc(100dvh-2rem)] max-w-3xl overflow-y-auto p-0 text-start"
    >
      <DialogHeader class="sr-only">
        <DialogTitle>
          {result.failedCount === 0
            ? m.bulk_entry_result_success_title()
            : m.bulk_entry_result_partial_title()}
        </DialogTitle>
      </DialogHeader>
      <ImportResultSummary result={result} embedded />
    </DialogContent>
  {/if}
</Dialog>
