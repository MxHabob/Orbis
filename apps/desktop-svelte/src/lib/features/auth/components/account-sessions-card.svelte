<script lang="ts">
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui-svelte/card";
import * as m from "$lib/paraglide/messages";

interface Props {
  activeSessions: any[];
}

let { activeSessions, ...restProps }: Props & Record<string, any> = $props();
</script>

<Card {...restProps}>
  <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
    <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
      {m.settings_security_sessions()}
    </CardTitle>
  </CardHeader>
  <CardContent>
    <div class="flex flex-col">
      <div class="px-6 py-3 bg-muted/20 border-b border-border text-[11px] text-muted-foreground text-start select-none">
        {m.settings_security_sessions_desc()}
      </div>

      <div class="flex flex-col">
        {#each activeSessions as session (session.id)}
          <div class="flex items-center justify-between px-6 py-4 border-b border-border/40 last:border-b-0">
            <div class="flex items-center gap-3">
              <div class="p-2.5 rounded-lg bg-muted text-foreground">
                <session.icon size={18} />
              </div>
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-bold text-foreground">{session.device}</span>
                <span class="text-[10px] text-muted-foreground">{session.location} • {session.ip}</span>
              </div>
            </div>
            
            {#if session.active}
              <Badge variant="success" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full">
                Active Now
              </Badge>
            {:else}
              <Badge variant="secondary" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full">
                Logged out
              </Badge>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </CardContent>
</Card>
