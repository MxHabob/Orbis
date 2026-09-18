<!--
  Connection Setup Screen — First-time database connection setup.
  Clean, minimal UI built using @orbis/ui Card, Button, Input, Logo, and standard design tokens.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Logo } from "@orbis/ui/logo";
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
} from "@lucide/svelte";

const isRTL = $derived(getLocale().startsWith("ar"));

let selectedMode: ConnectionMode = $state("local");
let serverUrl = $state("");
let dbFilename = $state("orbis.db");
let isConnecting = $state(false);
let isCheckingHealth = $state(false);
let error = $state("");
let healthResult = $state<RemoteHealthResult | null>(null);

async function checkHealth() {
  if (!serverUrl.trim()) return;
  isCheckingHealth = true;
  healthResult = null;
  error = "";
  try {
    healthResult = await api.connectionCheckRemoteHealth(serverUrl.trim());
    if (!healthResult.reachable) {
      error = healthResult.error || m.setup_connection_unreachable();
    }
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isCheckingHealth = false;
  }
}

async function connect() {
  isConnecting = true;
  error = "";
  try {
    await api.connectionSetConfig(
      selectedMode,
      selectedMode === "remote" ? serverUrl.trim() : undefined,
      selectedMode === "local" ? dbFilename.trim() : undefined
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
      error = result.message || m.setup_connection_failed();
    }
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isConnecting = false;
  }
}
</script>

<svelte:head>
  <title>{m.setup_connection_page_title()}</title>
</svelte:head>

<div class="w-full max-w-3xl">
  <Card class="bg-card border border-border shadow-2xl rounded-2xl overflow-hidden flex flex-col md:flex-row">
    <!-- Left Brand Column -->
    <div class="w-full md:w-20 bg-primary flex md:flex-col items-center justify-between md:justify-start p-4 md:py-8 gap-4 text-primary-foreground shrink-0 select-none">
      <div class="flex items-center justify-center text-primary-foreground">
        <Logo class="size-8 text-primary-foreground" dotClass="fill-white" />
      </div>
    </div>

    <!-- Main Content Form Area -->
    <div class="flex-1 p-6 sm:p-8 lg:p-10 flex flex-col justify-between min-w-0">
      <div>
        <!-- Header -->
        <div class="mb-6">
          <h2 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
            {m.setup_connection_title()}
          </h2>
          <p class="text-xs text-muted-foreground mt-1">
            {m.setup_connection_subtitle()}
          </p>
        </div>

        {#if error}
          <div class="mb-5 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{error}</span>
          </div>
        {/if}

        <!-- Mode Selection Cards -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-6">
          <!-- Local Mode -->
          <button
            type="button"
            onclick={() => {
              selectedMode = "local";
              error = "";
              healthResult = null;
            }}
            class="flex items-start gap-3 p-4 rounded-xl border text-start transition-all duration-200 cursor-pointer {selectedMode === 'local'
              ? 'bg-primary/10 border-primary/50 text-foreground shadow-sm'
              : 'bg-card hover:bg-muted/40 border-border text-muted-foreground'}"
          >
            <div class="p-2 rounded-lg shrink-0 {selectedMode === 'local' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
              <Database class="w-4 h-4" />
            </div>
            <div class="min-w-0">
              <div class="font-semibold text-sm text-foreground">
                {m.setup_connection_local_title()}
              </div>
              <div class="text-[11px] text-muted-foreground mt-0.5">
                {m.setup_connection_local_desc()}
              </div>
            </div>
          </button>

          <!-- Remote Mode -->
          <button
            type="button"
            onclick={() => {
              selectedMode = "remote";
              error = "";
            }}
            class="flex items-start gap-3 p-4 rounded-xl border text-start transition-all duration-200 cursor-pointer {selectedMode === 'remote'
              ? 'bg-primary/10 border-primary/50 text-foreground shadow-sm'
              : 'bg-card hover:bg-muted/40 border-border text-muted-foreground'}"
          >
            <div class="p-2 rounded-lg shrink-0 {selectedMode === 'remote' ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
              <Globe class="w-4 h-4" />
            </div>
            <div class="min-w-0">
              <div class="font-semibold text-sm text-foreground">
                {m.setup_connection_remote_title()}
              </div>
              <div class="text-[11px] text-muted-foreground mt-0.5">
                {m.setup_connection_remote_desc()}
              </div>
            </div>
          </button>
        </div>

        <!-- Mode Specific Configuration -->
        {#if selectedMode === "local"}
          <div class="space-y-3 p-4 rounded-xl bg-muted/20 border border-border/60">
            <div>
              <label for="dbFilename" class="block text-xs font-medium text-foreground mb-1.5">
                {m.setup_connection_db_file_label()}
              </label>
              <Input
                id="dbFilename"
                type="text"
                bind:value={dbFilename}
                class="h-10 text-xs bg-background border-input"
                placeholder="orbis.db"
              />
              <p class="text-[10px] text-muted-foreground mt-1">
                {m.setup_connection_db_file_hint()}
              </p>
            </div>
          </div>
        {:else}
          <div class="space-y-3 p-4 rounded-xl bg-muted/20 border border-border/60">
            <div>
              <label for="serverUrl" class="block text-xs font-medium text-foreground mb-1.5">
                {m.setup_connection_server_url_label()}
              </label>
              <div class="flex gap-2">
                <Input
                  id="serverUrl"
                  type="url"
                  bind:value={serverUrl}
                  class="h-10 text-xs bg-background border-input flex-1"
                  placeholder="https://orbis.school.edu"
                />
                <Button
                  type="button"
                  variant="outline"
                  onclick={checkHealth}
                  disabled={!serverUrl.trim() || isCheckingHealth}
                  class="h-10 px-3 text-xs gap-1.5 shrink-0 cursor-pointer border-border"
                >
                  {#if isCheckingHealth}
                    <Loader2 class="w-3.5 h-3.5 animate-spin" />
                  {:else}
                    <Activity class="w-3.5 h-3.5 text-primary" />
                  {/if}
                  <span>{m.setup_connection_test_btn()}</span>
                </Button>
              </div>
            </div>

            {#if healthResult}
              <div class="flex items-center gap-2 p-2.5 rounded-lg text-xs {healthResult.reachable ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-destructive/10 text-destructive border border-destructive/20'}">
                {#if healthResult.reachable}
                  <CheckCircle2 class="w-4 h-4 shrink-0" />
                  <span>
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

      <!-- Action Button -->
      <div class="pt-6 mt-6 border-t border-border/40 flex items-center justify-end">
        <Button
          type="button"
          variant="default"
          onclick={connect}
          disabled={isConnecting || (selectedMode === "remote" && !serverUrl.trim())}
          class="inline-flex items-center gap-2 px-6 h-10 text-xs font-semibold cursor-pointer"
        >
          {#if isConnecting}
            <Loader2 class="w-4 h-4 animate-spin" />
            <span>{m.setup_connection_connecting()}</span>
          {:else}
            <span>{m.setup_connection_connect_btn()}</span>
            {#if isRTL}
              <ArrowLeft class="w-4 h-4" />
            {:else}
              <ArrowRight class="w-4 h-4" />
            {/if}
          {/if}
        </Button>
      </div>
    </div>
  </Card>
</div>
