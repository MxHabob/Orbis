<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, type IntegrationDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import {
  apps,
  categories,
  getAppsByCategory,
  type OrbisApp,
  getCategoryName,
  getCategoryNameAr,
} from "@orbis/app-store";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Card, CardContent, CardFooter } from "@orbis/ui/card";
import AppIcon from "$lib/features/ai/components/integrations/app-icon.svelte";
import CategoryFilter from "$lib/features/ai/components/integrations/category-filter.svelte";
import SearchField from "$lib/shared/components/search-field.svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import IntegrationConnectSheet from "../components/integration-connect-sheet.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";
import { Check, Bolt, Sparkles } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSaving = $state(false);
let activeCategory = $state("all");
let searchQuery = $state("");
let connectedIntegrations = $state<IntegrationDto[]>([]);
let selectedApp = $state<OrbisApp | null>(null);
let sheetOpen = $state(false);

const stepTasks = $derived(getTasksForStep("integrations", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const list = await api.listIntegrations().catch(() => []);
    connectedIntegrations = Array.isArray(list) ? list : [];
  } catch (err) {
    console.warn("Failed to load integrations:", err);
  } finally {
    isLoading = false;
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
    activeCategory === "all" ? apps : getAppsByCategory(activeCategory as any);

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
  sheetOpen = true;
}

async function handleAppConnected(providerName: string) {
  await loadData();
  await systemStore.loadOnboardingState(orgId);
}

async function handleAppDisconnected(providerName: string) {
  await loadData();
  await systemStore.loadOnboardingState(orgId);
}

async function handleContinue() {
  isSaving = true;
  try {
    await systemStore.loadOnboardingState(orgId);
    await navigateOnboardingRoute(`/${orgId}/onboarding/review`);
  } finally {
    isSaving = false;
  }
}
</script>

<div class="space-y-6">
  <!-- Onboarding Step Indicator & Title -->
  <OnboardingStepHeader
    stepNumber={9}
    title={m.onboarding_step_integrations_title()}
    description={m.onboarding_step_integrations_subtitle()}
    isRequired={false}
    status={isTaskComplete || connectedCount > 0 ? "completed" : "in_progress"}
  />

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
        <button
          type="button"
          onclick={() => openConnectSheet(app)}
          class="block h-full group text-start focus:outline-hidden cursor-pointer"
        >
          <Card class="h-full flex flex-col justify-between hover:border-foreground/30 transition-colors shadow-2xs">
            <CardContent class="p-5 flex flex-col flex-1 text-start">
              <!-- Top: Icon + Status Badges -->
              <div class="flex items-center justify-between gap-3 mb-4">
                <div class="w-12 h-7 flex items-center justify-start text-foreground shrink-0">
                  <AppIcon
                    icon={app.icon}
                    logo={app.logo}
                    size={24}
                    class="w-full h-full [&>svg]:max-w-[48px] [&>svg]:max-h-[26px]"
                  />
                </div>
                <div class="flex gap-1.5 flex-wrap justify-end">
                  {#if connected}
                    <Badge
                      variant="outline"
                      class="text-[10px] font-semibold text-emerald-600 dark:text-emerald-400 border-emerald-500/30 bg-emerald-500/10 px-2 py-0.5 flex items-center gap-1"
                    >
                      <span class="size-1.5 rounded-full bg-emerald-500"></span>
                      <span>{m.ai_center_integrations_status_connected()}</span>
                    </Badge>
                  {:else if app.status === "error"}
                    <Badge variant="destructive" class="text-[10px] font-semibold px-2 py-0.5">
                      {m.ai_center_integrations_status_error()}
                    </Badge>
                  {:else if app.beta}
                    <Badge variant="secondary" class="text-[10px] font-semibold px-2 py-0.5">
                      {m.ai_center_integrations_status_beta()}
                    </Badge>
                  {:else if !app.active}
                    <Badge variant="outline" class="text-[10px] font-medium text-muted-foreground bg-muted px-2 py-0.5">
                      {m.ai_center_integrations_status_coming_soon()}
                    </Badge>
                  {/if}
                </div>
              </div>

              <!-- Name -->
              <h3 class="text-sm font-semibold text-foreground mb-1.5 group-hover:text-primary transition-colors">
                {app.name}
              </h3>

              <!-- Description -->
              <p class="text-xs text-muted-foreground leading-relaxed flex-1 line-clamp-2">
                {l(app.short_description_ar, app.short_description)}
              </p>
            </CardContent>

            <!-- Footer: Category Tag & Configure Action -->
            <CardFooter class="py-2.5 px-5 flex items-center justify-between border-t border-border/30 bg-muted/10">
              <Badge variant="outline" class="text-[10px] font-mono font-medium text-muted-foreground border-border bg-muted/40">
                {l(getCategoryNameAr(app.category), getCategoryName(app.category))}
              </Badge>

              <span class="text-xs font-semibold text-primary group-hover:underline flex items-center gap-1">
                {#if connected}
                  <span>{m.settings_connection_settings()}</span>
                {:else}
                  <Bolt class="size-3" />
                  <span>{m.ui_integration_connect({ name: "" }).replace(/\s*$/, "") || "ربط"}</span>
                {/if}
              </span>
            </CardFooter>
          </Card>
        </button>
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

  <!-- Connection / Configuration Sheet matching Archive Preview Sheet -->
  <IntegrationConnectSheet
    bind:open={sheetOpen}
    app={selectedApp}
    {orgId}
    onconnected={handleAppConnected}
    ondisconnected={handleAppDisconnected}
  />

  <!-- Bottom Navigation Actions Bar -->
  <div class="mt-8 flex items-center justify-end gap-3 pb-8 pt-4 border-t border-border/40">
    <Button
      variant="default"
      disabled={isSaving}
      onclick={handleContinue}
      class="text-xs font-semibold shadow-xs cursor-pointer"
    >
      {#if isSaving}
        <LoadingIndicator size={14} class="size-3.5 me-1.5 text-white" />
      {/if}
      <span>{m.onboarding_step_integrations_continue_btn()}</span>
    </Button>
  </div>
</div>
