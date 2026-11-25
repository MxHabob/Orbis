<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import AcademicCalendarsManager from "$lib/features/settings/academic-calendars-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academic_calendars_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicCalendarsManager {orgId} />
</div>
