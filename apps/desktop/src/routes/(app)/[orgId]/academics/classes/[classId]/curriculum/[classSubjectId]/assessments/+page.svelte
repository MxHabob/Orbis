<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { SectionSubjectResults } from "$lib/features/academics";

  const orgId = $derived(page.params.orgId || "");
  const classId = $derived(page.params.classId || "");
  const SectionSubjectId = $derived(page.params.SectionSubjectId || "");
</script>

<svelte:head>
  <title>{m.academics_overview_card_classes_desc()} | Orbis</title>
</svelte:head>

<SectionSubjectResults {orgId} {classId} {SectionSubjectId} />
