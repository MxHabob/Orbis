<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import { Unlink } from "@lucide/svelte";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  isGoogleLinked?: boolean;
  googleEmail?: string | null;
  onRefresh?: () => void;
}

let { isGoogleLinked = false, googleEmail = null, onRefresh }: Props = $props();

let isLinking = $state(false);

async function handleLinkGoogle() {
  isLinking = true;
  try {
    const authUrl = `${window.location.origin}/api/v1/auth/identities/google/login`;
    try {
      const { openUrl } = await import("@tauri-apps/plugin-opener");
      await openUrl(authUrl);
    } catch {
      window.open(authUrl, "_blank", "noopener,noreferrer");
    }

    toast.info(m.common_account_identities_redirect_info());
  } catch (err: any) {
    toast.error(err.message || "Failed to connect Google account");
  } finally {
    isLinking = false;
  }
}

function handleDisconnectGoogle() {
  isGoogleLinked = false;
  googleEmail = null;
  toast.success(m.common_account_identities_disconnect_success());
  onRefresh?.();
}
</script>

<SettingsCard title={m.common_account_identities_section()}>
  <SettingsRow
    description={isGoogleLinked
      ? m.common_account_identities_google_linked_desc({ email: googleEmail || "Google Account" })
      : m.common_account_identities_google_desc()}
    last
  >
    {#snippet headerSlot()}
      <div class="flex items-center gap-2">
        <!-- Google Colored SVG Icon -->
        <svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24">
          <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
          <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
          <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
          <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
        </svg>
        <Label class="text-xs font-semibold text-foreground">
          {m.common_account_identities_google_title()}
        </Label>
        {#if isGoogleLinked}
          <Badge variant="default" class="text-[10px] font-medium py-0 px-2 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
            {m.common_account_identities_connected()}
          </Badge>
        {/if}
      </div>
    {/snippet}

    {#snippet children()}
      {#if isGoogleLinked}
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="h-9 px-4 text-xs font-medium text-destructive hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
          onclick={handleDisconnectGoogle}
        >
          <Unlink class="w-3.5 h-3.5 me-1.5" />
          <span>{m.common_account_identities_disconnect()}</span>
        </Button>
      {:else}
        <Button
          type="button"
          variant="outline"
          size="sm"
          disabled={isLinking}
          class="h-9 px-4 text-xs font-semibold gap-2"
          onclick={handleLinkGoogle}
        >
          <svg class="w-3.5 h-3.5 shrink-0" viewBox="0 0 24 24">
            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
          </svg>
          <span>{isLinking ? m.common_account_identities_connecting() : m.common_account_identities_connect_google()}</span>
        </Button>
      {/if}
    {/snippet}
  </SettingsRow>
</SettingsCard>
