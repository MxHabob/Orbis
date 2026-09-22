<!--
@component UserDetailLayout
Pattern A: DetailPageHeader + MainViewport (Avatar sidebar + SecondaryMenu + content).
Decomposed from the original 699-line monolith into a layout + 4 sub-pages.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { confirmStore } from "$lib/stores";
import { api } from "$lib/api";
import type { User } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { Trash } from "@orbis/icons";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import type { Snippet } from "svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import UserResetPasswordDialog from "$lib/features/settings/components/user-reset-password-dialog.svelte";
import { createUserQuery } from "$lib/features/settings/queries";
import { useQueryClient } from "@tanstack/svelte-query";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId);
const userId = $derived(page.params.userId);

// Shared user state
const userQuery = createUserQuery(
  () => String(orgId || ""),
  () => String(userId || ""),
);
const queryClient = useQueryClient();
const user = $derived((userQuery.data as any) ?? null);
const isLoading = $derived(userQuery.isLoading);
const fullName = $derived(user?.full_name ?? "");
const email = $derived(user?.email ?? "");
const role = $derived(user?.roles?.[0] ?? "staff");
const isActiveState = $derived(
  user?.is_active !== undefined ? Boolean(user.is_active) : true,
);
const avatarUrl = $derived(user?.avatar_url ?? null);

function handleAvatarUpload(url: string) {
  queryClient.setQueryData(["user", orgId, userId], (current: any) =>
    current ? { ...current, avatar_url: url } : current,
  );
}

// Reset password modal
let showResetModal = $state(false);
let newPassword = $state("");
let confirmPassword = $state("");
let isResettingPassword = $state(false);

async function handleResetPassword(e: Event) {
  e.preventDefault();
  if (newPassword !== confirmPassword) {
    toast.error(m.settings_user_detail_reset_mismatch());
    return;
  }
  if (!userId) return;
  isResettingPassword = true;
  try {
    await api.updateUser(userId as string, {
      new_password: newPassword,
    });
    toast.success(m.settings_user_detail_reset_success());
    showResetModal = false;
    newPassword = "";
    confirmPassword = "";
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Failed");
  } finally {
    isResettingPassword = false;
  }
}

async function handleDeleteUser() {
  if (!userId) return;
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete() + "؟",
    description: m.settings_user_detail_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    await api.updateUser(userId as string, { is_active: false } as any);
    toast.success(m.settings_user_detail_deleted());
    await goto(localizeHref(`/${orgId}/settings/users`));
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Failed");
  }
}

const basePath = $derived(`/${orgId}/settings/users/${userId}`);

const menuItems = $derived([
  {
    path: "profile",
    label: m.settings_user_detail_tab_profile(),
  },
  {
    path: "roles",
    label: m.settings_nav_permissions(),
  },
  {
    path: "sessions",
    label: m.settings_security_sessions(),
  },
  {
    path: "preferences",
    label: m.settings_user_detail_tab_settings(),
  },
]);
</script>

<svelte:head>
  <title>{fullName || m.settings_user_detail_title()} | Orbis</title>
</svelte:head>

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background"
>
  <!-- Header -->
  <DetailPageHeader
    backHref={`/${orgId}/settings/users`}
    backLabel={m.settings_nav_users()}
  >
    {#if isLoading}
      <Skeleton class="h-6 w-32" />
    {:else}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {fullName || m.settings_user_detail_title()}
      </h1>
      {#if isActiveState}
        <Badge
          variant="success"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
        >
          {m.common_phase_active()}
        </Badge>
      {:else}
        <Badge
          variant="destructive"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
        >
          {m.common_status_disabled()}
        </Badge>
      {/if}
    {/if}

    {#snippet actions()}
      <Button
        variant="outline"
        size="sm"
        onclick={() => (showResetModal = true)}
      >
        {m.settings_user_detail_btn_reset_pass()}
      </Button>
      <Button
        variant="outline"
        size="sm"
        class="text-destructive border-destructive/20 hover:bg-destructive/5"
        onclick={handleDeleteUser}
      >
        <Trash size={13} class="me-1" />
        {m.settings_user_detail_btn_delete()}
      </Button>
    {/snippet}
  </DetailPageHeader>

  <!-- Resizable split view -->
  <div class="flex-1 w-full overflow-hidden">
    <MainViewport
      name="user-detail-viewport"
      leftWidth={{ default: 220, min: 180 }}
    >
      <!-- Left Sidebar (Avatar + basic info) -->
      {#snippet left()}
        <div
          class="h-full p-4 flex flex-col gap-5 overflow-y-auto scrollbar-hide text-start bg-transparent select-none"
        >
          <div class="w-full flex justify-start">
            <AvatarUpload
              {avatarUrl}
              entityType="user"
              entityId={String(userId)}
              size={220}
              fallbackText={fullName}
              class="rounded-xl border border-border"
              onupload={handleAvatarUpload}
            />
          </div>

          <div class="flex flex-col gap-1 w-full pt-1 text-start">
            <span
              class="text-xs text-muted-foreground font-mono truncate max-w-full"
              >{email || "No email"}</span
            >
            <div class="pt-2">
              <Badge
                variant="secondary"
                class="text-[10px] font-semibold px-2 py-0.5 uppercase tracking-wider"
              >
                {role.replace("_", " ")}
              </Badge>
            </div>
          </div>
        </div>
      {/snippet}

      <!-- Right Workspace Area -->
      {#snippet middle()}
        <div class="h-full w-full overflow-y-auto p-6 scrollbar-hide">
          <div class="max-w-3xl mx-auto flex flex-col gap-6">
            <!-- Secondary Menu -->
            <SecondaryMenu items={menuItems} {basePath} />

            <!-- Children Sub-Route Viewport -->
            <div class="w-full flex-1">
              {@render children()}
            </div>
          </div>
        </div>
      {/snippet}
    </MainViewport>
  </div>
</div>

<UserResetPasswordDialog bind:open={showResetModal} fullName={fullName} bind:newPassword bind:confirmPassword isResettingPassword={isResettingPassword} onsubmit={handleResetPassword} oncancel={() => { showResetModal = false; newPassword = ""; confirmPassword = ""; }} />
