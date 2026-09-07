<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
  isUpdating?: boolean;
  onsubmit: (e: Event) => void;
}

let {
  currentPassword = $bindable(),
  newPassword = $bindable(),
  confirmPassword = $bindable(),
  isUpdating = false,
  onsubmit,
}: Props = $props();
</script>

<SettingsCard title={m.common_account_password_section()} {onsubmit}>
  <!-- Current Password -->
  <SettingsRow
    title={m.common_account_password_current_label()}
    description={m.common_account_password_current_desc()}
    labelFor="current_pass"
  >
    {#snippet children()}
      <Input
        id="current_pass"
        type="password"
        bind:value={currentPassword}
        placeholder="••••••••"
        required
        disabled={isUpdating}
        class="h-9 w-full bg-background"
      />
    {/snippet}
  </SettingsRow>

  <!-- New Password -->
  <SettingsRow
    title={m.common_account_password_new_label()}
    description={m.common_account_password_new_desc()}
    labelFor="new_pass"
  >
    {#snippet children()}
      <Input
        id="new_pass"
        type="password"
        bind:value={newPassword}
        placeholder="••••••••"
        required
        minlength={6}
        disabled={isUpdating}
        class="h-9 w-full bg-background"
      />
    {/snippet}
  </SettingsRow>

  <!-- Confirm Password -->
  <SettingsRow
    title={m.common_account_password_confirm_label()}
    description={m.common_account_password_confirm_desc()}
    labelFor="confirm_pass"
    last
  >
    {#snippet children()}
      <Input
        id="confirm_pass"
        type="password"
        bind:value={confirmPassword}
        placeholder="••••••••"
        required
        disabled={isUpdating}
        class="h-9 w-full bg-background"
      />
    {/snippet}
  </SettingsRow>

  {#snippet footer()}
    <Button
      type="submit"
      class="font-semibold text-xs h-8 px-4"
      disabled={isUpdating}
    >
      {isUpdating ? m.common_account_password_updating() : m.common_account_password_update_btn()}
    </Button>
  {/snippet}
</SettingsCard>
