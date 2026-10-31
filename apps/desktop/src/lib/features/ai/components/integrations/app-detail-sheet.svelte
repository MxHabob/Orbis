<script lang="ts">
import { goto } from "$app/navigation";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import type { OrbisApp } from "@orbis/app-store";
import { api, extractErrorMessage } from "$lib/api";
import { l } from "$lib/core/i18n";
import { renderReleaseMarkdown } from "$lib/utils/release-markdown";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { toast } from "@orbis/ui/sonner";
import AppConnectSheet from "./app-connect-sheet.svelte";
import AppIcon from "./app-icon.svelte";
import { getCategoryName, getCategoryNameAr } from "@orbis/app-store";

let {
  open = $bindable(false),
  orgId,
  app,
  onclose,
  onChange,
}: {
  open: boolean;
  orgId: string;
  app: OrbisApp;
  onclose?: () => void;
  onChange?: () => void;
} = $props();

const queryClient = useQueryClient();
const providerName = $derived(
  app.category === "storage" ? app.id : (app.adapter_name ?? app.id),
);
const legacyProviderName = $derived(
  app.legacy_provider_name ??
    (app.category !== "storage" &&
    app.adapter_name &&
    app.adapter_name !== providerName
      ? app.adapter_name
      : null),
);
let isConnecting = $state(false);
let isDisconnecting = $state(false);
let connectSheetOpen = $state(false);
let connectError = $state<string | null>(null);
let openAccordionItems = $state(["description", "features"]);

function handleOpenChange(value: boolean) {
  open = value;
  if (value) {
    openAccordionItems = ["description", "features"];
  } else {
    onclose?.();
  }
}

const statusQuery = createQuery(() => ({
  queryKey: ["integration-status", orgId, providerName],
  queryFn: async () => {
    const status = await api.getIntegrationStatus(providerName);
    if (!status?.connected && legacyProviderName) {
      return api.getIntegrationStatus(legacyProviderName);
    }
    return status;
  },
  enabled: !!providerName,
  staleTime: 30 * 1000,
}));

const isConnected = $derived(statusQuery.data?.connected ?? false);
const connectedProviderName = $derived(
  statusQuery.data?.config?.provider_name ?? providerName,
);
const activeFeatures = $derived(
  typeof document !== "undefined" &&
    document.documentElement.dir === "rtl" &&
    app.features_ar
    ? app.features_ar
    : app.features,
);
const appName = $derived(l(app.name_ar, app.name) ?? app.name);

async function handleConnect(values: Record<string, unknown>) {
  isConnecting = true;
  connectError = null;
  try {
    const secrets: Record<string, string> = {};
    const settings: Record<string, unknown> = {};

    for (const setting of app.settings) {
      const value = values[setting.id];
      if (value === undefined || value === null || value === "") continue;
      if (
        setting.type === "password" ||
        setting.id.includes("secret") ||
        setting.id.includes("key") ||
        setting.id.includes("token")
      ) {
        secrets[setting.id] = String(value);
      } else {
        settings[setting.id] = value;
      }
    }

    for (const [key, value] of Object.entries(values)) {
      if (key in secrets || key in settings) continue;
      if (
        key.includes("secret") ||
        key.includes("key") ||
        key.includes("token") ||
        key.includes("password")
      ) {
        secrets[key] = String(value);
      } else {
        settings[key] = value;
      }
    }
    if (
      app.provider_trait === "NotificationProvider" &&
      app.notification_channels?.length
    ) {
      settings.__notification_channels = app.notification_channels;
    }

    await api.connectApp({
      provider_name: providerName,
      category: app.category || "ai-provider",
      env: "production",
      secrets,
      settings,
    });

    await Promise.all([
      queryClient.invalidateQueries({
        queryKey: ["integration-status", orgId, providerName],
      }),
      ...(legacyProviderName
        ? [
            queryClient.invalidateQueries({
              queryKey: ["integration-status", orgId, legacyProviderName],
            }),
          ]
        : []),
      queryClient.invalidateQueries({ queryKey: ["integration"] }),
    ]);
    toast.success(m.ui_integration_connected({ name: app.name }));
    connectSheetOpen = false;
    onChange?.();
    if (app.id === "orbis_mobile") {
      await goto(localizeHref(`/${orgId}/settings/mulverse`));
    }
  } catch (error) {
    connectError = extractErrorMessage(error);
    toast.error(connectError);
  } finally {
    isConnecting = false;
  }
}

async function handleDisconnect() {
  if (isDisconnecting) return;
  isDisconnecting = true;
  try {
    await api.disconnectApp(connectedProviderName);
    await Promise.all([
      queryClient.invalidateQueries({
        queryKey: ["integration-status", orgId, providerName],
      }),
      ...(legacyProviderName
        ? [
            queryClient.invalidateQueries({
              queryKey: ["integration-status", orgId, legacyProviderName],
            }),
          ]
        : []),
      queryClient.invalidateQueries({ queryKey: ["integration"] }),
    ]);
    toast.success(m.ui_integration_disconnected({ name: app.name }));
    onChange?.();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isDisconnecting = false;
  }
}
</script>

