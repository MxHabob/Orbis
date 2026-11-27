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
import { api, extractErrorMessage } from "$lib/api";
import type { UserDto, AccountProvisioningResultDto } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { Lock, Refresh, ShieldCheck, Trash, User } from "@orbis/icons";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import SplitActionMenu, {
  type SplitActionItem,
} from "$lib/shared/components/split-action-menu.svelte";
import { userAvatarDrafts } from "$lib/features/settings/user-avatar-draft.svelte";
import type { Snippet } from "svelte";

import { formatDate } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import CredentialDeliveryModal from "$lib/shared/components/credential-delivery-modal.svelte";
import {
  isUserActive,
  invalidateUserQueries,
} from "$lib/features/settings/user-detail";
import { useQueryClient } from "@tanstack/svelte-query";
import { createUserQuery } from "$lib/features/settings/queries";

let { children }: { children: Snippet } = $props();

const queryClient = useQueryClient();
const orgId = $derived(page.params.orgId);
const userId = $derived(page.params.userId);
$effect(() => {
  const id = String(userId || "");
  if (id && !(id in userAvatarDrafts)) userAvatarDrafts[id] = null;
});

// Shared user state
const userQuery = createUserQuery(
  () => String(orgId || ""),
  () => String(userId || ""),
);
const user = $derived<UserDto | null>(userQuery.data ?? null);
const isLoading = $derived(userQuery.isLoading);
const fullName = $derived(user?.full_name ?? "");
const email = $derived(user?.email ?? "");
const role = $derived(user?.roles?.[0] ?? "staff");
const isActiveState = $derived(isUserActive(user));
const avatarUrl = $derived(user?.avatar_url ?? null);

// CSPRNG Temporary Password Generation
let credentialModalOpen = $state(false);
let activeProvisioningResult = $state<AccountProvisioningResultDto | null>(
  null,
);

async function handleGenerateTempPassword() {
  if (!userId) return;
  const confirmed = await confirmStore.prompt({
    title: m.shared_ui_reset_password_confirmation_title(),
    description: m.shared_ui_generate_temp_password_confirmation({
      user: fullName || email,
    }),
    confirmText: m.settings_user_detail_btn_reset_pass(),
  });
  if (confirmed) {
    try {
      const result = await api.resetUserPassword(userId);
      activeProvisioningResult = result;
      credentialModalOpen = true;
      toast.success(m.employees_password_reset_success());
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e) || m.shared_ui_reset_password_failed());
    }
  }
}

