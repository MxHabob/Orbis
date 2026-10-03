<!--
  @component ChangelogWindowPage
  Independent native window and full page route for Orbis & Mulverse Changelog & Release Notes.
  Renders genuine Markdown from GitHub Releases using 'marked' with zero fake data.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { marked } from "marked";
import { api } from "$lib/api/client";
import type { ReleaseChangelogItem, UpdateCheckResult, MulverseEcosystemInfo } from "$lib/api/types";
import { getLocale } from "$lib/paraglide/runtime";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Sparkles,
  ArrowPath,
  Check,
  Calendar,
  Tag,
  ShieldCheck,
  Cube,
} from "@orbis/icons";

let releases = $state<ReleaseChangelogItem[]>([]);
let ecosystemInfo = $state<MulverseEcosystemInfo | null>(null);
let selectedVersion = $state<string>("");
let isLoading = $state(true);
let isCheckingUpdate = $state(false);
let updateResult = $state<UpdateCheckResult | null>(null);

const isRTL = $derived(getLocale().startsWith("ar"));

const activeRelease = $derived(
  releases.find((r) => r.version === selectedVersion) || releases[0] || null
);

const renderedMarkdown = $derived.by(() => {
  if (!activeRelease) return "";
  if (activeRelease.raw_body) {
    return marked.parse(activeRelease.raw_body) as string;
  }
  // Fallback to highlights if raw body is empty
  const highlights = isRTL ? activeRelease.highlights_ar : activeRelease.highlights_en;
  const mdList = highlights.map((h) => `- ${h}`).join("\n");
  return marked.parse(`### ${activeRelease.title_en}\n\n${mdList}`) as string;
});

async function loadData() {
  isLoading = true;
  try {
    const [list, info] = await Promise.all([
      api.systemGetChangelog(),
      api.systemGetEcosystemInfo(),
    ]);
    releases = list;
    ecosystemInfo = info;
    if (list.length > 0) {
      selectedVersion = list[0].version;
    }
  } catch (err) {
    console.error("Failed to load changelog data:", err);
  } finally {
    isLoading = false;
  }
}

async function handleCheckForUpdates() {
  isCheckingUpdate = true;
  try {
    const res = await api.systemCheckForUpdates();
    updateResult = res;
  } catch (err) {
    console.error("Failed to check for updates:", err);
  } finally {
    isCheckingUpdate = false;
  }
}

onMount(() => {
  loadData();
});
</script>

<svelte:head>
  <title>{m.ui_changelog_what_s_new_changelog_orbis()}</title>
</svelte:head>

