<!--
  @component StepReview
  Step 5: Pre-Flight Verification & System Bootstrap Launch.
  Comprehensive overview of all parameters before executing 0-to-1 initialization.
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  CURRENCY_OPTIONS,
  TIMEZONE_OPTIONS,
  COUNTRIES_LIST,
  EDUCATIONAL_TRACK_OPTIONS,
  ORGANIZATION_SIZE_OPTIONS,
} from "../types";
import { getBlueprint } from "@orbis/blueprints";
import { Button } from "@orbis/ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import {
  Tag,
  ShieldCheck,
  Check,
} from "@orbis/icons";

let {
  form,
  isSubmitting = false,
  isComplete = false,
  error = null,
  activeTaskIndex = 0,
  onLaunch,
  onEnterWorkspace,
  isRTL = false,
}: {
  form: OnboardingFormModel;
  isSubmitting?: boolean;
  isComplete?: boolean;
  error?: string | null;
  activeTaskIndex?: number;
  onLaunch: () => void;
  onEnterWorkspace: () => void;
  isRTL?: boolean;
} = $props();

const blueprint = $derived(getBlueprint(form.institutionType));
const tz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const cur = $derived(CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode));
const country = $derived(COUNTRIES_LIST.find((c) => c.code === form.countryCode));
const track = $derived(EDUCATIONAL_TRACK_OPTIONS.find((t) => t.value === form.educationalTrack));
const size = $derived(ORGANIZATION_SIZE_OPTIONS.find((s) => s.value === form.organizationSize));

const executionTasksEn = [
  "Creating organization profile and metadata",
  "Configuring institution blueprint & academic rules",
  "Provisioning Primary Administrator credentials",
  "Initializing Chart of Accounts & currency rates",
  "Verifying autonomous node operational readiness",
];

const executionTasksAr = [
  "إنشاء سجل المؤسسة وبيانات الهوية",
  "تهيئة نموذج المؤسسة الأكاديمي واللوائح",
  "إنشاء حساب المشرف العام وتوثيق الصلاحيات",
  "تهيئة دليل الحسابات المالي والعملة الأساسية",
  "التحقق من الجاهزية التشغيلية للنواة المستقلة",
];

const tasks = $derived(isRTL ? executionTasksAr : executionTasksEn);
</script>

