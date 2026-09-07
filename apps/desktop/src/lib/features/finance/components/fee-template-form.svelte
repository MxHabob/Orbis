<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, OrbisError } from "$lib/api";
import { feeTemplateInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui";
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
import { toast } from "@orbis/ui";

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
const form = new FormController({
  initialValues: {
    name: "",
    name_ar: null,
    code: null,
    category: "tuition",
    default_amount: 0,
    revenue_account_id: null,
    receivable_account_id: null,
    applicable_levels: null,
    frequency: "yearly",
    is_mandatory: true,
  },
  schema: feeTemplateInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

// Grade levels applicability state
let classes = $state<any[]>([]);
let isLoadingClasses = $state(false);
let uniqueLevels = $state<{ level: string; label: string }[]>([]);
let applyToAllGrades = $state(true);
let selectedLevels = $state<string[]>([]);

// Derived categories and frequencies for Orbis Select components
const categories = $derived([
  {
    value: "tuition",
    label: m.common_ft_category_tuition
      ? m.common_ft_category_tuition()
      : m.common_ft_category_tuition(),
  },
  {
    value: "transport",
    label: m.common_ft_category_transport
      ? m.common_ft_category_transport()
      : m.common_ft_category_transport(),
  },
  {
    value: "books",
    label: m.common_ft_category_books(),
  },
  {
    value: "uniform",
    label: m.common_ft_category_uniform
      ? m.common_ft_category_uniform()
      : m.common_ft_category_uniform(),
  },
  {
    value: "exam",
    label: m.common_ft_category_exam
      ? m.common_ft_category_exam()
      : m.common_ft_category_exam(),
  },
  {
    value: "other",
    label: m.common_guardians_relationship_other
      ? m.common_guardians_relationship_other()
      : m.common_guardians_relationship_other(),
  },
]);
const categoryLabels = $derived(
  categories.reduce(
    (acc, c) => ({ ...acc, [c.value]: c.label }),
    {} as Record<string, string>,
  ),
);
const selectedCategoryLabel = $derived(categoryLabels[form.values.category] ?? form.values.category);

const frequencies = $derived([
  {
    value: "once",
    label: m.common_ft_frequency_once
      ? m.common_ft_frequency_once()
      : m.common_ft_frequency_once(),
  },
  {
    value: "monthly",
    label: m.common_ft_frequency_monthly
      ? m.common_ft_frequency_monthly()
      : m.common_ft_frequency_monthly(),
  },
  {
    value: "termly",
    label: m.common_ft_frequency_termly
      ? m.common_ft_frequency_termly()
      : m.common_ft_frequency_termly(),
  },
  {
    value: "yearly",
    label: m.common_ft_frequency_yearly
      ? m.common_ft_frequency_yearly()
      : m.common_ft_frequency_yearly(),
  },
]);
const frequencyLabels = $derived(
  frequencies.reduce(
    (acc, f) => ({ ...acc, [f.value]: f.label }),
    {} as Record<string, string>,
  ),
);
const selectedFrequencyLabel = $derived(
  frequencyLabels[form.values.frequency] ?? form.values.frequency,
);

// Initialize form if in edit mode
$effect(() => {
  if (template) {
    form.values.name = template.name || "";
    form.values.name_ar = template.name_ar || null;
    form.values.code = template.code || null;
    form.values.category = template.category || "tuition";
    form.values.frequency = template.frequency || "yearly";
    form.values.is_mandatory = template.is_mandatory === 1;

    // Load amount directly from default_amount
    form.values.default_amount = template.default_amount || 0;

    // Load applicable levels
    if (template.applicable_levels) {
      try {
        const rules = JSON.parse(template.applicable_levels);
        const levels = rules.levels || (rules.level ? [rules.level] : []);
        if (levels.length > 0) {
          selectedLevels = levels;
          applyToAllGrades = false;
        } else {
          selectedLevels = [];
          applyToAllGrades = true;
        }
      } catch (e) {
        selectedLevels = [];
        applyToAllGrades = true;
      }
    } else {
      selectedLevels = [];
      applyToAllGrades = true;
    }
  }
});

async function loadClasses() {
  if (!orgId) return;
  isLoadingClasses = true;
  try {
    const list = await api.listClasses();
    classes = list;

    const levelNames: Record<string, { en: string; ar: string }> = {
      kg1: { en: "KG 1", ar: "روضة أولى" },
      kg2: { en: "KG 2", ar: "روضة ثانية" },
      grade1: { en: "Grade 1", ar: "الصف الأول" },
      grade2: { en: "Grade 2", ar: "الصف الثاني" },
      grade3: { en: "Grade 3", ar: "الصف الثالث" },
      grade4: { en: "Grade 4", ar: "الصف الرابع" },
      grade5: { en: "Grade 5", ar: "الصف الخامس" },
      grade6: { en: "Grade 6", ar: "الصف السادس" },
      grade7: { en: "Grade 7", ar: "الصف السابع" },
      grade8: { en: "Grade 8", ar: "الصف الثامن" },
      grade9: { en: "Grade 9", ar: "الصف التاسع" },
    };

    const levelsSet = new Set<string>();
    for (const cls of list) {
      if (cls.level) {
        levelsSet.add(cls.level);
      }
    }

    uniqueLevels = Array.from(levelsSet).map((lvl) => {
      const nameObj = levelNames[lvl] || { en: lvl, ar: lvl };
      return {
        level: lvl,
        label: l(nameObj.ar, nameObj.en),
      };
    });
  } catch (e) {
    console.error("Error loading classes for fee template form:", e);
  } finally {
    isLoadingClasses = false;
  }
}

$effect(() => {
  if (orgId) {
    loadClasses();
  }
});

async function handleSubmit() {
  const applicableLevels =
    !applyToAllGrades && selectedLevels.length > 0
      ? JSON.stringify({ levels: selectedLevels })
      : null;
  if (!applyToAllGrades && selectedLevels.length === 0) {
    toast.error(
      m.common_ft_error_levels_required(),
    );
    return;
  }
  const submitted = await form.submit(async (payload) => {
    const request = { ...payload, applicable_levels: applicableLevels };
    if (template) {
      await api.finance.updateFeeTemplate(template.id, request);
      toast.success(
        m.common_ft_update_success(),
      );
    } else {
      await api.createFeeTemplate(request);
      toast.success(
        m.common_ft_create_success(),
      );
    }
  });
  if (submitted) onSuccess();
  else if (form.formError) toast.error(form.formError);
  else if (Object.keys(form.errors).length > 0) toast.error(m.common_ft_error_name_required());
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
        : m.common_ft_sheet_create_title()}
    </Sheet.Title>
  </Sheet.Header>

  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <Accordion type="multiple" value={["details", "applicability"]} class="flex flex-col gap-4">
      <AccordionItem value="details" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_ft_sheet_create_title()}</AccordionTrigger>
        <AccordionContent>
    <!-- Name (Arabic) -->
    <FormField id="ft_form_name_ar" label={m.common_ft_col_name()} error={form.errors.name_ar?.[0]}>
      {#snippet children()}
      <Input 
        id="ft_form_name_ar"
        type="text" 
        bind:value={form.values.name_ar}
        onblur={() => form.markTouched("name_ar")}
        placeholder={m.common_ft_name_ar_placeholder()}
        disabled={form.isSubmitting}
      />
      {/snippet}
    </FormField>

    <!-- Name (English) -->
    <FormField id="ft_form_name_en" label={m.common_user_form_name_en()} error={form.errors.name?.[0]} required>
      {#snippet children()}
      <Input 
        id="ft_form_name_en"
        type="text" 
        bind:value={form.values.name}
        onblur={() => form.markTouched("name")}
        placeholder={m.common_ft_name_en_placeholder()}
        required
        disabled={form.isSubmitting}
      />
      {/snippet}
    </FormField>

    <!-- Code/Symbol -->
    <FormField id="ft_form_code" label={m.common_ledger_col_code()} error={form.errors.code?.[0]}>
      {#snippet children()}
      <Input 
        id="ft_form_code"
        type="text" 
        bind:value={form.values.code}
        placeholder={m.common_ft_code_placeholder()}
        disabled={form.isSubmitting}
      />
      {/snippet}
    </FormField>

    <!-- Category & Frequency Grid -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Category -->
      <FormField label={m.common_ft_col_category()} error={form.errors.category?.[0]} required>
        {#snippet children()}
        <Select type="single" value={form.values.category} onValueChange={(v) => (form.values.category = v ?? "tuition")} disabled={form.isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedCategoryLabel}
          </SelectTrigger>
          <SelectContent>
            {#each categories as c (c.value)}
              <SelectItem value={c.value}>{c.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      <!-- Frequency -->
      <FormField label={m.common_ft_col_frequency()} error={form.errors.frequency?.[0]} required>
        {#snippet children()}
        <Select type="single" value={form.values.frequency} onValueChange={(v) => (form.values.frequency = v ?? "yearly")} disabled={form.isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedFrequencyLabel}
          </SelectTrigger>
          <SelectContent>
            {#each frequencies as f (f.value)}
              <SelectItem value={f.value}>{f.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>
    </div>

    <!-- Amount -->
    <FormField id="ft_form_amount" label={m.common_ft_col_amount()} error={form.errors.default_amount?.[0]} required>
      {#snippet children()}
      <Input 
        id="ft_form_amount"
        type="number" 
        min="0"
        step="0.01"
        bind:value={form.values.default_amount}
        onblur={() => form.markTouched("default_amount")}
        placeholder={m.common_ft_amount_placeholder()}
        required
        disabled={form.isSubmitting}
      />
      {/snippet}
    </FormField>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="applicability" class="rounded-lg">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_ft_applicability()}</AccordionTrigger>
        <AccordionContent>

    <!-- Grade Level Applicability -->
    <div class="flex flex-col gap-3 p-4 bg-muted/20 rounded-lg border border-border/30">
      <span class="text-xs font-bold text-foreground text-start">
        {m.common_ft_target_levels()}
      </span>
      
      <div class="flex items-center gap-2">
        <Checkbox 
          id="ft_form_apply_all" 
          checked={applyToAllGrades} 
          onCheckedChange={(val) => applyToAllGrades = !!val} 
          disabled={form.isSubmitting}
        />
        <Label for="ft_form_apply_all" class="text-xs cursor-pointer">
          {m.common_ft_apply_all()}
        </Label>
      </div>

      {#if !applyToAllGrades}
        {#if isLoadingClasses}
          <span class="text-xs text-muted-foreground text-start">{m.common_loading()}</span>
        {:else}
          <div class="grid grid-cols-2 gap-2 mt-1">
            {#each uniqueLevels as lvl}
              <div class="flex items-center gap-2">
                <Checkbox 
                  id={`ft_level_${lvl.level}`} 
                  checked={selectedLevels.includes(lvl.level)} 
                  onCheckedChange={(val) => {
                    if (val) {
                      selectedLevels = [...selectedLevels, lvl.level];
                    } else {
                      selectedLevels = selectedLevels.filter(x => x !== lvl.level);
                    }
                  }}
                  disabled={isSubmitting}
                />
                <Label for={`ft_level_${lvl.level}`} class="text-xs cursor-pointer text-start">
                  {lvl.label}
                </Label>
              </div>
            {/each}
            {#if uniqueLevels.length === 0}
              <span class="text-xs text-muted-foreground col-span-2 text-start">
                {m.common_ft_no_levels()}
              </span>
            {/if}
          </div>
        {/if}
      {/if}
    </div>

    <!-- Checkboxes -->
    <div class="flex gap-6 mt-2">
      <!-- Mandatory Switch -->
      <div class="flex items-center gap-2">
        <Checkbox 
          id="ft_mandatory_chk" 
                  checked={form.values.is_mandatory} 
                  onCheckedChange={(val) => form.values.is_mandatory = !!val} 
                  disabled={form.isSubmitting}
        />
        <Label for="ft_mandatory_chk" class="text-xs cursor-pointer">
          {m.common_ft_mandatory()}
        </Label>
      </div>

      <!-- Active Switch -->
      <div class="flex items-center gap-2">
        <Checkbox 
          id="ft_active_chk" 
                  checked={template ? template.is_active === 1 : true} 
                  disabled={form.isSubmitting}
        />
        <Label for="ft_active_chk" class="text-xs cursor-pointer">
          {m.common_phase_active()}
        </Label>
      </div>
    </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>

  <!-- Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 shrink-0 bg-background/50">
    <Button 
      type="button" 
        variant="ghost" 
      onclick={onCancel}
      disabled={form.isSubmitting}
      class="flex-1 h-9 text-sm"
    >
      {m.common_action_cancel()}
    </Button>
    <Button 
      type="submit" 
      disabled={form.isSubmitting}
      class="flex-1 h-9 text-sm font-semibold"
    >
      {#if form.isSubmitting}
        {m.common_state_saving()}
      {:else}
        {m.common_action_save()}
      {/if}
    </Button>
  </div>
</form>
