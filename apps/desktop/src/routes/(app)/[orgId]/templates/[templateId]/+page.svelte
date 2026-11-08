<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import TemplateStudio from "$lib/features/templates/template-studio.svelte";

let { data }: { data: { orgId: string; templateId: string } } = $props();
const orgId = $derived(data?.orgId || page.params.orgId || "");
const templateId = $derived(data?.templateId || page.params.templateId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_templates()} - Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <TemplateStudio {orgId} {templateId} />
</div>
