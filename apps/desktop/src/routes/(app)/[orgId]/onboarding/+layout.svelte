<script lang="ts">
import { page } from "$app/state";
import type { Snippet } from "svelte";
import OnboardingShell from "$lib/features/onboarding-wizard/onboarding-shell.svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
const orgId = $derived(data?.orgId ?? page.params.orgId);
</script>

<OnboardingShell {orgId}>
  {@render children()}
</OnboardingShell>
