<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { api, queryClient, queryKeys } from "$lib/api";
import type { MulverseDeploymentDto, MulverseEndpointDto } from "$lib/api";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { ArchiveBox, Cloud, Database, Merge, Sparkles } from "@orbis/icons";
import { resolve } from "$app/paths";
import { localizeHref } from "$lib/paraglide/runtime";

function showMutationError(error: unknown) {
  toast.error(error instanceof Error ? error.message : String(error));
}

let { data }: { data: { orgId: string } } = $props();
let apiKey = $state("");
let deploymentName = $state("");
let deploymentEnvironment = $state("production");
let editingDeploymentName = $state("");
let editingDeploymentEnvironment = $state("");
let selectedDeploymentId = $state<string | null>(null);
let editingDeploymentId = $state<string | null>(null);
let editingEndpointId = $state<string | null>(null);
let endpointUrl = $state("");
let endpointScope = $state<"public" | "local_network">("public");
let newEndpointUrl = $state("");

const connection = createQuery(() => ({
  queryKey: ["mulverseConnection", data.orgId],
  queryFn: () => api.getMulverseConnection(),
  retry: false,
}));

const integrations = createQuery(() => ({
  queryKey: queryKeys.integration.list(data.orgId),
  queryFn: () => api.listIntegrations(),
  retry: false,
}));

function isIntegrationEnabled(...providerNames: string[]): boolean {
  const normalizedNames = providerNames.map((name) =>
    name.toLowerCase().replaceAll("_", "-"),
  );
  return (
    integrations.data?.some(
      (integration) =>
        integration.is_enabled &&
        normalizedNames.includes(
          integration.provider_name.toLowerCase().replaceAll("_", "-"),
        ),
    ) ?? false
  );
}

const orbisAiEnabled = $derived(isIntegrationEnabled("orbis-ai"));
const orbisMobileEnabled = $derived(
  isIntegrationEnabled("orbis-mobile") || connection.data?.connected === true,
);
const mulverseCloudEnabled = $derived(isIntegrationEnabled("mulverse-cloud"));

const deployments = createQuery(() => ({
  queryKey: ["mulverseDeployments", data.orgId],
  queryFn: () => api.listMulverseDeployments(),
  enabled: connection.data?.connected === true,
}));

const endpoints = createQuery(() => ({
  queryKey: ["mulverseEndpoints", data.orgId, selectedDeploymentId],
  queryFn: () => api.listMulverseEndpoints(selectedDeploymentId!),
  enabled: !!selectedDeploymentId,
}));

const connectMutation = createMutation(() => ({
  mutationFn: () => api.connectMulverse(apiKey.trim()),
  onSuccess: async () => {
    apiKey = "";
    await queryClient.invalidateQueries({
      queryKey: ["mulverseConnection", data.orgId],
    });
    toast.success(m.settings_orbis_mobile_toast_connected());
  },
  onError: showMutationError,
}));

const disconnectMutation = createMutation(() => ({
  mutationFn: () => api.disconnectMulverse(),
  onSuccess: async () => {
    selectedDeploymentId = null;
    await queryClient.invalidateQueries({
      queryKey: ["mulverseConnection", data.orgId],
    });
    await queryClient.invalidateQueries({
      queryKey: ["mulverseDeployments", data.orgId],
    });
    toast.success(m.settings_orbis_mobile_toast_disconnected());
  },
  onError: showMutationError,
}));

const createDeploymentMutation = createMutation(() => ({
  mutationFn: () =>
    api.createMulverseDeployment({
      name: deploymentName.trim(),
      environment: deploymentEnvironment.trim(),
    }),
  onSuccess: async () => {
    deploymentName = "";
    await queryClient.invalidateQueries({
      queryKey: ["mulverseDeployments", data.orgId],
    });
    toast.success(m.settings_orbis_mobile_toast_deployment_created());
  },
  onError: showMutationError,
}));

const updateDeploymentMutation = createMutation(() => ({
  mutationFn: (deployment: MulverseDeploymentDto) =>
    api.updateMulverseDeployment(deployment.id, {
      name: editingDeploymentName.trim(),
      environment: editingDeploymentEnvironment.trim(),
    }),
  onSuccess: async () => {
    editingDeploymentId = null;
    await queryClient.invalidateQueries({
      queryKey: ["mulverseDeployments", data.orgId],
    });
    toast.success(m.settings_orbis_mobile_toast_deployment_updated());
  },
  onError: showMutationError,
}));

