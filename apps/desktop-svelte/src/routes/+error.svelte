<script lang="ts">
import * as m from "$lib/paraglide/messages";
  import { page } from "$app/stores";
  import ErrorState from "$lib/shared/components/error-state.svelte";
  
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center bg-background p-4">
  <div class="max-w-md w-full border border-border/50 rounded-lg shadow-sm bg-card">
    <ErrorState
      title={$page.status + " " + m.common_error_occurred()}
      message={$page.error?.message || m.common_unexpected_error_message()}
      onRetry={() => window.location.reload()}
    />
  </div>
</div>
