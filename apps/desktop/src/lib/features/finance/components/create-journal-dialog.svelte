<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { formatCurrency, l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import { toast } from "@orbis/ui";
import { AdjustmentsHorizontal, CheckCircle, Grid, InfoCircle, Plus, Scale, Trash } from "@orbis/icons";

interface JournalLine {
  id: string;
  accountId: string;
  debit: number;
  credit: number;
  description: string;
}

let {
  open = $bindable(false),
  accounts = [],
  currencies = [],
  baseCurrency = "SAR",
  onsubmit,
}: {
  open: boolean;
  accounts: any[];
  currencies: { code: string; name: string }[];
  baseCurrency?: string;
  onsubmit: (entry: {
    referenceNumber: string;
    accountingDate: string;
    currency: string;
    description: string;
    lines: JournalLine[];
    totalAmount: number;
  }) => Promise<void>;
} = $props();

let referenceNumber = $state(`JV-${Date.now().toString().slice(-6)}`);
let accountingDate = $state(new Date().toISOString().slice(0, 10));
let selectedCurrency = $state(baseCurrency);
let description = $state("");
let isSubmitting = $state(false);

let lines = $state<JournalLine[]>([
  { id: "1", accountId: "", debit: 0, credit: 0, description: "" },
  { id: "2", accountId: "", debit: 0, credit: 0, description: "" },
]);

function addLine() {
  lines = [
    ...lines,
    {
      id: Math.random().toString(36).slice(2, 9),
      accountId: "",
      debit: 0,
      credit: 0,
      description: "",
    },
  ];
}

function removeLine(index: number) {
  if (lines.length <= 2) {
    toast.error("يجب أن يحتوي قيد اليومية على سطرين على الأقل (مدين ودائن)");
    return;
  }
  lines = lines.filter((_, i) => i !== index);
}

// Computations
const totalDebit = $derived(
  lines.reduce((sum, l) => sum + (Number(l.debit) || 0), 0),
);
const totalCredit = $derived(
  lines.reduce((sum, l) => sum + (Number(l.credit) || 0), 0),
);
const difference = $derived(Math.abs(totalDebit - totalCredit));
const isBalanced = $derived(
  totalDebit > 0 && totalCredit > 0 && Math.abs(totalDebit - totalCredit) < 0.001,
);

async function handleSubmit() {
  if (!isBalanced) {
    toast.error("القيد المحاسبي غير متوازن! يجب أن يتساوى إجمالي المدين مع إجمالي الدائن.");
    return;
  }

  for (const line of lines) {
    if (!line.accountId) {
      toast.error("يرجى تحديد الحساب المحاسبي لجميع أسطر القيد.");
      return;
    }
  }

  isSubmitting = true;
  try {
    await onsubmit({
      referenceNumber,
      accountingDate,
      currency: selectedCurrency,
      description,
      lines,
      totalAmount: totalDebit,
    });
    toast.success("تم ترحيل قيد اليومية بنجاح");
    open = false;
  } catch (e: any) {
    console.error("Failed to post journal entry:", e);
    toast.error(e?.message || "فشل ترحيل قيد اليومية");
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-3xl flex flex-col p-6 text-start">
    <DialogHeader class="border-b pb-3">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Scale class="w-4 h-4" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            إنشاء وترحيل قيد يومية جديد (Journal Voucher)
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground">
            إدخال قيود اليومية المزدوجة مع التحقق الفوري من توازن المدين والدائن
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <div class="space-y-4 py-3 overflow-y-auto max-h-[60vh] pr-1">
      <!-- Header Meta -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        <div class="space-y-1">
          <Label class="text-xs font-semibold">رقم السند المرجعي</Label>
          <Input type="text" bind:value={referenceNumber} class="h-8 text-xs font-mono" />
        </div>

        <div class="space-y-1">
          <Label class="text-xs font-semibold">تاريخ القيد</Label>
          <Input type="date" bind:value={accountingDate} class="h-8 text-xs" />
        </div>

        <div class="space-y-1">
          <Label class="text-xs font-semibold">عملة القيد</Label>
          <Select value={selectedCurrency} onValueChange={(v) => (selectedCurrency = v)}>
            <SelectTrigger class="h-8 text-xs">
              <span>{selectedCurrency}</span>
            </SelectTrigger>
            <SelectContent>
              {#each currencies as c}
                <SelectItem value={c.code}>{c.code} - {c.name}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="space-y-1">
        <Label class="text-xs font-semibold">البيان العام للقيد</Label>
        <Input
          type="text"
          placeholder="شرح موجز للعملية المالية أو القيد الافتتاحي..."
          bind:value={description}
          class="h-8 text-xs"
        />
      </div>

      <!-- Journal Lines Grid -->
      <div class="rounded-xl border border-border/70 overflow-hidden mt-4">
        <div class="bg-muted/40 px-3 py-2 border-b border-border/70 grid grid-cols-12 gap-2 text-xs font-bold text-foreground">
          <div class="col-span-5">الحساب المحاسبي</div>
          <div class="col-span-3">البيان الفرعي</div>
          <div class="col-span-2 text-end">مدين (Debit)</div>
          <div class="col-span-2 text-end">دائن (Credit)</div>
        </div>

        <div class="divide-y divide-border/50 p-2 space-y-2">
          {#each lines as line, index}
            <div class="grid grid-cols-12 gap-2 items-center">
              <div class="col-span-5">
                <AccountSearchInput
                  bind:value={line.accountId}
                  {accounts}
                  placeholder="اختر أو اكتب الحساب..."
                  size="sm"
                  class="w-full text-xs"
                />
              </div>

              <div class="col-span-3">
                <Input
                  type="text"
                  placeholder="بيان اختياري..."
                  bind:value={line.description}
                  class="h-8 text-xs"
                />
              </div>

              <div class="col-span-2">
                <Input
                  type="number"
                  step="0.01"
                  min="0"
                  bind:value={line.debit}
                  oninput={() => { if (line.debit > 0) line.credit = 0; }}
                  class="h-8 text-xs font-mono text-end"
                />
              </div>

              <div class="col-span-2 flex items-center gap-1">
                <Input
                  type="number"
                  step="0.01"
                  min="0"
                  bind:value={line.credit}
                  oninput={() => { if (line.credit > 0) line.debit = 0; }}
                  class="h-8 text-xs font-mono text-end"
                />
                <button
                  type="button"
                  onclick={() => removeLine(index)}
                  class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors"
                >
                  <Trash class="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          {/each}
        </div>

        <div class="p-2 border-t border-border/60 bg-muted/20">
          <Button variant="ghost" size="sm" onclick={addLine} class="h-7 text-xs font-semibold gap-1">
            <Plus class="w-3.5 h-3.5" />
            إضافة سطر حساب آخر
          </Button>
        </div>
      </div>

      <!-- Live Balance Verification Bar -->
      <div class="p-3 rounded-xl border flex flex-col sm:flex-row items-center justify-between gap-3 text-xs {isBalanced ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-700 dark:text-emerald-300' : 'bg-red-500/10 border-red-500/30 text-red-700 dark:text-red-300'}">
        <div class="flex items-center gap-2">
          {#if isBalanced}
            <CheckCircle class="w-4 h-4 text-emerald-600 shrink-0" />
            <span class="font-bold">القيد متوازن وجاهز للترحيل</span>
          {:else}
            <InfoCircle class="w-4 h-4 text-red-600 shrink-0" />
            <span class="font-bold">القيد غير متوازن! الفارق: {formatCurrency(difference, selectedCurrency)}</span>
          {/if}
        </div>

        <div class="flex items-center gap-4 font-mono font-bold">
          <div>إجمالي المدين: {formatCurrency(totalDebit, selectedCurrency)}</div>
          <div>إجمالي الدائن: {formatCurrency(totalCredit, selectedCurrency)}</div>
        </div>
      </div>
    </div>

    <DialogFooter class="border-t pt-3 flex items-center justify-end gap-2">
      <Button variant="outline" size="sm" onclick={() => (open = false)} disabled={isSubmitting}>
        إلغاء
      </Button>
      <Button
        variant="default"
        size="sm"
        onclick={handleSubmit}
        disabled={!isBalanced || isSubmitting}
        class="font-bold gap-1.5 shadow-sm"
      >
        <Scale class="w-3.5 h-3.5" />
        {isSubmitting ? "جاري الترحيل..." : "ترحيل القيد للميزانية"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
