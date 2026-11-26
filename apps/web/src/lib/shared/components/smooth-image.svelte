<script lang="ts">
import { Photo } from "@orbis/icons";

type ImageEvent = Event & { currentTarget: EventTarget & Element };

let {
  src,
  alt,
  class: className = "h-full w-full object-cover",
  containerClass = "",
  style,
  loading = "lazy",
  decoding = "async",
  fetchpriority,
  width,
  height,
  onload,
  onerror,
}: {
  src?: string | null;
  alt: string;
  class?: string;
  containerClass?: string;
  style?: string;
  loading?: "lazy" | "eager";
  decoding?: "async" | "sync" | "auto";
  fetchpriority?: "high" | "low" | "auto";
  width?: number | string;
  height?: number | string;
  onload?: (event: ImageEvent) => void;
  onerror?: (event: ImageEvent) => void;
} = $props();

let imgEl = $state<HTMLImageElement | null>(null);
let loadedSrc = $state<string | null>(null);
let erroredSrc = $state<string | null>(null);

const resolvedSrc = $derived(src || "");
const loaded = $derived(Boolean(resolvedSrc) && loadedSrc === resolvedSrc);
const errored = $derived(!resolvedSrc || erroredSrc === resolvedSrc);

// Sync with browser cache state when `src` changes or `imgEl` mounts.
$effect(() => {
  const currentSrc = resolvedSrc;
  const el = imgEl;
  if (currentSrc && el && el.complete && el.naturalWidth > 0) {
    loadedSrc = currentSrc;
  }
});

function handleLoad(event: ImageEvent) {
  loadedSrc = resolvedSrc;
  onload?.(event);
}

function handleError(event: ImageEvent) {
  erroredSrc = resolvedSrc;
  onerror?.(event);
}
</script>

<div class="relative overflow-hidden {containerClass}">
  {#if !loaded && !errored}
    <div
      aria-hidden="true"
      class="pointer-events-none absolute inset-0 bg-muted/50 animate-pulse"
    ></div>
  {/if}

  {#if errored}
    <div class="flex h-full w-full items-center justify-center bg-muted/30 text-muted-foreground/40">
      <Photo size={20} />
    </div>
  {:else}
    <img
      bind:this={imgEl}
      src={resolvedSrc}
      {alt}
      {loading}
      {decoding}
      {fetchpriority}
      {width}
      {height}
      {style}
      onload={handleLoad}
      onerror={handleError}
      class="{className} transition-opacity duration-200 ease-out {loaded
        ? 'opacity-100'
        : 'opacity-0'}"
    />
  {/if}
</div>
