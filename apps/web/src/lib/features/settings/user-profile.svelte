<!--
  @component UserProfile
  User profile management in Settings.
  Follows the canonical Organization Settings layout using Card + SettingsRow + CopyableInput.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type GroupDetailDto,
  type UserDto,
} from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Switch } from "@orbis/ui/switch";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { Pencil, UserGroup } from "@orbis/icons";
import { createUserQuery } from "$lib/features/settings/queries";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import CopyableInput from "$lib/shared/components/copyable-input.svelte";
import RoleSelect from "$lib/shared/components/role-select.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import ManageUserGroupsSheet from "./components/manage-user-groups-sheet.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import { userAvatarDrafts } from "./user-avatar-draft.svelte";
import { invalidateUserQueries, isUserActive } from "./user-detail";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const userQuery = createUserQuery(
  () => orgId,
  () => userId,
);
const queryClient = useQueryClient();
const user = $derived<UserDto | null>(userQuery.data ?? null);
const isLoading = $derived(userQuery.isLoading);

const groupsQuery = createQuery<GroupDetailDto[]>(() => ({
  queryKey: ["groups", orgId],
  queryFn: () => api.listGroups(),
  staleTime: 5 * 60 * 1000,
}));

const allGroups = $derived(groupsQuery.data ?? []);

const userGroupsQuery = createQuery<GroupDetailDto[]>(() => ({
  queryKey: ["user_assigned_groups", orgId, userId, allGroups.map((g) => g.id)],
  queryFn: async () => {
    const memberships = await Promise.all(
      allGroups.map(async (g) => {
        try {
          const members = await api.getGroupMembers(g.id);
          return members.some((member) => member.user_id === userId) ? g : null;
        } catch {
          return null;
        }
      }),
    );
    return memberships.filter((g): g is GroupDetailDto => g !== null);
  },
  enabled: !!userId && allGroups.length > 0,
  staleTime: 60 * 1000,
}));

const userAssignedGroups = $derived(userGroupsQuery.data ?? []);
const userAssignedGroupIds = $derived(userAssignedGroups.map((g) => g.id));

let groupsSheetOpen = $state(false);

// Form fields
let fullName = $state("");
let nameAr = $state("");
let email = $state("");
let phone = $state("");
let nationalId = $state("");
let gender = $state("");
let birthDate = $state("");
let address = $state("");
let role = $state("staff");
let isActiveState = $state(true);

let initializedUserId = $state<string | null>(null);
$effect(() => {
  const fetched = user;
  if (!fetched || initializedUserId === userId) return;
  fullName = fetched.full_name || "";
  nameAr = fetched.full_name_ar || "";
  email = fetched.email || "";
  phone = fetched.phone || "";
  nationalId = fetched.national_id || "";
  gender = fetched.gender || "";
  birthDate = fetched.birth_date || "";
  address = fetched.address || "";
  role = fetched.roles?.[0] || "staff";
  isActiveState = isUserActive(fetched);
  initializedUserId = userId;
});

const isDirty = $derived.by(() => {
  if (!user) return false;
  return (
    fullName !== (user.full_name || "") ||
    nameAr !== (user.full_name_ar || "") ||
    phone !== (user.phone || "") ||
    nationalId !== (user.national_id || "") ||
    gender !== (user.gender || "") ||
    birthDate !== (user.birth_date || "") ||
    address !== (user.address || "") ||
    role !== (user.roles?.[0] || "staff") ||
    isActiveState !== isUserActive(user) ||
    !!userAvatarDrafts[userId]
  );
});

function resetForm() {
  if (!user) return;
  fullName = user.full_name || "";
  nameAr = user.full_name_ar || "";
  email = user.email || "";
  phone = user.phone || "";
  nationalId = user.national_id || "";
  gender = user.gender || "";
  birthDate = user.birth_date || "";
  address = user.address || "";
  role = user.roles?.[0] || "staff";
  isActiveState = isUserActive(user);
  userAvatarDrafts[userId] = null;
}

const toggleStatusMutation = createMutation(() => ({
  mutationFn: (nextState: boolean) =>
    api.updateUser(userId, { is_active: nextState }),
  onSuccess: async (_data, nextState) => {
    isActiveState = nextState;
    await invalidateUserQueries(queryClient, orgId, userId);
    toast.success(
      nextState
        ? m.settings_user_detail_activate_success
          ? m.settings_user_detail_activate_success()
          : m.ui_copy_account_activated_ecaf5563()
        : m.settings_user_detail_deactivate_success
          ? m.settings_user_detail_deactivate_success()
          : m.ui_copy_account_deactivated_37ae84cf(),
    );
  },
  onError: (e: unknown) => {
    toast.error(extractErrorMessage(e) || m.ui_copy_update_failed_19a589b0());
  },
}));

