<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, queryKeys } from "$lib/api";
import { useQueryClient } from "@tanstack/svelte-query";
import { feeTemplateInputSchema } from "@orbis/schemas";
import { createClassesQuery } from "$lib/features/academics/queries";
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

function parseInitialApplicableLevels(raw: any): { levels: string[]; applyToAll: boolean } {
  if (!raw) return { levels: [], applyToAll: true };
  try {
    let levels: string[] = [];
    if (typeof raw === "string") {
      const parsed = JSON.parse(raw);
      levels = Array.isArray(parsed) ? parsed : (parsed.levels || (parsed.level ? [parsed.level] : []));
    } else if (Array.isArray(raw)) {
      levels = raw;
    } else if (typeof raw === "object") {
      const obj = raw as any;
      levels = Array.isArray(obj.levels) ? obj.levels : (obj.level ? [obj.level] : []);
    }
    if (levels.length > 0) {
      return { levels, applyToAll: false };
    }
  } catch (e) {
    // Ignore parse error
  }
  return { levels: [], applyToAll: true };
}

const initialLevelsData = parseInitialApplicableLevels(template?.applicable_levels);
let applyToAllGrades = $state(initialLevelsData.applyToAll);
let selectedLevels = $state<string[]>(initialLevelsData.levels);

// Form state initialized directly from template or defaults
const form = new FormController({
  initialValues: {
    name: template?.name || "",
    name_ar: template?.name_ar || null,
    code: template?.code || null,
    category: template?.category || "tuition",
    default_amount: template?.default_amount || 0,
    revenue_account_id: template?.revenue_account_id || null,
    receivable_account_id: template?.receivable_account_id || null,
    applicable_levels: template?.applicable_levels || null,
    frequency: template?.frequency === "annual" ? "yearly" : (template?.frequency || "yearly"),
    is_mandatory: template ? (template.is_mandatory === 1 || template.is_mandatory === true) : true,
  },
  schema: feeTemplateInputSchema,
});

const queryClient = useQueryClient();

// TanStack Query for Classes
const classesQuery = createClassesQuery(() => orgId);
const classes = $derived(classesQuery.data ?? []);
const isLoadingClasses = $derived(classesQuery.isLoading);

// Dynamic grade levels extracted directly from the classes database table
const uniqueLevels = $derived.by(() => {
  const levelMap = new Map<string, { ar: string; en: string }>();
  for (const cls of classes) {
    if (cls.level) {
      if (!levelMap.has(cls.level)) {
        const rawAr = (cls as any).name_ar || cls.name || cls.level;
        const rawEn = cls.name || (cls as any).name_ar || cls.level;
        levelMap.set(cls.level, {
          ar: rawAr,
          en: rawEn,
        });
      }
    }
  }

  // Ensure any selected levels from existing template are visible
  for (const lvl of selectedLevels) {
    if (!levelMap.has(lvl)) {
      levelMap.set(lvl, {
        ar: lvl,
        en: lvl,
      });
    }
  }

  return Array.from(levelMap.entries()).map(([lvl, nameObj]) => ({
    level: lvl,
    label: l(nameObj.ar, nameObj.en),
  }));
});

// Derived categories and frequencies for Orbis Select components
const categories = $derived([
  {
    value: "tuition",
    label: m.common_ft_category_tuition(),
  },
  {
    value: "transport",
    label: m.common_ft_category_transport(),
  },
  {
    value: "books",
    label: m.common_ft_category_books(),
  },
  {
    value: "uniform",
    label: m.common_ft_category_uniform(),
  },
  {
    value: "exam",
    label: m.common_ft_category_exam(),
  },
  {
    value: "other",
    label: m.common_guardians_relationship_other(),
  },
]);
const categoryLabels = $derived(
  categories.reduce(
    (acc, c) => ({ ...acc, [c.value]: c.label }),
    {} as Record<string, string>,
  ),
);
const selectedCategoryLabel = $derived(
  categoryLabels[form.values.category] ?? form.values.category,
);

const frequencies = $derived([
  {
    value: "once",
    label: m.common_ft_frequency_once(),
  },
  {
    value: "monthly",
    label: m.common_ft_frequency_monthly(),
  },
  {
    value: "termly",
    label: m.common_ft_frequency_termly(),
  },
  {
    value: "yearly",
    label: m.common_ft_frequency_yearly(),
  },
  {
    value: "annual",
    label: m.common_ft_frequency_yearly(),
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

async function handleSubmit() {
  const applicableLevels =
    !applyToAllGrades && selectedLevels.length > 0
      ? JSON.stringify({ levels: selectedLevels })
      : null;
  if (!applyToAllGrades && selectedLevels.length === 0) {
    toast.error(m.common_ft_error_levels_required());
    return;
  }
  const submitted = await form.submit(async (payload) => {
    const request = { ...payload, applicable_levels: applicableLevels };
    if (template?.id) {
      await api.updateFeeTemplate(template.id, request);
      toast.success(m.common_ft_update_success());
    } else {
      await api.createFeeTemplate(request);
      toast.success(m.common_ft_create_success());
    }
    await queryClient.invalidateQueries({
      queryKey: queryKeys.finance.feeTemplates(orgId),
    });
  });
  if (submitted) onSuccess();
  else if (form.formError) toast.error(form.formError);
  else if (Object.keys(form.errors).length > 0)
    toast.error(m.common_ft_error_name_required());
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
                  disabled={form.isSubmitting}
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
