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
  { value: "manual", label: m.common_dt_type_manual ? m.common_dt_type_manual() : "يدوي" },
  {
    value: "scholarship",
    label: m.common_dt_type_scholarship ? m.common_dt_type_scholarship() : "منحة دراسية",
  },
  {
    value: "system",
    label: m.common_dt_type_system ? m.common_dt_type_system() : "تلقائي بالنظام",
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
    label: m.common_dt_method_percentage ? m.common_dt_method_percentage() : "نسبة مئوية (%)",
  },
  {
    value: "fixed",
    label: m.common_dt_method_fixed ? m.common_dt_method_fixed() : "مبلغ ثابت",
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
    toast.error(
      m.common_status_undefined() === "غير محدد"
        ? "الاسم مطلوب"
        : "Name is required",
    );
    return;
  }
  if (value < 0) {
    toast.error(
      m.common_status_undefined() === "غير محدد"
        ? "يجب أن تكون القيمة رقمًا موجبًا"
        : "Value must be a positive number",
    );
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
      await commands.finance.updateDiscountTemplate(
        orgId,
        template.id,
        request,
      );
      toast.success(
        m.common_dt_update_success
          ? m.common_dt_update_success()
          : "Discount template updated successfully",
      );
    } else {
      await commands.finance.createDiscountTemplate(orgId, request);
      toast.success(
        m.common_dt_create_success
          ? m.common_dt_create_success()
          : "Discount template created successfully",
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
        ? (m.common_action_edit ? m.common_action_edit() : "تعديل قالب الخصم") 
        : (m.common_dt_sheet_create_title ? m.common_dt_sheet_create_title() : "إنشاء قالب خصم")}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
    <!-- Name -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_name" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_name ? m.common_dt_form_name() : "اسم الخصم / المنحة"} *
      </Label>
      <Input 
        id="dt_form_name"
        type="text" 
        bind:value={name}
        placeholder="e.g., Brother Discount (15%) or Scholarship"
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Code/Symbol -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_code" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_code ? m.common_dt_form_code() : "الكود / الرمز"}
      </Label>
      <Input 
        id="dt_form_code"
        type="text" 
        bind:value={code}
        placeholder="e.g., BRO_15"
        disabled={isSubmitting}
      />
    </div>

    <!-- Type & Calculation Method Grid -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Type -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold text-muted-foreground">
          {m.common_dt_form_type ? m.common_dt_form_type() : "نوع الخصم"} *
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
          {m.common_dt_col_method ? m.common_dt_col_method() : "طريقة الحساب"} *
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
        {m.common_dt_col_value ? m.common_dt_col_value() : "القيمة"} *
      </Label>
      <Input 
        id="dt_form_value"
        type="number" 
        min="0"
        bind:value={value}
        placeholder={calculationMethod === 'percentage' ? 'e.g., 15' : 'e.g., 50000'}
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Priority (optional) -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="dt_form_priority" class="text-xs font-semibold text-muted-foreground">
        {m.common_dt_form_priority ? m.common_dt_form_priority() : "الأولوية"}
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
          {m.common_dt_form_auto_apply ? m.common_dt_form_auto_apply() : "تطبيق تلقائي"}
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
          {m.common_dt_form_stackable ? m.common_dt_form_stackable() : "قابل للدمج / التداخل"}
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
          {m.common_dt_form_requires_approval ? m.common_dt_form_requires_approval() : "يتطلب موافقة الإدارة"}
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
          {m.common_status_active ? m.common_status_active() : "نشط"}
        </Label>
      </div>
    </div>
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
      {m.common_action_cancel ? m.common_action_cancel() : "إلغاء"}
    </Button>
    <Button 
      type="submit" 
      disabled={isSubmitting}
      class="flex-1 h-9 text-sm font-semibold"
    >
      {#if isSubmitting}
        {m.common_state_saving ? m.common_state_saving() : "جاري الحفظ..."}
      {:else}
        {m.common_action_save ? m.common_action_save() : "حفظ قالب الخصم"}
      {/if}
    </Button>
  </div>
</form>
