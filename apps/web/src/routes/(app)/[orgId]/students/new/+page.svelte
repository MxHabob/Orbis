<script lang="ts">
import { page } from "$app/state";
import StudentForm from "$lib/features/students/student-form.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

{#key orgId}
  <StudentForm {orgId} />
{/key}
