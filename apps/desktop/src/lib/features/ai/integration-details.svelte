<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import {
  getAppById,
  getAppsByCategory,
  getCategoryName,
  getCategoryNameAr,
} from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import AppIcon from "$lib/features/ai/components/integrations/app-icon.svelte";
import AppConnectForm from "$lib/features/ai/components/integrations/app-connect-form.svelte";
import { l } from "$lib/core/i18n";
import { toast } from "@orbis/ui/sonner";

let { orgId, appId }: { orgId: string; appId: string } = $props();
const queryClient = useQueryClient();

const app = $derived(getAppById(appId));
const relatedApps = $derived(
  app
    ? getAppsByCategory(app.category)
        .filter((a: OrbisApp) => a.id !== app.id)
        .slice(0, 4)
    : [],
);
const basePath = $derived(`/${orgId}/ai-center/integrations`);

let isConnecting = $state(false);
let connectError = $state<string | null>(null);

const providerName = $derived(app?.adapter_name ?? appId);

const statusQuery = createQuery(() => ({
  queryKey: ["integration-status", orgId, providerName],
  queryFn: async () => {
    try {
      const status = await api.getIntegrationStatus(providerName);
      return status?.connected ?? false;
    } catch {
      return false;
    }
  },
  enabled: !!providerName,
  staleTime: 30 * 1000,
}));

const isConnected = $derived(statusQuery.data ?? false);

/** Simple markdown bold and link parser */
function renderDescription(text: string): string {
  return text
    .replace(
      /\*\*(.*?)\*\*/g,
      "<strong class='text-foreground font-medium'>$1</strong>",
    )
    .replace(
      /\[(.*?)\]\((.*?)\)/g,
      "<a href='$2' target='_blank' rel='noopener noreferrer' class='text-primary hover:underline'>$1</a>",
    )
    .replace(
      /\n\n/g,
      "</p><p class='mb-3 text-sm text-muted-foreground leading-relaxed'>",
    )
    .replace(/\n/g, "<br />");
}

async function handleConnect(values: Record<string, unknown>) {
  isConnecting = true;
  connectError = null;
  try {
    const secrets: Record<string, string> = {};
    const settings: Record<string, unknown> = {};

    if (app) {
      for (const s of app.settings) {
        const val = values[s.id];
        if (val !== undefined && val !== null && val !== "") {
          if (
            s.type === "password" ||
            s.id.includes("secret") ||
            s.id.includes("key") ||
            s.id.includes("token")
          ) {
            secrets[s.id] = String(val);
          } else {
            settings[s.id] = val;
          }
        }
      }
    }
    for (const [k, v] of Object.entries(values)) {
      if (k in secrets || k in settings) continue;
      if (
        k.includes("secret") ||
        k.includes("key") ||
        k.includes("token") ||
        k.includes("password")
      ) {
        secrets[k] = String(v);
      } else {
        settings[k] = v;
      }
    }

    await api.connectApp({
      provider_name: providerName,
      category: app?.category || "ai-provider",
      env: "production",
      secrets,
      settings,
    });

    await queryClient.invalidateQueries({
      queryKey: ["integration-status", orgId, providerName],
    });
    await queryClient.invalidateQueries({
      queryKey: ["integration"],
    });
    toast.success(
      m.ui_integration_connected({ name: app?.name ?? providerName }),
    );
  } catch (err) {
    connectError = extractErrorMessage(err);
    toast.error(connectError);
  } finally {
    isConnecting = false;
  }
}

