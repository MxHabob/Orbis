<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  primaryColor: string;
  favicon: string;
  isSaving?: boolean;
}

let {
  primaryColor = $bindable(),
  favicon = $bindable(),
  isSaving = false,
}: Props = $props();
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_general_section_branding()}</CardTitle>
  </CardHeader>
  <CardContent>
    <SettingsRow
      title={m.settings_general_favicon()}
      description={m.settings_general_favicon_desc()}
      labelFor="org_favicon"
    >
      {#snippet children()}
        <Input
          id="org_favicon"
          type="url"
          bind:value={favicon}
          placeholder="https://example.com/favicon.ico"
          class="h-9 w-full"
          disabled={isSaving}
        />
      {/snippet}
    </SettingsRow>

    <!-- Primary Color -->
    <SettingsRow
      description={m.settings_general_color_desc()}
      controlClass="justify-start  gap-2"
      last
    >
      {#snippet children()}
        <FormField id="org_color" label={m.settings_general_color()}>
          {#snippet children()}
            <div class="flex w-full items-center gap-2">
              <input
                id="org_color"
                type="color"
                bind:value={primaryColor}
                class="size-9 shrink-0 cursor-pointer rounded-md border border-border bg-background p-1 shadow-sm"
                disabled={isSaving}
              />
              <Input
                type="text"
                bind:value={primaryColor}
                placeholder="#84cc16"
                class="h-9 w-28 shrink-0 bg-background text-center font-mono uppercase"
                disabled={isSaving}
              />
            </div>
          {/snippet}
        </FormField>
      {/snippet}
    </SettingsRow>
  </CardContent>
</Card>
