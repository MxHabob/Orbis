<!--
  Step 7: Review & Launch
  Summary of configuration and final launch button.
-->
<script lang="ts">
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import {
  School,
  Calendar,
  Building,
  GraduationCap,
  Sparkles,
  Loader2,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";

async function handleCompleteSetup() {
  const orgId = authStore.orgId;
  if (!orgId) return;
  const success = await onboardingState.completeSetup(orgId);
  if (success) {
    authStore.markSetupComplete();
    goto(localizeHref(`/${orgId}`));
  }
}
</script>

<div class="max-w-4xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_review_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_review_desc()}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- School info card -->
    <Card class="bg-card border-border">
      <CardHeader class="pb-2 text-start">
        <CardTitle class="text-sm font-semibold text-muted-foreground flex gap-2 items-center font-serif">
          <School class="w-4 h-4 text-primary" />
          <span>{m.onboarding_review_institution_info()}</span>
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-2 text-sm text-start">
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_institution_name()}</span>
          <span class="font-medium text-foreground">{onboardingState.orgNameAr}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_org_id()}</span>
          <span class="font-mono text-xs text-muted-foreground">{authStore.orgId}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_currency()}</span>
          <span class="font-medium text-emerald-500">{onboardingState.currency}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_locale_timezone()}</span>
          <span class="font-medium text-foreground">
            {onboardingState.localizedLanguageName} / {onboardingState.timezone}
          </span>
        </div>
      </CardContent>
    </Card>

    <!-- Calendar card -->
    <Card class="bg-card border-border">
      <CardHeader class="pb-2 text-start">
        <CardTitle class="text-sm font-semibold text-muted-foreground flex gap-2 items-center font-serif">
          <Calendar class="w-4 h-4 text-primary" />
          <span>{onboardingState.calendarLabel}</span>
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-2 text-sm font-sans text-start">
        <div class="flex justify-between">
          <span class="text-muted-foreground font-medium">{onboardingState.calendarLabel}</span>
          <span class="font-medium text-foreground font-sans">{onboardingState.academicYearNameAr}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground font-medium">{m.onboarding_review_year_duration()}</span>
          <span class="font-mono text-xs text-muted-foreground">{onboardingState.startDate} إلى {onboardingState.endDate}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground font-medium">{m.onboarding_review_sem1()}</span>
          <span class="font-mono text-xs text-muted-foreground">{onboardingState.semester1Start} إلى {onboardingState.semester1End}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground font-medium">{m.onboarding_review_sem2()}</span>
          <span class="font-mono text-xs text-muted-foreground">{onboardingState.semester2Start} إلى {onboardingState.semester2End}</span>
        </div>
      </CardContent>
    </Card>

    <!-- Buildings & Rooms card -->
    <Card class="bg-card border-border">
      <CardHeader class="pb-2 text-start">
        <CardTitle class="text-sm font-semibold text-muted-foreground flex gap-2 items-center font-serif">
          <Building class="w-4 h-4 text-primary" />
          <span>{m.onboarding_review_buildings_halls()}</span>
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-2 text-sm text-start">
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_bld1()}</span>
          <span class="font-medium text-foreground">
            {onboardingState.bld1Rooms} {onboardingState.classroomsLabelPlural}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_bld2()}</span>
          <span class="font-medium text-foreground">
            {onboardingState.bld2Rooms} {onboardingState.classroomsLabelPlural}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_capacity()}</span>
          <span class="font-medium text-primary">
            {m.onboarding_review_capacity_value({ count: onboardingState.totalCapacity })}
          </span>
        </div>
      </CardContent>
    </Card>

    <!-- Stages & Subjects card -->
    <Card class="bg-card border-border">
      <CardHeader class="pb-2 text-start">
        <CardTitle class="text-sm font-semibold text-muted-foreground flex gap-2 items-center font-serif">
          <GraduationCap class="w-4 h-4 text-primary" />
          <span>{onboardingState.stageLabel}</span>
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-2 text-sm text-start">
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_enabled_stages({ stage: onboardingState.stageLabel })}</span>
          <span class="font-medium text-foreground">
            {onboardingState.enabledStages} {onboardingState.stagesLabelPlural}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_divisions_label({ division: onboardingState.divisionLabel })}</span>
          <span class="font-medium text-foreground">
            {onboardingState.totalDivisions}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_subjects_label({ subject: onboardingState.subjectLabel })}</span>
          <span class="font-medium text-foreground">
            {onboardingState.enabledSubjects}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">{m.onboarding_review_time_slots()}</span>
          <span class="font-medium text-foreground">
            {m.onboarding_review_slots_value({ count: onboardingState.totalSlots })}
          </span>
        </div>
      </CardContent>
    </Card>
  </div>

  <div class="p-5 rounded-xl border border-border bg-muted text-muted-foreground text-xs leading-relaxed max-w-4xl space-y-1 text-start">
    <h5 class="font-semibold text-foreground font-serif">{m.onboarding_review_warn_title()}</h5>
    <p>{m.onboarding_review_warn_desc()}</p>
  </div>

  <!-- Central Launch Button -->
  <div class="pt-6 flex justify-center">
    <Button
      onclick={handleCompleteSetup}
      disabled={onboardingState.isLoading}
      size="lg"
      class="px-10 py-6 text-base font-bold bg-primary text-primary-foreground hover:bg-primary/90 rounded-xl shadow-lg flex gap-3 items-center justify-center transition-transform hover:scale-105"
    >
      {#if onboardingState.isLoading}
        <Loader2 class="w-5 h-5 animate-spin" />
        <span>{m.onboarding_review_launching_btn()}</span>
      {:else}
        <Sparkles class="w-5 h-5 animate-pulse" />
        <span>{m.onboarding_review_launch_btn()}</span>
      {/if}
    </Button>
  </div>
</div>
