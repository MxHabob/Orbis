<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import type { BulkCommitResult } from "../types";

  interface Props {
    result: BulkCommitResult;
    onreset: () => void;
    onretryfailed?: () => void;
    onexportreport?: () => void;
  }

  let { result, onreset, onretryfailed, onexportreport }: Props = $props();

  const licenseBlocked = $derived(result.licenseRejected);
  const groupedErrors = $derived.by(() => {
    const groups: Array<{ reason: string; count: number }> = [];
    for (const error of result.errors) {
      const reason = error.reason;
      const existing = groups.find((group) => group.reason === reason);
      if (existing) {
        existing.count++;
      } else {
        groups.push({ reason, count: 1 });
      }
    }
    return groups;
  });
  const remainingErrorCount = $derived(
    result.errors.length -
      groupedErrors
        .slice(0, 4)
        .reduce((total, group) => total + group.count, 0),
  );
</script>

<section class="mx-auto my-6 w-full max-w-2xl overflow-hidden rounded-xl border border-border/70 bg-background shadow-lg animate-in fade-in zoom-in-95 duration-200">
  <div class="flex items-center gap-3 border-b border-border/60 px-5 py-4">
    {#if result.failedCount === 0}
      <div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
        <span class="text-lg" aria-hidden="true">✓</span>
      </div>
      <h2 class="text-base font-semibold text-foreground">{m.bulk_entry_result_success_title()}</h2>
    {:else}
      <div class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-destructive/10 text-destructive">
        <span class="text-lg" aria-hidden="true">!</span>
      </div>
      <h2 class="text-base font-semibold text-foreground">{m.bulk_entry_result_partial_title()}</h2>
    {/if}
  </div>

  <div class="grid grid-cols-2 gap-3 p-5 sm:grid-cols-4">
    <div class="rounded-lg border border-border/70 bg-muted/20 px-3 py-2.5 text-center">
      <span class="block text-xs text-muted-foreground">{m.bulk_entry_total_records()}</span>
      <span class="mt-1 block text-xl font-semibold tabular-nums text-foreground">{result.totalRows}</span>
    </div>

    <div class="rounded-lg border border-emerald-500/20 bg-emerald-500/5 px-3 py-2.5 text-center">
      <span class="block text-xs text-emerald-700 dark:text-emerald-400">{m.bulk_entry_committed()}</span>
      <span class="mt-1 block text-xl font-semibold tabular-nums text-emerald-700 dark:text-emerald-400">{result.committedCount}</span>
    </div>

    <div class="rounded-lg border border-destructive/20 bg-destructive/5 px-3 py-2.5 text-center">
      <span class="block text-xs text-destructive">{m.bulk_entry_failed()}</span>
      <span class="mt-1 block text-xl font-semibold tabular-nums text-destructive">{result.failedCount}</span>
    </div>

    <div class="rounded-lg border border-border/70 bg-muted/20 px-3 py-2.5 text-center">
      <span class="block text-xs text-muted-foreground">{m.bulk_entry_skipped()}</span>
      <span class="mt-1 block text-xl font-semibold tabular-nums text-foreground">{result.skippedCount}</span>
    </div>
  </div>

  {#if result.failedCount > 0 && result.errors.length > 0}
    <div class="mx-5 mb-5 rounded-lg border border-destructive/20 bg-destructive/[0.03] p-3">
      {#if licenseBlocked}
        <p class="text-xs leading-relaxed text-destructive">
          {m.bulk_entry_result_license_invalid()}
        </p>
      {/if}
      <p class="mb-2 text-xs font-medium text-destructive">
        {m.bulk_entry_quarantined_errors()}
      </p>
      <div class="max-h-36 space-y-2 overflow-y-auto">
        {#each groupedErrors.slice(0, 4) as group (group.reason)}
          <div class="flex items-center justify-between gap-3 rounded-md border border-border/60 bg-background px-3 py-2 text-xs">
            <span class="min-w-0 flex-1 truncate text-foreground" title={group.reason}>
              {group.reason}
            </span>
            <span class="shrink-0 tabular-nums text-muted-foreground">
              {group.count}
            </span>
          </div>
        {/each}
        {#if remainingErrorCount > 0}
          <p class="text-center text-xs text-muted-foreground">
            {m.bulk_entry_more_rows({ count: remainingErrorCount })}
          </p>
        {/if}
      </div>
    </div>
  {/if}

  {#if result.warnings.length > 0}
    <p class="mx-5 mb-4 text-xs text-amber-700 dark:text-amber-400">
      {m.bulk_entry_registration_setup_warnings()}: {result.warnings.length}
    </p>
  {/if}

  <footer class="flex flex-wrap items-center justify-between gap-2 border-t border-border/60 px-5 py-3">
    <Button variant="outline" size="sm" onclick={onreset}>
      {m.bulk_entry_start_new_import()}
    </Button>
    <div class="flex flex-wrap items-center gap-2">
      {#if onexportreport}
        <Button variant="secondary" size="sm" onclick={onexportreport}>
          {m.bulk_entry_export_completion_report()}
        </Button>
      {/if}

      {#if result.failedCount > 0 && onretryfailed && !licenseBlocked}
        <Button variant="default" size="sm" onclick={onretryfailed}>
          {m.bulk_entry_retry_failed({ count: result.failedCount })}
        </Button>
      {/if}
    </div>
  </footer>
</section>
