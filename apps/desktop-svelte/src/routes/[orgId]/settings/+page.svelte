<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Label } from "@orbis/ui-svelte/label";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
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
    const errorMsg = (e as any)?.message || "Unknown error";
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

<div class="h-full w-full overflow-y-auto bg-background scrollbar-hide">
  <div class="max-w-3xl mx-auto p-8 flex flex-col gap-10">
    <!-- Header -->
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground">{m.settings_general_title()}</h1>
      <p class="text-xs text-muted-foreground">{m.settings_general_desc()}</p>
    </div>

    {#if isLoading}
      <div class="flex flex-col gap-8">
        {@render SectionSkeleton()}
        {@render SectionSkeleton()}
      </div>
    {:else}
      <form onsubmit={handleSave} class="flex flex-col gap-10">
        <!-- Section 1: Org Info & Branding -->
        <div class="flex flex-col gap-6 text-start">
          <div class="border-b border-border/60 pb-3">
            <h2 class="text-sm font-semibold text-foreground">{m.settings_general_section_info()}</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Org Name -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="org_name" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_name()}
              </Label>
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
              <Label for="org_color" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_color()}
              </Label>
              <div class="flex gap-2">
                <input
                  id="org_color"
                  type="color"
                  bind:value={primaryColor}
                  class="w-9 h-9 rounded border border-border cursor-pointer bg-transparent"
                  disabled={isSaving}
                />
                <Input
                  type="text"
                  bind:value={primaryColor}
                  placeholder="#84cc16"
                  class="font-mono uppercase flex-1 h-9"
                  disabled={isSaving}
                />
              </div>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-6">
            <!-- Logo URL -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="org_logo" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_logo()}
              </Label>
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
        <div class="flex flex-col gap-6 text-start">
          <div class="border-b border-border/60 pb-3">
            <h2 class="text-sm font-semibold text-foreground">{m.settings_general_section_regional()}</h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Currency -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="org_currency" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_currency()}
              </Label>
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
              <Label for="org_timezone" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_timezone()}
              </Label>
              <Select type="single" value={timezone} onValueChange={(v) => (timezone = v ?? "")} disabled={isSaving}>
                <SelectTrigger class="h-9 w-full">
                  {timezones.find(tz => tz.value === timezone)?.label || timezone}
                </SelectTrigger>
                <SelectContent>
                  {#each timezones as tz}
                    <SelectItem value={tz.value}>{tz.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>

            <!-- Locale / Language -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="org_locale" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_locale()}
              </Label>
              <Select type="single" value={locale} onValueChange={(v) => (locale = v ?? "")} disabled={isSaving}>
                <SelectTrigger class="h-9 w-full">
                  {locales.find(loc => loc.value === locale)?.label || locale}
                </SelectTrigger>
                <SelectContent>
                  {#each locales as loc}
                    <SelectItem value={loc.value}>{loc.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>

            <!-- Date Format -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="org_date_format" class="text-xs font-medium text-muted-foreground">
                {m.settings_general_date_format()}
              </Label>
              <Select type="single" value={dateFormat} onValueChange={(v) => (dateFormat = v ?? "")} disabled={isSaving}>
                <SelectTrigger class="h-9 w-full">
                  {dateFormats.find(df => df.value === dateFormat)?.label || dateFormat}
                </SelectTrigger>
                <SelectContent>
                  {#each dateFormats as df}
                    <SelectItem value={df.value}>{df.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Hijri Calendar Toggle -->
          <div class="flex items-center justify-between border-t border-border/40 pt-4 mt-2">
            <div class="flex flex-col gap-1 text-start">
              <Label for="hijri_enabled" class="text-xs font-semibold text-foreground">
                {m.settings_general_hijri()}
              </Label>
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
        <div class="flex justify-end gap-3 pb-8 border-t border-border/40 pt-6">
          <Button
            type="submit"
            variant="lime"
            class="px-6 font-semibold h-9 text-xs"
            disabled={isSaving}
          >
            {isSaving ? m.settings_general_saving() : m.settings_general_save()}
          </Button>
        </div>
      </form>
    {/if}
  </div>
</div>

{#snippet SectionSkeleton()}
  <div class="flex flex-col gap-6 text-start">
    <div class="border-b border-border/60 pb-3">
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
