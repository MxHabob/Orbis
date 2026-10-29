<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ExamControl } from "$lib/features/exams";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_exams_center_control_title()} | Orbis</title>
</svelte:head>

<ExamControl {orgId} />
