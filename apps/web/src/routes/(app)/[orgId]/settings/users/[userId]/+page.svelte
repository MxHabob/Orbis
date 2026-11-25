<script lang="ts">
import { page } from "$app/state";
import UserProfile from "$lib/features/settings/user-profile.svelte";

const orgId = $derived(page.params.orgId || "");
const userId = $derived(page.params.userId || "");
</script>

<UserProfile {orgId} {userId} />
