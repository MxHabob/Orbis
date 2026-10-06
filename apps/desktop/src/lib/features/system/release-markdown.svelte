<script lang="ts">
  import { renderReleaseMarkdown } from "$lib/utils/release-markdown";

  let { markdown = "" }: { markdown?: string | null } = $props();
  const rendered = $derived(renderReleaseMarkdown(markdown));
</script>

<div class="release-markdown text-sm leading-7 text-foreground">
  <!-- This HTML is produced by the release renderer, which escapes raw HTML and restricts link/image URLs. -->
  {@html rendered}
</div>

<style>
  .release-markdown :global(h1),
  .release-markdown :global(h2),
  .release-markdown :global(h3),
  .release-markdown :global(h4) {
    margin-block: 1.25em 0.5em;
    font-weight: 700;
    line-height: 1.3;
  }

  .release-markdown :global(h1) { font-size: 1.5rem; }
  .release-markdown :global(h2) { font-size: 1.25rem; }
  .release-markdown :global(h3) { font-size: 1.1rem; }
  .release-markdown :global(p),
  .release-markdown :global(ul),
  .release-markdown :global(ol),
  .release-markdown :global(blockquote),
  .release-markdown :global(pre),
  .release-markdown :global(table) {
    margin-block: 0.75em;
  }
  .release-markdown :global(ul) { list-style: disc; padding-inline-start: 1.5rem; }
  .release-markdown :global(ol) { list-style: decimal; padding-inline-start: 1.5rem; }
  .release-markdown :global(li + li) { margin-block-start: 0.25em; }
  .release-markdown :global(a) { color: var(--color-primary); text-decoration: underline; text-underline-offset: 3px; }
  .release-markdown :global(blockquote) {
    border-inline-start: 3px solid var(--color-border);
    padding-inline-start: 1rem;
    color: var(--color-muted-foreground);
  }
  .release-markdown :global(code) {
    border-radius: 0.25rem;
    background: var(--color-muted);
    padding: 0.1em 0.35em;
    font-family: var(--font-mono);
    font-size: 0.9em;
  }
  .release-markdown :global(pre) {
    overflow-x: auto;
    border: 1px solid var(--color-border);
    border-radius: 0.5rem;
    background: var(--color-muted);
    padding: 1rem;
  }
  .release-markdown :global(pre code) { padding: 0; background: transparent; }
  .release-markdown :global(table) { display: block; max-width: 100%; overflow-x: auto; border-collapse: collapse; }
  .release-markdown :global(th),
  .release-markdown :global(td) { border: 1px solid var(--color-border); padding: 0.45rem 0.7rem; text-align: start; }
  .release-markdown :global(th) { background: var(--color-muted); font-weight: 600; }
  .release-markdown :global(img) {
    display: block;
    max-height: 28rem;
    max-width: 100%;
    margin-block: 1rem;
    border: 1px solid var(--color-border);
    border-radius: 0.75rem;
    object-fit: contain;
  }
</style>
