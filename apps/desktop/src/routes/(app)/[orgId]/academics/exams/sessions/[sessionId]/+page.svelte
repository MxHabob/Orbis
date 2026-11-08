<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ExamSessionDetails } from "$lib/features/exams";

const orgId = $derived(page.params.orgId || "");
const sessionId = $derived(page.params.sessionId || "");
</script>

<svelte:head>
  <title>{m.common_exams_sessions_title()} | Orbis</title>
</svelte:head>

<ExamSessionDetails {orgId} {sessionId} />
