<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

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

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!name.trim()) {
    toast.error("Name is required");
    return;
  }
  if (value < 0) {
    toast.error("Value must be a positive number");
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
      await commands.finance.updateDiscountTemplate(orgId, template.id, request);
      toast.success(m.dt_update_success ? m.dt_update_success() : "Discount template updated successfully");
    } else {
      await commands.finance.createDiscountTemplate(orgId, request);
      toast.success(m.dt_create_success ? m.dt_create_success() : "Discount template created successfully");
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-5 scrollbar-hide">
  <!-- Name -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="dt_form_name" class="text-xs font-semibold text-foreground">
      {m.dt_form_name ? m.dt_form_name() : "اسم الخصم / المنحة"}
    </label>
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
    <label for="dt_form_code" class="text-xs font-semibold text-foreground">
      {m.dt_form_code ? m.dt_form_code() : "الكود / الرمز"}
    </label>
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
      <label for="dt_form_type" class="text-xs font-semibold text-foreground">
        {m.attendance_status_unknown() === "غير محدد" ? "نوع الخصم" : "Discount Type"}
      </label>
      <select 
        id="dt_form_type"
        bind:value={itemType}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isSubmitting}
      >
        <option value="manual">{m.attendance_status_unknown() === "غير محدد" ? "يدوي" : "Manual"}</option>
        <option value="scholarship">{m.attendance_status_unknown() === "غير محدد" ? "منحة دراسية" : "Scholarship"}</option>
        <option value="system">{m.attendance_status_unknown() === "غير محدد" ? "تلقائي بالنظام" : "System"}</option>
      </select>
    </div>

    <!-- Calculation Method -->
    <div class="flex flex-col gap-1.5 text-start">
      <label for="dt_form_method" class="text-xs font-semibold text-foreground">
        {m.dt_form_method ? m.dt_form_method() : "طريقة الحساب"}
      </label>
      <select 
        id="dt_form_method"
        bind:value={calculationMethod}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isSubmitting}
      >
        <option value="percentage">{m.dt_method_percentage ? m.dt_method_percentage() : "نسبة مئوية (%)"}</option>
        <option value="fixed">{m.dt_method_fixed ? m.dt_method_fixed() : "مبلغ ثابت"}</option>
      </select>
    </div>
  </div>

  <!-- Value -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="dt_form_value" class="text-xs font-semibold text-foreground">
      {m.dt_form_value ? m.dt_form_value() : "القيمة"}
    </label>
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
    <label for="dt_form_priority" class="text-xs font-semibold text-foreground">
      {m.attendance_status_unknown() === "غير محدد" ? "الأولوية" : "Priority"}
    </label>
    <Input 
      id="dt_form_priority"
      type="number" 
      min="1"
      bind:value={priority}
      placeholder="e.g., 1 (highest)"
      disabled={isSubmitting}
    />
  </div>

  <!-- Options Checkboxes Grid -->
  <div class="grid grid-cols-2 gap-4 mt-2">
    <!-- Auto Apply -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={isAutoApply} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.attendance_status_unknown() === "غير محدد" ? "تطبيق تلقائي" : "Auto Apply"}</span>
    </label>

    <!-- Stackable -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={isStackable} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.attendance_status_unknown() === "غير محدد" ? "قابل للدمج / التداخل" : "Stackable"}</span>
    </label>

    <!-- Requires Approval -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={requiresApproval} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.attendance_status_unknown() === "غير محدد" ? "يتطلب موافقة الإدارة" : "Requires Approval"}</span>
    </label>

    <!-- Is Active -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={isActive} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.dt_form_is_active ? m.dt_form_is_active() : "نشط"}</span>
    </label>
  </div>

  <!-- Form Actions -->
  <div class="flex items-center justify-end gap-2 pt-4 border-t border-border mt-auto">
    <Button 
      type="button" 
      variant="ghost" 
      onclick={onCancel}
      disabled={isSubmitting}
      class="text-xs font-bold px-4 cursor-pointer"
    >
      {m.dt_form_cancel ? m.dt_form_cancel() : "إلغاء"}
    </Button>
    <Button 
      type="submit" 
      disabled={isSubmitting}
      class="text-xs font-bold px-4 cursor-pointer"
    >
      {#if isSubmitting}
        {m.dt_form_saving ? m.dt_form_saving() : "جاري الحفظ..."}
      {:else}
        {m.dt_form_save ? m.dt_form_save() : "حفظ قالب الخصم"}
      {/if}
    </Button>
  </div>
</form>
