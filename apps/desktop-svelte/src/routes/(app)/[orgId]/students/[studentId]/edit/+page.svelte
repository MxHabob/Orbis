<script lang="ts">
import { page } from "$app/state";
import StudentForm from "$lib/components/students/student-form.svelte";

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || "");
</script>

<div class="h-full w-full overflow-hidden">
	<StudentForm studentId={studentId} orgId={orgId} />
</div>
