<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import { createOrganizationQuery } from "./queries";
import AuthenticationSettingsCard from "./components/authentication-settings-card.svelte";

let { orgId }: { orgId: string } = $props();

const orgQuery = createOrganizationQuery(() => orgId);
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <main class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide">
    <div class="mx-auto w-full max-w-7xl">
      {#if orgQuery.isLoading}
        <SettingsFormSkeleton />
      {:else if orgQuery.error}
        <div class="rounded-xl border border-destructive/30 bg-card p-6 text-center">
          <p class="text-sm font-semibold text-destructive">
            {m.settings_ui_auth_load_error()}
          </p>
          <p class="mt-1 text-xs text-muted-foreground">
            {orgQuery.error.message}
          </p>
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="mt-4 h-8 text-xs"
            onclick={() => orgQuery.refetch()}
          >
            {m.common_retry()}
          </Button>
        </div>
      {:else if orgQuery.data}
        {#key orgQuery.data}
          <AuthenticationSettingsCard org={orgQuery.data} />
        {/key}
      {/if}
    </div>
  </main>
</div>
