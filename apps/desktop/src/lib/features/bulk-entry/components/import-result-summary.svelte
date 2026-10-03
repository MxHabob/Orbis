<script lang="ts">
  import type { BulkCommitResult } from "../types";

  interface Props {
    result: BulkCommitResult;
    onreset: () => void;
    onretryfailed?: () => void;
    onexportreport?: () => void;
  }

  let { result, onreset, onretryfailed, onexportreport }: Props = $props();
</script>

<div class="w-full max-w-2xl mx-auto my-8 p-8 rounded-3xl bg-card border border-border shadow-xl space-y-6 animate-in fade-in zoom-in-95 duration-200" dir="rtl">
  <!-- Status Badge & Icon -->
  <div class="flex flex-col items-center text-center space-y-3">
    {#if result.failedCount === 0}
      <div class="h-16 w-16 rounded-2xl bg-emerald-500/15 border border-emerald-500/30 text-emerald-600 dark:text-emerald-400 flex items-center justify-center text-3xl shadow-inner">
        ✓
      </div>
      <h2 class="text-2xl font-black text-foreground">اكتملت عملية الاستيراد بنجاح تام</h2>
      <p class="text-xs text-muted-foreground max-w-md">
        تم قيد واعتماد كافة السجلات بنجاح في قاعدة البيانات وتسجيلها في سجل التدقيق (<span class="font-mono text-primary font-bold">{result.batchId}</span>).
      </p>
    {:else}
      <div class="h-16 w-16 rounded-2xl bg-amber-500/15 border border-amber-500/30 text-amber-600 dark:text-amber-400 flex items-center justify-center text-3xl shadow-inner">
        ⚠
      </div>
      <h2 class="text-2xl font-black text-foreground">اكتمل الاستيراد مع وجود بعض التعثرات</h2>
      <p class="text-xs text-muted-foreground max-w-md">
        تم اعتماد السجلات الصالحة بنجاح، بينما تم عزل الصفوف المتعثرة لمراجعتها وتصحيحها.
      </p>
    {/if}
  </div>

  <!-- Metric Counters -->
  <div class="grid grid-cols-4 gap-3">
    <div class="p-3.5 rounded-xl bg-muted/40 border border-border flex flex-col items-center text-center">
      <span class="text-xs text-muted-foreground">إجمالي السجلات</span>
      <span class="text-2xl font-black text-foreground mt-1">{result.totalRows}</span>
    </div>

    <div class="p-3.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex flex-col items-center text-center">
      <span class="text-xs text-emerald-600 dark:text-emerald-400 font-bold">تم اعتمادها</span>
      <span class="text-2xl font-black text-emerald-600 dark:text-emerald-400 mt-1">{result.committedCount}</span>
    </div>

    <div class="p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 flex flex-col items-center text-center">
      <span class="text-xs text-destructive font-bold">تعثرت</span>
      <span class="text-2xl font-black text-destructive mt-1">{result.failedCount}</span>
    </div>

    <div class="p-3.5 rounded-xl bg-muted/40 border border-border flex flex-col items-center text-center">
      <span class="text-xs text-muted-foreground">مستثناة</span>
      <span class="text-2xl font-black text-foreground mt-1">{result.skippedCount}</span>
    </div>
  </div>

  <!-- Error Quarantine Breakdown if any -->
  {#if result.failedCount > 0 && result.errors.length > 0}
    <div class="p-4 rounded-2xl bg-destructive/5 border border-destructive/20 space-y-2.5">
      <span class="text-xs font-bold text-destructive block">تفاصيل الأخطاء والصفوف المعزولة:</span>
      <div class="max-h-40 overflow-y-auto space-y-1.5 text-xs text-muted-foreground pr-1">
        {#each result.errors.slice(0, 10) as err}
          <div class="p-2 rounded-lg bg-card border border-border flex items-start gap-2">
            <span class="px-1.5 py-0.5 rounded bg-destructive/15 text-destructive font-bold font-mono text-[10px]">
              صف {err.rowNumber}
            </span>
            <span class="text-foreground">{err.reason}</span>
          </div>
        {/each}
        {#if result.errors.length > 10}
          <span class="text-[11px] text-muted-foreground block text-center mt-1">
            + {result.errors.length - 10} صفوف أخرى...
          </span>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Actions -->
  <div class="pt-4 border-t border-border flex items-center justify-between gap-3">
    <button
      onclick={onreset}
      class="px-5 py-2.5 rounded-xl border border-border text-foreground hover:bg-muted font-bold transition-colors text-xs"
    >
      بدء عملية استيراد جديدة
    </button>

    <div class="flex items-center gap-2">
      {#if onexportreport}
        <button
          onclick={onexportreport}
          class="px-4 py-2.5 rounded-xl bg-muted border border-border text-foreground hover:bg-muted/80 font-bold transition-colors text-xs flex items-center gap-1.5"
        >
          <span>تصدير تقرير الإنجاز</span>
          <span>📄</span>
        </button>
      {/if}

      {#if result.failedCount > 0 && onretryfailed}
        <button
          onclick={onretryfailed}
          class="px-5 py-2.5 rounded-xl bg-primary text-primary-foreground font-bold hover:bg-primary/90 transition-all shadow-md text-xs flex items-center gap-1.5"
        >
          <span>تصحيح وإعادة محاولة الصفوف المتعثرة ({result.failedCount})</span>
          <span>◄</span>
        </button>
      {/if}
    </div>
  </div>
</div>
