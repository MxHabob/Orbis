<!--
  @component StepMulverseIntro
  Step 1: Introduction to Orbis & the Mulverse Enterprise Ecosystem.
  Spacious, elegant showcase with authentic OS telemetry HUD and core sovereignty pillars.
-->
<script lang="ts">
import type { EnvironmentDetectionResult } from "$lib/api/types";
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

<div class="space-y-8 max-w-5xl mx-auto py-2">
  <!-- Brand Showcase Hero Banner -->
  <div class="overflow-hidden rounded-2xl border border-border/70 bg-card shadow-lg relative group">
    <!-- Visual Image Container with Subtle Ambient Glow -->
    <div class="w-full h-56 sm:h-72 md:h-80 bg-gradient-to-b from-muted/30 to-card/60 relative overflow-hidden flex items-center justify-center border-b border-border/40">
      <img
        src={mulverseOrbis}
        alt="Mulverse Orbis Platform"
        class="w-full h-full object-cover select-none"
        draggable="false"
      />
      <div class="absolute inset-0 bg-gradient-to-t from-card via-transparent to-transparent opacity-80 pointer-events-none"></div>
    </div>

    <div class="p-6 sm:p-8 space-y-6 text-start">
      <div class="space-y-2">
        <div class="flex items-center gap-2.5">
          <span class="text-xs font-bold text-primary tracking-widest font-mono uppercase">
            MULVERSE ECOSYSTEM
          </span>
          <span class="text-muted-foreground/40">•</span>
          <Badge variant="outline" class="text-[11px] font-mono px-2 py-0.5 border-primary/30 text-primary bg-primary/5">
            v0.3.0
          </Badge>
        </div>

        <h1 class="text-2xl sm:text-3xl font-bold text-foreground tracking-tight">
          {isRTL ? "منظومة أوربيس للتشغيل المؤسسي الذاتي" : "Orbis Sovereign Enterprise Platform"}
        </h1>

        <p class="text-sm text-muted-foreground leading-relaxed max-w-3xl">
          {isRTL
            ? "بيئة عمل متكاملة للمؤسسات التعليمية والأكاديمية، تمنحك السيادة المطلقة على بياناتك وسجلاتها مع سرعة معالجة استثنائية عبر محرك محلي فائق الكفاءة وبنية تحتية مستقلة."
            : "An autonomous operating platform engineered for educational institutions and modern academies, delivering uncompromising data sovereignty and native performance with zero cloud lock-in."}
        </p>
      </div>

      <!-- Live Environment Detection HUD -->
      <div class="rounded-xl border border-border/70 bg-muted/25 p-5 space-y-3.5">
        <div class="flex items-center justify-between border-b border-border/40 pb-2.5">
          <div class="flex items-center gap-2">
            <span class="size-2 rounded-full bg-emerald-500 animate-pulse"></span>
            <span class="text-xs font-semibold text-foreground font-mono">
              {isRTL ? "البيئة والمنطقة المكتشفة تلقائياً" : "Auto-Detected Environment & Region"}
            </span>
          </div>

          {#if isLoadingEnv}
            <span class="text-xs text-muted-foreground animate-pulse font-mono">
              {isRTL ? "جاري الفحص المباشر..." : "Calibrating..."}
            </span>
          {:else}
            <Badge variant="secondary" class="text-[11px] font-mono px-2.5 py-0.5 text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 border border-emerald-500/20">
              {isRTL ? "تم الضبط الذكي" : "System Calibrated"}
            </Badge>
          {/if}
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 text-xs">
          <!-- Country & Region -->
          <div class="p-3 rounded-lg bg-background border border-border/60 space-y-1">
            <span class="text-[11px] text-muted-foreground block font-mono">
              {isRTL ? "الدولة والمنطقة" : "Country / Region"}
            </span>
            <span class="font-semibold text-foreground text-sm truncate block">
              {envData ? `${envData.flag_emoji} ${isRTL ? envData.country_name_ar : envData.country_name_en}` : "🇸🇦 Saudi Arabia"}
            </span>
          </div>

          <!-- Display Language -->
          <div class="p-3 rounded-lg bg-background border border-border/60 space-y-1">
            <span class="text-[11px] text-muted-foreground block font-mono">
              {isRTL ? "لغة النظام المكتشفة" : "Detected Language"}
            </span>
            <span class="font-semibold text-foreground text-sm truncate block">
              {envData ? envData.detected_language : "English (United States)"}
            </span>
          </div>

          <!-- Currency -->
          <div class="p-3 rounded-lg bg-background border border-border/60 space-y-1">
            <span class="text-[11px] text-muted-foreground block font-mono">
              {isRTL ? "العملة الأساسية" : "Base Currency"}
            </span>
            <span class="font-semibold text-foreground font-mono text-sm truncate block">
              {envData ? `${envData.currency_code} (${isRTL ? envData.currency_symbol_ar : envData.currency_symbol_en})` : "SAR (ر.س)"}
            </span>
          </div>

          <!-- Engine Core -->
          <div class="p-3 rounded-lg bg-background border border-border/60 space-y-1">
            <span class="text-[11px] text-muted-foreground block font-mono">
              {isRTL ? "محرك البيانات" : "Execution Core"}
            </span>
            <span class="font-semibold text-foreground font-mono text-xs truncate block">
              SQLite 3 • Rust Native
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 4 Mulverse Architectural Foundations -->
  <div class="space-y-4">
    <div class="px-1 text-start">
      <h2 class="text-xs font-bold tracking-wider text-muted-foreground uppercase font-mono">
        {isRTL ? "ركائز منظومة أوربيس ومزايا السيادة المؤسسية" : "Enterprise Sovereignty Pillars"}
      </h2>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <!-- Pillar 1 -->
      <div class="p-5 rounded-xl border border-border/60 bg-card shadow-xs space-y-2 text-start transition-colors hover:border-border">
        <div class="font-bold text-sm text-foreground">
          {isRTL ? "السيادة الرقمية والاستقلالية الكاملة" : "Data Sovereignty & Local First"}
        </div>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {isRTL
            ? "سجلات المؤسسة، الحسابات المالية، ودرجات الطلاب مشفرة ومخزنة محلياً بالكامل على جهازك دون أي تبعية سحابية إلزامية."
            : "All institutional records, financial ledgers, and student files are stored locally and encrypted on your machine with zero cloud lock-in."}
        </p>
      </div>

      <!-- Pillar 2 -->
      <div class="p-5 rounded-xl border border-border/60 bg-card shadow-xs space-y-2 text-start transition-colors hover:border-border">
        <div class="font-bold text-sm text-foreground">
          {isRTL ? "محرك Rust فائق السرعة" : "High-Velocity Rust Engine"}
        </div>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {isRTL
            ? "معالجة فورية فائقة السرعة للدرجات المعقدة والمحاسبة المزدوجة ومصفوفات الحصص الزمنية بأعلى كفاءة واستقرار."
            : "Instant native computations for multi-criteria grading, double-entry financial journals, and schedule solver."}
        </p>
      </div>

      <!-- Pillar 3 -->
      <div class="p-5 rounded-xl border border-border/60 bg-card shadow-xs space-y-2 text-start transition-colors hover:border-border">
        <div class="font-bold text-sm text-foreground">
          {isRTL ? "محرك DocForge للنشر البصري والشهادات" : "DocForge Dynamic Publishing"}
        </div>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {isRTL
            ? "توليد الشهادات والبطاقات والتقارير المعتمدة بدقة طباعية متناهية مع دعم التوقيع الرقمي والرموز المشفرة."
            : "Render official certificates, transcripts, ID cards, and financial statements with native vector fidelity."}
        </p>
      </div>

      <!-- Pillar 4 -->
      <div class="p-5 rounded-xl border border-border/60 bg-card shadow-xs space-y-2 text-start transition-colors hover:border-border">
        <div class="font-bold text-sm text-foreground">
          {isRTL ? "الأتمتة المؤسسية والذكاء الاصطناعي" : "Autonomous Workflows & AI"}
        </div>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {isRTL
            ? "مساعدون أذكياء ومحرك Autopilot لإنجاز المهام الدورية، مطابقة الرسوم، وإدارة العمليات تلقائياً."
            : "Built-in Autopilot engine and contextual assistants to automate recurring administrative workflows securely."}
        </p>
      </div>
    </div>
  </div>
</div>
