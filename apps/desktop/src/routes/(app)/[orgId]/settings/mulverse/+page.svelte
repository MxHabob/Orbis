<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { createMutation, createQuery } from "@tanstack/svelte-query";
  import { api, queryClient } from "$lib/api";
  import type { MulverseDeploymentDto, MulverseEndpointDto } from "$lib/api";

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
      await queryClient.invalidateQueries({ queryKey: ["mulverseConnection", data.orgId] });
    },
  }));

  const disconnectMutation = createMutation(() => ({
    mutationFn: () => api.disconnectMulverse(),
    onSuccess: async () => {
      selectedDeploymentId = null;
      await queryClient.invalidateQueries({ queryKey: ["mulverseConnection", data.orgId] });
      await queryClient.invalidateQueries({ queryKey: ["mulverseDeployments", data.orgId] });
    },
  }));

  const createDeploymentMutation = createMutation(() => ({
    mutationFn: () =>
      api.createMulverseDeployment({
        name: deploymentName.trim(),
        environment: deploymentEnvironment.trim(),
      }),
    onSuccess: async () => {
      deploymentName = "";
      await queryClient.invalidateQueries({ queryKey: ["mulverseDeployments", data.orgId] });
    },
  }));

  const updateDeploymentMutation = createMutation(() => ({
    mutationFn: (deployment: MulverseDeploymentDto) =>
      api.updateMulverseDeployment(deployment.id, {
        name: editingDeploymentName.trim(),
        environment: editingDeploymentEnvironment.trim(),
      }),
    onSuccess: async () => {
      editingDeploymentId = null;
      await queryClient.invalidateQueries({ queryKey: ["mulverseDeployments", data.orgId] });
    },
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
    },
  }));

  const updateEndpointMutation = createMutation(() => ({
    mutationFn: ({ endpoint, status, url }: {
      endpoint: MulverseEndpointDto;
      status?: "active" | "disabled";
      url?: string;
    }) =>
      api.updateMulverseEndpoint(endpoint.id, {
        url,
        status,
      }),
    onSuccess: async () => {
      editingEndpointId = null;
      endpointUrl = "";
      await queryClient.invalidateQueries({
        queryKey: ["mulverseEndpoints", data.orgId, selectedDeploymentId],
      });
    },
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
  <title>{m.settings_orbis_mobile_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background">
  <div class="mx-auto flex w-full max-w-4xl flex-col gap-6 p-6">
    <header class="space-y-2">
      <h1 class="text-xl font-semibold text-foreground">{m.settings_orbis_mobile_title()}</h1>
      <p class="max-w-3xl text-sm leading-6 text-muted-foreground">
        {m.settings_orbis_mobile_description()}
      </p>
    </header>

    {#if connection.isPending}
      <div class="space-y-3" aria-label={m.common_loading()}>
        <div class="h-24 animate-pulse rounded-xl bg-muted"></div>
        <div class="h-36 animate-pulse rounded-xl bg-muted"></div>
      </div>
    {:else if connection.isError}
      <section class="space-y-3 rounded-xl border border-destructive/30 bg-card p-5">
        <p class="text-sm text-destructive">{connection.error.message}</p>
        <button class="rounded-md border border-border px-4 py-2 text-sm" onclick={() => connection.refetch()}>
          {m.common_retry()}
        </button>
      </section>
    {:else if !connection.data?.connected}
      <section class="space-y-4 rounded-xl border border-border bg-card p-5">
        <div>
          <h2 class="text-sm font-semibold text-foreground">{m.settings_orbis_mobile_connect_title()}</h2>
          <p class="mt-1 text-sm leading-6 text-muted-foreground">
            {m.settings_orbis_mobile_connect_description()}
          </p>
        </div>
        <form class="flex flex-col gap-3 sm:flex-row" onsubmit={submitConnect}>
          <label class="sr-only" for="mulverse-key">{m.settings_orbis_mobile_api_key()}</label>
          <input
            id="mulverse-key"
            type="password"
            autocomplete="off"
            bind:value={apiKey}
            placeholder={m.settings_orbis_mobile_api_key()}
            class="min-h-11 min-w-0 flex-1 rounded-md border border-border bg-background px-3 text-sm text-foreground"
            required
          />
          <button
            type="submit"
            disabled={!apiKey.trim() || connectMutation.isPending}
            class="min-h-11 rounded-md bg-primary px-5 text-sm font-medium text-primary-foreground disabled:opacity-50"
          >
            {connectMutation.isPending ? m.common_loading() : m.settings_orbis_mobile_connect()}
          </button>
        </form>
        {#if connectMutation.isError}
          <p class="text-sm text-destructive">{connectMutation.error.message}</p>
        {/if}
      </section>
    {:else}
      <section class="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-border bg-card p-5">
        <div>
          <h2 class="text-sm font-semibold text-foreground">{m.settings_orbis_mobile_connected()}</h2>
          <p class="mt-1 text-sm text-muted-foreground">
            {connection.data.organization_id}
          </p>
        </div>
        <button
          class="min-h-11 rounded-md border border-border px-4 text-sm text-foreground disabled:opacity-50"
          disabled={disconnectMutation.isPending}
          onclick={() => disconnectMutation.mutate()}
        >
          {disconnectMutation.isPending ? m.common_loading() : m.settings_orbis_mobile_disconnect()}
        </button>
        {#if disconnectMutation.isError}
          <p class="w-full text-sm text-destructive">{disconnectMutation.error.message}</p>
        {/if}
      </section>

      <section class="space-y-4 rounded-xl border border-border bg-card p-5">
        <div>
          <h2 class="text-sm font-semibold text-foreground">{m.settings_orbis_mobile_deployments()}</h2>
          <p class="mt-1 text-sm text-muted-foreground">{m.settings_orbis_mobile_deployments_description()}</p>
        </div>
        <form class="grid gap-3 sm:grid-cols-[1fr_1fr_auto]" onsubmit={submitDeployment}>
          <label class="sr-only" for="deployment-name">{m.settings_orbis_mobile_deployment_name()}</label>
          <input id="deployment-name" bind:value={deploymentName} placeholder={m.settings_orbis_mobile_deployment_name()} class="min-h-11 rounded-md border border-border bg-background px-3 text-sm" required />
          <label class="sr-only" for="deployment-environment">{m.settings_orbis_mobile_environment()}</label>
          <input id="deployment-environment" bind:value={deploymentEnvironment} placeholder={m.settings_orbis_mobile_environment()} class="min-h-11 rounded-md border border-border bg-background px-3 text-sm" required />
          <button type="submit" disabled={!deploymentName.trim() || !deploymentEnvironment.trim() || createDeploymentMutation.isPending} class="min-h-11 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground disabled:opacity-50">
            {m.settings_orbis_mobile_add_deployment()}
          </button>
        </form>
        {#if createDeploymentMutation.isError}
          <p class="text-sm text-destructive">{createDeploymentMutation.error.message}</p>
        {/if}
        {#if deployments.isPending}
          <div class="h-20 animate-pulse rounded-lg bg-muted"></div>
        {:else if deployments.isError}
          <div class="space-y-2 text-sm text-destructive">
            <p>{deployments.error.message}</p>
            <button class="underline" onclick={() => deployments.refetch()}>{m.common_retry()}</button>
          </div>
        {:else if deployments.data?.length === 0}
          <p class="rounded-lg border border-dashed border-border p-4 text-sm text-muted-foreground">
            {m.settings_orbis_mobile_empty_deployments()}
          </p>
        {:else}
          <div class="divide-y divide-border">
            {#each deployments.data ?? [] as deployment (deployment.id)}
              <div class="space-y-3 py-4">
                <div class="flex flex-wrap items-center justify-between gap-3">
                  <button class="text-start font-medium text-foreground underline-offset-4 hover:underline" onclick={() => (selectedDeploymentId = selectedDeploymentId === deployment.id ? null : deployment.id)}>
                    {deployment.name}
                  </button>
                  <span class="text-xs text-muted-foreground">
                    {deployment.environment} · {statusMessage(deployment.status)}
                  </span>
                  <button class="rounded-md border border-border px-3 py-2 text-sm" onclick={() => { editingDeploymentId = editingDeploymentId === deployment.id ? null : deployment.id; editingDeploymentName = deployment.name; editingDeploymentEnvironment = deployment.environment; }}>
                    {m.common_action_edit()}
                  </button>
                </div>
                {#if editingDeploymentId === deployment.id}
                  <form class="grid gap-2 sm:grid-cols-[1fr_1fr_auto]" onsubmit={(event) => saveDeployment(event, deployment)}>
                    <input bind:value={editingDeploymentName} aria-label={m.settings_orbis_mobile_deployment_name()} class="min-h-10 rounded-md border border-border bg-background px-3 text-sm" required />
                    <input bind:value={editingDeploymentEnvironment} aria-label={m.settings_orbis_mobile_environment()} class="min-h-10 rounded-md border border-border bg-background px-3 text-sm" required />
                    <button type="submit" class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground">{m.common_action_save()}</button>
                  </form>
                {/if}
                {#if updateDeploymentMutation.isError}
                  <p class="text-sm text-destructive">{updateDeploymentMutation.error.message}</p>
                {/if}
                {#if selectedDeploymentId === deployment.id}
                  <div class="space-y-3 rounded-lg bg-muted/30 p-4">
                    <h3 class="text-sm font-semibold">{m.settings_orbis_mobile_endpoints()}</h3>
                    {#if endpoints.isPending}
                      <div class="h-14 animate-pulse rounded bg-muted"></div>
                    {:else if endpoints.isError}
                      <div class="space-y-2 text-sm text-destructive">
                        <p>{endpoints.error.message}</p>
                        <button class="underline" onclick={() => endpoints.refetch()}>{m.common_retry()}</button>
                      </div>
                    {:else if endpoints.data?.length === 0}
                      <p class="text-sm text-muted-foreground">{m.settings_orbis_mobile_empty_endpoints()}</p>
                    {:else}
                      {#each endpoints.data ?? [] as endpoint (endpoint.id)}
                        <div class="flex flex-wrap items-center justify-between gap-3 rounded-md border border-border bg-background p-3">
                          <div class="min-w-0 flex-1">
                            <p class="break-all text-sm text-foreground">{endpoint.url}</p>
                            <p class="mt-1 text-xs text-muted-foreground">
                              {endpoint.network_scope} · {statusMessage(endpoint.status)}
                            </p>
                          </div>
                          {#if endpoint.status !== "revoked"}
                            {#if editingEndpointId === endpoint.id}
                              <form class="flex w-full gap-2" onsubmit={(event) => saveEndpoint(event, endpoint)}>
                                <input bind:value={endpointUrl} placeholder={endpoint.url} aria-label={m.settings_orbis_mobile_endpoint_url()} class="min-h-10 min-w-0 flex-1 rounded-md border border-border bg-background px-3 text-sm" />
                                <button type="submit" class="rounded-md bg-primary px-3 py-2 text-sm text-primary-foreground">{m.common_action_save()}</button>
                              </form>
                            {:else}
                              <button class="rounded-md border border-border px-3 py-2 text-sm" onclick={() => { editingEndpointId = endpoint.id; endpointUrl = ""; }}>
                                {m.settings_orbis_mobile_edit_endpoint()}
                              </button>
                              {#if endpoint.status === "disabled"}
                                <button class="rounded-md border border-border px-3 py-2 text-sm" onclick={() => updateEndpointMutation.mutate({ endpoint, status: "active" })}>
                                  {m.settings_orbis_mobile_activate()}
                                </button>
                              {:else}
                                <button class="rounded-md border border-border px-3 py-2 text-sm" onclick={() => updateEndpointMutation.mutate({ endpoint, status: "disabled" })}>
                                  {m.settings_orbis_mobile_disable()}
                                </button>
                              {/if}
                            {/if}
                          {/if}
                        </div>
                      {/each}
                    {/if}
                    <form class="grid gap-2 sm:grid-cols-[1fr_auto_auto]" onsubmit={submitEndpoint}>
                      <label class="sr-only" for="endpoint-url">{m.settings_orbis_mobile_endpoint_url()}</label>
                      <input id="endpoint-url" bind:value={newEndpointUrl} placeholder={m.settings_orbis_mobile_endpoint_url()} class="min-h-11 min-w-0 rounded-md border border-border bg-background px-3 text-sm" required />
                      <label class="sr-only" for="endpoint-scope">{m.settings_orbis_mobile_network_scope()}</label>
                      <select id="endpoint-scope" bind:value={endpointScope} class="min-h-11 rounded-md border border-border bg-background px-3 text-sm">
                        <option value="public">{m.settings_orbis_mobile_public_endpoint()}</option>
                        <option value="local_network">{m.settings_orbis_mobile_local_endpoint()}</option>
                      </select>
                      <button type="submit" disabled={!newEndpointUrl.trim() || createEndpointMutation.isPending} class="min-h-11 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground disabled:opacity-50">
                        {m.settings_orbis_mobile_add_endpoint()}
                      </button>
                    </form>
                    {#if createEndpointMutation.isError}
                      <p class="text-sm text-destructive">{createEndpointMutation.error.message}</p>
                    {/if}
                    {#if updateEndpointMutation.isError}
                      <p class="text-sm text-destructive">{updateEndpointMutation.error.message}</p>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </section>
    {/if}

    <section class="space-y-4 rounded-xl border border-border bg-card p-5">
      <div>
        <h2 class="text-sm font-semibold text-foreground">
          {m.settings_orbis_mobile_directory_title()}
        </h2>
        <p class="mt-1 text-sm leading-6 text-muted-foreground">
          {m.settings_orbis_mobile_directory_description()}
        </p>
      </div>
      <ul class="list-disc space-y-2 ps-5 text-sm leading-6 text-muted-foreground">
        <li>{m.settings_orbis_mobile_directory_step_one()}</li>
        <li>{m.settings_orbis_mobile_directory_step_two()}</li>
        <li>{m.settings_orbis_mobile_directory_step_three()}</li>
      </ul>
      <p class="text-sm text-muted-foreground">{m.settings_orbis_mobile_staff_actions()}</p>
    </section>

    <p class="rounded-lg border border-primary/20 bg-primary/5 p-4 text-sm leading-6 text-muted-foreground">
      {m.settings_orbis_mobile_privacy()}
    </p>
  </div>
</div>
