<!--
  @component ConnectionSetup
  Database and remote server connection configuration.
-->
<script lang="ts">
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
    await api.connectionSetConfig({
      mode: values.mode as ConnectionMode,
      serverUrl: values.mode === "remote" ? values.serverUrl.trim() : undefined,
      dbFilename: values.mode === "local" ? values.dbFilename.trim() : undefined,
    });

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
        <Badge variant="outline" class="text-[10px] px-2.5 py-0.5 gap-1.5 text-muted-foreground font-mono bg-background/50">
          <ChartBar size={10} class="text-emerald-500 animate-pulse" />
          <span>V2.4 ENGINE</span>
        </Badge>
      </div>
    </div>

    <!-- Form & Architecture Split -->
    <form onsubmit={handleConnect} class="p-5 sm:p-7 lg:p-8 flex flex-col justify-between space-y-6">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 lg:gap-8 items-stretch">
        
        <!-- Left Column: Architecture Context (5 cols) -->
        <div class="lg:col-span-5 rounded-2xl bg-muted/10 border border-border/60 p-5 flex flex-col justify-between space-y-5 text-start">
          <div class="space-y-4">
            <div class="flex items-center gap-2 pb-1 border-b border-border/40">
              <Database size={15} class="text-primary" />
              <h3 class="font-bold text-xs sm:text-sm text-foreground">
                {m.ui_copy_system_architecture_c18b3d52()}
              </h3>
            </div>
            
            <p class="text-xs text-muted-foreground leading-relaxed">
              {m.ui_copy_orbis_supports_two_flexible_operating_mode_f562dda7()}
            </p>

            <div class="space-y-3">
              <div class="p-3 rounded-xl border border-border/50 bg-card/60 space-y-1">
                <div class="flex items-center gap-2 font-bold text-xs text-foreground">
                  <Computer size={13} class="text-primary" />
                  <span>{m.ui_copy_local_embedded_mode_9a445d7e()}</span>
                </div>
                <p class="text-[11px] text-muted-foreground leading-normal">
                  {m.ui_copy_operates_completely_offline_with_an_embedd_6b260871()}
                </p>
              </div>

              <div class="p-3 rounded-xl border border-border/50 bg-card/60 space-y-1">
                <div class="flex items-center gap-2 font-bold text-xs text-foreground">
                  <GlobeAlt size={13} class="text-primary" />
                  <span>{m.ui_copy_remote_cloud_mode_0bab5a3e()}</span>
                </div>
                <p class="text-[11px] text-muted-foreground leading-normal">
                  {m.ui_copy_connects_to_a_central_orbis_api_server_for_ed966b7f()}
                </p>
              </div>
            </div>
          </div>

          <div class="flex items-center gap-2 pt-2 border-t border-border/30 text-[11px] text-muted-foreground">
            <UserBadgeCheck size={12} class="text-primary shrink-0" />
            <span>{m.ui_copy_all_connections_and_data_are_strictly_isol_e420944a()}</span>
          </div>
        </div>

        <!-- Right Column: Mode Selection & Input Controls (7 cols) -->
        <div class="lg:col-span-7 flex flex-col justify-between space-y-5 text-start">
          <div class="space-y-5">
            {#if form.formError}
              <div class="flex items-center gap-2.5 p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs animate-in fade-in">
                <InfoCircle class="w-4 h-4 shrink-0" />
                <span>{form.formError}</span>
              </div>
            {/if}

            <!-- Mode Selector Cards -->
            <div class="space-y-2">
              <label class="text-xs font-bold text-foreground">
                {m.ui_copy_select_connection_mode_acd68c8e()}
              </label>
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                
                <!-- Local Option -->
                <button
                  type="button"
                  class="cursor-pointer text-start p-4 rounded-2xl border-2 transition-all flex flex-col justify-between gap-3 relative {form.values.mode === 'local' ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/60 bg-muted/5 hover:border-border hover:bg-muted/15'}"
                  onclick={() => { form.values.mode = "local"; form.clearError(); healthResult = null; }}
                >
                  <div class="flex items-center justify-between w-full">
                    <div class="size-9 rounded-xl flex items-center justify-center {form.values.mode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <Database size={16} />
                    </div>
                    {#if form.values.mode === "local"}
                      <div class="size-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center">
                        <CheckCircle size={12} />
                      </div>
                    {/if}
                  </div>
                  <div>
                    <h4 class="text-xs font-bold text-foreground">
                      {m.setup_connection_local_title()}
                    </h4>
                    <p class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2 leading-normal">
                      {m.setup_connection_local_desc()}
                    </p>
                  </div>
                </button>

                <!-- Remote Option -->
                <button
                  type="button"
                  class="cursor-pointer text-start p-4 rounded-2xl border-2 transition-all flex flex-col justify-between gap-3 relative {form.values.mode === 'remote' ? 'border-primary bg-primary/5 shadow-sm' : 'border-border/60 bg-muted/5 hover:border-border hover:bg-muted/15'}"
                  onclick={() => { form.values.mode = "remote"; form.clearError(); healthResult = null; }}
                >
                  <div class="flex items-center justify-between w-full">
                    <div class="size-9 rounded-xl flex items-center justify-center {form.values.mode === 'remote' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                      <GlobeAlt size={16} />
                    </div>
                    {#if form.values.mode === "remote"}
                      <div class="size-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center">
                        <CheckCircle size={12} />
                      </div>
                    {/if}
                  </div>
                  <div>
                    <h4 class="text-xs font-bold text-foreground">
                      {m.setup_connection_remote_title()}
                    </h4>
                    <p class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2 leading-normal">
                      {m.setup_connection_remote_desc()}
                    </p>
                  </div>
                </button>
              </div>
            </div>

            <!-- Parameters Input Box -->
            <div class="p-4 sm:p-5 rounded-2xl bg-muted/10 border border-border/60 space-y-4">
              {#if form.values.mode === "local"}
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
                      class="h-9 text-xs font-mono font-medium bg-background border-input"
                      placeholder="orbis.db"
                      style="direction: ltr;"
                    />
                  {/snippet}
                </FormField>
                <p class="text-[11px] text-muted-foreground flex items-center gap-1.5">
                  <Computer size={12} class="text-muted-foreground/80 shrink-0" />
                  <span>{m.setup_connection_db_file_hint()}</span>
                </p>
              {:else}
                <div class="space-y-2">
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
                          class="h-9 text-xs font-mono bg-background border-input flex-1"
                          placeholder="http://localhost:3000"
                          style="direction: ltr;"
                        />
                        <Button
                          type="button"
                          variant="outline"
                          size="sm"
                          class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
                          onclick={checkHealth}
                          disabled={isCheckingHealth || !form.values.serverUrl.trim()}
                        >
                          {#if isCheckingHealth}
                            <ArrowPath class="size-3.5 animate-spin" />
                          {:else}
                            <ChartBar class="size-3.5 text-primary" />
                          {/if}
                          <span>{m.setup_connection_test_btn()}</span>
                        </Button>
                      </div>
                    {/snippet}
                  </FormField>

                  {#if healthResult}
                    <div class="p-2.5 rounded-xl border text-xs flex items-center gap-2 {healthResult.reachable ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-destructive/10 border-destructive/20 text-destructive'}">
                      {#if healthResult.reachable}
                        <CheckCircle class="size-4 shrink-0" />
                        <span>
                          {m.setup_connection_connected_status({
                            status: healthResult.status ?? "ok",
                            version: healthResult.version ?? "1.0",
                          })}
                        </span>
                      {:else}
                        <InfoCircle class="size-4 shrink-0" />
                        <span>{healthResult.error ?? m.setup_connection_unreachable()}</span>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/if}
            </div>
          </div>

          <div class="pt-2">
            <Button
              type="submit"
              variant="default"
              class="w-full h-10 text-xs font-bold gap-2 cursor-pointer shadow-sm shadow-primary/20"
              disabled={form.isSubmitting || (form.values.mode === 'remote' && healthResult !== null && !healthResult.reachable)}
            >
              {#if form.isSubmitting}
                <ArrowPath class="size-4 animate-spin" />
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
        </div>
      </div>
    </form>
  </div>
</div>
