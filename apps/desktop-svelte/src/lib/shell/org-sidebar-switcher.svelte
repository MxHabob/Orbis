<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuPortal,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@orbis/ui-svelte";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { onMount } from "svelte";

let {
  orgId,
  orgName: initialOrgName = "Orbis School",
}: {
  orgId: string;
  orgName?: string;
} = $props();

let localOrgName = $state("");
let customProjects = $state<{ id: string; label: string; logoType: string }[]>(
  [],
);

$effect(() => {
  if (initialOrgName && initialOrgName !== "Orbis School") {
    localOrgName = initialOrgName;
    return;
  }
  const storedCustom = localStorage.getItem("orbis_custom_orgs");
  if (storedCustom) {
    try {
      const customProjects = JSON.parse(storedCustom);
      const match = customProjects.find((p: any) => p.id === orgId);
      if (match) {
        localOrgName = match.label;
        return;
      }
    } catch {}
  }
  if (orgId) {
    localOrgName = orgId
      .split("-")
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(" ");
  } else {
    localOrgName = "Orbis School";
  }
});

const orgNameDisplay = $derived(
  localOrgName || initialOrgName || "Orbis School",
);

const orgInitials = $derived(
  orgNameDisplay.trim().charAt(0).toUpperCase() || "O",
);

onMount(() => {
  const storedCustom = localStorage.getItem("orbis_custom_orgs");
  if (storedCustom) {
    try {
      customProjects = JSON.parse(storedCustom);
    } catch {}
  }
});

let projects = $derived.by(() => {
  const list = [...customProjects];
  if (orgId && !list.some((p) => p.id === orgId)) {
    list.unshift({
      id: orgId,
      label: orgNameDisplay,
      logoType: "default",
    });
  }
  return list;
});

function getOrgColors(name: string) {
  const safeName = (name || "Orbis").toLowerCase();
  const colors = [
    { bg: "bg-[#e91e63]", shadow: "#3b071e" }, // Pink (matching screenshot)
    { bg: "bg-[#9c27b0]", shadow: "#1b001a" }, // Purple
    { bg: "bg-[#2563eb]", shadow: "#1e3a8a" }, // Blue
    { bg: "bg-[#059669]", shadow: "#064e3b" }, // Emerald
    { bg: "bg-[#7c3aed]", shadow: "#2e1065" }, // Violet
    { bg: "bg-[#d97706]", shadow: "#78350f" }, // Amber
  ];

  if (safeName.includes("authen") || safeName.includes("orbis")) {
    return colors[0];
  }

  let hash = 0;
  for (let i = 0; i < safeName.length; i++) {
    hash = safeName.charCodeAt(i) + ((hash << 5) - hash);
  }
  const index = Math.abs(hash) % colors.length;
  return colors[index];
}

const activeColors = $derived(getOrgColors(orgNameDisplay));

function handleNavigate(path: string) {
  goto(localizeHref(path));
}

function handleSelect(id: string) {
  goto(localizeHref(`/${id}`));
}
</script>

<DropdownMenu>
  <DropdownMenuTrigger class="outline-none">
    <div class="flex">
      <Tooltip>
        <TooltipTrigger>
          <button
            type="button"
            class="w-8 h-8 rounded-md {activeColors.bg} outline-none select-none cursor-pointer btn-tactile p-0 border-none flex items-center justify-center relative"
            style="--btn-shadow: {activeColors.shadow};"
            aria-label={orgNameDisplay}
          >
            <Avatar class="h-full w-full rounded-md bg-transparent">
              <AvatarFallback class="bg-transparent text-white font-bold text-sm">
                {orgInitials}
              </AvatarFallback>
            </Avatar>
          </button>
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={12}>
          {orgNameDisplay}
        </TooltipContent>
      </Tooltip>
    </div>
  </DropdownMenuTrigger>

  <DropdownMenuContent align="start" side="right" sideOffset={12} class="w-56 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
    <!-- Header info -->
    <div class="flex items-center gap-3 px-3 py-2.5 border-b border-border/50">
      <Avatar
        class="h-8 w-8 rounded-[8px] {activeColors.bg} font-bold text-white text-xs"
        style="box-shadow: 0 2px 0 0 {activeColors.shadow}, 0 0 0 1px {activeColors.shadow};"
      >
        <AvatarFallback class="{activeColors.bg} text-white font-bold text-xs">
          {orgInitials}
        </AvatarFallback>
      </Avatar>
      <div class="flex flex-col min-w-0 leading-tight">
        <span class="text-xs font-bold uppercase tracking-wider text-foreground truncate">{orgNameDisplay}</span>
        <span class="text-[9px] text-muted-foreground uppercase font-semibold">{projects.length} {projects.length === 1 ? 'Project' : 'Projects'}</span>
      </div>
    </div>

    <!-- Items -->
    <div class="py-1">
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleNavigate(`/${orgId}/settings`)}>
        Organization Settings
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleNavigate(`/${orgId}/settings`)}>
        Projects
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleNavigate(`/${orgId}/students`)}>
        Members
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded">
        Teams
      </DropdownMenuItem>
      <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded">
        Usage & Billing
      </DropdownMenuItem>
    </div>

    <DropdownMenuSeparator class="bg-border/50" />

    <!-- Submenu for Switch Organization -->
    <div class="py-1">
      <DropdownMenuSub>
        <DropdownMenuSubTrigger class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded">
          Switch Organization
        </DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent class="w-48 p-1 z-50 bg-popover border border-border shadow-md rounded-md">
            {#each projects as project}
              {@const projColors = getOrgColors(project.label)}
              <DropdownMenuItem class="cursor-pointer flex items-center gap-2 px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded" onclick={() => handleSelect(project.id)}>
                <Avatar
                  class="h-5 w-5 rounded-[6px] {projColors.bg} font-bold text-white text-[9px]"
                  style="box-shadow: 0 1.5px 0 0 {projColors.shadow}, 0 0 0 0.5px {projColors.shadow};"
                >
                  <AvatarFallback class="{projColors.bg} text-white font-bold text-[9px]">
                    {(project.label || "O").charAt(0).toUpperCase()}
                  </AvatarFallback>
                </Avatar>
                <span class="truncate font-medium">{project.label}</span>
              </DropdownMenuItem>
            {/each}
            <DropdownMenuSeparator class="bg-border/50" />
            <DropdownMenuItem class="cursor-pointer px-3 py-1.5 text-xs text-foreground hover:bg-accent rounded font-medium" >
              + Create a new organization
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuPortal>
      </DropdownMenuSub>
    </div>
  </DropdownMenuContent>
</DropdownMenu>
