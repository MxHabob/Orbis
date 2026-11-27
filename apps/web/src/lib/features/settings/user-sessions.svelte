<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type UserDeviceDto } from "$lib/api";
import { authStore, confirmStore } from "$lib/stores";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { Computer, Trash, ShieldCheck } from "@orbis/icons";
import { formatDate, l } from "$lib/core/i18n";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import EmptyState from "$lib/shared/components/empty-state.svelte";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const queryClient = useQueryClient();
const isOwnAccount = $derived(Boolean(userId) && authStore.userId === userId);

const devicesQuery = createQuery<UserDeviceDto[]>(() => ({
  queryKey: ["auth", "devices", orgId, userId],
  queryFn: () => api.listDevices(),
  enabled: isOwnAccount,
  staleTime: 60 * 1000,
}));

const activeSessions = $derived(devicesQuery.data ?? []);

const revokeMutation = createMutation(() => ({
  mutationFn: (deviceId: string) => api.revokeDevice(deviceId),
  onSuccess: async () => {
    await queryClient.invalidateQueries({
      queryKey: ["auth", "devices", orgId, userId],
    });
    toast.success(m.ui_copy_device_session_revoked_1690099f());
  },
  onError: (e: unknown) => {
    toast.error(
      extractErrorMessage(e) || m.ui_copy_failed_to_revoke_device_fa2a67e6(),
    );
  },
}));

async function handleRevoke(deviceId: string, deviceName: string) {
  const confirmed = await confirmStore.prompt({
    title: m.ui_copy_revoke_device_session_359e4435(),
    description: l(
      m.settings_ui_session_terminate_confirm({ device: deviceName }),
      `Are you sure you want to revoke the session for ${deviceName}?`,
    ),
    confirmText: m.ui_copy_revoke_session_3a121f98(),
    variant: "destructive",
  });
  if (confirmed) {
    revokeMutation.mutate(deviceId);
  }
}
</script>

{#if !isOwnAccount}
  <EmptyState
    icon={Computer}
    title={m.settings_user_detail_no_sessions()}
  />
{:else if devicesQuery.isLoading}
  <div class="flex flex-col gap-4">
    {#each Array(3) as _, index (index)}
      <div class="flex items-center gap-3 py-3">
        <Skeleton class="h-10 w-10 rounded-md" />
        <div class="flex flex-col gap-1">
          <Skeleton class="h-4 w-40" />
          <Skeleton class="h-3 w-60" />
        </div>
      </div>
    {/each}
  </div>
{:else if devicesQuery.isError}
  <EmptyState
    icon={Computer}
    title={m.common_search_load_failed()}
    description={extractErrorMessage(devicesQuery.error)}
    actionLabel={m.common_retry()}
    onaction={() => void devicesQuery.refetch()}
  />
{:else if activeSessions.length === 0}
  <EmptyState
    icon={Computer}
    title={m.settings_user_detail_no_sessions()}
  />
{:else}
  <div class="flex flex-col gap-3 text-start">
    {#each activeSessions as session (session.id)}
      {@const deviceTitle =
        session.device_name || session.device_id || "Unknown Device"}
      {@const platformInfo =
        [session.os, session.browser].filter(Boolean).join(" • ") ||
        session.device_type}
      <div
        class="flex items-center justify-between p-3.5 rounded-md border border-border bg-card transition-colors"
      >
        <div class="flex items-center gap-3.5">
          <div
            class="p-2.5 rounded-md bg-primary/10 border border-primary/20 text-primary shrink-0"
          >
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
                  <span>{m.ui_copy_trusted_1c40faa5()}</span>
                </Badge>
              {/if}
            </div>
            <span class="text-[11px] text-muted-foreground font-mono">
              {platformInfo} • {session.last_active_at
                ? formatDate(session.last_active_at, "datetime")
                : "—"}
            </span>
          </div>
        </div>

        <Button
          variant="ghost"
          size="sm"
          class="text-xs text-destructive hover:bg-destructive/10 h-8 px-3 gap-1.5 cursor-pointer"
          disabled={revokeMutation.isPending}
          onclick={() => handleRevoke(session.id, deviceTitle)}
        >
          <Trash size={13} />
          <span>{m.common_account_sessions_revoke_btn()}</span>
        </Button>
      </div>
    {/each}
  </div>
{/if}