<div class="space-y-6 max-w-3xl mx-auto py-1">
  <div class="space-y-1 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {isRTL ? "المرحلة ٥ من ٥" : "Step 5 of 5"}
      </span>
    </div>
    <h2 class="text-xl font-bold tracking-tight text-foreground">
      {isComplete
        ? (isRTL ? "تمت تهيئة النظام بنجاح" : "System Initialized Successfully")
        : isSubmitting
          ? (isRTL ? "جاري تهيئة النظام وتأسيس العقدة..." : "Initializing Autonomous Node...")
          : (isRTL ? "مراجعة الإعدادات وبدء التهيئة" : "Review & Launch Setup")}
    </h2>
    <p class="text-xs text-muted-foreground">
      {isComplete
        ? (isRTL ? "تم تأسيس المؤسسة بنجاح، وجميع الوحدات الأساسية جاهزة للعمل." : "Your workspace has been provisioned and is ready for production.")
        : isSubmitting
          ? (isRTL ? "يرجى الانتظار ريثما تكتمل التهيئة الأساسية للنظام المحلي." : "Please wait while Orbis provisions initial records and configurations.")
          : (isRTL ? "تحقق من صحة البيانات أدناه قبل المتابعة لإنشاء المنشأة." : "Review your settings below before initiating autonomous bootstrap.")}
    </p>
  </div>

  {#if isComplete}
    <Card class="border-border/60 shadow-lg text-center p-8 space-y-5 bg-card">
      <div class="size-14 rounded-2xl bg-emerald-500/10 text-emerald-500 flex items-center justify-center mx-auto border border-emerald-500/20 shadow-md">
        <Check size={28} strokeWidth={2.5} />
      </div>

      <div class="space-y-2">
        <h3 class="text-lg font-bold text-foreground">
          {isRTL ? "أهلاً بك في أوربيس • Mulverse OS" : "Welcome to Orbis • Mulverse OS"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-lg mx-auto leading-relaxed">
          {isRTL
            ? `تم تأسيس ${form.organizationNameAr || form.organizationName} بنجاح كعقدة ذاتية مستقلة، وتعيين ${form.adminEmail} كمشرف عام للنظام.`
            : `${form.organizationName} has been initialized as an autonomous sovereign node with ${form.adminEmail} as Primary Administrator.`}
        </p>
      </div>

      <div class="pt-2">
        <Button
          type="button"
          variant="default"
          size="default"
          onclick={onEnterWorkspace}
          class="h-10 px-8 text-xs font-semibold gap-2 shadow-md shadow-primary/20"
        >
          <span>{isRTL ? "الدخول إلى مساحة العمل" : "Enter Workspace"}</span>
        </Button>
      </div>
    </Card>
  {:else if isSubmitting}
    <Card class="border-border/60 shadow-md p-6 space-y-4 bg-card/95">
      <div class="flex items-center gap-2.5 pb-2 border-b border-border/40">
        <div class="size-2 rounded-full bg-primary animate-ping"></div>
        <span class="text-xs font-semibold text-foreground font-mono">
          {isRTL ? "مصفوفة مهام التهيئة المباشرة" : "Live Initialization Execution Pipeline"}
        </span>
      </div>

      <div class="space-y-2.5">
        {#each tasks as task, idx}
          {@const isDone = idx < activeTaskIndex}
          {@const isCurrent = idx === activeTaskIndex}

          <div
            class="flex items-center gap-3 text-xs py-1.5 px-3 rounded-lg transition-colors {isCurrent
              ? 'bg-primary/10 text-foreground font-medium border border-primary/20'
              : isDone
                ? 'text-muted-foreground bg-muted/20'
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
        <div class="p-4 rounded-xl border border-destructive/30 bg-destructive/10 text-destructive text-xs space-y-1">
          <div class="font-bold flex items-center gap-1.5">
            <span>{isRTL ? "فشلت عملية التهيئة:" : "Bootstrap Error:"}</span>
          </div>
          <p class="leading-relaxed">{error}</p>
        </div>
      {/if}

      <!-- 3 Columns Overview Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3.5 text-start">
        <!-- Blueprint -->
        <Card class="border-border/60 shadow-xs bg-card/95">
          <CardHeader class="p-4 pb-2 space-y-1">
            <span class="text-[10px] text-muted-foreground uppercase font-mono flex items-center gap-1">
              <Tag size={11} />
              <span>{isRTL ? "النموذج الأكاديمي" : "Academic Blueprint"}</span>
            </span>
            <CardTitle class="text-sm font-semibold text-foreground">
              {isRTL ? blueprint.nameAr : blueprint.nameEn}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground space-y-1">
            <div class="font-medium text-foreground">{isRTL ? track?.labelAr : track?.labelEn}</div>
            <div class="text-[11px] text-muted-foreground">{isRTL ? size?.labelAr : size?.labelEn}</div>
          </CardContent>
        </Card>

        <!-- Org Profile -->
        <Card class="border-border/60 shadow-xs bg-card/95">
          <CardHeader class="p-4 pb-2 space-y-1">
            <span class="text-[10px] text-muted-foreground uppercase font-mono flex items-center gap-1">
              <span>{isRTL ? "بيانات المنشأة" : "Organization"}</span>
            </span>
            <CardTitle class="text-sm font-semibold text-foreground truncate">
              {form.organizationNameAr || form.organizationName}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground space-y-1">
            <div class="text-foreground truncate">{form.organizationName}</div>
            <div class="text-[11px] font-mono flex items-center gap-1.5">
              <span>{country?.flag} {country?.code}</span>
              <span>•</span>
              <span>{cur?.code}</span>
              <span>•</span>
              <span>{tz?.offset}</span>
            </div>
          </CardContent>
        </Card>

        <!-- Admin -->
        <Card class="border-border/60 shadow-xs bg-card/95">
          <CardHeader class="p-4 pb-2 space-y-1">
            <span class="text-[10px] text-muted-foreground uppercase font-mono flex items-center gap-1">
              <span>{isRTL ? "المشرف العام" : "Administrator"}</span>
            </span>
            <CardTitle class="text-sm font-semibold text-foreground truncate">
              {form.adminFirstName} {form.adminLastName}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 pt-1 text-xs text-muted-foreground space-y-1">
            <div class="truncate font-mono text-foreground">{form.adminEmail}</div>
            <div class="text-[11px] font-mono text-muted-foreground">{form.adminPhone || (isRTL ? "بدون هاتف" : "No phone")}</div>
          </CardContent>
        </Card>
      </div>

      <!-- Action Card -->
      <Card class="border-border/60 shadow-sm p-5 text-center space-y-3 bg-gradient-to-b from-card to-muted/20">
        <div class="flex items-center justify-center gap-1.5 text-xs text-muted-foreground font-mono">
          <ShieldCheck size={14} class="text-emerald-500" />
          <span>Mulverse Autonomous Core • Sovereign Local Database</span>
        </div>

        <p class="text-xs text-muted-foreground max-w-md mx-auto leading-relaxed">
          {isRTL
            ? "سيتم تأسيس قاعدة البيانات المحلية، إنشاء الحساب الإداري، وتهيئة دليل الحسابات المالي الأولي فور الضغط على زر التهيئة."
            : "Clicking initialize will provision local tables, primary administrator credentials, and foundational ledger."}
        </p>

        <Button
          type="button"
          variant="default"
          size="default"
          onclick={onLaunch}
          class="h-10 px-8 text-xs font-semibold gap-2 shadow-md shadow-primary/20"
        >
          <span>{isRTL ? "بدء تهيئة النظام" : "Initialize System"}</span>
        </Button>
      </Card>
    </div>
  {/if}
</div>
