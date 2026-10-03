<!--
  @component FinanceSettingsCard
  Configures institutional finance policies and invoice defaults.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Input } from "@orbis/ui/input";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
import { ReceiptPercent } from "@orbis/icons";


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
        <CardTitle>{m.ui_copy_billing_financial_policies_9353d39e()}</CardTitle>
        <CardDescription>{m.ui_copy_define_institutional_tax_rates_invoice_due_f4132822()}</CardDescription>
      </div>
    </div>
  </CardHeader>
  <CardContent class="flex flex-col gap-5">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{m.ui_copy_default_tax_rate_1ebc240f()}</Label>
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
        <span class="text-[10px] text-muted-foreground">{m.ui_copy_e_g_0_15_represents_15_vat_3eb644d1()}</span>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{m.ui_copy_invoice_due_days_604a36c8()}</Label>
        <Input
          type="number"
          min="1"
          max="365"
          bind:value={invoiceDueDays}
          disabled={isSaving}
          class="h-8 text-xs w-36"
        />
        <span class="text-[10px] text-muted-foreground">{m.ui_copy_days_allowed_for_payment_before_becoming_o_be9960ff()}</span>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{m.ui_copy_late_fee_type_e7186de3()}</Label>
        <Select
          type="single"
          value={lateFeeType}
          onValueChange={(v) => { if (v) lateFeeType = v; }}
          disabled={isSaving}
        >
          <SelectTrigger class="h-8 text-xs w-48">
            <span>{lateFeeType === "percentage" ? m.common_cycle_form_grading_percentage() : m.common_dt_method_fixed()}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="fixed">{m.common_dt_method_fixed()}</SelectItem>
            <SelectItem value="percentage">{m.common_dt_method_percentage()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{m.ui_copy_late_fee_value_6da26129()}</Label>
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
          <Label class="text-xs font-semibold">{m.ui_copy_allow_partial_payments_4d1af6e1()}</Label>
          <span class="text-[11px] text-muted-foreground">{m.ui_copy_enable_paying_invoice_fees_in_installments_5caee8f9()}</span>
        </div>
        <Switch bind:checked={allowPartialPayments} disabled={isSaving} />
      </div>
    </div>
  </CardContent>
</Card>
