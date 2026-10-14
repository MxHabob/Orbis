<script lang="ts">
  import { onMount } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { goto } from "$app/navigation";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { api, extractErrorMessage } from "$lib/api";
  import { toast } from "@orbis/ui";
  import { l } from "$lib/core/i18n";
  import {
    createTemplateFromCatalog,
    listReadyTemplates,
    type ReadyTemplate,
  } from "../marketplace-client";

  interface Props {
    orgId: string;
    onimported: () => Promise<void>;
  }

  let { orgId, onimported }: Props = $props();

  let sectionElement: HTMLElement;
  let templates = $state<ReadyTemplate[]>([]);
  let isLoading = $state(false);
  let importingId = $state<string | null>(null);
  let search = $state("");
  let errorMessage = $state("");
  let hasLoaded = $state(false);

  const filteredTemplates = $derived.by(() => {
    const query = search.trim().toLocaleLowerCase();
    if (!query) return templates;

    return templates.filter((template) =>
      [
        template.name,
        template.name_ar,
        template.description,
        template.description_ar,
        template.document_type,
        template.entity_type,
      ].some((value) => value?.toLocaleLowerCase().includes(query)),
    );
  });

  onMount(() => {
    if (typeof IntersectionObserver === "undefined") {
      void loadTemplates();
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((entry) => entry.isIntersecting)) {
          observer.disconnect();
          void loadTemplates();
        }
      },
      { rootMargin: "240px" },
    );

    observer.observe(sectionElement);
    return () => observer.disconnect();
  });

  async function loadTemplates() {
    if (isLoading) return;
    isLoading = true;
    errorMessage = "";
    try {
      templates = await listReadyTemplates();
      hasLoaded = true;
    } catch (error) {
      errorMessage = extractErrorMessage(error);
    } finally {
      isLoading = false;
    }
  }

  async function useTemplate(template: ReadyTemplate) {
    importingId = template.id;
    errorMessage = "";
    try {
      const created = await api.createTemplate(
        await createTemplateFromCatalog(template),
      );
      await onimported();
      toast.success(m.settings_template_marketplace_import_success());
      await goto(`/${orgId}/templates/${created.id}`);
    } catch (error) {
      errorMessage = extractErrorMessage(error);
    } finally {
      importingId = null;
    }
  }
</script>

<section
  bind:this={sectionElement}
  class="mt-8 border-t border-border/60 pt-6"
  aria-labelledby="ready-templates-heading"
>
  <h2 id="ready-templates-heading" class="mb-4 text-base font-semibold">
    {m.settings_template_marketplace_title()}
  </h2>

  {#if hasLoaded && templates.length > 0}
    <div class="mb-4 max-w-sm">
      <Input
        bind:value={search}
        placeholder={m.settings_template_marketplace_search()}
        aria-label={m.settings_template_marketplace_search()}
      />
    </div>
  {/if}

  {#if errorMessage}
    <div
      class="flex flex-wrap items-center justify-between gap-3 rounded-md border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm text-destructive"
      role="alert"
    >
      <p>{errorMessage}</p>
      <Button
        variant="outline"
        size="sm"
        onclick={loadTemplates}
        disabled={isLoading}
      >
        {m.settings_template_marketplace_retry()}
      </Button>
    </div>
  {:else if isLoading}
    <p
      class="rounded-xl border border-border/60 py-8 text-center text-sm text-muted-foreground"
      aria-live="polite"
    >
      {m.settings_template_marketplace_loading()}
    </p>
  {:else if !hasLoaded}
    <div class="h-4" aria-hidden="true"></div>
  {:else if filteredTemplates.length === 0}
    <p class="rounded-xl border border-border/60 py-8 text-center text-sm text-muted-foreground">
      {m.settings_template_marketplace_empty()}
    </p>
  {:else}
    <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
      {#each filteredTemplates as template (template.id)}
        <article class="overflow-hidden rounded-xl border border-border/60 bg-card">
          {#if template.preview_url}
            <img
              src={template.preview_url}
              alt={l(template.name_ar, template.name)}
              class="h-36 w-full object-cover"
              loading="lazy"
              decoding="async"
            />
          {:else}
            <div class="flex h-36 items-center justify-center bg-muted/30 text-xs text-muted-foreground">
              {template.document_type}
            </div>
          {/if}
          <div class="flex items-center justify-between gap-3 p-3">
            <div class="min-w-0">
              <h3 class="truncate text-sm font-medium">
                {l(template.name_ar, template.name)}
              </h3>
              <p class="mt-0.5 truncate text-[11px] text-muted-foreground">
                {template.document_type}
              </p>
            </div>
            <Button
              size="sm"
              class="shrink-0"
              onclick={() => useTemplate(template)}
              disabled={importingId !== null}
            >
              {importingId === template.id
                ? m.settings_template_marketplace_loading()
                : m.settings_template_marketplace_get()}
            </Button>
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
