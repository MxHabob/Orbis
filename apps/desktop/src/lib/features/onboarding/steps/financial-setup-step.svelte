<!--
  Step 6: Tuition & Registration Fees (الرسوم الدراسية)
  Categorized by educational stage groups (Kindergarten, Primary, Preparatory, Secondary)
  with unified group fee batch setting and individual grade customization.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  DollarSign,
  GraduationCap,
  BookOpen,
  Sparkles,
  School,
  Check,
  Layers,
  ArrowDownToDot,
} from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import type { GradeLevel } from "../types";
import { l } from "$lib/core/i18n";

const enabledGradeLevels = $derived(
  onboardingState.academics.gradeLevels.filter((gl) => gl.enabled),
);

// Group state for batch fee inputs
let groupBatchAmounts = $state<Record<string, number | "">>({
  kg: "",
  primary: "",
  prep: "",
  secondary: "",
  other: "",
});

interface EducationalStageGroup {
  id: string;
  name_ar: string;
  name_en: string;
  desc_ar: string;
  desc_en: string;
  icon: any;
  color: string;
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
      icon: Sparkles,
      color: "text-pink-500 bg-pink-500/10 border-pink-500/20",
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
      icon: BookOpen,
      color: "text-blue-500 bg-blue-500/10 border-blue-500/20",
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
      icon: GraduationCap,
      color: "text-emerald-500 bg-emerald-500/10 border-emerald-500/20",
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
      icon: School,
      color: "text-purple-500 bg-purple-500/10 border-purple-500/20",
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
      icon: Layers,
      color: "text-amber-500 bg-amber-500/10 border-amber-500/20",
      grades: otherGrades,
    });
  }

  return groups;
});

function applyBatchFeeToGroup(groupId: string, grades: GradeLevel[]) {
  const amount = groupBatchAmounts[groupId];
  if (amount === "" || amount === undefined || Number(amount) < 0) return;

  const numAmount = Number(amount);
  for (const g of grades) {
    onboardingState.finance.registrationFees[g.level] = numAmount;
  }
  onboardingState.save();
}
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-2.5 text-start">
  
  <!-- Header Bar -->
  <div class="flex items-center justify-between border-b border-border/50 pb-1.5 shrink-0">
    <div class="flex items-center gap-2">
      <DollarSign size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {m.common_onboarding_fees_label({ currency: onboardingState.profile.currency })}
      </h3>
    </div>
    <div class="flex items-center gap-2">
      <span class="text-[11px] font-bold text-muted-foreground font-mono">
        {l("تحديد الرسوم حسب المجموعات والمراحل", "Fee structure by stage groups")}
      </span>
      <span class="text-[11px] font-bold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2.5 py-0.5 rounded-full font-mono">
        {onboardingState.profile.currency}
      </span>
    </div>
  </div>

  <!-- Stage Groups Structured Vertical List (Expands to fill 100% available space) -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide space-y-3.5 p-1">
    {#each stageGroups as group (group.id)}
      <div class="rounded-2xl border border-border/80 bg-card/90 shadow-2xs overflow-hidden p-3.5 space-y-3">
        
        <!-- Group Header & Bulk Fee Setter -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-2.5 border-b border-border/50">
          <div class="flex items-center gap-2.5">
            <div class="size-7 rounded-xl border flex items-center justify-center {group.color}">
              <group.icon size={14} />
            </div>
            <div>
              <div class="flex items-center gap-2">
                <h4 class="font-bold text-xs sm:text-sm text-foreground">
                  {l(group.name_ar, group.name_en)}
                </h4>
                <Badge variant="secondary" class="text-[10px] px-1.5 py-0 font-mono">
                  {group.grades.length} {l("صفوف", "grades")}
                </Badge>
              </div>
              <p class="text-[10.5px] text-muted-foreground">
                {l(group.desc_ar, group.desc_en)}
              </p>
            </div>
          </div>

          <!-- Batch Fee Setter Input & Button -->
          <div class="flex items-center gap-1.5 shrink-0 bg-muted/30 p-1 rounded-xl border border-border/60">
            <div class="relative w-28">
              <Input
                type="number"
                min="0"
                placeholder={l("مبلغ موحد", "Uniform Fee")}
                bind:value={groupBatchAmounts[group.id]}
                class="h-7 text-xs font-bold font-mono bg-background text-end pe-2 ps-2"
              />
            </div>
            <Button
              variant="outline"
              size="xs"
              class="h-7 px-2.5 text-[11px] gap-1 font-bold cursor-pointer border-border shrink-0"
              onclick={() => applyBatchFeeToGroup(group.id, group.grades)}
              disabled={groupBatchAmounts[group.id] === "" || Number(groupBatchAmounts[group.id]) < 0}
            >
              <ArrowDownToDot size={12} />
              <span>{l("تطبيق للمجموعة", "Apply to Group")}</span>
            </Button>
          </div>
        </div>

        <!-- Grade Cards Grid inside this Group -->
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-6 gap-2.5">
          {#each group.grades as gl (gl.level)}
            <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/70 bg-background/80 hover:bg-muted/15 transition-colors shadow-2xs">
              <div class="flex items-center gap-2 min-w-0">
                <div class="size-2.5 rounded-full shrink-0" style="background-color: {gl.color};"></div>
                <span class="font-bold text-foreground text-xs truncate">{l(gl.name_ar, gl.name)}</span>
              </div>

              <div class="relative w-28 shrink-0">
                <Input
                  type="number"
                  min="0"
                  placeholder="0"
                  bind:value={onboardingState.finance.registrationFees[gl.level]}
                  onchange={() => onboardingState.save()}
                  class="h-7 ps-8 text-end text-xs font-bold font-mono bg-background border-border/70 focus:border-primary"
                />
                <div class="absolute inset-y-0 inset-s-2 flex items-center pointer-events-none text-[9px] font-bold text-muted-foreground uppercase font-sans">
                  {onboardingState.profile.currency}
                </div>
              </div>
            </div>
          {/each}
        </div>

      </div>
    {:else}
      <div class="py-16 text-center text-xs text-muted-foreground">
        {m.common_onboarding_fees_no_stages({ stage: onboardingState.stageLabel })}
      </div>
    {/each}
  </div>

</div>
