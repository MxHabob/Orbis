<!--
  @component StepReview
  Step 4: Summary Review, System Bootstrap Execution & Readiness Verification.
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  CURRENCY_OPTIONS,
  TIMEZONE_OPTIONS,
} from "../types";
import { getBlueprint } from "@orbis/blueprints";
import {
  AcademicCap,
  ArrowPath,
  BuildingOffice,
  CheckCircle,
  InfoCircle,
  GlobeAlt,
  ShieldCheck,
  Sparkles,
  UserBadgeCheck,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
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
  "Creating organization profile and persistent state",
  "Configuring institution blueprint and default settings",
  "Provisioning Super Administrator account and IAM roles",
  "Initializing base academic structure and calendar",
  "Verifying system operational readiness",
];

const executionTasksAr = [
  "إنشاء سجل المؤسسة وإعداد مساحة العمل",
  "تهيئة نموذج المؤسسة الأكاديمي والإعدادات الافتراضية",
  "إنشاء حساب المشرف العام وتعيين الصلاحيات الإدارية الكاملة",
  "تهيئة الهيكل الأكاديمي الأساسي والتقويم",
  "التحقق النهائي من جاهزية النظام للتشغيل",
];

const tasks = $derived(isRTL ? executionTasksAr : executionTasksEn);
</script>

