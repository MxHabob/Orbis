<script lang="ts">
import { page } from "$app/state";
import UserRoles from "$lib/features/settings/user-roles.svelte";

const orgId = $derived(page.params.orgId || "");
const userId = $derived(page.params.userId || "");
</script>

<UserRoles {orgId} {userId} />
