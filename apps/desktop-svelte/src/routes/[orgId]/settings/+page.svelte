<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import AvatarUpload from "$lib/components/shared/avatar-upload.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
let orgId = $derived(page.params.orgId);

// Form state
let isLoading = $state(true);
let isSaving = $state(false);

let name = $state("");
let slug = $state("");
let id = $state("");
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
    slug = (org.slug as string) || "";
    id = (org.id as string) || "";
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

async function handleSaveAvatar(e: Event) {
  e.preventDefault();
  if (!orgId) return;
  isSaving = true;
  try {
    const request = {
      logo: logo || null,
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

function handleRemoveOrganization() {
  toast.error(getLocale() === "ar" ? "هذا الإجراء غير متاح في تطبيق سطح المكتب." : "This action is disabled in the desktop application.");
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
  <title>{m.settings_general_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="w-full p-6 flex flex-col gap-6">
    {#if isLoading}
      <div class="flex flex-col gap-6">
        {@render SectionSkeleton()}
        {@render SectionSkeleton()}
      </div>
    {:else}
      <form onsubmit={handleSave} class="flex flex-col gap-6">

        <div class="border border-border rounded-md overflow-hidden flex flex-col">
          <!-- Card Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.settings_general_section_info()}
          </div>

          <!-- Card Body / Rows -->
          <div class="flex flex-col">
            <!-- Organization Slug -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="org_slug" class="text-xs font-semibold text-foreground flex items-center gap-1">
                  {m.settings_general_slug()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_slug_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="org_slug"
                  type="text"
                  value={slug}
                  disabled
                  class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80"
                />
              </div>
            </div>

            <!-- Display Name -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="org_name" class="text-xs font-semibold text-foreground flex items-center gap-1">
                  {m.settings_general_display_name()}
                  <span class="text-destructive">*</span>
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_display_name_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="org_name"
                  type="text"
                  bind:value={name}
                  placeholder={m.settings_general_name_placeholder()}
                  required
                  disabled={isSaving}
                  class="h-9 w-full bg-background"
                />
              </div>
            </div>

            <!-- Organization ID -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="org_id" class="text-xs font-semibold text-foreground">
                  {m.settings_general_id()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_id_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="org_id"
                  type="text"
                  value={id}
                  disabled
                  class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Section 2: Branding Settings (primary color) -->
        <div class="border border-border rounded-md overflow-hidden flex flex-col">
          <!-- Card Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.settings_general_section_branding()}
          </div>

          <div class="flex flex-col">
            <!-- Primary Color -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="org_color" class="text-xs font-semibold text-foreground">
                  {m.settings_general_color()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_color_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start gap-2">
                <input
                  id="org_color"
                  type="color"
                  bind:value={primaryColor}
                  class="w-9 h-9 rounded-lg border border-border cursor-pointer bg-transparent shrink-0"
                  disabled={isSaving}
                />
                <Input
                  type="text"
                  bind:value={primaryColor}
                  placeholder="#84cc16"
                  class="font-mono uppercase h-9 w-28 text-center shrink-0 bg-background"
                  disabled={isSaving}
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Section 3: Regional Settings (currency, timezone, language, date format, hijri calendar) -->
        <div class="border border-border rounded-md overflow-hidden flex flex-col">
          <!-- Card Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.settings_general_section_regional()}
          </div>

          <div class="flex flex-col">
            <!-- Currency -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="org_currency" class="text-xs font-semibold text-foreground">
                  {m.settings_general_currency()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_currency_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="org_currency"
                  type="text"
                  bind:value={currency}
                  placeholder={m.settings_general_currency_placeholder()}
                  class="uppercase font-mono w-full text-start h-9 bg-background"
                  required
                  disabled={isSaving}
                />
              </div>
            </div>

            <!-- Timezone -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label class="text-xs font-semibold text-foreground">
                  {m.settings_general_timezone()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_timezone_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Select type="single" value={timezone} onValueChange={(v) => (timezone = v ?? "")} disabled={isSaving}>
                  <SelectTrigger class="h-9 w-full bg-background">
                    {timezones.find(tz => tz.value === timezone)?.label || timezone}
                  </SelectTrigger>
                  <SelectContent>
                    {#each timezones as tz}
                      <SelectItem value={tz.value}>{tz.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <!-- Locale / Language -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label class="text-xs font-semibold text-foreground">
                  {m.settings_general_locale()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_locale_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Select type="single" value={locale} onValueChange={(v) => (locale = v ?? "")} disabled={isSaving}>
                  <SelectTrigger class="h-9 w-full bg-background">
                    {locales.find(loc => loc.value === locale)?.label || locale}
                  </SelectTrigger>
                  <SelectContent>
                    {#each locales as loc}
                      <SelectItem value={loc.value}>{loc.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <!-- Date Format -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label class="text-xs font-semibold text-foreground">
                  {m.settings_general_date_format()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_date_format_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Select type="single" value={dateFormat} onValueChange={(v) => (dateFormat = v ?? "")} disabled={isSaving}>
                  <SelectTrigger class="h-9 w-full bg-background">
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
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="hijri_enabled" class="text-xs font-semibold text-foreground">
                  {m.settings_general_hijri()}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_hijri_desc()}</span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start md:justify-start">
                <Switch
                  id="hijri_enabled"
                  bind:checked={hijriEnabled}
                  disabled={isSaving}
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex justify-end gap-3 pb-8 border-b border-border">
          <Button
            type="submit"
            class="px-6 font-semibold h-9 text-xs"
            disabled={isSaving}
          >
            {isSaving ? m.settings_general_saving() : m.settings_general_save()}
          </Button>
        </div>
      </form>

      <!-- Section 4: Avatar (preview, choices, save footer) -->
      <div class="border border-border rounded-md overflow-hidden flex flex-col">
        <!-- Card Header -->
        <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
          {m.settings_general_section_avatar()}
        </div>

        <!-- Card Body -->
        <div class="px-6 py-5 flex items-center justify-start text-start select-none">
          <AvatarUpload
            bind:avatarUrl={logo}
            size={64}
            fallbackText={name}
          />
        </div>

        <!-- Card Footer -->
        <div class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start">
          <Button type="button" class="font-semibold text-xs h-8 px-4" onclick={handleSaveAvatar} disabled={isSaving}>
            {isSaving ? m.settings_general_saving() : m.settings_general_avatar_save()}
          </Button>
        </div>
      </div>

      <!-- Section 5: Remove Organization -->
      <div class="border border-border rounded-md overflow-hidden flex flex-col">
        <!-- Card Header -->
        <div class="bg-muted/50 px-6 py-3 border border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
          {m.settings_general_section_remove()}
        </div>

        <!-- Card Body -->
        <div class="px-6 py-5 flex flex-col md:flex-row md:items-center justify-between gap-4 text-start">
          <div class="flex flex-col gap-1">
            <span class="text-xs font-semibold text-foreground">{m.settings_general_remove_title()}</span>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.settings_general_remove_desc()}
            </span>
          </div>
          <div class="w-full md:w-auto flex items-center justify-end shrink-0">
            <Button type="button" variant="destructive" onclick={handleRemoveOrganization} class="font-semibold text-xs h-9 px-6 select-none">
              {m.settings_general_remove_btn()}
            </Button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

{#snippet SectionSkeleton()}
  <div class="bg-card border border-border rounded-xl overflow-hidden flex flex-col">
    <div class="bg-muted/20 px-6 py-3 border-b border-border">
      <Skeleton class="h-5 w-32" />
    </div>
    <div class="flex flex-col">
      <div class="px-6 py-4 flex items-center justify-between border-b border-border last:border-b-0 gap-4">
        <div class="flex flex-col gap-2">
          <Skeleton class="h-4 w-28" />
          <Skeleton class="h-3 w-48" />
        </div>
        <Skeleton class="h-9 w-64" />
      </div>
      <div class="px-6 py-4 flex items-center justify-between border-b border-border last:border-b-0 gap-4">
        <div class="flex flex-col gap-2">
          <Skeleton class="h-4 w-28" />
          <Skeleton class="h-3 w-48" />
        </div>
        <Skeleton class="h-9 w-64" />
      </div>
    </div>
  </div>
{/snippet}