async function handleDisconnect() {
  try {
    await api.disconnectApp(providerName);
    await queryClient.invalidateQueries({
      queryKey: ["integration-status", orgId, providerName],
    });
    await queryClient.invalidateQueries({
      queryKey: ["integration"],
    });
    toast.success(
      m.ui_integration_disconnected({ name: app?.name ?? providerName }),
    );
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

const activeFeatures = $derived(
  app ? (document.documentElement.dir === "rtl" && app.features_ar ? app.features_ar : app.features) : []
);
</script>

{#if app}
  <div class="h-full overflow-y-auto">
    <div class="max-w-5xl mx-auto px-6 py-8">
      <!-- Breadcrumb -->
      <nav aria-label="Breadcrumb" class="mb-8">
        <ol class="flex items-center gap-2 text-xs">
          <li>
            <a
              href={basePath}
              class="text-muted-foreground hover:text-foreground transition-colors"
            >
              {m.ai_center_nav_integrations()}
            </a>
          </li>
          <li class="text-muted-foreground/50">/</li>
          <li class="text-foreground font-medium">
            {app.name}
          </li>
        </ol>
      </nav>

      <!-- 3-Column Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-10 lg:gap-14">
        <!-- ─── Main Content (2/3) ─── -->
        <div class="lg:col-span-2 space-y-8 text-start">
          <!-- Header -->
          <div class="flex items-start gap-5">
            <div
              class="w-16 h-10 flex items-center justify-start text-foreground shrink-0"
            >
              <AppIcon icon={app.icon} logo={app.logo} size={32} class="w-full h-full [&>svg]:max-w-17 [&>svg]:max-h-9" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2.5 mb-1.5">
                <h1 class="text-xl font-semibold text-foreground truncate">
                  {app.name}
                </h1>
                {#if app.beta}
                  <span class="text-[10px] font-medium bg-primary/10 text-primary px-2 py-0.5 rounded-md shrink-0">
                    {m.ai_center_integrations_status_beta()}
                  </span>
                {/if}
                {#if !app.active}
                  <span class="text-[10px] font-medium bg-muted text-muted-foreground px-2 py-0.5 rounded-md shrink-0">
                    {m.ui_copy_coming_soon_3ac6c043()}
                  </span>
                {/if}
              </div>
              <p class="text-xs text-muted-foreground">
                {l(getCategoryNameAr(app.category), getCategoryName(app.category))}
              </p>
            </div>
          </div>

          <!-- Short Description -->
          <p class="text-sm text-foreground leading-relaxed">
            {l(app.short_description_ar, app.short_description)}
          </p>

          <!-- Full Description -->
          {#if app.description}
            <div class="border-t border-border/40 pt-6">
              <div class="prose-sm">
                <p class="mb-3 text-sm text-muted-foreground leading-relaxed">
                  {@html renderDescription(l(app.description_ar ?? "", app.description))}
                </p>
              </div>
            </div>
          {/if}

          <!-- Features -->
          {#if activeFeatures && activeFeatures.length > 0}
            <Card>
              <CardHeader>
                <CardTitle>
                  {m.ui_copy_key_features_ce213b9d()}
                </CardTitle>
              </CardHeader>
              <CardContent class="p-6">
                <ul class="space-y-3">
                  {#each activeFeatures as feature}
                    <li class="flex items-start gap-2.5">
                      <div class="w-1.5 h-1.5 bg-primary rounded-full mt-1.5 shrink-0"></div>
                      <span class="text-xs text-muted-foreground leading-relaxed">{feature}</span>
                    </li>
                  {/each}
                </ul>
              </CardContent>
            </Card>
          {/if}
        </div>

        <!-- ─── Sidebar (1/3) ─── -->
        <div class="lg:col-span-1">
          <div class="sticky top-6 space-y-5 text-start">
            <!-- CTA Card -->
            <Card>
              <CardContent class="p-5">
                {#if isConnected}
                  <div class="flex items-center gap-2 mb-3">
                    <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                    <h3 class="text-sm font-medium text-emerald-600 dark:text-emerald-400">
                      {m.common_account_identities_connected()}
                    </h3>
                  </div>
                  <p class="text-xs text-muted-foreground mb-4 leading-relaxed">
                    {m.ui_integration_connected_operational({ name: app.name })}
                  </p>
                  <Button class="w-full" size="sm" variant="outline" onclick={handleDisconnect}>
                    {m.ui_copy_disconnect_c584faa5()}
                  </Button>
                {:else}
                  <h3 class="text-sm font-medium text-foreground mb-3">
                    {app.active ? m.ui_copy_get_started_c431c86e() : m.ui_copy_coming_soon_3ac6c043()}
                  </h3>
                  <p class="text-xs text-muted-foreground mb-4 leading-relaxed">
                    {app.active
                      ? m.ui_integration_connect_for_automation({ name: app.name })
                      : m.ui_integration_work_in_progress({ name: app.name })}
                  </p>
                  {#if connectError}
                    <p class="text-[10px] text-red-500 mb-2">{connectError}</p>
                  {/if}
                  <Button
                    class="w-full"
                    size="sm"
                    disabled={!app.active || isConnecting}
                    onclick={() => {
                      const formEl = document.querySelector('form');
                      if (formEl) {
                        formEl.requestSubmit();
                      }
                    }}
                  >
                    {isConnecting ? m.setup_connection_connecting() : app.active ? m.ui_integration_connect({ name: app.name }) : m.ai_center_integrations_status_coming_soon()}
                  </Button>
                {/if}
              </CardContent>
            </Card>

            <!-- Settings Form -->
            {#if app.settings.length > 0 && app.active && !isConnected}
              <Card>
                <CardHeader>
                  <CardTitle>
                    {m.ui_copy_configuration_9ecff7d2()}
                  </CardTitle>
                </CardHeader>
                <CardContent class="p-5">
                  <AppConnectForm settings={app.settings} onsubmit={handleConnect} />
                </CardContent>
              </Card>
            {/if}

            <!-- Related Apps -->
            {#if relatedApps.length > 0}
              <Card>
                <CardHeader>
                  <CardTitle>
                    {m.ui_copy_related_integrations_ab667842()}
                  </CardTitle>
                </CardHeader>
                <CardContent class="p-5 space-y-3">
                  {#each relatedApps as related}
                    <a
                      href="{basePath}/{related.id}"
                      class="flex items-center gap-3 group"
                    >
                      <div
                        class="w-8 h-5 flex items-center justify-start text-foreground shrink-0"
                      >
                        <AppIcon icon={related.icon} logo={related.logo} size={18} class="w-full h-full [&>svg]:max-w-7 [&>svg]:max-h-4.5" />
                      </div>
                      <span
                        class="text-xs text-foreground group-hover:text-muted-foreground
                               transition-colors truncate"
                      >
                        {related.name}
                      </span>
                    </a>
                  {/each}
                </CardContent>
              </Card>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
{:else}
  <!-- App Not Found -->
  <div class="h-full flex items-center justify-center">
    <div class="text-center">
      <h2 class="text-sm font-medium text-foreground mb-2">
        {m.ui_copy_integration_not_found_a8511a65()}
      </h2>
      <p class="text-xs text-muted-foreground mb-4">
        {m.ui_integration_not_found({ integrationId: appId })}
      </p>
      <Button size="sm" variant="outline" href={basePath}>
        {m.ui_copy_back_to_integrations_26602ac9()}
      </Button>
    </div>
  </div>
{/if}
