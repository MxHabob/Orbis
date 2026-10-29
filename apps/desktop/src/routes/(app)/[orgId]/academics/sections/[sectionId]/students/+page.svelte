<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ClassStudents } from "$lib/features/academics";

const orgId = $derived(page.params.orgId || "");
const sectionId = $derived(page.params.sectionId || "");
</script>

<svelte:head>
  <title>{m.academics_class_detail_tab_students()} | Orbis</title>
</svelte:head>

<ClassStudents {orgId} classId={sectionId} />
