<script lang="ts">
import { page } from "$app/state";
import type { Snippet } from "svelte";
import OnboardingFrame from "$lib/features/onboarding-wizard/onboarding-frame.svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
const orgId = $derived(data?.orgId ?? page.params.orgId);
</script>

<OnboardingFrame {orgId}>
  {@render children()}
</OnboardingFrame>
