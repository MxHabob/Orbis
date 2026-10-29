<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import type { ChunkProgress } from "../types";

interface Props {
  open: boolean;
  progress: ChunkProgress | null;
}

let { open, progress }: Props = $props();

const percentage = $derived(
  progress && progress.totalRows > 0
    ? Math.min(
        100,
        Math.round((progress.processedRows / progress.totalRows) * 100),
      )
    : 0,
);
</script>

{#if open && progress}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md p-4 animate-in fade-in duration-150">
    <div class="w-full max-w-md rounded-2xl bg-card border border-border shadow-2xl p-6 flex flex-col items-center text-center space-y-5">
      <!-- Animated Spinner / Icon -->
      <div class="h-16 w-16 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary relative">
        <LoadingIndicator viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" size={32} class="h-8 w-8" />
      </div>

      <!-- Title & Subtitle -->
      <div>
        <h3 class="text-lg font-bold text-foreground">{m.bulk_entry_progress_title()}</h3>
        <p class="text-xs text-muted-foreground mt-1">
          {m.bulk_entry_progress_description()}
        </p>
      </div>

      <!-- Progress Bar -->
      <div class="w-full space-y-2">
        <div class="flex justify-between text-xs font-semibold text-muted-foreground px-0.5">
          <span>{m.bulk_entry_progress_chunk({ current: progress.currentChunk, total: progress.totalChunks })}</span>
          <span class="text-primary font-bold">{percentage}%</span>
        </div>

        <div class="h-2.5 w-full bg-muted rounded-full overflow-hidden border border-border/50">
          <div
            class="h-full bg-primary transition-all duration-300 rounded-full"
            style="width: {percentage}%"
          ></div>
        </div>
      </div>

      <!-- Live Stats -->
      <div class="w-full grid grid-cols-2 gap-2 pt-2 border-t border-border text-xs">
        <div class="p-2.5 rounded-lg bg-muted/40 flex flex-col items-center">
          <span class="text-muted-foreground">{m.bulk_entry_progress_processed()}</span>
          <span class="text-base font-bold text-primary mt-0.5">{progress.committedRows}</span>
        </div>

        <div class="p-2.5 rounded-lg bg-muted/40 flex flex-col items-center">
          <span class="text-muted-foreground">{m.bulk_entry_progress_failed()}</span>
          <span class="text-base font-bold text-destructive mt-0.5">{progress.failedRows}</span>
        </div>
      </div>
    </div>
  </div>
{/if}