const createEndpointMutation = createMutation(() => ({
  mutationFn: () =>
    api.createMulverseEndpoint(selectedDeploymentId!, {
      url: newEndpointUrl.trim(),
      network_scope: endpointScope,
    }),
  onSuccess: async () => {
    newEndpointUrl = "";
    await queryClient.invalidateQueries({
      queryKey: ["mulverseEndpoints", data.orgId, selectedDeploymentId],
    });
    toast.success(m.settings_orbis_mobile_toast_endpoint_added());
  },
  onError: showMutationError,
}));

const updateEndpointMutation = createMutation(() => ({
  mutationFn: ({
    endpoint,
    status,
    url,
  }: {
    endpoint: MulverseEndpointDto;
    status?: "active" | "disabled";
    url?: string;
  }) =>
    api.updateMulverseEndpoint(endpoint.id, {
      url,
      status,
    }),
  onSuccess: async (_result, variables) => {
    editingEndpointId = null;
    endpointUrl = "";
    await queryClient.invalidateQueries({
      queryKey: ["mulverseEndpoints", data.orgId, selectedDeploymentId],
    });
    if (variables.status === "active") {
      toast.success(m.settings_orbis_mobile_toast_endpoint_activated());
    } else if (variables.status === "disabled") {
      toast.success(m.settings_orbis_mobile_toast_endpoint_disabled());
    } else {
      toast.success(m.settings_orbis_mobile_toast_endpoint_updated());
    }
  },
  onError: showMutationError,
}));

function submitConnect(event: SubmitEvent) {
  event.preventDefault();
  connectMutation.mutate();
}

function submitDeployment(event: SubmitEvent) {
  event.preventDefault();
  createDeploymentMutation.mutate();
}

function submitEndpoint(event: SubmitEvent) {
  event.preventDefault();
  createEndpointMutation.mutate();
}

function saveDeployment(event: SubmitEvent, deployment: MulverseDeploymentDto) {
  event.preventDefault();
  updateDeploymentMutation.mutate(deployment);
}

function saveEndpoint(event: SubmitEvent, endpoint: MulverseEndpointDto) {
  event.preventDefault();
  const nextUrl = endpointUrl.trim();
  if (nextUrl && nextUrl !== endpoint.url) {
    updateEndpointMutation.mutate({ endpoint, url: nextUrl });
  } else {
    editingEndpointId = null;
    endpointUrl = "";
  }
}

function statusMessage(status: string): string {
  switch (status) {
    case "active":
      return m.settings_orbis_mobile_status_active();
    case "disabled":
      return m.settings_orbis_mobile_status_disabled();
    case "revoked":
      return m.settings_orbis_mobile_status_revoked();
    case "pending":
      return m.settings_orbis_mobile_status_pending();
    case "suspended":
      return m.settings_orbis_mobile_status_suspended();
    default:
      return status;
  }
}
</script>

