<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { StudentDocuments } from "$lib/features/students";

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || "");
</script>

<svelte:head>
  <title>{m.students_detail_tab_documents()}</title>
</svelte:head>

<div class="h-full w-full min-h-0 flex-1">
  <StudentDocuments {orgId} {studentId} />
</div>
