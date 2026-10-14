<!--
  @component ConnectionSetup
  Database and remote server connection configuration.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { api } from "$lib/api/client";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import type { RemoteHealthResult } from "$lib/api/types";
import { ArrowLeft, ArrowRight, ChartBar, CheckCircle, Computer, Database, GlobeAlt, InfoCircle } from "@orbis/icons";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { onMount } from "svelte";
import { configureApiClient } from "$lib/api";


const isRTL = $derived(getLocale().startsWith("ar"));

const connectionSchema = z.object({
  mode: z.enum(["local", "direct", "api"]),
  dbFilename: z.string().trim().min(1, m.setup_connection_db_file_label()),
  serverUrl: z.string().trim(),
  host: z.string().trim(),
  port: z.coerce.number().int().min(1).max(65535),
  databaseName: z.string().trim(),
  username: z.string().trim(),
  password: z.string(),
}).superRefine((data, context) => {
  if (data.mode === "api" && (
    !data.serverUrl ||
    (!data.serverUrl.startsWith("http://") && !data.serverUrl.startsWith("https://"))
  )) {
    context.addIssue({
      code: "custom",
      message: m.setup_connection_server_url_label(),
      path: ["serverUrl"],
    });
  }
  if (data.mode === "direct") {
    const requiredFields = [
      ["host", data.host],
      ["databaseName", data.databaseName],
      ["username", data.username],
      ["password", data.password],
    ] as const;
    for (const [field, value] of requiredFields) {
      if (!value.trim()) {
        context.addIssue({
          code: "custom",
          message: m.setup_connection_required_field(),
          path: [field],
        });
      }
    }
  }
});

type ConnectionFormValues = z.infer<typeof connectionSchema>;

const form = new FormController<ConnectionFormValues>({
  initialValues: {
    mode: "local",
    dbFilename: "orbis.db",
    serverUrl: "",
    host: "localhost",
    port: 5432,
    databaseName: "orbis_db",
    username: "",
    password: "",
  },
  schema: connectionSchema,
});

function fieldError(field: keyof ConnectionFormValues): string | undefined {
  return form.errors[field]?.[0];
}

let isCheckingHealth = $state(false);
let healthResult = $state<RemoteHealthResult | null>(null);

onMount(() => {
  configureApiClient("local");
});

async function checkHealth() {
  const url = form.values.serverUrl.trim();
  if (!url) return;
  
  isCheckingHealth = true;
  healthResult = null;
  form.formError = undefined;
  
  try {
    healthResult = await api.connectionCheckRemoteHealth(url);
    if (!healthResult.reachable) {
      form.formError = healthResult.error || m.setup_connection_unreachable();
    }
  } catch (e) {
    form.formError = e instanceof Error ? e.message : String(e);
  } finally {
    isCheckingHealth = false;
  }
}

async function handleConnect(e: SubmitEvent | MouseEvent) {
  if (e) e.preventDefault();

  await form.submit(async (values) => {
    await api.connectionSetConfig({
      mode: values.mode,
      serverUrl: values.mode === "api" ? values.serverUrl.trim() : undefined,
      databaseEngine: values.mode === "direct" ? "postgres" : undefined,
      host: values.mode === "direct" ? values.host.trim() : undefined,
      port: values.mode === "direct" ? values.port : undefined,
      databaseName: values.mode === "direct" ? values.databaseName.trim() : undefined,
      username: values.mode === "direct" ? values.username.trim() : undefined,
      password: values.mode === "direct" ? values.password : undefined,
      dbFilename: values.mode === "local" ? values.dbFilename.trim() : undefined,
    });

    if (values.mode === "api") {
      configureApiClient("api", values.serverUrl.trim());
    } else {
      configureApiClient("local");
    }
    const result = await api.connectionConnect();
    if (result.connected) {
      const hasOrg = await api.systemHasOrganization();
      if (hasOrg) {
        goto(resolve(localizeHref("/login")), { replaceState: true });
      } else {
        goto(resolve(localizeHref("/onboarding")), { replaceState: true });
      }
    } else {
      throw new Error(result.message || m.setup_connection_failed());
    }
  });
}
</script>

