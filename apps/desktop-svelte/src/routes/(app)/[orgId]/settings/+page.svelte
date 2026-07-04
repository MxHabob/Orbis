<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";
import {
  InfoSettingsCard,
  BrandingSettingsCard,
  RegionalSettingsCard,
  AvatarSettingsCard,
  DangerZoneSettingsCard,
} from "$lib/features/settings";

let orgId = $derived(page.params.orgId);

// Form state
let isLoading = $state(true);
let isSaving = $state(false);

let name = $state("");
let slug = $state("");
let id = $state("");
let logo = $state("");
let primaryColor = $state("#7c3aed");
let currency = $state("YER");
let timezone = $state("Asia/Aden");
let locale = $state("ar");
let dateFormat = $state("YYYY-MM-DD");
let hijriEnabled = $state(false);

async function handleLanguageChange(newLocale: string) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";

  locale = newLocale;

  // Auto-save form to database before reloading to avoid losing unsaved input
  if (orgId) {
    try {
      const request = {
        name,
        logo: logo || null,
        primary_color: primaryColor,
        currency,
        timezone,
        locale: newLocale,
        date_format: dateFormat,
        hijri_enabled: hijriEnabled ? 1 : 0,
      };
      await commands.users.updateOrganization(orgId, request);
    } catch (e) {
      console.error("Failed to auto-save before language switch:", e);
    }
  }

  if (targetLang === getLocale()) return;

  setLocale(targetLang);
}

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
  toast.error(
    getLocale() === "ar"
      ? "هذا الإجراء غير متاح في تطبيق سطح المكتب."
      : "This action is disabled in the desktop application.",
  );
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
        <InfoSettingsCard
          {slug}
          bind:name
          {id}
          {isSaving}
        />

        <BrandingSettingsCard
          bind:primaryColor
          {isSaving}
        />

        <RegionalSettingsCard
          bind:currency
          bind:timezone
          bind:locale
          bind:dateFormat
          bind:hijriEnabled
          {isSaving}
          onLanguageChange={handleLanguageChange}
        />

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

      <AvatarSettingsCard
        bind:logo
        {name}
        {isSaving}
        onsave={handleSaveAvatar}
      />

      <DangerZoneSettingsCard
        onremove={handleRemoveOrganization}
      />
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
