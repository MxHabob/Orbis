<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type FeeTemplateDto, type DiscountTemplateDto, type AccountDto } from "$lib/api";
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
import { getTasksForStep } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingFee = $state(false);
let isSubmittingDiscount = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Existing Data
let accounts = $state<AccountDto[]>([]);
let feeTemplates = $state<FeeTemplateDto[]>([]);
let discountTemplates = $state<DiscountTemplateDto[]>([]);

// Fee Template Form
let feeName = $state("Annual Tuition Fee");
let feeNameAr = $state("الرسوم الدراسية السنوية");
let feeAmount = $state<number>(15000);
let feeCurrency = $state("SAR");
let feeCategory = $state("tuition");
let feeFrequency = $state("yearly");

const FEE_CATEGORIES = [
  { value: "tuition", label: "Tuition Fee (رسوم دراسية)" },
  { value: "registration", label: "Registration / Admission (رسوم تسجيل وقبول)" },
  { value: "transport", label: "Transportation / Bus (رسوم النقل المدرسي)" },
  { value: "books", label: "Books & Study Materials (الكتب والمقررات)" },
  { value: "uniform", label: "School Uniform (الزي المدرسي)" },
  { value: "activity", label: "Extracurricular Activities (أنشطة لا منهجية)" },
];

const FREQUENCIES = [
  { value: "yearly", label: "Annual (سنوي)" },
  { value: "termly", label: "Per Term / Semester (فصلي)" },
  { value: "monthly", label: "Monthly (شهري)" },
  { value: "once", label: "One-Time (مرة واحدة)" },
];

const PRESET_FEES = [
  { name: "Annual Tuition Fee", nameAr: "الرسوم الدراسية السنوية", amount: 15000, category: "tuition", frequency: "yearly" },
  { name: "Registration & Admission", nameAr: "رسوم القبول والتسجيل", amount: 1000, category: "registration", frequency: "once" },
  { name: "Bus Transportation", nameAr: "رسوم النقل المدرسي", amount: 3500, category: "transport", frequency: "yearly" },
];

const PRESET_DISCOUNTS = [
  { name: "Second Sibling (10%)", nameAr: "خصم الأخ الثاني (10%)", type: "percentage", value: 10 },
  { name: "Third Sibling (20%)", nameAr: "خصم الأخ الثالث (20%)", type: "percentage", value: 20 },
  { name: "Staff Child Discount (50%)", nameAr: "خصم أبناء العاملين (50%)", type: "percentage", value: 50 },
  { name: "Academic Merit Scholarship (25%)", nameAr: "منحة التفوق الدراسي (25%)", type: "percentage", value: 25 },
];

