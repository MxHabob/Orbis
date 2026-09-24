<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { StudentGuardians } from "$lib/features/students";

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || "");
</script>

<svelte:head>
  <title>{m.common_guardians_tab()}</title>
</svelte:head>

<div class="h-full w-full min-h-0 flex-1">
  <StudentGuardians {orgId} {studentId} />
</div>
