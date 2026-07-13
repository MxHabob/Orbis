<!--
@component UserDetailLayout
Pattern A: DetailPageHeader + MainViewport (Avatar sidebar + SecondaryMenu + content).
Decomposed from the original 699-line monolith into a layout + 4 sub-pages.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/core/ipc";
import type { User } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogDescription,
  DialogHeader,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { toast } from "svelte-sonner";
import { Trash2 } from "lucide-svelte";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import type { Snippet } from "svelte";
import * as settings from "$lib/paraglide/messages/settings";
import { localizeHref } from "$lib/paraglide/runtime";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId);
const userId = $derived(page.params.userId);

// Shared user state
let user = $state<User | null>(null);
let isLoading = $state(true);
let fullName = $state("");
let email = $state("");
let role = $state("teacher");
let isActiveState = $state(true);
let avatarUrl = $state<string | null>(null);

// Reset password modal
let showResetModal = $state(false);
let newPassword = $state("");
let confirmPassword = $state("");
let isResettingPassword = $state(false);

async function loadUser() {
  if (!userId) return;
  isLoading = true;
  try {
    const fetched = (await commands.iam.getUser(userId)) as any;
    user = fetched;
    fullName = fetched.full_name || "";
    email = fetched.email || "";
    role = fetched.primary_role || fetched.role || "teacher";
    isActiveState =
      fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
    avatarUrl = fetched.avatar_url || fetched.avatar || null;
  } catch (e) {
    console.error("Failed to load user:", e);
    toast.error(settings.user_detail_load_error());
  } finally {
    isLoading = false;
  }
}

onMount(loadUser);

async function handleResetPassword(e: Event) {
  e.preventDefault();
  if (newPassword !== confirmPassword) {
    toast.error(settings.user_detail_reset_mismatch());
    return;
  }
  if (!userId) return;
  isResettingPassword = true;
  try {
    await commands.iam.changePassword(userId as string, {
      new_password: newPassword,
    });
    toast.success(settings.user_detail_reset_success());
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
  if (!confirm(settings.user_detail_delete_confirm())) return;
  try {
    await commands.iam.deleteUser(userId as string, "admin");
    toast.success(settings.user_detail_deleted());
    await goto(localizeHref(`/${orgId}/settings/users`));
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Failed");
  }
}

const basePath = $derived(`/${orgId}/settings/users/${userId}`);

const menuItems = $derived([
  {
    path: "profile",
    label: settings.user_detail_tab_profile(),
  },
  {
    path: "roles",
    label: settings.user_detail_tab_roles(),
  },
  {
    path: "sessions",
    label: settings.user_detail_tab_sessions(),
  },
  {
    path: "preferences",
    label: settings.user_detail_tab_settings(),
  },
]);
</script>

<svelte:head>
  <title>{fullName || settings.user_detail_title()} | Orbis</title>
</svelte:head>

<div
  class="h-full w-full flex flex-col overflow-hidden bg-background"
>
  <!-- Header -->
  <DetailPageHeader
    backHref={`/${orgId}/settings/users`}
    backLabel={settings.nav_users()}
  >
    {#if isLoading}
      <Skeleton class="h-6 w-32" />
    {:else}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {fullName || settings.user_detail_title()}
      </h1>
      {#if isActiveState}
        <Badge
          variant="success"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
        >
          {settings.user_detail_badge_active()}
        </Badge>
      {:else}
        <Badge
          variant="destructive"
          class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
        >
          {settings.user_detail_badge_inactive()}
        </Badge>
      {/if}
    {/if}

    {#snippet actions()}
      <Button
        variant="outline"
        size="sm"
        onclick={() => (showResetModal = true)}
      >
        {settings.user_detail_btn_reset_pass()}
      </Button>
      <Button
        variant="outline"
        size="sm"
        class="text-destructive border-destructive/20 hover:bg-destructive/5"
        onclick={handleDeleteUser}
      >
        <Trash2 size={13} class="me-1" />
        {settings.user_detail_btn_delete()}
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
              bind:avatarUrl
              size={220}
              fallbackText={fullName}
              class="rounded-xl border border-border"
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

<!-- Reset Password Dialog -->
<Dialog bind:open={showResetModal}>
  <DialogContent class="max-w-md text-start">
    <form onsubmit={handleResetPassword} class="flex flex-col gap-4">
      <DialogHeader>
        <DialogTitle>{settings.user_detail_reset_pass_title()}</DialogTitle>
        <DialogDescription>
          {settings.user_detail_reset_pass_desc({
            name: fullName || "User",
          })}
        </DialogDescription>
      </DialogHeader>

      <div class="flex flex-col gap-2 text-start">
        <Label
          for="new_password"
          class="text-xs font-medium text-muted-foreground"
        >
          {settings.user_detail_new_password()}
        </Label>
        <Input
          id="new_password"
          type="password"
          bind:value={newPassword}
          placeholder="••••••••"
          required
          minlength={6}
          disabled={isResettingPassword}
        />
      </div>

      <div class="flex flex-col gap-2 text-start">
        <Label
          for="confirm_password"
          class="text-xs font-medium text-muted-foreground"
        >
          {settings.user_detail_confirm_password()}
        </Label>
        <Input
          id="confirm_password"
          type="password"
          bind:value={confirmPassword}
          placeholder="••••••••"
          required
          disabled={isResettingPassword}
        />
      </div>

      <DialogFooter class="border-t border-border pt-4 mt-2">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          onclick={() => {
            showResetModal = false;
            newPassword = "";
            confirmPassword = "";
          }}
          disabled={isResettingPassword}
        >
          {settings.user_detail_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          size="sm"
          class="font-bold"
          disabled={isResettingPassword}
        >
          {isResettingPassword
            ? "..."
            : settings.user_detail_btn_reset_pass()}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
