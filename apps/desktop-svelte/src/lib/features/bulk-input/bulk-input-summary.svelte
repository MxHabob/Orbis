<!--
@component BulkInputSummary
Summary bar showing validation stats and collapsible error details.
-->
<script lang="ts">
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";
import { CircleCheck, CircleX, Hash } from "lucide-svelte";
import * as common from "$lib/paraglide/messages/common";

interface Props {
  totalFilledRows: number;
  validCount: number;
  errorCount: number;
  errorDetails: Array<{ rowIndex: number; field: string; message: string }>;
}

let { totalFilledRows, validCount, errorCount, errorDetails }: Props = $props();
</script>

<div class="flex flex-col gap-3">
  <!-- Summary Badges -->
  <div class="flex items-center gap-3 flex-wrap">
    <div class="flex items-center gap-1.5">
      <Hash size={13} class="text-muted-foreground" />
      <span class="text-xs font-medium text-muted-foreground">{common.bulk_input_total()}:</span>
      <Badge variant="secondary" class="text-xs px-2 py-0.5">{totalFilledRows}</Badge>
    </div>

    <div class="flex items-center gap-1.5">
      <CircleCheck size={13} class="text-primary" />
      <span class="text-xs font-medium text-muted-foreground">{common.bulk_input_valid()}:</span>
      <Badge variant="success" class="text-xs px-2 py-0.5">{validCount}</Badge>
    </div>

    <div class="flex items-center gap-1.5">
      <CircleX size={13} class="text-destructive" />
      <span class="text-xs font-medium text-muted-foreground">{common.bulk_input_errors()}:</span>
      <Badge variant={errorCount > 0 ? "destructive" : "secondary"} class="text-xs px-2 py-0.5">{errorCount}</Badge>
    </div>
  </div>

  <!-- Collapsible Error Details -->
  {#if errorCount > 0}
    <Accordion type="single" collapsible class="w-full">
      <AccordionItem value="errors" class="border border-destructive/20 rounded-lg bg-destructive/5">
        <AccordionTrigger class="px-4 py-2 text-xs font-medium text-destructive hover:no-underline">
          {common.bulk_input_error_details()} ({errorDetails.length})
        </AccordionTrigger>
        <AccordionContent class="px-4 pb-3">
          <div class="flex flex-col gap-1.5 max-h-40 overflow-y-auto scrollbar-hide">
            {#each errorDetails as error}
              <div class="flex items-start gap-2 text-xs">
                <Badge variant="destructive" class="text-[10px] px-1.5 py-0 shrink-0 font-mono">
                  {common.bulk_input_row()} {error.rowIndex}
                </Badge>
                <span class="text-destructive/80">
                  <span class="font-medium">{error.field}:</span> {error.message}
                </span>
              </div>
            {/each}
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  {/if}
</div>
