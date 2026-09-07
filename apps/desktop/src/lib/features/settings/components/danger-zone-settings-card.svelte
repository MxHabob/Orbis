<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  onremove: () => void;
}

let { onremove }: Props = $props();
</script>

<SettingsCard title={m.settings_general_remove_btn()}>
  <SettingsRow
    title={m.settings_general_remove_btn()}
    description={m.settings_general_remove_desc()}
    controlClass="justify-end shrink-0"
    last
  >
    {#snippet children()}
      <Button type="button" variant="destructive" onclick={onremove} class="font-semibold text-xs h-9 px-6 select-none">
        {m.settings_general_remove_btn()}
      </Button>
    {/snippet}
  </SettingsRow>
</SettingsCard>
