<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import ExamControl from "$lib/features/exams/exam-control.svelte";
</script>

<svelte:head>
  <title>{m.ui_page_bulk_entry_grades()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ExamControl orgId={page.params.orgId || ""} bulk />
</div>
