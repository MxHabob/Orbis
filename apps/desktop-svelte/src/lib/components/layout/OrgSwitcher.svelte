<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Check,
  ChevronDown,
  Search,
  ExternalLink,
  Settings,
  Star,
  HelpCircle,
  Plus,
} from "lucide-svelte";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { onMount } from "svelte";

// Props received from parent
let {
  orgId,
  orgName = "Orbis School",
}: {
  orgId: string;
  orgName?: string;
} = $props();

// Switcher state
let open = $state(false);
let searchQuery = $state("");
let isCreating = $state(false);
let newOrgName = $state("");

// Persisted starred projects and custom projects
let starredProjects = $state<string[]>([]);
let customProjects = $state<{ id: string; label: string; logoType: string }[]>([]);

onMount(() => {
  const storedStars = localStorage.getItem("orbis_starred_orgs");
  if (storedStars) {
    try {
      starredProjects = JSON.parse(storedStars);
    } catch {}
  }

  const storedCustom = localStorage.getItem("orbis_custom_orgs");
  if (storedCustom) {
    try {
      customProjects = JSON.parse(storedCustom);
    } catch {}
  }
});

// Dynamic organizations list (Real user orgs + custom created orgs, no fake projects)
let projects = $derived.by(() => {
  const baseList = [...customProjects];
  const list = [...baseList];

  // Prepend current organization if it's not in the list
  if (orgId && !list.some((p) => p.id === orgId)) {
    list.unshift({
      id: orgId,
      label: orgName || orgId,
      logoType: "default",
    });
  }
  return list;
});

// Filter items based on search query
let filteredProjects = $derived(
  projects.filter((p) =>
    p.label.toLowerCase().includes(searchQuery.toLowerCase())
  )
);

// Selected organization model
let activeProject = $derived(
  projects.find((p) => p.id === orgId) || projects[0] || {
    id: orgId,
    label: orgName,
    logoType: "default",
  }
);

function handleSelect(id: string) {
  open = false;
  searchQuery = "";
  goto(localizeHref(`/${id}`));
}

function resetSearch() {
  searchQuery = "";
}

function toggleStar(id: string, event: Event) {
  event.stopPropagation();
  if (starredProjects.includes(id)) {
    starredProjects = starredProjects.filter((x) => x !== id);
  } else {
    starredProjects = [...starredProjects, id];
  }
  localStorage.setItem("orbis_starred_orgs", JSON.stringify(starredProjects));
}

function handleCreateOrg() {
  if (!newOrgName.trim()) return;
  const newId = newOrgName.trim().toLowerCase().replace(/\s+/g, "-");

  const newProj = {
    id: newId,
    label: newOrgName.trim(),
    logoType: "default",
  };

  customProjects = [...customProjects, newProj];
  localStorage.setItem("orbis_custom_orgs", JSON.stringify(customProjects));

  // Reset creation form
  newOrgName = "";
  isCreating = false;
  open = false;

  // Navigate to newly created organization
  goto(localizeHref(`/${newId}`));
}

// Generate colored initials avatar dynamically
function getOrgColor(name: string) {
  const colors = [
    "bg-blue-500/10 text-blue-600 border-blue-200 dark:bg-blue-500/20 dark:text-blue-400 dark:border-blue-800",
    "bg-emerald-500/10 text-emerald-600 border-emerald-200 dark:bg-emerald-500/20 dark:text-emerald-400 dark:border-emerald-800",
    "bg-indigo-500/10 text-indigo-600 border-indigo-200 dark:bg-indigo-500/20 dark:text-indigo-400 dark:border-indigo-800",
    "bg-violet-500/10 text-violet-600 border-violet-200 dark:bg-violet-500/20 dark:text-violet-400 dark:border-violet-800",
    "bg-amber-500/10 text-amber-600 border-amber-200 dark:bg-amber-500/20 dark:text-amber-400 dark:border-amber-800",
    "bg-rose-500/10 text-rose-600 border-rose-200 dark:bg-rose-500/20 dark:text-rose-400 dark:border-rose-800",
  ];
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  const index = Math.abs(hash) % colors.length;
  return colors[index];
}

