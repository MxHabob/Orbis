<!--
  @component OnboardingRedirect
  Route /onboarding — Redirects to /bootstrap for backward compatibility.
  The bootstrap wizard now lives at /bootstrap.
  Post-bootstrap onboarding (Setup Center) is at /[orgId]/onboarding.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { onMount } from "svelte";
import { localizeHref } from "$lib/paraglide/runtime";

onMount(() => {
  goto(localizeHref("/bootstrap"), { replaceState: true });
});
</script>

<div class="fixed inset-0 flex items-center justify-center bg-background text-muted-foreground">
  <span class="text-sm">Redirecting...</span>
</div>
