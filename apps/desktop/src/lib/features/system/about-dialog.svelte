<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { api } from "$lib/api/client";
  import type { AppInfo } from "$lib/api/types";
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
  } from "@orbis/ui/dialog";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { LoadingIndicator } from "@orbis/ui";

  let { open = $bindable(false) }: { open?: boolean } = $props();

  let info = $state<AppInfo | null>(null);
  let isLoading = $state(false);
  let loadError = $state<string | null>(null);
  let isCopied = $state(false);

  async function loadAppInfo() {
    isLoading = true;
    loadError = null;
    try {
      info = await api.systemGetAppInfo();
    } catch (error) {
      console.error("Failed to load application information:", error);
      loadError = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }

  async function copyDiagnostics() {
    if (!info) return;
    const diagnostics = JSON.stringify(
      {
        application: info.name,
        version: info.version,
        build_profile: info.build_profile,
        target_os: info.target_os,
        target_arch: info.target_arch,
        timestamp: new Date().toISOString(),
        user_agent: typeof navigator !== "undefined" ? navigator.userAgent : undefined,
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

  $effect(() => {
    if (open && !info && !isLoading) void loadAppInfo();
  });
</script>

<Dialog bind:open>
  <DialogContent class="flex max-h-[85vh] max-w-2xl flex-col gap-0 overflow-hidden border-border/60 bg-background/95 p-0 shadow-2xl backdrop-blur-md">
    <DialogHeader class="space-y-2 border-b border-border/40 bg-gradient-to-b from-primary/10 via-primary/5 to-transparent px-6 py-5 text-start">
      <div class="flex items-center gap-2.5">
        <div class="flex size-9 items-center justify-center rounded-xl border border-primary/30 bg-primary/15 text-primary">
          <span class="text-xs font-bold">A</span>
        </div>
        <div class="min-w-0">
          <DialogTitle class="flex flex-wrap items-center gap-2 text-lg font-bold tracking-tight text-foreground">
            {#if info}
              <span>{info.name}</span>
              <Badge variant="outline" class="h-4 border-primary/40 bg-primary/10 px-1.5 font-mono text-[10px] text-primary">
                v{info.version}
              </Badge>
            {:else}
              <span>{m.ui_system_about_build_runtime_telemetry()}</span>
            {/if}
          </DialogTitle>
        </div>
      </div>
      <DialogDescription class="pt-1 text-xs leading-relaxed text-muted-foreground">
        {m.ui_system_about_description()}
      </DialogDescription>
    </DialogHeader>

    <div class="flex-1 space-y-4 overflow-y-auto p-6">
      {#if isLoading}
        <p class="flex items-center gap-2 text-sm text-muted-foreground">
          <LoadingIndicator size="sm" label={m.ui_system_about_loading()} />
          <span aria-hidden="true">{m.ui_system_about_loading()}</span>
        </p>
      {:else if loadError}
        <p class="text-sm text-destructive" role="alert">{m.ui_system_about_load_failed()} {loadError}</p>
      {:else if info}
        <div class="space-y-2">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-foreground">
            {m.ui_system_about_build_runtime_telemetry()}
          </h3>
          <div class="grid grid-cols-1 gap-2.5 sm:grid-cols-2">
            <div class="space-y-0.5 rounded-lg border border-border/50 bg-muted/20 p-3">
              <span class="block font-mono text-[10px] uppercase text-muted-foreground">{m.ui_system_about_version()}</span>
              <span class="font-mono text-xs font-semibold text-foreground">v{info.version}</span>
            </div>
            <div class="space-y-0.5 rounded-lg border border-border/50 bg-muted/20 p-3">
              <span class="block font-mono text-[10px] uppercase text-muted-foreground">{m.ui_system_about_build_profile()}</span>
              <span class="text-xs font-semibold text-foreground">{info.build_profile}</span>
            </div>
            <div class="space-y-0.5 rounded-lg border border-border/50 bg-muted/20 p-3 sm:col-span-2">
              <span class="block font-mono text-[10px] uppercase text-muted-foreground">{m.ui_system_about_target_os_arch()}</span>
              <span class="font-mono text-xs font-semibold uppercase text-foreground">
                {info.target_os}{info.target_arch ? ` / ${info.target_arch}` : ""}
              </span>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <div class="flex items-center justify-between border-t border-border/40 bg-muted/20 px-6 py-3">
      <Button variant="outline" size="sm" disabled={!info} onclick={copyDiagnostics} class="h-8 px-3 text-xs font-normal">
        {isCopied ? m.ui_system_about_copied_diagnostics() : m.ui_system_about_copy_diagnostics()}
      </Button>
      <Button variant="default" size="sm" onclick={() => (open = false)} class="h-8 px-5 text-xs font-medium">
        {m.ui_system_about_close()}
      </Button>
    </div>
  </DialogContent>
</Dialog>
