<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface SessionItem {
  id: string;
  device: string;
  location?: string;
  ip?: string;
  active?: boolean;
  isTrusted?: boolean;
  isCurrent?: boolean;
  lastActive?: string;
  icon?: any;
}

interface Props {
  activeSessions: SessionItem[];
  onRevoke?: (id: string) => void;
}

let { activeSessions, onRevoke }: Props = $props();
</script>

<SettingsCard title={m.common_account_sessions_section()}>
  {#each activeSessions as session, i (session.id)}
    <SettingsRow
      description={`${session.location || m.common_account_sessions_active_session()} • ${session.ip || "Localhost"}${session.lastActive ? ` • ${m.common_account_sessions_last_active({ time: session.lastActive })}` : ""}`}
      last={i === activeSessions.length - 1}
    >
      {#snippet headerSlot()}
        <div class="flex items-center gap-2">
          <span class="text-xs font-semibold text-foreground">{session.device}</span>
          {#if session.isCurrent}
            <Badge variant="default" class="text-[10px] font-medium py-0 px-1.5 bg-primary/15 text-primary border-primary/20">
              {m.common_account_sessions_current_device()}
            </Badge>
          {/if}
          {#if session.isTrusted}
            <Badge variant="default" class="text-[10px] font-medium py-0 px-1.5 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
              {m.common_account_sessions_trusted()}
            </Badge>
          {/if}
        </div>
      {/snippet}

      {#snippet children()}
        {#if !session.isCurrent && onRevoke}
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="h-8 px-3 text-xs text-destructive hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
            onclick={() => onRevoke(session.id)}
          >
            <span>{m.common_account_sessions_revoke_btn()}</span>
          </Button>
        {:else if session.isCurrent}
          <Badge variant="default" class="text-[10px] font-medium py-0.5 px-2 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
            {m.common_account_sessions_active_now()}
          </Badge>
        {/if}
      {/snippet}
    </SettingsRow>
  {/each}

  {#if activeSessions.length === 0}
    <div class="py-8 text-center text-xs text-muted-foreground">
      {m.common_account_sessions_empty()}
    </div>
  {/if}
</SettingsCard>
