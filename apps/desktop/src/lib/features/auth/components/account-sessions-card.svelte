<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui/card";
import { ShieldCheck, LogOut, Laptop, Smartphone, Monitor } from "@lucide/svelte";

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

let { activeSessions, onRevoke, ...restProps }: Props & Record<string, any> = $props();
</script>

<Card class="border border-border/80 shadow-sm rounded-2xl overflow-hidden" {...restProps}>
  <CardHeader class="bg-muted/40 px-6 py-4 border-b border-border/60">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-xl bg-primary/10 text-primary">
          <Monitor size={20} class="stroke-[2.2]" />
        </div>
        <div>
          <CardTitle class="text-sm font-bold text-foreground">
            {m.settings_security_sessions()}
          </CardTitle>
          <p class="text-xs text-muted-foreground mt-0.5">
            {m.settings_security_sessions_desc()}
          </p>
        </div>
      </div>
    </div>
  </CardHeader>

  <CardContent class="p-0">
    <div class="flex flex-col divide-y divide-border/40">
      {#each activeSessions as session (session.id)}
        <div class="flex items-center justify-between px-6 py-4 hover:bg-muted/20 transition-colors">
          <div class="flex items-center gap-3.5">
            <div class="p-2.5 rounded-xl bg-muted/80 text-foreground border border-border/40 shrink-0">
              {#if session.icon}
                <session.icon size={18} />
              {:else}
                <Laptop size={18} />
              {/if}
            </div>
            <div class="flex flex-col text-start gap-1">
              <div class="flex items-center gap-2">
                <span class="text-xs font-bold text-foreground">{session.device}</span>
                {#if session.isCurrent}
                  <Badge variant="default" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full bg-primary/15 text-primary border-primary/20">
                    This Device
                  </Badge>
                {/if}
                {#if session.isTrusted}
                  <Badge variant="success" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full flex items-center gap-1">
                    <ShieldCheck size={10} />
                    <span>Trusted Device (30d)</span>
                  </Badge>
                {/if}
              </div>
              <span class="text-[11px] text-muted-foreground">
                {session.location || "Active Session"} • {session.ip || "Local Network"}
                {#if session.lastActive}
                  • Last active {session.lastActive}
                {/if}
              </span>
            </div>
          </div>
          
          <div class="flex items-center gap-2">
            {#if !session.isCurrent && onRevoke}
              <Button
                variant="ghost"
                size="sm"
                class="rounded-xl text-xs font-medium text-destructive hover:bg-destructive/10 hover:text-destructive gap-1.5 h-8 px-2.5"
                onclick={() => onRevoke(session.id)}
              >
                <LogOut size={13} />
                <span>Revoke</span>
              </Button>
            {:else if session.isCurrent}
              <Badge variant="success" class="text-[9px] font-bold uppercase py-0.5 px-2.5 rounded-full">
                Active Now
              </Badge>
            {/if}
          </div>
        </div>
      {/each}

      {#if activeSessions.length === 0}
        <div class="py-8 text-center text-xs text-muted-foreground">
          No other active sessions recorded.
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
