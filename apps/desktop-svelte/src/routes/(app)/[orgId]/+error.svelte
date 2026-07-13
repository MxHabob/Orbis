<script lang="ts">
  import { page } from "$app/stores";
  import ErrorState from "$lib/shared/components/error-state.svelte";
  import * as common from "$lib/paraglide/messages/common";
</script>

<div class="h-full w-full flex flex-col items-center justify-center p-4">
  <div class="max-w-md w-full border border-border/50 rounded-lg shadow-sm bg-card">
    <ErrorState
      title={$page.status + " " + common.error_occurred()}
      message={$page.error?.message || common.unexpected_error_message()}
      onRetry={() => window.location.reload()}
    />
  </div>
</div>
