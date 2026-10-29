<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import AcademicCalendar from "$lib/features/academics/academic-calendar.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_nav_calendar()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicCalendar {orgId} />
</div>
