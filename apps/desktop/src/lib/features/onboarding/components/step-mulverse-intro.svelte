<!--
  @component StepMulverseIntro
  Step 1: Introduction to Orbis & the Mulverse Enterprise Ecosystem.
  Purely informative / architectural showcase featuring authentic telemetry,
  local sovereignty pillars, and real-time Tauri environment detection HUD.
-->
<script lang="ts">
import type { EnvironmentDetectionResult } from "$lib/api/types";
import { Card, CardContent } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import mulverseOrbis from "$lib/assets/mulverse-orbis.png";

let {
  envData = null,
  isLoadingEnv = false,
  isRTL = false,
}: {
  envData: EnvironmentDetectionResult | null;
  isLoadingEnv?: boolean;
  isRTL?: boolean;
} = $props();
</script>

<div class="space-y-6 max-w-3xl mx-auto py-1">
  <!-- Brand Showcase Card with mulverse-orbis image -->
  <div class="overflow-hidden rounded-2xl border border-border/70 bg-card shadow-sm">
    <div class="w-full aspect-[16/6] bg-muted/20 border-b border-border/40 overflow-hidden flex items-center justify-center">
      <img
        src={mulverseOrbis}
        alt="Mulverse Orbis"
        class="w-full h-full object-cover"
        draggable="false"
      />
    </div>

    <div class="p-6 space-y-4 text-start">
      <div class="space-y-1">
        <div class="flex items-center gap-2">
          <span class="text-[11px] font-bold text-primary uppercase tracking-widest font-mono">MULVERSE ECOSYSTEM</span>
          <Badge variant="outline" class="text-[10px] font-mono px-1.5 py-0 h-4 border-primary/40 text-primary bg-primary/5">
            v0.3.0
          </Badge>
        </div>
        <h1 class="text-2xl font-bold text-foreground tracking-tight">
          {isRTL ? "منظومة أوربيس للتشغيل المؤسسي الذاتي" : "Orbis Autonomous Operating System"}
        </h1>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {isRTL
            ? "نظام تشغيل المؤسسات التعليمية والأكاديمية، المطور كعقدة ذاتية مستقلة (Autonomous Node) تمنحك السيادة الكاملة على بياناتك مع أداء فائق السرعة عبر محرك Rust وقاعدة بيانات محلية متقدمة."
            : "The sovereign enterprise institutional operating system, engineered as an autonomous local-first node within the Mulverse ecosystem with native speed and zero cloud lock-in."}
        </p>
      </div>

      <!-- Live Environment Detection HUD -->
      <div class="rounded-xl border border-border/60 bg-muted/30 p-4 space-y-2.5">
        <div class="flex items-center justify-between">
          <span class="text-xs font-semibold text-foreground font-mono">
            {isRTL ? "البيئة والمنطقة المكتشفة تلقائياً" : "Auto-Detected Regional Context"}
          </span>

          {#if isLoadingEnv}
            <span class="text-[11px] text-muted-foreground animate-pulse font-mono">
              {isRTL ? "جاري الفحص..." : "Detecting..."}
            </span>
          {:else}
            <Badge variant="secondary" class="text-[10px] font-mono px-2 py-0.5 text-emerald-600 dark:text-emerald-400 bg-emerald-500/10">
              {isRTL ? "تم الضبط الذكي" : "Config Applied"}
            </Badge>
          {/if}
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
          <!-- Country & Region -->
          <div class="p-2.5 rounded-lg bg-background border border-border/50 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block font-mono">
              {isRTL ? "الدولة والمنطقة" : "Country / Region"}
            </span>
            <span class="font-medium text-foreground truncate block">
              {envData ? `${envData.flag_emoji} ${isRTL ? envData.country_name_ar : envData.country_name_en}` : "🇸🇦 Saudi Arabia"}
            </span>
          </div>

          <!-- Currency -->
          <div class="p-2.5 rounded-lg bg-background border border-border/50 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block font-mono">
              {isRTL ? "العملة الأساسية" : "Base Currency"}
            </span>
            <span class="font-medium text-foreground font-mono truncate block">
              {envData ? `${envData.currency_code} (${envData.currency_symbol_ar})` : "SAR (ر.س)"}
            </span>
          </div>

          <!-- Timezone -->
          <div class="p-2.5 rounded-lg bg-background border border-border/50 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block font-mono">
              {isRTL ? "المنطقة الزمنية" : "Timezone"}
            </span>
            <span class="font-medium text-foreground font-mono truncate block">
              {envData ? `${envData.timezone.split("/")[1] || envData.timezone} (${envData.timezone_offset})` : "Riyadh (UTC+3)"}
            </span>
          </div>

          <!-- Engine -->
          <div class="p-2.5 rounded-lg bg-background border border-border/50 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block font-mono">
              {isRTL ? "محرك البيانات" : "Data Core"}
            </span>
            <span class="font-medium text-foreground font-mono text-[11px] truncate block">
              SQLite 3 Local
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 4 Mulverse Architectural Pillars Grid -->
  <div class="space-y-3">
    <div class="px-1">
      <h2 class="text-xs font-semibold tracking-tight text-foreground uppercase font-mono">
        {isRTL ? "ركائز منظومة أوربيس ومزايا السيادة المؤسسية" : "Architectural Foundations"}
      </h2>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <!-- Pillar 1 -->
      <Card class="border-border/60 bg-card shadow-xs">
        <CardContent class="p-4 space-y-1 text-start">
          <div class="font-semibold text-xs text-foreground">
            {isRTL ? "السيادة الرقمية والاستقلالية الكاملة" : "Data Sovereignty & Local First"}
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {isRTL
              ? "سجلات المؤسسة، الحسابات المالية، ودرجات الطلاب مشفرة ومخزنة محلياً بالكامل على جهازك دون أي تبعية سحابية إلزامية."
              : "All institutional records, financial ledgers, and student files are stored locally and encrypted on your machine with zero cloud lock-in."}
          </p>
        </CardContent>
      </Card>

      <!-- Pillar 2 -->
      <Card class="border-border/60 bg-card shadow-xs">
        <CardContent class="p-4 space-y-1 text-start">
          <div class="font-semibold text-xs text-foreground">
            {isRTL ? "نواة Rust فائقة السرعة" : "Zero-Latency Rust Engine"}
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {isRTL
              ? "معالجة فورية فائقة السرعة للدرجات المعقدة والمحاسبة المزدوجة ومصفوفات الحصص الزمنية بأعلى كفاءة."
              : "Instant native computations for multi-criteria grading, double-entry financial journals, and schedule solver."}
          </p>
        </CardContent>
      </Card>

      <!-- Pillar 3 -->
      <Card class="border-border/60 bg-card shadow-xs">
        <CardContent class="p-4 space-y-1 text-start">
          <div class="font-semibold text-xs text-foreground">
            {isRTL ? "محرك DocForge للنشر البصري" : "DocForge Visual Publishing"}
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {isRTL
              ? "تصميم وطباعة الشهادات والبطاقات والتقارير المعتمدة بدقة بصرية متناهية ورموز QR مشفرة للتحقق."
              : "High-precision layout engine for official certificates, transcripts, diplomas, and ID cards with instant generation."}
          </p>
        </CardContent>
      </Card>

      <!-- Pillar 4 -->
      <Card class="border-border/60 bg-card shadow-xs">
        <CardContent class="p-4 space-y-1 text-start">
          <div class="font-semibold text-xs text-foreground">
            {isRTL ? "محرك التشغيل الذكي Autopilot" : "Autopilot Smart Operations"}
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {isRTL
              ? "تتبع الجاهزية التشغيلية وإغلاق الفترات التقويمية والتنبيهات الاستباقية للعمليات المدرسية آلياً."
              : "Continuous background orchestration of enrollment cycles, grading windows, and proactive academic workflows."}
          </p>
        </CardContent>
      </Card>
    </div>
  </div>
</div>
