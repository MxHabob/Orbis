<!--
  @component AboutDialog
  Comprehensive "About Orbis — Mulverse System" modal displaying authentic versioning,
  Mulverse ecosystem architecture, hardware & OS runtime telemetry, and diagnostic copy capability.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import type { MulverseEcosystemInfo } from "$lib/api/types";
import { getLocale } from "$lib/paraglide/runtime";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";

let {
  open = $bindable(false),
}: {
  open?: boolean;
} = $props();

let info = $state<MulverseEcosystemInfo | null>(null);
let isLoading = $state(true);
let isCopied = $state(false);

const isRTL = $derived(getLocale().startsWith("ar"));

async function loadEcosystemInfo() {
  isLoading = true;
  try {
    const data = await api.systemGetEcosystemInfo();
    info = data;
  } catch (err) {
    console.error("Failed to load ecosystem info:", err);
  } finally {
    isLoading = false;
  }
}

async function copyDiagnostics() {
  if (!info) return;
  const diagnosticsText = JSON.stringify(
    {
      product: info.product_name,
      ecosystem: info.ecosystem_name,
      brand: info.brand,
      version: info.version,
      build_profile: info.build_profile,
      target_os: info.target_os,
      target_arch: info.target_arch,
      node_id: info.node_id,
      node_type: info.node_type,
      database_engine: info.database_engine,
      github_repo: info.github_repo,
      timestamp: new Date().toISOString(),
      userAgent: typeof navigator !== "undefined" ? navigator.userAgent : "Tauri WebView",
    },
    null,
    2
  );

  try {
    await navigator.clipboard.writeText(diagnosticsText);
    isCopied = true;
    setTimeout(() => {
      isCopied = false;
    }, 2500);
  } catch (err) {
    console.error("Failed to copy diagnostics:", err);
  }
}

onMount(() => {
  loadEcosystemInfo();
});

$effect(() => {
  if (open && !info) {
    loadEcosystemInfo();
  }
});
</script>

