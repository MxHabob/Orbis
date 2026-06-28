<!--
  @page CurrenciesPage
  Allows administrators to manage multi-currency exchange rates, define base currencies,
  and trigger default accounting configuration seeding.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Coins,
  RefreshCw,
  Edit2,
  Check,
  X,
  Sparkles,
  DollarSign,
  Globe,
  Info,
} from "lucide-svelte";
import { fade } from "svelte/transition";

let { data }: { data: { orgId: string } } = $props();

// State
let currencies = $state<any[]>([]);
let isLoading = $state(true);
let isSeeding = $state(false);
let error = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Editing state
let editingCode = $state<string | null>(null);
let editRate = $state<number>(1.0);

async function loadCurrencies() {
  isLoading = true;
  error = null;
  try {
    currencies = await commands.finance.getCurrencies(data.orgId);
  } catch (e) {
    console.error("Error loading currencies:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadCurrencies();
  }
});

async function startEditing(c: any) {
  editingCode = c.code;
  editRate = c.exchange_rate;
}

async function cancelEditing() {
  editingCode = null;
}

async function saveRate(code: string) {
  if (editRate <= 0) {
    error = "سعر الصرف يجب أن يكون أكبر من الصفر";
    return;
  }

  try {
    await commands.finance.updateCurrency(data.orgId, code, {
      exchange_rate: editRate,
    });
    successMessage = "تم تحديث سعر الصرف بنجاح";
    editingCode = null;
    await loadCurrencies();

    setTimeout(() => {
      successMessage = null;
    }, 3000);
  } catch (e) {
    console.error("Error updating exchange rate:", e);
    error = e instanceof Error ? e.message : String(e);
  }
}

