<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { commands } from "$lib/core/ipc";
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
let nameAr = $state("");
let code = $state("");
let category = $state("tuition");
let amount = $state<number>(0);
let frequency = $state("yearly");
let isMandatory = $state(true);
let isActive = $state(true);

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
const selectedCategoryLabel = $derived(categoryLabels[category] ?? category);

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
  frequencyLabels[frequency] ?? frequency,
);

// Initialize form if in edit mode
$effect(() => {
  if (template) {
    name = template.name || "";
    nameAr = template.name_ar || "";
    code = template.code || "";
    category = template.category || "tuition";
    frequency = template.frequency || "yearly";
    isMandatory = template.is_mandatory === 1;
    isActive = template.is_active === 1;

    // Load amount directly from default_amount
    amount = template.default_amount || 0;

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
    const list = await commands.academic.listClasses();
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
  if (!name.trim()) {
    toast.error(m.common_ft_error_name_required());
    return;
  }
  if (!applyToAllGrades && selectedLevels.length === 0) {
    toast.error(
      m.common_ft_error_levels_required(),
    );
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name.trim(),
      name_ar: nameAr.trim() || null,
      code: code.trim() || null,
      category,
      default_amount: Number(amount || 0),
      revenue_account_id: "", // Managed automatically by backend
      receivable_account_id: "", // Managed automatically by backend
      applicable_levels:
        !applyToAllGrades && selectedLevels.length > 0
          ? JSON.stringify({ levels: selectedLevels })
          : null,
      frequency,
      installment_config: null,
      late_fee_rules: null,
      is_mandatory: isMandatory ? 1 : 0,
      is_active: isActive ? 1 : 0,
    };

    if (template) {
      await commands.finance.updateFeeTemplate(template.id, request as any);
      toast.success(
        m.common_ft_update_success(),
      );
    } else {
      await commands.finance.createFeeTemplate(request as any);
      toast.success(
        m.common_ft_create_success(),
      );
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
      <AccordionItem value="details" class="rounded-lg border border-border/50 px-4">
        <AccordionTrigger class="text-sm font-semibold hover:no-underline">{m.common_ft_sheet_create_title()}</AccordionTrigger>
        <AccordionContent>
    <!-- Name (Arabic) -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="ft_form_name_ar" class="text-xs font-semibold text-muted-foreground">
        {m.common_ft_col_name()}
      </Label>
      <Input 
        id="ft_form_name_ar"
        type="text" 
        bind:value={nameAr}
        placeholder={m.common_ft_name_ar_placeholder()}
        disabled={isSubmitting}
      />
    </div>

    <!-- Name (English) -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="ft_form_name_en" class="text-xs font-semibold text-muted-foreground">
        {m.common_user_form_name_en()} *
      </Label>
      <Input 
        id="ft_form_name_en"
        type="text" 
        bind:value={name}
        placeholder={m.common_ft_name_en_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Code/Symbol -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="ft_form_code" class="text-xs font-semibold text-muted-foreground">
        {m.common_ft_form_code()}
      </Label>
      <Input 
        id="ft_form_code"
        type="text" 
        bind:value={code}
        placeholder={m.common_ft_code_placeholder()}
        disabled={isSubmitting}
      />
    </div>

    <!-- Category & Frequency Grid -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Category -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold text-muted-foreground">
          {m.common_ft_col_category()} *
        </Label>
        <Select type="single" value={category} onValueChange={(v) => (category = v ?? "tuition")} disabled={isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedCategoryLabel}
          </SelectTrigger>
          <SelectContent>
            {#each categories as c (c.value)}
              <SelectItem value={c.value}>{c.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Frequency -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold text-muted-foreground">
          {m.common_ft_col_frequency()} *
        </Label>
        <Select type="single" value={frequency} onValueChange={(v) => (frequency = v ?? "yearly")} disabled={isSubmitting}>
          <SelectTrigger class="h-9 text-sm">
            {selectedFrequencyLabel}
          </SelectTrigger>
          <SelectContent>
            {#each frequencies as f (f.value)}
              <SelectItem value={f.value}>{f.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Amount -->
    <div class="flex flex-col gap-1.5 text-start">
      <Label for="ft_form_amount" class="text-xs font-semibold text-muted-foreground">
        {m.common_ft_col_amount()} *
      </Label>
      <Input 
        id="ft_form_amount"
        type="number" 
        min="0"
        step="0.01"
        bind:value={amount}
        placeholder={m.common_ft_amount_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>

        </AccordionContent>
      </AccordionItem>

      <AccordionItem value="applicability" class="rounded-lg border border-border/50 px-4">
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
          disabled={isSubmitting}
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
          checked={isMandatory} 
          onCheckedChange={(val) => isMandatory = !!val} 
          disabled={isSubmitting}
        />
        <Label for="ft_mandatory_chk" class="text-xs cursor-pointer">
          {m.common_ft_mandatory()}
        </Label>
      </div>

      <!-- Active Switch -->
      <div class="flex items-center gap-2">
        <Checkbox 
          id="ft_active_chk" 
          checked={isActive} 
          onCheckedChange={(val) => isActive = !!val} 
          disabled={isSubmitting}
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
