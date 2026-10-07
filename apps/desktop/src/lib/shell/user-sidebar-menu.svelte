<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
  DropdownMenuPortal,
} from "@orbis/ui/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@orbis/ui";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { authStore, effectiveContext } from "$lib/stores";
import { api } from "$lib/api/client";
import { l } from "$lib/core/i18n/locale";
import { onMount } from "svelte";

let {
  orgId,
}: {
  orgId: string;
} = $props();

let branches = $state<{ id: string; name: string; name_ar?: string | null }[]>([]);

onMount(() => {
  api
    .listOrgUnits()
    .then((units) => {
      branches = units
        .filter((unit) => unit.is_active && unit.unit_type === "branch")
        .map((unit) => ({ id: unit.id, name: unit.name, name_ar: unit.name_ar }));
    })
    .catch((error: unknown) => {
      console.error("Failed to load organization branches:", error);
    });
});

const displayName = $derived(l(authStore.user?.full_name_ar, authStore.user?.full_name ?? "") ?? "");
const userInitials = $derived(displayName.trim().charAt(0).toUpperCase());

const displayEmail = $derived(
  authStore.user?.email ?? "",
);

const activeBranchName = $derived.by(() => {
  if (!effectiveContext.activeOrgUnitId) return m.shared_ui_all_branches();
  const found = branches.find((b) => b.id === effectiveContext.activeOrgUnitId);
  return found ? l(found.name_ar, found.name) : m.shared_ui_selected_branch();
});

const userRoleDisplay = $derived(
  authStore.user?.roles?.[0] ?? "",
);

function handleNavigate(path: string) {
  goto(localizeHref(path));
}

async function handleLogout() {
  await authStore.logout();
  goto(localizeHref("/login"));
}

async function handleBranchSelect(id: string | null) {
  await api.selectOrgUnit(id ?? undefined);
  effectiveContext.setOrgUnit(id);
  effectiveContext.setClass(null);
  await goto(localizeHref(`/${orgId}`));
}
</script>

<DropdownMenu>
  <DropdownMenuTrigger class="outline-none">
    <div class="flex">
      <Tooltip>
        <TooltipTrigger>
          <button
            type="button"
            class="w-8 h-8 rounded-[10px] bg-[#b02a87] outline-none select-none cursor-pointer btn-tactile p-0 border-none flex items-center justify-center relative"
            style="--btn-shadow: #4a0438;"
            aria-label={m.common_nav_account()}
          >
            <Avatar class="h-full w-full rounded-[10px] bg-transparent">
              <AvatarImage src={authStore.user?.avatar_url ? String(authStore.user.avatar_url) : undefined} alt={displayName} class="object-cover" />
              <AvatarFallback class="bg-transparent text-white font-bold text-sm">
                {userInitials}
              </AvatarFallback>
            </Avatar>
          </button>
        </TooltipTrigger>
        <TooltipContent side="inline-end" sideOffset={10} class="flex flex-col gap-0.5 text-xs">
          <div class="font-bold">{displayName}</div>
          <div class="text-[10px] text-muted-foreground flex items-center gap-1">
            <span class="opacity-80">🏢</span> {activeBranchName}
          </div>
        </TooltipContent>
      </Tooltip>
    </div>
  </DropdownMenuTrigger>
 
  <DropdownMenuPortal>
    <DropdownMenuContent align="end" side="inline-end" sideOffset={12} class="w-72 p-1.5 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
      <div class="flex items-start gap-3 px-3 py-2.5 border-b border-border/50">
        <Avatar class="h-9 w-9 rounded-full shrink-0">
          <AvatarImage src={authStore.user?.avatar_url ? String(authStore.user.avatar_url) : undefined} alt={displayName} class="object-cover" />
          <AvatarFallback class="bg-[#b02a87] text-white font-bold text-xs">
            {userInitials}
          </AvatarFallback>
        </Avatar>
        <div class="flex flex-col min-w-0 leading-tight gap-1">
          {#if displayName}
            <span class="text-xs font-bold text-foreground truncate">{displayName}</span>
          {/if}
          {#if displayEmail}
            <span class="text-[10px] text-muted-foreground truncate">{displayEmail}</span>
          {/if}
          <div class="flex items-center gap-1.5 mt-0.5 flex-wrap">
            {#if userRoleDisplay}
              <span class="text-[9px] px-1.5 py-0.5 bg-primary/10 text-primary rounded font-semibold capitalize">
                {userRoleDisplay}
              </span>
            {/if}
            <span class="text-[9px] px-1.5 py-0.5 bg-muted text-muted-foreground rounded font-medium flex items-center gap-1">
              <span>🏢</span>
              <span class="truncate max-w-[120px]">{activeBranchName}</span>
            </span>
          </div>
        </div>
      </div>

      <!-- Items -->
      <div class="py-1">
        <DropdownMenuSub>
          <DropdownMenuSubTrigger class="cursor-pointer px-3 py-1.5 text-xs rounded">
            <span>{m.common_header_switch_branch()}</span>
            <span class="ms-auto max-w-28 truncate text-[10px] text-muted-foreground">{activeBranchName}</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent class="w-52 p-1">
            <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleBranchSelect(null)}>
              {m.shared_ui_all_branches()}
            </DropdownMenuItem>
            {#each branches as branch (branch.id)}
              <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs rounded" onclick={() => handleBranchSelect(branch.id)}>
                {l(branch.name_ar, branch.name)}
              </DropdownMenuItem>
            {/each}
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleNavigate(`/${orgId}/account`)}>
          {m.common_account_settings_title()}
        </DropdownMenuItem>
        <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => api.connectionOpenWindow()}>
          {m.settings_connection_settings()}
        </DropdownMenuItem>
        <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded text-destructive" onclick={handleLogout}>
          {m.common_sign_out()}
        </DropdownMenuItem>
      </div>
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
