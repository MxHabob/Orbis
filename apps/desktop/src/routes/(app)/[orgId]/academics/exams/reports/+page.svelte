<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { ExamReports } from "$lib/features/exams";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_exams_center_reports_title()} | Orbis</title>
</svelte:head>

<ExamReports {orgId} />
