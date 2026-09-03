<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";

interface Props {
  slug: string;
  name: string;
  nameAr: string;
  organizationId: string;
  isSaving?: boolean;
}

let { slug, name = $bindable(), nameAr = $bindable(), organizationId, isSaving = false }: Props = $props();
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
      <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-end">
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
        <Label for="org_name_ar" class="text-xs font-semibold text-foreground">الاسم العربي</Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم المستخدم في المستندات والقوالب العربية</span>
      </div>
      <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-end">
        <Input id="org_name_ar" type="text" dir="rtl" bind:value={nameAr} disabled={isSaving} class="h-9 w-full bg-background text-end" />
      </div>
    </div>

    <!-- Display Name -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="org_name" class="text-xs font-semibold text-foreground">
          {m.settings_general_display_name()}<span class="text-destructive" aria-hidden="true"> *</span>
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_name_desc()}</span>
      </div>
      <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-end">
        <Input id="org_name" type="text" dir="ltr" bind:value={name} placeholder={m.settings_general_name_placeholder()} required disabled={isSaving} class="h-9 w-full bg-background text-start" />
      </div>
    </div>

    <!-- Organization ID -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="organization_id" class="text-xs font-semibold text-foreground">
          {m.settings_general_id()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_id_desc()}</span>
      </div>
      <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-end">
        <Input
          id="organization_id"
          type="text"
          value={organizationId}
          disabled
          class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
        />
      </div>
    </div>
  </div>
</div>
