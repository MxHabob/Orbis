<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import LogoUpload from "$lib/shared/components/logo-upload.svelte";
import SettingsCard from "$lib/shared/components/settings-card.svelte";

interface Props {
  logo: string;
  name: string;
  orgId: string;
  isSaving?: boolean;
  onsave: (e: Event) => void;
}

let { logo = $bindable(), name, orgId, isSaving = false, onsave }: Props = $props();
</script>

<SettingsCard title={m.common_account_avatar_section()}>
  <div class="px-6 py-5 flex items-center justify-start text-start select-none">
    <LogoUpload
      bind:logoUrl={logo}
      size={64}
      entityType="organization"
      entityId={orgId}
    />
  </div>

  {#snippet footer()}
    <Button type="button" class="font-semibold text-xs h-8 px-4" onclick={onsave} disabled={isSaving}>
      {isSaving ? m.common_state_saving() : m.settings_general_avatar_save()}
    </Button>
  {/snippet}
</SettingsCard>
