<!--
  @component GroupDetailView
  Group detail inspection, member management, and 2D IAM Permission Matrix editor.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
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
import { IAM_DOMAINS, type IamDomainDef, type IamResourceDef } from "$lib/core/iam";
import BulkAssignDialog from "./bulk-assign-dialog.svelte";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import ConfirmDialog from "$lib/shared/components/confirm-dialog.svelte";
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
        (res.labelAr && res.labelAr.toLowerCase().includes(q)) ||
        (res.labelEn && res.labelEn.toLowerCase().includes(q)) ||
        res.key.toLowerCase().includes(q);

      if (q && !resMatchesSearch) {
        filteredActions = filteredActions.filter(
          (a) =>
            a.key.toLowerCase().includes(q) ||
            (a.labelAr && a.labelAr.toLowerCase().includes(q)) ||
            (a.labelEn && a.labelEn.toLowerCase().includes(q))
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
      (domain.nameAr && domain.nameAr.toLowerCase().includes(q)) ||
      (domain.nameEn && domain.nameEn.toLowerCase().includes(q)) ||
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
    toast.success(m.groups_permissions_saved?.() || "تم تحديث صلاحيات المجموعة بنجاح");
    isPermissionsDirty = false;
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

function promptRemoveMember(member: GroupMemberInfoDto) {
  targetMember = member;
  removeDialogOpen = true;
}

async function confirmRemoveMember() {
  if (!targetMember) return;
  try {
    await removeMemberMutation.mutateAsync({
      groupId,
      userId: targetMember.user_id,
    });
    toast.success(l("تم استبعاد العضو من المجموعة بنجاح", "Member removed from group successfully"));
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    removeDialogOpen = false;
    targetMember = null;
  }
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  {#if groupQuery.isLoading}
    <div class="h-full w-full flex flex-col items-center justify-center gap-3 text-muted-foreground">
      <ArrowPath size={24} class="animate-spin text-primary" />
      <span class="text-xs">{l("جاري تحميل بيانات المجموعة...", "Loading group...")}</span>
    </div>
  {:else if groupQuery.error || !group}
    <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
      <p class="text-xs text-destructive font-semibold">{extractErrorMessage(groupQuery.error) || "Group not found"}</p>
      <Button variant="outline" size="sm" class="text-xs" onclick={() => goto(localizeHref(`/${orgId}/settings/groups`))}>
        {l("العودة لقائمة المجموعات", "Back to Groups")}
      </Button>
    </div>
  {:else}
    <!-- Header Bar -->
    <div class="border-b border-border bg-card px-6 py-4 flex items-center justify-between shrink-0 shadow-2xs">
      <div class="flex items-center gap-3">
        <Button
          variant="ghost"
          size="icon"
          class="size-8 cursor-pointer"
          onclick={() => goto(localizeHref(`/${orgId}/settings/groups`))}
        >
          <ArrowLeft size={16} />
        </Button>

        <div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
          <UserGroup size={20} />
        </div>

        <div class="flex flex-col">
          <div class="flex items-center gap-2">
            <h1 class="text-base font-bold text-foreground truncate">
              {group.name_ar || group.name}
            </h1>
            {#if group.is_system}
              <Badge variant="secondary" class="text-[10px]">
                {m.groups_system_badge?.() || "نظام"}
              </Badge>
            {/if}
          </div>
          <span class="text-xs text-muted-foreground">
            {group.description || l("إدارة أعضاء وصلاحيات المجموعة", "Manage members and permissions")}
          </span>
        </div>
      </div>

      <!-- Navigation Tabs -->
      <div class="flex items-center gap-1 bg-muted/60 p-1 rounded-lg border border-border/60">
        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'members' ? 'bg-card text-foreground shadow-2xs font-semibold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "members")}
        >
          <Users size={13} />
          <span>{m.groups_tab_members?.({ count: formatNumber(members.length) }) || `الأعضاء (${members.length})`}</span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 {activeTab === 'permissions' ? 'bg-card text-foreground shadow-2xs font-semibold' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (activeTab = "permissions")}
        >
          <Keyframes size={13} />
          <span>{m.groups_tab_permissions?.() || "مصفوفة الصلاحيات"}</span>
          {#if isPermissionsDirty}
            <span class="size-1.5 rounded-full bg-amber-500"></span>
          {/if}
        </button>
      </div>
    </div>

    <!-- Content Area -->
    <div class="flex-1 min-h-0 overflow-y-auto p-6">
      <div class="max-w-5xl mx-auto flex flex-col gap-6 pb-20">

        <!-- TAB 1: MEMBERS MANAGEMENT -->
        {#if activeTab === "members"}
          <div class="flex flex-col gap-4 bg-card border border-border rounded-xl p-5 shadow-xs">
            <div class="flex items-center justify-between border-b border-border/60 pb-3">
              <div class="flex items-center gap-2">
                <Users size={16} class="text-primary" />
                <h2 class="text-xs font-bold text-foreground uppercase tracking-wider">{l("قائمة المستخدمين الأعضاء في المجموعة", "Group Members")}</h2>
              </div>

              <Button
                variant="default"
                size="sm"
                class="text-xs cursor-pointer gap-1.5"
                onclick={() => (bulkAssignOpen = true)}
              >
                <Plus size={13} />
                <span>{m.groups_bulk_assign?.() || "إسناد مستخدمين للمجموعة"}</span>
              </Button>
            </div>

            {#if membersQuery.isLoading}
              <div class="p-8 text-center text-xs text-muted-foreground flex items-center justify-center gap-2">
                <ArrowPath size={14} class="animate-spin text-primary" />
                <span>{l("جاري تحميل الأعضاء...", "Loading members...")}</span>
              </div>
            {:else if members.length === 0}
              <div class="p-8 text-center text-xs text-muted-foreground flex flex-col items-center justify-center gap-2">
                <UserGroup size={24} class="text-muted-foreground/60" />
                <span>{l("لا يوجد أعضاء في هذه المجموعة حالياً.", "No members in this group.")}</span>
                <Button variant="outline" size="sm" class="text-xs mt-2" onclick={() => (bulkAssignOpen = true)}>
                  {m.groups_bulk_assign?.() || "إسناد أعضاء الآن"}
                </Button>
              </div>
            {:else}
              <div class="divide-y divide-border/40 border border-border/60 rounded-lg overflow-hidden">
                {#each members as member (member.user_id)}
                  <div class="flex items-center justify-between p-3 bg-card hover:bg-muted/20 transition-colors">
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
                          <span>• {l("انضم", "Joined")}: {formatDate(member.joined_at, "date")}</span>
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
                      <span>{m.groups_remove_member?.() || "استبعاد"}</span>
                    </Button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
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
                      {l("مصفوفة صلاحيات المجموعة", "Group Permissions Matrix")}
                    </h2>
                    <Badge variant="default" class="font-mono text-xs">
                      {totalGrantedPerms} {l("صلاحية نشطة", "active perms")}
                    </Badge>
                  </div>
                  <p class="text-[11px] text-muted-foreground pt-0.5">
                    {l("تمنح هذه الصلاحيات تلقائياً لكافة أعضاء المجموعة عبر منظومة الصلاحيات الفعالة (Effective Permissions).", "Granted to all group members via Effective Permissions.")}
                  </p>
                </div>
              </div>

              <!-- Matrix Summary Stats -->
              <div class="flex items-center gap-2 bg-card p-1.5 rounded-lg border border-border/50 text-xs shrink-0">
                <div class="flex flex-col items-center px-3 py-1">
                  <span class="text-[10px] text-muted-foreground uppercase">{l("المجالات المغطاة", "Domains")}</span>
                  <span class="font-mono font-bold text-foreground">{activeDomainsCount} / {IAM_DOMAINS.length}</span>
                </div>
                <div class="h-6 w-px bg-border/60"></div>
                <div class="flex flex-col items-center px-3 py-1">
                  <span class="text-[10px] text-muted-foreground uppercase">{l("نسبة التغطية", "Coverage")}</span>
                  <span class="font-mono font-bold text-primary">{totalSystemPerms > 0 ? Math.round((totalGrantedPerms / totalSystemPerms) * 100) : 0}%</span>
                </div>
              </div>
            </div>

            <!-- Matrix Toolbar: Search + Scope Filter Pills + Domain Tabs -->
            <div class="flex flex-col gap-3">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <!-- Search Input -->
                <div class="relative flex-1 max-w-sm">
                  <Search size={14} class="absolute start-2.5 top-2.5 text-muted-foreground pointer-events-none" />
                  <Input
                    type="search"
                    placeholder={l("بحث في الصلاحيات، المجالات، والعمليات...", "Search permissions, domains...")}
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
                    {l("الكل", "All")} ({totalSystemPerms})
                  </button>
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {permFilterScope === 'granted' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
                    onclick={() => (permFilterScope = "granted")}
                  >
                    {l("الممنوحة فقط", "Granted")} ({totalGrantedPerms})
                  </button>
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {permFilterScope === 'ungranted' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
                    onclick={() => (permFilterScope = "ungranted")}
                  >
                    {l("غير الممنوحة", "Ungranted")} ({totalSystemPerms - totalGrantedPerms})
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
                  {l("كافة المجالات", "All Domains")}
                </button>
                {#each domainMatrixData as d (d.id)}
                  <button
                    type="button"
                    class="px-2.5 py-1 rounded-md transition-all cursor-pointer whitespace-nowrap text-xs border flex items-center gap-1.5 {selectedDomainId === d.id ? 'bg-primary/10 border-primary text-primary font-bold' : 'border-border/60 bg-muted/20 text-muted-foreground hover:text-foreground'}"
                    onclick={() => (selectedDomainId = d.id)}
                  >
                    <span>{d.nameAr || d.nameEn}</span>
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
                <span class="text-xs font-semibold text-foreground">{l("لا توجد صلاحيات مطابقة للبحث أو الفلتر", "No matching permissions found")}</span>
                <span class="text-[11px] text-muted-foreground">{l("جرب تغيير عبارة البحث أو اختيار نطاق آخر.", "Try adjusting search terms or filters.")}</span>
              </div>
            {:else}
              <div class="flex flex-col gap-4">
                {#each visibleDomains as domain (domain.id)}
                  <div class="flex flex-col gap-3 rounded-xl border border-border/80 bg-card p-5 shadow-xs">
                    <!-- Domain Header -->
                    <div class="flex items-center justify-between border-b border-border/60 pb-3">
                      <div class="flex items-center gap-2.5">
                        <div class="size-7 rounded-md bg-primary/10 flex items-center justify-center text-primary">
                          <Keyframes size={14} />
                        </div>
                        <div class="flex flex-col">
                          <div class="flex items-center gap-2">
                            <h3 class="text-xs font-bold text-foreground">{domain.nameAr || domain.nameEn}</h3>
                            <span class="text-[10px] font-mono text-muted-foreground">({domain.id})</span>
                          </div>
                          <span class="text-[10px] text-muted-foreground">{domain.descAr || domain.descEn}</span>
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
                          {domain.grantedCount} / {domain.totalCount} {l("مفعلة", "active")}
                        </Badge>

                        <div class="flex items-center gap-2 ps-2 border-s border-border/60">
                          <button
                            type="button"
                            class="text-[11px] text-primary hover:underline cursor-pointer"
                            onclick={() => selectAllInDomain(domain)}
                          >
                            {l("تحديد الكل", "Select all")}
                          </button>
                          <span class="text-muted-foreground text-[10px]">•</span>
                          <button
                            type="button"
                            class="text-[11px] text-muted-foreground hover:underline cursor-pointer"
                            onclick={() => clearAllInDomain(domain)}
                          >
                            {l("إلغاء التحديد", "Clear all")}
                          </button>
                        </div>
                      </div>
                    </div>

                    <!-- Resources & 2D Action Matrix -->
                    <div class="flex flex-col gap-3 pt-1">
                      {#each domain.filteredResources as res (res.key)}
                        <div class="flex flex-col gap-2 p-3 rounded-lg bg-muted/20 border border-border/40">
                          <!-- Resource Title & Key -->
                          <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                              <span class="text-xs font-semibold text-foreground">{res.labelAr || res.labelEn}</span>
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
                                title="{res.key}:{act.key} - {act.descAr || act.descEn || ''}"
                              >
                                <div class="flex size-4 items-center justify-center rounded-xs {act.isGranted ? 'bg-emerald-600 text-white' : 'border border-border'}">
                                  {#if act.isGranted}
                                    <CheckCircle size={11} />
                                  {/if}
                                </div>
                                <span>{act.labelAr || act.labelEn}</span>
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
        saveLabel={m.settings_action_save_changes?.() || "حفظ الصلاحيات"}
        onsave={handleSavePermissions}
        onreset={() => {
          if (group) {
            selectedPermissions = new Set(group.permissions);
            isPermissionsDirty = false;
          }
        }}
        resetLabel={m.settings_action_discard_changes?.() || "تراجع"}
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

  <!-- Remove Member Confirm Dialog -->
  <ConfirmDialog
    bind:open={removeDialogOpen}
    title={l("تأكيد استبعاد العضو", "Remove Member")}
    description={m.groups_remove_confirm?.() || "هل أنت متأكد من استبعاد هذا المستخدم من المجموعة؟"}
    confirmText={m.groups_remove_member?.() || "استبعاد"}
    variant="destructive"
    onConfirm={confirmRemoveMember}
  />
</div>
