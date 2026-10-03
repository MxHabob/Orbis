<!--
  @component FinanceSettingsCard
  Configures institutional finance policies and invoice defaults.
-->
<script lang="ts">
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Input } from "@orbis/ui/input";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
import { ReceiptPercent } from "@orbis/icons";
import { l } from "$lib/core/i18n";

let {
  currency = $bindable("SAR"),
  taxRate = $bindable(0.15),
  invoiceDueDays = $bindable(14),
  lateFeeType = $bindable("fixed"),
  lateFeeValue = $bindable(0.0),
  allowPartialPayments = $bindable(false),
  isSaving = false,
}: {
  currency?: string;
  taxRate?: number;
  invoiceDueDays?: number;
  lateFeeType?: string;
  lateFeeValue?: number;
  allowPartialPayments?: boolean;
  isSaving?: boolean;
} = $props();
</script>

<Card class="border-border/80 shadow-xs">
  <CardHeader>
    <div class="flex items-center gap-3 text-start">
      <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
        <ReceiptPercent class="w-4 h-4" />
      </div>
      <div class="flex flex-col">
        <CardTitle>{l("سياسات الفواتير والتحصيل المالي", "Billing & Financial Policies")}</CardTitle>
        <CardDescription>{l("تحديد نسب الضريبة وشروط الاستحقاق وغرامات التأخير", "Define institutional tax rates, invoice due terms, and late fee policies")}</CardDescription>
      </div>
    </div>
  </CardHeader>
  <CardContent class="flex flex-col gap-5">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{l("نسبة الضريبة الافتراضية (%)", "Default Tax Rate (%)")}</Label>
        <Input
          type="number"
          step="0.01"
          min="0"
          max="1"
          value={taxRate}
          oninput={(e) => taxRate = parseFloat(e.currentTarget.value) || 0}
          disabled={isSaving}
          class="h-8 text-xs w-36"
        />
        <span class="text-[10px] text-muted-foreground">{l("مثال: 0.15 تمثل ضريبة 15%", "e.g. 0.15 represents 15% VAT")}</span>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{l("فترة استحقاق الفواتير (أيام)", "Invoice Due Days")}</Label>
        <Input
          type="number"
          min="1"
          max="365"
          bind:value={invoiceDueDays}
          disabled={isSaving}
          class="h-8 text-xs w-36"
        />
        <span class="text-[10px] text-muted-foreground">{l("عدد الأيام المسموح بها للسداد قبل استحقاق الفاتورة", "Days allowed for payment before becoming overdue")}</span>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{l("نوع غرامة التأخير", "Late Fee Type")}</Label>
        <Select
          type="single"
          value={lateFeeType}
          onValueChange={(v) => { if (v) lateFeeType = v; }}
          disabled={isSaving}
        >
          <SelectTrigger class="h-8 text-xs w-48">
            <span>{lateFeeType === "percentage" ? l("نسبة مئوية", "Percentage") : l("مبلغ ثابت", "Fixed Amount")}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="fixed">{l("مبلغ ثابت", "Fixed Amount")}</SelectItem>
            <SelectItem value="percentage">{l("نسبة مئوية (%)", "Percentage (%)")}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{l("قيمة غرامة التأخير", "Late Fee Value")}</Label>
        <Input
          type="number"
          step="0.01"
          min="0"
          value={lateFeeValue}
          oninput={(e) => lateFeeValue = parseFloat(e.currentTarget.value) || 0}
          disabled={isSaving}
          class="h-8 text-xs w-36"
        />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20 md:col-span-2">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("السماح بالدفع الجزئي", "Allow Partial Payments")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("تمكين سداد رسوم الفاتورة على دفعات متعددة", "Enable paying invoice fees in installments")}</span>
        </div>
        <Switch bind:checked={allowPartialPayments} disabled={isSaving} />
      </div>
    </div>
  </CardContent>
</Card>
