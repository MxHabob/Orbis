<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import ResultsReview from "$lib/features/academics/results-review.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_nav_results_review()} | Orbis</title>
</svelte:head>

<ResultsReview {orgId} />

