<script lang="ts">
  import { page } from "$app/state";
  import StudentForm from "$lib/features/students/student-form.svelte";

  const orgId = $derived(page.params.orgId || "");
  const studentId = $derived(page.params.studentId || "");
</script>

<div class="h-full w-full overflow-hidden">
  <StudentForm {orgId} {studentId} />
</div>
