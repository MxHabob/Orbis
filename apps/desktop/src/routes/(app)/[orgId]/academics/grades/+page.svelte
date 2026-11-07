<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import ExamControl from "$lib/features/exams/exam-control.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_nav_grade_entry()} | Orbis</title>
</svelte:head>

<ExamControl {orgId} />

