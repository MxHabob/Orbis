<!--
  Connection Setup Screen — First-time database connection setup.
  Spacious, luxury widescreen layout utilizing screen real estate with dual-column symmetry and architecture highlights.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Logo } from "@orbis/ui/logo";
import { Badge } from "@orbis/ui/badge";
import type { ConnectionMode, RemoteHealthResult } from "$lib/api/types";
import {
  Database,
  Globe,
  Loader2,
  CheckCircle2,
  AlertCircle,
  ArrowRight,
  ArrowLeft,
  Activity,
  Server,
  Zap,
  Shield,
  HardDrive,
  Cpu,
} from "@lucide/svelte";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { l } from "$lib/core/i18n";

const isRTL = $derived(getLocale().startsWith("ar"));

const connectionSchema = z.object({
  mode: z.enum(["local", "remote"]),
  dbFilename: z.string().trim().min(1, m.setup_connection_db_file_label()),
  serverUrl: z.string().trim(),
}).refine(
  (data) => {
    if (data.mode === "remote") {
      return (
        data.serverUrl.length > 0 &&
        (data.serverUrl.startsWith("http://") || data.serverUrl.startsWith("https://"))
      );
    }
    return true;
  },
  {
    message: m.setup_connection_server_url_label(),
    path: ["serverUrl"],
  },
);

type ConnectionFormValues = z.infer<typeof connectionSchema>;

const form = new FormController<ConnectionFormValues>({
  initialValues: {
    mode: "local",
    dbFilename: "orbis.db",
    serverUrl: "",
  },
  schema: connectionSchema,
});

function fieldError(field: keyof ConnectionFormValues): string | undefined {
  return form.errors[field]?.[0];
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
    await api.connectionSetConfig(
      values.mode as ConnectionMode,
      values.mode === "remote" ? values.serverUrl.trim() : undefined,
      values.mode === "local" ? values.dbFilename.trim() : undefined,
    );

    const result = await api.connectionConnect();
    if (result.connected) {
      const hasOrg = await api.systemHasOrganization();
      if (hasOrg) {
        goto(localizeHref("/login"), { replaceState: true });
      } else {
        goto(localizeHref("/setup/bootstrap"), { replaceState: true });
      }
    } else {
      throw new Error(result.message || m.setup_connection_failed());
    }
  });
}
</script>

<svelte:head>
  <title>{m.setup_connection_page_title()}</title>
</svelte:head>

