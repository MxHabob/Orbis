<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

interface Props {
  slug: string;
  name: string;
  nameAr: string;
  organizationId: string;
  isSaving?: boolean;
}

let {
  slug,
  name = $bindable(),
  nameAr = $bindable(),
  organizationId,
  isSaving = false,
}: Props = $props();
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_general_section_info()}</CardTitle>
  </CardHeader>
  <CardContent>
    <!-- Organization Slug -->
    <SettingsRow
      title={m.settings_general_slug()}
      description={m.settings_general_slug_desc()}
      labelFor="org_slug"
    >
      {#snippet children()}
        <Input
          id="org_slug"
          type="text"
          value={slug}
          disabled
          class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80"
        />
      {/snippet}
    </SettingsRow>

    <SettingsRow
      title={m.settings_general_display_name()}
      description={m.settings_general_name_desc()}
      labelFor="org_name"
      required
    >
      {#snippet children()}
        <div class="w-full min-w-0">
          <BilingualNameFields
            arId="org_name_ar"
            enId="org_name"
            arLabel={m.settings_ui_organization_arabic_name()}
            enLabel={m.settings_general_display_name()}
            bind:arValue={nameAr}
            bind:enValue={name}
            enPlaceholder={m.settings_general_name_placeholder()}
            kind="label"
            requestSuggestion={suggestLocalTranslation}
            enRequired
            disabled={isSaving}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />
        </div>
      {/snippet}
    </SettingsRow>

    <!-- Organization ID -->
    <SettingsRow
      title={m.settings_general_id()}
      description={m.settings_general_id_desc()}
      labelFor="organization_id"
      last
    >
      {#snippet children()}
        <Input
          id="organization_id"
          type="text"
          value={organizationId}
          disabled
          class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
        />
      {/snippet}
    </SettingsRow>
  </CardContent>
</Card>
