<script lang="ts">
import { page } from "$app/state";
import { toast } from "svelte-sonner";
import * as settings from "$lib/paraglide/messages/settings";
import { extractErrorMessage } from "$lib/core/ipc";
import {
  createOrganizationQuery,
  createUpdateOrganizationMutation,
  RegistrationSettingsForm,
} from "$lib/features/settings";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let orgId = $derived(page.params.orgId as string);

// Query & Mutation
const orgQuery = createOrganizationQuery(() => orgId);
const updateOrgMutation = createUpdateOrganizationMutation();

let isLoading = $derived($orgQuery.isLoading);
let isSaving = $derived($updateOrgMutation.isPending);

async function handleSave(registrationConfig: any) {
  const org = $orgQuery.data;
  if (!orgId || !org) return;

  let settingsObj: Record<string, any> = {};
  if (org.settings) {
    try {
      settingsObj = JSON.parse(org.settings as string);
    } catch {
      settingsObj = {};
    }
  }

  settingsObj.registration_config = registrationConfig;

  const request = {
    name: org.name,
    logo: org.logo || null,
    primary_color: org.primary_color,
    currency: org.currency,
    timezone: org.timezone,
    locale: org.locale,
    date_format: org.date_format,
    hijri_enabled: org.hijri_enabled,
    settings: JSON.stringify(settingsObj),
  };

  $updateOrgMutation.mutate(
    { orgId, request },
    {
      onSuccess: () => {
        toast.success(settings.registration_save_success());
      },
      onError: (e: unknown) => {
        toast.error(
          settings.registration_save_error({ error: extractErrorMessage(e) }),
        );
      },
    },
  );
}
</script>

<svelte:head>
  <title>{settings.registration_window_title()} | Orbis</title>
</svelte:head>

{#if isLoading}
  <SettingsFormSkeleton />
{:else if $orgQuery.data}
  {#key $orgQuery.data}
    <RegistrationSettingsForm
      org={$orgQuery.data}
      {isSaving}
      onsave={handleSave}
    />
  {/key}
{/if}

