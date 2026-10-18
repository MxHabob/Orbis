<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import * as m from "$lib/paraglide/messages";
  import { api, extractErrorMessage } from "$lib/api";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Badge } from "@orbis/ui/badge";
  import { toast } from "@orbis/ui";
  import { l } from "$lib/core/i18n";
  import {
    createTemplateFromCatalog,
    isNetworkUnavailable,
    listReadyTemplates,
    type ReadyTemplate,
  } from "../marketplace-client";

  let { orgId }: { orgId: string } = $props();

  let templates = $state<ReadyTemplate[]>([]);
  let search = $state("");
  let isLoading = $state(true);
  let importingId = $state<string | null>(null);
  let errorMessage = $state("");
  let networkUnavailable = $state(false);
  let retryOfflineAction = $state<(() => void) | null>(null);

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
    void loadTemplates();
  });

  async function loadTemplates() {
    isLoading = true;
    errorMessage = "";
    networkUnavailable = false;
    retryOfflineAction = null;
    try {
      templates = await listReadyTemplates();
    } catch (error) {
      if (isNetworkUnavailable(error)) {
        networkUnavailable = true;
      } else {
        errorMessage = extractErrorMessage(error);
      }
    } finally {
      isLoading = false;
    }
  }

  function retry() {
    if (retryOfflineAction) {
      retryOfflineAction();
    } else {
      void loadTemplates();
    }
  }

  async function useTemplate(template: ReadyTemplate) {
    importingId = template.id;
    errorMessage = "";
    networkUnavailable = false;
    retryOfflineAction = null;
    try {
      const created = await api.createTemplate(
        await createTemplateFromCatalog(template),
      );
      toast.success(m.settings_template_marketplace_import_success());
      await goto(`/${orgId}/templates/${created.id}`);
    } catch (error) {
      if (isNetworkUnavailable(error)) {
        networkUnavailable = true;
        retryOfflineAction = () => {
          void useTemplate(template);
        };
      } else {
        errorMessage = extractErrorMessage(error);
      }
    } finally {
      importingId = null;
    }
  }
</script>

<div class="flex h-full min-h-0 flex-col">
  <div class="shrink-0 px-3 py-2">
    <Input
      type="search"
      placeholder={m.settings_template_marketplace_search()}
      aria-label={m.settings_template_marketplace_search()}
      bind:value={search}
      class="h-9 text-xs"
    />
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto px-3 pb-3 scrollbar-hide">
    {#if networkUnavailable}
      <div
        class="flex flex-col items-center gap-3 rounded-lg border border-border/50 bg-muted/20 p-4 text-center"
        role="status"
        aria-live="polite"
      >
        <p class="text-xs text-muted-foreground">
          {m.settings_template_marketplace_offline()}
        </p>
        <Button
          variant="outline"
          size="sm"
          onclick={retry}
          disabled={isLoading || importingId !== null}
        >
          {m.settings_template_marketplace_retry()}
        </Button>
      </div>
    {:else if errorMessage}
      <div
        class="flex flex-col items-center gap-3 rounded-lg border border-destructive/30 bg-destructive/5 p-4 text-center"
        role="alert"
      >
        <p class="text-xs text-destructive">{errorMessage}</p>
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
      <p class="py-8 text-center text-xs text-muted-foreground" aria-live="polite">
        {m.settings_template_marketplace_loading()}
      </p>
    {:else if filteredTemplates.length === 0}
      <p class="rounded-lg border border-border/50 px-3 py-8 text-center text-xs text-muted-foreground">
        {m.settings_template_marketplace_empty()}
      </p>
    {:else}
      <div class="flex flex-col gap-3">
        {#each filteredTemplates as template (template.id)}
          <article class="overflow-hidden rounded-lg border border-border/50 bg-background/40">
            {#if template.preview_url}
              <img
                src={template.preview_url}
                alt={l(template.name_ar, template.name)}
                class="h-36 w-full border-b border-border/40 bg-muted/20 object-cover"
                loading="lazy"
                decoding="async"
              />
            {:else}
              <div class="flex h-28 items-center justify-center border-b border-border/40 bg-muted/20 text-xs font-medium text-muted-foreground">
                {template.document_type}
              </div>
            {/if}

            <div class="flex flex-col gap-3 p-3">
              <div class="min-w-0">
                <h3 class="truncate text-xs font-semibold text-foreground">
                  {l(template.name_ar, template.name)}
                </h3>
                {#if template.description_ar || template.description}
                  <p class="mt-1 line-clamp-2 text-[11px] text-muted-foreground">
                    {l(template.description_ar, template.description)}
                  </p>
                {/if}
              </div>

              <div class="flex flex-wrap gap-1">
                <Badge variant="secondary" class="h-5 px-1.5 text-[10px] font-normal">
                  {template.document_type}
                </Badge>
                <Badge variant="secondary" class="h-5 px-1.5 text-[10px] font-normal">
                  {template.entity_type}
                </Badge>
                <Badge variant="secondary" class="h-5 px-1.5 text-[10px] font-normal">
                  {template.page_size}
                </Badge>
              </div>

              <Button
                size="sm"
                class="h-8 w-full text-xs"
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
  </div>
</div>
