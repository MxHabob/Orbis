<!--
  Minimalist, elegant Progress Bar for the onboarding wizard.
  Compact and thin, presenting a sleek modern style.
-->
<script lang="ts">
import { Progress } from "@orbis/ui/progress";

let {
  progressValue,
}: {
  progressValue: number;
} = $props();
</script>

<div class="w-full max-w-md mx-auto mb-8 px-4">
  <Progress value={progressValue} class="h-1" />
</div>
