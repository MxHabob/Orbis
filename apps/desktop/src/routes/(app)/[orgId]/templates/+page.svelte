<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  import TemplatesGallery from "$lib/features/templates/templates-gallery.svelte";

  let { data }: { data: { orgId: string } } = $props();
  const orgId = $derived(data?.orgId || page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_templates()} - Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <TemplatesGallery {orgId} />
</div>
