<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardFooter,
} from "@orbis/ui/card";
import { Refresh } from "@orbis/icons";
import {
  loadModelCatalog,
  modelCatalog,
  refreshModelCatalog,
} from "$lib/features/ai/model-catalog.svelte";

let isRefreshing = $state(false);
let errorMessage = $state("");

onMount(() => {
  loadModelCatalog().catch((error: unknown) => {
    errorMessage = error instanceof Error ? error.message : String(error);
    console.error("Failed to load the cached AI model catalog", error);
  });
});

const cachedModelCount = $derived(
  Object.values(modelCatalog.providers).reduce(
    (count, options) => count + options.length,
    0,
  ),
);

async function handleRefresh() {
  isRefreshing = true;
  errorMessage = "";
  try {
    const result = await refreshModelCatalog();
    toast.success(
      m.settings_static_data_updated({
        providers: result.providerCount,
        models: result.modelCount,
      }),
    );
  } catch (error) {
    errorMessage = error instanceof Error ? error.message : String(error);
    console.error("Failed to refresh the AI model catalog", error);
    toast.error(m.settings_static_data_update_failed());
  } finally {
    isRefreshing = false;
  }
}
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_static_data_title()}</CardTitle>
  </CardHeader>
  <CardContent class="px-6 py-5">
    <p class="text-xs leading-relaxed text-muted-foreground">
      {m.settings_static_data_description()}
    </p>
  </CardContent>
  <CardFooter class="flex flex-wrap items-center justify-between gap-3">
    <div class="min-w-0 space-y-1">
      {#if modelCatalog.updatedAt}
        <p class="text-[11px] text-muted-foreground">
          {m.settings_static_data_cache_status({
            models: cachedModelCount,
            date: new Date(modelCatalog.updatedAt).toLocaleString(),
          })}
        </p>
      {/if}
      {#if errorMessage}
        <p class="text-xs text-destructive" role="alert">
          {m.settings_static_data_cache_error({ error: errorMessage })}
        </p>
      {/if}
    </div>
    <Button
      type="button"
      variant="outline"
      size="sm"
      class="h-8 shrink-0 px-4 text-xs font-semibold"
      disabled={isRefreshing}
      onclick={handleRefresh}
    >
      {#if isRefreshing}
        <Refresh size={14} class="me-1 animate-spin" />
        {m.settings_static_data_refreshing()}
      {:else}
        <Refresh size={14} class="me-1" />
        {m.settings_static_data_refresh()}
      {/if}
    </Button>
  </CardFooter>
</Card>
