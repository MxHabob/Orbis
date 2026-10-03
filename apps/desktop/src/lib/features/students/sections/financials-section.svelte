<!--
  @component FinancialsSection
  Tuition fee schedule, scholarships, discounts, and optional auxiliary charges.
  Calculates all fees in the organization's official currency.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Switch } from "@orbis/ui/switch";
import { Checkbox } from "@orbis/ui/checkbox";
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
import { Tag, Sparkles, BookOpen } from "@orbis/icons";
import { formatCurrency } from "$lib/core/i18n";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="flex flex-col gap-6">
  
  <!-- Base Tuition Fee Schedule Card -->
  <Card class="w-full border-border/80 shadow-xs">
    <CardHeader class="py-3 px-6 border-b border-border/40 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Tag class="w-4 h-4 text-primary" />
        <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
          {m.ui_copy_approved_base_tuition_schedule_106c2f8b()}
        </CardTitle>
      </div>
      <span class="text-xs font-mono font-bold text-primary">
        {formatCurrency(state.mandatoryTotal, state.currency)}
      </span>
    </CardHeader>

    <CardContent class="p-6">
      <div class="flex flex-col gap-3 text-start">
        <div class="flex justify-between items-center text-xs">
          <span class="text-muted-foreground">
            {m.ui_copy_selected_class_f9c503a7()} <strong class="text-foreground">{state.selectedClassName}</strong>
          </span>
          <span class="text-[11px] text-muted-foreground">
            ({state.mandatoryFeesList.length} {m.ui_copy_fee_items_5a58e60b()})
          </span>
        </div>

        {#if state.mandatoryFeesList.length > 0}
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-2">
            {#each state.mandatoryFeesList as feeTpl}
              <div class="p-3.5 rounded-lg bg-muted/20 border border-border flex items-center justify-between text-xs text-start">
                <div class="flex flex-col gap-0.5">
                  <span class="font-bold text-foreground">{feeTpl.name_ar || feeTpl.name}</span>
                  <span class="text-[10px] text-muted-foreground capitalize">{feeTpl.category || m.common_ft_category_tuition()}</span>
                </div>
                <span class="font-mono font-bold text-foreground">
                  {formatCurrency(Number(feeTpl.default_amount) || 0, state.currency)}
                </span>
              </div>
            {/each}
          </div>
        {:else}
          <div class="p-4 rounded-lg bg-muted/20 border border-border text-center text-xs text-muted-foreground">
            {m.ui_copy_no_mandatory_fee_templates_mapped_for_this_e7b59267()}
          </div>
        {/if}
      </div>
    </CardContent>
  </Card>

  <!-- Scholarships & Discounts Card -->
  <Card class="w-full border-border/80 shadow-xs">
    <CardHeader class="py-3 px-6 flex flex-row items-center justify-between border-b border-border/40">
      <div class="flex items-center gap-2">
        <Sparkles class="w-4 h-4 text-primary" />
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
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">
                {m.ui_copy_discount_policy_type_1b319712()}
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">
                {m.ui_copy_apply_discount_from_approved_templates_or__043a0d02()}
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
                  {m.ui_copy_discount_template_9e6c5879()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">
                  {m.ui_copy_select_one_of_the_approved_discount_templa_44351fda()}
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
                      {state.financial.discountTemplates.find(d => d.id === state.financial.selectedDiscountTemplateId)?.name || m.ui_copy_select_template_2e406eb0()}
                    </span>
                  </SelectTrigger>
                  <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                    {#each state.financial.discountTemplates as tpl}
                      <SelectItem value={tpl.id} class="text-xs text-start cursor-pointer justify-start">
                        {tpl.name_ar || tpl.name} ({tpl.calculation_method === 'percentage' ? `${tpl.value}%` : formatCurrency(Number(tpl.value), state.currency)})
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
                  {m.ui_copy_enter_direct_discount_amount_ada748b6()}
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

  <!-- Optional Auxiliary Services Card -->
  <Card class="w-full border-border/80 shadow-xs">
    <CardHeader class="py-3 px-6 border-b border-border/40">
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {m.common_form_optional_fees_section()}
      </CardTitle>
    </CardHeader>

    <CardContent class="p-6">
      {#if state.optionalFeesList.length === 0}
        <p class="text-xs text-muted-foreground text-center py-4">
          {m.ui_copy_no_auxiliary_services_or_optional_fees_con_fbfc0561()}
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
                    {fTpl.category || m.ui_copy_auxiliary_service_9cec34df()}
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
