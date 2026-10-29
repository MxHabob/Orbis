<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ClassDetails } from "$lib/features/academics";

const orgId = $derived(page.params.orgId || "");
const sectionId = $derived(page.params.sectionId || "");
</script>

<svelte:head>
  <title>{m.common_card_active_classes()} | Orbis</title>
</svelte:head>

<ClassDetails {orgId} classId={sectionId} />