<svelte:head>
  <title>{m.settings_mulverse_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background">
  <div class="mx-auto flex w-full max-w-7xl flex-col gap-7 px-5 py-7 lg:px-8">
    <header class="flex flex-col justify-between gap-5 border-b border-border pb-6 sm:flex-row sm:items-center">
      <div class="min-w-0">
        <h1 class="text-2xl font-semibold tracking-tight text-foreground">{m.settings_mulverse_title()}</h1>
        <p class="mt-2 max-w-3xl text-sm leading-6 text-muted-foreground">
          {m.settings_mulverse_description()}
        </p>
      </div>
    </header>

    <section class="space-y-4" aria-labelledby="mulverse-products-heading">
      <div>
        <h2 id="mulverse-products-heading" class="text-lg font-semibold tracking-tight text-foreground">
          {m.settings_mulverse_products_title()}
        </h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
          {m.settings_mulverse_products_description()}
        </p>
      </div>

      {#if integrations.isPending}
        <div class="grid gap-4 md:grid-cols-3" aria-label={m.common_loading()}>
          {#each [0, 1, 2] as item (item)}
            <Card class="h-32 animate-pulse border-border/50 bg-muted/40" />
          {/each}
        </div>
      {:else if integrations.isError}
        <Card class="flex-row flex-wrap items-center justify-between gap-4 border-destructive/30 bg-destructive/5 p-4" role="alert">
          <p class="text-sm text-destructive">{integrations.error.message}</p>
          <Button variant="outline" size="sm" onclick={() => integrations.refetch()}>
            {m.common_retry()}
          </Button>
        </Card>
      {:else if orbisAiEnabled || orbisMobileEnabled || mulverseCloudEnabled}
        <div class="grid gap-4 md:grid-cols-3">
          {#if orbisAiEnabled}
            <a href={resolve(localizeHref(`/${data.orgId}/ai-center/integrations`))} class="rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
              <Card class="h-full gap-3 border-border/50 bg-card/60 p-5 shadow-xs transition-colors hover:border-primary/40">
                <div class="flex items-center gap-3">
                  <Sparkles size={18} class="shrink-0 text-primary" />
                  <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_orbis_ai()}</h3>
                </div>
                <Badge variant="success" class="w-fit">{m.settings_mulverse_enabled()}</Badge>
              </Card>
            </a>
          {/if}
          {#if orbisMobileEnabled}
            <a href="#mulverse-mobile-settings" class="rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
              <Card class="h-full gap-3 border-border/50 bg-card/60 p-5 shadow-xs transition-colors hover:border-primary/40">
                <div class="flex items-center gap-3">
                  <Merge size={18} class="shrink-0 text-primary" />
                  <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_orbis_mobile()}</h3>
                </div>
                <Badge variant="success" class="w-fit">{m.settings_mulverse_enabled()}</Badge>
              </Card>
            </a>
          {/if}
          {#if mulverseCloudEnabled}
            <a href={resolve(localizeHref(`/${data.orgId}/ai-center/integrations`))} class="rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
              <Card class="h-full gap-3 border-border/50 bg-card/60 p-5 shadow-xs transition-colors hover:border-primary/40">
                <div class="flex items-center gap-3">
                  <Cloud size={18} class="shrink-0 text-primary" />
                  <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_cloud()}</h3>
                </div>
                <Badge variant="success" class="w-fit">{m.settings_mulverse_enabled()}</Badge>
              </Card>
            </a>
          {/if}
        </div>
      {:else}
        <Card class="flex flex-col items-start gap-3 border-dashed border-border bg-muted/20 p-5 sm:flex-row sm:items-center sm:justify-between">
          <p class="max-w-2xl text-sm leading-6 text-muted-foreground">{m.settings_mulverse_no_products()}</p>
          <Button href={localizeHref(`/${data.orgId}/ai-center/integrations`)} variant="outline" size="sm" class="shrink-0">
            {m.settings_mulverse_browse_integrations()}
          </Button>
        </Card>
      {/if}
    </section>

    <section class="space-y-4" aria-labelledby="mulverse-upcoming-heading">
      <div>
        <h2 id="mulverse-upcoming-heading" class="text-lg font-semibold tracking-tight text-foreground">
          {m.settings_mulverse_upcoming_title()}
        </h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
          {m.settings_mulverse_upcoming_description()}
        </p>
      </div>

      <div class="grid gap-4 md:grid-cols-3">
        <Card class="gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
          <div class="flex items-start gap-3">
            <Database size={19} class="mt-0.5 shrink-0 text-muted-foreground" />
            <div>
              <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_database_title()}</h3>
              <p class="mt-1 text-xs leading-5 text-muted-foreground">{m.settings_mulverse_database_description()}</p>
            </div>
          </div>
          <Badge variant="secondary" class="mt-auto w-fit">{m.settings_mulverse_coming_soon()}</Badge>
        </Card>

        <Card class="gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
          <div class="flex items-start gap-3">
            <ArchiveBox size={19} class="mt-0.5 shrink-0 text-muted-foreground" />
            <div>
              <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_backups_title()}</h3>
              <p class="mt-1 text-xs leading-5 text-muted-foreground">{m.settings_mulverse_backups_description()}</p>
            </div>
          </div>
          <Badge variant="secondary" class="mt-auto w-fit">{m.settings_mulverse_coming_soon()}</Badge>
        </Card>

        <Card class="gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
          <div class="flex items-start gap-3">
            <Cloud size={19} class="mt-0.5 shrink-0 text-muted-foreground" />
            <div>
              <h3 class="text-sm font-semibold text-foreground">{m.settings_mulverse_storage_title()}</h3>
              <p class="mt-1 text-xs leading-5 text-muted-foreground">{m.settings_mulverse_storage_description()}</p>
            </div>
          </div>
          <Badge variant="secondary" class="mt-auto w-fit">{m.settings_mulverse_coming_soon()}</Badge>
        </Card>
      </div>
    </section>

    {#if orbisMobileEnabled && connection.isPending}
      <div class="grid items-start gap-5 lg:grid-cols-[minmax(0,1fr)_19rem]" aria-label={m.common_loading()}>
        <main id="mulverse-mobile-settings" class="min-w-0 space-y-5">
          <Card class="gap-0 border-border/50 bg-card/60 shadow-xs">
            <div class="space-y-2 border-b border-border p-5 sm:p-6">
              <Skeleton class="h-4 w-2/5 rounded-sm" />
              <Skeleton class="h-3 w-4/5 rounded-sm" />
              <Skeleton class="h-3 w-3/5 rounded-sm" />
            </div>
            <div class="flex flex-col gap-3 p-5 sm:flex-row sm:p-6">
              <Skeleton class="h-11 min-w-0 flex-1 rounded-md" />
              <Skeleton class="h-11 w-full rounded-md sm:w-28" />
            </div>
          </Card>

          <Card class="gap-0 border-border/50 bg-card/60 shadow-xs">
            <div class="space-y-2 border-b border-border p-5 sm:p-6">
              <Skeleton class="h-4 w-1/3 rounded-sm" />
              <Skeleton class="h-3 w-3/4 rounded-sm" />
            </div>
            <div class="grid gap-3 border-b border-border bg-muted/20 p-5 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto] sm:p-6">
              <Skeleton class="h-11 rounded-md" />
              <Skeleton class="h-11 rounded-md" />
              <Skeleton class="h-11 w-full rounded-md sm:w-32" />
            </div>
            <div class="space-y-2 p-4 sm:p-5">
              {#each [0, 1] as row (row)}
                <div class="flex items-center gap-3 rounded-md border border-border bg-background p-3 sm:p-4">
                  <div class="min-w-0 flex-1 space-y-2">
                    <Skeleton class="h-3.5 w-2/5 rounded-sm" />
                    <Skeleton class="h-3 w-1/4 rounded-sm" />
                  </div>
                  <Skeleton class="h-6 w-16 rounded-full" />
                  <Skeleton class="h-8 w-14 rounded-md" />
                </div>
              {/each}
            </div>
          </Card>
        </main>

        <aside class="space-y-5">
          <Card class="gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
            <Skeleton class="h-3.5 w-2/3 rounded-sm" />
            <Skeleton class="h-3 w-full rounded-sm" />
            <Skeleton class="h-3 w-4/5 rounded-sm" />
            <div class="space-y-4 pt-2">
              {#each [0, 1, 2] as step (step)}
                <div class="flex items-start gap-3">
                  <Skeleton class="size-6 shrink-0 rounded-full" />
                  <div class="flex-1 space-y-2 pt-1">
                    <Skeleton class="h-3 w-full rounded-sm" />
                    <Skeleton class="h-3 w-4/5 rounded-sm" />
                  </div>
                </div>
              {/each}
            </div>
          </Card>
          <Card class="gap-2 border-primary/20 bg-primary/[0.04] p-5">
            <Skeleton class="h-3 w-full rounded-sm" />
            <Skeleton class="h-3 w-5/6 rounded-sm" />
          </Card>
        </aside>
      </div>
    {:else if orbisMobileEnabled && connection.isError}
      <Card class="gap-4 border-destructive/30 bg-card p-5 shadow-xs">
        <p class="text-sm text-destructive">{connection.error.message}</p>
        <Button variant="outline" size="lg" onclick={() => connection.refetch()}>
          {m.common_retry()}
        </Button>
      </Card>
    {:else if orbisMobileEnabled}
      <div class="grid items-start gap-5 lg:grid-cols-[minmax(0,1fr)_19rem]">
        <main id="mulverse-mobile-settings" class="min-w-0 space-y-5">
          {#if !connection.data?.connected}
            <Card class="border-border/50 bg-card/60 shadow-xs">
              <div class="flex items-start gap-4 border-b border-border p-5 sm:p-6">
                <div>
                  <p class="mt-1 max-w-2xl text-sm leading-6 text-muted-foreground">
                    {m.settings_orbis_mobile_connect_description()}
                  </p>
                </div>
              </div>
              <form class="flex flex-col gap-3 p-5 sm:flex-row sm:p-6" onsubmit={submitConnect}>
                <label class="sr-only" for="mulverse-key">{m.settings_orbis_mobile_api_key()}</label>
                <Input
                  id="mulverse-key"
                  type="password"
                  autocomplete="off"
                  bind:value={apiKey}
                  placeholder={m.settings_orbis_mobile_api_key()}
                  class="h-11 min-h-11 min-w-0 flex-1 border-border bg-background text-sm text-foreground"
                  required
                />
                <Button
                  type="submit"
                  disabled={!apiKey.trim() || connectMutation.isPending}
                  class="h-11 min-h-11 px-5 text-sm font-semibold"
                >
                  {connectMutation.isPending ? m.common_loading() : m.settings_orbis_mobile_connect()}
                </Button>
              </form>
            </Card>
          {:else}
            <Card class="flex-row flex-wrap items-center justify-between gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
              <div class="flex min-w-0 items-center gap-3">
                <div class="min-w-0">
                  <p class="mt-1 truncate font-mono text-xs text-muted-foreground" dir="ltr">
                    {connection.data.customer_id}
                  </p>
                </div>
              </div>
              <Button
                variant="outline"
                size="lg"
                class="min-h-10 text-sm"
                disabled={disconnectMutation.isPending}
                onclick={() => disconnectMutation.mutate()}
              >
                {disconnectMutation.isPending ? m.common_loading() : m.settings_orbis_mobile_disconnect()}
              </Button>
            </Card>

            <Card class="border-border/50 bg-card/60 shadow-xs">
              <div class="flex flex-col gap-4 border-b border-border p-5 sm:flex-row sm:items-start sm:justify-between sm:p-6">
                <div class="flex items-start gap-3">
                  <div>
                    <p class="mt-1 max-w-xl text-sm leading-6 text-muted-foreground">{m.settings_orbis_mobile_deployments_description()}</p>
                  </div>
                </div>
              </div>

              <form class="grid gap-3 border-b border-border bg-muted/20 p-5 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto] sm:p-6" onsubmit={submitDeployment}>
                <label class="sr-only" for="deployment-name">{m.settings_orbis_mobile_deployment_name()}</label>
                <Input
                  id="deployment-name"
                  bind:value={deploymentName}
                  placeholder={m.settings_orbis_mobile_deployment_name()}
                  class="h-11 min-h-11 border-border bg-background text-sm text-foreground"
                  required
                />
                <label class="sr-only" for="deployment-environment">{m.settings_orbis_mobile_environment()}</label>
                <Input
                  id="deployment-environment"
                  bind:value={deploymentEnvironment}
                  placeholder={m.settings_orbis_mobile_environment()}
                  class="h-11 min-h-11 border-border bg-background text-sm text-foreground"
                  required
                />
                <Button
                  type="submit"
                  disabled={!deploymentName.trim() || !deploymentEnvironment.trim() || createDeploymentMutation.isPending}
                  class="h-11 min-h-11 px-4 text-sm font-semibold"
                >
                  {m.settings_orbis_mobile_add_deployment()}
                </Button>
              </form>

              {#if deployments.isPending}
                <div class="space-y-3 p-5" aria-label={m.common_loading()}>
                  <div class="h-16 animate-pulse rounded-lg bg-muted"></div>
                  <div class="h-16 animate-pulse rounded-lg bg-muted"></div>
                </div>
              {:else if deployments.isError}
                <div class="space-y-3 p-5 text-sm text-destructive">
                  <p>{deployments.error.message}</p>
                  <Button variant="outline" onclick={() => deployments.refetch()}>
                    {m.common_retry()}
                  </Button>
                </div>
              {:else if deployments.data?.length === 0}
                <div class="flex flex-col items-center px-6 py-12 text-center">
                  <p class="max-w-sm text-sm leading-6 text-muted-foreground">{m.settings_orbis_mobile_empty_deployments()}</p>
                </div>
              {:else}
                <div class="space-y-2 p-4 sm:p-5">
                  {#each deployments.data ?? [] as deployment (deployment.id)}
                    <article class={`overflow-hidden rounded-lg border transition-colors ${selectedDeploymentId === deployment.id ? "border-primary/40 bg-primary/[0.03]" : "border-border bg-background hover:border-muted-foreground/30"}`}>
                      <div class="flex flex-wrap items-center gap-3 p-3 sm:p-4">
                        <Button
                          variant="ghost"
                          class="h-auto min-h-9 min-w-0 flex-1 justify-start px-0 text-start"
                          aria-expanded={selectedDeploymentId === deployment.id}
                          onclick={() => (selectedDeploymentId = selectedDeploymentId === deployment.id ? null : deployment.id)}
                        >
                          <span class="min-w-0">
                            <span class="block truncate text-sm font-semibold text-foreground">{deployment.name}</span>
                            <span class="mt-1 block text-xs text-muted-foreground">{deployment.environment}</span>
                          </span>
                        </Button>
                        <Badge variant={deployment.status === "active" ? "success" : "outline"} class="px-2.5 py-1 text-xs">
                          {statusMessage(deployment.status)}
                        </Badge>
                        <Button
                          variant="outline"
                          size="sm"
                          class="min-h-9 text-xs"
                          aria-label={`${m.common_action_edit()} ${deployment.name}`}
                          onclick={() => {
                            editingDeploymentId = editingDeploymentId === deployment.id ? null : deployment.id;
                            editingDeploymentName = deployment.name;
                            editingDeploymentEnvironment = deployment.environment;
                          }}
                        >
                          {m.common_action_edit()}
                        </Button>
                      </div>

                      {#if editingDeploymentId === deployment.id}
                        <form class="grid gap-2 border-t border-border bg-muted/20 p-3 sm:grid-cols-[1fr_1fr_auto]" onsubmit={(event) => saveDeployment(event, deployment)}>
                          <Input bind:value={editingDeploymentName} aria-label={m.settings_orbis_mobile_deployment_name()} class="h-10 min-h-10 border-border bg-background text-sm text-foreground" required />
                          <Input bind:value={editingDeploymentEnvironment} aria-label={m.settings_orbis_mobile_environment()} class="h-10 min-h-10 border-border bg-background text-sm text-foreground" required />
                          <Button type="submit" disabled={updateDeploymentMutation.isPending} class="h-10 min-h-10 px-4">{m.common_action_save()}</Button>
                        </form>
                      {/if}

                      {#if selectedDeploymentId === deployment.id}
                        <div class="space-y-4 border-t border-border bg-muted/20 p-4 sm:p-5">
                          {#if endpoints.isPending}
                            <div class="h-14 animate-pulse rounded-md bg-muted"></div>
                          {:else if endpoints.isError}
                            <div class="space-y-2 text-sm text-destructive">
                              <p>{endpoints.error.message}</p>
                              <Button variant="outline" onclick={() => endpoints.refetch()}>
                                {m.common_retry()}
                              </Button>
                            </div>
                          {:else if endpoints.data?.length === 0}
                            <p class="rounded-md border border-dashed border-border px-4 py-5 text-center text-sm text-muted-foreground">{m.settings_orbis_mobile_empty_endpoints()}</p>
                          {:else}
                            <div class="space-y-2">
                              {#each endpoints.data ?? [] as endpoint (endpoint.id)}
                                <div class="flex flex-wrap items-center gap-3 rounded-md border border-border bg-background p-3">
                                  <div class="min-w-0 flex-1">
                                    <p class="break-all font-mono text-xs leading-5 text-foreground" dir="ltr">{endpoint.url}</p>
                                    <p class="mt-1 text-xs text-muted-foreground">
                                      {endpoint.network_scope === "public" ? m.settings_orbis_mobile_public_endpoint() : m.settings_orbis_mobile_local_endpoint()} · {statusMessage(endpoint.status)}
                                    </p>
                                  </div>
                                  {#if endpoint.status !== "revoked"}
                                    {#if editingEndpointId === endpoint.id}
                                      <form class="flex w-full gap-2" onsubmit={(event) => saveEndpoint(event, endpoint)}>
                                        <Input bind:value={endpointUrl} placeholder={endpoint.url} aria-label={m.settings_orbis_mobile_endpoint_url()} class="h-10 min-h-10 min-w-0 flex-1 border-border bg-background text-sm text-foreground" />
                                        <Button type="submit" disabled={updateEndpointMutation.isPending} class="h-10 min-h-10 px-3">{m.common_action_save()}</Button>
                                      </form>
                                    {:else}
                                      <Button variant="outline" size="sm" class="min-h-9 text-xs" onclick={() => { editingEndpointId = endpoint.id; endpointUrl = ""; }}>
                                        {m.settings_orbis_mobile_edit_endpoint()}
                                      </Button>
                                      {#if endpoint.status === "disabled"}
                                        <Button variant="outline" size="sm" class="min-h-9 text-xs" disabled={updateEndpointMutation.isPending} onclick={() => updateEndpointMutation.mutate({ endpoint, status: "active" })}>
                                          {m.settings_orbis_mobile_activate()}
                                        </Button>
                                      {:else}
                                        <Button variant="outline" size="sm" class="min-h-9 text-xs" disabled={updateEndpointMutation.isPending} onclick={() => updateEndpointMutation.mutate({ endpoint, status: "disabled" })}>
                                          {m.settings_orbis_mobile_disable()}
                                        </Button>
                                      {/if}
                                    {/if}
                                  {/if}
                                </div>
                              {/each}
                            </div>
                          {/if}
                          <form class="grid gap-2 border-t border-border pt-4 sm:grid-cols-[minmax(0,1fr)_auto_auto]" onsubmit={submitEndpoint}>
                            <label class="sr-only" for="endpoint-url">{m.settings_orbis_mobile_endpoint_url()}</label>
                            <Input id="endpoint-url" bind:value={newEndpointUrl} placeholder={m.settings_orbis_mobile_endpoint_url()} class="h-11 min-h-11 min-w-0 border-border bg-background text-sm text-foreground" required />
                            <label class="sr-only" for="endpoint-scope">{m.settings_orbis_mobile_network_scope()}</label>
                            <Select type="single" bind:value={endpointScope}>
                              <SelectTrigger id="endpoint-scope" class="h-11 w-full rounded-md border-border bg-background text-sm">
                                {endpointScope === "public" ? m.settings_orbis_mobile_public_endpoint() : m.settings_orbis_mobile_local_endpoint()}
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="public">{m.settings_orbis_mobile_public_endpoint()}</SelectItem>
                                <SelectItem value="local_network">{m.settings_orbis_mobile_local_endpoint()}</SelectItem>
                              </SelectContent>
                            </Select>
                            <Button type="submit" disabled={!newEndpointUrl.trim() || createEndpointMutation.isPending} class="h-11 min-h-11 px-4 text-sm font-semibold">
                              {m.settings_orbis_mobile_add_endpoint()}
                            </Button>
                          </form>
                        </div>
                      {/if}
                    </article>
                  {/each}
                </div>
              {/if}
            </Card>
          {/if}
        </main>

        <aside class="space-y-5">
          {#if connection.data?.connected}
            <Card class="gap-4 border-border/50 bg-card/60 p-5 shadow-xs">
              <p class="text-sm leading-6 text-muted-foreground">{m.settings_orbis_mobile_directory_description()}</p>
              <ol class="mt-5 space-y-4">
                {#each [m.settings_orbis_mobile_directory_step_one(), m.settings_orbis_mobile_directory_step_two(), m.settings_orbis_mobile_directory_step_three()] as step, index (index)}
                  <li class="flex gap-3">
                    <span class="flex size-6 shrink-0 items-center justify-center rounded-full border border-border bg-background text-[11px] font-semibold text-muted-foreground">{index + 1}</span>
                    <span class="pt-0.5 text-xs leading-5 text-muted-foreground">{step}</span>
                  </li>
                {/each}
              </ol>
              <p class="mt-5 border-t border-border pt-4 text-xs leading-5 text-muted-foreground">{m.settings_orbis_mobile_staff_actions()}</p>
            </Card>
          {:else}
            <Card class="gap-2 border-border/50 bg-card/60 p-5 shadow-xs">
              <p class="text-sm leading-6 text-muted-foreground">{m.settings_orbis_mobile_directory_description()}</p>
            </Card>
          {/if}

          <Card class="border-primary/20 bg-primary/[0.04] p-5">
            <p class="text-xs leading-5 text-muted-foreground">{m.settings_orbis_mobile_privacy()}</p>
          </Card>
        </aside>
      </div>
    {/if}
  </div>
</div>
