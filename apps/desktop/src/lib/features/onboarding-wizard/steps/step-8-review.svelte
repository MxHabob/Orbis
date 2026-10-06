<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
import { api } from "$lib/api";
import { systemStore, authStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Progress } from "@orbis/ui/progress";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { deriveSteps, type OnboardingStep } from "../onboarding-steps";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

let isRefreshing = $state(false);
let isLaunching = $state(false);
let launchSuccess = $state(false);
let errorMessage = $state<string | null>(null);

onMount(async () => {
  await refreshDAG();
});

async function refreshDAG() {
  isRefreshing = true;
  try {
    await systemStore.loadOnboardingState(orgId);
    await systemStore.loadReadiness();
  } catch (err: any) {
    console.error("Failed to refresh onboarding DAG state:", err);
    errorMessage = err?.message || "Failed to calculate readiness";
  } finally {
    isRefreshing = false;
  }
}

const state = $derived(systemStore.onboardingState);
const tasks = $derived(systemStore.onboardingTasks);
const steps = $derived(deriveSteps(tasks));
const overallProgress = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);
const requiredTasks = $derived(tasks.filter((t) => t.is_required));
const completedRequiredTasks = $derived(requiredTasks.filter((t) => t.status === "completed"));
const canLaunch = $derived(completedRequiredTasks.length === requiredTasks.length || isReady);

