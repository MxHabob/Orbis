<!--
@component IntegrationDetailPage
App detail page — matches midday.ai/integrations/[slug] design.
Layout: Breadcrumb + 3-column grid (2 main + 1 sidebar).
Main: App header + description + features.
Sidebar: CTA card + settings form + related apps.
-->
<script lang="ts">
import { getAppById, getAppsByCategory, getCategoryName, getCategoryNameAr } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import AppIcon from "$lib/features/ai-center/components/integrations/app-icon.svelte";
import AppConnectForm from "$lib/features/ai-center/components/integrations/app-connect-form.svelte";
import { l } from "$lib/core/i18n";

let { data }: { data: { orgId: string; appId: string } } = $props();

const app = $derived(getAppById(data.appId));
const relatedApps = $derived(
  app
    ? getAppsByCategory(app.category)
        .filter((a: OrbisApp) => a.id !== app.id)
        .slice(0, 4)
    : []
);
const basePath = $derived(`/${data.orgId}/ai-center/integrations`);

let isConnecting = $state(false);
let isConnected = $state(false);
let connectError = $state<string | null>(null);

// Check current status on load
$effect(() => {
  if (data.appId) {
    commands.aiCenter.getIntegrationStatus(data.appId)
      .then((status: any) => {
        isConnected = status?.connected ?? false;
      })
      .catch(() => { isConnected = false; });
  }
});

/** Simple markdown bold and link parser */
function renderDescription(text: string): string {
  return text
    .replace(/\*\*(.*?)\*\*/g, "<strong class='text-foreground font-medium'>$1</strong>")
    .replace(/\[(.*?)\]\((.*?)\)/g, "<a href='$2' target='_blank' rel='noopener noreferrer' class='text-primary hover:underline'>$1</a>")
    .replace(/\n\n/g, "</p><p class='mb-3 text-sm text-muted-foreground leading-relaxed'>")
    .replace(/\n/g, "<br />");
}

async function handleConnect(values: Record<string, unknown>) {
  isConnecting = true;
  connectError = null;
  try {
    await commands.aiCenter.connectIntegration(data.appId, values);
    isConnected = true;
  } catch (e: any) {
    connectError = e?.message ?? "Connection failed";
  } finally {
    isConnecting = false;
  }
}

async function handleDisconnect() {
  try {
    await commands.aiCenter.disconnectIntegration(data.appId);
    isConnected = false;
  } catch (e: any) {
    console.error("[AI Center] Disconnect failed:", e);
  }
}

const activeFeatures = $derived(
  app
    ? l(app.features_ar, app.features)
    : []
);
</script>