function getOrgInitials(name: string) {
  return name
    .split(/[\s-_]+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((word) => word[0].toUpperCase())
    .join("");
}
</script>

<div class="flex min-w-0 justify-center overflow-hidden">
  <DropdownMenu bind:open>
    <DropdownMenuTrigger>
      <Button
        variant="secondary"
        size="sm"
        class="text-[12px] font-semibold max-w-full"
      >
        <!-- Dynamic Initials Avatar -->
        <div class="flex size-5 shrink-0 items-center justify-center rounded border font-bold text-[9px] font-mono {getOrgColor(activeProject.label)}">
          {getOrgInitials(activeProject.label)}
        </div>

        <span class="truncate">{activeProject.label}</span>
        <ChevronDown size={12} strokeWidth={2} class="shrink-0 text-muted-foreground opacity-60" />
      </Button>
    </DropdownMenuTrigger>

    <DropdownMenuContent
      align="center"
      sideOffset={8}
      class="w-72 p-0 overflow-hidden select-none z-50"
    >
      {#if isCreating}
        <!-- Create Organization Form -->
        <div class="p-3 space-y-3">
          <div class="text-xs font-semibold text-foreground flex items-center justify-between">
            <span>{m.switcher_new_org_title()}</span>
            <button
              type="button"
              onclick={() => isCreating = false}
              class="text-muted-foreground hover:text-foreground text-[10px] underline"
            >
              {m.switcher_cancel()}
            </button>
          </div>
          <div class="relative">
            <input
              type="text"
              bind:value={newOrgName}
              placeholder={m.switcher_new_org_placeholder()}
              class="w-full h-8 px-2.5 text-xs bg-background border border-input rounded-md focus:outline-none focus:ring-1 focus:ring-ring"
              onkeydown={(e) => {
                if (e.key === "Enter") handleCreateOrg();
              }}
            />
          </div>
          <Button
            size="sm"
            class="w-full h-8 text-xs font-medium"
            onclick={handleCreateOrg}
            disabled={!newOrgName.trim()}
          >
            {m.switcher_create()}
          </Button>
        </div>
      {:else}
        <!-- Header -->
        <div class="flex items-center justify-between px-3 py-2 border-b border-border/50 text-[11px] font-semibold text-muted-foreground bg-muted/20">
          <div class="flex items-center gap-1">
            <span>{m.switcher_filter_orgs()}</span>
            <div class="text-muted-foreground/60 cursor-help" title={m.switcher_help()}>
              <HelpCircle size={12} />
            </div>
          </div>
          <button
            type="button"
            onclick={resetSearch}
            class="hover:text-foreground transition-colors text-[10px] font-medium"
          >
            {m.switcher_reset()}
          </button>
        </div>

        <!-- Search Bar -->
        <div class="relative p-2 border-b border-border/50 flex items-center">
          <Search size={13} class="absolute left-4 top-1/2 -translate-y-1/2 text-muted-foreground/60 pointer-events-none" />
          <input
            type="text"
            placeholder={m.switcher_search_placeholder()}
            bind:value={searchQuery}
            class="w-full h-8 pl-8 pr-3 text-xs bg-background border border-input rounded-md focus:outline-none focus:ring-1 focus:ring-ring placeholder-muted-foreground/50"
          />
        </div>

        <!-- Organizations List -->
        <div class="max-h-56 overflow-y-auto p-1 space-y-0.5">
          {#each filteredProjects as project (project.id)}
            <div
              role="button"
              tabindex="0"
              onclick={() => handleSelect(project.id)}
              onkeydown={(e) => {
                if (e.key === "Enter" || e.key === " ") handleSelect(project.id);
              }}
              class="flex items-center justify-between px-2.5 py-1.5 text-xs rounded-md hover:bg-muted/70 cursor-pointer group transition-colors"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <!-- Checkbox -->
                <div
                  class="flex size-4 shrink-0 items-center justify-center rounded border transition-colors
                  {project.id === orgId ? 'bg-indigo-600 border-indigo-600 text-white' : 'border-border bg-transparent text-transparent'}"
                >
                  {#if project.id === orgId}
                    <Check size={10} strokeWidth={3.5} />
                  {/if}
                </div>

                <!-- Dynamic Initials Avatar -->
                <div class="flex size-5 shrink-0 items-center justify-center rounded border font-bold text-[9px] font-mono {getOrgColor(project.label)}">
                  {getOrgInitials(project.label)}
                </div>

                <!-- Name -->
                <span class="truncate font-medium text-foreground">{project.label}</span>
              </div>

              <!-- Actions on Hover -->
              <div class="opacity-0 group-hover:opacity-100 flex items-center gap-2 transition-opacity duration-100">
                <a
                  href={`https://orbis.ai/${project.id}`}
                  target="_blank"
                  onclick={(e) => e.stopPropagation()}
                  class="text-muted-foreground hover:text-foreground transition-colors"
                  title={m.switcher_open_browser()}
                >
                  <ExternalLink size={12} />
                </a>
                <a
                  href={localizeHref(`/${project.id}/settings`)}
                  onclick={(e) => {
                    e.stopPropagation();
                    open = false;
                  }}
                  class="text-muted-foreground hover:text-foreground transition-colors"
                  title={m.switcher_settings()}
                >
                  <Settings size={12} />
                </a>
                <button
                  type="button"
                  onclick={(e) => toggleStar(project.id, e)}
                  class="hover:text-amber-500 transition-colors p-0"
                  title={m.switcher_star()}
                >
                  {#if starredProjects.includes(project.id)}
                    <Star size={12} fill="#f59e0b" class="text-amber-500" />
                  {:else}
                    <Star size={12} class="text-muted-foreground" />
                  {/if}
                </button>
              </div>
            </div>
          {:else}
            <div class="px-3 py-4 text-center text-xs text-muted-foreground">
              {m.switcher_no_results()}
            </div>
          {/each}
        </div>

        <!-- Create Button -->
        <DropdownMenuSeparator />
        <div class="p-1">
          <button
            type="button"
            onclick={() => isCreating = true}
            class="w-full h-8 border border-border/60 hover:bg-muted/70 rounded-md flex items-center justify-center gap-1.5 text-xs font-semibold text-foreground transition-colors"
          >
            <Plus size={13} />
            <span>{m.switcher_create_org()}</span>
          </button>
        </div>
      {/if}
    </DropdownMenuContent>
  </DropdownMenu>
</div>
