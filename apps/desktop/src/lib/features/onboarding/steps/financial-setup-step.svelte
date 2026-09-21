<!--
  Step 6: Tuition & Registration Fees (الرسوم الدراسية والتسجيل)
  Categorized by educational stage groups with clean stage fee setting and individual grade customization.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Check,
  ChevronDown,
  ChevronUp,
  SlidersHorizontal,
  RotateCcw,
} from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import type { GradeLevel } from "../types";
import { l } from "$lib/core/i18n";

const enabledGradeLevels = $derived(
  onboardingState.academics.gradeLevels.filter((gl) => gl.enabled),
);

let expandedStages = $state<Record<string, boolean>>({});
let globalBatchAmount = $state<number | "">("");
let showGlobalBatchBar = $state(false);

interface EducationalStageGroup {
  id: string;
  name_ar: string;
  name_en: string;
  desc_ar: string;
  desc_en: string;
  grades: GradeLevel[];
}

const stageGroups = $derived.by(() => {
  const groups: EducationalStageGroup[] = [];

  const kgGrades = enabledGradeLevels.filter((g) =>
    g.level.startsWith("kg") || g.level.includes("kindergarten")
  );
  const primaryGrades = enabledGradeLevels.filter((g) => {
    const num = parseInt(g.level.replace(/\D/g, ""), 10);
    return g.level.startsWith("grade") && num >= 1 && num <= 6;
  });
  const prepGrades = enabledGradeLevels.filter((g) => {
    const num = parseInt(g.level.replace(/\D/g, ""), 10);
    return g.level.startsWith("grade") && num >= 7 && num <= 9;
  });
  const secGrades = enabledGradeLevels.filter((g) => {
    const num = parseInt(g.level.replace(/\D/g, ""), 10);
    return (
      (g.level.startsWith("grade") && num >= 10 && num <= 12) ||
      g.level.includes("secondary")
    );
  });

  const categorizedLevels = new Set([
    ...kgGrades.map((g) => g.level),
    ...primaryGrades.map((g) => g.level),
    ...prepGrades.map((g) => g.level),
    ...secGrades.map((g) => g.level),
  ]);

  const otherGrades = enabledGradeLevels.filter(
    (g) => !categorizedLevels.has(g.level)
  );

  if (kgGrades.length > 0) {
    groups.push({
      id: "kg",
      name_ar: "مرحلة رياض الأطفال (KG)",
      name_en: "Kindergarten (KG)",
      desc_ar: "صفوف الروضة والتمهيدي",
      desc_en: "Early childhood levels",
      grades: kgGrades,
    });
  }

  if (primaryGrades.length > 0) {
    groups.push({
      id: "primary",
      name_ar: "المرحلة الابتدائية (1 - 6)",
      name_en: "Primary Stage (Grades 1 - 6)",
      desc_ar: "صفوف التعليم الأساسي المبكر",
      desc_en: "Elementary school grades",
      grades: primaryGrades,
    });
  }

  if (prepGrades.length > 0) {
    groups.push({
      id: "prep",
      name_ar: "المرحلة الأساسية / الإعدادية (7 - 9)",
      name_en: "Preparatory / Middle Stage (7 - 9)",
      desc_ar: "صفوف التعليم الأساسي المتقدم",
      desc_en: "Middle school grades",
      grades: prepGrades,
    });
  }

  if (secGrades.length > 0) {
    groups.push({
      id: "secondary",
      name_ar: "المرحلة الثانوية (1 - 3 ثانوي)",
      name_en: "Secondary Stage (10 - 12)",
      desc_ar: "صفوف التعليم الثانوي التخصصي",
      desc_en: "High school grades",
      grades: secGrades,
    });
  }

  if (otherGrades.length > 0) {
    groups.push({
      id: "other",
      name_ar: "المراحل والصفوف الأكاديمية",
      name_en: "Academic Stages & Levels",
      desc_ar: "كافة المستويات والصفوف المفعلة",
      desc_en: "Active grades & modules",
      grades: otherGrades,
    });
  }

  return groups;
});

function getStageFee(grades: GradeLevel[]): number | "" {
  if (grades.length === 0) return "";
  const firstVal = onboardingState.finance.registrationFees[grades[0].level];
  const allSame = grades.every(
    (g) => onboardingState.finance.registrationFees[g.level] === firstVal,
  );
  return allSame && firstVal !== undefined ? firstVal : (firstVal ?? "");
}

