<!--
  @component StepReview
  Step 4: Summary Review & System Bootstrap Execution.
  Minimalist layout using @orbis/ui components and design tokens.
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  CURRENCY_OPTIONS,
  TIMEZONE_OPTIONS,
} from "../types";
import { getBlueprint } from "@orbis/blueprints";
import { Button } from "@orbis/ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";

let {
  form,
  isSubmitting = false,
  isComplete = false,
  error = null,
  activeTaskIndex = 0,
  onLaunch,
  onRetry,
  onEnterWorkspace,
  isRTL = false,
}: {
  form: OnboardingFormModel;
  isSubmitting?: boolean;
  isComplete?: boolean;
  error?: string | null;
  activeTaskIndex?: number;
  onLaunch: () => void;
  onRetry: () => void;
  onEnterWorkspace: () => void;
  isRTL?: boolean;
} = $props();

const blueprint = $derived(getBlueprint(form.institutionType));
const tz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const cur = $derived(CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode));

const executionTasksEn = [
  "Creating organization profile",
  "Configuring institution blueprint",
  "Provisioning Administrator account",
  "Initializing foundational ledger",
  "Verifying system readiness",
];

const executionTasksAr = [
  "إنشاء سجل المؤسسة",
  "تهيئة نموذج المؤسسة الأكاديمي",
  "إنشاء حساب المشرف العام",
  "تهيئة دليل الحسابات الأولي",
  "التحقق من الجاهزية التشغيلية",
];

const tasks = $derived(isRTL ? executionTasksAr : executionTasksEn);
</script>

<div class="space-y-6 max-w-3xl mx-auto">
  <div class="space-y-1">
    <h2 class="text-lg font-semibold tracking-tight text-foreground">
      {isComplete
        ? (isRTL ? "تمت تهيئة النظام بنجاح" : "System Initialized")
        : isSubmitting
          ? (isRTL ? "جاري تهيئة النظام..." : "Initializing System...")
          : (isRTL ? "مراجعة البيانات وبدء التهيئة" : "Review & Launch Setup")}
    </h2>
    <p class="text-xs text-muted-foreground">
      {isComplete
        ? (isRTL ? "تم إعداد مساحة العمل الخاصة بمؤسستك وجاهزة للاستخدام." : "Your workspace has been successfully provisioned.")
        : isSubmitting
          ? (isRTL ? "يرجى الانتظار ريثما تكتمل التهيئة الأساسية للمؤسسة." : "Please wait while Orbis provisions initial records.")
          : (isRTL ? "تحقق من صحة البيانات أدناه قبل المتابعة لإنشاء المنشأة." : "Confirm your settings before executing system bootstrap.")}
    </p>
  </div>

  {#if isComplete}
    <Card class="border-border/60 shadow-xs text-center p-6 space-y-4">
      <div class="space-y-1">
        <h3 class="text-base font-semibold text-foreground">
          {isRTL ? "أهلاً بك في أوربيس" : "Welcome to Orbis"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-md mx-auto">
          {isRTL
            ? `تم تأسيس ${form.organizationNameAr || form.organizationName} بنجاح، وتعيين ${form.adminEmail} كمشرف عام.`
            : `${form.organizationName} has been initialized with ${form.adminEmail} as Primary Administrator.`}
        </p>
      </div>

      <div class="pt-2">
        <Button
          type="button"
          variant="default"
          size="sm"
          onclick={onEnterWorkspace}
          class="h-9 px-6 text-xs font-medium"
        >
          {isRTL ? "الدخول إلى مساحة العمل" : "Enter Workspace"}
        </Button>
      </div>
    </Card>
  {:else if isSubmitting}
    <Card class="border-border/60 shadow-xs p-6 space-y-4">
      <div class="space-y-2">
        {#each tasks as task, idx}
          {@const isDone = idx < activeTaskIndex}
          {@const isCurrent = idx === activeTaskIndex}
          
          <div
            class="flex items-center gap-2.5 text-xs py-1 {isCurrent
              ? 'text-foreground font-medium'
              : isDone
                ? 'text-muted-foreground'
                : 'text-muted-foreground/40'}"
          >
            <span class="size-2 rounded-full {isDone ? 'bg-emerald-500' : isCurrent ? 'bg-primary animate-pulse' : 'bg-muted'}"></span>
            <span>{task}</span>
          </div>
        {/each}
      </div>
    </Card>
  {:else}
    <div class="space-y-4">
      {#if error}
        <div class="p-3 rounded-lg border border-destructive/30 bg-destructive/10 text-destructive text-xs">
          <span class="font-medium">{isRTL ? "خطأ في التهيئة:" : "Bootstrap Error:"}</span>
          <span class="ms-1">{error}</span>
        </div>
      {/if}

      <div class="grid grid-cols-1 md:grid-cols-3 gap-3.5">
        <!-- Blueprint -->
        <Card class="border-border/60 shadow-xs">
          <CardHeader class="p-4 pb-2">
            <span class="text-[11px] text-muted-foreground uppercase font-mono">
              {isRTL ? "النموذج" : "Blueprint"}
            </span>
            <CardTitle class="text-sm font-semibold text-foreground">
              {isRTL ? blueprint.nameAr : blueprint.nameEn}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground line-clamp-2">
            {isRTL ? blueprint.descriptionAr : blueprint.descriptionEn}
          </CardContent>
        </Card>

        <!-- Org -->
        <Card class="border-border/60 shadow-xs">
          <CardHeader class="p-4 pb-2">
            <span class="text-[11px] text-muted-foreground uppercase font-mono">
              {isRTL ? "المنشأة" : "Organization"}
            </span>
            <CardTitle class="text-sm font-semibold text-foreground truncate">
              {form.organizationName}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground space-y-0.5 font-mono">
            <div>/{form.organizationSlug}</div>
            <div>{cur?.code} • {tz?.offset}</div>
          </CardContent>
        </Card>

        <!-- Admin -->
        <Card class="border-border/60 shadow-xs">
          <CardHeader class="p-4 pb-2">
            <span class="text-[11px] text-muted-foreground uppercase font-mono">
              {isRTL ? "المشرف" : "Administrator"}
            </span>
            <CardTitle class="text-sm font-semibold text-foreground truncate">
              {form.adminFirstName} {form.adminLastName}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground truncate font-mono">
            {form.adminEmail}
          </CardContent>
        </Card>
      </div>

      <!-- Action Card -->
      <Card class="border-border/60 shadow-xs p-5 text-center space-y-3">
        <p class="text-xs text-muted-foreground">
          {isRTL
            ? "سيتم إنشاء الحساب الإداري الأول ودليل الحسابات المالي الأولي مباشرة."
            : "This will provision the organization foundation and initial administrator."}
        </p>

        <Button
          type="button"
          variant="default"
          size="sm"
          onclick={onLaunch}
          class="h-9 px-6 text-xs font-medium"
        >
          <span>{isRTL ? "بدء تهيئة النظام" : "Initialize System"}</span>
        </Button>
      </Card>
    </div>
  {/if}
</div>
