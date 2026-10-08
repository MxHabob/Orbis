<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  import AcademicYearsManager from "$lib/features/settings/academic-years-manager.svelte";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academic_years_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicYearsManager {orgId} />
</div>
