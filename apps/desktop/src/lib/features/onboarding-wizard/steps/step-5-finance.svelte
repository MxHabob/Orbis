<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { formatNumber, l } from "$lib/core/i18n";
import {
  api,
  type FeeTemplateDto,
  type DiscountTemplateDto,
  type AccountDto,
} from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import OnboardingPresetBar from "../components/onboarding-preset-bar.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import DeleteRecordButton from "../components/delete-record-button.svelte";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingFee = $state(false);
let isSubmittingDiscount = $state(false);
let successMessage = $state<string | null>(null);

// Existing Data
let accounts = $state<AccountDto[]>([]);
let feeTemplates = $state<FeeTemplateDto[]>([]);
let discountTemplates = $state<DiscountTemplateDto[]>([]);

// Fee Template Form
let feeName = $state("");
let feeNameAr = $state("");
let feeAmount = $state<number>(0);
let feeCurrency = $state("SAR");
let feeCategory = $state("tuition");
let feeFrequency = $state("yearly");
let discountName = $state("");
let discountNameAr = $state("");
let discountType = $state("percentage");
let discountValue = $state<number>(0);

const FEE_CATEGORIES = [
  { value: "tuition", label: m.onboarding_fee_category_tuition() },
  { value: "registration", label: m.onboarding_fee_category_registration() },
  { value: "transport", label: m.onboarding_fee_category_transport() },
  { value: "books", label: m.onboarding_fee_category_books() },
  { value: "uniform", label: m.onboarding_fee_category_uniform() },
  { value: "activity", label: m.onboarding_fee_category_activity() },
];

const FREQUENCIES = [
  { value: "yearly", label: m.onboarding_fee_frequency_yearly() },
  { value: "termly", label: m.onboarding_fee_frequency_termly() },
  { value: "monthly", label: m.onboarding_fee_frequency_monthly() },
  { value: "once", label: m.onboarding_fee_frequency_once() },
];

const PRESET_FEES = [
  {
    name: "Annual Tuition Fee",
    nameAr: "الرسوم الدراسية السنوية",
    amount: 10000,
    category: "tuition",
    frequency: "yearly",
  },
  {
    name: "Registration & Admission",
    nameAr: "رسوم القبول والتسجيل",
    amount: 1000,
    category: "registration",
    frequency: "once",
  },
  {
    name: "Bus Transportation",
    nameAr: "رسوم النقل المدرسي",
    amount: 3500,
    category: "transport",
    frequency: "yearly",
  },
];

const PRESET_DISCOUNTS = [
  {
    name: "Second Sibling (10%)",
    nameAr: "خصم الأخ الثاني (10%)",
    type: "percentage",
    value: 10,
  },
  {
    name: "Third Sibling (20%)",
    nameAr: "خصم الأخ الثالث (20%)",
    type: "percentage",
    value: 20,
  },
  {
    name: "Staff Child Discount (50%)",
    nameAr: "خصم أبناء العاملين (50%)",
    type: "percentage",
    value: 50,
  },
  {
    name: "Academic Merit Scholarship (25%)",
    nameAr: "منحة التفوق الدراسي (25%)",
    type: "percentage",
    value: 25,
  },
];

const stepTasks = $derived(
  getTasksForStep("finance", systemStore.onboardingTasks),
);
const isTaskComplete = $derived(
  stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"),
);
const hasFeeAccounts = $derived(
  accounts.some(
    (account) =>
      account.account_type === "asset" && account.code.startsWith("1103"),
  ) &&
    accounts.some(
      (account) =>
        account.account_type === "revenue" && account.code.startsWith("410"),
    ),
);

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [accRes, feeRes, discRes, org] = await Promise.all([
      api.listAccounts(),
      api.listFeeTemplates(),
      api.listDiscountTemplates(),
      api.getCurrentOrg(),
    ]);
    accounts = accRes;
    feeTemplates = feeRes;
    discountTemplates = discRes;
    if (org?.base_currency_code) {
      feeCurrency = org.base_currency_code;
    }
  } catch (err: any) {
    console.error("Failed to load finance data:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isLoading = false;
  }
}