async function toggleAccountStatus() {
  if (!userId || !user || toggleStatusMutation.isPending) return;
  const nextState = !isActiveState;
  const confirmMsg = nextState
    ? m.settings_user_detail_activate_confirm
      ? m.settings_user_detail_activate_confirm()
      : m.ui_copy_activate_account_89c1b85d()
    : m.settings_user_detail_deactivate_confirm
      ? m.settings_user_detail_deactivate_confirm()
      : m.ui_copy_deactivate_account_084a7a7b();
  const confirmed = await confirmStore.prompt({
    title: m.common_word_status ? m.common_word_status() : m.common_status(),
    description: confirmMsg,
    confirmText: m.common_word_status
      ? m.common_word_status()
      : m.ui_copy_confirm_0100f51c(),
    variant: nextState ? "default" : "destructive",
  });
  if (!confirmed) return;
  toggleStatusMutation.mutate(nextState);
}

const saveProfileMutation = createMutation(() => ({
  mutationFn: async () => {
    const avatarFile = userAvatarDrafts[userId];
    let avatarUrl: string | undefined;
    if (avatarFile) {
      const uploaded = await api.uploadFile({
        file: avatarFile,
        name: avatarFile.name,
        entity_type: "user",
        entity_id: userId,
        category: "avatar",
        asset_type: "image",
        visibility: "public",
      });
      avatarUrl = uploaded.url || uploaded.file_path;
    }

    const initialRole = user?.roles?.[0] || "staff";
    const roleChanged = Boolean(role) && role !== initialRole;

    await api.updateUser(userId, {
      full_name: fullName.trim() || undefined,
      full_name_ar: nameAr.trim() || undefined,
      phone: phone.trim() || undefined,
      national_id: nationalId.trim() || undefined,
      gender: gender || undefined,
      birth_date: birthDate || undefined,
      address: address.trim() || undefined,
      is_active: isActiveState,
      ...(roleChanged ? { roles: [role] } : {}),
      ...(avatarUrl ? { avatar_url: avatarUrl } : {}),
    });
    return avatarUrl;
  },
  onSuccess: async (avatarUrl) => {
    userAvatarDrafts[userId] = null;
    await invalidateUserQueries(queryClient, orgId, userId);
    if (avatarUrl) {
      await queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
    }
    toast.success(
      m.settings_user_detail_saved
        ? m.settings_user_detail_saved()
        : m.ui_copy_profile_saved_successfully_42b4e478(),
    );
  },
  onError: (e: unknown) => {
    toast.error(extractErrorMessage(e) || m.ui_copy_save_failed_38ef99fb());
  },
}));

const isSaving = $derived(
  saveProfileMutation.isPending || toggleStatusMutation.isPending,
);
</script>

