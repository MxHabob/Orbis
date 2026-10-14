<!--
  @component ConnectionSetup
  Database and remote server connection configuration.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Logo } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import type { ConnectionMode, RemoteHealthResult } from "$lib/api/types";
import { ArrowLeft, ArrowPath, ArrowRight, ChartBar, CheckCircle, Computer, Database, GlobeAlt, InfoCircle, UserBadgeCheck } from "@orbis/icons";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { onMount } from "svelte";
import { configureApiClient } from "$lib/api";
import { l } from "$lib/core/i18n";


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
          message: l("هذا الحقل مطلوب", "This field is required"),
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
      mode: values.mode as ConnectionMode,
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
        goto(localizeHref("/login"), { replaceState: true });
      } else {
        goto(localizeHref("/onboarding"), { replaceState: true });
      }
    } else {
      throw new Error(result.message || m.setup_connection_failed());
    }
  });
}
</script>

<div class="min-h-full w-full bg-[#191e28] px-4 py-6 sm:px-6 lg:px-8">
  <div class="mx-auto flex min-h-[calc(100vh-3rem)] max-w-6xl items-center justify-center">
    <div class="w-full max-w-5xl overflow-hidden rounded-[26px] border border-border/80 bg-[#1e232d]/90 shadow-[0_20px_80px_rgba(0,0,0,0.38)] backdrop-blur-xl">
      <header class="flex items-center justify-between border-b border-border/70 bg-[#1a1f2a]/80 px-5 py-3.5 sm:px-6">
        <div class="flex items-center gap-3">
          <div class="flex size-8 items-center justify-center rounded-xl border border-primary/30 bg-primary/10 text-primary">
            <Logo class="size-4" dotClass="fill-primary" />
          </div>
          <div class="text-left">
            <h1 class="text-sm font-semibold text-foreground sm:text-base">{m.setup_connection_title()}</h1>
            <p class="text-[11px] text-muted-foreground">{m.setup_connection_subtitle()}</p>
          </div>
        </div>

        <Badge variant="outline" class="gap-1.5 border-border/60 bg-background/40 px-2.5 py-0.5 text-[10px] font-mono text-muted-foreground">
          <ChartBar size={10} class="text-emerald-500" />
          <span>{m.auth_ui_connection_engine_label()}</span>
        </Badge>
      </header>

      <form onsubmit={handleConnect} class="p-4 sm:p-6 lg:p-8">
        {#if form.formError}
          <div class="mb-5 flex items-center gap-2.5 rounded-xl border border-destructive/20 bg-destructive/10 p-3 text-xs text-destructive">
            <InfoCircle class="size-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        <div class="grid grid-cols-1 gap-6 xl:grid-cols-[1.08fr_1.72fr]">
          <aside class="rounded-2xl border border-border/60 bg-muted/10 p-4 text-left shadow-inner shadow-black/10 sm:p-5">
            <div class="mb-4 flex items-center gap-2 border-b border-border/40 pb-2">
              <Database size={15} class="text-primary" />
              <h2 class="text-xs font-bold text-foreground sm:text-sm">{m.ui_copy_system_architecture_c18b3d52()}</h2>
            </div>

            <p class="text-xs leading-relaxed text-muted-foreground">
              {l("يدعم Orbis ثلاثة أنماط اتصال لتناسب بيئات المؤسسات المختلفة:", "Orbis supports three connection modes for different institutional environments:")}
            </p>

            <div class="mt-4 space-y-3">
              <div class="rounded-xl border border-border/50 bg-card/60 p-3">
                <div class="flex items-center gap-2 text-xs font-bold text-foreground">
                  <Computer size={13} class="text-primary" />
                  <span>{m.ui_copy_local_embedded_mode_9a445d7e()}</span>
                </div>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">
                  {m.ui_copy_operates_completely_offline_with_an_embedd_6b260871()}
                </p>
              </div>

              <div class="rounded-xl border border-border/50 bg-card/60 p-3">
                <div class="flex items-center gap-2 text-xs font-bold text-foreground">
                  <Database size={13} class="text-primary" />
                  <span>{l("اتصال مباشر بـ PostgreSQL", "Direct PostgreSQL")}</span>
                </div>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">
                  {l("يتصل التطبيق مباشرةً بقاعدة PostgreSQL عبر الشبكة المحلية.", "The application connects directly to PostgreSQL over the network.")}
                </p>
              </div>

              <div class="rounded-xl border border-border/50 bg-card/60 p-3">
                <div class="flex items-center gap-2 text-xs font-bold text-foreground">
                  <GlobeAlt size={13} class="text-primary" />
                  <span>{m.ui_copy_remote_cloud_mode_0bab5a3e()}</span>
                </div>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">
                  {m.ui_copy_connects_to_a_central_orbis_api_server_for_ed966b7f()}
                </p>
              </div>
            </div>

            <div class="mt-5 flex items-center gap-2 border-t border-border/30 pt-3 text-[11px] text-muted-foreground">
              <UserBadgeCheck size={12} class="shrink-0 text-primary" />
              <span>{m.ui_copy_all_connections_and_data_are_strictly_isol_e420944a()}</span>
            </div>
          </aside>

          <section class="space-y-5">
            <div class="text-center lg:text-left">
              <p class="text-xs font-bold text-foreground">{m.ui_copy_select_connection_mode_acd68c8e()}</p>
            </div>

            <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
              <button
                type="button"
                class="cursor-pointer rounded-2xl border-2 p-4 text-left transition-all {form.values.mode === 'local' ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/60 bg-muted/5 hover:border-border hover:bg-muted/15'}"
                onclick={() => { form.values.mode = 'local'; form.clearError(); healthResult = null; }}
              >
                <div class="mb-3 flex items-center justify-between">
                  <div class="flex size-9 items-center justify-center rounded-xl {form.values.mode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <Database size={16} />
                  </div>
                  {#if form.values.mode === 'local'}
                    <div class="flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground">
                      <CheckCircle size={12} />
                    </div>
                  {/if}
                </div>
                <h3 class="text-xs font-bold text-foreground">{m.setup_connection_local_title()}</h3>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">{m.setup_connection_local_desc()}</p>
              </button>

              <button
                type="button"
                class="cursor-pointer rounded-2xl border-2 p-4 text-left transition-all {form.values.mode === 'direct' ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/60 bg-muted/5 hover:border-border hover:bg-muted/15'}"
                onclick={() => { form.values.mode = 'direct'; form.clearError(); healthResult = null; }}
              >
                <div class="mb-3 flex items-center justify-between">
                  <div class="flex size-9 items-center justify-center rounded-xl {form.values.mode === 'direct' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <Database size={16} />
                  </div>
                  {#if form.values.mode === 'direct'}
                    <div class="flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground">
                      <CheckCircle size={12} />
                    </div>
                  {/if}
                </div>
                <h3 class="text-xs font-bold text-foreground">{l("اتصال PostgreSQL مباشر", "Direct PostgreSQL")}</h3>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">
                  {l("اتصال مباشر بقاعدة PostgreSQL عبر الشبكة دون وسيط API.", "Direct network connection to PostgreSQL without an API intermediary.")}
                </p>
              </button>

              <button
                type="button"
                class="cursor-pointer rounded-2xl border-2 p-4 text-left transition-all {form.values.mode === 'api' ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/60 bg-muted/5 hover:border-border hover:bg-muted/15'}"
                onclick={() => { form.values.mode = 'api'; form.clearError(); healthResult = null; }}
              >
                <div class="mb-3 flex items-center justify-between">
                  <div class="flex size-9 items-center justify-center rounded-xl {form.values.mode === 'api' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                    <GlobeAlt size={16} />
                  </div>
                  {#if form.values.mode === 'api'}
                    <div class="flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground">
                      <CheckCircle size={12} />
                    </div>
                  {/if}
                </div>
                <h3 class="text-xs font-bold text-foreground">{m.setup_connection_remote_title()}</h3>
                <p class="mt-1 text-[11px] leading-normal text-muted-foreground">{m.setup_connection_remote_desc()}</p>
              </button>
            </div>

            <div class="rounded-2xl border border-border/60 bg-muted/10 p-4 sm:p-5">
              {#if form.values.mode === 'local'}
                <div class="space-y-3">
                  <FormField id="dbFilename" label={m.setup_connection_db_file_label()} error={fieldError('dbFilename')} required>
                    {#snippet children()}
                      <Input id="dbFilename" type="text" bind:value={form.values.dbFilename} disabled={form.isSubmitting} class="h-9 border-input bg-background text-xs font-mono" placeholder="orbis.db" />
                    {/snippet}
                  </FormField>
                  <p class="flex items-center gap-1.5 text-[11px] text-muted-foreground">
                    <Computer size={12} class="shrink-0 text-muted-foreground/80" />
                    <span>{m.setup_connection_db_file_hint()}</span>
                  </p>
                </div>
              {:else if form.values.mode === 'api'}
                <div class="space-y-3">
                  <FormField id="serverUrl" label={m.setup_connection_server_url_label()} error={fieldError('serverUrl')} required>
                    {#snippet children()}
                      <div class="flex gap-2">
                        <Input id="serverUrl" type="url" bind:value={form.values.serverUrl} disabled={form.isSubmitting} class="h-9 flex-1 border-input bg-background text-xs font-mono" placeholder="http://localhost:3000" />
                        <Button type="button" variant="outline" size="sm" class="h-9 shrink-0 border-border px-3 text-xs" onclick={checkHealth} disabled={isCheckingHealth || !form.values.serverUrl.trim()}>
                          {#if isCheckingHealth}
                            <LoadingIndicator class="size-3.5" size={14} />
                          {:else}
                            <ChartBar class="size-3.5 text-primary" />
                          {/if}
                          <span>{m.setup_connection_test_btn()}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                  {#if healthResult}
                    <div class="flex items-center gap-2 rounded-xl border p-2.5 text-xs {healthResult.reachable ? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' : 'border-destructive/20 bg-destructive/10 text-destructive'}">
                      {#if healthResult.reachable}
                        <CheckCircle class="size-4 shrink-0" />
                        <span>{m.setup_connection_connected_status({ status: healthResult.status ?? 'ok', version: healthResult.version ?? '1.0' })}</span>
                      {:else}
                        <InfoCircle class="size-4 shrink-0" />
                        <span>{healthResult.error ?? m.setup_connection_unreachable()}</span>
                      {/if}
                    </div>
                  {/if}
                </div>
              {:else}
                <div class="space-y-4">
                  <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                    <FormField id="host" label={m.ui_copy_database_host_218c7a89()} error={fieldError('host')} required>
                      {#snippet children()}
                        <Input id="host" bind:value={form.values.host} disabled={form.isSubmitting} placeholder="localhost" />
                      {/snippet}
                    </FormField>

                    <FormField id="port" label={m.ui_copy_port_966ae516()} error={fieldError('port')} required>
                      {#snippet children()}
                        <Input id="port" type="number" bind:value={form.values.port} disabled={form.isSubmitting} />
                      {/snippet}
                    </FormField>

                    <FormField id="databaseName" label={m.ui_copy_database_name_72beaf93()} error={fieldError('databaseName')} required>
                      {#snippet children()}
                        <Input id="databaseName" bind:value={form.values.databaseName} disabled={form.isSubmitting} placeholder="orbis_db" />
                      {/snippet}
                    </FormField>

                    <FormField id="username" label={m.ui_copy_username_f733f5b3()} error={fieldError('username')} required>
                      {#snippet children()}
                        <Input id="username" bind:value={form.values.username} disabled={form.isSubmitting} />
                      {/snippet}
                    </FormField>
                  </div>

                  <FormField id="password" label={m.common_user_form_password()} error={fieldError('password')} required>
                    {#snippet children()}
                      <Input id="password" type="password" bind:value={form.values.password} disabled={form.isSubmitting} />
                    {/snippet}
                  </FormField>

                  <p class="text-[11px] text-muted-foreground">
                    {l("يتصل هذا الجهاز مباشرةً بقاعدة البيانات. استخدم حسابًا محدود الصلاحيات وعلى شبكة موثوقة.", "This device connects directly to the database. Use a least-privilege account on a trusted network.")}
                  </p>
                </div>
              {/if}
            </div>

            <Button type="submit" variant="default" class="h-11 w-full text-xs font-bold shadow-sm shadow-primary/20" disabled={form.isSubmitting || (form.values.mode === 'api' && healthResult !== null && !healthResult.reachable)}>
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
          </section>
        </div>
      </form>
    </div>
  </div>
</div>
