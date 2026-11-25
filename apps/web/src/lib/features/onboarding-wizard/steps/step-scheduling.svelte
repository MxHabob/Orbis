<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Card, CardContent } from "@orbis/ui/card";
import OnboardingStepHeader from "../onboarding-step-header.svelte";

let { orgId }: { orgId: string } = $props();
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    title={m.onboarding_step_scheduling_title()}
    description={m.onboarding_step_scheduling_subtitle()}
  />

  <Card>
    <CardContent class="space-y-4 p-5">
      <p class="text-sm text-muted-foreground">
        {m.onboarding_step_scheduling_optional_notice()}
      </p>
      <Button
        variant="outline"
        onclick={() => goto(localizeHref(`/${orgId}/academics/schedule/generator`))}
      >
        {m.onboarding_step_scheduling_open_workspace()}
      </Button>
    </CardContent>
  </Card>
</div>
