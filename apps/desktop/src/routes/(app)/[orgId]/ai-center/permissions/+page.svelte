<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { AiPermissions } from "$lib/features/ai";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.ai_center_permissions_title ? m.ai_center_permissions_title() : "مصفوفة الصلاحيات والحوكمة"} | Orbis</title>
</svelte:head>

<AiPermissions {orgId} />
