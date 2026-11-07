<!--
  @component ExamControlRedirect
  Grade entry has moved to /academics/grades.
  This page redirects for backward-compatibility.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";

const orgId = $derived(page.params.orgId || "");

$effect(() => {
  if (orgId) {
    void goto(`/${orgId}/academics/grades`, { replaceState: true });
  }
});
</script>
