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

async function handleSave(registrationConfig: any): Promise<boolean> {
  if (!orgId) return false;

  isSaving = true;
  try {
    const currentSettings = await api.getSettings();
    const updatedSettings = {
      ...currentSettings,
      admission: registrationConfig.admission,
      enrollment: registrationConfig.enrollment ?? currentSettings.enrollment,
      student_sort:
        registrationConfig.student_sort ?? currentSettings.student_sort,
    };
    await api.updateSettings(updatedSettings);
    toast.success(m.settings_registration_save_success());
    return true;
  } catch (e: unknown) {
    toast.error(
      m.settings_registration_save_error({ error: extractErrorMessage(e) }),
    );
    return false;
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <main class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide">
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
  </main>
</div>
