<!--
@component BulkSummary
Validation summary bar with counts and collapsible error details.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { AdjustmentsHorizontal, Hashtag, InfoCircle } from "@orbis/icons";

interface Props {
  totalFilled: number;
  valid: number;
  errorCount: number;
  errorDetails: Array<{ rowIndex: number; field: string; message: string }>;
  isSubmitting?: boolean;
  submitProgress?: number;
}

let {
  totalFilled,
  valid,
  errorCount,
  errorDetails,
  isSubmitting = false,
  submitProgress = 0,
}: Props = $props();

let progressPercent = $derived(
  totalFilled > 0 ? Math.round((valid / totalFilled) * 100) : 0,
);
</script>

<div class="flex flex-col gap-3">
  <!-- Progress Bar (during submission) -->
  {#if isSubmitting}
    <div class="flex flex-col gap-1.5">
      <div class="flex items-center justify-between">
        <span class="text-xs font-medium text-foreground">{m.common_state_saving()}</span>
        <span class="text-xs text-muted-foreground">{submitProgress}%</span>
      </div>
      <Progress value={submitProgress} class="h-1.5" />
    </div>
  {/if}

  <!-- Summary Badges -->
  <div class="flex items-center gap-4 flex-wrap">
    <div class="flex items-center gap-1.5">
      <Hashtag size={13} class="text-muted-foreground" />
      <span class="text-xs font-medium text-muted-foreground">{m.common_grades_col_total()}:</span>
      <Badge variant="secondary" class="text-xs px-2 py-0.5">{totalFilled}</Badge>
    </div>

    <div class="flex items-center gap-1.5">
      <CircleCheck size={13} class="text-primary" />
      <span class="text-xs font-medium text-muted-foreground">{m.common_bulk_input_valid()}:</span>
      <Badge variant="success" class="text-xs px-2 py-0.5">{valid}</Badge>
    </div>

    {#if errorCount > 0}
      <div class="flex items-center gap-1.5">
        <CircleX size={13} class="text-destructive" />
        <span class="text-xs font-medium text-muted-foreground">{m.common_bulk_input_errors()}:</span>
        <Badge variant="destructive" class="text-xs px-2 py-0.5">{errorCount}</Badge>
      </div>
    {/if}

    <!-- Validity bar -->
    {#if totalFilled > 0}
      <div class="flex-1 min-w-24">
        <div class="h-1.5 rounded-full bg-muted overflow-hidden">
          <div
            class="h-full rounded-full transition-all duration-300"
            class:bg-primary={progressPercent === 100}
            class:bg-destructive={progressPercent < 100 && errorCount > 0}
            class:bg-muted-foreground={progressPercent < 100 && errorCount === 0}
            style="width: {progressPercent}%"
          ></div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Collapsible Error Details -->
  {#if errorCount > 0}
    <Accordion type="single" class="w-full">
      <AccordionItem value="errors" class="rounded-lg bg-destructive/5">
        <AccordionTrigger class="px-4 py-2 text-xs font-medium text-destructive hover:no-underline">
          <div class="flex items-center gap-1.5">
            <InfoCircle size={13} />
            {m.common_bulk_input_error_details()} ({errorDetails.length})
          </div>
        </AccordionTrigger>
        <AccordionContent class="px-4 pb-3">
          <div class="flex flex-col gap-1.5 max-h-44 overflow-y-auto scrollbar-hide">
            {#each errorDetails.slice(0, 50) as error}
              <div class="flex items-start gap-2 text-xs">
                <Badge variant="destructive" class="text-[10px] px-1.5 py-0 shrink-0 font-mono">
                  {m.common_bulk_input_row()} {error.rowIndex}
                </Badge>
                <span class="text-destructive/80">
                  <span class="font-semibold">{error.field}:</span> {error.message}
                </span>
              </div>
            {/each}
            {#if errorDetails.length > 50}
              <span class="text-xs text-muted-foreground">... +{errorDetails.length - 50} more</span>
            {/if}
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  {/if}
</div>
