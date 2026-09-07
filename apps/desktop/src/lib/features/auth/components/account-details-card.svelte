<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  fullName: string;
  userId: string;
  isSavingDetails?: boolean;
}

let {
  fullName = $bindable(),
  userId,
  isSavingDetails = false,
}: Props = $props();
</script>

<SettingsCard title={m.common_account_details_section()}>
  <!-- Full Name -->
  <SettingsRow
    title={m.common_col_name()}
    description={m.common_account_details_name_desc()}
    labelFor="full_name"
    required
  >
    {#snippet children()}
      <Input
        id="full_name"
        type="text"
        bind:value={fullName}
        required
        disabled={isSavingDetails}
        class="h-9 w-full bg-background"
      />
    {/snippet}
  </SettingsRow>

  <!-- User ID -->
  <SettingsRow
    title={m.common_account_details_userid()}
    description={m.common_account_details_userid_desc()}
    labelFor="user_id"
    last
  >
    {#snippet children()}
      <Input
        id="user_id"
        type="text"
        value={userId}
        disabled
        class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
      />
    {/snippet}
  </SettingsRow>
</SettingsCard>
