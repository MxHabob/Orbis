<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuPortal,
} from "@orbis/ui-svelte/dropdown-menu";
import { Tooltip, TooltipContent, TooltipTrigger, Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { authStore } from "$lib/stores";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
}: {
  orgId: string;
} = $props();

const userInitials = $derived(
  (authStore.user?.full_name ?? "User")
    .trim()
    .charAt(0)
    .toUpperCase() || "U"
);

const displayName = $derived(
  (authStore.user?.full_name ?? "User").toUpperCase()
);

const displayEmail = $derived(
  (authStore.user?.email ?? "no-email@orbis.ai").toUpperCase()
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
            aria-label={m.nav_account()}
          >
            <Avatar class="h-full w-full rounded-[10px] bg-transparent">
              <AvatarImage src={authStore.user?.avatar ? String(authStore.user.avatar) : undefined} alt={authStore.user?.full_name ?? 'User'} class="object-cover" />
              <AvatarFallback class="bg-transparent text-white font-bold text-sm">
                {userInitials}
              </AvatarFallback>
            </Avatar>
          </button>
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={10}>
          {authStore.user?.full_name ?? m.nav_account()}
        </TooltipContent>
      </Tooltip>
    </div>
  </DropdownMenuTrigger>
 
  <DropdownMenuPortal>
    <DropdownMenuContent align="end" side="right" sideOffset={12} class="w-64 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
      <!-- Header info -->
      <div class="flex items-center gap-3 px-3 py-2.5 border-b border-border/50">
        <Avatar class="h-8 w-8 rounded-full">
          <AvatarImage src={authStore.user?.avatar ? String(authStore.user.avatar) : undefined} alt={authStore.user?.full_name ?? 'User'} class="object-cover" />
          <AvatarFallback class="bg-[#b02a87] text-white font-bold text-xs">
            {userInitials}
          </AvatarFallback>
        </Avatar>
        <div class="flex flex-col min-w-0 leading-tight">
          <span class="text-xs font-bold text-foreground truncate">{displayName}</span>
          <span class="text-[9px] text-muted-foreground truncate font-semibold text-start">{displayEmail}</span>
        </div>
      </div>

      <!-- Items -->
      <div class="py-1">
        <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleNavigate(`/${orgId}/settings`)}>
          User Settings
        </DropdownMenuItem>
        <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={handleLogout}>
          Sign Out
        </DropdownMenuItem>
      </div>
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
