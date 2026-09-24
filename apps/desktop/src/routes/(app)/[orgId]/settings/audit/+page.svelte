<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { AuditLogViewer } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.audit_title ? m.audit_title() : "Audit Log"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AuditLogViewer {orgId} />
</div>
