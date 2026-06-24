<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let orgId = $derived(page.params.orgId);

// Form state
let isLoading = $state(true);
let isSaving = $state(false);

let name = $state("");
let logo = $state("");
let primaryColor = $state("#84cc16"); // default lime
let currency = $state("YER");
let timezone = $state("Asia/Aden");
let locale = $state("ar");
let dateFormat = $state("YYYY-MM-DD");
let hijriEnabled = $state(false);

async function loadOrganization() {
  if (!orgId) return;
  isLoading = true;
  try {
    const org = await commands.users.getOrganization(orgId);
    name = (org.name as string) || "";
    logo = (org.logo as string) || "";
    primaryColor = (org.primary_color as string) || "#84cc16";
    currency = (org.currency as string) || "YER";
    timezone = (org.timezone as string) || "Asia/Aden";
    locale = (org.locale as string) || "ar";
    dateFormat = (org.date_format as string) || "YYYY-MM-DD";
    hijriEnabled = (org.hijri_enabled as number) === 1;
  } catch (e) {
    console.error("Failed to load organization:", e);
    toast.error(m.error_load_overview());
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId) {
    loadOrganization();
  }
});

async function handleSave(e: Event) {
  e.preventDefault();
  if (!orgId) return;
  isSaving = true;
  try {
    const request = {
      name,
      logo: logo || null,
      primary_color: primaryColor,
      currency,
      timezone,
      locale,
      date_format: dateFormat,
      hijri_enabled: hijriEnabled ? 1 : 0,
    };
    await commands.users.updateOrganization(orgId, request);
    toast.success(m.settings_general_save_success());
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.settings_general_save_error({ error: errorMsg }));
  } finally {
    isSaving = false;
  }
}

// Lists for dropdown options
const locales = [
  { value: "ar", label: m.header_language_ar() },
  { value: "en", label: m.header_language_en() },
];

const dateFormats = [
  { value: "YYYY-MM-DD", label: "YYYY-MM-DD (2026-06-24)" },
  { value: "DD/MM/YYYY", label: "DD/MM/YYYY (24/06/2026)" },
  { value: "MM/DD/YYYY", label: "MM/DD/YYYY (06/24/2026)" },
];

const timezones = [
  { value: "Asia/Aden", label: "Asia/Aden (GMT+3)" },
  { value: "Asia/Riyadh", label: "Asia/Riyadh (GMT+3)" },
  { value: "Asia/Dubai", label: "Asia/Dubai (GMT+4)" },
  { value: "Africa/Cairo", label: "Africa/Cairo (GMT+3)" },
  { value: "UTC", label: "UTC (GMT+0)" },
];
</script>

<svelte:head>
  <title>{m.settings_general_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <!-- Header -->
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground">{m.settings_general_title()}</h1>
      <p class="text-xs text-muted-foreground">{m.settings_general_desc()}</p>
    </div>

    {#if isLoading}
      <div class="flex flex-col gap-6">
        {@render CardSkeleton()}
        {@render CardSkeleton()}
      </div>
    {:else}
      <form onsubmit={handleSave} class="flex flex-col gap-6">
        <!-- Section 1: Org Info & Branding -->
        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
          <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">{m.settings_general_section_info()}</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Org Name -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_name" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_name()}
              </label>
              <Input
                id="org_name"
                type="text"
                bind:value={name}
                placeholder={m.settings_general_name_placeholder()}
                required
                disabled={isSaving}
              />
            </div>

            <!-- Primary Color Colorpicker -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_color" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_color()}
              </label>
              <div class="flex gap-2">
                <input
                  id="org_color"
                  type="color"
                  bind:value={primaryColor}
                  class="w-10 h-10 rounded border border-border cursor-pointer bg-transparent"
                  disabled={isSaving}
                />
                <Input
                  type="text"
                  bind:value={primaryColor}
                  placeholder="#84cc16"
                  class="font-mono uppercase flex-1"
                  disabled={isSaving}
                />
              </div>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-6">
            <!-- Logo URL -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_logo" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_logo()}
              </label>
              <div class="flex items-center gap-4">
                {#if logo}
                  <img
                    src={logo}
                    alt="Logo"
                    class="w-12 h-12 rounded-lg object-contain bg-muted border border-border"
                    onerror={() => logo = ''}
                  />
                {:else}
                  <div class="w-12 h-12 rounded-lg bg-muted border border-border/80 flex items-center justify-center text-xs text-muted-foreground">
                    No Logo
                  </div>
                {/if}
                <Input
                  id="org_logo"
                  type="text"
                  bind:value={logo}
                  placeholder={m.settings_general_logo_placeholder()}
                  class="flex-1"
                  disabled={isSaving}
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Section 2: Regional Settings -->
        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
          <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">{m.settings_general_section_regional()}</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Currency -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_currency" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_currency()}
              </label>
              <Input
                id="org_currency"
                type="text"
                bind:value={currency}
                placeholder={m.settings_general_currency_placeholder()}
                class="uppercase font-mono"
                required
                disabled={isSaving}
              />
            </div>

            <!-- Timezone -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_timezone" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_timezone()}
              </label>
              <select
                id="org_timezone"
                bind:value={timezone}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
                disabled={isSaving}
              >
                {#each timezones as tz}
                  <option value={tz.value}>{tz.label}</option>
                {/each}
              </select>
            </div>

            <!-- Locale / Language -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_locale" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_locale()}
              </label>
              <select
                id="org_locale"
                bind:value={locale}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
                disabled={isSaving}
              >
                {#each locales as loc}
                  <option value={loc.value}>{loc.label}</option>
                {/each}
              </select>
            </div>

            <!-- Date Format -->
            <div class="flex flex-col gap-2 text-start">
              <label for="org_date_format" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_date_format()}
              </label>
              <select
                id="org_date_format"
                bind:value={dateFormat}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
                disabled={isSaving}
              >
                {#each dateFormats as df}
                  <option value={df.value}>{df.label}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- Hijri Calendar Toggle -->
          <div class="flex items-center justify-between border-t border-border/40 pt-4">
            <div class="flex flex-col gap-1 text-start">
              <label for="hijri_enabled" class="text-xs font-semibold text-foreground">
                {m.settings_general_hijri()}
              </label>
              <span class="text-[10px] text-muted-foreground">
                {m.settings_general_hijri_desc()}
              </span>
            </div>
            <Switch
              id="hijri_enabled"
              bind:checked={hijriEnabled}
              disabled={isSaving}
            />
          </div>
        </div>

        <!-- Submit Actions -->
        <div class="flex justify-end gap-3 pb-8">
          <Button
            type="submit"
            variant="lime"
            class="px-6 font-semibold h-10 text-sm"
            disabled={isSaving}
          >
            {isSaving ? m.settings_general_saving() : m.settings_general_save()}
          </Button>
        </div>
      </form>
    {/if}
  </div>
</div>

{#snippet CardSkeleton()}
  <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
    <div class="border-b border-border/60 pb-4">
      <Skeleton class="h-5 w-32" />
    </div>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="flex flex-col gap-2">
        <Skeleton class="h-3 w-16" />
        <Skeleton class="h-9 w-full" />
      </div>
      <div class="flex flex-col gap-2">
        <Skeleton class="h-3 w-16" />
        <Skeleton class="h-9 w-full" />
      </div>
    </div>
  </div>
{/snippet}