<div class="flex h-full w-full flex-col gap-6 overflow-y-auto p-4 sm:p-6">
  <form onsubmit={handleConnect} class="mx-auto flex w-full max-w-7xl flex-col gap-6">
    <Card>
      <CardHeader>
        <CardTitle>{m.setup_connection_title()}</CardTitle>
        <p class="text-xs text-muted-foreground">{m.setup_connection_subtitle()}</p>
      </CardHeader>
      <CardContent class="p-0">
        <div role="group" aria-label={m.setup_connection_mode_title()}>
          <button
            type="button"
            aria-pressed={form.values.mode === "local"}
            class="flex w-full items-center gap-4 border-t border-border px-6 py-4 text-start transition-colors hover:bg-muted/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
            onclick={() => { form.values.mode = "local"; form.clearError(); healthResult = null; }}
          >
            <span class="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
              <Computer size={16} />
            </span>
            <span class="flex min-w-0 flex-1 flex-col gap-1">
              <span class="text-xs font-semibold text-foreground">{m.setup_connection_local_title()}</span>
              <span class="text-[11px] leading-relaxed text-muted-foreground">{m.setup_connection_local_desc()}</span>
            </span>
            {#if form.values.mode === "local"}
              <CheckCircle class="size-4 shrink-0 text-primary" />
            {/if}
          </button>

          <button
            type="button"
            aria-pressed={form.values.mode === "direct"}
            class="flex w-full items-center gap-4 border-t border-border px-6 py-4 text-start transition-colors hover:bg-muted/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
            onclick={() => { form.values.mode = "direct"; form.clearError(); healthResult = null; }}
          >
            <span class="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
              <Database size={16} />
            </span>
            <span class="flex min-w-0 flex-1 flex-col gap-1">
              <span class="text-xs font-semibold text-foreground">{m.setup_connection_direct_title()}</span>
              <span class="text-[11px] leading-relaxed text-muted-foreground">{m.setup_connection_direct_desc()}</span>
            </span>
            {#if form.values.mode === "direct"}
              <CheckCircle class="size-4 shrink-0 text-primary" />
            {/if}
          </button>

          <button
            type="button"
            aria-pressed={form.values.mode === "api"}
            class="flex w-full items-center gap-4 border-y border-border px-6 py-4 text-start transition-colors hover:bg-muted/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
            onclick={() => { form.values.mode = "api"; form.clearError(); healthResult = null; }}
          >
            <span class="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
              <GlobeAlt size={16} />
            </span>
            <span class="flex min-w-0 flex-1 flex-col gap-1">
              <span class="text-xs font-semibold text-foreground">{m.setup_connection_remote_title()}</span>
              <span class="text-[11px] leading-relaxed text-muted-foreground">{m.setup_connection_remote_desc()}</span>
            </span>
            {#if form.values.mode === "api"}
              <CheckCircle class="size-4 shrink-0 text-primary" />
            {/if}
          </button>
        </div>
        <p class="px-6 py-4 text-[11px] leading-relaxed text-muted-foreground">{m.ui_copy_all_connections_and_data_are_strictly_isol_e420944a()}</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle>{m.setup_connection_configuration_title()}</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        {#if form.formError}
          <div class="mx-6 mt-5 flex items-center gap-2.5 rounded-md border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive" role="alert">
            <InfoCircle class="size-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        {#if form.values.mode === "local"}
          <SettingsRow
            title={m.setup_connection_db_file_label()}
            description={m.setup_connection_db_file_hint()}
            labelFor="dbFilename"
            required
            last
          >
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="dbFilename" bind:value={form.values.dbFilename} disabled={form.isSubmitting} class="h-9 bg-background font-mono" placeholder="orbis.db" aria-invalid={!!fieldError("dbFilename")} />
                {#if fieldError("dbFilename")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("dbFilename")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
        {:else if form.values.mode === "api"}
          <SettingsRow
            title={m.setup_connection_server_url_label()}
            description={m.setup_connection_server_url_description()}
            labelFor="serverUrl"
            required
            last={!healthResult}
          >
            {#snippet children()}
              <div class="w-full space-y-2">
                <div class="flex gap-2">
                  <Input id="serverUrl" type="url" bind:value={form.values.serverUrl} disabled={form.isSubmitting} class="h-9 min-w-0 flex-1 bg-background font-mono" placeholder="https://school.example.com" aria-invalid={!!fieldError("serverUrl")} />
                  <Button type="button" variant="outline" size="sm" class="h-9 shrink-0" onclick={checkHealth} disabled={isCheckingHealth || !form.values.serverUrl.trim()}>
                    {#if isCheckingHealth}
                      <LoadingIndicator class="size-3.5" size={14} />
                    {:else}
                      <ChartBar class="size-3.5" />
                    {/if}
                    <span>{m.setup_connection_test_btn()}</span>
                  </Button>
                </div>
                {#if fieldError("serverUrl")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("serverUrl")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          {#if healthResult}
            <div class="mx-6 mb-4 flex items-center gap-2 rounded-md border px-3 py-2 text-xs {healthResult.reachable ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-700 dark:text-emerald-400' : 'border-destructive/30 bg-destructive/10 text-destructive'}">
              {#if healthResult.reachable}
                <CheckCircle class="size-4 shrink-0" />
                <span>{m.setup_connection_connected_status({ version: healthResult.version ?? "1.0", latency: healthResult.latency_ms })}</span>
              {:else}
                <InfoCircle class="size-4 shrink-0" />
                <span>{healthResult.error ?? m.setup_connection_unreachable()}</span>
              {/if}
            </div>
          {/if}
        {:else}
          <SettingsRow title={m.ui_copy_database_host_218c7a89()} labelFor="host" required>
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="host" bind:value={form.values.host} disabled={form.isSubmitting} placeholder="localhost" aria-invalid={!!fieldError("host")} />
                {#if fieldError("host")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("host")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow title={m.ui_copy_port_966ae516()} labelFor="port" required>
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="port" type="number" bind:value={form.values.port} disabled={form.isSubmitting} aria-invalid={!!fieldError("port")} />
                {#if fieldError("port")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("port")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow title={m.ui_copy_database_name_72beaf93()} labelFor="databaseName" required>
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="databaseName" bind:value={form.values.databaseName} disabled={form.isSubmitting} placeholder="orbis_db" aria-invalid={!!fieldError("databaseName")} />
                {#if fieldError("databaseName")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("databaseName")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow title={m.ui_copy_username_f733f5b3()} labelFor="username" required>
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="username" bind:value={form.values.username} disabled={form.isSubmitting} aria-invalid={!!fieldError("username")} />
                {#if fieldError("username")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("username")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow title={m.common_user_form_password()} labelFor="password" required>
            {#snippet children()}
              <div class="w-full space-y-1">
                <Input id="password" type="password" bind:value={form.values.password} disabled={form.isSubmitting} aria-invalid={!!fieldError("password")} />
                {#if fieldError("password")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("password")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow
            title={m.setup_connection_direct_security_title()}
            description={m.setup_connection_direct_security_hint()}
            last
          />
        {/if}

        <div class="flex justify-end border-t border-border px-6 py-4">
          <Button type="submit" variant="default" class="min-w-40" disabled={form.isSubmitting || (form.values.mode === "api" && healthResult !== null && !healthResult.reachable)}>
            {#if form.isSubmitting}
              <LoadingIndicator class="size-4" size={16} />
              <span>{m.setup_connection_connecting()}</span>
            {:else}
              <span>{m.setup_connection_connect_btn()}</span>
              {#if isRTL}
                <ArrowLeft class="size-4" />
              {:else}
                <ArrowRight class="size-4" />
              {/if}
            {/if}
          </Button>
        </div>
      </CardContent>
    </Card>
  </form>
</div>