async function handleCompleteAndLaunch() {
  isLaunching = true;
  errorMessage = null;

  try {
    await refreshDAG();
    launchSuccess = true;

    // Wait a brief moment to show celebration state, then navigate to dashboard
    setTimeout(async () => {
      await goto(`/${orgId}`);
    }, 1200);
  } catch (err: any) {
    console.error("Failed to complete onboarding:", err);
    errorMessage = err?.message || "Failed to complete setup";
    isLaunching = false;
  }
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={8}
    title="Final Review & Organization Launch"
    titleAr="المراجعة الشاملة وإطلاق المنشأة التعليمية"
    description="Verify that all structural, academic, financial, and operational requirements have been configured."
    descriptionAr="تحقق من اكتمال كافة المتطلبات الهيكلية والأكاديمية والمالية والتشغيلية لإطلاق النظام."
    isRequired={true}
    status={isReady ? "completed" : "in_progress"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if launchSuccess}
    <div class="rounded-xl border border-emerald-500/40 bg-emerald-500/10 p-8 text-center space-y-4 animate-in fade-in zoom-in duration-300">
      <div class="mx-auto flex size-16 items-center justify-center rounded-full bg-emerald-500 text-white text-3xl font-bold shadow-lg shadow-emerald-500/30">
        ✓
      </div>
      <h2 class="text-2xl font-bold text-foreground">
        {isArabic ? "تهانينا! تم إكمال تهيئة المنشأة بنجاح" : "Congratulations! Setup is Complete"}
      </h2>
      <p class="text-sm text-muted-foreground max-w-md mx-auto">
        {isArabic ? "جاري نقلك إلى لوحة التحكم الرئيسية لمباشرة العمليات الأكاديمية والمالية..." : "Redirecting to your workspace dashboard to start operations..."}
      </p>
      <div class="flex justify-center pt-2">
        <span class="size-6 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
      </div>
    </div>
  {:else}
    <!-- Readiness Scorecard -->
    <div class="rounded-xl border border-border/60 bg-card p-6 shadow-xs space-y-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            {isArabic ? "مستوى الجاهزية التشغيلية" : "Operational Readiness Score"}
          </span>
          <div class="text-3xl font-extrabold text-foreground mt-1 flex items-center gap-3">
            <span>{overallProgress}%</span>
            {#if isReady}
              <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-xs py-1 px-2.5">
                {isArabic ? "جاهز للتشغيل الرسمي" : "Production Ready"}
              </Badge>
            {:else}
              <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400 text-xs py-1 px-2.5">
                {isArabic ? `${completedRequiredTasks.length} من ${requiredTasks.length} مهام إلزامية` : `${completedRequiredTasks.length} of ${requiredTasks.length} Required Tasks`}
              </Badge>
            {/if}
          </div>
        </div>

        <Button
          variant="outline"
          size="sm"
          disabled={isRefreshing}
          onclick={refreshDAG}
          class="text-xs self-start sm:self-auto"
        >
          {isRefreshing ? (isArabic ? "جاري الفحص..." : "Auditing...") : (isArabic ? "🔄 إعادة الفحص الشامل" : "🔄 Re-evaluate DAG")}
        </Button>
      </div>

      <Progress value={overallProgress} class="h-2.5 bg-muted" />
    </div>

    <!-- Stage-by-Stage Audit Matrix -->
    <OnboardingSection
      title="Stage-by-Stage Verification Checklist"
      titleAr="قائمة التدقيق الشامل لمراحل التهيئة"
      description="Click on any stage to review or adjust its configuration."
      descriptionAr="انقر على أي مرحلة لمراجعة تفاصيلها أو تعديل البيانات."
    >
      <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
        {#each steps.filter((s) => s.id !== "review") as step (step.id)}
          <div class="flex items-center justify-between p-4 text-xs transition-colors hover:bg-muted/30">
            <div class="flex items-center gap-3">
              <!-- Step Status Badge -->
              <div class="flex size-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold {step.status === 'completed'
                ? 'bg-emerald-500 text-white'
                : step.status === 'in_progress'
                ? 'bg-blue-500 text-white'
                : 'bg-muted text-muted-foreground'}">
                {#if step.status === "completed"}
                  ✓
                {:else}
                  {step.order}
                {/if}
              </div>

              <div>
                <div class="font-semibold text-foreground flex items-center gap-2">
                  <span>{isArabic ? step.labelAr : step.label}</span>
                  {#if step.isRequired}
                    <span class="text-[10px] text-amber-500 font-medium">*</span>
                  {/if}
                </div>
                <div class="text-[11px] text-muted-foreground">
                  {isArabic ? step.descriptionAr : step.description}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-3">
              {#if step.status === "completed"}
                <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-[10px]">
                  {isArabic ? "مكتمل" : "Ready"}
                </Badge>
              {:else if step.status === "in_progress"}
                <Badge variant="outline" class="border-blue-500/30 bg-blue-500/10 text-blue-600 dark:text-blue-400 text-[10px]">
                  {step.progress}%
                </Badge>
              {:else}
                <Badge variant="secondary" class="text-[10px]">
                  {isArabic ? "غير مكتمل" : "Pending"}
                </Badge>
              {/if}

              <Button
                variant="ghost"
                size="sm"
                onclick={() => goto(`/${orgId}/onboarding/${step.route}`)}
                class="text-xs font-medium text-primary hover:text-primary hover:bg-primary/10"
              >
                {isArabic ? "تعديل ←" : "Configure →"}
              </Button>
            </div>
          </div>
        {/each}
      </div>
    </OnboardingSection>

    <!-- Bottom Action & Final Launch Bar -->
    <div class="flex flex-col sm:flex-row items-center justify-between gap-4 border-t border-border/50 pt-6">
      <Button
        variant="outline"
        onclick={() => goto(`/${orgId}/onboarding/operations`)}
        class="text-xs w-full sm:w-auto"
      >
        {isArabic ? "← الخطوة السابقة (العمليات)" : "← Back to Operations"}
      </Button>

      <div class="flex items-center gap-3 w-full sm:w-auto">
        <Button
          variant="default"
          size="lg"
          disabled={isLaunching}
          onclick={handleCompleteAndLaunch}
          class="text-sm font-bold w-full sm:w-auto shadow-md bg-emerald-600 hover:bg-emerald-700 text-white"
        >
          {isLaunching
            ? (isArabic ? "جاري إطلاق النظام..." : "Launching System...")
            : (isArabic ? "🚀 إتمام التهيئة وبدء تشغيل المنشأة" : "🚀 Complete Setup & Launch Workspace")}
        </Button>
      </div>
    </div>
  {/if}
</div>
