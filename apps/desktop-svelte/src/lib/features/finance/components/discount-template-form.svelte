<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import * as Sheet from "@orbis/ui-svelte/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";
import { toast } from "svelte-sonner";

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

// Form state
let isSubmitting = $state(false);

let name = $state("");
let code = $state("");
let itemType = $state("manual");
let calculationMethod = $state("percentage");
let value = $state<number>(0);
let requiresApproval = $state(false);
let isAutoApply = $state(false);
let isStackable = $state(false);
let priority = $state<number | null>(null);
let isActive = $state(true);

// Options for Type
const itemTypes = $derived([
  {
    value: "manual",
    label: m.common_dt_type_manual(),
  },
  {
    value: "scholarship",
    label: m.common_dt_type_scholarship(),
  },
  {
    value: "system",
    label: m.common_dt_type_system(),
  },
]);
const itemTypeLabels = $derived(
  itemTypes.reduce(
    (acc, t) => ({ ...acc, [t.value]: t.label }),
    {} as Record<string, string>,
  ),
);
const selectedItemTypeLabel = $derived(itemTypeLabels[itemType] ?? itemType);

// Options for Calculation Method
const calculationMethods = $derived([
  {
    value: "percentage",
    label: m.common_dt_method_percentage(),
  },
  {
    value: "fixed",
    label: m.common_dt_method_fixed(),
  },
]);
const calculationMethodLabels = $derived(
  calculationMethods.reduce(
    (acc, cm) => ({ ...acc, [cm.value]: cm.label }),
    {} as Record<string, string>,
  ),
);
const selectedCalculationMethodLabel = $derived(
  calculationMethodLabels[calculationMethod] ?? calculationMethod,
);

// Initialize form if in edit mode
$effect(() => {
  if (template) {
    name = template.name || "";
    code = template.code || "";
    itemType = template.item_type || "manual";
    calculationMethod = template.calculation_method || "percentage";
    value = Number(template.value || 0);
    requiresApproval = template.requires_approval === 1;
    isAutoApply = template.is_auto_apply === 1;
    isStackable = template.is_stackable === 1;
    priority = template.priority;
    isActive = template.is_active === 1;
  }
});

async function handleSubmit() {
  if (!name.trim()) {
    toast.error(m.common_dt_error_name_required());
    return;
  }
  if (value < 0) {
    toast.error(m.common_dt_error_value_positive());
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name.trim(),
      code: code.trim() || null,
      item_type: itemType,
      calculation_method: calculationMethod,
      value: Math.round(value),
      eligibility_rules: null,
      limits: null,
      requires_approval: requiresApproval ? 1 : 0,
      approver_roles: null,
      is_auto_apply: isAutoApply ? 1 : 0,
      is_stackable: isStackable ? 1 : 0,
      priority: priority,
      is_active: isActive ? 1 : 0,
    };

    if (template) {
      await commands.finance.updateDiscountTemplate(template.id, request);
      toast.success(
        m.common_dt_update_success(),
      );
    } else {
      await commands.finance.createDiscountTemplate(request);
      toast.success(
        m.common_dt_create_success(),
      );
    }

    onSuccess();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form 
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }} 
  class="flex flex-col h-full overflow-hidden gap-0"
>
  <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0 text-start">
    <Sheet.Title class="text-base font-bold text-foreground">
      {template 
        ? m.common_action_edit()
        : m.common_dt_sheet_create_title()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["details", "options"]} class="flex flex-col gap-4">
      <AccordionItem value="details" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_dt_form_name()}</AccordionTrigger>
        <AccordionContent>
    <!-- Name -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_name" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_name()} *
      </Label>
      <Input 
        id="dt_form_name"
        type="text" 
        bind:value={name}
        placeholder={m.common_dt_name_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Code/Symbol -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_code" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_code()}
      </Label>
      <Input 
        id="dt_form_code"
        type="text" 
        bind:value={code}
        placeholder={m.common_dt_code_placeholder()}
        disabled={isSubmitting}
      />
    </div>

    <!-- Type & Calculation Method Grid -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Type -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold text-muted-foreground">
          {m.common_dt_form_type()} *
        </Label>
        <Select type="single" value={itemType} onValueChange={(v) => (itemType = v ?? "manual")} disabled={isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedItemTypeLabel}
          </SelectTrigger>
          <SelectContent>
            {#each itemTypes as t (t.value)}
              <SelectItem value={t.value}>{t.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Calculation Method -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold text-muted-foreground">
          {m.common_dt_col_method()} *
        </Label>
        <Select type="single" value={calculationMethod} onValueChange={(v) => (calculationMethod = v ?? "percentage")} disabled={isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedCalculationMethodLabel}
          </SelectTrigger>
          <SelectContent>
            {#each calculationMethods as cm (cm.value)}
              <SelectItem value={cm.value}>{cm.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Value -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_value" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_col_value()} *
      </Label>
      <Input 
        id="dt_form_value"
        type="number" 
        min="0"
        bind:value={value}
        placeholder={calculationMethod === 'percentage' ? m.common_dt_value_percentage_placeholder() : m.common_dt_value_fixed_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Priority (optional) -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_priority" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_priority()}
      </Label>
      <Input 
        id="dt_form_priority"
        type="number" 
        min="1"
        bind:value={priority}
        placeholder={m.common_dt_form_priority_placeholder ? m.common_dt_form_priority_placeholder() : "e.g., 1 (highest)"}
        disabled={isSubmitting}
      />
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="options" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_dt_form_auto_apply()}</AccordionTrigger>
        <AccordionContent>

    <!-- Options Checkboxes Grid -->
    <div class="grid grid-cols-2 gap-4 mt-2 bg-muted/20 p-4 rounded-lg border border-border/30">
      <!-- Auto Apply -->
      <div class="flex items-center gap-2">
        <Checkbox 
          id="dt_auto_apply_chk" 
          checked={isAutoApply} 
          onCheckedChange={(val) => isAutoApply = !!val} 
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
          checked={isStackable} 
          onCheckedChange={(val) => isStackable = !!val} 
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
          checked={requiresApproval} 
          onCheckedChange={(val) => requiresApproval = !!val} 
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
          checked={isActive} 
          onCheckedChange={(val) => isActive = !!val} 
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
  <div class="px-6 py-4 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button 
      type="button" 
      variant="outline" 
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