function isStageUniform(grades: GradeLevel[]): boolean {
  if (grades.length <= 1) return true;
  const firstVal = onboardingState.finance.registrationFees[grades[0].level];
  return grades.every(
    (g) => onboardingState.finance.registrationFees[g.level] === firstVal,
  );
}

function setStageFee(grades: GradeLevel[], amount: number | "") {
  const val = amount === "" || amount === undefined || Number(amount) < 0 ? 0 : Number(amount);
  for (const g of grades) {
    onboardingState.finance.registrationFees[g.level] = val;
  }
  onboardingState.save();
}

function applyGlobalFee() {
  if (globalBatchAmount === "" || Number(globalBatchAmount) < 0) return;
  const val = Number(globalBatchAmount);
  for (const g of enabledGradeLevels) {
    onboardingState.finance.registrationFees[g.level] = val;
  }
  onboardingState.save();
  showGlobalBatchBar = false;
}

function toggleExpandStage(stageId: string) {
  expandedStages[stageId] = !expandedStages[stageId];
}
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-3 text-start animate-in fade-in duration-300">
  
  <!-- Header Bar -->
  <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
    <div>
      <h3 class="text-xs sm:text-sm font-bold text-foreground leading-tight">
        {m.common_onboarding_fees_label({ currency: onboardingState.profile.currency })}
      </h3>
      <p class="text-[10.5px] text-muted-foreground mt-0.5">
        {l(
          "حدد الرسوم الدراسية والتسجيل للمراحل التعليمية مع إمكانية تخصيص رسوم كل صف",
          "Configure tuition and registration fees per stage group with optional grade-level overrides",
        )}
      </p>
    </div>

    <!-- Right Controls: Currency Badge & Global Batch Button -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="xs"
        class="h-7 text-[11px] gap-1.5 font-bold cursor-pointer text-muted-foreground hover:text-foreground border-border/70"
        onclick={() => (showGlobalBatchBar = !showGlobalBatchBar)}
      >
        <SlidersHorizontal size={12} />
        <span>{l("تسعير موحد للكل", "Global Fee")}</span>
      </Button>

      <span class="text-[11px] font-bold text-primary bg-primary/10 border border-primary/20 px-2.5 py-0.5 rounded-full font-mono">
        {onboardingState.profile.currency}
      </span>
    </div>
  </div>

  <!-- Optional Global Batch Toolbar -->
  {#if showGlobalBatchBar}
    <div class="flex items-center justify-between gap-3 p-3 rounded-2xl bg-card border border-border/70 shadow-2xs animate-in fade-in shrink-0">
      <span class="text-xs font-bold text-foreground">
        {l("تطبيق مبلغ موحد لكافة المراحل والصفوف:", "Apply uniform amount across all stages:")}
      </span>
      <div class="flex items-center gap-2">
        <div class="relative w-36">
          <Input
            type="number"
            min="0"
            placeholder="0"
            bind:value={globalBatchAmount}
            class="h-7.5 text-xs font-bold font-mono bg-background pe-8 text-start"
          />
          <div class="absolute inset-y-0 inset-e-2 flex items-center pointer-events-none text-[9px] font-bold text-muted-foreground uppercase">
            {onboardingState.profile.currency}
          </div>
        </div>
        <Button
          size="xs"
          class="h-7.5 px-3 text-xs font-bold gap-1 cursor-pointer"
          onclick={applyGlobalFee}
          disabled={globalBatchAmount === "" || Number(globalBatchAmount) < 0}
        >
          <Check size={12} />
          <span>{l("تطبيق", "Apply")}</span>
        </Button>
      </div>
    </div>
  {/if}

  <!-- Stage Groups Structured Vertical List -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide space-y-3 p-0.5">
    {#each stageGroups as group (group.id)}
      {@const currentStageFee = getStageFee(group.grades)}
      {@const isUniform = isStageUniform(group.grades)}
      {@const isExpanded = !!expandedStages[group.id]}

      <div class="rounded-2xl border border-border/70 bg-card shadow-2xs overflow-hidden transition-all duration-150">
        
        <!-- Stage Row / Header -->
        <div class="p-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3 bg-muted/10">
          
          <!-- Stage Identity -->
          <div>
            <div class="flex items-center gap-2">
              <h4 class="font-bold text-xs sm:text-sm text-foreground">
                {l(group.name_ar, group.name_en)}
              </h4>
              <Badge variant="secondary" class="text-[10px] px-2 py-0 font-medium">
                {group.grades.length} {l("صفوف", "grades")}
              </Badge>
              {#if !isUniform}
                <Badge variant="outline" class="text-[9.5px] px-1.5 py-0 text-amber-600 dark:text-amber-400 border-amber-500/30 bg-amber-500/10 font-medium">
                  {l("رسوم متباينة", "Customized")}
                </Badge>
              {/if}
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              {l(group.desc_ar, group.desc_en)}
            </p>
          </div>

          <!-- Stage Fee Setting Input & Grade Customization Toggle -->
          <div class="flex items-center gap-2.5 shrink-0 self-end sm:self-auto">
            
            <!-- Stage Uniform Fee Input -->
            <div class="flex items-center gap-1.5 bg-background p-1 rounded-xl border border-border/70 shadow-2xs">
              <span class="text-[11px] font-bold text-muted-foreground px-2">
                {l("رسوم المرحلة:", "Stage Fee:")}
              </span>
              <div class="relative w-32">
                <Input
                  type="number"
                  min="0"
                  placeholder={isUniform ? "0" : l("متباين", "Mixed")}
                  value={currentStageFee}
                  oninput={(e) => {
                    const target = e.target as HTMLInputElement;
                    setStageFee(group.grades, target.value === "" ? "" : Number(target.value));
                  }}
                  class="h-7 text-xs font-bold font-mono bg-background pe-8 text-start border-transparent focus:border-primary"
                />
                <div class="absolute inset-y-0 inset-e-2 flex items-center pointer-events-none text-[9px] font-bold text-muted-foreground uppercase font-mono">
                  {onboardingState.profile.currency}
                </div>
              </div>
            </div>

            <!-- Toggle Details Button -->
            <Button
              variant="outline"
              size="xs"
              class="h-8.5 px-3 text-xs gap-1.5 font-bold cursor-pointer border-border/70 bg-background"
              onclick={() => toggleExpandStage(group.id)}
            >
              <span class="text-[11px]">{isExpanded ? l("إخفاء الصفوف", "Hide") : l("تخصيص الصفوف", "Customize")}</span>
              {#if isExpanded}
                <ChevronUp size={13} />
              {:else}
                <ChevronDown size={13} />
              {/if}
            </Button>

          </div>
        </div>

        <!-- Collapsible Individual Grade Customization Area -->
        {#if isExpanded}
          <div class="p-3.5 border-t border-border/40 bg-muted/5 space-y-2.5 animate-in fade-in duration-150">
            <div class="flex items-center justify-between pb-1">
              <span class="text-[11px] font-bold text-muted-foreground">
                {l("تخصيص الرسوم لكل صف دراسي في هذه المرحلة:", "Individual fee customization per grade:")}
              </span>
              {#if !isUniform}
                <Button
                  variant="ghost"
                  size="xs"
                  class="h-6 px-2 text-[10px] gap-1 text-muted-foreground hover:text-foreground cursor-pointer"
                  onclick={() => setStageFee(group.grades, currentStageFee)}
                >
                  <RotateCcw size={10} />
                  <span>{l("إعادة التوحيد مع المرحلة", "Reset to Stage Fee")}</span>
                </Button>
              {/if}
            </div>

            <!-- Grade Items Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-2.5">
              {#each group.grades as gl (gl.level)}
                {@const isGradeCustom = currentStageFee !== "" && onboardingState.finance.registrationFees[gl.level] !== currentStageFee}

                <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/70 bg-background hover:bg-muted/10 transition-colors shadow-2xs gap-2">
                  <div class="flex items-center gap-2 min-w-0 flex-1">
                    <div class="size-2 rounded-full shrink-0" style="background-color: {gl.color || '#3b82f6'};"></div>
                    <span class="font-bold text-foreground text-xs truncate">
                      {l(gl.name_ar, gl.name)}
                    </span>
                  </div>

                  <div class="relative w-28 shrink-0">
                    <Input
                      type="number"
                      min="0"
                      placeholder="0"
                      bind:value={onboardingState.finance.registrationFees[gl.level]}
                      onchange={() => onboardingState.save()}
                      class="h-7 text-xs font-bold font-mono bg-background pe-8 text-start border-border/60 focus:border-primary {isGradeCustom ? 'border-amber-500/60 ring-1 ring-amber-500/20' : ''}"
                    />
                    <div class="absolute inset-y-0 inset-e-2 flex items-center pointer-events-none text-[8.5px] font-bold text-muted-foreground uppercase font-mono">
                      {onboardingState.profile.currency}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}

      </div>
    {:else}
      <div class="py-16 text-center text-xs text-muted-foreground">
        {m.common_onboarding_fees_no_stages({ stage: onboardingState.stageLabel })}
      </div>
    {/each}
  </div>

</div>


