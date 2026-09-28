<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, type UserDeviceDto } from "$lib/api";
import { confirmStore } from "$lib/stores";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { Computer, Trash, ShieldCheck } from "@orbis/icons";
import { fade } from "svelte/transition";
import { formatDate, l } from "$lib/core/i18n";

let { orgId, userId }: { orgId: string; userId: string } = $props();

let activeSessions = $state<UserDeviceDto[]>([]);
let isLoading = $state(true);

async function loadSessions() {
  if (!userId) return;
  isLoading = true;
  try {
    const rawSessions = await api.listDevices();
    activeSessions = rawSessions;
  } catch (err) {
    console.error("Failed to load user sessions:", err);
  } finally {
    isLoading = false;
  }
}

async function handleRevoke(deviceId: string, deviceName: string) {
  const confirmed = await confirmStore.prompt({
    title: l("إنهاء جلسة الجهاز؟", "Revoke Device Session?"),
    description: l(
      `هل أنت متأكد من إنهاء جلسة الجهاز (${deviceName}) وفصل تسجيل الدخول؟`,
      `Are you sure you want to revoke the session for ${deviceName}?`
    ),
    confirmText: l("إنهاء الجلسة", "Revoke Session"),
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await api.revokeDevice(deviceId);
      activeSessions = activeSessions.filter((s) => s.id !== deviceId);
      toast.success(l("تم إنهاء الجلسة بنجاح", "Device session revoked"));
    } catch (e: any) {
      toast.error(e?.message || l("فشل إنهاء الجلسة", "Failed to revoke device"));
    }
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
    <Computer size={40} class="text-muted-foreground/30" />
    <p class="text-xs font-semibold">
      {m.settings_user_detail_no_sessions()}
    </p>
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-3 text-start">
    {#each activeSessions as session (session.id)}
      {@const deviceTitle = session.device_name || session.device_id || "Unknown Device"}
      {@const platformInfo = [session.os, session.browser].filter(Boolean).join(" • ") || session.device_type}
      <div
        class="flex items-center justify-between p-3.5 rounded-xl border border-border/60 bg-card hover:border-border transition-colors"
      >
        <div class="flex items-center gap-3.5">
          <div class="p-2.5 rounded-lg bg-primary/10 border border-primary/20 text-primary shrink-0">
            <Computer size={18} />
          </div>
          <div class="flex flex-col gap-1 text-start">
            <div class="flex items-center gap-2 flex-wrap">
              <span class="text-xs font-bold text-foreground">
                {deviceTitle}
              </span>
              {#if session.is_trusted}
                <Badge
                  variant="success"
                  class="text-[9px] font-semibold py-0 px-1.5 h-4.5 rounded gap-1"
                >
                  <ShieldCheck class="w-3 h-3" />
                  <span>{l("موثوق", "Trusted")}</span>
                </Badge>
              {/if}
            </div>
            <span class="text-[11px] text-muted-foreground font-mono">
              {platformInfo} • {session.last_active_at ? formatDate(session.last_active_at, "datetime") : "—"}
            </span>
          </div>
        </div>

        <Button
          variant="ghost"
          size="sm"
          class="text-xs text-destructive hover:bg-destructive/10 h-8 px-3 gap-1.5 cursor-pointer"
          onclick={() => handleRevoke(session.id, deviceTitle)}
        >
          <Trash size={13} />
          <span>{l("إنهاء الجلسة", "Revoke")}</span>
        </Button>
      </div>
    {/each}
  </div>
{/if}