<div class="min-h-screen bg-background text-foreground flex flex-col selection:bg-primary/20 selection:text-primary font-sans">
  <!-- Top Native Header Bar -->
  <header class="h-14 border-b border-border/50 bg-background/95 backdrop-blur-md sticky top-0 z-30 flex items-center justify-between px-6">
    <div class="flex items-center gap-3">
      <div class="size-7 rounded-lg bg-primary/10 border border-primary/25 flex items-center justify-center text-primary">
        <Cube size={16} />
      </div>
      <div class="flex items-center gap-2">
        <span class="text-sm font-bold tracking-tight text-foreground">Orbis</span>
        <span class="text-xs text-muted-foreground">•</span>
        <span class="text-xs text-muted-foreground font-medium">
          {m.ui_changelog_what_s_new_releases()}
        </span>
      </div>
      <Badge variant="outline" class="text-[10px] font-mono px-1.5 py-0 h-4 border-primary/40 text-primary bg-primary/5">
        v{ecosystemInfo?.version || "0.3.0"}
      </Badge>
    </div>

    <div class="flex items-center gap-3">
      <Button
        variant="outline"
        size="sm"
        disabled={isCheckingUpdate}
        onclick={handleCheckForUpdates}
        class="h-8 text-xs font-normal gap-1.5 px-3 border-border/60"
      >
        <span class={isCheckingUpdate ? "animate-spin" : ""}>
          <ArrowPath size={13} />
        </span>
        <span>
          {isCheckingUpdate
            ? (m.ui_changelog_checking())
            : (m.ui_changelog_check_for_updates())}
        </span>
      </Button>

      <Button
        variant="ghost"
        size="sm"
        onclick={() => window.open(`https://github.com/${ecosystemInfo?.github_repo || "MxHabob/Orbis"}`, "_blank")}
        class="h-8 text-xs font-normal gap-1 px-2.5 text-muted-foreground hover:text-foreground"
      >
        <span>GitHub</span>
      </Button>
    </div>
  </header>

  <!-- Live Update Banner (if checked) -->
  {#if updateResult}
    <div class="px-6 py-2.5 border-b text-xs flex items-center justify-between transition-all {updateResult.has_update
      ? 'bg-primary/10 border-primary/20 text-primary'
      : 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400'}">
      <div class="flex items-center gap-2">
        {#if updateResult.has_update}
          <span class="font-medium">
            {isRTL
              ? `يتوفر تحديث جديد (${updateResult.latest_version}) متاح للتحميل والترقية!`
              : `New version ${updateResult.latest_version} is available!`}
          </span>
        {:else}
          <Check size={14} />
          <span>
            {isRTL
              ? `أنت تستخدم أحدث إصدار معتمد من النظام (${updateResult.current_version}).`
              : `You are running the latest official release (v${updateResult.current_version}).`}
          </span>
        {/if}
      </div>

      {#if updateResult.release_url}
        <a
          href={updateResult.release_url}
          target="_blank"
          rel="noreferrer"
          class="text-[11px] underline flex items-center gap-1 font-medium hover:opacity-80"
        >
          <span>{m.ui_changelog_download_from_github()}</span>
        </a>
      {/if}
    </div>
  {/if}

  <!-- Main Body Grid: Version Rail (Left) + Markdown Content (Right) -->
  <div class="flex-1 max-w-6xl w-full mx-auto grid grid-cols-1 md:grid-cols-12 overflow-hidden">
    <!-- Version Rail -->
    <aside class="md:col-span-4 border-r border-border/40 p-4 sm:p-5 flex flex-col gap-2 overflow-y-auto bg-muted/10">
      <div class="px-2 py-1 text-[11px] font-bold text-muted-foreground uppercase tracking-wider font-mono">
        {m.ui_changelog_release_history()}
      </div>

      {#if isLoading}
        <div class="p-6 text-center text-xs text-muted-foreground animate-pulse">
          {m.ui_changelog_fetching_live_release_data_from_github()}
        </div>
      {:else}
        {#each releases as rel}
          {@const isSelected = activeRelease?.version === rel.version}
          <button
            type="button"
            onclick={() => (selectedVersion = rel.version)}
            class="w-full text-start p-3 rounded-xl text-xs transition-all flex flex-col gap-1.5 cursor-pointer border {isSelected
              ? 'bg-background text-foreground shadow-sm border-border font-medium'
              : 'text-muted-foreground hover:text-foreground hover:bg-muted/30 border-transparent'}"
          >
            <div class="flex items-center justify-between gap-1">
              <span class="font-mono font-bold text-foreground text-sm">v{rel.version}</span>
              {#if rel.is_latest}
                <Badge variant="default" class="text-[9px] px-1.5 py-0 h-4 bg-primary text-primary-foreground font-mono">
                  {m.ui_changelog_latest()}
                </Badge>
              {:else if rel.is_current}
                <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4 font-mono">
                  {m.ui_changelog_installed()}
                </Badge>
              {/if}
            </div>

            <span class="text-xs text-foreground/80 line-clamp-1">
              {isRTL ? rel.title_ar : rel.title_en}
            </span>

            <div class="flex items-center gap-1 text-[10px] text-muted-foreground font-mono">
              <Calendar size={11} />
              <span>{rel.release_date}</span>
            </div>
          </button>
        {/each}
      {/if}

      <!-- Mulverse Info Badge -->
      <div class="mt-auto pt-4 p-3 rounded-xl border border-border/40 bg-card/60 text-[11px] text-muted-foreground space-y-1">
        <div class="flex items-center gap-1.5 font-semibold text-foreground">
          <ShieldCheck size={13} class="text-primary" />
          <span>Mulverse Autonomous Core</span>
        </div>
        <p class="leading-relaxed">
          {isRTL
            ? "نظام مبني على نواة Rust وSQLite المحلية لضمان السيادة الرقمية والسرعة الفائقة."
            : "Engineered on Rust & SQLite for native speed and complete institutional sovereignty."}
        </p>
      </div>
    </aside>

    <!-- Release Detail View with Real Rendered Markdown -->
    <main class="md:col-span-8 p-6 sm:p-8 overflow-y-auto space-y-6 bg-background">
      {#if activeRelease}
        <!-- Header info -->
        <div class="space-y-3 pb-5 border-b border-border/40">
          <div class="flex items-center justify-between flex-wrap gap-2">
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <Badge variant="outline" class="text-[10px] font-mono px-2 py-0.5 border-primary/40 text-primary bg-primary/5">
                  v{activeRelease.version}
                </Badge>
                <h1 class="text-xl sm:text-2xl font-bold text-foreground tracking-tight">
                  {isRTL ? activeRelease.title_ar : activeRelease.title_en}
                </h1>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onclick={() => window.open(activeRelease.html_url, "_blank")}
                class="h-8 text-xs font-normal gap-1.5 px-3"
              >
                <span>{m.ui_changelog_github_release()}</span>
              </Button>
            </div>
          </div>

          <div class="flex items-center gap-4 text-xs text-muted-foreground font-mono">
            <span class="flex items-center gap-1">
              <Calendar size={12} />
              <span>Released: {activeRelease.release_date}</span>
            </span>
            <span class="flex items-center gap-1">
              <Tag size={12} />
              <span>Target: Autonomous Node</span>
            </span>
          </div>
        </div>

        <!-- Markdown Body (Rendered via 'marked') -->
        <div class="prose prose-sm dark:prose-invert max-w-none text-foreground leading-relaxed space-y-4">
          {@html renderedMarkdown}
        </div>

        <!-- Highlights Box -->
        <div class="space-y-2.5 pt-4 border-t border-border/40">
          <h3 class="text-xs font-bold uppercase tracking-wider text-foreground font-mono flex items-center gap-1.5">
            <Sparkles size={13} class="text-primary" />
            <span>{m.ui_changelog_release_verification_matrix()}</span>
          </h3>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            {#each (isRTL ? activeRelease.highlights_ar : activeRelease.highlights_en) as item}
              <div class="p-3 rounded-lg border border-border/50 bg-muted/20 flex items-start gap-2.5 text-xs">
                <div class="size-4 rounded-full bg-primary/10 text-primary flex items-center justify-center shrink-0 mt-0.5">
                  <Check size={11} strokeWidth={2.5} />
                </div>
                <span class="text-foreground leading-snug">{item}</span>
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <div class="py-24 text-center text-sm text-muted-foreground">
          {m.ui_changelog_loading_release_changelog()}
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  :global(.prose h1) {
    font-size: 1.35rem;
    font-weight: 700;
    margin-top: 1rem;
    margin-bottom: 0.5rem;
  }
  :global(.prose h2) {
    font-size: 1.15rem;
    font-weight: 700;
    margin-top: 1rem;
    margin-bottom: 0.5rem;
  }
  :global(.prose h3) {
    font-size: 1rem;
    font-weight: 600;
    margin-top: 0.75rem;
    margin-bottom: 0.25rem;
  }
  :global(.prose ul) {
    list-style-type: disc;
    padding-inline-start: 1.25rem;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
  }
  :global(.prose li) {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }
</style>
