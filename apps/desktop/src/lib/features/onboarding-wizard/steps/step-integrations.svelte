<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { createQuery } from "@tanstack/svelte-query";
import * as m from "$lib/paraglide/messages";
import { api, queryKeys } from "$lib/api";
import { effectiveContext, systemStore } from "$lib/stores";
import {
  apps,
  categories,
  getAppsByCategory,
  type AppCategory,
  type OrbisApp,
} from "@orbis/app-store";
import { Button } from "@orbis/ui/button";
import AppCard from "$lib/features/ai/components/integrations/app-card.svelte";
import CategoryFilter from "$lib/features/ai/components/integrations/category-filter.svelte";
import SearchField from "$lib/shared/components/search-field.svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import AppDetailSheet from "$lib/features/ai/components/integrations/app-detail-sheet.svelte";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import { deriveSteps } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { Check, Sparkles } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

let activeCategory = $state<"all" | AppCategory>("all");
let searchQuery = $state("");
let errorMessage = $state<string | null>(null);
let selectedApp = $state<OrbisApp | null>(null);
let detailSheetOpen = $state(false);
const integrationsQuery = createQuery(() => ({
  queryKey: queryKeys.integration.list(orgId),
  queryFn: () => api.listIntegrations(),
  enabled: Boolean(orgId),
  staleTime: 30 * 1000,
}));
const connectedIntegrations = $derived(integrationsQuery.data ?? []);
const isLoading = $derived(integrationsQuery.isLoading);

const stepStatus = $derived(
  deriveSteps(
    systemStore.onboardingTasks,
    (permission) => effectiveContext.hasPermission(permission),
  ).find((step) => step.id === "integrations")?.status ?? "not_started",
);

async function loadData() {
  errorMessage = null;
  try {
    const result = await integrationsQuery.refetch();
    if (result.isError) {
      errorMessage = m.common_action_failed();
    }
  } catch (error) {
    console.error("Failed to reload integrations:", error);
    errorMessage = error instanceof Error ? error.message : m.common_action_failed();
  }
}

function isAppConnected(app: OrbisApp): boolean {
  const provider = (app.adapter_name ?? app.id).toLowerCase();
  return connectedIntegrations.some(
    (item) =>
      item.provider_name.toLowerCase() === provider ||
      item.provider_name.toLowerCase() === app.id.toLowerCase()
  );
}

const filteredApps = $derived.by(() => {
  let result: OrbisApp[] =
    activeCategory === "all" ? apps : getAppsByCategory(activeCategory);

  if (searchQuery.trim()) {
    const q = searchQuery.toLowerCase().trim();
    result = result.filter(
      (app) =>
        app.name.toLowerCase().includes(q) ||
        (app.name_ar && app.name_ar.toLowerCase().includes(q)) ||
        app.id.toLowerCase().includes(q) ||
        app.short_description.toLowerCase().includes(q) ||
        (app.short_description_ar && app.short_description_ar.toLowerCase().includes(q))
    );
  }

  return result;
});

const connectedCount = $derived(
  apps.filter((app) => isAppConnected(app)).length
);

function openConnectSheet(app: OrbisApp) {
  selectedApp = app;
  detailSheetOpen = true;
}

function refreshAfterIntegrationChange() {
  void loadData();
  void refreshOnboardingState(orgId, m.common_action_failed()).catch((error) => {
    console.error("Failed to refresh onboarding state after integration changes:", error);
    errorMessage = error instanceof Error ? error.message : m.common_action_failed();
  });
}
</script>

<div class="space-y-6">
  <!-- Onboarding Step Indicator & Title -->
  <OnboardingStepHeader
    stepNumber={9}
    title={m.onboarding_step_integrations_title()}
    description={m.onboarding_step_integrations_subtitle()}
    isRequired={false}
    status={stepStatus}
  />

  {#if errorMessage || integrationsQuery.isError}
    <div class="flex items-center justify-between gap-3 rounded-xl border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive" role="alert">
      <span>{errorMessage ?? m.common_action_failed()}</span>
      <Button variant="outline" size="sm" onclick={loadData}>
        {m.onboarding_retry()}
      </Button>
    </div>
  {/if}

  <!-- Integrations Gallery Header with Centered Search -->
  <div class="space-y-4 pt-1">
    <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
      <!-- Connected Services Metric Pill -->
      <div class="flex items-center gap-2">
        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20">
          <Sparkles class="size-3.5" />
          <span>{connectedCount} {m.common_account_identities_connected()}</span>
        </span>
      </div>

      <!-- Centered Search Field -->
      <div class="w-full sm:max-w-md">
        <SearchField
          bind:value={searchQuery}
          placeholder={m.ai_center_integrations_search_placeholder()}
        />
      </div>
    </div>

    <!-- Category Pills Bar -->
    <div class="flex justify-start sm:justify-center overflow-x-auto py-1 scrollbar-hide">
      <CategoryFilter {categories} bind:active={activeCategory} />
    </div>
  </div>

  <!-- Apps Grid matching IntegrationsGallery -->
  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.common_control_loading()}</span>
    </div>
  {:else if filteredApps.length > 0}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      {#each filteredApps as app (app.id)}
        {@const connected = isAppConnected(app)}
        <AppCard
          {app}
          {connected}
          actionLabel={connected ? m.settings_connection_settings() : m.common_action_connect()}
          onselect={() => openConnectSheet(app)}
        />
      {/each}
    </div>
  {:else}
    <AiEmptyState
      text={searchQuery
        ? m.ai_ui_integrations_search_empty({ query: searchQuery })
        : m.ai_ui_integrations_category_empty()}
      actionLabel={searchQuery ? m.ai_ui_reset_filters_search() : m.ai_ui_view_all_integrations()}
      onaction={() => {
        searchQuery = "";
        activeCategory = "all";
      }}
      hat="random"
    />
  {/if}

</div>

{#if selectedApp}
  <AppDetailSheet
    bind:open={detailSheetOpen}
    {orgId}
    app={selectedApp}
    onChange={refreshAfterIntegrationChange}
    onclose={() => {
      detailSheetOpen = false;
      selectedApp = null;
      refreshAfterIntegrationChange();
    }}
  />
{/if}
