<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  ChevronLeft,
  ChevronRight,
  Folder,
  FolderOpen,
  Home,
  Sparkles,
  User,
  AcademicCap,
} from "@orbis/icons";
import type { FolderDto } from "$lib/api";

let {
  orgId,
  currentFolder = null,
  breadcrumbs = [],
  onnavigate,
}: {
  orgId: string;
  currentFolder?: FolderDto | null;
  breadcrumbs?: FolderDto[];
  onnavigate: (folderId?: string | null) => void;
} = $props();

const isCurriculum = $derived(
  currentFolder?.folder_type === "curriculum" ||
    currentFolder?.metadata?.ai_agent_accessible === true,
);
const isStudent = $derived(
  currentFolder?.folder_type === "student" ||
    currentFolder?.entity_type === "student",
);
</script>

<nav aria-label={m.operations_ui_folder_breadcrumbs()} class="flex items-center flex-wrap gap-1.5 text-xs text-muted-foreground">
  <!-- Root / Home Button -->
  <button
    type="button"
    onclick={() => onnavigate(null)}
    class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md transition-colors hover:text-foreground hover:bg-muted/60 {currentFolder ? 'font-normal' : 'font-semibold text-foreground bg-muted/40'}"
  >
    <Home class="w-3.5 h-3.5" />
    <span>{m.operations_ui_documents_archive()}</span>
  </button>

  {#if breadcrumbs && breadcrumbs.length > 0}
    {#each breadcrumbs as crumb, i}
      <ChevronLeft class="w-3 h-3 text-muted-foreground/50 rtl:rotate-0 ltr:rotate-180 shrink-0" />
      <button
        type="button"
        onclick={() => onnavigate(crumb.id)}
        class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md transition-colors hover:text-foreground hover:bg-muted/60 truncate max-w-40 font-normal"
        title={crumb.name}
      >
        {#if crumb.folder_type === "curriculum"}
          <Sparkles class="w-3.5 h-3.5 text-emerald-500 shrink-0" />
        {:else if crumb.folder_type === "student"}
          <AcademicCap class="w-3.5 h-3.5 text-indigo-500 shrink-0" />
        {:else}
          <Folder class="w-3.5 h-3.5 text-amber-500 shrink-0" />
        {/if}
        <span class="truncate">{crumb.name}</span>
      </button>
    {/each}
  {/if}

  {#if currentFolder}
    <ChevronLeft class="w-3 h-3 text-muted-foreground/50 rtl:rotate-0 ltr:rotate-180 shrink-0" />
    <div class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md bg-accent/60 text-accent-foreground font-semibold truncate max-w-56">
      {#if isCurriculum}
        <Sparkles class="w-3.5 h-3.5 text-emerald-500 shrink-0" />
      {:else if isStudent}
        <AcademicCap class="w-3.5 h-3.5 text-indigo-500 shrink-0" />
      {:else}
        <FolderOpen class="w-3.5 h-3.5 text-amber-500 shrink-0" />
      {/if}
      <span class="truncate">{currentFolder.name}</span>
    </div>
  {/if}
</nav>
