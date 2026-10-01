<!--
  @component StepMulverseIntro
  Step 1: Introduction & Sovereign Ecosystem Overview.
  Executive, responsive, and spacious design with zero visual mirroring artifacts.
  Uses real ReleaseChangelogItem as the authoritative source of truth.
  Hero image at the top, concise highlights grid, and external license link.
-->
<script lang="ts">
import { onMount } from "svelte";
import { marked } from "marked";
import { api } from "$lib/api/client";
import type {
  ReleaseChangelogItem,
  MulverseEcosystemInfo,
} from "$lib/api/types";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Check,
  Sparkles,
  ShieldCheck,
  Calendar,
} from "@orbis/icons";
import mulverseOrbis from "$lib/assets/mulverse-orbis.png";

let {
  isRTL = false,
}: {
  isRTL?: boolean;
} = $props();

const LICENSE_URL = "http://mulverse.com/a/orbis/license";

let releases = $state<ReleaseChangelogItem[]>([]);
let ecosystemInfo = $state<MulverseEcosystemInfo | null>(null);
let showFullNotes = $state(false);

onMount(async () => {
  try {
    const [chList, eco] = await Promise.all([
      api.systemGetChangelog(),
      api.systemGetEcosystemInfo(),
    ]);
    releases = chList;
    ecosystemInfo = eco;
  } catch (err) {
    console.error("Failed to load ecosystem changelog:", err);
  }
});

const activeRelease = $derived(
  releases.find((r) => r.is_latest) || releases[0] || null
);

const highlights = $derived.by(() => {
  if (!activeRelease) return [];
  return isRTL ? activeRelease.highlights_ar : activeRelease.highlights_en;
});

const renderedRawBody = $derived.by(() => {
  if (!activeRelease?.raw_body) return "";
  return marked.parse(activeRelease.raw_body) as string;
});

async function openLicense() {
  try {
    const { openUrl } = await import("@tauri-apps/plugin-opener");
    await openUrl(LICENSE_URL);
  } catch {
    window.open(LICENSE_URL, "_blank");
  }
}
</script>

<div class="space-y-6 w-full max-w-5xl mx-auto py-2">
  <!-- Top Main Card with Hero Media at the Top -->
  <div class="rounded-2xl border border-border/80 bg-card shadow-lg overflow-hidden relative select-none">
    <!-- 1. Hero Image Container at the Very Top -->
    <div class="w-full h-44 sm:h-52 md:h-60 lg:h-64 bg-muted/30 relative overflow-hidden flex items-center justify-center border-b border-border/50">
      <img
        src={mulverseOrbis}
        alt="Orbis Sovereign Platform"
        class="w-full h-full object-cover select-none"
        draggable="false"
      />
      <div class="absolute inset-0 bg-gradient-to-t from-card via-card/30 to-transparent pointer-events-none"></div>

      <!-- Floating Version Pill over Hero -->
      <div class="absolute bottom-3 inset-s-4 z-10 flex items-center gap-2">
        <Badge variant="secondary" class="text-[11px] font-mono px-2.5 py-0.5 bg-background/85 backdrop-blur-md border border-border/70 text-foreground shadow-xs">
          {activeRelease ? `v${activeRelease.version}` : "v0.3.0"}
        </Badge>
        <span class="text-[11px] font-mono text-muted-foreground bg-background/70 backdrop-blur-md px-2 py-0.5 rounded-md border border-border/50">
          {ecosystemInfo?.edition || "Enterprise Edition"}
        </span>
      </div>
    </div>

    <!-- 2. Content Body (Concise, High-End & Logical) -->
    <div class="p-6 sm:p-8 space-y-6 text-start">
      <!-- Title & Headline -->
      <div class="space-y-2">
        <div class="flex items-center justify-between flex-wrap gap-2">
          <span class="text-[10px] font-bold tracking-widest text-primary uppercase font-mono">
            {ecosystemInfo ? `${ecosystemInfo.brand.toUpperCase()} • ${ecosystemInfo.product_name.toUpperCase()}` : "MULVERSE • ORBIS"}
          </span>

          <!-- Official License External Link -->
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onclick={openLicense}
            class="h-7 px-2.5 text-[11px] font-mono gap-1.5 text-muted-foreground hover:text-foreground hover:bg-muted/50 rounded-full"
            title="Open official license agreement in browser"
          >
            <ShieldCheck size={13} class="text-primary" />
            <span>{isRTL ? "اتفاقية الترخيص" : "License Agreement"}</span>
          </Button>
        </div>

        <h1 class="text-2xl sm:text-3xl font-extrabold text-foreground tracking-tight">
          {activeRelease
            ? (isRTL ? activeRelease.title_ar : activeRelease.title_en)
            : (isRTL ? "منظومة أوربيس للتشغيل والسيادة المؤسسية" : "Orbis Sovereign Enterprise Platform")}
        </h1>

        <p class="text-xs sm:text-sm text-muted-foreground leading-relaxed max-w-3xl">
          {isRTL
            ? "نواة تشغيلية مستقلة مبنية محلياً (Local-First)، تمنح مؤسستك السيادة الرقمية الكاملة على السجلات الأكاديمية والمالية مع استقرار فائق وسرعة معالجة فورية."
            : "An autonomous local-first operating system engineered for educational institutions, ensuring uncompromised data ownership and instantaneous native execution."}
        </p>
      </div>

      <!-- 3. Real Release Highlights Grid (Authoritative Source: ReleaseChangelogItem) -->
      <div class="space-y-3">
        <div class="flex items-center justify-between border-b border-border/40 pb-2">
          <div class="flex items-center gap-2">
            <span class="text-xs font-bold text-foreground font-mono">
              {isRTL ? "المزايا والقدرات التشغيلية المعتمدة" : "Operational Capabilities"}
            </span>
            {#if activeRelease}
              <span class="text-[10px] font-mono text-muted-foreground flex items-center gap-1">
                <Calendar size={11} />
                <span>{activeRelease.release_date}</span>
              </span>
            {/if}
          </div>

          {#if activeRelease?.raw_body}
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onclick={() => (showFullNotes = !showFullNotes)}
              class="h-6 px-2 text-[10px] font-mono text-primary hover:bg-primary/5 rounded"
            >
              <Sparkles size={11} class="me-1" />
              <span>{showFullNotes ? (isRTL ? "عرض القائمة" : "Show List") : (isRTL ? "سجل التغييرات الكامل" : "Full Changelog")}</span>
            </Button>
          {/if}
        </div>

        {#if showFullNotes && activeRelease?.raw_body}
          <div class="rounded-xl border border-border/60 bg-muted/20 p-4 max-h-60 overflow-y-auto prose dark:prose-invert prose-xs text-xs text-muted-foreground leading-relaxed">
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html renderedRawBody}
          </div>
        {:else}
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            {#each highlights as item}
              <div class="p-3.5 rounded-xl border border-border/60 bg-muted/20 flex items-start gap-2.5 transition-colors hover:border-border">
                <div class="size-4 rounded-full border border-primary/30 bg-primary/10 text-primary flex items-center justify-center shrink-0 mt-0.5">
                  <Check size={10} strokeWidth={3} />
                </div>
                <span class="text-xs font-medium text-foreground/90 leading-snug">
                  {item}
                </span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
