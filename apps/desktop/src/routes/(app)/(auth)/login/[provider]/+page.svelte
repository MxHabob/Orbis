<!--
  Provider Login Page — (auth)/login/[provider]/+page.svelte
  Direct re-export/rendering of the OAuth provider handling page.
-->
<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";

onMount(() => {
  const provider = page.params.provider || "google";
  const search = page.url.search || "";
  const hash = typeof window !== "undefined" ? window.location.hash : "";
  goto(localizeHref(`/auth/${provider}${search}${hash}`), {
    replaceState: true,
  });
});
</script>