<Dialog bind:open>
  <DialogContent class="max-w-2xl max-h-[85vh] p-0 gap-0 overflow-hidden flex flex-col bg-background/95 backdrop-blur-md border-border/60 shadow-2xl">
    <!-- Header with Mulverse Brand Glow -->
    <DialogHeader class="px-6 py-5 border-b border-border/40 bg-gradient-to-b from-primary/10 via-primary/5 to-transparent text-start space-y-2">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="size-9 rounded-xl bg-primary/15 border border-primary/30 flex items-center justify-center text-primary shadow-xs">
          </div>
          <div>
            <DialogTitle class="text-lg font-bold text-foreground tracking-tight flex items-center gap-2">
              <span>{info?.product_name || "Orbis Next-Gen SIS / ERP"}</span>
              <Badge variant="outline" class="text-[10px] font-mono px-1.5 py-0 h-4 border-primary/40 text-primary bg-primary/10">
                v{info?.version || "0.3.0"}
              </Badge>
            </DialogTitle>
            <p class="text-xs text-primary font-medium tracking-wide">
              {info?.brand || "Mulverse Systems"} • {info?.ecosystem_name || "Mulverse"}
            </p>
          </div>
        </div>

        <Badge variant="secondary" class="text-[10px] font-mono uppercase px-2 py-0.5">
          {info?.edition || "Autonomous Desktop"}
        </Badge>
      </div>

      <DialogDescription class="text-xs text-muted-foreground leading-relaxed pt-1">
        {isRTL
          ? (info?.description_ar || m.ui_system_about_default_description())
          : (info?.description_en || m.ui_system_about_default_description())}
      </DialogDescription>
    </DialogHeader>

    <!-- Scrollable Body with Real System Telemetry -->
    <div class="flex-1 overflow-y-auto p-6 space-y-5">
      <!-- Technical Telemetry Matrix -->
      <div class="space-y-2">
        <div class="flex items-center justify-between">
          <span class="text-xs font-semibold uppercase tracking-wider text-foreground font-mono flex items-center gap-1.5">
            <span>{m.ui_system_about_build_runtime_telemetry()}</span>
          </span>
          <span class="text-[10px] text-muted-foreground font-mono">
            Node: {info?.node_id || "node-local"}
          </span>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-3 gap-2.5">
          <div class="p-3 rounded-lg border border-border/50 bg-muted/20 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block uppercase font-mono">{m.ui_system_about_version()}</span>
            <span class="text-xs font-semibold text-foreground font-mono">v{info?.version || "0.3.0"}</span>
          </div>

          <div class="p-3 rounded-lg border border-border/50 bg-muted/20 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block uppercase font-mono">{m.ui_system_about_build_profile()}</span>
            <span class="text-xs font-semibold text-foreground">{info?.build_profile || "Development"}</span>
          </div>

          <div class="p-3 rounded-lg border border-border/50 bg-muted/20 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block uppercase font-mono">{m.ui_system_about_target_os_arch()}</span>
            <span class="text-xs font-semibold text-foreground font-mono uppercase">{info?.target_os} / {info?.target_arch}</span>
          </div>

          <div class="p-3 rounded-lg border border-border/50 bg-muted/20 space-y-0.5 sm:col-span-2">
            <span class="text-[10px] text-muted-foreground block uppercase font-mono">{m.ui_system_about_database_core()}</span>
            <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
              <span>{info?.database_engine || "Embedded SQLite 3"}</span>
            </span>
          </div>

          <div class="p-3 rounded-lg border border-border/50 bg-muted/20 space-y-0.5">
            <span class="text-[10px] text-muted-foreground block uppercase font-mono">{m.ui_system_about_node_role()}</span>
            <span class="text-xs font-semibold text-foreground font-mono">{info?.node_type || "Autonomous Node"}</span>
          </div>
        </div>
      </div>

      <!-- Mulverse Architectural Pillars -->
      <div class="space-y-2.5 pt-2">
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground font-mono flex items-center gap-1.5">
          <span>{m.ui_system_about_mulverse_architectural_pillars()}</span>
        </span>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
          {#if info?.pillars}
            {#each info.pillars as pillar}
              <div class="p-3.5 rounded-lg border border-border/60 bg-card/60 space-y-1.5 hover:border-primary/40 transition-colors">
                <div class="flex items-center gap-2 text-foreground font-semibold text-xs">
                  <span>{isRTL ? pillar.title_ar : pillar.title_en}</span>
                </div>
                <p class="text-[11px] text-muted-foreground leading-relaxed">
                  {isRTL ? pillar.description_ar : pillar.description_en}
                </p>
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <!-- Legal & Company Links -->
      <div class="p-3.5 rounded-lg border border-border/40 bg-muted/10 flex items-center justify-between text-xs">
        <div class="space-y-0.5">
          <span class="font-medium text-foreground">© 2026 Mulverse Systems. All Rights Reserved.</span>
          <p class="text-[11px] text-muted-foreground">
            {m.ui_system_about_licensed_for_sovereign_autonomous_institution_deployment()}
          </p>
        </div>

        <Button
          variant="outline"
          size="sm"
          onclick={() => window.open(`https://github.com/${info?.github_repo || "MxHabob/Orbis"}`, "_blank")}
          class="h-8 text-xs font-normal gap-1.5 px-3"
        >
          <span>GitHub</span>
        </Button>
      </div>
    </div>

    <!-- Footer Bar with Copy Diagnostics & Close Actions -->
    <div class="px-6 py-3 border-t border-border/40 bg-muted/20 flex items-center justify-between">
      <Button
        variant="outline"
        size="sm"
        onclick={copyDiagnostics}
        class="h-8 text-xs font-normal gap-1.5 px-3 border-border/60"
      >
        {#if isCopied}
          <span class="text-emerald-600 dark:text-emerald-400 font-medium">
            {m.ui_system_about_copied_diagnostics()}
          </span>
        {:else}
          <span>{m.ui_system_about_copy_diagnostics()}</span>
        {/if}
      </Button>

      <Button
        variant="default"
        size="sm"
        onclick={() => (open = false)}
        class="h-8 px-5 text-xs font-medium"
      >
        <span>{m.ui_system_about_close()}</span>
      </Button>
    </div>
  </DialogContent>
</Dialog>
