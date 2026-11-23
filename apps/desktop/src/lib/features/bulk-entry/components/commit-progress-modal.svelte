<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { LoadingIndicator } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Progress } from "@orbis/ui/progress";
import type { ChunkProgress } from "../types";

interface Props {
  open: boolean;
  progress: ChunkProgress | null;
}

let { open, progress }: Props = $props();
let elapsedSeconds = $state(0);

const percentage = $derived(
  progress && progress.totalRows > 0
    ? Math.min(
        100,
        Math.round((progress.processedRows / progress.totalRows) * 100),
      )
    : 0,
);
const elapsedTime = $derived(
  `${Math.floor(elapsedSeconds / 60)}:${String(elapsedSeconds % 60).padStart(2, "0")}`,
);
const chunkStart = $derived(
  progress && !progress.isFinished ? progress.processedRows + 1 : 0,
);
const chunkEnd = $derived(
  progress && !progress.isFinished
    ? Math.min(progress.processedRows + progress.chunkSize, progress.totalRows)
    : 0,
);

$effect(() => {
  if (!open) {
    elapsedSeconds = 0;
    return;
  }

  const startedAt = Date.now();
  const updateElapsed = () => {
    elapsedSeconds = Math.floor((Date.now() - startedAt) / 1000);
  };
  updateElapsed();
  const timer = setInterval(updateElapsed, 1000);
  return () => clearInterval(timer);
});
</script>

<Dialog
  {open}
  onOpenChange={(nextOpen) => {
    if (!nextOpen) return;
  }}
>
  {#if progress}
    <DialogContent
      class="max-w-md gap-0 overflow-hidden p-0 text-start"
      showCloseButton={false}
      onEscapeKeydown={(event) => event.preventDefault()}
      onInteractOutside={(event) => event.preventDefault()}
    >
      <DialogHeader
        class="border-b border-border/40 bg-muted/20 px-5 py-4"
      >
        <div class="flex items-start gap-3">
          <div
            class="flex size-9 shrink-0 items-center justify-center rounded-xl border border-border/60 bg-muted text-muted-foreground"
          >
            <LoadingIndicator size={18} class="size-[18px]" />
          </div>
          <DialogTitle class="min-w-0 text-base font-bold text-foreground">
            {m.bulk_entry_progress_title()}
          </DialogTitle>
        </div>
      </DialogHeader>

      <div class="space-y-5 px-5 py-4">
        <div
          class="flex items-center justify-between gap-3 text-xs"
          role="status"
          aria-live="polite"
        >
          {#if progress.totalChunks > 1}
            <span class="font-semibold text-foreground">
              {m.bulk_entry_progress_chunk({
                current: progress.currentChunk,
                total: progress.totalChunks,
              })}
            </span>
          {:else}
            <span class="font-semibold text-foreground">
              {m.bulk_entry_progress_title()}
            </span>
          {/if}
          <span class="shrink-0 tabular-nums text-muted-foreground">
            {m.bulk_entry_progress_elapsed({ time: elapsedTime })}
          </span>
        </div>

        {#if !progress.isFinished && progress.totalChunks > 1}
          <div
            class="rounded-lg border border-border/60 bg-muted/30 px-3 py-2 text-xs text-muted-foreground"
          >
            {m.bulk_entry_progress_chunk_rows({
              start: chunkStart,
              end: chunkEnd,
            })}
          </div>
        {/if}

        <div class="space-y-2">
          <div class="flex items-center justify-between gap-3 text-xs">
            <span class="text-muted-foreground">
              {m.bulk_entry_progress_total()}
              <span class="ms-1 font-semibold tabular-nums text-foreground">
                {progress.processedRows} / {progress.totalRows}
              </span>
            </span>
            <span class="font-bold tabular-nums text-foreground">
              {percentage}%
            </span>
          </div>
          <Progress
            value={percentage}
            class="h-2.5 bg-muted"
            aria-label={m.bulk_entry_progress_title()}
          />
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div
            class="flex flex-col items-center rounded-lg border border-border/60 bg-muted/30 p-3"
          >
            <span class="text-xs text-muted-foreground">
              {m.bulk_entry_progress_processed()}
            </span>
            <span class="mt-1 text-lg font-bold tabular-nums text-foreground">
              {progress.committedRows}
            </span>
          </div>
          <div
            class="flex flex-col items-center rounded-lg border border-border/60 bg-muted/30 p-3"
          >
            <span class="text-xs text-muted-foreground">
              {m.bulk_entry_progress_failed()}
            </span>
            <span class="mt-1 text-lg font-bold tabular-nums text-destructive">
              {progress.failedRows}
            </span>
          </div>
        </div>
      </div>

      <div
        class="border-t border-border/60 bg-muted/20 px-5 py-3 text-xs leading-relaxed text-muted-foreground"
      >
        {m.bulk_entry_progress_keep_open()}
      </div>
    </DialogContent>
  {/if}
</Dialog>