{#if app}
  <div class="h-full w-full overflow-y-auto scrollbar-hide flex flex-col bg-background">
    <div class="px-8 py-6 w-full max-w-none lg:max-w-7xl mx-auto flex-1">
      <!-- Breadcrumb -->
      <nav class="mb-7">
        <ol class="flex items-center gap-1.5 text-xs">
          <li>
            <a
              href={basePath}
              class="text-muted-foreground hover:text-foreground transition-colors"
            >
              {l("التكاملات", "Integrations")}
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
        <div class="lg:col-span-2 space-y-8">
          <!-- Header -->
          <div class="flex items-start gap-5">
            <div
              class="w-14 h-14 flex items-center justify-center rounded-xl
                     bg-muted/50 text-foreground shrink-0"
            >
              <AppIcon icon={app.icon} logo={app.logo} size={28} />
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2.5 mb-1.5">
                <h1 class="text-xl font-semibold text-foreground truncate">
                  {app.name}
                </h1>
                {#if app.beta}
                  <span class="text-[10px] font-medium bg-primary/10 text-primary px-2 py-0.5 rounded-md shrink-0">
                    {l("تجريبي", "Beta")}
                  </span>
                {/if}
                {#if !app.active}
                  <span class="text-[10px] font-medium bg-muted text-muted-foreground px-2 py-0.5 rounded-md shrink-0">
                    {l("قريباً", "Coming soon")}
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
            <div class="border border-border/60 p-5 lg:p-6 rounded-lg">
              <h2 class="text-sm font-medium text-foreground mb-4">
                {l("المميزات الرئيسية", "Key Features")}
              </h2>
              <ul class="space-y-3">
                {#each activeFeatures as feature}
                  <li class="flex items-start gap-2.5">
                    <div class="w-1 h-1 bg-foreground rounded-full mt-1.5 shrink-0"></div>
                    <span class="text-xs text-muted-foreground leading-relaxed">{feature}</span>
                  </li>
                {/each}
              </ul>
            </div>
          {/if}
        </div>

        <!-- ─── Sidebar (1/3) ─── -->
        <div class="lg:col-span-1">
          <div class="sticky top-6 space-y-5">
            <!-- CTA Card -->
            <div class="border border-border/60 p-5 rounded-lg">
              {#if isConnected}
                <div class="flex items-center gap-2 mb-3">
                  <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                  <h3 class="text-sm font-medium text-emerald-600 dark:text-emerald-400">
                    {l("متصل", "Connected")}
                  </h3>
                </div>
                <p class="text-xs text-muted-foreground mb-4 leading-relaxed">
                  {l(`${app.name} متصل ويعمل بشكل صحيح.`, `${app.name} is connected and operational.`)}
                </p>
                <Button class="w-full" size="sm" variant="outline" onclick={handleDisconnect}>
                  {l("قطع الاتصال", "Disconnect")}
                </Button>
              {:else}
                <h3 class="text-sm font-medium text-foreground mb-3">
                  {app.active ? l("ابدأ الآن", "Get started") : l("قريباً", "Coming soon")}
                </h3>
                <p class="text-xs text-muted-foreground mb-4 leading-relaxed">
                  {app.active
                    ? l(`قم بربط ${app.name} بنظام Orbis لبدء الأتمتة.`, `Connect ${app.name} to your Orbis system and start automating.`)
                    : l(`نعمل حالياً على تكامل ${app.name}. ترقبوا جديدنا.`, `We're working on the ${app.name} integration. Stay tuned.`)}
                </p>
                {#if connectError}
                  <p class="text-[10px] text-red-500 mb-2">{connectError}</p>
                {/if}
                <Button class="w-full" size="sm" disabled={!app.active || isConnecting}>
                  {isConnecting ? l("جاري الاتصال...", "Connecting...") : app.active ? l(`ربط ${app.name}`, `Connect ${app.name}`) : l("قريباً", "Coming Soon")}
                </Button>
              {/if}
            </div>

            <!-- Settings Form -->
            {#if app.settings.length > 0 && app.active && !isConnected}
              <div class="border border-border/60 p-5 rounded-lg">
                <h3 class="text-xs font-medium text-muted-foreground mb-4 uppercase tracking-wider">
                  {l("الإعدادات", "Configuration")}
                </h3>
                <AppConnectForm settings={app.settings} onsubmit={handleConnect} />
              </div>
            {/if}

            <!-- Related Apps -->
            {#if relatedApps.length > 0}
              <div class="border border-border/60 p-5 rounded-lg">
                <h3 class="text-xs font-medium text-muted-foreground mb-4 uppercase tracking-wider">
                  {l("التكاملات ذات الصلة", "Related integrations")}
                </h3>
                <div class="space-y-3">
                  {#each relatedApps as related}
                    <a
                      href="{basePath}/{related.id}"
                      class="flex items-center gap-3 group"
                    >
                      <div
                        class="w-8 h-8 flex items-center justify-center rounded-md
                               bg-muted/50 text-foreground shrink-0"
                      >
                        <AppIcon icon={related.icon} logo={related.logo} size={16} />
                      </div>
                      <span
                        class="text-xs text-foreground group-hover:text-muted-foreground
                               transition-colors truncate"
                      >
                        {related.name}
                      </span>
                    </a>
                  {/each}
                </div>
              </div>
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
        {l("التكامل غير موجود", "Integration not found")}
      </h2>
      <p class="text-xs text-muted-foreground mb-4">
        {l(`التكامل "${data.appId}" غير موجود.`, `The integration "${data.appId}" does not exist.`)}
      </p>
      <Button size="sm" variant="outline" href={basePath}>
        {l("العودة للتكاملات", "Back to Integrations")}
      </Button>
    </div>
  </div>
{/if}
