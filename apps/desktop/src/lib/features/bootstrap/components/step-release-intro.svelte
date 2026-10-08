<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { onMount } from "svelte";
  import { api } from "$lib/api/client";
  import type { AppInfo, ReleaseChangelogItem } from "$lib/api/types";
  import { Badge } from "@orbis/ui/badge";
  import { Calendar, Sparkles } from "@orbis/icons";
  import ReleaseMarkdown from "$lib/features/system/release-markdown.svelte";
  import EmptyState from "$lib/shared/components/empty-state.svelte";
  import { formatDate as formatOrgDate } from "$lib/core/i18n";

  let { isRTL = false }: { isRTL?: boolean } = $props();

  let appInfo = $state<AppInfo | null>(null);
  let latestRelease = $state<ReleaseChangelogItem | null>(null);
  let isLoading = $state(true);

  function formatDate(value?: string) {
    if (!value) return null;
    const date = new Date(value);
    return Number.isNaN(date.getTime())
      ? null
      : formatOrgDate(date, "date");
  }

  onMount(async () => {
    try {
      const [info, releases] = await Promise.all([
        api.systemGetAppInfo().catch((err) => {
          console.warn("App info load failed:", err);
          return null;
        }),
        api.systemGetChangelog().catch((err) => {
          console.warn("Changelog load failed, treating as empty:", err);
          return [] as ReleaseChangelogItem[];
        }),
      ]);
      appInfo = info;
      latestRelease = releases.find((release) => release.is_latest) ?? releases[0] ?? null;
    } catch (error) {
      console.warn("Failed to load application release information:", error);
      latestRelease = null;
    } finally {
      isLoading = false;
    }
  });
</script>

<section class="mx-auto w-full max-w-5xl space-y-6 py-2">
  <div class="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-lg">
    <div class="space-y-2 border-b border-border/50 p-6 sm:p-8">
      {#if appInfo}
        <div class="flex flex-wrap items-center gap-2">
          <h1 class="text-2xl font-extrabold tracking-tight text-foreground sm:text-3xl">{appInfo.name}</h1>
          <Badge variant="outline" class="border-primary/40 bg-primary/5 font-mono text-primary">
            v{appInfo.version}
          </Badge>
        </div>
      {/if}
      <p class="max-w-3xl text-sm leading-relaxed text-muted-foreground">
        {m.ui_onboarding_intro_description()}
      </p>
    </div>

    <div class="space-y-5 p-6 sm:p-8">
      {#if isLoading}
        <p class="animate-pulse text-sm text-muted-foreground">{m.ui_changelog_fetching_live_release_data_from_github()}</p>
      {:else if latestRelease}
        {@const publishedDate = formatDate(latestRelease.published_at)}
        <div class="flex flex-wrap items-start justify-between gap-3">
          <div class="space-y-2">
            <p class="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {m.ui_onboarding_intro_latest_release()}
            </p>
            <h2 class="text-xl font-bold tracking-tight text-foreground">{latestRelease.title}</h2>
            <div class="flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
              <Badge variant="secondary" class="font-mono">v{latestRelease.version}</Badge>
              {#if publishedDate}
                <span class="flex items-center gap-1">
                  <Calendar size={12} />{publishedDate}
                </span>
              {/if}
            </div>
          </div>
          <a
            href={latestRelease.html_url}
            target="_blank"
            rel="noreferrer"
            class="text-xs text-primary underline underline-offset-4"
          >
            {m.ui_changelog_github_release()}
          </a>
        </div>

        {#if latestRelease.images.length > 0}
          <section class="space-y-3">
            <h3 class="text-xs font-bold uppercase tracking-wider text-foreground">
              {m.ui_onboarding_intro_release_images()}
            </h3>
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
              {#each latestRelease.images as image (image)}
                <img
                  src={image}
                  alt={latestRelease.title}
                  loading="lazy"
                  referrerpolicy="no-referrer"
                  class="max-h-72 w-full rounded-xl border border-border/60 bg-muted/20 object-contain"
                />
              {/each}
            </div>
          </section>
        {/if}

        {#if latestRelease.body?.trim()}
          <section class="space-y-3">
            <h3 class="text-xs font-bold uppercase tracking-wider text-foreground">
              {m.ui_onboarding_intro_release_notes()}
            </h3>
            <div class="max-h-[32rem] overflow-y-auto rounded-xl border border-border/60 bg-muted/20 p-4">
              <ReleaseMarkdown markdown={latestRelease.body} />
            </div>
          </section>
        {:else}
          <p class="rounded-xl border border-border/60 bg-muted/20 p-4 text-sm text-muted-foreground">
            {m.ui_changelog_no_release_notes()}
          </p>
        {/if}

        {#if latestRelease.resources.length > 0}
          <section class="space-y-3">
            <h3 class="text-xs font-bold uppercase tracking-wider text-foreground">
              {m.ui_changelog_release_resources()}
            </h3>
            <ul class="space-y-2">
              {#each latestRelease.resources as resource (resource.url)}
                <li>
                  <a
                    href={resource.url}
                    target="_blank"
                    rel="noreferrer"
                    class="flex items-center justify-between gap-3 rounded-lg border border-border/60 bg-muted/20 px-3 py-2 text-sm text-primary underline underline-offset-4"
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
        <EmptyState
          icon={Sparkles}
          title={m.ui_onboarding_intro_no_releases()}
          description={m.ui_changelog_no_release_notes()}
          class="py-12"
        />
      {/if}
    </div>
  </div>
</section>