const stepTasks = $derived(getTasksForStep("finance", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [accRes, feeRes, discRes, org] = await Promise.all([
      api.listAccounts().catch(() => []),
      api.listFeeTemplates().catch(() => []),
      api.listDiscountTemplates().catch(() => []),
      api.getCurrentOrg().catch(() => null),
    ]);
    accounts = accRes;
    feeTemplates = feeRes;
    discountTemplates = discRes;
    if (org?.base_currency_code) {
      feeCurrency = org.base_currency_code;
    }
  } catch (err: any) {
    console.error("Failed to load finance data:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isLoading = false;
  }
}

async function handleCreateFeeTemplate() {
  if (!feeName.trim() && !feeNameAr.trim()) {
    errorMessage = m.onboarding_step5_fee_err_name();
    return;
  }

  isSubmittingFee = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createFeeTemplate({
      name: feeName.trim() || feeNameAr.trim(),
      name_ar: feeNameAr.trim() || undefined,
      amount: Number(feeAmount) || 0,
      currency: feeCurrency,
      category: feeCategory,
      frequency: feeFrequency,
      is_active: true,
    });

    feeTemplates = [...feeTemplates, created];
    feeName = "";
    feeNameAr = "";
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step5_fee_created_success();
  } catch (err: any) {
    console.error("Failed to create fee template:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingFee = false;
  }
}

async function handleAddPresetFee(preset: typeof PRESET_FEES[0]) {
  isSubmittingFee = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createFeeTemplate({
      name: preset.name,
      name_ar: preset.nameAr,
      amount: preset.amount,
      currency: feeCurrency,
      category: preset.category,
      frequency: preset.frequency,
      is_active: true,
    });

    feeTemplates = [...feeTemplates, created];
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step5_fee_created_success();
  } catch (err: any) {
    console.error("Failed to add preset fee:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingFee = false;
  }
}

async function handleAddPresetDiscount(preset: typeof PRESET_DISCOUNTS[0]) {
  isSubmittingDiscount = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createDiscountTemplate({
      name: preset.name,
      name_ar: preset.nameAr,
      discount_type: preset.type,
      value: preset.value,
      is_active: true,
    });

    discountTemplates = [...discountTemplates, created];
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step5_discount_created_success();
  } catch (err: any) {
    console.error("Failed to add preset discount:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingDiscount = false;
  }
}

async function handleContinue() {
  if (feeTemplates.length === 0 && feeName.trim()) {
    await handleCreateFeeTemplate();
  }
  await navigateOnboardingRoute(`/${orgId}/onboarding/assessment`);
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

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

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
      isComplete={true}
    >
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
    </OnboardingSection>

    <!-- Section 2: Fee Templates -->
    <OnboardingSection
      title={m.onboarding_step5_fees_title()}
      description={m.onboarding_step5_fees_desc()}
      badge={feeTemplates.length > 0 ? `${feeTemplates.length}` : undefined}
      isComplete={feeTemplates.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Preset Buttons -->
        {#if feeTemplates.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="text-xs font-semibold text-foreground">
              💳 {m.onboarding_step5_quick_fees()}
            </div>
            <div class="flex flex-wrap gap-2">
              {#each PRESET_FEES as preset}
                <Button
                  variant="outline"
                  size="sm"
                  disabled={isSubmittingFee}
                  onclick={() => handleAddPresetFee(preset)}
                  class="text-xs bg-background hover:bg-muted"
                >
                  + {preset.name_ar || preset.name} ({preset.amount.toLocaleString()} {feeCurrency})
                </Button>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Fee Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_name_en()}
              </Label>
              <Input
                bind:value={feeName}
                placeholder={m.onboarding_step5_fee_name_en_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_name_ar()}
              </Label>
              <Input
                bind:value={feeNameAr}
                placeholder={m.onboarding_step5_fee_name_ar_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step5_fee_amount()}
              </Label>
              <Input
                type="number"
                bind:value={feeAmount}
                placeholder="15000"
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
                  {#each FEE_CATEGORIES as c}
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
                  {#each FREQUENCIES as f}
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
              disabled={isSubmittingFee || (!feeName.trim() && !feeNameAr.trim())}
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
                  <Badge variant="secondary" class="font-mono text-xs font-semibold">
                    {Number(ft.amount).toLocaleString()} {ft.currency || feeCurrency}
                  </Badge>
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
        <!-- Quick Preset Discounts -->
        {#if discountTemplates.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="text-xs font-semibold text-foreground">
              🎁 {m.onboarding_step5_quick_discounts()}
            </div>
            <div class="flex flex-wrap gap-2">
              {#each PRESET_DISCOUNTS as preset}
                <Button
                  variant="outline"
                  size="sm"
                  disabled={isSubmittingDiscount}
                  onclick={() => handleAddPresetDiscount(preset)}
                  class="text-xs bg-background hover:bg-muted"
                >
                  + {preset.name_ar || preset.name}
                </Button>
              {/each}
            </div>
          </div>
        {/if}

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
                <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400 font-mono text-xs">
                  {dt.value}{dt.discount_type === 'percentage' ? '%' : ` ${feeCurrency}`}
                </Badge>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Inline Actions Bar -->
    <div class="mt-8 flex items-center justify-end gap-3 pb-8 pt-4 border-t border-border/40">
      <Button
        variant="default"
        disabled={feeTemplates.length === 0 && !feeName.trim()}
        onclick={handleContinue}
        class="text-xs font-semibold shadow-xs"
      >
        {m.onboarding_step5_continue_btn()}
      </Button>
    </div>
  {/if}
</div>
