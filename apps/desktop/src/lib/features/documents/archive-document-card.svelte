<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Button } from "@orbis/ui/button";
  import { Checkbox } from "@orbis/ui/checkbox";
  import { getDocumentSource, resolveDocumentUrl } from "$lib/features/documents/queries";
  import { Download, FileText, Music, Play, Trash2 } from "@lucide/svelte";
  import type { Documents } from "@orbis/sdk";

  let {
    document,
    selected = false,
    onpreview,
    ondelete,
    ontoggle,
  }: {
    document: Documents;
    selected?: boolean;
    onpreview: (document: Documents) => void;
    ondelete: (id: string) => void;
    ontoggle: (id: string, selected: boolean) => void;
  } = $props();

  const mimeType = $derived(document.mime_type?.toLowerCase() ?? "");
  const isImage = $derived(mimeType.startsWith("image/"));
  const isVideo = $derived(mimeType.startsWith("video/"));
  const isAudio = $derived(mimeType.startsWith("audio/"));
  const source = $derived(getDocumentSource(document));
  const extension = $derived(document.extension?.toUpperCase() || "FILE");
  let isNearViewport = $state(false);

  function stopPropagation(event: MouseEvent) {
    event.stopPropagation();
  }

  function loadWhenVisible(node: HTMLDivElement) {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (!entry?.isIntersecting) return;
        isNearViewport = true;
        observer.disconnect();
      },
      { rootMargin: "240px" },
    );
    observer.observe(node);
    return { destroy: () => observer.disconnect() };
  }

</script>

<article class="archive-card group">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="archive-media relative cursor-pointer overflow-hidden rounded-lg bg-muted/30"
    use:loadWhenVisible
    role="button"
    tabindex="0"
    onclick={() => onpreview(document)}
    onkeydown={(event) => event.key === "Enter" && onpreview(document)}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="absolute inset-s-2 top-2 z-10 rounded-md bg-background/85 p-1 shadow-sm" onclick={(event) => event.stopPropagation()}>
      <Checkbox checked={selected} onCheckedChange={(value) => ontoggle(document.id, value === true)} aria-label={`تحديد ${document.name}`} />
    </div>
    {#if isNearViewport && isImage}
      {#await resolveDocumentUrl(source) then mediaUrl}
        <img src={mediaUrl} alt={document.name} loading="lazy" decoding="async" width="320" height="240" class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105" />
      {:catch}
        <FileText class="m-auto h-8 w-8 text-muted-foreground/60" />
      {/await}
    {:else if isNearViewport && isVideo}
      {#await resolveDocumentUrl(source) then sourceUrl}
        <video src={sourceUrl} muted preload="none" class="h-full w-full object-cover" aria-label={document.name}></video>
        <div class="pointer-events-none absolute inset-0 flex items-center justify-center bg-black/10">
          <span class="rounded-full bg-black/60 p-2 text-white"><Play class="h-4 w-4 fill-current" /></span>
        </div>
      {/await}
    {:else if isAudio}
      <div class="flex h-full flex-col items-center justify-center gap-2 text-muted-foreground/70">
        <Music class="h-10 w-10" strokeWidth={1.5} />
        <span class="text-[10px]">ملف صوتي</span>
      </div>
    {:else}
      <div class="flex h-full flex-col items-center justify-center gap-2 text-muted-foreground/70">
        <FileText class="h-9 w-9" strokeWidth={1.5} />
        <span class="rounded bg-muted px-2 py-0.5 text-[10px] font-semibold">{extension}</span>
      </div>
    {/if}

    <div class="absolute inset-0 flex items-center justify-center gap-2 bg-background/60 opacity-0 backdrop-blur-[2px] transition-opacity group-hover:opacity-100">
      {#if isNearViewport}
        {#await resolveDocumentUrl(source) then sourceUrl}
          <a
            href={sourceUrl}
            download={document.name}
            class="rounded-md bg-background/95 p-2 text-foreground shadow-sm"
            title={m.settings_archive_action_download()}
            aria-label={m.settings_archive_action_download()}
            onclick={stopPropagation}
          >
            <Download class="h-4 w-4" />
          </a>
        {/await}
      {/if}
      <Button
        variant="destructive"
        size="icon-sm"
        onclick={(event) => { stopPropagation(event); ondelete(document.id); }}
        title={m.common_action_delete()}
        aria-label={m.common_action_delete()}
      >
        <Trash2 class="h-4 w-4" />
      </Button>
    </div>
  </div>

  <div class="min-w-0 px-0.5 pt-2">
    <p class="truncate text-xs font-medium text-foreground" title={document.name}>{document.name}</p>
    <div class="mt-1 flex items-center justify-between gap-2 text-[10px] text-muted-foreground">
      <span class="truncate">{document.category || "archive"}</span>
      <span class="shrink-0 font-mono">{((document.size || 0) / 1024).toFixed(1)} KB</span>
    </div>
  </div>
</article>

<style>
  .archive-card {
    content-visibility: auto;
    contain-intrinsic-size: 240px;
  }

  .archive-media {
    aspect-ratio: 4 / 3;
  }
</style>
