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
let ledgers = $state<any[]>([]);
let isLoadingLedgers = $state(false);

let name = $state("");
let nameAr = $state("");
let code = $state("");
let category = $state("tuition");
let amount = $state<number>(0);
let frequency = $state("yearly");
let revenueAccountId = $state("");
let receivableAccountId = $state("");
let isMandatory = $state(true);
let isActive = $state(true);

// Initialize form if in edit mode
$effect(() => {
  if (template) {
    name = template.name || "";
    nameAr = template.name_ar || "";
    code = template.code || "";
    category = template.category || "tuition";
    frequency = template.frequency || "yearly";
    revenueAccountId = template.revenue_account_id || "";
    receivableAccountId = template.receivable_account_id || "";
    isMandatory = template.is_mandatory === 1;
    isActive = template.is_active === 1;
    
    // Parse pricing config for amount
    if (template.pricing_config) {
      try {
        const config = JSON.parse(template.pricing_config);
        amount = Number(config.amount || 0);
      } catch (e) {
        amount = 0;
      }
    } else {
      amount = 0;
    }
  }
});

async function loadLedgerAccounts() {
  if (!orgId) return;
  isLoadingLedgers = true;
  try {
    const list = await commands.finance.getLedgerAccounts(orgId);
    ledgers = list;
    
    // Auto-select revenue and receivable accounts if not set
    if (!revenueAccountId) {
      const rev = list.find((l: any) => String(l.item_type || "").toLowerCase() === "revenue");
      if (rev) revenueAccountId = rev.id;
    }
    if (!receivableAccountId) {
      const rec = list.find((l: any) => String(l.item_type || "").toLowerCase() === "asset" || String(l.item_type || "").toLowerCase() === "receivable");
      if (rec) receivableAccountId = rec.id;
    }
  } catch (e) {
    console.error("Error loading ledger accounts in fee template form:", e);
  } finally {
    isLoadingLedgers = false;
  }
}

