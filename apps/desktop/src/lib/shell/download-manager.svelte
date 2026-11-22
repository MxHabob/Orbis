<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import { Progress } from "@orbis/ui/progress";
import { Download, ArrowPath } from "@orbis/icons";
import { api } from "$lib/api/client";
import { isDesktopApp } from "$lib/api/runtime";
import type { SolverSetupStatusDto } from "$lib/api/types";
import { updateManager } from "$lib/features/system/update-manager.svelte";

let isOpen = $state(false);
let isRetrying = $state(false);
let solverSetup = $state<SolverSetupStatusDto | null>(null);
let unlistenSetup: UnlistenFn | undefined;
let shouldShowDownloadManager = $derived(
  updateManager.isChecking ||
    updateManager.isDownloading ||
    updateManager.isApplying ||
    updateManager.isReadyToRestart ||
    updateManager.updateResult?.has_update === true ||
    solverSetup?.state === "preparing" ||
    solverSetup?.state === "ready",
);

onMount(() => {
  void updateManager.initialize();
  if (!isDesktopApp) return;

  let disposed = false;
  const listenerPromise = listen<SolverSetupStatusDto>(
    "solver-setup-status-changed",
    ({ payload }) => {
      if (payload.state === "failed") {
        solverSetup = null;
        isOpen = false;
      } else {
        solverSetup = payload;
      }
      if (payload.state === "preparing") {
        isOpen = true;
      } else if (payload.state === "ready") {
        isOpen = true;
        window.setTimeout(() => {
          if (solverSetup?.state === "ready") solverSetup = null;
        }, 6000);
      }
    },
  );
  void listenerPromise.then(async (unlisten) => {
    if (disposed) {
      unlisten();
      return;
    }
    unlistenSetup = unlisten;
    try {
      const current = await api.getSolverSetupStatus();
      if (current.state === "preparing") {
        solverSetup = current;
      } else if (current.state === "failed") {
        solverSetup = null;
      } else if (current.state === "ready") {
        solverSetup = current;
      }
    } catch (error) {
      console.error("Failed to load solver download status", error);
    }
  });

  return () => {
    disposed = true;
    unlistenSetup?.();
  };
});

async function retry() {
  isRetrying = true;
  try {
    await updateManager.retry();
  } catch (error) {
    console.warn("Update manager retry failed:", error);
  } finally {
    isRetrying = false;
  }
}

async function retrySolverSetup() {
  try {
    solverSetup = await api.startSolverSetup();
  } catch (error) {
    console.error("Failed to restart solver setup", error);
  }
}
</script>