async function handleDeleteUser() {
  if (!userId) return;
  const confirmed = await confirmStore.prompt({
    title: m.shared_ui_delete_user_confirmation_title(),
    description: m.settings_user_detail_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  try {
    await api.updateUser(userId, { is_active: false });
    await invalidateUserQueries(queryClient, String(orgId || ""), userId);
    toast.success(m.settings_user_detail_deleted());
    await goto(localizeHref(`/${orgId}/settings/users`));
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || m.shared_ui_action_failed());
  }
}

const basePath = $derived(`/${orgId}/settings/users/${userId}`);

const menuItems = $derived([
  {
    path: "",
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

async function handleToggleAccountStatus() {
  if (!userId || !user) return;
  const nextState = !isActiveState;
  const confirmed = await confirmStore.prompt({
    title: m.common_word_status ? m.common_word_status() : m.common_status(),
    description: nextState
      ? m.settings_user_detail_activate_confirm
        ? m.settings_user_detail_activate_confirm()
        : m.ui_copy_activate_account_89c1b85d()
      : m.settings_user_detail_deactivate_confirm
        ? m.settings_user_detail_deactivate_confirm()
        : m.ui_copy_deactivate_account_084a7a7b(),
    confirmText: m.common_word_status
      ? m.common_word_status()
      : m.ui_copy_confirm_0100f51c(),
    variant: nextState ? "default" : "destructive",
  });
  if (!confirmed) return;
  try {
    await api.updateUser(userId, { is_active: nextState });
    await invalidateUserQueries(queryClient, String(orgId || ""), userId);
    toast.success(
      nextState
        ? m.settings_user_detail_activate_success
          ? m.settings_user_detail_activate_success()
          : m.ui_copy_account_activated_ecaf5563()
        : m.settings_user_detail_deactivate_success
          ? m.settings_user_detail_deactivate_success()
          : m.ui_copy_account_deactivated_37ae84cf(),
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || m.ui_copy_update_failed_19a589b0());
  }
}

const headerActions = $derived<SplitActionItem[]>(
  user
    ? [
        {
          id: "refresh",
          label: m.common_refresh(),
          icon: Refresh,
          disabled: userQuery.isFetching,
          onSelect: () => void userQuery.refetch(),
        },
        {
          id: "audit",
          label: m.ui_copy_user_audit_logs_55f2f1f7(),
          icon: ShieldCheck,
          onSelect: () =>
            void goto(localizeHref(`/${orgId}/settings/audit?userId=${userId}`)),
        },
        {
          id: "toggle-status",
          label: isActiveState
            ? m.ui_copy_deactivate_account_084a7a7b()
            : m.ui_copy_activate_account_89c1b85d(),
          icon: User,
          separatorBefore: true,
          onSelect: handleToggleAccountStatus,
        },
        {
          id: "delete_user",
          label: m.settings_user_detail_btn_delete(),
          icon: Trash,
          destructive: true,
          separatorBefore: true,
          onSelect: handleDeleteUser,
        },
      ]
    : [],
);
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
      <SplitActionMenu
        label={m.settings_user_detail_btn_reset_pass()}
        menuLabel={m.operations_ui_actions()}
        icon={Lock}
        variant="outline"
        disabled={isLoading || !user}
        actions={headerActions}
        onprimary={handleGenerateTempPassword}
      />
    {/snippet}
  </DetailPageHeader>

  <!-- Resizable split view -->
  <div class="flex-1 w-full overflow-hidden">
    <MainViewport
      name="user-detail-viewport"
      leftWidth={{ default: 256, min: 220 }}
    >
      <!-- Left Sidebar (Avatar + basic info) -->
      {#snippet left()}
        <div
          class="h-full p-4 flex flex-col gap-5 overflow-y-auto scrollbar-hide text-start bg-transparent select-none"
        >
          <div class="w-full flex justify-center">
            <AvatarUpload
              {avatarUrl}
              bind:file={userAvatarDrafts[userId ?? ""]}
              deferred
              entityType="user"
              entityId={String(userId)}
              size={192}
              fallbackText={fullName}
              class="rounded-md border border-border"
            />
          </div>

          <div class="flex flex-col gap-1.5 w-full pt-1 text-start">
            <span
              class="text-xs text-muted-foreground font-mono truncate max-w-full"
              >{email || "No email"}</span
            >
            <div class="flex flex-wrap items-center gap-1.5 pt-1">
              <Badge
                variant="secondary"
                class="text-[10px] font-semibold px-2 py-0.5 uppercase tracking-wider"
              >
                {role.replace("_", " ")}
              </Badge>
              {#if user?.is_mfa_enabled}
                <Badge
                  variant="success"
                  class="text-[10px] font-semibold px-2 py-0.5 gap-1"
                >
                  <ShieldCheck size={11} />
                  <span>MFA</span>
                </Badge>
              {/if}
            </div>
            {#if user?.created_at}
              <span class="text-[11px] font-mono text-muted-foreground pt-1">
                {formatDate(user.created_at)}
              </span>
            {/if}
          </div>
        </div>
      {/snippet}

      <!-- Right Workspace Area -->
      {#snippet middle()}
        <div class="h-full w-full overflow-y-auto p-6 pb-24 scrollbar-hide">
          <div class="w-full max-w-6xl mx-auto flex flex-col gap-6">
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


<CredentialDeliveryModal
  bind:open={credentialModalOpen}
  provisioning={activeProvisioningResult}
  userName={fullName || email}
/>
