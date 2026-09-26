<!--
  @component FinancialsSection
  Tuition fees, discounts, and optional auxiliary charges calculator.
  Provides real-time dynamic total calculations.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Switch } from "@orbis/ui/switch";
import { Checkbox } from "@orbis/ui/checkbox";
import { Separator } from "@orbis/ui/separator";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Tag, Sparkles } from "@orbis/icons";
import { l, formatCurrency } from "$lib/core/i18n";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
  <!-- Inputs Column -->
  <div class="lg:col-span-2 flex flex-col gap-6">
    
    <!-- Discounts Container -->
    <Card class="w-full">
      <CardHeader class="py-3 px-6 flex flex-row items-center justify-between border-b border-border/40">
        <div class="flex items-center gap-2">
          <Tag class="w-4 h-4 text-primary" />
          <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
            {m.common_form_discount_section()}
          </CardTitle>
        </div>
        <CardAction>
          <div class="flex items-center gap-2">
            <Label class="text-[11px] font-semibold text-muted-foreground cursor-pointer" for="applyDiscountSwitch">
              {m.common_form_apply_discount()}
            </Label>
            <Switch id="applyDiscountSwitch" bind:checked={state.financial.applyDiscount} />
          </div>
        </CardAction>
      </CardHeader>

      {#if state.financial.applyDiscount}
        <CardContent class="p-0">
          <div class="flex flex-col divide-y divide-border">
            <!-- Discount Type -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label class="text-xs font-semibold text-foreground">
                  {l("نوع المنحة أو الخصم", "Discount Type")}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">
                  {l("تطبيق خصم من قالب معتمد في النظام أو إدخال مبلغ مالي مخصص", "Apply discount from a pre-configured template or specify a custom amount")}
                </span>
              </div>
              <div class="w-full md:w-120 shrink-0 flex gap-2">
                <Button
                  variant="ghost"
                  type="button"
                  class="flex-1 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.financial.discountType === 'template' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                  onclick={() => (state.financial.discountType = 'template')}
                >
                  <span>{m.common_form_discount_type_template()}</span>
                </Button>
                <Button 
                  variant="ghost" 
                  type="button"
                  class="flex-1 border rounded-lg text-xs h-9 cursor-pointer transition-all duration-150 {state.financial.discountType === 'custom' ? 'border-primary bg-primary/10 font-bold text-primary shadow-xs' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                  onclick={() => (state.financial.discountType = 'custom')}
                >
                  <span>{m.common_form_discount_type_custom()}</span>
                </Button>
              </div>
            </div>

            <!-- Discount Selection / Input -->
            {#if state.financial.discountType === 'template'}
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label class="text-xs font-semibold text-foreground">
                    {l("قالب المنحة / الخصم", "Discount Template")}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {l("اختر إحدى لوائح وقوالب الخصومات المعتمدة", "Select one of the approved discount templates")}
                  </span>
                </div>
                <div class="w-full md:w-120 shrink-0">
                  <Select 
                    type="single"
                    value={state.financial.selectedDiscountTemplateId}
                    onValueChange={(val) => (state.financial.selectedDiscountTemplateId = val || "")}
                  >
                    <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between bg-background">
                      <span class="truncate">
                        {state.financial.discountTemplates.find(d => d.id === state.financial.selectedDiscountTemplateId)?.name || l("اختر قالب الخصم...", "Select template...")}
                      </span>
                    </SelectTrigger>
                    <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                      {#each state.financial.discountTemplates as tpl}
                        <SelectItem value={tpl.id} class="text-xs text-start cursor-pointer justify-start">
                          {tpl.name_ar || tpl.name} ({tpl.calculation_method === 'percentage' ? `${tpl.value}%` : formatCurrency(Number(tpl.value))})
                        </SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              </div>
            {:else}
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="customDiscount" class="text-xs font-semibold text-foreground">{m.common_form_discount_custom_amount()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">
                    {l("إدخال مبلغ الخصم المالي المباشر", "Enter direct discount amount")}
                  </span>
                </div>
                <div class="w-full md:w-120 shrink-0">
                  <Input 
                    id="customDiscount"
                    type="number"
                    min="0"
                    placeholder="e.g. 50000"
                    class="h-9 w-full bg-background font-mono"
                    bind:value={state.financial.customDiscountAmount}
                  />
                </div>
              </div>
            {/if}
          </div>
        </CardContent>
      {/if}
    </Card>

    <!-- Optional Fees Container -->
    <Card class="w-full">
      <CardHeader class="py-3 px-6 border-b border-border/40">
        <div class="flex items-center gap-2">
          <Sparkles class="w-4 h-4 text-primary" />
          <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
            {m.common_form_optional_fees_section()}
          </CardTitle>
        </div>
      </CardHeader>

      <CardContent class="p-6">
        {#if state.optionalFeesList.length === 0}
          <p class="text-xs text-muted-foreground text-center py-4">
            {l("لا توجد رسوم أو خدمات اختيارية مضافة في النظام حالياً.", "No optional fees registered in the system currently.")}
          </p>
        {:else}
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            {#each state.optionalFeesList as fTpl}
              {@const isChecked = state.financial.selectedOptionalFees[fTpl.id]?.checked || false}
              <div class="flex items-center justify-between p-3.5 border rounded-lg bg-background hover:bg-accent/5 transition-all duration-150 gap-4 text-start {isChecked ? 'border-primary/40 bg-primary/5' : 'border-border'}">
                <div class="flex items-start gap-2.5">
                  <Checkbox 
                    id={`fee_${fTpl.id}`} 
                    checked={isChecked}
                    onCheckedChange={(checked) => {
                      if (state.financial.selectedOptionalFees[fTpl.id]) {
                        state.financial.selectedOptionalFees[fTpl.id].checked = !!checked;
                      }
                    }}
                  />
                  <div class="flex flex-col gap-0.5 select-none">
                    <Label for={`fee_${fTpl.id}`} class="text-xs font-bold text-foreground cursor-pointer">
                      {fTpl.name_ar || fTpl.name}
                    </Label>
                    <span class="text-[10px] text-muted-foreground capitalize">
                      {fTpl.category || l("رسوم إضافية", "Auxiliary Fee")}
                    </span>
                  </div>
                </div>

                <div class="w-28 shrink-0 flex items-center gap-1.5">
                  <Input 
                    type="number"
                    disabled={!isChecked}
                    class="text-xs h-8 text-center font-bold px-1 bg-background font-mono"
                    bind:value={state.financial.selectedOptionalFees[fTpl.id].amount}
                  />
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </CardContent>
    </Card>

  </div>

  <!-- Live Calculator Summary Column -->
  <div class="flex flex-col gap-6">
    <Card class="w-full sticky top-6">
      <CardHeader class="py-3 px-6 border-b border-border/40">
        <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
          {m.common_form_summary_section()}
        </CardTitle>
      </CardHeader>

      <CardContent class="p-6 flex flex-col gap-4 text-start">
        <!-- Mandatory Fees -->
        <div class="flex justify-between items-center text-xs">
          <span class="text-muted-foreground">{m.common_form_summary_mandatory()}:</span>
          <span class="font-bold text-foreground font-mono">{formatCurrency(state.mandatoryTotal)}</span>
        </div>

        <!-- Optional Fees -->
        <div class="flex justify-between items-center text-xs">
          <span class="text-muted-foreground">{m.common_form_summary_optional()}:</span>
          <span class="font-bold text-foreground font-mono">+{formatCurrency(state.optionalTotal)}</span>
        </div>

        <!-- Discount Total -->
        <div class="flex justify-between items-center text-xs">
          <span class="text-muted-foreground">{m.common_form_summary_discount()}:</span>
          <span class="font-bold text-destructive font-mono">-{formatCurrency(state.discountTotal)}</span>
        </div>

        <Separator class="bg-border/60 my-1" />

        <!-- Net Total -->
        <div class="flex justify-between items-center">
          <span class="text-xs font-bold text-foreground">{m.common_form_summary_net()}:</span>
          <span class="text-base font-black text-primary font-mono">{formatCurrency(state.netTotal)}</span>
        </div>

        <!-- Mandatory Breakdown -->
        {#if state.mandatoryFeesList.length > 0}
          <div class="mt-4 p-3 rounded-lg bg-muted/20 border border-border space-y-2 select-none">
            <span class="text-[10px] font-bold text-muted-foreground block border-b border-border/40 pb-1">
              {l("تفاصيل الرسوم الأساسية المعتمدة للصف:", "Approved basic fees breakdown for class:")}
            </span>
            {#each state.mandatoryFeesList as feeTpl}
              <div class="flex justify-between items-center text-[10px]">
                <span class="text-muted-foreground truncate max-w-35">
                  {feeTpl.name_ar || feeTpl.name}
                </span>
                <span class="font-mono text-foreground font-semibold">{formatCurrency(Number(feeTpl.default_amount) || 0)}</span>
              </div>
            {/each}
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>
