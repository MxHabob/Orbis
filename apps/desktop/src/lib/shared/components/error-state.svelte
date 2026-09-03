<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AlertCircle, RefreshCcw } from "@lucide/svelte";
import { Button } from "@orbis/ui/button";

let {
  title = m.common_error_occurred(),
  message = m.common_error_state_message(),
  onRetry,
}: {
  title?: string;
  message?: string;
  onRetry?: () => void;
} = $props();
</script>

<div class="flex flex-col items-center justify-center p-8 text-center h-full min-h-50">
  <div class="w-12 h-12 rounded-full bg-destructive/10 flex items-center justify-center mb-4">
    <AlertCircle class="w-6 h-6 text-destructive" />
  </div>
  <h3 class="text-lg font-semibold text-foreground mb-2">{title}</h3>
  <p class="text-sm text-muted-foreground max-w-100 mb-6">
    {message}
  </p>
  {#if onRetry}
    <Button variant="outline" onclick={onRetry}>
      <RefreshCcw class="w-4 h-4 mr-2" />
      {m.common_retry()}
    </Button>
  {/if}
</div>
