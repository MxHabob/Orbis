<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  Plus,
  Search,
  Pencil,
  Trash2,
  Shield,
  User as UserIcon,
  Users,
  Eye,
} from "lucide-svelte";
import UserCreateSheet from "$lib/components/sheets/user-create-sheet.svelte";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import * as m from "$lib/paraglide/messages";
import type { User } from "$lib/services/ipc";
import { authStore } from "$lib/stores";

let { data }: { data: { orgId: string } } = $props();

// State
let users = $state<User[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters
let searchQuery = $state("");
let selectedRole = $state("all");

// Sheet controls
let isSheetOpen = $state(false);
let editingUser = $state<User | null>(null);

async function loadUsers() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.users.getUsers(data.orgId);
    users = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error("Failed to load users");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadUsers();
  }
});

// Filter users list client-side for immediate responsive experience
const filteredUsers = $derived(
  users.filter((user) => {
    const matchesSearch =
      (user.full_name || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      (user.name_ar || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (user.email || "").toLowerCase().includes(searchQuery.toLowerCase());

    const matchesRole =
      selectedRole === "all" ||
      String(user.role || "").toLowerCase() === selectedRole.toLowerCase();

    return matchesSearch && matchesRole;
  }),
);

function openCreateSheet() {
  editingUser = null;
  isSheetOpen = true;
}

function openEditSheet(user: User) {
  editingUser = user;
  isSheetOpen = true;
}

async function handleDeleteUser(user: User) {
  const confirmMsg = m.settings_users_delete_confirm();
  if (!confirm(confirmMsg)) return;

  try {
    const deletedBy = authStore.userId || "unknown";
    await commands.users.deleteUser(user.id, deletedBy);
    toast.success(m.settings_users_delete_success());
    loadUsers();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getRoleLabel(roleStr: string): string {
  switch (String(roleStr).toLowerCase()) {
    case "admin":
    case "super_admin":
    case "principal":
      return m.settings_users_filter_admin();
    case "teacher":
      return m.settings_users_filter_teacher();
    case "parent":
    case "guardian":
      return m.settings_users_filter_guardian();
    case "student":
      return m.settings_users_filter_student();
    default:
      return roleStr || "Staff";
  }
}

function getRoleBadgeClass(roleStr: string): string {
  switch (String(roleStr).toLowerCase()) {
    case "admin":
    case "super_admin":
    case "principal":
      return "bg-rose-500/10 text-rose-500 border-rose-500/20";
    case "teacher":
      return "bg-blue-500/10 text-blue-500 border-blue-500/20";
    case "parent":
    case "guardian":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "student":
      return "bg-purple-500/10 text-purple-500 border-purple-500/20";
    default:
      return "bg-slate-500/10 text-slate-500 border-slate-500/20";
  }
}
</script>

<svelte:head>
  <title>{m.settings_users_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <span>{m.settings_users_title()}</span>
        {#if !isLoading}
          <span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {filteredUsers.length}
          </span>
        {/if}
      </h1>
      <p class="text-xs text-muted-foreground">{m.settings_users_desc()}</p>
    </div>

    <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      {m.settings_users_add()}
    </Button>
  </div>

  <!-- Search and Filtering Bar -->
  <div class="flex flex-col sm:flex-row gap-3 items-center w-full">
    <!-- Search Field -->
    <div class="relative flex-1 w-full">
      <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
      <Input
        type="text"
        bind:value={searchQuery}
        placeholder="Search by name or email..."
        class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs"
      />
    </div>

    <!-- Filter by Role -->
    <div class="w-full sm:w-48 text-start">
      <Select type="single" value={selectedRole} onValueChange={(v) => (selectedRole = v ?? "all")}>
        <SelectTrigger class="h-9 w-full text-xs">
          {#if selectedRole === "all"}
            {m.settings_users_filter_all()}
          {:else if selectedRole === "admin"}
            {m.settings_users_filter_admin()}
          {:else if selectedRole === "teacher"}
            {m.settings_users_filter_teacher()}
          {:else if selectedRole === "parent"}
            {m.settings_users_filter_guardian()}
          {:else if selectedRole === "student"}
            {m.settings_users_filter_student()}
          {:else}
            Staff
          {/if}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">{m.settings_users_filter_all()}</SelectItem>
          <SelectItem value="admin">{m.settings_users_filter_admin()}</SelectItem>
          <SelectItem value="teacher">{m.settings_users_filter_teacher()}</SelectItem>
          <SelectItem value="parent">{m.settings_users_filter_guardian()}</SelectItem>
          <SelectItem value="student">{m.settings_users_filter_student()}</SelectItem>
          <SelectItem value="staff">Staff</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>

  <!-- Main Grid/Table Content -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
      <p class="text-xs text-destructive font-semibold">Failed to load users</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadUsers}>{m.error_retry()}</Button>
    </div>
  {:else if filteredUsers.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <Users size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">{m.settings_users_empty()}</p>
        <p class="text-[10px] text-muted-foreground/80">{m.settings_users_empty_desc()}</p>
      </div>
    </div>
  {:else}
    <div class="border border-border rounded-xl bg-card/20 overflow-hidden" in:fade={{ duration: 150 }}>
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-transparent">
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.settings_users_col_name()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.settings_users_col_email()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.settings_users_col_role()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.settings_users_col_phone()}</TableHead>
              <TableHead class="text-end text-xs font-bold text-muted-foreground px-6">{m.settings_users_col_actions()}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredUsers as user (user.id)}
              <TableRow class="hover:bg-accent/5">
                <!-- Name with Avatar -->
                <TableCell>
                  <a
                    href={localizeHref(`/${data.orgId}/settings/users/${user.id}`)}
                    class="flex items-center gap-3 hover:opacity-80 transition-opacity"
                  >
                    <Avatar class="w-8 h-8 border border-border/80">
                      <AvatarImage src={user.avatar ? String(user.avatar) : undefined} alt={user.full_name} class="object-cover" />
                      <AvatarFallback class="text-xs font-bold bg-muted text-muted-foreground">
                        {(user.full_name || "U").charAt(0).toUpperCase()}
                      </AvatarFallback>
                    </Avatar>
                    <div class="flex flex-col text-start">
                      <span class="text-xs font-bold text-foreground">
                        {getLocale() === "ar" && user.name_ar ? user.name_ar : user.full_name}
                      </span>
                      {#if getLocale() === "ar" && user.name_ar && user.full_name}
                        <span class="text-[9px] text-muted-foreground leading-none mt-0.5">{user.full_name}</span>
                      {/if}
                    </div>
                  </a>
                </TableCell>
                
                <!-- Email -->
                <TableCell class="text-xs text-muted-foreground font-mono">
                  {user.email || "—"}
                </TableCell>
                
                <!-- Role Badge -->
                <TableCell>
                  <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase {getRoleBadgeClass(String(user.role))}">
                    {getRoleLabel(String(user.role))}
                  </span>
                </TableCell>
                
                <!-- Phone -->
                <TableCell class="text-xs text-muted-foreground font-mono">
                  {user.phone || "—"}
                </TableCell>
                
                <!-- Actions -->
                <TableCell class="text-end px-6">
                  <div class="flex items-center justify-end gap-1.5">
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title={getLocale() === "ar" ? "تفاصيل المستخدم" : "User Details"}
                      href={localizeHref(`/${data.orgId}/settings/users/${user.id}`)}
                    >
                      <Eye size={12} class="text-muted-foreground hover:text-foreground" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title="Edit"
                      onclick={() => openEditSheet(user)}
                    >
                      <Pencil size={12} class="text-muted-foreground hover:text-foreground" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title="Delete"
                      disabled={user.id === authStore.userId}
                      onclick={() => handleDeleteUser(user)}
                    >
                      <Trash2 size={12} class="text-muted-foreground hover:text-destructive" />
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    </div>
  {/if}
</div>

<!-- Modal Sheets -->
<UserCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  user={editingUser}
  onSuccess={loadUsers}
/>
