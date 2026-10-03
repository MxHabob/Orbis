<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  primaryColor: string;
  isSaving?: boolean;
}

let { primaryColor = $bindable(), isSaving = false }: Props = $props();
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_general_section_branding()}</CardTitle>
  </CardHeader>
  <CardContent>
    <!-- Primary Color -->
    <SettingsRow
      description={m.settings_general_color_desc()}
      controlClass="justify-start gap-2"
      last
    >
      {#snippet children()}
        <FormField id="org_color" label={m.settings_general_color()}>
          {#snippet children()}
            <input
              id="org_color"
              type="color"
              bind:value={primaryColor}
              class="w-9 h-9 cursor-pointer"
              disabled={isSaving}
            />
            <Input
              type="text"
              bind:value={primaryColor}
              placeholder="#84cc16"
              class="font-mono uppercase h-9 w-28 text-center shrink-0 bg-background"
              disabled={isSaving}
            />
          {/snippet}
        </FormField>
      {/snippet}
    </SettingsRow>
  </CardContent>
</Card>