<Sheet bind:open onOpenChange={handleOpenChange}>
  <SheetContent
    side="end"
    contentClass="data-[side=end]:w-[min(32rem,calc(100vw-1rem))] data-[side=end]:sm:max-w-[min(32rem,calc(100vw-1rem))]"
    class="flex h-full flex-col overflow-hidden p-0"
  >
    <div
      class="relative flex h-40 shrink-0 items-center justify-center overflow-hidden bg-muted/30"
      aria-hidden="true"
    >
      <div class="absolute inset-0 opacity-40 [background-image:radial-gradient(circle,var(--color-border)_1px,transparent_1px)] [background-size:10px_10px]"></div>
      <AppIcon
        icon={app.icon}
        logo={app.logo}
        size={80}
        class="relative z-10 h-20 w-64 justify-center [&>svg]:!h-20 [&>svg]:!w-auto [&>svg]:max-w-full [&>img]:max-h-20 [&>img]:max-w-full"
      />
    </div>

    <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <SheetHeader class="shrink-0 border-b border-border/50 px-6 py-5 text-start">
        <div class="flex items-center justify-between gap-4">
          <div class="min-w-0">
            <SheetTitle class="truncate text-lg leading-normal">
              {appName}
            </SheetTitle>
            <p class="mt-2 text-xs text-muted-foreground">
              {l(getCategoryNameAr(app.category), getCategoryName(app.category))}
            </p>
          </div>
          <div class="flex shrink-0 items-center gap-2">
            {#if app.beta}
              <Badge variant="secondary">{m.ai_center_integrations_status_beta()}</Badge>
            {/if}
            {#if isConnected}
              <Badge variant="outline" class="border-primary/30 text-primary">
                {m.ai_center_integrations_status_connected()}
              </Badge>
            {/if}
          </div>
          {#if statusQuery.isError}
            <div class="flex items-center justify-between gap-3 rounded-lg border border-destructive/30 bg-destructive/10 p-3 text-xs text-destructive" role="alert">
              <span>{extractErrorMessage(statusQuery.error) || m.common_action_failed()}</span>
              <Button
                variant="ghost"
                size="sm"
                onclick={() => void statusQuery.refetch()}
              >
                {m.onboarding_retry()}
              </Button>
            </div>
          {/if}
        </div>
        <div class="flex gap-2 pt-3">
          {#if isConnected}
            <Button variant="outline" size="sm" disabled={isDisconnecting} onclick={handleDisconnect}>
              {m.ui_copy_disconnect_c584faa5()}
            </Button>
          {:else}
            <Button
              size="sm"
              disabled={!app.active || isConnecting || statusQuery.isError}
              onclick={() => {
                connectError = null;
                connectSheetOpen = true;
              }}
            >
              {app.active
                ? m.ui_integration_connect({ name: appName })
                : m.ai_center_integrations_status_coming_soon()}
            </Button>
          {/if}
        </div>
      </SheetHeader>

      {#if app.category === "storage" && isConnected && statusQuery.data?.is_healthy === false}
        <div class="mx-6 mt-4 flex items-start justify-between gap-3 rounded-lg border border-destructive/30 bg-destructive/10 p-3 text-xs text-destructive" role="alert">
          <span>{statusQuery.data.health_error || m.common_action_failed()}</span>
          <Button
            variant="ghost"
            size="sm"
            onclick={() => void statusQuery.refetch()}
          >
            {m.onboarding_retry()}
          </Button>
        </div>
      {/if}

      <div class="min-h-0 flex-1 overflow-y-auto scrollbar-hide px-6 pb-6">
        <Accordion
          type="multiple"
          bind:value={openAccordionItems}
          class="mt-3"
        >
          {#if app.description || app.short_description}
            <AccordionItem value="description" class="border-b-0">
              <AccordionTrigger>{m.ai_ui_integration_how_it_works()}</AccordionTrigger>
              <AccordionContent class="text-sm text-muted-foreground">
                <p class="mb-3 text-sm leading-relaxed">
                  {l(app.short_description_ar, app.short_description)}
                </p>
                {#if app.description}
                  <div class="prose-sm max-w-none text-sm leading-relaxed">
                    {@html renderReleaseMarkdown(l(app.description_ar ?? "", app.description))}
                  </div>
                {/if}
              </AccordionContent>
            </AccordionItem>
          {/if}

          {#if activeFeatures.length > 0}
            <AccordionItem value="features" class="border-b-0">
              <AccordionTrigger>{m.ui_copy_key_features_ce213b9d()}</AccordionTrigger>
              <AccordionContent>
                <ul class="space-y-3">
                  {#each activeFeatures as feature (feature)}
                    <li class="flex items-start gap-2.5">
                      <span class="mt-1.5 size-1.5 shrink-0 rounded-full bg-primary"></span>
                      <span class="text-sm leading-relaxed text-muted-foreground">{feature}</span>
                    </li>
                  {/each}
                </ul>
              </AccordionContent>
            </AccordionItem>
          {/if}
        </Accordion>
      </div>
    </div>
  </SheetContent>
</Sheet>

{#if connectSheetOpen}
  <AppConnectSheet
    bind:open={connectSheetOpen}
    {app}
    submitting={isConnecting}
    error={connectError}
    onsubmit={handleConnect}
  />
{/if}
