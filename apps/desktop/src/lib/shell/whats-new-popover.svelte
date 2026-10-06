<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { api } from "$lib/api/client";
  import type { ReleaseChangelogItem } from "$lib/api/types";
  import { getLocale } from "$lib/paraglide/runtime";
  import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { ArrowPath, Calendar, Sparkles } from "@orbis/icons";
  import ReleaseMarkdown from "$lib/features/system/release-markdown.svelte";

  let isOpen = $state(false);
  let latestRelease = $state<ReleaseChangelogItem | null>(null);
  let isLoading = $state(false);
  let hasLoaded = $state(false);
  let loadError = $state(false);
  let openError = $state(false);

  const locale = $derived(getLocale());
  const releaseDate = $derived.by(() => {
    const value = latestRelease?.published_at;
    if (!value) return null;

    const date = new Date(value);
    return Number.isNaN(date.getTime())
      ? null
      : new Intl.DateTimeFormat(locale, { dateStyle: "medium" }).format(date);
  });

  async function loadChangelog() {
    isLoading = true;
    loadError = false;
    hasLoaded = true;
    try {
      const releases = await api.systemGetChangelog();
      latestRelease = releases.find((release) => release.is_latest) ?? releases[0] ?? null;
    } catch (error) {
      console.error("Failed to load the latest changelog entry:", error);
      loadError = true;
    } finally {
      isLoading = false;
    }
  }

  async function openChangelog() {
    openError = false;
    try {
      await api.openChangelogWindow();
      isOpen = false;
    } catch (error) {
      console.error("Failed to open the changelog:", error);
      openError = true;
    }
  }

  function handleOpenChange(open: boolean) {
    isOpen = open;
    if (open && !hasLoaded && !isLoading) void loadChangelog();
  }
</script>

<Popover bind:open={isOpen} onOpenChange={handleOpenChange}>
  <PopoverTrigger>
    <Button
      variant="outline"
      size="icon-sm"
      class="cursor-pointer"
      title={m.common_whats_new_header_title()}
      aria-label={m.common_whats_new_header_title()}
    >
      <Sparkles size={14} class="text-foreground stroke-[2px]" />
    </Button>
  </PopoverTrigger>

  <PopoverContent
    dir="auto"
    side="bottom"
    align="end"
    sideOffset={8}
    class="w-[min(420px,calc(100vw-2rem))] max-w-[calc(100vw-2rem)] overflow-hidden rounded-xl p-0 font-sans text-start"
  >
    <div class="flex items-center justify-between border-b border-border px-4 py-3">
      <h4 class="text-sm font-bold text-foreground">
        {m.common_whats_new_header_title()}
      </h4>
      <Button
        variant="ghost"
        size="icon-xs"
        class="cursor-pointer"
        title={m.ui_changelog_release_history()}
        aria-label={m.ui_changelog_release_history()}
        disabled={isLoading}
        onclick={() => void loadChangelog()}
      >
        <span class={isLoading ? "animate-spin" : ""}><ArrowPath size={14} /></span>
      </Button>
    </div>

    <div class="max-h-[min(440px,calc(100vh-7rem))] overflow-y-auto px-4 py-4">
      {#if isLoading}
        <p class="py-8 text-center text-xs text-muted-foreground" aria-live="polite">
          {m.ui_changelog_loading_release_changelog()}
        </p>
      {:else if loadError}
        <div class="space-y-3 py-4 text-center">
          <p class="text-xs text-destructive" role="alert">{m.ui_changelog_load_failed()}</p>
          <Button variant="outline" size="sm" onclick={() => void loadChangelog()}>
            {m.common_retry()}
          </Button>
        </div>
      {:else if latestRelease}
        <article class="flex flex-col gap-3">
          <div class="flex items-start justify-between gap-2">
            <div class="min-w-0">
              <h5 class="text-sm font-bold leading-snug text-foreground">
                {latestRelease.title}
              </h5>
              <div class="mt-1 flex flex-wrap items-center gap-x-2 gap-y-1 text-[11px] text-muted-foreground">
                <span class="font-mono">v{latestRelease.version}</span>
                {#if releaseDate}
                  <span class="flex items-center gap-1">
                    <Calendar size={11} />
                    {releaseDate}
                  </span>
                {/if}
              </div>
            </div>
            <Badge variant="default" class="shrink-0 bg-primary px-1.5 text-[9px] text-primary-foreground">
              {m.ui_changelog_latest()}
            </Badge>
          </div>

          {#if latestRelease.images.length > 0}
            <div class="grid grid-cols-2 gap-2">
              {#each latestRelease.images.slice(0, 2) as image (image)}
                <img
                  src={image}
                  alt={latestRelease.title}
                  loading="lazy"
                  referrerpolicy="no-referrer"
                  class="max-h-24 w-full rounded-lg border border-border/50 bg-muted/20 object-cover"
                />
              {/each}
            </div>
          {/if}

          {#if latestRelease.body?.trim()}
            <div class="max-h-56 overflow-y-auto rounded-lg border border-border/50 bg-muted/20 p-3">
              <ReleaseMarkdown markdown={latestRelease.body} />
            </div>
          {:else}
            <p class="rounded-lg border border-border/50 bg-muted/20 p-3 text-xs text-muted-foreground">
              {m.ui_changelog_no_release_notes()}
            </p>
          {/if}

          {#if openError}
            <p class="text-xs text-destructive" role="alert">
              {m.common_whats_new_open_changelog_failed()}
            </p>
          {/if}

          <Button variant="outline" size="sm" class="w-full cursor-pointer" onclick={() => void openChangelog()}>
            {m.common_whats_new_read_more()}
          </Button>
        </article>
      {:else}
        <p class="py-8 text-center text-xs text-muted-foreground">
          {m.ui_changelog_no_releases()}
        </p>
      {/if}
    </div>
  </PopoverContent>
</Popover>
