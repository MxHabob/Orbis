<!--
  Step 7: School Registration Fees Setup
  List of active grade levels with input fields to set registration fees.
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Coins } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";

const enabledGradeLevels = $derived(
  onboardingState.gradeLevels.filter((gl) => gl.enabled),
);
const isArabic = $derived(m.attendance_status_unknown() === "غير محدد");
</script>

<div class="max-w-4xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {isArabic ? "رسوم التسجيل للمراحل الدراسية" : "Registration Fees per Grade Level"}
    </h3>
    <p class="text-muted-foreground text-sm">
      {isArabic 
        ? "حدد رسوم التسجيل لكل صف دراسي مفعل. سيقوم النظام تلقائياً بإنشاء قوالب الرسوم وإسنادها للطلاب عند تسجيلهم." 
        : "Define the registration fees for each enabled grade level. The system will auto-create fee templates and apply them during enrollment."}
    </p>
  </div>

  <!-- Registration Fees Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each enabledGradeLevels as gl (gl.level)}
      <div class="bg-card border border-border rounded-xl p-4 flex items-center justify-between gap-4">
        <div class="flex gap-3 items-center text-start flex-1 min-w-0">
          <!-- Color indicator -->
          <div
            class="w-10 h-10 rounded-lg flex items-center justify-center shrink-0"
            style="background-color: {gl.color}20; border: 1px solid {gl.color}40;"
          >
            <Coins class="w-5 h-5 animate-pulse" style="color: {gl.color};" />
          </div>
          <div class="truncate text-start">
            <h4 class="font-bold text-foreground text-sm font-serif">
              {gl.name_ar}
            </h4>
            <p class="text-xs text-muted-foreground font-mono mt-0.5">{gl.name}</p>
          </div>
        </div>

        <div class="w-[180px] space-y-1.5 text-start shrink-0">
          <Label class="text-[11px] font-semibold text-muted-foreground">
            {isArabic ? "رسوم التسجيل (ريال)" : "Registration Fee (YER)"}
          </Label>
          <div class="relative">
            <Input
              type="number"
              min="0"
              placeholder="0.00"
              bind:value={onboardingState.registrationFees[gl.level]}
              class="h-9 pr-12 text-sm text-end font-bold font-mono"
            />
            <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none text-[10px] font-bold text-muted-foreground uppercase">
              YER
            </div>
          </div>
        </div>
      </div>
    {/each}
    {#if enabledGradeLevels.length === 0}
      <div class="col-span-2 text-center py-10 text-muted-foreground border border-dashed rounded-xl">
        {isArabic ? "لم يتم تفعيل أي مراحل دراسية. يرجى العودة لخطوة 'المراحل والفصول'." : "No grade levels enabled. Please go back to 'Grades & Classes' step."}
      </div>
    {/if}
  </div>
</div>