{#if shouldShowDownloadManager}
  <Popover bind:open={isOpen}>
    <PopoverTrigger>
      <Button
        variant="outline"
        size="icon-sm"
        class="relative shrink-0"
        title={m.ui_download_manager_title()}
        aria-label={m.ui_download_manager_title()}
        aria-busy={updateManager.isChecking || updateManager.isDownloading || updateManager.isApplying}
      >
        {#if updateManager.isChecking}
          <ArrowPath size={14} class="animate-spin text-muted-foreground" />
        {:else if updateManager.isDownloading || updateManager.isApplying}
          <ArrowPath size={14} class="animate-spin text-primary" />
        {:else}
          <Download size={14} class={updateManager.error ? "text-destructive" : "text-muted-foreground"} />
        {/if}
        {#if updateManager.updateResult?.has_update || updateManager.error || updateManager.isReadyToRestart}
          <span
            class="absolute -top-0.5 -inset-e-0.5 size-2 rounded-full bg-primary ring-2 ring-card"
            class:bg-destructive={updateManager.error !== null}
            aria-hidden="true"
          ></span>
        {/if}
      </Button>
    </PopoverTrigger>

    <PopoverContent
      side="bottom"
      align="end"
      sideOffset={8}
      class="w-[min(360px,calc(100vw-2rem))] space-y-4 rounded-xl p-4 text-start"
    >
      <div class="flex items-center justify-between gap-3">
        <h2 class="text-sm font-semibold">{m.ui_download_manager_title()}</h2>
        <Button
          variant="ghost"
          size="icon-xs"
          title={m.ui_changelog_check_for_updates()}
          aria-label={m.ui_changelog_check_for_updates()}
          disabled={updateManager.isChecking || updateManager.isDownloading || updateManager.isApplying}
          onclick={() => void updateManager.checkForUpdates().catch((error) => console.warn("Update check failed:", error))}
        >
          <ArrowPath size={14} class={updateManager.isChecking ? "animate-spin" : ""} />
        </Button>
      </div>

      {#if solverSetup?.state === "preparing"}
        <div class="space-y-2" role="status" aria-live="polite">
          <p class="text-xs text-muted-foreground">{m.ai_ui_solver_download_in_progress()}</p>
          <Progress value={0} class="download-progress-indeterminate h-1.5 w-full" />
        </div>
      {:else if solverSetup?.state === "failed"}
        <div class="space-y-3">
          <div>
            <p class="text-xs font-medium text-destructive">{m.ai_ui_solver_download_failed()}</p>
            {#if solverSetup.error}
              <p class="mt-1 break-words text-xs text-muted-foreground" role="alert">{solverSetup.error}</p>
            {/if}
          </div>
          <Button
            variant="outline"
            size="sm"
            class="w-full"
            disabled={isRetrying}
            onclick={() => {
              isRetrying = true;
              void retrySolverSetup().finally(() => (isRetrying = false));
            }}
          >
            {isRetrying ? m.ui_changelog_checking() : m.common_retry()}
          </Button>
        </div>
      {:else if solverSetup?.state === "ready"}
        <div role="status" aria-live="polite">
          <p class="text-xs text-muted-foreground">{m.ai_ui_solver_download_ready()}</p>
        </div>
      {/if}

      {#if updateManager.isChecking}
        <p class="text-xs text-muted-foreground" aria-live="polite">
          {m.ui_changelog_checking()}
        </p>
      {:else if updateManager.isDownloading}
        <div class="space-y-2" aria-live="polite">
          <div class="flex items-center justify-between gap-3 text-xs">
            <span>{m.ui_download_manager_downloading_update()}</span>
            {#if updateManager.progress?.total_bytes}
              <span class="font-mono text-muted-foreground">
                {Math.min(100, Math.round((updateManager.progress.downloaded_bytes / updateManager.progress.total_bytes) * 100))}%
              </span>
            {:else if updateManager.progress}
              <span class="font-mono text-muted-foreground">
                {(updateManager.progress.downloaded_bytes / (1024 * 1024)).toFixed(1)} MB
              </span>
            {/if}
          </div>
          <Progress
            value={updateManager.progress?.total_bytes
              ? Math.min(updateManager.progress.downloaded_bytes, updateManager.progress.total_bytes)
              : 0}
            max={updateManager.progress?.total_bytes ?? 100}
            class={updateManager.progress?.total_bytes
              ? "h-1.5 w-full"
              : "download-progress-indeterminate h-1.5 w-full"}
            aria-label={m.ui_download_manager_title()}
          />
        </div>
      {:else if updateManager.error}
        <div class="space-y-3">
          <p class="text-xs text-destructive" role="alert">
            {m.ui_download_manager_update_failed()} {updateManager.error}
          </p>
          <Button
            variant="outline"
            size="sm"
            class="w-full"
            disabled={isRetrying}
            onclick={() => void retry()}
          >
            {isRetrying ? m.ui_changelog_checking() : m.common_retry()}
          </Button>
        </div>
      {:else if updateManager.updateResult?.has_update}
        <div class="space-y-3">
          <p class="text-xs text-foreground">
            {m.shared_ui_update_available({ version: updateManager.updateResult.latest_version })}
          </p>
          {#if updateManager.isReadyToRestart}
            <p class="text-xs text-muted-foreground" aria-live="polite">
              {m.ui_download_manager_update_ready()}
            </p>
            <Button
              variant="default"
              size="sm"
              class="w-full"
              onclick={() => void updateManager.installUpdate().catch((error) => console.warn("Update installation failed:", error))}
            >
              {m.ui_download_manager_restart_to_update()}
            </Button>
          {:else}
            <Button
              variant="default"
              size="sm"
              class="w-full"
              disabled={updateManager.isDownloading}
              onclick={() => void updateManager.downloadUpdate().catch((error) => console.warn("Update download failed:", error))}
            >
              {m.ui_download_manager_download_update()}
            </Button>
          {/if}
        </div>
      {:else if updateManager.updateResult}
        <p class="text-xs text-muted-foreground">
          {m.shared_ui_latest_version_installed({ version: updateManager.updateResult.current_version })}
        </p>
      {:else}
        <p class="text-xs text-muted-foreground">
          {m.ui_changelog_check_for_updates()}
        </p>
      {/if}
    </PopoverContent>
  </Popover>
{/if}

<style>
  :global(.download-progress-indeterminate [data-slot="progress-indicator"]) {
    width: 35%;
    flex: none;
    animation: download-progress-slide 1.4s ease-in-out infinite;
  }

  @keyframes download-progress-slide {
    from {
      transform: translateX(-100%);
    }

    to {
      transform: translateX(300%);
    }
  }
</style>