$effect(() => {
  if (orgId) {
    loadLedgerAccounts();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!name.trim()) {
    toast.error("Name (English) is required");
    return;
  }
  if (!revenueAccountId) {
    toast.error("Revenue Account is required");
    return;
  }
  if (!receivableAccountId) {
    toast.error("Receivable Account is required");
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name.trim(),
      name_ar: nameAr.trim() || null,
      code: code.trim() || null,
      category,
      revenue_account_id: revenueAccountId,
      receivable_account_id: receivableAccountId,
      pricing_type: "fixed",
      pricing_config: JSON.stringify({ amount }),
      applicability_rules: null,
      frequency,
      frequency_config: null,
      late_fee_rules: null,
      allowed_discounts: null,
      is_mandatory: isMandatory ? 1 : 0,
      is_active: isActive ? 1 : 0,
    };

    if (template) {
      await commands.finance.updateFeeTemplate(orgId, template.id, request);
      toast.success(m.ft_update_success ? m.ft_update_success() : "Fee template updated successfully");
    } else {
      await commands.finance.createFeeTemplate(orgId, request);
      toast.success(m.ft_create_success ? m.ft_create_success() : "Fee template created successfully");
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
  <!-- Name (Arabic) -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="ft_form_name_ar" class="text-xs font-semibold text-foreground">
      {m.ft_form_name ? m.ft_form_name() : "اسم القالب بالعربية"}
    </label>
    <Input 
      id="ft_form_name_ar"
      type="text" 
      bind:value={nameAr}
      placeholder="مثال: رسوم تسجيل الصف الأول"
      disabled={isSubmitting}
    />
  </div>

  <!-- Name (English) -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="ft_form_name_en" class="text-xs font-semibold text-foreground">
      {m.ft_form_name_en ? m.ft_form_name_en() : "Name (English)"}
    </label>
    <Input 
      id="ft_form_name_en"
      type="text" 
      bind:value={name}
      placeholder="e.g., Grade 1 Enrollment Fees"
      required
      disabled={isSubmitting}
    />
  </div>

  <!-- Code/Symbol -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="ft_form_code" class="text-xs font-semibold text-foreground">
      {m.attendance_status_unknown() === "غير محدد" ? "رمز القالب / الكود" : "Template Code"}
    </label>
    <Input 
      id="ft_form_code"
      type="text" 
      bind:value={code}
      placeholder="e.g., FEES_GR1"
      disabled={isSubmitting}
    />
  </div>

  <!-- Category & Frequency Grid -->
  <div class="grid grid-cols-2 gap-4">
    <!-- Category -->
    <div class="flex flex-col gap-1.5 text-start">
      <label for="ft_form_category" class="text-xs font-semibold text-foreground">
        {m.ft_form_category ? m.ft_form_category() : "الفئة"}
      </label>
      <select 
        id="ft_form_category"
        bind:value={category}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isSubmitting}
      >
        <option value="tuition">{m.ft_category_tuition ? m.ft_category_tuition() : "رسوم دراسية"}</option>
        <option value="transport">{m.ft_category_transport ? m.ft_category_transport() : "نقل"}</option>
        <option value="books">{m.ft_category_books ? m.ft_category_books() : "كتب"}</option>
        <option value="uniform">{m.ft_category_uniform ? m.ft_category_uniform() : "زي مدرسي"}</option>
        <option value="exam">{m.ft_category_exam ? m.ft_category_exam() : "رسوم امتحانات"}</option>
        <option value="other">{m.ft_category_other ? m.ft_category_other() : "أخرى"}</option>
      </select>
    </div>

    <!-- Frequency -->
    <div class="flex flex-col gap-1.5 text-start">
      <label for="ft_form_frequency" class="text-xs font-semibold text-foreground">
        {m.ft_form_frequency ? m.ft_form_frequency() : "التكرار"}
      </label>
      <select 
        id="ft_form_frequency"
        bind:value={frequency}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isSubmitting}
      >
        <option value="once">{m.ft_frequency_once ? m.ft_frequency_once() : "مرة واحدة"}</option>
        <option value="monthly">{m.ft_frequency_monthly ? m.ft_frequency_monthly() : "شهري"}</option>
        <option value="termly">{m.ft_frequency_termly ? m.ft_frequency_termly() : "فصلي"}</option>
        <option value="yearly">{m.ft_frequency_yearly ? m.ft_frequency_yearly() : "سنوي"}</option>
      </select>
    </div>
  </div>

  <!-- Amount -->
  <div class="flex flex-col gap-1.5 text-start">
    <label for="ft_form_amount" class="text-xs font-semibold text-foreground">
      {m.ft_form_amount ? m.ft_form_amount() : "المبلغ"}
    </label>
    <Input 
      id="ft_form_amount"
      type="number" 
      min="0"
      step="0.01"
      bind:value={amount}
      placeholder="e.g., 150000"
      required
      disabled={isSubmitting}
    />
  </div>

  <!-- Accounts mapping -->
  <div class="flex flex-col gap-4 p-3 bg-muted/40 rounded-lg border border-border/60">
    <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider text-start">
      {m.attendance_status_unknown() === "غير محدد" ? "الربط المحاسبي والدفاتر" : "GL Account Mapping"}
    </span>
    
    <!-- Revenue Account Select -->
    <div class="flex flex-col gap-1.5 text-start">
      <label for="ft_form_rev_acc" class="text-[11px] font-semibold text-muted-foreground">
        {m.ft_form_revenue_account ? m.ft_form_revenue_account() : "حساب الإيرادات (الدائن)"}
      </label>
      <select 
        id="ft_form_rev_acc"
        bind:value={revenueAccountId}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isLoadingLedgers || isSubmitting}
      >
        {#if isLoadingLedgers}
          <option>{m.loading()}</option>
        {:else}
          {#each ledgers.filter(l => String(l.item_type || "").toLowerCase() === "revenue") as l (l.id)}
            <option value={String(l.id)}>{l.name_ar || l.name} ({l.code})</option>
          {/each}
          {#if ledgers.filter(l => String(l.item_type || "").toLowerCase() === "revenue").length === 0}
            <option value="">-- No Revenue Accounts found --</option>
            {#each ledgers as l (l.id)}
              <option value={String(l.id)}>{l.name_ar || l.name} ({l.code})</option>
            {/each}
          {/if}
        {/if}
      </select>
    </div>

    <!-- Receivable Account Select -->
    <div class="flex flex-col gap-1.5 text-start">
      <label for="ft_form_rec_acc" class="text-[11px] font-semibold text-muted-foreground">
        {m.ft_form_receivable_account ? m.ft_form_receivable_account() : "حساب المدينين (المدين)"}
      </label>
      <select 
        id="ft_form_rec_acc"
        bind:value={receivableAccountId}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full"
        disabled={isLoadingLedgers || isSubmitting}
      >
        {#if isLoadingLedgers}
          <option>{m.loading()}</option>
        {:else}
          {#each ledgers.filter(l => String(l.item_type || "").toLowerCase() === "asset" || String(l.item_type || "").toLowerCase() === "receivable") as l (l.id)}
            <option value={String(l.id)}>{l.name_ar || l.name} ({l.code})</option>
          {/each}
          {#if ledgers.filter(l => String(l.item_type || "").toLowerCase() === "asset" || String(l.item_type || "").toLowerCase() === "receivable").length === 0}
            <option value="">-- No Receivable Accounts found --</option>
            {#each ledgers as l (l.id)}
              <option value={String(l.id)}>{l.name_ar || l.name} ({l.code})</option>
            {/each}
          {/if}
        {/if}
      </select>
    </div>
  </div>

  <!-- Checkboxes -->
  <div class="flex gap-6 mt-2">
    <!-- Mandatory Switch -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={isMandatory} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.attendance_status_unknown() === "غير محدد" ? "إلزامي لجميع الطلاب" : "Mandatory for all students"}</span>
    </label>

    <!-- Active Switch -->
    <label class="flex items-center gap-2 cursor-pointer select-none text-xs font-semibold">
      <input 
        type="checkbox" 
        bind:checked={isActive} 
        disabled={isSubmitting}
        class="w-4 h-4 rounded text-primary focus:ring-primary border-border"
      />
      <span>{m.ft_form_is_active ? m.ft_form_is_active() : "نشط"}</span>
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
      {m.ft_form_cancel ? m.ft_form_cancel() : "إلغاء"}
    </Button>
    <Button 
      type="submit" 
      disabled={isSubmitting}
      class="text-xs font-bold px-4 cursor-pointer"
    >
      {#if isSubmitting}
        {m.ft_form_saving ? m.ft_form_saving() : "جاري الحفظ..."}
      {:else}
        {m.ft_form_save ? m.ft_form_save() : "حفظ القالب"}
      {/if}
    </Button>
  </div>
</form>
