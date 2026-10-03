<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import * as Sheet from "@orbis/ui/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { toast, FormField } from "@orbis/ui";
import { discountTemplateInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  orgId,
  template, // Optional, if provided we are in EDIT mode
  onSuccess,
  onCancel,
}: {
  orgId: string;
  template?: any;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const form = new FormController({
  initialValues: {
    name: template?.name ?? "",
    code: template?.code ?? "",
    item_type: template?.item_type ?? "manual",
    calculation_method: template?.calculation_method ?? "percentage",
    value: Number(template?.value ?? 0),
    priority: template?.priority ?? null as number | null,
    requires_approval: template ? Number(template.requires_approval ?? 0) : 0,
    is_auto_apply: template ? Number(template.is_auto_apply ?? 0) : 0,
    is_stackable: template ? Number(template.is_stackable ?? 0) : 0,
    is_active: template ? Number(template.is_active ?? 1) : 1,
  },
  schema: discountTemplateInputSchema,
});

const isSubmitting = $derived(form.isSubmitting);

// Static options definition
const itemTypes = [
  { value: "manual", label: m.common_dt_type_manual() },
  { value: "scholarship", label: m.common_dt_type_scholarship() },
  { value: "system", label: m.common_dt_type_system() },
];

const calculationMethods = [
  { value: "percentage", label: m.common_dt_method_percentage() },
  { value: "fixed", label: m.common_dt_method_fixed() },
];

const selectedItemTypeLabel = $derived(
  itemTypes.find((t) => t.value === form.values.item_type)?.label ?? form.values.item_type
);

const selectedCalculationMethodLabel = $derived(
  calculationMethods.find((cm) => cm.value === form.values.calculation_method)?.label ?? form.values.calculation_method
);

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    const request = {
      ...payload,
      value: Math.round(Number(payload.value)),
      eligibility_rules: null,
      limits: null,
      approver_roles: null,
    };

    if (template?.id) {
      toast.info(m.common_dt_update_success());
    } else {
      await api.createDiscountTemplate({
        code: request.code || undefined,
        name: request.name,
        discountType: request.item_type || "manual",
        value: request.value,
        isApprovalRequired: Boolean(payload.requires_approval),
        isAutoApply: Boolean(payload.is_auto_apply),
        isStackable: Boolean(payload.is_stackable),
        priority: payload.priority ?? undefined,
      });
      toast.success(m.common_dt_create_success());
    }
  });

  if (submitted) {
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(m.common_dt_error_name_required());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
    <Sheet.Title class="text-base font-bold text-foreground">
      {template ? m.common_action_edit() : m.common_dt_sheet_create_title()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["details", "options"]} class="flex flex-col gap-4">
      <AccordionItem value="details" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_dt_form_name()}</AccordionTrigger>
        <AccordionContent>
          <!-- Name -->
          <FormField id="dt_form_name" label={m.common_dt_form_name()} error={form.errors.name?.[0]} required>
            {#snippet children()}
              <Input 
                id="dt_form_name"
                type="text" 
                bind:value={form.values.name}
                onblur={() => form.markTouched("name")}
                placeholder={m.common_dt_name_placeholder()}
                required
                disabled={isSubmitting}
              />
            {/snippet}
          </FormField>

          <!-- Code/Symbol -->
          <FormField id="dt_form_code" label={m.common_dt_form_code()} error={form.errors.code?.[0]}>
            {#snippet children()}
              <Input 
                id="dt_form_code"
                type="text" 
                bind:value={form.values.code}
                placeholder={m.common_dt_code_placeholder()}
                disabled={isSubmitting}
              />
            {/snippet}
          </FormField>

          <!-- Type & Calculation Method Grid -->
          <div class="grid grid-cols-2 gap-4">
            <!-- Type -->
            <FormField label={m.common_dt_form_type()} error={form.errors.item_type?.[0]} required>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.item_type}
                  onValueChange={(v) => (form.values.item_type = v ?? "manual")}
                  disabled={isSubmitting}
                >
                  <SelectTrigger class="h-9 text-sm">
                    {selectedItemTypeLabel}
                  </SelectTrigger>
                  <SelectContent>
                    {#each itemTypes as t (t.value)}
                      <SelectItem value={t.value}>{t.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>

            <!-- Calculation Method -->
            <FormField label={m.common_dt_col_method()} error={form.errors.calculation_method?.[0]} required>
              {#snippet children()}
                <Select
                  type="single"
                  value={form.values.calculation_method}
                  onValueChange={(v) => (form.values.calculation_method = v ?? "percentage")}
                  disabled={isSubmitting}
                >
                  <SelectTrigger class="h-9 text-sm">
                    {selectedCalculationMethodLabel}
                  </SelectTrigger>
                  <SelectContent>
                    {#each calculationMethods as cm (cm.value)}
                      <SelectItem value={cm.value}>{cm.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>
          </div>

          <!-- Value -->
          <FormField id="dt_form_value" label={m.common_dt_col_value()} error={form.errors.value?.[0]} required>
            {#snippet children()}
              <Input 
                id="dt_form_value"
                type="number" 
                min="0"
                step="any"
                bind:value={form.values.value}
                onblur={() => form.markTouched("value")}
                placeholder={form.values.calculation_method === 'percentage' ? m.common_dt_value_percentage_placeholder() : m.common_dt_value_fixed_placeholder()}
                required
                disabled={isSubmitting}
              />
            {/snippet}
          </FormField>

          <!-- Priority (optional) -->
          <FormField id="dt_form_priority" label={m.common_dt_form_priority()} error={form.errors.priority?.[0]}>
            {#snippet children()}
              <Input 
                id="dt_form_priority"
                type="number" 
                min="1"
                bind:value={form.values.priority}
                placeholder={m.common_dt_form_priority_placeholder()}
                disabled={isSubmitting}
              />
            {/snippet}
          </FormField>
        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="options" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_dt_form_auto_apply()}</AccordionTrigger>
        <AccordionContent>
          <!-- Options Checkboxes Grid -->
          <div class="grid grid-cols-2 gap-4 mt-2 bg-muted/20 p-4 rounded-lg border border-border/30">
            <!-- Auto Apply -->
            <div class="flex items-center gap-2">
              <Checkbox 
                id="dt_auto_apply_chk" 
                checked={form.values.is_auto_apply === 1} 
                onCheckedChange={(val) => form.values.is_auto_apply = val ? 1 : 0} 
                disabled={isSubmitting}
              />
              <Label for="dt_auto_apply_chk" class="text-xs cursor-pointer select-none">
                {m.common_dt_form_auto_apply()}
              </Label>
            </div>

            <!-- Stackable -->
            <div class="flex items-center gap-2">
              <Checkbox 
                id="dt_stackable_chk" 
                checked={form.values.is_stackable === 1} 
                onCheckedChange={(val) => form.values.is_stackable = val ? 1 : 0} 
                disabled={isSubmitting}
              />
              <Label for="dt_stackable_chk" class="text-xs cursor-pointer select-none">
                {m.common_dt_form_stackable()}
              </Label>
            </div>

            <!-- Requires Approval -->
            <div class="flex items-center gap-2">
              <Checkbox 
                id="dt_requires_approval_chk" 
                checked={form.values.requires_approval === 1} 
                onCheckedChange={(val) => form.values.requires_approval = val ? 1 : 0} 
                disabled={isSubmitting}
              />
              <Label for="dt_requires_approval_chk" class="text-xs cursor-pointer select-none">
                {m.common_dt_form_requires_approval()}
              </Label>
            </div>

            <!-- Is Active -->
            <div class="flex items-center gap-2">
              <Checkbox 
                id="dt_active_chk" 
                checked={form.values.is_active === 1} 
                onCheckedChange={(val) => form.values.is_active = val ? 1 : 0} 
                disabled={isSubmitting}
              />
              <Label for="dt_active_chk" class="text-xs cursor-pointer select-none">
                {m.common_phase_active()}
              </Label>
            </div>
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>

  <!-- Form Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button 
      type="button" 
      variant="ghost" 
      onclick={onCancel}
      disabled={isSubmitting}
      class="flex-1 h-9 text-sm"
    >
      {m.common_action_cancel()}
    </Button>
    <Button 
      type="submit" 
      disabled={isSubmitting}
      class="flex-1 h-9 text-sm font-semibold"
    >
      {#if isSubmitting}
        {m.common_state_saving()}
      {:else}
        {m.common_action_save()}
      {/if}
    </Button>
  </div>
</form>
