<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Label } from "@orbis/ui/label";
import { Settings, UserBadgeCheck } from "@orbis/icons";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import MfaEnrollModal from "./mfa-enroll-modal.svelte";

interface Props {
  isMfaEnabled?: boolean;
  onRefresh?: () => void;
}

let { isMfaEnabled = false, onRefresh }: Props = $props();

let showEnrollModal = $state(false);
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.common_account_mfa_section()}</CardTitle>
  </CardHeader>
  <CardContent>
    <SettingsRow
      description={isMfaEnabled ? m.common_account_mfa_enabled_desc() : m.common_account_mfa_disabled_desc()}
      last
    >
      {#snippet headerSlot()}
        <div class="flex items-center gap-2">
          <Label class="text-xs font-semibold text-foreground">
            {m.common_account_mfa_totp_title()}
          </Label>
          {#if isMfaEnabled}
            <Badge variant="default" class="text-[10px] font-medium py-0 px-2 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
              {m.common_account_mfa_active()}
            </Badge>
          {:else}
            <Badge variant="secondary" class="text-[10px] font-medium py-0 px-2 text-muted-foreground">
              {m.common_account_mfa_disabled()}
            </Badge>
          {/if}
        </div>
      {/snippet}

      {#snippet children()}
        {#if isMfaEnabled}
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="h-9 px-4 text-xs font-medium gap-1.5"
            onclick={() => (showEnrollModal = true)}
          >
            <Settings class="w-3.5 h-3.5" />
            <span>{m.common_account_mfa_reconfigure_btn()}</span>
          </Button>
        {:else}
          <Button
            type="button"
            size="sm"
            class="h-9 px-4 text-xs font-semibold gap-1.5"
            onclick={() => (showEnrollModal = true)}
          >
            <UserBadgeCheck class="w-3.5 h-3.5" />
            <span>{m.common_account_mfa_enable_btn()}</span>
          </Button>
        {/if}
      {/snippet}
    </SettingsRow>
  </CardContent>
</Card>

<MfaEnrollModal
  bind:open={showEnrollModal}
  onSuccess={() => {
    showEnrollModal = false;
    onRefresh?.();
  }}
/>
