<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { goto } from "$app/navigation";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Badge } from "@orbis/ui/badge";
  import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
  import { api, extractErrorMessage } from "$lib/api";
  import { toast } from "@orbis/ui";
  import { l } from "$lib/core/i18n";
  import {
    createTemplateFromCatalog,
    isNetworkUnavailable,
    listReadyTemplates,
    type ReadyTemplate,
  } from "../marketplace-client";

  interface Props {
    open: boolean;
    orgId: string;
    onimported: () => Promise<void>;
  }

  let { open = $bindable(false), orgId, onimported }: Props = $props();

  let templates = $state<ReadyTemplate[]>([]);
  let isLoading = $state(false);
  let importingId = $state<string | null>(null);
  let search = $state("");
  let errorMessage = $state("");
  let networkUnavailable = $state(false);
  let retryOfflineAction = $state<(() => void) | null>(null);
  let hasLoaded = $state(false);
  let wasOpen = false;

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

  $effect(() => {
    if (open && !wasOpen) {
      wasOpen = true;
      if (!hasLoaded) void loadTemplates();
    } else if (!open) {
      wasOpen = false;
    }
  });

  async function loadTemplates() {
    if (isLoading) return;
    isLoading = true;
    errorMessage = "";
    networkUnavailable = false;
    retryOfflineAction = null;
    try {
      templates = await listReadyTemplates();
      hasLoaded = true;
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
      await onimported();
      toast.success(m.settings_template_marketplace_import_success());
      open = false;
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

<Sheet bind:open>
  <SheetContent
    side="end"
    contentClass="data-[side=end]:w-[min(96vw,64rem)] data-[side=end]:sm:!max-w-5xl"
    class="flex h-full flex-col gap-0 overflow-hidden p-0"
  >
    <SheetHeader class="shrink-0 border-b border-border/40 px-6 py-4 pe-14 text-start">
      <SheetTitle class="text-xl font-bold text-foreground">
        {m.settings_template_marketplace_title()}
      </SheetTitle>
    </SheetHeader>

    {#if hasLoaded && templates.length > 0}
      <div class="shrink-0 border-b border-border/40 px-6 py-4">
        <Input
          bind:value={search}
          placeholder={m.settings_template_marketplace_search()}
          aria-label={m.settings_template_marketplace_search()}
        />
      </div>
    {/if}

    <div class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide">
      {#if networkUnavailable}
        <div
          class="flex flex-col items-center gap-3 rounded-xl border border-border/50 bg-muted/20 px-4 py-8 text-center"
          role="status"
          aria-live="polite"
        >
          <p class="text-sm text-muted-foreground">
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
      {:else if hasLoaded && filteredTemplates.length === 0}
        <p class="rounded-xl border border-border/60 py-8 text-center text-sm text-muted-foreground">
          {m.settings_template_marketplace_empty()}
        </p>
      {:else if hasLoaded}
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each filteredTemplates as template (template.id)}
            <article class="flex min-w-0 flex-col overflow-hidden rounded-xl border border-border/50 bg-card">
              {#if template.preview_url}
                <img
                  src={template.preview_url}
                  alt={l(template.name_ar, template.name)}
                  class="h-40 w-full border-b border-border/40 bg-muted/20 object-cover"
                  loading="lazy"
                  decoding="async"
                />
              {:else}
                <div class="flex h-40 items-center justify-center border-b border-border/40 bg-muted/20 text-sm font-medium text-muted-foreground">
                  {template.document_type}
                </div>
              {/if}

              <div class="flex flex-1 flex-col gap-4 p-4">
                <div class="min-w-0">
                  <h3 class="truncate text-sm font-semibold text-foreground">
                    {l(template.name_ar, template.name)}
                  </h3>
                  {#if template.description_ar || template.description}
                    <p class="mt-1 line-clamp-2 text-xs text-muted-foreground">
                      {l(template.description_ar, template.description)}
                    </p>
                  {/if}
                </div>

                <div class="flex flex-wrap gap-1.5">
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
                  class="mt-auto w-full"
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
  </SheetContent>
</Sheet>