<div class="space-y-6">
  <div class="text-center max-w-xl mx-auto space-y-2">
    <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">
      <Sparkles size={14} />
      <span>{isRTL ? "الخطوة الرابعة" : "Step 4 of 4"}</span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isComplete
        ? (isRTL ? "تمت تهيئة النظام بنجاح!" : "System Initialized Successfully!")
        : isSubmitting
          ? (isRTL ? "جاري تهيئة وتشغيل النظام..." : "Initializing & Bootstrapping System...")
          : (isRTL ? "مراجعة البيانات وبدء التهيئة" : "Review & Launch Setup")}
    </h2>
    <p class="text-sm text-muted-foreground">
      {isComplete
        ? (isRTL ? "تم إعداد مساحة العمل الخاصة بمؤسستك وجاهزة للاستخدام الآن." : "Your workspace is ready. You can now access all administrative modules.")
        : isSubmitting
          ? (isRTL ? "يرجى الانتظار ريثما يقوم أوربيس بإنشاء المنظومة وتعيين الصلاحيات." : "Please wait while Orbis configures your organization, roles, and defaults.")
          : (isRTL ? "تحقق من صحة البيانات أدناه قبل المتابعة لإنشاء المنشأة وحساب المشرف." : "Review your settings below before initiating the system bootstrap.")}
    </p>
  </div>

  {#if isComplete}
    <!-- Success Celebration View -->
    <div class="max-w-xl mx-auto text-center space-y-6 bg-card/80 border border-emerald-500/30 rounded-2xl p-8 shadow-lg shadow-emerald-500/5 backdrop-blur-sm">
      <div class="size-16 rounded-2xl bg-emerald-500/10 text-emerald-500 flex items-center justify-center mx-auto ring-8 ring-emerald-500/5 animate-bounce">
        <CheckCircle size={36} />
      </div>

      <div class="space-y-2">
        <h3 class="text-xl font-bold text-foreground">
          {isRTL ? "أهلاً بك في أوربيس!" : "Welcome to Orbis!"}
        </h3>
        <p class="text-sm text-muted-foreground max-w-md mx-auto">
          {isRTL
            ? `تم تأسيس ${form.organizationNameAr || form.organizationName} بنجاح، وتعيين ${form.adminEmail} كمشرف عام للنظام.`
            : `${form.organizationName} has been fully provisioned with ${form.adminEmail} as Primary Super Administrator.`}
        </p>
      </div>

      <div class="pt-4">
        <Button
          type="button"
          onclick={onEnterWorkspace}
          class="w-full sm:w-auto px-8 h-11 text-sm font-semibold shadow-md shadow-primary/20"
        >
          {isRTL ? "الدخول إلى لوحة التحكم الرئيسية" : "Enter Administrative Workspace"}
        </Button>
      </div>
    </div>
  {:else if isSubmitting}
    <!-- Live Execution State -->
    <div class="max-w-xl mx-auto space-y-6 bg-card/80 border border-border/80 rounded-2xl p-8 shadow-md backdrop-blur-sm">
      <div class="flex items-center justify-center gap-3 text-primary">
        <ArrowPath size={24} class="animate-spin" />
        <span class="text-base font-semibold">
          {isRTL ? "جاري تنفيذ التهيئة الآلية..." : "Executing System Bootstrap..."}
        </span>
      </div>

      <!-- Execution Task List -->
      <div class="space-y-3 pt-2">
        {#each tasks as task, idx}
          {@const isDone = idx < activeTaskIndex}
          {@const isCurrent = idx === activeTaskIndex}
          
          <div
            class="flex items-center gap-3 p-3 rounded-xl border text-xs transition-all {isCurrent
              ? 'border-primary bg-primary/5 text-foreground font-medium'
              : isDone
                ? 'border-emerald-500/30 bg-emerald-500/5 text-muted-foreground'
                : 'border-border/50 text-muted-foreground/60 opacity-60'}"
          >
            <div class="size-5 rounded-full flex items-center justify-center shrink-0">
              {#if isDone}
                <div class="text-emerald-500">
                  <CheckCircle size={18} />
                </div>
              {:else if isCurrent}
                <div class="size-4 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
              {:else}
                <div class="size-2 rounded-full bg-muted-foreground/30"></div>
              {/if}
            </div>
            <span>{task}</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <!-- Summary Review Cards -->
    <div class="max-w-3xl mx-auto space-y-4">
      {#if error}
        <div class="p-4 rounded-xl border border-destructive/30 bg-destructive/10 text-destructive flex items-start gap-3 text-xs">
          <InfoCircle size={18} class="shrink-0 mt-0.5" />
          <div class="space-y-1">
            <span class="font-semibold">{isRTL ? "فشلت عملية التهيئة:" : "Bootstrap Error:"}</span>
            <p>{error}</p>
          </div>
        </div>
      {/if}

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <!-- Blueprint Summary -->
        <div class="bg-card/60 border border-border/80 rounded-2xl p-5 space-y-3">
          <div class="flex items-center gap-2 text-primary pb-2 border-b border-border/60">
            <AcademicCap size={16} />
            <h3 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {isRTL ? "النموذج الأكاديمي" : "Blueprint"}
            </h3>
          </div>
          <div>
            <div class="font-semibold text-sm text-foreground">
              {isRTL ? blueprint.nameAr : blueprint.nameEn}
            </div>
            <span class="text-xs font-mono text-muted-foreground">{blueprint.type}</span>
          </div>
          <div class="text-xs text-muted-foreground line-clamp-3">
            {isRTL ? blueprint.descriptionAr : blueprint.descriptionEn}
          </div>
        </div>

        <!-- Org Summary -->
        <div class="bg-card/60 border border-border/80 rounded-2xl p-5 space-y-3">
          <div class="flex items-center gap-2 text-primary pb-2 border-b border-border/60">
            <BuildingOffice size={16} />
            <h3 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {isRTL ? "بيانات المنشأة" : "Organization"}
            </h3>
          </div>
          <div class="space-y-1">
            <div class="font-semibold text-sm text-foreground">
              {form.organizationName}
            </div>
            {#if form.organizationNameAr}
              <div class="text-xs text-muted-foreground font-arabic" dir="rtl">
                {form.organizationNameAr}
              </div>
            {/if}
            <div class="text-xs font-mono text-primary pt-1">
              /{form.organizationSlug}
            </div>
          </div>
          <div class="pt-2 border-t border-border/40 space-y-1 text-xs text-muted-foreground">
            <div class="flex justify-between">
              <span>{isRTL ? "العملة:" : "Currency:"}</span>
              <span class="font-medium text-foreground">{cur?.code} ({cur?.symbolAr || cur?.symbolEn})</span>
            </div>
            <div class="flex justify-between">
              <span>{isRTL ? "المنطقة:" : "Timezone:"}</span>
              <span class="font-medium text-foreground">{tz?.offset}</span>
            </div>
          </div>
        </div>

        <!-- Admin Summary -->
        <div class="bg-card/60 border border-border/80 rounded-2xl p-5 space-y-3">
          <div class="flex items-center gap-2 text-primary pb-2 border-b border-border/60">
            <UserBadgeCheck size={16} />
            <h3 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {isRTL ? "المشرف العام" : "Administrator"}
            </h3>
          </div>
          <div class="space-y-1">
            <div class="font-semibold text-sm text-foreground">
              {form.adminFirstName} {form.adminLastName}
            </div>
            {#if form.adminFirstNameAr || form.adminLastNameAr}
              <div class="text-xs text-muted-foreground font-arabic" dir="rtl">
                {form.adminFirstNameAr} {form.adminLastNameAr}
              </div>
            {/if}
            <div class="text-xs font-mono text-muted-foreground truncate pt-1" title={form.adminEmail}>
              {form.adminEmail}
            </div>
          </div>
          <div class="pt-2 border-t border-border/40 text-xs">
            <Badge variant="outline" class="bg-primary/10 text-primary border-primary/20 text-[10px]">
              {isRTL ? "صلاحيات المشرف العام الكاملة" : "Super Administrator"}
            </Badge>
          </div>
        </div>
      </div>

      <!-- Action Card -->
      <div class="bg-card/80 border border-primary/30 rounded-2xl p-6 text-center space-y-4 shadow-sm">
        <div class="space-y-1">
          <h4 class="font-bold text-base text-foreground">
            {isRTL ? "جاهز لبدء تشغيل النظام؟" : "Ready to Initialize Your Organization?"}
          </h4>
          <p class="text-xs text-muted-foreground">
            {isRTL
              ? "سيتم إنشاء قاعدة البيانات، إعداد جداول الصلاحيات، وحساب المشرف الأول مباشرة."
              : "This will execute database schema provisioning, system roles assignment, and initial administrator creation."}
          </p>
        </div>

        <Button
          type="button"
          onclick={onLaunch}
          class="w-full sm:w-auto px-10 h-11 text-sm font-semibold shadow-md shadow-primary/20"
        >
          <Sparkles size={16} class="me-2" />
          <span>{isRTL ? "بدء تهيئة النظام والتشغيل الآن" : "Initialize System & Launch"}</span>
        </Button>
      </div>
    </div>
  {/if}
</div>
