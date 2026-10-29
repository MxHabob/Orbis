<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import type {
  AppInfo,
  ReleaseChangelogItem,
  UpdateCheckResult,
} from "$lib/api/types";
import { formatDate as formatOrgDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { LoadingIndicator, toast } from "@orbis/ui";
import { ArrowPath, Sparkles } from "@orbis/icons";
import ReleaseMarkdown from "$lib/features/system/release-markdown.svelte";
import EmptyState from "$lib/shared/components/empty-state.svelte";

let releases = $state<ReleaseChangelogItem[]>([]);
let appInfo = $state<AppInfo | null>(null);
let isLoading = $state(true);
let isCheckingUpdate = $state(false);
let isInstallingUpdate = $state(false);
let updateResult = $state<UpdateCheckResult | null>(null);

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
  isCheckingUpdate = true;
  updateResult = null;
  try {
    const res = await api.systemCheckForUpdates();
    updateResult = res;
    if (res.status === "unavailable") {
      toast.info(m.ui_changelog_updates_unavailable_staging());
    } else if (!res.has_update) {
      toast.info(
        m.shared_ui_latest_version_installed({ version: res.current_version }),
      );
    }
  } catch (error) {
    console.warn("Check for updates failed:", error);
    const msg = error instanceof Error ? error.message : String(error);
    toast.error(`${m.ui_changelog_update_failed()} ${msg}`);
  } finally {
    isCheckingUpdate = false;
  }
}

async function handleInstallUpdate() {
  isInstallingUpdate = true;
  try {
    await api.systemInstallUpdate();
  } catch (error) {
    console.error("Failed to install update:", error);
    const msg = error instanceof Error ? error.message : String(error);
    toast.error(`${m.ui_changelog_update_failed()} ${msg}`);
  } finally {
    isInstallingUpdate = false;
  }
}

function closeWindow() {
  window.close();
}

onMount(() => {
  void loadData();
});
</script>

<svelte:head>
  <title>{currentRelease?.title ? `${currentRelease.title} (v${currentRelease.version})` : m.ui_changelog_what_s_new_releases()}</title>
</svelte:head>

<div class="h-screen overflow-hidden bg-background font-sans text-foreground flex flex-col">
  <main class="min-h-0 flex-1 w-full overflow-y-auto scrollbar-hide">
    <div class="w-full max-w-2xl sm:max-w-3xl mx-auto p-5 sm:p-7 flex flex-col gap-5">
      <!-- Update Available Action Bar -->
      {#if updateResult?.has_update}
        <div class="flex items-center justify-between rounded-2xl border border-primary/20 bg-primary/10 text-primary p-3 text-xs">
          <span class="font-medium">{m.shared_ui_update_available({ version: updateResult.latest_version })}</span>
          <Button
            variant="default"
            size="sm"
            disabled={isInstallingUpdate}
            onclick={handleInstallUpdate}
            class="h-7 px-3 text-xs font-medium cursor-pointer"
          >
            {isInstallingUpdate ? m.ui_changelog_installing_update() : m.ui_changelog_install_and_restart()}
          </Button>
        </div>
      {/if}

      {#if isLoading}
        <div class="flex flex-col items-center justify-center py-28 gap-3 text-muted-foreground text-xs">
          <LoadingIndicator size="sm" />
          <span>{m.ui_changelog_loading_release_changelog()}</span>
        </div>
      {:else if currentRelease}
        <!-- Visual Banner (Dynamic - only if release provides images) -->
        {#if currentRelease.images && currentRelease.images.length > 0}
          <div class="relative w-full h-48 sm:h-56 overflow-hidden rounded-2xl border border-border/40 flex items-center justify-center bg-muted/20 shadow-xs">
            <img
              src={currentRelease.images[0]}
              alt={currentRelease.title}
              class="w-full h-full object-cover select-none"
            />
          </div>
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
                <img
                  src={img}
                  alt={currentRelease.title}
                  loading="lazy"
                  class="w-full max-h-60 rounded-xl border border-border/50 bg-muted/20 object-contain"
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
  </main>

  <!-- Fixed bottom action row -->
  <div class="shrink-0 w-full max-w-2xl sm:max-w-3xl mx-auto px-5 sm:px-7 pb-5 sm:pb-7 pt-4 border-t border-border/40 bg-background">
    <div class="flex items-center justify-between gap-2">
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
  </div>
</div>
