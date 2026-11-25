<script lang="ts">
import { page } from "$app/state";
import UserPreferences from "$lib/features/settings/user-preferences.svelte";

const orgId = $derived(page.params.orgId || "");
const userId = $derived(page.params.userId || "");
</script>

<UserPreferences {orgId} {userId} />
