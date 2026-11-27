<!--
  @component ConnectionSetup
  Database and remote server connection configuration.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import type { Pathname } from "$app/types";
import { api } from "$lib/api/client";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import type { RemoteHealthResult } from "$lib/api/types";
import {
  ArrowLeft,
  ArrowRight,
  ChartBar,
  CheckCircle,
  Computer,
  Database,
  FolderOpen,
  GlobeAlt,
  InfoCircle,
} from "@orbis/icons";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { onMount } from "svelte";
import { configureApiClient, isDesktopApp } from "$lib/api";

const isRTL = $derived(getLocale().startsWith("ar"));

const connectionSchema = z
  .object({
    mode: z.enum(["local", "api"]).optional(),
    dbFilename: z.string().trim(),
    serverUrl: z.string().trim(),
  })
  .superRefine((data, context) => {
    if (!data.mode) {
      context.addIssue({
        code: "custom",
        message: m.setup_connection_mode_title(),
        path: ["mode"],
      });
    }
    if (data.mode === "local" && !data.dbFilename) {
      context.addIssue({
        code: "custom",
        message: m.setup_connection_db_file_label(),
        path: ["dbFilename"],
      });
    }
    if (
      data.mode === "api" &&
      (!data.serverUrl ||
        (!data.serverUrl.startsWith("http://") &&
          !data.serverUrl.startsWith("https://")))
    ) {
      context.addIssue({
        code: "custom",
        message: m.setup_connection_server_url_label(),
        path: ["serverUrl"],
      });
    }
  });

type ConnectionFormValues = z.infer<typeof connectionSchema>;

const form = new FormController<ConnectionFormValues>({
  initialValues: {
    mode: undefined,
    dbFilename: "orbis.db",
    serverUrl: "",
  },
  schema: connectionSchema,
});

function fieldError(field: keyof ConnectionFormValues): string | undefined {
  return form.errors[field]?.[0];
}

let customDatabasePath = $state("");

onMount(async () => {
  try {
    if (isDesktopApp) {
      configureApiClient("local");
    } else {
      form.values.mode = "api";
    }

    const config = await api.connectionGetConfig();
    if (isDesktopApp) {
      form.values.dbFilename = config.local?.db_filename ?? "orbis.db";
      customDatabasePath = config.local?.custom_path ?? "";
    } else {
      form.values.serverUrl =
        config.api?.server_url ?? import.meta.env?.VITE_API_URL ?? "";
    }
  } catch (error) {
    form.formError = error instanceof Error ? error.message : String(error);
  }
});

async function browseDatabasePath() {
  form.formError = undefined;
  try {
    const path = await api.databasePickSavePath({
      defaultFilename: form.values.dbFilename || "orbis.db",
      filterName: "SQLite Database (*.db, *.sqlite)",
      extensions: ["db", "sqlite", "sqlite3"],
    });
    if (path) {
      customDatabasePath = path;
      const selectedFilename = path.split(/[\\/]/).pop();
      if (selectedFilename) form.values.dbFilename = selectedFilename;
    }
  } catch (error) {
    form.formError = error instanceof Error ? error.message : String(error);
  }
}

async function continueToOrganization() {
  const hasOrg = await api.systemHasOrganization();
  await goto(
    resolve(localizeHref(hasOrg ? "/login" : "/onboarding") as Pathname),
    { replaceState: true },
  );
}

let isCheckingHealth = $state(false);
let healthResult = $state<RemoteHealthResult | null>(null);

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
    if (!values.mode) {
      throw new Error(m.setup_connection_mode_title());
    }
    await api.connectionSetConfig({
      mode: values.mode,
      serverUrl: values.mode === "api" ? values.serverUrl.trim() : undefined,
      dbFilename:
        values.mode === "local" ? values.dbFilename.trim() : undefined,
      customPath:
        values.mode === "local" && customDatabasePath.trim()
          ? customDatabasePath.trim()
          : undefined,
    });
    localStorage.removeItem("orbis_connection_scope");

    try {
      const result = await api.connectionConnect();
      if (result.connected) {
        if (result.mode === "api") {
          configureApiClient("api", result.server_url);
        }
        await continueToOrganization();
      } else {
        throw new Error(result.message || m.setup_connection_failed());
      }
    } catch (error) {
      configureApiClient("local");
      throw error;
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
          {#if isDesktopApp}
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
          {/if}

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

        {#if !form.values.mode}
          <p class="px-6 py-4 text-sm text-muted-foreground">
            {m.setup_connection_mode_title()}
          </p>
        {:else if form.values.mode === "local" && isDesktopApp}
          <SettingsRow
            title={m.setup_connection_db_file_label()}
            description={m.setup_connection_db_file_hint()}
            labelFor="dbFilename"
            required
          >
            {#snippet children()}
              <div class="w-full">
                <Input id="dbFilename" bind:value={form.values.dbFilename} disabled={form.isSubmitting} class="h-9 bg-background font-mono" placeholder="orbis.db" aria-invalid={!!fieldError("dbFilename")} />
                {#if fieldError("dbFilename")}
                  <p class="text-[11px] text-destructive" role="alert">{fieldError("dbFilename")}</p>
                {/if}
              </div>
            {/snippet}
          </SettingsRow>
          <SettingsRow
            title={m.ui_copy_custom_absolute_file_path_optional_0c3adc12()}
            description={m.ui_copy_leave_blank_for_default_appdata_path_a40f6b20()}
            labelFor="customDatabasePath"
            last
          >
            {#snippet children()}
              <div class="w-full">
                <div class="flex gap-2">
                  <Input
                    id="customDatabasePath"
                    type="text"
                    bind:value={customDatabasePath}
                    disabled={form.isSubmitting}
                    class="h-9 min-w-0 flex-1 bg-background font-mono"
                    placeholder={m.ui_copy_default_application_data_folder_cc5cf159()}
                  />
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-9 shrink-0 gap-1.5"
                    onclick={browseDatabasePath}
                    disabled={form.isSubmitting}
                  >
                    <FolderOpen class="size-3.5" />
                    <span>{m.database_browse_file?.() || m.ui_copy_browse_f7b87051()}</span>
                  </Button>
                </div>
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
        {/if}

        <div class="flex justify-end gap-2 border-t border-border px-6 py-4">
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
