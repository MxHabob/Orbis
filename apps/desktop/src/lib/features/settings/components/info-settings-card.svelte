<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

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

    <!-- Arabic Display Name -->
    <SettingsRow
      title={m.settings_ui_organization_arabic_name()}
      description={m.settings_ui_organization_arabic_name_description()}
      labelFor="org_name_ar"
    >
      {#snippet children()}
        <Input id="org_name_ar" type="text" dir="rtl" bind:value={nameAr} disabled={isSaving} class="h-9 w-full bg-background text-end" />
      {/snippet}
    </SettingsRow>

    <!-- Display Name -->
    <SettingsRow
      title={m.settings_general_display_name()}
      description={m.settings_general_name_desc()}
      labelFor="org_name"
      required
    >
      {#snippet children()}
        <Input id="org_name" type="text" dir="ltr" bind:value={name} placeholder={m.settings_general_name_placeholder()} required disabled={isSaving} class="h-9 w-full bg-background text-start" />
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
