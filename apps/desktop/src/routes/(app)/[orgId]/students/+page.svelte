<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { StudentDirectory } from "$lib/features/students";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.common_page_title_students()}</title>
</svelte:head>

<div class="h-full w-full min-h-0 flex-1">
  <StudentDirectory orgId={data.orgId} />
</div>