async function handleCreateFeeTemplate() {
  if (!feeName.trim() && !feeNameAr.trim()) {
    toast.error(m.onboarding_step5_fee_err_name());
    return;
  }

  isSubmittingFee = true;
  successMessage = null;

  try {
    const created = await api.createFeeTemplate({
      name: feeName.trim() || feeNameAr.trim(),
      name_ar: feeNameAr.trim() || undefined,
      default_amount: String(Number(feeAmount)),
      currency: feeCurrency,
      category: feeCategory,
      frequency:
        feeFrequency === "yearly"
          ? "annual"
          : feeFrequency === "termly"
            ? "termly"
            : feeFrequency,
      is_mandatory: true,
    });

    feeTemplates = [...feeTemplates, created];
    feeName = "";
    feeNameAr = "";
    feeAmount = 0;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step5_fee_created_success();
  } catch (err: any) {
    console.error("Failed to create fee template:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingFee = false;
  }
}

function applyFeePreset(preset: (typeof PRESET_FEES)[number]) {
  feeName = preset.name;
  feeNameAr = preset.nameAr;
  feeAmount = preset.amount;
  feeCategory = preset.category;
  feeFrequency = preset.frequency;
}

function applyDiscountPreset(preset: (typeof PRESET_DISCOUNTS)[number]) {
  discountName = preset.name;
  discountNameAr = preset.nameAr;
  discountType = preset.type;
  discountValue = preset.value;
}

async function handleCreateDiscount() {
  if (!discountName.trim() && !discountNameAr.trim()) {
    toast.error(m.onboarding_step5_fee_err_name());
    return;
  }
  if (discountValue <= 0) {
    toast.error(m.finance_ui_disbursement_amount_invalid());
    return;
  }
  if (discountType === "percentage" && discountValue > 100) {
    toast.error(m.ui_copy_settings_contain_invalid_values_fae2c24c());
    return;
  }

  isSubmittingDiscount = true;
  successMessage = null;

  try {
    const created = await api.createDiscountTemplate({
      name: discountName.trim() || discountNameAr.trim(),
      name_ar: discountNameAr.trim() || undefined,
      discount_type: discountType,
      value: discountValue,
      is_active: true,
    });

    discountTemplates = [...discountTemplates, created];
    discountName = "";
    discountNameAr = "";
    discountValue = 0;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step5_discount_created_success();
  } catch (err: any) {
    console.error("Failed to add preset discount:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingDiscount = false;
  }
}

async function deleteFeeTemplate(template: FeeTemplateDto) {
  const deleted = await api.deleteFeeTemplate(String(template.id));
  if (!deleted) throw new Error(m.common_action_failed());
  feeTemplates = feeTemplates.filter((item) => item.id !== template.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
}

async function deleteDiscountTemplate(template: DiscountTemplateDto) {
  const deleted = await api.deleteDiscountTemplate(String(template.id));
  if (!deleted) throw new Error(m.common_action_failed());
  discountTemplates = discountTemplates.filter(
    (item) => item.id !== template.id,
  );
  await refreshOnboardingState(orgId, m.common_action_failed());
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={6}
    title={m.onboarding_step5_title()}
    description={m.onboarding_step5_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : feeTemplates.length > 0 ? "in_progress" : "not_started"}
  />


  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.common_control_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Chart of Accounts Status -->
    <OnboardingSection
      title={m.onboarding_step5_coa_title()}
      description={m.onboarding_step5_coa_desc()}
      badge={accounts.length > 0 ? `${accounts.length}` : undefined}
      isComplete={hasFeeAccounts}
    >
      {#if hasFeeAccounts}
        <div class="rounded-lg border border-emerald-500/20 bg-emerald-500/5 p-4 flex items-center justify-between">
          <div class="flex items-center gap-3">
            <span class="flex size-8 items-center justify-center rounded-lg bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 font-bold">
              ✓
            </span>
            <div>
              <div class="text-xs font-semibold text-foreground">
                {m.onboarding_step5_coa_active_title()}
              </div>
              <div class="text-[11px] text-muted-foreground">
                {m.onboarding_step5_coa_active_desc()}
              </div>
            </div>
          </div>
          <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-xs">
            {m.onboarding_step5_coa_configured_badge()}
          </Badge>
        </div>
      {:else}
        <div class="rounded-lg border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-700 dark:text-amber-300">
          {m.onboarding_step5_coa_missing()}
        </div>
      {/if}
    </OnboardingSection>

    <!-- Section 2: Fee Templates -->
    <OnboardingSection
      title={m.onboarding_step5_fees_title()}
      description={m.onboarding_step5_fees_desc()}
      badge={feeTemplates.length > 0 ? `${feeTemplates.length}` : undefined}
      isComplete={feeTemplates.length > 0}
    >
      <div class="space-y-4">
        <OnboardingPresetBar
          title={m.onboarding_step5_quick_fees()}
        >
          {#each PRESET_FEES as preset (preset.name)}
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingFee}
              onclick={() => applyFeePreset(preset)}
              class="text-xs"
            >
              {l(preset.nameAr, preset.name)} · {formatNumber(preset.amount)} {feeCurrency}
            </Button>
          {/each}
        </OnboardingPresetBar>

        <!-- Fee Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-fee-name-ar"
                enId="onboarding-fee-name-en"
                arLabel={m.onboarding_step5_fee_name_ar()}
                enLabel={m.onboarding_step5_fee_name_en()}
                bind:arValue={feeNameAr}
                bind:enValue={feeName}
                arPlaceholder={m.onboarding_step5_fee_name_ar_placeholder()}
                enPlaceholder={m.onboarding_step5_fee_name_en_placeholder()}
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmittingFee}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_amount()}
              </Label>
              <Input
                type="number"
                bind:value={feeAmount}
                placeholder="0"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_category()}
              </Label>
              <Select type="single" bind:value={feeCategory}>
                <SelectTrigger class="w-full">
                  {FEE_CATEGORIES.find((c) => c.value === feeCategory)?.label || feeCategory}
                </SelectTrigger>
                <SelectContent>
                  {#each FEE_CATEGORIES as c (c.value)}
                    <SelectItem value={c.value}>{c.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5 sm:col-span-2">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_frequency()}
              </Label>
              <Select type="single" bind:value={feeFrequency}>
                <SelectTrigger class="w-full">
                  {FREQUENCIES.find((f) => f.value === feeFrequency)?.label || feeFrequency}
                </SelectTrigger>
                <SelectContent>
                  {#each FREQUENCIES as f (f.value)}
                    <SelectItem value={f.value}>{f.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingFee || (!feeName.trim() && !feeNameAr.trim()) || feeAmount <= 0}
              onclick={handleCreateFeeTemplate}
              class="text-xs font-semibold"
            >
              {isSubmittingFee ? m.common_control_loading() : m.onboarding_step5_add_fee_btn()}
            </Button>
          </div>
        </div>

        <!-- Existing Fees List -->
        {#if feeTemplates.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step5_configured_fees()}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each feeTemplates as ft (ft.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {ft.name_ar || ft.name}
                    </div>
                    <div class="text-[10px] text-muted-foreground capitalize">
                      {ft.category || "tuition"} | {ft.frequency || "yearly"}
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge variant="secondary" class="font-mono text-xs font-semibold">
                      {formatNumber(Number(ft.default_amount))} {ft.currency || feeCurrency}
                    </Badge>
                    <DeleteRecordButton label={ft.name_ar || ft.name} onDelete={() => deleteFeeTemplate(ft)} />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Section 3: Discounts & Scholarships -->
    <OnboardingSection
      title={m.onboarding_step5_discounts_title()}
      description={m.onboarding_step5_discounts_desc()}
      badge={discountTemplates.length > 0 ? `${discountTemplates.length}` : undefined}
      isComplete={discountTemplates.length > 0}
    >
      <div class="space-y-4">
        <OnboardingPresetBar
          title={m.onboarding_step5_quick_discounts()}
        >
          {#each PRESET_DISCOUNTS as preset (preset.name)}
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingDiscount}
              onclick={() => applyDiscountPreset(preset)}
              class="text-xs"
            >
              {l(preset.nameAr, preset.name)}
            </Button>
          {/each}
        </OnboardingPresetBar>

        <div class="space-y-4">
          <BilingualNameFields
            arId="onboarding-discount-name-ar"
            enId="onboarding-discount-name-en"
            arLabel={m.common_ledger_name_ar()}
            enLabel={m.common_ledger_name_en()}
            bind:arValue={discountNameAr}
            bind:enValue={discountName}
            arPlaceholder={m.ui_copy_discount_template_9e6c5879()}
            enPlaceholder={m.ui_copy_discount_template_9e6c5879()}
            kind="label"
            requestSuggestion={suggestLocalTranslation}
            replaceUneditedInitialValues
            disabled={isSubmittingDiscount}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">{m.ui_copy_discount_policy_type_1b319712()}</Label>
              <Select type="single" bind:value={discountType}>
                <SelectTrigger class="w-full">
                  {discountType === "percentage"
                    ? m.settings_ui_discount_type_percentage()
                    : m.settings_ui_discount_type_fixed()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="percentage">{m.settings_ui_discount_type_percentage()}</SelectItem>
                  <SelectItem value="fixed_amount">{m.settings_ui_discount_type_fixed()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {discountType === "percentage" ? m.common_control_col_percentage() : m.finance_ui_discount_amount_optional()}
              </Label>
              <Input
                type="number"
                min="0"
                max={discountType === "percentage" ? 100 : undefined}
                bind:value={discountValue}
                placeholder="0"
              />
            </div>
          </div>
          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingDiscount || (!discountName.trim() && !discountNameAr.trim()) || discountValue <= 0}
              onclick={handleCreateDiscount}
              class="text-xs font-semibold"
            >
              {isSubmittingDiscount ? m.common_control_loading() : m.common_action_add()}
            </Button>
          </div>
        </div>

        <!-- Existing Discounts List -->
        {#if discountTemplates.length > 0}
          <div class="grid gap-2 sm:grid-cols-2">
            {#each discountTemplates as dt (dt.id)}
              <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                <div>
                  <div class="font-medium text-foreground">
                    {dt.name_ar || dt.name}
                  </div>
                  <div class="text-[10px] text-muted-foreground capitalize">
                    {dt.discount_type || "percentage"}
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400 font-mono text-xs">
                    {dt.value}{dt.discount_type === 'percentage' ? '%' : ` ${feeCurrency}`}
                  </Badge>
                  <DeleteRecordButton label={dt.name_ar || dt.name} onDelete={() => deleteDiscountTemplate(dt)} />
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
