<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

import { setLocale } from "$lib/paraglide/runtime";
import {
  createOrganizationQuery,
  createUpdateOrganizationMutation,
  GeneralSettingsForm,
} from "$lib/features/settings";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let orgId = $derived(page.params.orgId as string);

// Query & Mutation
const orgQuery = createOrganizationQuery(() => orgId);
const updateOrgMutation = createUpdateOrganizationMutation();

let isLoading = $derived(orgQuery.isLoading);
let isSaving = $derived(updateOrgMutation.isPending);

async function handleLanguageChange(newLocale: string, currentValues: any) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";

  // Auto-save form to database before reloading to avoid losing unsaved input
  if (orgId) {
    try {
      await updateOrgMutation.mutateAsync({
        orgId,
        request: {
          name: currentValues.name,
          slug: currentValues.slug,
          locale: targetLang,
          timezone: currentValues.timezone,
        },
      });
    } catch {
      // ignore save error, proceed with language switch
    }
  }

  setLocale(targetLang);
}

async function handleSave(data: any) {
  if (!orgId) return;
  try {
    await updateOrgMutation.mutateAsync({ orgId, request: data });
    toast.success(m.settings_general_save_success());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleSaveAvatar(fileOrUrl: File | string | null) {
  if (!orgId) return;
  try {
    let avatarUrl: string | undefined;
    if (fileOrUrl instanceof File) {
      throw new Error("Avatar file upload is not available in the desktop client");
    } else if (typeof fileOrUrl === "string") {
      avatarUrl = fileOrUrl;
    } else {
      avatarUrl = undefined;
    }

    await updateOrgMutation.mutateAsync({
      orgId,
      request: {
        avatar: avatarUrl,
      },
    });

    toast.success(m.settings_general_save_success());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleRemoveOrganization() {
  if (!orgId) return;
  toast.error("Organization deletion is not available in the desktop client");
}
</script>

<svelte:head>
  <title>{m.common_header_org_settings()} | Orbis</title>
</svelte:head>

{#if isLoading}
  <SettingsFormSkeleton />
{:else if orgQuery.data}
  {#key orgQuery.data}
    <GeneralSettingsForm
      org={orgQuery.data}
      {isSaving}
      onsave={handleSave}
      onsaveavatar={handleSaveAvatar}
      onremove={handleRemoveOrganization}
      onlanguagechange={handleLanguageChange}
    />
  {/key}
{/if}
