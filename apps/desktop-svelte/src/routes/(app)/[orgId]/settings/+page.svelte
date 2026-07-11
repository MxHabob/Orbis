<script lang="ts">
import { page } from "$app/state";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";
import {
  createOrganizationQuery,
  createUpdateOrganizationMutation,
  GeneralSettingsForm,
} from "$lib/features/settings";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let orgId = $derived(page.params.orgId as string);

// Query & Mutation
const orgQuery = createOrganizationQuery(orgId);
const updateOrgMutation = createUpdateOrganizationMutation();

let isLoading = $derived($orgQuery.isLoading);
let isSaving = $derived($updateOrgMutation.isPending);

async function handleLanguageChange(newLocale: string, currentValues: any) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";

  // Auto-save form to database before reloading to avoid losing unsaved input
  if (orgId) {
    try {
      await commands.organization.updateOrganization(orgId, currentValues);
    } catch (e) {
      console.error("Failed to auto-save before language switch:", e);
    }
  }

  if (targetLang === getLocale()) return;

  setLocale(targetLang);
}

async function handleSaveAvatar(logo: string | null) {
  if (!orgId) return;

  $updateOrgMutation.mutate(
    {
      orgId,
      request: {
        logo,
      },
    },
    {
      onSuccess: () => {
        toast.success(m.settings_general_save_success());
      },
      onError: (e: unknown) => {
        toast.error(
          m.settings_general_save_error({ error: extractErrorMessage(e) }),
        );
      },
    },
  );
}

function handleRemoveOrganization() {
  toast.error(m.settings_general_desktop_disabled());
}

async function handleSave(request: any) {
  if (!orgId) return;

  $updateOrgMutation.mutate(
    {
      orgId,
      request,
    },
    {
      onSuccess: () => {
        toast.success(m.settings_general_save_success());
      },
      onError: (e: unknown) => {
        toast.error(
          m.settings_general_save_error({ error: extractErrorMessage(e) }),
        );
      },
    },
  );
}
</script>

<svelte:head>
  <title>{m.settings_general_window_title()} | Orbis</title>
</svelte:head>

{#if isLoading}
  <SettingsFormSkeleton />
{:else if $orgQuery.data}
  {#key $orgQuery.data}
    <GeneralSettingsForm
      org={$orgQuery.data}
      {isSaving}
      onsave={handleSave}
      onsaveavatar={handleSaveAvatar}
      onremove={handleRemoveOrganization}
      onlanguagechange={handleLanguageChange}
    />
  {/key}
{/if}