async function seedFinancials() {
  if (
    !confirm(
      "هل أنت متأكد من رغبتك في تهيئة النظام المالي الافتراضي؟ سيتم إنشاء العملات (ريال، دولار، سعودي) وشجرة الحسابات المحاسبية الأساسية.",
    )
  )
    return;

  isSeeding = true;
  error = null;
  successMessage = null;
  try {
    await commands.finance.seedDefaultFinancials(data.orgId);
    successMessage =
      "تم تهيئة النظام المالي وتنزيل شجرة الحسابات والعملات الافتراضية بنجاح!";
    await loadCurrencies();
  } catch (e) {
    console.error("Error seeding financials:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isSeeding = false;
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Top Header Section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <Coins class="text-primary" size={20} />
        <span>إدارة العملات وأسعار الصرف</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        تهيئة العملات المقبولة في المدرسة وتعديل أسعار الصرف اليومية مقابل العملة المحلية الأساسية.
      </p>
    </div>

    <!-- Seeding Button -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={seedFinancials}
        disabled={isSeeding}
        class="h-9 text-xs flex items-center gap-1.5 cursor-pointer border-primary/30 hover:bg-primary/5 text-primary font-semibold"
      >
        <Sparkles size={13} class={isSeeding ? "animate-spin" : ""} />
        <span>{isSeeding ? "جاري التهيئة..." : "تهيئة النظام المالي والعملات"}</span>
      </Button>
      
      <Button variant="outline" size="sm" onclick={loadCurrencies} class="h-9 text-xs flex items-center gap-1 cursor-pointer">
        <RefreshCw size={13} />
        <span>تحديث</span>
      </Button>
    </div>
  </div>

  <!-- Messages -->
  {#if error}
    <div in:fade={{ duration: 150 }} class="mt-4 border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start">
      <span class="font-bold">خطأ:</span> {error}
    </div>
  {/if}
  {#if successMessage}
    <div in:fade={{ duration: 150 }} class="mt-4 border border-emerald-500/20 bg-emerald-500/10 text-emerald-500 rounded-xl p-4 flex items-center gap-3 text-sm text-start font-medium">
      <Check size={16} />
      <span>{successMessage}</span>
    </div>
  {/if}

  <!-- Info Alert Box -->
  <div class="mt-6 border border-border/60 bg-muted/15 rounded-xl p-4 flex gap-3 text-start text-xs select-none hover:border-primary/25 transition-all duration-200">
    <Info class="text-primary shrink-0" size={16} />
    <div class="flex flex-col gap-1 text-muted-foreground leading-relaxed">
      <span class="font-bold text-foreground text-xs">كيف يعمل نظام أسعار الصرف؟</span>
      <span>يتم اعتماد <strong>العملة الأساسية (ريال يمني YER)</strong> كعملة التقييم والتقارير في النظام. عند سداد الطلاب برسوم أجنبية (مثل الدولار أو السعودي)، يقوم النظام تلقائياً بتحويل المبلغ وحسابه في الدفاتر بالعملة الأساسية بناءً على أسعار الصرف المدخلة هنا، مما يضمن دقة الأرصدة والموازين دون أي تضارب.</span>
    </div>
  </div>

  <!-- Content Section -->
  <div class="flex-1 overflow-y-auto py-6">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(3) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if currencies.length === 0}
      <!-- Zero state -->
      <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-4">
        <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <Globe size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">لم يتم تهيئة أي عملات بعد</h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          اضغط على زر <strong>"تهيئة النظام المالي والعملات"</strong> في الأعلى لتنزيل العملات وشجرة الحسابات الافتراضية للبدء فوراً.
        </p>
        <Button variant="default" size="sm" onclick={seedFinancials} class="mt-2 cursor-pointer font-bold">
          تهيئة النظام الآن
        </Button>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30 shadow-sm">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">رمز العملة</TableHead>
              <TableHead class="text-start font-bold text-xs">اسم العملة</TableHead>
              <TableHead class="text-start font-bold text-xs">الرمز / العلامة</TableHead>
              <TableHead class="text-start font-bold text-xs">موضع الرمز</TableHead>
              <TableHead class="text-start font-bold text-xs">النوع</TableHead>
              <TableHead class="text-end font-bold text-xs">سعر الصرف مقابل العملة الأساسية</TableHead>
              <TableHead class="text-end font-bold text-xs w-[120px]">إجراءات</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each currencies as c (c.code)}
              <TableRow class="hover:bg-muted/20 transition-all duration-150">
                <TableCell class="text-start font-mono text-xs text-foreground font-bold">{c.code}</TableCell>
                <TableCell class="text-start text-xs font-semibold">{c.name_ar || c.name}</TableCell>
                <TableCell class="text-start font-mono text-xs text-muted-foreground">{c.symbol}</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground font-semibold">
                  {c.symbol_position === 'before' ? 'قبل المبلغ ($10)' : 'بعد المبلغ (10﷼)'}
                </TableCell>
                <TableCell class="text-start text-xs">
                  {#if c.is_base === 1}
                    <span class="px-2 py-0.5 rounded-full text-[9px] font-bold bg-primary/10 text-primary border border-primary/20">
                      العملة الأساسية
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded-full text-[9px] font-semibold bg-muted text-muted-foreground border border-border/40">
                      عملة فرعية
                    </span>
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs font-bold font-mono">
                  {#if editingCode === c.code}
                    <div class="flex items-center justify-end gap-1.5" transition:fade={{ duration: 100 }}>
                      <Input
                        type="number"
                        step="0.01"
                        bind:value={editRate}
                        class="h-8 w-[100px] text-end font-mono text-xs font-bold bg-background border border-primary"
                      />
                      <span class="text-[10px] text-muted-foreground font-bold">YER</span>
                    </div>
                  {:else}
                    <span class="text-foreground">
                      {c.is_base === 1 ? '1.0000' : c.exchange_rate.toFixed(4)}
                    </span>
                    {#if c.is_base !== 1}
                      <span class="text-[10px] text-muted-foreground font-bold mr-1">YER</span>
                    {/if}
                  {/if}
                </TableCell>
                <TableCell class="text-end text-xs">
                  {#if c.is_base === 1}
                    <span class="text-[10px] text-muted-foreground/60 italic font-semibold">غير قابلة للتعديل</span>
                  {:else if editingCode === c.code}
                    <div class="flex items-center justify-end gap-1 select-none">
                      <Button
                        variant="default"
                        size="icon"
                        onclick={() => saveRate(c.code)}
                        class="h-7 w-7 bg-emerald-500 hover:bg-emerald-600 text-white cursor-pointer rounded-md"
                      >
                        <Check size={12} />
                      </Button>
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={cancelEditing}
                        class="h-7 w-7 cursor-pointer rounded-md border-border/80"
                      >
                        <X size={12} />
                      </Button>
                    </div>
                  {:else}
                    <Button
                      variant="outline"
                      size="sm"
                      onclick={() => startEditing(c)}
                      class="h-8 text-[11px] flex items-center gap-1 cursor-pointer hover:bg-primary/5 hover:text-primary"
                    >
                      <Edit2 size={10} />
                      <span>تعديل السعر</span>
                    </Button>
                  {/if}
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>
