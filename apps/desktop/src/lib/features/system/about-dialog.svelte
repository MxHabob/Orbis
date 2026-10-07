<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { api } from "$lib/api/client";
  import type { AppInfo } from "$lib/api/types";
  import {
    Dialog,
    DialogContent,
    DialogTitle,
    DialogDescription,
  } from "@orbis/ui/dialog";
  import { Button } from "@orbis/ui/button";
  import { Check, Clipboard, GlobeAlt } from "@orbis/icons";
  import mulverseOrbisImg from "$lib/assets/mulverse-orbis.png";

  let { open = $bindable(false) }: { open?: boolean } = $props();

  let info = $state<AppInfo | null>(null);
  let isLoading = $state(false);
  let isCopied = $state(false);

  const OFFICIAL_WEBSITE = "https://www.mulverse.com/";

  async function loadAppInfo() {
    isLoading = true;
    try {
      info = await api.systemGetAppInfo();
    } catch (error) {
      console.error("Failed to load application information:", error);
    } finally {
      isLoading = false;
    }
  }

  async function copyDiagnostics() {
    if (!info) return;
    const diagnostics = JSON.stringify(
      {
        application: info.name,
        company: "Mulverse Inc.",
        headquarters: "Sana'a, Al-Jaraf, Yemen",
        website: OFFICIAL_WEBSITE,
        version: info.version,
        build_profile: info.build_profile,
        target_os: info.target_os,
        target_arch: info.target_arch,
        timestamp: new Date().toISOString(),
      },
      null,
      2,
    );

    try {
      await navigator.clipboard.writeText(diagnostics);
      isCopied = true;
      setTimeout(() => (isCopied = false), 2500);
    } catch (error) {
      console.error("Failed to copy application diagnostics:", error);
    }
  }

  function openWebsite() {
    window.open(OFFICIAL_WEBSITE, "_blank", "noopener,noreferrer");
  }

  $effect(() => {
    if (open && !info && !isLoading) void loadAppInfo();
  });
</script>

<Dialog bind:open>
  <DialogContent class="w-full max-w-lg sm:max-w-[560px] p-5 gap-0 rounded-3xl border border-border/60 bg-card/98 shadow-2xl backdrop-blur-xl overflow-hidden font-sans">
    <DialogTitle class="sr-only">About {info?.name ?? "Orbis"}</DialogTitle>
    <DialogDescription class="sr-only">Orbis and Mulverse Information</DialogDescription>

    <!-- Top Visual Showcase Banner (Exact match with reference layout) -->
    <div class="relative w-full h-52 sm:h-56 overflow-hidden rounded-2xl border border-border/40 bg-muted/40 shadow-xs flex items-center justify-center">
      <img
        src={mulverseOrbisImg}
        alt="Mulverse Orbis"
        class="w-full h-full object-cover"
      />
    </div>

    <!-- Bottom Content Area -->
    <div class="pt-4 px-1 pb-1 flex flex-col gap-1.5 text-start">
      <!-- Eyebrow text -->
      <span class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground/80">
        {m.ui_system_about_company_name()}
      </span>

      <!-- Bold Main Title -->
      <div class="flex items-center gap-2">
        <h2 class="text-xl sm:text-2xl font-black tracking-tight text-foreground capitalize">
          {info?.name ?? "Orbis"}
        </h2>
        {#if info}
          <span class="text-[11px] font-mono font-semibold px-2 py-0.5 rounded-full bg-primary/10 text-primary border border-primary/20">
            v{info.version}
          </span>
        {/if}
      </div>

      <!-- Clean Description Body -->
      <p class="text-xs leading-relaxed text-muted-foreground max-w-lg pt-0.5">
        {m.ui_system_about_company_desc()}
      </p>

      <!-- Minimal System Info Row (Dynamic - No hardcoded or fabricated metrics) -->
      {#if info}
        <div class="flex flex-wrap items-center gap-x-2 gap-y-0.5 text-[10.5px] text-muted-foreground/70 font-mono pt-1">
          <span class="capitalize">{info.build_profile}</span>
          <span>•</span>
          <span>{info.target_os}{info.target_arch ? `/${info.target_arch}` : ""}</span>
          <span>•</span>
          <span class="font-sans">{m.ui_system_about_hq_location()}</span>
        </div>
      {/if}

      <!-- Bottom Action Buttons (Matching reference image: secondary text + primary accent button) -->
      <div class="flex items-center justify-between gap-2 pt-3 mt-1">
        <button
          type="button"
          onclick={openWebsite}
          class="inline-flex items-center gap-1.5 text-xs font-semibold text-muted-foreground hover:text-foreground transition-colors cursor-pointer select-none"
        >
          <GlobeAlt size={13} />
          <span>{m.ui_system_about_website()}</span>
        </button>

        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={copyDiagnostics}
            class="inline-flex items-center gap-1.5 text-xs font-semibold text-muted-foreground hover:text-foreground px-2.5 py-1.5 rounded-xl transition-colors cursor-pointer select-none"
          >
            {#if isCopied}
              <Check size={13} class="text-emerald-500" />
              <span>{m.ui_system_about_copied_diagnostics()}</span>
            {:else}
              <Clipboard size={13} />
              <span>{m.ui_system_about_copy_diagnostics()}</span>
            {/if}
          </button>

          <Button
            variant="default"
            size="sm"
            onclick={() => (open = false)}
            class="text-xs h-9 px-5 rounded-xl font-semibold cursor-pointer shadow-xs bg-primary text-primary-foreground hover:bg-primary/90"
          >
            {m.ui_system_about_close()}
          </Button>
        </div>
      </div>
    </div>
  </DialogContent>
</Dialog>
