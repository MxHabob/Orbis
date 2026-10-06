<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";
import { setLocale } from "$lib/paraglide/runtime";
import {
  createOrganizationQuery,
  createUpdateOrganizationMutation,
} from "./queries";
import GeneralSettingsForm from "./components/general-settings-form.svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { orgId }: { orgId: string } = $props();

// Query & Mutation
const orgQuery = createOrganizationQuery(() => orgId);
const updateOrgMutation = createUpdateOrganizationMutation();

let isLoading = $derived(orgQuery.isLoading);
let isSaving = $derived(updateOrgMutation.isPending);

async function handleLanguageChange(newLocale: string, currentValues: any) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";

  if (orgId) {
    try {
      await updateOrgMutation.mutateAsync({
        orgId,
        request: {
          name: currentValues.name,
          slug: currentValues.slug,
          locale: newLocale,
          timezone: currentValues.timezone,
        },
      });
    } catch (error) {
      toast.error(extractErrorMessage(error));
      return false;
    }
  }

  setLocale(targetLang);
  return true;
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
    await updateOrgMutation.mutateAsync({
      orgId,
      request: {
        logo: typeof fileOrUrl === "string" ? fileOrUrl : null,
      },
    });

    toast.success(m.settings_general_save_success());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleRemoveOrganization() {
  if (!orgId) return;
  toast.error(m.settings_ui_organization_delete_unavailable());
}
</script>

{#if isLoading}
  <SettingsFormSkeleton />
{:else if orgQuery.data}
  {#key orgQuery.data}
    <GeneralSettingsForm
      org={orgQuery.data}
      {orgId}
      {isSaving}
      onsave={handleSave}
      onsaveavatar={handleSaveAvatar}
      onremove={handleRemoveOrganization}
      onlanguagechange={handleLanguageChange}
    />
  {/key}
{/if}
