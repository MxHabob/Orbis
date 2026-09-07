<!--
@component UserSessionsPage
Sessions tab for user detail — lists active devices/sessions.
Extracted from the original monolith TabsContent "sessions".
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { api, } from "$lib/api";
import type { UserSession } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { Monitor, Smartphone } from "@lucide/svelte";
import { fade } from "svelte/transition";
import { formatDate } from "$lib/core/i18n";

const userId = $derived(page.params.userId);

let activeSessions = $state<any[]>([]);
let isLoading = $state(true);

async function loadSessions() {
  if (!userId) return;
  isLoading = true;
  try {
    const rawSessions = await api.iam.listSessions(userId);
    activeSessions = rawSessions.map((s: UserSession) => {
      const isMobile =
        s.device_type === "mobile" ||
        (s.user_agent && /mobile|android|iphone|ipad/i.test(s.user_agent));
      const icon = isMobile ? Smartphone : Monitor;

      let device = "Unknown Device";
      if (s.device_name) {
        device = s.device_name;
        if (s.browser) device += ` (${s.browser})`;
      } else if (s.os) {
        device = s.os;
        if (s.browser) device += ` (${s.browser})`;
      }

      const location =
        s.is_current === 1
          ? m.settings_user_session_current_device()
          : m.settings_user_session_active_device();
      const time =
        s.is_current === 1
          ? m.settings_user_session_active_now()
          : s.last_active_at
            ? formatDate(s.last_active_at)
            : "";

      return {
        id: s.id,
        device,
        location,
        ip: s.ip_address || "127.0.0.1",
        active: s.is_current === 1,
        icon,
        time,
      };
    });
  } catch (err) {
    console.error("Failed to load user sessions:", err);
  } finally {
    isLoading = false;
  }
}

onMount(loadSessions);
</script>

{#if isLoading}
  <div class="flex flex-col gap-4">
    {#each Array(3) as _}
      <div class="flex items-center gap-3 py-3">
        <Skeleton class="h-10 w-10 rounded-lg" />
        <div class="flex flex-col gap-1">
          <Skeleton class="h-4 w-40" />
          <Skeleton class="h-3 w-60" />
        </div>
      </div>
    {/each}
  </div>
{:else if activeSessions.length === 0}
  <div
    class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-border border-dashed rounded-xl gap-3"
  >
    <Monitor size={40} class="text-muted-foreground/30" />
    <p class="text-xs font-semibold">
      {m.settings_user_detail_no_sessions()}
    </p>
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-4 text-start">
    {#each activeSessions as session (session.id)}
      <div
        class="flex items-center justify-between py-3 border-b border-border/40 last:border-b-0"
      >
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-muted text-foreground">
            <session.icon size={18} />
          </div>
          <div class="flex flex-col gap-0.5">
            <span
              class="text-xs font-bold text-foreground flex items-center gap-2"
            >
              <span>{session.device}</span>
              {#if session.active}
                <Badge
                  variant="success"
                  class="text-[8px] font-semibold uppercase py-0 px-1.5 h-4.5 rounded"
                >
                  {m.settings_user_session_active_now()}
                </Badge>
              {/if}
            </span>
            <span class="text-[10px] text-muted-foreground font-mono">
              {session.location} • {session.ip} • {session.time}
            </span>
          </div>
        </div>
      </div>
    {/each}
  </div>
{/if}
