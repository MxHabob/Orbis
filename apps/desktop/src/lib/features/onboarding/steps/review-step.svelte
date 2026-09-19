<!--
  Step 7: Review & Summary (مراجعة البيانات)
  Compact 2x2 grid summary of configuration before final launch.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";
import { l } from "$lib/core/i18n";
import { Building2, Calendar, GraduationCap, DollarSign, CheckCircle2 } from "@lucide/svelte";
</script>

<div class="w-full space-y-3 text-start">
  <div class="flex items-center justify-between border-b border-border/50 pb-1.5">
    <div class="flex items-center gap-2">
      <CheckCircle2 size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {m.common_onboarding_review_launch_btn ? "مراجعة التهيئة الأكاديمية" : "Setup Summary"}
      </h3>
    </div>
    <span class="text-[11px] font-bold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-full">
      {l("جاهز للإطلاق", "Ready to Launch")}
    </span>
  </div>

  <!-- Multi-Column Clean Summary Grid (Expanded for Widescreen) -->
  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-3.5 max-h-[340px] overflow-y-auto scrollbar-hide pr-1">

    <!-- Card 1: Institution Details -->
    <div class="border border-border/70 rounded-2xl bg-card p-3 space-y-2 shadow-2xs">
      <div class="flex items-center gap-1.5 text-xs font-bold text-primary border-b border-border/40 pb-1.5">
        <Building2 size={13} />
        <h4>{m.common_onboarding_review_institution_info()}</h4>
      </div>

      <div class="space-y-1.5 text-xs">
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_institution_name()}</span>
          <span class="font-bold text-foreground truncate max-w-[180px]">{l(onboardingState.profile.orgNameAr, onboardingState.profile.orgName)}</span>
        </div>
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_currency()}</span>
          <span class="font-bold text-emerald-600 dark:text-emerald-400 font-mono">{onboardingState.profile.currency}</span>
        </div>
        <div class="flex justify-between py-0.5">
          <span class="text-muted-foreground">{m.common_onboarding_review_locale_timezone()}</span>
          <span class="font-bold text-foreground font-mono text-[11px]">{onboardingState.profile.timezone}</span>
        </div>
      </div>
    </div>

    <!-- Card 2: Academic Calendar -->
    <div class="border border-border/70 rounded-2xl bg-card p-3 space-y-2 shadow-2xs">
      <div class="flex items-center gap-1.5 text-xs font-bold text-primary border-b border-border/40 pb-1.5">
        <Calendar size={13} />
        <h4>{onboardingState.calendarLabel}</h4>
      </div>

      <div class="space-y-1.5 text-xs">
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">{onboardingState.calendarLabel}</span>
          <span class="font-bold text-foreground truncate max-w-[180px]">{l(onboardingState.calendar.academicYearNameAr, onboardingState.calendar.academicYearName)}</span>
        </div>
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_year_duration()}</span>
          <span class="font-bold text-foreground font-mono text-[11px]">{onboardingState.calendar.startDate} &rarr; {onboardingState.calendar.endDate}</span>
        </div>
        <div class="flex justify-between py-0.5">
          <span class="text-muted-foreground">أيام الدراسة</span>
          <span class="font-bold text-foreground">{onboardingState.academics.workingDays.length} أيام / {onboardingState.academics.totalSlots} حصص</span>
        </div>
      </div>
    </div>

    <!-- Card 3: Facilities & Spaces -->
    <div class="border border-border/70 rounded-2xl bg-card p-3 space-y-2 shadow-2xs">
      <div class="flex items-center gap-1.5 text-xs font-bold text-primary border-b border-border/40 pb-1.5">
        <Building2 size={13} />
        <h4>{m.common_onboarding_review_buildings_halls()}</h4>
      </div>

      <div class="space-y-1.5 text-xs">
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">المباني والقاعات</span>
          <span class="font-bold text-foreground">{onboardingState.facilities.buildings.length} مبنى / {onboardingState.facilities.rooms.length} قاعة</span>
        </div>
        <div class="flex justify-between py-0.5">
          <span class="text-muted-foreground">{m.common_onboarding_review_capacity()}</span>
          <span class="font-bold text-primary font-mono">{onboardingState.facilities.totalCapacity} طالب</span>
        </div>
      </div>
    </div>

    <!-- Card 4: Academics & Fees -->
    <div class="border border-border/70 rounded-2xl bg-card p-3 space-y-2 shadow-2xs">
      <div class="flex items-center gap-1.5 text-xs font-bold text-primary border-b border-border/40 pb-1.5">
        <GraduationCap size={13} />
        <h4>{onboardingState.stageLabel} & {onboardingState.subjectLabel}</h4>
      </div>

      <div class="space-y-1.5 text-xs">
        <div class="flex justify-between py-0.5 border-b border-border/30">
          <span class="text-muted-foreground">المراحل والفصول</span>
          <span class="font-bold text-foreground">{onboardingState.academics.enabledStages} مراحل / {onboardingState.academics.totalDivisions} فصل</span>
        </div>
        <div class="flex justify-between py-0.5">
          <span class="text-muted-foreground">المواد المفعلة</span>
          <span class="font-bold text-foreground">{onboardingState.academics.enabledSubjects} مادة دراسية</span>
        </div>
      </div>
    </div>

  </div>
</div>
