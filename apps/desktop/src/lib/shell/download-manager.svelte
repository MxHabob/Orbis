<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import { Download, ArrowPath } from "@orbis/icons";
import { updateManager } from "$lib/features/system/update-manager.svelte";

let isOpen = $state(false);
let isRetrying = $state(false);
let shouldShowDownloadManager = $derived(
  updateManager.isChecking ||
    updateManager.isDownloading ||
    updateManager.isApplying ||
    updateManager.isReadyToRestart ||
    updateManager.error !== null ||
    updateManager.updateResult?.has_update === true,
);

onMount(() => {
  void updateManager.initialize();
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
          {#if updateManager.progress?.total_bytes}
            <progress
              class="h-1.5 w-full overflow-hidden rounded-full accent-primary"
              max={updateManager.progress.total_bytes}
              value={Math.min(updateManager.progress.downloaded_bytes, updateManager.progress.total_bytes)}
              aria-label={m.ui_download_manager_title()}
            ></progress>
          {/if}
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
