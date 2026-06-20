<script lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuPortal,
} from "@orbis/ui-svelte/dropdown-menu";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui-svelte";
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
        <TooltipTrigger asChild>
          <button
            class="flex h-9 w-9 items-center justify-center rounded-[10px] bg-[#9c27b0] font-bold text-white transition-transform active:translate-y-[2px] font-sans text-sm outline-none select-none cursor-pointer"
            style="box-shadow: 0 3px 0 0 #1b001a, 0 0 0 1px #1b001a;"
            aria-label={m.nav_account()}
          >
            {#if authStore.user?.avatar}
              <img
                src={authStore.user.avatar}
                alt={authStore.user.full_name ?? 'User'}
                class="h-full w-full object-cover rounded-[10px]"
              />
            {:else}
              {userInitials}
            {/if}
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
        <div
          class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[#9c27b0] font-bold text-white text-xs"
        >
          {#if authStore.user?.avatar}
            <img
              src={authStore.user.avatar}
              alt={authStore.user.full_name ?? 'User'}
              class="h-full w-full object-cover rounded-full"
            />
          {:else}
            {userInitials}
          {/if}
        </div>
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
