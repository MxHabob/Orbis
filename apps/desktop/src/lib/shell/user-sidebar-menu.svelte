<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
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
      branches = units.map((u) => ({ id: u.id, name: u.name, name_ar: u.name_ar }));
    })
    .catch(() => {
      branches = [];
    });
});

const userInitials = $derived(
  (authStore.user?.full_name ?? "User").trim().charAt(0).toUpperCase() || "U",
);

const displayName = $derived(
  authStore.user?.full_name ?? "User",
);

const displayEmail = $derived(
  authStore.user?.email ?? "no-email@orbis.ai",
);

const activeBranchName = $derived.by(() => {
  if (!effectiveContext.activeOrgUnitId) return "الإدارة العامة / All Branches";
  const found = branches.find((b) => b.id === effectiveContext.activeOrgUnitId);
  return found ? (found.name_ar || found.name) : "فرع محدد";
});

const userRoleDisplay = $derived(
  effectiveContext.effectiveRole || "User",
);

function handleNavigate(path: string) {
  goto(localizeHref(path));
}

async function handleLogout() {
  await authStore.logout();
  goto(localizeHref("/login"));
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
              <AvatarImage src={authStore.user?.avatar_url ? String(authStore.user.avatar_url) : undefined} alt={authStore.user?.full_name ?? 'User'} class="object-cover" />
              <AvatarFallback class="bg-transparent text-white font-bold text-sm">
                {userInitials}
              </AvatarFallback>
            </Avatar>
          </button>
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={10} class="flex flex-col gap-0.5 text-xs">
          <div class="font-bold">{displayName}</div>
          <div class="text-[10px] text-muted-foreground flex items-center gap-1">
            <span class="opacity-80">🏢</span> {activeBranchName}
          </div>
        </TooltipContent>
      </Tooltip>
    </div>
  </DropdownMenuTrigger>
 
  <DropdownMenuPortal>
    <DropdownMenuContent align="end" side="right" sideOffset={12} class="w-72 p-1.5 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
      <div class="flex items-start gap-3 px-3 py-2.5 border-b border-border/50">
        <Avatar class="h-9 w-9 rounded-full shrink-0">
          <AvatarImage src={authStore.user?.avatar_url ? String(authStore.user.avatar_url) : undefined} alt={authStore.user?.full_name ?? 'User'} class="object-cover" />
          <AvatarFallback class="bg-[#b02a87] text-white font-bold text-xs">
            {userInitials}
          </AvatarFallback>
        </Avatar>
        <div class="flex flex-col min-w-0 leading-tight gap-1">
          <span class="text-xs font-bold text-foreground truncate">{displayName}</span>
          <span class="text-[10px] text-muted-foreground truncate">{displayEmail}</span>
          <div class="flex items-center gap-1.5 mt-0.5 flex-wrap">
            <span class="text-[9px] px-1.5 py-0.5 bg-primary/10 text-primary rounded font-semibold capitalize">
              {userRoleDisplay}
            </span>
            <span class="text-[9px] px-1.5 py-0.5 bg-muted text-muted-foreground rounded font-medium flex items-center gap-1">
              <span>🏢</span>
              <span class="truncate max-w-[120px]">{activeBranchName}</span>
            </span>
          </div>
        </div>
      </div>

      <!-- Items -->
      <div class="py-1">
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