<div class="w-full max-w-5xl 2xl:max-w-6xl mx-auto p-1 sm:p-2 animate-in fade-in duration-300">
  <div class="rounded-2xl sm:rounded-3xl border border-border/80 bg-card/95 shadow-2xl backdrop-blur-xl overflow-hidden flex flex-col justify-between">
    
    <!-- Header Bar -->
    <div class="flex items-center justify-between px-6 sm:px-8 py-3.5 border-b border-border/60 bg-muted/15">
      <div class="flex items-center gap-3">
        <div class="size-8 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center">
          <Logo class="size-5 text-primary" dotClass="fill-primary" />
        </div>
        <div class="text-start">
          <h1 class="text-sm sm:text-base font-bold text-foreground leading-none">
            {m.setup_connection_title()}
          </h1>
          <p class="text-[11px] text-muted-foreground mt-0.5 font-medium">
            {m.setup_connection_subtitle()}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Badge variant="outline" class="text-[11px] px-3 py-1 gap-1.5 font-semibold bg-background/80 border-border/70">
          <Cpu size={13} class="text-primary" />
          <span>{l("محرك البيانات المدمج", "Embedded Data Engine")}</span>
        </Badge>
      </div>
    </div>

    <!-- Main Content Form Area -->
    <form onsubmit={handleConnect} class="p-5 sm:p-7 lg:p-8 flex flex-col justify-between space-y-5">
      <div>
        {#if form.formError}
          <div class="mb-4 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 text-start items-start">
          
          <!-- Left Column: Architecture & Overview (5 Cols) -->
          <div class="lg:col-span-5 space-y-4 rounded-2xl bg-muted/10 border border-border/60 p-4 sm:p-5">
            <div class="flex items-center gap-2 pb-1 border-b border-border/40">
              <Server size={15} class="text-primary" />
              <h3 class="font-bold text-xs sm:text-sm text-foreground">
                {l("خيارات بنية الاتصال والتخزين", "Storage Architecture")}
              </h3>
            </div>

            <!-- Embedded SQLite Overview -->
            <div class="p-3.5 rounded-xl border transition-all {form.values.mode === 'local' ? 'bg-card border-primary/40 shadow-2xs' : 'bg-card/50 border-border/50 opacity-70'}">
              <div class="flex items-center gap-2.5 mb-1.5">
                <div class="size-6 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
                  <HardDrive size={13} />
                </div>
                <h4 class="font-bold text-xs text-foreground">
                  {l("محرك SQLite المحلي السريع", "Embedded Local SQLite")}
                </h4>
              </div>
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                {l(
                  "تخزين فوري محلي بالكامل دون الحاجة إلى خوادم خارجية أو اتصال بالإنترنت، مثالي للمدارس والمنشآت الفردية.",
                  "Full local embedded storage with zero external dependencies or internet requirements. Ideal for single campuses."
                )}
              </p>
            </div>

            <!-- Remote Cloud Node Overview -->
            <div class="p-3.5 rounded-xl border transition-all {form.values.mode === 'remote' ? 'bg-card border-primary/40 shadow-2xs' : 'bg-card/50 border-border/50 opacity-70'}">
              <div class="flex items-center gap-2.5 mb-1.5">
                <div class="size-6 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
                  <Globe size={13} />
                </div>
                <h4 class="font-bold text-xs text-foreground">
                  {l("الخادم المركزي السحابي", "Remote Central Node")}
                </h4>
              </div>
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                {l(
                  "ربط التطبيق المكتبي بخادم مركزي لمزامنة البيانات بين فروع متعددة وكوادر إدارية متزامنة في الوقت الفعلي.",
                  "Connect to a centralized enterprise server for multi-branch sync and real-time administrative collaboration."
                )}
              </p>
            </div>

            <!-- Offline-First Security Guarantee -->
            <div class="flex items-center gap-2 p-2.5 rounded-xl bg-primary/5 border border-primary/15 text-[11px] text-muted-foreground">
              <Shield size={13} class="text-primary shrink-0" />
              <span>{l("حماية البيانات والنسخ الاحتياطي التلقائي مدمج بالكامل", "Built-in data privacy and automated resilience.")}</span>
            </div>
          </div>

          <!-- Right Column: Mode Selector & Inputs (7 Cols) -->
          <div class="lg:col-span-7 space-y-4">
            <div class="flex items-center justify-between pb-1 border-b border-border/40">
              <div class="flex items-center gap-2">
                <Zap size={15} class="text-primary" />
                <h3 class="font-bold text-xs sm:text-sm text-foreground">
                  {l("تكوين وضع الاتصال", "Connection Configuration")}
                </h3>
              </div>
            </div>

            <!-- Mode Selector Toggle Buttons -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <!-- Local Mode Button -->
              <button
                type="button"
                onclick={() => {
                  form.values.mode = "local";
                  form.errors = {};
                  form.formError = undefined;
                  healthResult = null;
                }}
                class="p-3.5 rounded-2xl border text-start flex flex-col justify-between gap-2.5 transition-all duration-200 cursor-pointer shadow-2xs group {form.values.mode === 'local'
                  ? 'border-primary ring-2 ring-primary/20 bg-primary/5'
                  : 'border-border/70 bg-card hover:bg-muted/15'}"
              >
                <div class="flex items-center justify-between w-full">
                  <div class="size-7 rounded-lg flex items-center justify-center {form.values.mode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-foreground'} transition-colors">
                    <Database size={15} />
                  </div>
                  {#if form.values.mode === 'local'}
                    <CheckCircle2 size={13} class="text-primary" />
                  {/if}
                </div>
                <div>
                  <div class="font-bold text-xs text-foreground {form.values.mode === 'local' ? 'text-primary' : ''}">
                    {m.setup_connection_local_title()}
                  </div>
                  <div class="text-[10px] text-muted-foreground mt-0.5 line-clamp-2">
                    {m.setup_connection_local_desc()}
                  </div>
                </div>
              </button>

              <!-- Remote Mode Button -->
              <button
                type="button"
                onclick={() => {
                  form.values.mode = "remote";
                  form.errors = {};
                  form.formError = undefined;
                }}
                class="p-3.5 rounded-2xl border text-start flex flex-col justify-between gap-2.5 transition-all duration-200 cursor-pointer shadow-2xs group {form.values.mode === 'remote'
                  ? 'border-primary ring-2 ring-primary/20 bg-primary/5'
                  : 'border-border/70 bg-card hover:bg-muted/15'}"
              >
                <div class="flex items-center justify-between w-full">
                  <div class="size-7 rounded-lg flex items-center justify-center {form.values.mode === 'remote' ? 'bg-primary text-primary-foreground' : 'bg-muted text-foreground'} transition-colors">
                    <Globe size={15} />
                  </div>
                  {#if form.values.mode === 'remote'}
                    <CheckCircle2 size={13} class="text-primary" />
                  {/if}
                </div>
                <div>
                  <div class="font-bold text-xs text-foreground {form.values.mode === 'remote' ? 'text-primary' : ''}">
                    {m.setup_connection_remote_title()}
                  </div>
                  <div class="text-[10px] text-muted-foreground mt-0.5 line-clamp-2">
                    {m.setup_connection_remote_desc()}
                  </div>
                </div>
              </button>
            </div>

            <!-- Mode Specific Configuration Panel -->
            {#if form.values.mode === "local"}
              <div class="space-y-3 p-4 rounded-2xl bg-muted/15 border border-border/60">
                <FormField
                  id="dbFilename"
                  label={m.setup_connection_db_file_label()}
                  error={fieldError("dbFilename")}
                  required
                >
                  {#snippet children()}
                    <Input
                      id="dbFilename"
                      type="text"
                      bind:value={form.values.dbFilename}
                      disabled={form.isSubmitting}
                      class="h-10 text-xs bg-background border-input font-mono"
                      placeholder="orbis.db"
                    />
                  {/snippet}
                </FormField>
                <p class="text-[11px] text-muted-foreground leading-relaxed">
                  {m.setup_connection_db_file_hint()}
                </p>
              </div>
            {:else}
              <div class="space-y-3 p-4 rounded-2xl bg-muted/15 border border-border/60">
                <FormField
                  id="serverUrl"
                  label={m.setup_connection_server_url_label()}
                  error={fieldError("serverUrl")}
                  required
                >
                  {#snippet children()}
                    <div class="flex gap-2">
                      <Input
                        id="serverUrl"
                        type="url"
                        bind:value={form.values.serverUrl}
                        disabled={form.isSubmitting}
                        class="h-10 text-xs bg-background border-input flex-1 font-mono text-left"
                        style="direction: ltr;"
                        placeholder="https://orbis.institution.edu"
                      />
                      <Button
                        type="button"
                        variant="outline"
                        onclick={checkHealth}
                        disabled={!form.values.serverUrl.trim() || isCheckingHealth || form.isSubmitting}
                        class="h-10 px-3.5 text-xs gap-1.5 shrink-0 cursor-pointer border-border font-medium"
                      >
                        {#if isCheckingHealth}
                          <Loader2 class="w-3.5 h-3.5 animate-spin" />
                        {:else}
                          <Activity class="w-3.5 h-3.5 text-primary" />
                        {/if}
                        <span>{m.setup_connection_test_btn()}</span>
                      </Button>
                    </div>
                  {/snippet}
                </FormField>

                {#if healthResult}
                  <div class="flex items-center gap-2 p-3 rounded-xl text-xs {healthResult.reachable ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20' : 'bg-destructive/10 text-destructive border border-destructive/20'}">
                    {#if healthResult.reachable}
                      <CheckCircle2 class="w-4 h-4 shrink-0" />
                      <span class="font-medium">
                        {m.setup_connection_connected_status({
                          version: healthResult.version ?? 'v1.0',
                          latency: String(healthResult.latency_ms ?? 0)
                        })}
                      </span>
                    {:else}
                      <AlertCircle class="w-4 h-4 shrink-0" />
                      <span>{healthResult.error ?? m.setup_connection_unreachable()}</span>
                    {/if}
                  </div>
                {/if}
              </div>
            {/if}

          </div>

        </div>
      </div>

      <!-- Action Footer -->
      <div class="pt-3 border-t border-border/50 flex items-center justify-end">
        <Button
          type="submit"
          variant="default"
          disabled={form.isSubmitting || (form.values.mode === "remote" && !form.values.serverUrl.trim())}
          class="inline-flex items-center gap-2 px-7 h-9 text-xs font-semibold cursor-pointer disabled:opacity-50"
        >
          {#if form.isSubmitting}
            <Loader2 class="w-3.5 h-3.5 animate-spin" />
            <span>{m.setup_connection_connecting()}</span>
          {:else}
            <span>{m.setup_connection_connect_btn()}</span>
            {#if isRTL}
              <ArrowLeft class="w-3.5 h-3.5" />
            {:else}
              <ArrowRight class="w-3.5 h-3.5" />
            {/if}
          {/if}
        </Button>
      </div>
    </form>
  </div>
</div>