{#if isLoading}
  <div class="flex flex-col gap-5 text-start">
    <Skeleton class="h-72 w-full rounded-md" />
    <Skeleton class="h-56 w-full rounded-md" />
    <Skeleton class="h-28 w-full rounded-md" />
  </div>
{:else}
  <div class="flex flex-col gap-5 text-start">
    <!-- Identity & Account -->
    <Card>
      <div class="border-b border-border bg-muted/20 px-4 py-3 sm:px-6">
        <h2 class="text-xs font-semibold text-muted-foreground">
          {m.settings_user_detail_tab_profile()}
        </h2>
      </div>

      <SettingsRow
        title={m.ui_copy_enabled_aceb3f59()}
        labelFor="user_enabled"
      >
        <Switch
          id="user_enabled"
          checked={isActiveState}
          disabled={isSaving}
          onCheckedChange={toggleAccountStatus}
        />
      </SettingsRow>

      <SettingsRow title={m.common_word_fullname()} required>
        <div class="w-full">
          <BilingualNameFields
            arId="nameAr"
            enId="fullName"
            arLabel={m.ui_copy_full_name_arabic_4871e6bf()}
            enLabel={m.common_word_fullname()}
            bind:arValue={nameAr}
            bind:enValue={fullName}
            kind="person-name"
            requestSuggestion={suggestLocalTranslation}
            enRequired
            disabled={isSaving}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />
        </div>
      </SettingsRow>

      <SettingsRow title={m.common_word_email()} labelFor="email">
        <CopyableInput
          id="email"
          label={m.common_word_email()}
          value={email}
        />
      </SettingsRow>

      <SettingsRow title={m.ui_copy_primary_role_b4890578()} labelFor="role">
        <div class="w-full">
          <RoleSelect bind:value={role} />
        </div>
      </SettingsRow>

      <SettingsRow title={m.employees_phone()} labelFor="phone">
        <Input
          id="phone"
          type="tel"
          bind:value={phone}
          disabled={isSaving}
          class="h-10 w-full"
        />
      </SettingsRow>

      <SettingsRow title="ID" labelFor="userId" last>
        <CopyableInput id="userId" label="ID" value={userId} />
      </SettingsRow>
    </Card>

    <!-- Demographic Details -->
    <Card>
      <div class="border-b border-border bg-muted/20 px-4 py-3 sm:px-6">
        <h2 class="text-xs font-semibold text-muted-foreground">
          {m.employees_national_id()} &amp; {m.employees_address()}
        </h2>
      </div>

      <SettingsRow title={m.employees_national_id()} labelFor="nationalId">
        <Input
          id="nationalId"
          type="text"
          bind:value={nationalId}
          disabled={isSaving}
          class="h-10 w-full"
        />
      </SettingsRow>

      <SettingsRow title={m.employees_gender()}>
        <Select
          type="single"
          value={gender}
          disabled={isSaving}
          onValueChange={(v) => (gender = v ?? "")}
        >
          <SelectTrigger class="h-10 w-full">
            {#if gender === "male"}
              {m.employees_gender_male()}
            {:else if gender === "female"}
              {m.employees_gender_female()}
            {:else}
              —
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="male">{m.employees_gender_male()}</SelectItem>
            <SelectItem value="female">{m.employees_gender_female()}</SelectItem>
          </SelectContent>
        </Select>
      </SettingsRow>

      <SettingsRow title={m.employees_birth_date()} labelFor="birthDate">
        <Input
          id="birthDate"
          type="date"
          bind:value={birthDate}
          disabled={isSaving}
          class="h-10 w-full"
        />
      </SettingsRow>

      <SettingsRow title={m.employees_address()} labelFor="address" last>
        <Input
          id="address"
          type="text"
          bind:value={address}
          disabled={isSaving}
          class="h-10 w-full"
        />
      </SettingsRow>
    </Card>

    <!-- Assigned User Groups -->
    <Card>
      <div class="flex items-center justify-between border-b border-border bg-muted/20 px-4 py-3 sm:px-6">
        <div class="flex items-center gap-2">
          <UserGroup size={15} class="text-primary" />
          <h2 class="text-xs font-semibold text-muted-foreground">
            {m.employees_user_groups()}
          </h2>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="text-xs h-7 gap-1.5 cursor-pointer"
          onclick={() => (groupsSheetOpen = true)}
        >
          <Pencil size={12} />
          <span>{m.ui_copy_manage_groups_bb35a396()}</span>
        </Button>
      </div>

      <div class="px-4 py-4 sm:px-6">
        {#if groupsQuery.isLoading}
          <div class="flex gap-2">
            <Skeleton class="h-6 w-24 rounded-full" />
            <Skeleton class="h-6 w-28 rounded-full" />
          </div>
        {:else if userAssignedGroups.length === 0}
          <span class="text-xs text-muted-foreground">
            {m.ui_copy_no_groups_currently_assigned_to_this_user_2a949232()}
          </span>
        {:else}
          <div class="flex flex-wrap gap-2">
            {#each userAssignedGroups as grp (grp.id)}
              <Badge
                variant="secondary"
                class="text-xs py-1 px-2.5 font-medium flex items-center gap-1.5"
              >
                <UserGroup size={12} />
                <span>{l(grp.name_ar, grp.name) || grp.name}</span>
              </Badge>
            {/each}
          </div>
        {/if}
      </div>
    </Card>
  </div>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={() => saveProfileMutation.mutate()}
    onreset={resetForm}
  />

  <ManageUserGroupsSheet
    bind:open={groupsSheetOpen}
    {orgId}
    {userId}
    userName={user?.full_name || fullName}
    assignedGroupIds={userAssignedGroupIds}
    onsaved={async () => {
      await Promise.all([
        queryClient.invalidateQueries({
          queryKey: ["user_assigned_groups", orgId, userId],
        }),
        groupsQuery.refetch(),
      ]);
    }}
  />
{/if}
