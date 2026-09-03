<!--
@component UserDetailRedirect
Redirects to the profile sub-page.
-->
<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { onMount } from "svelte";

const orgId = $derived(page.params.orgId);
const userId = $derived(page.params.userId);

onMount(() => {
  goto(localizeHref(`/${orgId}/settings/users/${userId}/profile`), {
    replaceState: true,
  });
});
</script>
