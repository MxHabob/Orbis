<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";

interface Props {
  slug: string;
  name: string;
  nameAr: string;
  id: string;
  isSaving?: boolean;
}

let { slug, name = $bindable(), nameAr = $bindable(), id, isSaving = false }: Props = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col">
  <!-- Card Header -->
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.settings_general_section_info()}
  </div>

  <!-- Card Body / Rows -->
  <div class="flex flex-col">
    <!-- Organization Slug -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="org_slug" class="text-xs font-semibold text-foreground flex items-center gap-1">
          {m.settings_general_slug()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_slug_desc()}</span>
      </div>
      <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
        <Input
          id="org_slug"
          type="text"
          value={slug}
          disabled
          class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80"
        />
      </div>
    </div>

    <!-- Arabic Display Name -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم المستخدم في المستندات والقوالب العربية</span>
      </div>
      <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
        <FormField id="org_name_ar" label="الاسم العربي">
          {#snippet children()}
            <Input id="org_name_ar" type="text" dir="rtl" bind:value={nameAr} disabled={isSaving} class="h-9 w-full bg-background" />
          {/snippet}
        </FormField>
      </div>
    </div>

    <!-- Display Name -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_name_desc()}</span>
      </div>
      <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
        <FormField id="org_name" label={m.settings_general_display_name()} required>
          {#snippet children()}
            <Input id="org_name" type="text" bind:value={name} placeholder={m.settings_general_name_placeholder()} required disabled={isSaving} class="h-9 w-full bg-background" />
          {/snippet}
        </FormField>
      </div>
    </div>

    <!-- Organization ID -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="org_id" class="text-xs font-semibold text-foreground">
          {m.settings_general_id()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_id_desc()}</span>
      </div>
      <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
        <Input
          id="org_id"
          type="text"
          value={id}
          disabled
          class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
        />
      </div>
    </div>
  </div>
</div>
