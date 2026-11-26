<script lang="ts">
import SmoothImage from "$lib/shared/components/smooth-image.svelte";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import { isDesktopApp } from "$lib/api/runtime";
import { extractErrorMessage } from "$lib/api";
import type { AppInfo, ReleaseChangelogItem } from "$lib/api/types";
import { formatDate as formatOrgDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { LoadingIndicator, toast } from "@orbis/ui";
import { ArrowPath, Sparkles } from "@orbis/icons";
import ReleaseMarkdown from "$lib/features/system/release-markdown.svelte";
import EmptyState from "$lib/shared/components/empty-state.svelte";
import { updateManager } from "$lib/features/system/update-manager.svelte";

let releases = $state<ReleaseChangelogItem[]>([]);
let appInfo = $state<AppInfo | null>(null);
let isLoading = $state(true);
let isCheckingUpdate = $derived(updateManager.isChecking);
let isDownloadingUpdate = $derived(updateManager.isDownloading);
let isReadyToRestart = $derived(updateManager.isReadyToRestart);
let updateResult = $derived(updateManager.updateResult);
let updateProgress = $derived(updateManager.progress);

// Focus strictly on the current installed release
const currentRelease = $derived(
  releases.find((r) => r.is_current) ??
    releases.find((r) => r.version === appInfo?.version) ??
    releases[0] ??
    null,
);

function formatDate(value?: string) {
  if (!value) return null;
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? null : formatOrgDate(date, "date");
}

async function loadData() {
  isLoading = true;
  try {
    const [releaseList, info] = await Promise.all([
      api.systemGetChangelog().catch((err) => {
        console.warn("Changelog fetch failed, treating as empty:", err);
        return [] as ReleaseChangelogItem[];
      }),
      api.systemGetAppInfo().catch((err) => {
        console.warn("App info fetch failed:", err);
        return null;
      }),
    ]);
    releases = releaseList;
    appInfo = info;
  } catch (error) {
    console.warn("Failed to load changelog data:", error);
    releases = [];
  } finally {
    isLoading = false;
  }
}

async function handleCheckForUpdates() {
  try {
    const res = await updateManager.checkForUpdates();
    if (!res.has_update) {
      toast.info(
        m.shared_ui_latest_version_installed({ version: res.current_version }),
      );
    }
  } catch (error) {
    console.warn("Check for updates failed:", error);
    const msg = error instanceof Error ? error.message : String(error);
    toast.error(`${m.ui_changelog_update_failed()} ${msg}`);
  }
}

async function handleDownloadUpdate() {
  try {
    await updateManager.downloadUpdate();
  } catch (error) {
    console.error("Failed to download update:", error);
    const msg = error instanceof Error ? error.message : String(error);
    toast.error(`${m.ui_download_manager_update_failed()} ${msg}`);
  }
}

async function handleInstallUpdate() {
  try {
    await updateManager.installUpdate();
  } catch (error) {
    console.error("Failed to install update:", error);
    const msg = error instanceof Error ? error.message : String(error);
    toast.error(`${m.ui_changelog_update_failed()} ${msg}`);
  }
}

async function closeWindow() {
  if (!isDesktopApp) {
    window.close();
    return;
  }

  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().close();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

onMount(() => {
  void loadData();
});
</script>

<svelte:head>
  <title>{currentRelease?.title ? `${currentRelease.title} (v${currentRelease.version})` : m.ui_changelog_what_s_new_releases()}</title>
</svelte:head>

<div class="h-dvh w-dvw min-h-0 min-w-0 overflow-hidden bg-background font-sans text-foreground flex flex-col">
  <main class="min-h-0 flex-1 w-full max-w-2xl sm:max-w-3xl mx-auto overflow-y-auto scrollbar-hide p-5 sm:p-7 flex flex-col gap-5 justify-between">
    <div class="flex flex-col gap-5">
      <!-- Update Available Action Bar -->
      {#if updateResult?.has_update}
        <div class="flex items-center justify-between rounded-2xl border border-primary/20 bg-primary/10 text-primary p-3 text-xs">
          <span class="font-medium">{m.shared_ui_update_available({ version: updateResult.latest_version })}</span>
          <Button
            variant="default"
            size="sm"
            disabled={isDownloadingUpdate || updateManager.isApplying}
            onclick={isReadyToRestart ? handleInstallUpdate : handleDownloadUpdate}
            class="h-7 px-3 text-xs font-medium cursor-pointer"
          >
            {#if isDownloadingUpdate}
              {m.ui_download_manager_downloading_update()}
            {:else if isReadyToRestart}
              {m.ui_download_manager_restart_to_update()}
            {:else}
              {m.ui_download_manager_download_update()}
            {/if}
          </Button>
        </div>
        {#if isDownloadingUpdate}
          <div class="space-y-2 rounded-xl border border-border/50 bg-card/60 p-3 text-xs" aria-live="polite">
            <div class="flex items-center justify-between gap-3">
              <span>{m.ui_download_manager_downloading_update()}</span>
              {#if updateProgress?.total_bytes}
                <span class="font-mono text-muted-foreground">
                  {Math.min(100, Math.round((updateProgress.downloaded_bytes / updateProgress.total_bytes) * 100))}%
                </span>
              {:else if updateProgress}
                <span class="font-mono text-muted-foreground">
                  {(updateProgress.downloaded_bytes / (1024 * 1024)).toFixed(1)} MB
                </span>
              {/if}
            </div>
            {#if updateProgress?.total_bytes}
              <progress
                class="h-1.5 w-full overflow-hidden rounded-full accent-primary"
                max={updateProgress.total_bytes}
                value={Math.min(updateProgress.downloaded_bytes, updateProgress.total_bytes)}
                aria-label="Update download progress"
              ></progress>
            {/if}
          </div>
        {/if}
        {#if isReadyToRestart}
          <p class="text-xs text-muted-foreground" aria-live="polite">
            {m.ui_download_manager_update_ready()}
          </p>
        {/if}
      {/if}

      {#if isLoading}
        <div class="flex flex-col items-center justify-center py-28 gap-3 text-muted-foreground text-xs">
          <LoadingIndicator size="sm" />
          <span>{m.ui_changelog_loading_release_changelog()}</span>
        </div>
      {:else if currentRelease}
        <!-- Visual Banner (Dynamic - only if release provides images) -->
        {#if currentRelease.images && currentRelease.images.length > 0}
          <SmoothImage
            src={currentRelease.images[0]}
            alt={currentRelease.title}
            loading="eager"
            containerClass="w-full h-48 sm:h-56 rounded-2xl border border-border/40 bg-muted/20 shadow-xs"
            class="w-full h-full object-cover select-none"
          />
        {/if}

        <!-- Current Release Header & Typography -->
        <div class="flex flex-col gap-1.5 text-start">
          {#if appInfo?.name}
            <span class="text-[11px] font-semibold tracking-wider text-muted-foreground uppercase">
              {appInfo.name}
            </span>
          {/if}

          <div class="flex flex-wrap items-center gap-2">
            <h1 class="text-xl sm:text-2xl font-black tracking-tight capitalize">
              {currentRelease.title}
            </h1>
            {#if currentRelease.version}
              <span class="text-[11px] font-mono font-semibold px-2 py-0.5 rounded-full border">
                v{currentRelease.version}
              </span>
            {/if}
            {#if currentRelease.is_latest}
              <span class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-primary/10 text-primary border border-primary/20">
                {m.ui_changelog_latest()}
              </span>
            {/if}
          </div>

          <!-- Fully Dynamic Metadata Row -->
          <div class="flex flex-wrap items-center gap-x-2 gap-y-0.5 text-[10.5px] font-mono text-muted-foreground pt-1">
            {#if appInfo?.build_profile}
              <span class="capitalize">{appInfo.build_profile}</span>
            {/if}
            {#if appInfo?.target_os}
              {#if appInfo?.build_profile}
                <span>•</span>
              {/if}
              <span>{appInfo.target_os}{appInfo.target_arch ? `/${appInfo.target_arch}` : ""}</span>
            {/if}
            {#if currentRelease.published_at}
              {@const pubDate = formatDate(currentRelease.published_at)}
              {#if pubDate}
                {#if appInfo?.target_os || appInfo?.build_profile}
                  <span>•</span>
                {/if}
                <span>{pubDate}</span>
              {/if}
            {/if}
          </div>
        </div>

        <!-- Release Notes Body -->
        {#if currentRelease.body?.trim()}
          <section class="space-y-2 pt-2">
            <h2 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {m.ui_changelog_release_notes()}
            </h2>
            <div class="rounded-2xl border border-border/50 bg-card/60 p-4 sm:p-6 shadow-xs">
              <ReleaseMarkdown markdown={currentRelease.body} />
            </div>
          </section>
        {/if}

        <!-- Additional Images (Dynamic - only if release provides multiple images) -->
        {#if currentRelease.images && currentRelease.images.length > 1}
          <section class="space-y-2 pt-2">
            <h2 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {m.ui_changelog_release_images()}
            </h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {#each currentRelease.images.slice(1) as img (img)}
                <SmoothImage
                  src={img}
                  alt={currentRelease.title}
                  containerClass="w-full h-56 rounded-xl border border-border/50 bg-muted/20"
                  class="w-full h-full object-contain"
                />
              {/each}
            </div>
          </section>
        {/if}

        <!-- Release Resources (Dynamic - only if release provides resources) -->
        {#if currentRelease.resources && currentRelease.resources.length > 0}
          <section class="space-y-2 pt-2">
            <h2 class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {m.ui_changelog_release_resources()}
            </h2>
            <div class="flex flex-col gap-2">
              {#each currentRelease.resources as res (res.url)}
                <a
                  href={res.url}
                  target="_blank"
                  rel="noreferrer"
                  class="flex items-center justify-between p-3 rounded-xl border border-border/50 bg-card/60 hover:bg-muted/40 transition-colors text-xs"
                >
                  <span class="font-medium text-foreground">{res.name}</span>
                  <span class="font-mono text-muted-foreground">{(res.size / 1024).toFixed(0)} KB</span>
                </a>
              {/each}
            </div>
          </section>
        {/if}
      {:else}
        <!-- System Empty State when no releases are available -->
        <EmptyState
          icon={Sparkles}
          title={m.ui_changelog_no_releases()}
          description={m.ui_changelog_no_release_notes()}
          actionLabel={m.ui_changelog_check_for_updates()}
          onaction={handleCheckForUpdates}
          class="py-20"
        />
      {/if}
    </div>

    <!-- Bottom Action Row (Always present) -->
    <div class="flex items-center justify-between gap-2 pt-4 mt-2 border-t border-border/40">
      <div>
        {#if currentRelease?.html_url}
          <Button
            variant="ghost"
            size="sm"
            onclick={() => window.open(currentRelease.html_url, "_blank")}
            class="inline-flex items-center gap-1.5 text-xs font-semibold cursor-pointer select-none text-muted-foreground hover:text-foreground"
          >
            <Sparkles size={13} />
            <span>{m.ui_changelog_github_release()}</span>
          </Button>
        {/if}
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          disabled={isCheckingUpdate}
          onclick={handleCheckForUpdates}
          class="h-8 gap-1.5 px-3 text-xs font-semibold cursor-pointer select-none"
        >
          {#if isCheckingUpdate}
            <LoadingIndicator size={13} />
          {:else}
            <ArrowPath size={13} />
          {/if}
          <span>{isCheckingUpdate ? m.ui_changelog_checking() : m.ui_changelog_check_for_updates()}</span>
        </Button>

        <Button
          variant="default"
          size="sm"
          onclick={closeWindow}
          class="text-xs h-8 px-5 font-semibold cursor-pointer"
        >
          {m.ui_system_about_close()}
        </Button>
      </div>
    </div>
  </main>
</div>
