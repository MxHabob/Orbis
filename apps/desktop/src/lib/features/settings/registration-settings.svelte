<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import { createOrganizationQuery } from "./queries";
import RegistrationSettingsForm from "./components/registration-settings-form.svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { orgId }: { orgId: string } = $props();

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
      settingsObj =
        typeof org.settings === "string"
          ? JSON.parse(org.settings)
          : org.settings;
    } catch {
      settingsObj = {};
    }
  }

  settingsObj.registration = registrationConfig.registration;

  isSaving = true;
  try {
    await api.updateSettings(settingsObj);
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
