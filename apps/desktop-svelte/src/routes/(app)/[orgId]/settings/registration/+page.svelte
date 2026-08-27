<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { toast } from "svelte-sonner";

import { commands, extractErrorMessage } from "$lib/core/ipc";
import {
  createOrganizationQuery,
  RegistrationSettingsForm,
} from "$lib/features/settings";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let orgId = $derived(page.params.orgId as string);

// Query & Mutation
const orgQuery = createOrganizationQuery(() => orgId);
let isSaving = $state(false);

let isLoading = $derived(orgQuery.isLoading);

async function handleSave(registrationConfig: any) {
  const org = orgQuery.data;
  if (!orgId || !org) return;

  let settingsObj: Record<string, any> = {};
  if (org.settings) {
    try {
      settingsObj = typeof org.settings === "string"
        ? JSON.parse(org.settings)
        : org.settings;
    } catch {
      settingsObj = {};
    }
  }

  settingsObj.registration = registrationConfig.registration;

  isSaving = true;
  try {
    await commands.organization.updateSettings(settingsObj);
    toast.success(m.settings_registration_save_success());
  } catch (e: unknown) {
    toast.error(
      m.settings_registration_save_error({ error: extractErrorMessage(e) }),
    );
  } finally {
    isSaving = false;
  }
}
</script>

<svelte:head>
  <title>{m.settings_nav_registration()} | Orbis</title>
</svelte:head>

{#if isLoading}
  <SettingsFormSkeleton />
{:else if orgQuery.data}
  {#key orgQuery.data}
    <RegistrationSettingsForm
      org={orgQuery.data}
      {isSaving}
      onsave={handleSave}
    />
  {/key}
{/if}

