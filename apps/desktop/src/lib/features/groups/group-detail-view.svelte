<!--
  @component GroupDetailView
  Group detail inspection, member management, and 2D IAM Permission Matrix editor.
-->
<script lang="ts">
  import { Skeleton } from "@orbis/ui/skeleton";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import {
  ArrowLeft,
  ArrowPath,
  CheckCircle,
  Keyframes,
  LockClosed,
  Pencil,
  Plus,
  Search,
  ShieldCheck,
  Trash,
  User,
  UserGroup,
  Users,
  XMark,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import type { GroupDetailDto, GroupMemberInfoDto } from "$lib/api/types";
import {
  createGroupQuery,
  createGroupMembersQuery,
  createRemoveGroupMemberMutation,
  createSetGroupPermissionsMutation,
} from "./queries";
import {
  IAM_DOMAINS,
  translateIamMessage,
  type IamDomainDef,
  type IamResourceDef,
} from "$lib/core/iam";
import BulkAssignDialog from "./bulk-assign-dialog.svelte";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import { confirmStore } from "$lib/stores";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

interface Props {
  orgId: string;
  groupId: string;
}

let { orgId, groupId }: Props = $props();

let activeTab = $state<"members" | "permissions">("members");

// Queries
const groupQuery = createGroupQuery(() => groupId);
const membersQuery = createGroupMembersQuery(() => groupId);

// Mutations
const removeMemberMutation = createRemoveGroupMemberMutation();
const setPermissionsMutation = createSetGroupPermissionsMutation();

const group = $derived(groupQuery.data);
const members = $derived(membersQuery.data ?? []);

// Permissions Matrix State
let selectedPermissions = $state<Set<string>>(new Set());
let isPermissionsDirty = $state(false);
let isInitialized = $state(false);

// Matrix Filter State
let permSearch = $state("");
let permFilterScope = $state<"all" | "granted" | "ungranted">("all");
let selectedDomainId = $state<string>("all");

// Initialize selected permissions when groupQuery resolves
$effect(() => {
  if (group && !isInitialized) {
    selectedPermissions = new Set(group.permissions);
    isInitialized = true;
  }
});

// Bulk assign dialog state
let bulkAssignOpen = $state(false);

// Remove member state
let removeDialogOpen = $state(false);
let targetMember = $state<GroupMemberInfoDto | null>(null);

function togglePermission(perm: string) {
  const next = new Set(selectedPermissions);
  if (next.has(perm)) {
    next.delete(perm);
  } else {
    next.add(perm);
  }
  selectedPermissions = next;
  isPermissionsDirty = true;
}

function selectAllInDomain(domain: IamDomainDef) {
  const next = new Set(selectedPermissions);
  for (const res of domain.resources) {
    for (const act of res.actions) {
      next.add(`${res.key}:${act.key}`);
    }
  }
  selectedPermissions = next;
  isPermissionsDirty = true;
}

function clearAllInDomain(domain: IamDomainDef) {
  const next = new Set(selectedPermissions);
  for (const res of domain.resources) {
    for (const act of res.actions) {
      next.delete(`${res.key}:${act.key}`);
    }
  }
  selectedPermissions = next;
  isPermissionsDirty = true;
}

// 2D Domain Matrix Calculations for Groups
const domainMatrixData = $derived.by(() => {
  const q = permSearch.trim().toLowerCase();

  return IAM_DOMAINS.map((domain) => {
    let domainTotal = 0;
    let domainGranted = 0;

    const resources = domain.resources.map((res) => {
      const actions = res.actions.map((act) => {
        const permKey = `${res.key}:${act.key}`;
        const isGranted = selectedPermissions.has(permKey);
        if (isGranted) domainGranted++;
        domainTotal++;
        return {
          ...act,
          permKey,
          isGranted,
        };
      });

      let filteredActions = actions;
      if (permFilterScope === "granted") {
        filteredActions = actions.filter((a) => a.isGranted);
      } else if (permFilterScope === "ungranted") {
        filteredActions = actions.filter((a) => !a.isGranted);
      }

      const resMatchesSearch =
        !q ||
        translateIamMessage(res.labelKey, "ar").toLowerCase().includes(q) ||
        translateIamMessage(res.labelKey, "en").toLowerCase().includes(q) ||
        res.key.toLowerCase().includes(q);

      if (q && !resMatchesSearch) {
        filteredActions = filteredActions.filter(
          (a) =>
            a.key.toLowerCase().includes(q) ||
            translateIamMessage(a.labelKey, "ar").toLowerCase().includes(q) ||
            translateIamMessage(a.labelKey, "en").toLowerCase().includes(q)
        );
      }

      const resGrantedCount = actions.filter((a) => a.isGranted).length;

      return {
        ...res,
        actions,
        filteredActions,
        grantedCount: resGrantedCount,
        totalCount: actions.length,
        isMatch: resMatchesSearch || filteredActions.length > 0,
      };
    });

    const isDomainMatch =
      !q ||
      translateIamMessage(domain.nameKey, "ar").toLowerCase().includes(q) ||
      translateIamMessage(domain.nameKey, "en").toLowerCase().includes(q) ||
      resources.some((r) => r.isMatch);

    const filteredResources = resources.filter((r) => (q ? r.isMatch : r.filteredActions.length > 0));

    return {
      ...domain,
      totalCount: domainTotal,
      grantedCount: domainGranted,
      percentage: domainTotal > 0 ? Math.round((domainGranted / domainTotal) * 100) : 0,
      resources,
      filteredResources,
      isMatch: isDomainMatch && filteredResources.length > 0,
    };
  });
});

const totalSystemPerms = $derived(
  domainMatrixData.reduce((acc, d) => acc + d.totalCount, 0)
);
const totalGrantedPerms = $derived(
  domainMatrixData.reduce((acc, d) => acc + d.grantedCount, 0)
);
const activeDomainsCount = $derived(
  domainMatrixData.filter((d) => d.grantedCount > 0).length
);

const visibleDomains = $derived.by(() => {
  let list = domainMatrixData;
  if (selectedDomainId !== "all") {
    list = list.filter((d) => d.id === selectedDomainId);
  }
  if (permSearch.trim()) {
    list = list.filter((d) => d.isMatch);
  } else if (permFilterScope !== "all") {
    list = list.filter((d) => d.filteredResources.length > 0);
  }
  return list;
});

async function handleSavePermissions() {
  if (!group) return;
  try {
    await setPermissionsMutation.mutateAsync({
      id: group.id,
      permissions: Array.from(selectedPermissions),
    });
    toast.success(m.groups_permissions_saved());
    isPermissionsDirty = false;
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

async function promptRemoveMember(member: GroupMemberInfoDto) {

  const confirmed = await confirmStore.prompt({
    title: m.ui_copy_remove_member_a32249bd(),
    description: l(
      `هل أنت متأكد من رغبتك في استبعاد "${member.full_name}" من هذه المجموعة؟`,
      `Are you sure you want to remove "${member.full_name}" from this group?`
    ),
    confirmText: m.groups_remove_member(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await removeMemberMutation.mutateAsync({
      groupId,
      userId: member.user_id,
    });
    toast.success(m.ui_copy_member_removed_from_group_successfully_d752bc90());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  {#if groupQuery.isLoading}
    <div class="flex h-full flex-col gap-6 p-4" aria-busy="true">
      <span class="sr-only">{m.ui_copy_loading_group_e09b26dc()}</span>
      <div class="flex items-center gap-4 border-b border-border pb-4">
        <Skeleton class="h-9 w-9 rounded-md" />
        <Skeleton class="h-10 w-10 rounded-lg" />
        <div class="flex flex-1 flex-col gap-2">
          <Skeleton class="h-5 w-48 max-w-full" />
          <Skeleton class="h-3 w-72 max-w-full" />
        </div>
        <Skeleton class="h-8 w-24" />
      </div>
      <div class="flex gap-2 border-b border-border pb-2">
        <Skeleton class="h-8 w-24" />
        <Skeleton class="h-8 w-24" />
      </div>
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div class="rounded-lg border border-border p-4">
          <Skeleton class="mb-4 h-5 w-36" />
          <div class="flex flex-col gap-3">
            <Skeleton class="h-10 w-full" />
            <Skeleton class="h-10 w-full" />
            <Skeleton class="h-10 w-4/5" />
          </div>
        </div>
        <div class="rounded-lg border border-border p-4">
          <Skeleton class="mb-4 h-5 w-28" />
          <div class="flex flex-col gap-3">
            <Skeleton class="h-10 w-full" />
            <Skeleton class="h-10 w-full" />
            <Skeleton class="h-10 w-4/5" />
          </div>
        </div>
      </div>
    </div>
  {:else if groupQuery.error || !group}
    <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
      <p class="text-xs text-destructive font-semibold">{extractErrorMessage(groupQuery.error) || "Group not found"}</p>
      <Button variant="outline" size="sm" class="text-xs" onclick={() => goto(resolve(localizeHref(`/${orgId}/settings/groups`)))}>
        {m.ui_copy_back_to_groups_f1d6a091()}
      </Button>
    </div>
  {:else}
    <div class="shrink-0 border-b border-border bg-background px-6 pt-5">
      <div class="flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon-sm"
          class="shrink-0 text-muted-foreground"
          onclick={() => goto(resolve(localizeHref(`/${orgId}/settings/groups`)))}
          title={m.ui_copy_back_to_groups_f1d6a091()}
          aria-label={m.ui_copy_back_to_groups_f1d6a091()}
        >
          <ArrowLeft size={16} class="rtl:rotate-180" />
        </Button>
        <div class="min-w-0 flex-1">
          <div class="flex flex-wrap items-center gap-2">
            <h1 class="truncate text-lg font-bold tracking-tight text-foreground">
              {group.name_ar || group.name}
            </h1>
            {#if group.is_system}
              <Badge variant="secondary" class="text-[10px]">{m.groups_system_badge()}</Badge>
            {/if}
          </div>
          <p class="mt-0.5 truncate text-xs text-muted-foreground">
            {group.description || m.ui_copy_manage_members_and_permissions_18487d56()}
          </p>
        </div>
      </div>

      <div class="mt-4 flex items-center gap-5">
        <button
          type="button"
          class="relative flex items-center gap-1.5 border-b-2 px-1 pb-3 text-xs font-medium transition-colors cursor-pointer {activeTab === 'members' ? 'border-primary text-foreground font-semibold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "members")}
        >
          <Users size={13} />
          <span>{m.groups_tab_members({ count: formatNumber(members.length) })}</span>
        </button>

        <button
          type="button"
          class="relative flex items-center gap-1.5 border-b-2 px-1 pb-3 text-xs font-medium transition-colors cursor-pointer {activeTab === 'permissions' ? 'border-primary text-foreground font-semibold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "permissions")}
        >
          <Keyframes size={13} />
          <span>{m.groups_tab_permissions()}</span>
          {#if isPermissionsDirty}
            <span class="size-1.5 rounded-full bg-amber-500"></span>
          {/if}
        </button>
      </div>
    </div>

    <!-- Content Area -->
    <div class="flex-1 min-h-0 overflow-y-auto p-6">
      <div class="flex flex-col gap-5 pb-20">

        <!-- TAB 1: MEMBERS MANAGEMENT -->
        {#if activeTab === "members"}
          <Card class="flex flex-col">
            <CardHeader class="flex flex-row items-center justify-between gap-3 border-b border-border/50 bg-transparent p-4 pb-3 text-foreground normal-case tracking-normal">
              <div class="flex items-center gap-2">
                <Users size={16} class="text-primary" />
                <CardTitle class="text-sm font-semibold text-foreground">
                  {m.ui_copy_group_members_558b5bd6()}
                </CardTitle>
              </div>

              <Button
                variant="outline"
                size="sm"
                class="text-xs cursor-pointer gap-1.5"
                onclick={() => (bulkAssignOpen = true)}
              >
                <Plus size={13} />
                <span>{m.groups_bulk_assign()}</span>
              </Button>
            </CardHeader>

            <CardContent class="px-4 pb-4">
            {#if membersQuery.isLoading}
              <div class="divide-y divide-border/50 border-t border-border/50" aria-busy="true">
                <span class="sr-only">{m.ui_copy_loading_members_a59e1afa()}</span>
                {#each Array.from({ length: 5 }) as _, index (index)}
                  <div class="flex items-center justify-between gap-4 py-3">
                    <div class="flex min-w-0 flex-1 items-center gap-3">
                      <Skeleton class="h-9 w-9 shrink-0 rounded-full" />
                      <div class="flex min-w-0 flex-1 flex-col gap-2">
                        <Skeleton class="h-4 w-40 max-w-full" />
                        <Skeleton class="h-3 w-28 max-w-full" />
                      </div>
                    </div>
                    <Skeleton class="h-7 w-20 shrink-0" />
                  </div>
                {/each}
              </div>
            {:else if members.length === 0}
              <div class="p-8 text-center text-xs text-muted-foreground flex flex-col items-center justify-center gap-2">
                <UserGroup size={24} class="text-muted-foreground/60" />
                <span>{m.ui_copy_no_members_in_this_group_9ad47c68()}</span>
                <Button variant="outline" size="sm" class="text-xs mt-2" onclick={() => (bulkAssignOpen = true)}>
                  {m.groups_bulk_assign()}
                </Button>
              </div>
            {:else}
              <div class="divide-y divide-border/50 border-t border-border/50">
                {#each members as member (member.user_id)}
                  <div class="flex items-center justify-between gap-3 py-3 transition-colors hover:bg-muted/20">
                    <div class="flex items-center gap-3 min-w-0">
                      <UserAvatar
                        name={member.full_name || member.full_name_ar || member.email}
                        avatarUrl={member.avatar_url ?? undefined}
                        size="sm"
                      />
                      <div class="flex flex-col min-w-0">
                        <span class="text-xs font-semibold text-foreground truncate">
                          {member.full_name || member.full_name_ar || member.email}
                        </span>
                        <div class="flex items-center gap-3 text-[11px] font-mono text-muted-foreground">
                          {#if member.email}
                            <span>{member.email}</span>
                          {/if}
                          {#if member.phone}
                            <span>• {member.phone}</span>
                          {/if}
                          <span>• {m.ui_copy_joined_7a91794e()}: {formatDate(member.joined_at, "date")}</span>
                        </div>
                      </div>
                    </div>

                    <Button
                      variant="ghost"
                      size="sm"
                      class="text-xs text-destructive hover:text-destructive cursor-pointer gap-1"
                      onclick={() => promptRemoveMember(member)}
                    >
                      <XMark size={13} />
                      <span>{m.groups_remove_member()}</span>
                    </Button>
                  </div>
                {/each}
              </div>
            {/if}
            </CardContent>
          </Card>
        {/if}

        <!-- TAB 2: 2D PERMISSIONS MATRIX (Interactive Group Permissions Editor) -->
        {#if activeTab === "permissions"}
          <div class="flex flex-col gap-6">
            <!-- Matrix Header & Stats Banner -->
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 p-5 rounded-xl border border-primary/20 bg-primary/5 shadow-2xs">
              <div class="flex items-center gap-3">
                <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary shrink-0">
                  <ShieldCheck size={20} />
                </div>
                <div class="flex flex-col">
                  <div class="flex items-center gap-2">
                    <h2 class="text-sm font-bold text-foreground">
                      {m.ui_copy_group_permissions_matrix_c204dd4c()}
                    </h2>
                    <Badge variant="default" class="font-mono text-xs">
                      {totalGrantedPerms} {m.ui_copy_active_perms_750b7318()}
                    </Badge>
                  </div>
                  <p class="text-[11px] text-muted-foreground pt-0.5">
                    {m.ui_copy_granted_to_all_group_members_via_effective_6cabedbb()}
                  </p>
                </div>
              </div>

              <!-- Matrix Summary Stats -->
              <div class="flex items-center gap-2 text-xs shrink-0">
                <div class="flex flex-col items-center px-3 py-1">
                  <span class="text-[10px] text-muted-foreground uppercase">{m.ui_copy_domains_3ba7f409()}</span>
                  <span class="font-mono font-bold text-foreground">{activeDomainsCount} / {IAM_DOMAINS.length}</span>
                </div>
                <div class="h-6 w-px bg-border/60"></div>
                <div class="flex flex-col items-center px-3 py-1">
                  <span class="text-[10px] text-muted-foreground uppercase">{m.ui_copy_coverage_33d59781()}</span>
                  <span class="font-mono font-bold text-primary">{totalSystemPerms > 0 ? Math.round((totalGrantedPerms / totalSystemPerms) * 100) : 0}%</span>
                </div>
              </div>
            </div>

            <!-- Matrix Toolbar: Search + Scope Filter Pills + Domain Tabs -->
            <div class="flex flex-col gap-3">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <!-- Search Input -->
                <div class="relative flex-1 max-w-sm">
                  <Search size={14} class="absolute inset-s-2.5 top-2.5 text-muted-foreground pointer-events-none" />
                  <Input
                    type="search"
                    placeholder={m.ui_copy_search_permissions_domains_a694396c()}
                    class="ps-8 h-8 text-xs bg-card"
                    bind:value={permSearch}
                  />
                </div>

                <!-- Scope Filter Pills -->
                <div class="flex rounded-lg border border-border/60 p-0.5 bg-card text-xs shrink-0">
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {permFilterScope === 'all' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
                    onclick={() => (permFilterScope = "all")}
                  >
                    {m.common_filter_all()} ({totalSystemPerms})
                  </button>
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {permFilterScope === 'granted' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
                    onclick={() => (permFilterScope = "granted")}
                  >
                    {m.ui_copy_granted_5ca6208a()} ({totalGrantedPerms})
                  </button>
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {permFilterScope === 'ungranted' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
                    onclick={() => (permFilterScope = "ungranted")}
                  >
                    {m.ui_copy_ungranted_c882c857()} ({totalSystemPerms - totalGrantedPerms})
                  </button>
                </div>
              </div>

              <!-- Domain Selection Chips -->
              <div class="flex items-center gap-1.5 overflow-x-auto pb-1 scrollbar-none text-xs">
                <button
                  type="button"
                  class="px-2.5 py-1 rounded-md transition-all cursor-pointer whitespace-nowrap text-xs border {selectedDomainId === 'all' ? 'bg-primary/10 border-primary text-primary font-bold' : 'border-border/60 bg-muted/20 text-muted-foreground hover:text-foreground'}"
                  onclick={() => (selectedDomainId = "all")}
                >
                  {m.ui_copy_all_domains_bf738d98()}
                </button>
                {#each domainMatrixData as d (d.id)}
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md transition-all cursor-pointer whitespace-nowrap text-xs border flex items-center gap-1.5 {selectedDomainId === d.id ? 'bg-primary/10 border-primary text-primary font-bold' : 'border-border/60 bg-muted/20 text-muted-foreground hover:text-foreground'}"
                    onclick={() => (selectedDomainId = d.id)}
                  >
                    <span>{translateIamMessage(d.nameKey)}</span>
                    {#if d.grantedCount > 0}
                      <span class="font-mono text-[10px] px-1 py-0.2 rounded-xs {selectedDomainId === d.id ? 'bg-primary text-primary-foreground' : 'bg-primary/20 text-primary font-semibold'}">
                        {d.grantedCount}
                      </span>
                    {/if}
                  </button>
                {/each}
              </div>
            </div>

            <!-- Domain Matrix Cards -->
            {#if visibleDomains.length === 0}
              <div class="p-12 flex flex-col items-center justify-center text-center gap-2 text-muted-foreground border border-dashed border-border rounded-xl bg-card">
                <Search size={28} class="text-muted-foreground/40 mb-1" />
                <span class="text-xs font-semibold text-foreground">{m.ui_copy_no_matching_permissions_found_44c3e5b1()}</span>
                <span class="text-[11px] text-muted-foreground">{m.ui_copy_try_adjusting_search_terms_or_filters_247cf49a()}</span>
              </div>
            {:else}
              <div class="flex flex-col gap-4">
                {#each visibleDomains as domain (domain.id)}
                  <div class="flex flex-col gap-3 rounded-lg border border-border/70 bg-card p-4">
                    <!-- Domain Header -->
                    <div class="flex items-center justify-between border-b border-border/60 pb-3">
                      <div class="flex items-center gap-2.5">
                        <div class="size-7 rounded-md bg-primary/10 flex items-center justify-center text-primary">
                          <Keyframes size={14} />
                        </div>
                        <div class="flex flex-col">
                          <div class="flex items-center gap-2">
                            <h3 class="text-xs font-bold text-foreground">{translateIamMessage(domain.nameKey)}</h3>
                            <span class="text-[10px] font-mono text-muted-foreground">({domain.id})</span>
                          </div>
                          <span class="text-[10px] text-muted-foreground">{translateIamMessage(domain.descriptionKey)}</span>
                        </div>
                      </div>

                      <div class="flex items-center gap-3 shrink-0">
                        <div class="w-20 h-1.5 bg-muted rounded-full overflow-hidden hidden sm:block">
                          <div
                            class="h-full bg-primary rounded-full transition-all"
                            style="width: {domain.percentage}%"
                          ></div>
                        </div>

                        <Badge
                          variant={domain.grantedCount > 0 ? "default" : "outline"}
                          class="font-mono text-[10px] px-2 py-0.5"
                        >
                          {domain.grantedCount} / {domain.totalCount} {m.ui_copy_active_858559ba()}
                        </Badge>

                        <div class="flex items-center gap-2 ps-2 border-s border-border/60">
                          <button
                            type="button"
                            class="text-[11px] text-primary hover:underline cursor-pointer"
                            onclick={() => selectAllInDomain(domain)}
                          >
                            {m.ui_copy_select_all_99dda350()}
                          </button>
                          <span class="text-muted-foreground text-[10px]">•</span>
                          <button
                            type="button"
                            class="text-[11px] text-muted-foreground hover:underline cursor-pointer"
                            onclick={() => clearAllInDomain(domain)}
                          >
                            {m.ui_copy_clear_all_47145b72()}
                          </button>
                        </div>
                      </div>
                    </div>

                    <!-- Resources & 2D Action Matrix -->
                    <div class="flex flex-col gap-3 pt-1">
                      {#each domain.filteredResources as res (res.key)}
                        <div class="flex flex-col gap-2 border-t border-border/40 pt-3 first:border-t-0 first:pt-0">
                          <!-- Resource Title & Key -->
                          <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                              <span class="text-xs font-semibold text-foreground">{translateIamMessage(res.labelKey)}</span>
                              <span class="text-[10px] font-mono text-muted-foreground bg-card px-1.5 py-0.5 rounded-xs border border-border/40">
                                {res.key}
                              </span>
                            </div>
                            <span class="text-[10px] font-mono text-muted-foreground">
                              {res.grantedCount} / {res.totalCount}
                            </span>
                          </div>

                          <!-- 2D Actions Matrix Grid -->
                          <div class="flex flex-wrap gap-2 pt-0.5">
                            {#each res.filteredActions as act (act.key)}
                              <button
                                type="button"
                                class="flex items-center gap-1.5 px-3 py-1.5 rounded-md border text-xs transition-all cursor-pointer {act.isGranted
                                  ? 'border-emerald-500/40 bg-emerald-500/10 text-emerald-950 dark:text-emerald-200 font-semibold shadow-2xs'
                                  : 'border-border/60 bg-card text-muted-foreground hover:border-border'}"
                                onclick={() => togglePermission(act.permKey)}
                                title="{res.key}:{act.key} - {act.descriptionKey ? translateIamMessage(act.descriptionKey) : ''}"
                              >
                                <div class="flex size-4 items-center justify-center rounded-xs {act.isGranted ? 'bg-emerald-600 text-white' : 'border border-border'}">
                                  {#if act.isGranted}
                                    <CheckCircle size={11} />
                                  {/if}
                                </div>
                                <span>{translateIamMessage(act.labelKey)}</span>
                                <span class="text-[9px] font-mono opacity-70">({act.key})</span>
                              </button>
                            {/each}
                          </div>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/if}

      </div>
    </div>

    <!-- Permissions Save Bottom Bar -->
    {#if activeTab === "permissions"}
      <SettingsBottomBar
        visible={true}
        isSaving={setPermissionsMutation.isPending}
        isDirty={isPermissionsDirty}
        saveLabel={m.settings_action_save_changes()}
        onsave={handleSavePermissions}
        onreset={() => {
          if (group) {
            selectedPermissions = new Set(group.permissions);
            isPermissionsDirty = false;
          }
        }}
        resetLabel={m.settings_action_discard_changes()}
      />
    {/if}
  {/if}

  <!-- Bulk Assign Users Dialog -->
  <BulkAssignDialog
    bind:open={bulkAssignOpen}
    {orgId}
    groupId={groupId}
    groupName={group?.name_ar || group?.name || ""}
    existingMemberUserIds={members.map((m) => m.user_id)}
    oncomplete={() => membersQuery.refetch()}
  />
</div>
