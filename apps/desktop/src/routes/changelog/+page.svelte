<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { onMount } from "svelte";
  import { api } from "$lib/api/client";
  import type { AppInfo, ReleaseChangelogItem, UpdateCheckResult } from "$lib/api/types";
  import { getLocale } from "$lib/paraglide/runtime";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { LoadingIndicator } from "@orbis/ui";
  import { ArrowPath, Calendar, Check, Cube } from "@orbis/icons";
  import ReleaseMarkdown from "$lib/features/system/release-markdown.svelte";

  let releases = $state<ReleaseChangelogItem[]>([]);
  let appInfo = $state<AppInfo | null>(null);
  let selectedVersion = $state("");
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let isCheckingUpdate = $state(false);
  let isInstallingUpdate = $state(false);
  let updateResult = $state<UpdateCheckResult | null>(null);
  let updateError = $state<string | null>(null);

  const isRTL = $derived(getLocale().startsWith("ar"));
  const activeRelease = $derived(
    releases.find((release) => release.version === selectedVersion) ?? releases[0] ?? null,
  );

  function formatDate(value?: string) {
    if (!value) return null;
    const date = new Date(value);
    return Number.isNaN(date.getTime())
      ? null
      : date.toLocaleDateString(isRTL ? "ar" : "en", { dateStyle: "medium" });
  }

  async function loadData() {
    isLoading = true;
    loadError = null;
    try {
      const [releaseList, info] = await Promise.all([
        api.systemGetChangelog(),
        api.systemGetAppInfo(),
      ]);
      releases = releaseList;
      appInfo = info;
      selectedVersion = releaseList[0]?.version ?? "";
    } catch (error) {
      console.error("Failed to load changelog data:", error);
      loadError = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }

  async function handleCheckForUpdates() {
    isCheckingUpdate = true;
    updateError = null;
    updateResult = null;
    try {
      updateResult = await api.systemCheckForUpdates();
    } catch (error) {
      console.error("Failed to check for updates:", error);
      updateError = error instanceof Error ? error.message : String(error);
    } finally {
      isCheckingUpdate = false;
    }
  }

  async function handleInstallUpdate() {
    isInstallingUpdate = true;
    updateError = null;
    try {
      await api.systemInstallUpdate();
    } catch (error) {
      console.error("Failed to install update:", error);
      updateError = error instanceof Error ? error.message : String(error);
    } finally {
      isInstallingUpdate = false;
    }
  }

  onMount(() => {
    void loadData();
  });
</script>

<svelte:head>
  <title>{appInfo ? `${appInfo.name} — ${m.ui_changelog_what_s_new_releases()}` : m.ui_changelog_what_s_new_releases()}</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-background font-sans text-foreground">
  <header class="sticky top-0 z-30 flex h-14 items-center justify-between border-b border-border/50 bg-background/95 px-6 backdrop-blur-md">
    <div class="flex min-w-0 items-center gap-3">
      <div class="flex size-7 shrink-0 items-center justify-center rounded-lg border border-primary/25 bg-primary/10 text-primary">
        <Cube size={16} />
      </div>
      <div class="flex min-w-0 items-center gap-2">
        {#if appInfo}
          <span class="truncate text-sm font-bold tracking-tight">{appInfo.name}</span>
        {/if}
        <span class="text-xs text-muted-foreground">•</span>
        <span class="truncate text-xs font-medium text-muted-foreground">
          {m.ui_changelog_what_s_new_releases()}
        </span>
      </div>
      {#if appInfo}
        <Badge variant="outline" class="h-4 border-primary/40 bg-primary/5 px-1.5 font-mono text-[10px] text-primary">
          v{appInfo.version}
        </Badge>
      {/if}
    </div>

    <Button
      variant="outline"
      size="sm"
      disabled={isCheckingUpdate}
      onclick={handleCheckForUpdates}
      class="h-8 gap-1.5 border-border/60 px-3 text-xs font-normal"
    >
      {#if isCheckingUpdate}
        <LoadingIndicator size={13} />
      {:else}
        <ArrowPath size={13} />
      {/if}
      <span>{isCheckingUpdate ? m.ui_changelog_checking() : m.ui_changelog_check_for_updates()}</span>
    </Button>
  </header>

  {#if updateError}
    <div class="border-b border-destructive/20 bg-destructive/10 px-6 py-2.5 text-xs text-destructive" role="alert">
      {m.ui_changelog_update_failed()} {updateError}
    </div>
  {/if}

  {#if updateResult}
    <div class="flex items-center justify-between border-b px-6 py-2.5 text-xs {updateResult.has_update
      ? 'border-primary/20 bg-primary/10 text-primary'
      : 'border-emerald-500/20 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'}">
      <div class="flex items-center gap-2">
        {#if updateResult.has_update}
          <span class="font-medium">{m.shared_ui_update_available({ version: updateResult.latest_version })}</span>
        {:else}
          <Check size={14} />
          <span>{m.shared_ui_latest_version_installed({ version: updateResult.current_version })}</span>
        {/if}
      </div>
      {#if updateResult.has_update}
        <Button
          variant="default"
          size="sm"
          disabled={isInstallingUpdate}
          onclick={handleInstallUpdate}
          class="h-8 px-3 text-xs font-medium"
        >
          {isInstallingUpdate ? m.ui_changelog_installing_update() : m.ui_changelog_install_and_restart()}
        </Button>
      {:else if updateResult.release_url}
        <a href={updateResult.release_url} target="_blank" rel="noreferrer" class="underline">
          {m.ui_changelog_download_from_github()}
        </a>
      {/if}
    </div>
  {/if}

  <div class="mx-auto grid w-full max-w-6xl flex-1 grid-cols-1 overflow-hidden md:grid-cols-12">
    <aside class="flex flex-col gap-2 overflow-y-auto border-border/40 bg-muted/10 p-4 sm:p-5 md:col-span-4 md:border-e">
      <div class="px-2 py-1 font-mono text-[11px] font-bold uppercase tracking-wider text-muted-foreground">
        {m.ui_changelog_release_history()}
      </div>

      {#if isLoading}
        <div class="flex items-center justify-center gap-2 p-6 text-center text-xs text-muted-foreground">
          <LoadingIndicator
            size="sm"
            label={m.ui_changelog_fetching_live_release_data_from_github()}
          />
          <span aria-hidden="true">{m.ui_changelog_fetching_live_release_data_from_github()}</span>
        </div>
      {:else if loadError}
        <p class="p-3 text-xs text-destructive" role="alert">{m.ui_changelog_load_failed()}</p>
      {:else if releases.length === 0}
        <p class="p-3 text-xs text-muted-foreground">{m.ui_changelog_no_releases()}</p>
      {:else}
        {#each releases as release (release.version)}
          {@const selected = activeRelease?.version === release.version}
          {@const date = formatDate(release.published_at)}
          <button
            type="button"
            onclick={() => (selectedVersion = release.version)}
            aria-pressed={selected}
            class="flex w-full cursor-pointer flex-col gap-1.5 rounded-xl border p-3 text-start text-xs transition-all {selected
              ? 'border-border bg-background font-medium text-foreground shadow-sm'
              : 'border-transparent text-muted-foreground hover:bg-muted/30 hover:text-foreground'}"
          >
            <span class="flex items-center justify-between gap-1">
              <span class="font-mono text-sm font-bold text-foreground">v{release.version}</span>
              {#if release.is_latest}
                <Badge variant="default" class="h-4 bg-primary px-1.5 font-mono text-[9px] text-primary-foreground">
                  {m.ui_changelog_latest()}
                </Badge>
              {:else if release.is_current}
                <Badge variant="secondary" class="h-4 px-1.5 font-mono text-[9px]">
                  {m.ui_changelog_installed()}
                </Badge>
              {/if}
            </span>
            <span class="line-clamp-1 text-xs text-foreground/80">{release.title}</span>
            {#if date}
              <span class="flex items-center gap-1 font-mono text-[10px] text-muted-foreground">
                <Calendar size={11} />{date}
              </span>
            {/if}
          </button>
        {/each}
      {/if}
    </aside>

    <main class="space-y-6 overflow-y-auto bg-background p-6 sm:p-8 md:col-span-8">
      {#if isLoading}
        <p class="py-24 text-center text-sm text-muted-foreground">{m.ui_changelog_loading_release_changelog()}</p>
      {:else if loadError}
        <p class="py-24 text-center text-sm text-destructive" role="alert">{m.ui_changelog_load_failed()}</p>
      {:else if activeRelease}
        {@const date = formatDate(activeRelease.published_at)}
        <section class="space-y-3 border-b border-border/40 pb-5">
          <div class="flex flex-wrap items-center justify-between gap-2">
            <div class="flex min-w-0 items-center gap-2">
              <Badge variant="outline" class="shrink-0 border-primary/40 bg-primary/5 px-2 py-0.5 font-mono text-[10px] text-primary">
                v{activeRelease.version}
              </Badge>
              <h1 class="text-xl font-bold tracking-tight sm:text-2xl">{activeRelease.title}</h1>
            </div>
            <a
              href={activeRelease.html_url}
              target="_blank"
              rel="noreferrer"
              class="text-xs text-primary underline underline-offset-4"
            >
              {m.ui_changelog_github_release()}
            </a>
          </div>
          {#if date}
            <div class="flex items-center gap-1 text-xs text-muted-foreground">
              <Calendar size={12} />
              <span>{m.shared_ui_release_date({ date })}</span>
            </div>
          {/if}
        </section>

        {#if activeRelease.images.length > 0}
          <section class="space-y-3">
            <h2 class="text-xs font-bold uppercase tracking-wider text-foreground">{m.ui_changelog_release_images()}</h2>
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
              {#each activeRelease.images as image (image)}
                <img
                  src={image}
                  alt={activeRelease.title}
                  loading="lazy"
                  referrerpolicy="no-referrer"
                  class="max-h-72 w-full rounded-lg border border-border/50 bg-muted/20 object-contain"
                />
              {/each}
            </div>
          </section>
        {/if}

        {#if activeRelease.body?.trim()}
          <section class="space-y-3">
            <h2 class="text-xs font-bold uppercase tracking-wider text-foreground">{m.ui_changelog_release_notes()}</h2>
            <div class="rounded-lg border border-border/50 bg-muted/20 p-4">
              <ReleaseMarkdown markdown={activeRelease.body} />
            </div>
          </section>
        {:else}
          <p class="rounded-lg border border-border/50 bg-muted/20 p-4 text-sm text-muted-foreground">
            {m.ui_changelog_no_release_notes()}
          </p>
        {/if}

        {#if activeRelease.resources.length > 0}
          <section class="space-y-3">
            <h2 class="text-xs font-bold uppercase tracking-wider text-foreground">{m.ui_changelog_release_resources()}</h2>
            <ul class="space-y-2">
              {#each activeRelease.resources as resource (resource.url)}
                <li>
                  <a
                    href={resource.url}
                    target="_blank"
                    rel="noreferrer"
                    class="flex items-center justify-between gap-3 rounded-lg border border-border/50 bg-muted/20 px-3 py-2 text-sm text-primary underline underline-offset-4"
                  >
                    <span class="min-w-0 truncate">{resource.name}</span>
                    <span class="shrink-0 font-mono text-xs text-muted-foreground">{(resource.size / 1024).toFixed(0)} KB</span>
                  </a>
                </li>
              {/each}
            </ul>
          </section>
        {/if}
      {:else}
        <p class="py-24 text-center text-sm text-muted-foreground">{m.ui_changelog_no_releases()}</p>
      {/if}
    </main>
  </div>
</div>
