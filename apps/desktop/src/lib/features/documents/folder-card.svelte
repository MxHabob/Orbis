<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Folder,
  FolderOpen,
  Sparkles,
  AcademicCap,
  BuildingOffice,
  Trash,
  Pencil,
  EllipsisVertical,
  DocumentPage,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import type { FolderDto } from "$lib/api";

let {
  folder,
  itemCount = 0,
  onopen,
  ondelete,
  onrename,
}: {
  folder: FolderDto;
  itemCount?: number;
  onopen: (folder: FolderDto) => void;
  ondelete?: (folder: FolderDto) => void;
  onrename?: (folder: FolderDto) => void;
} = $props();

const isCurriculum = $derived(
  folder.folder_type === "curriculum" ||
  folder.metadata?.knowledge_base === true ||
  folder.metadata?.ai_agent_accessible === true
);

const isStudent = $derived(
  folder.folder_type === "student" ||
  folder.entity_type === "student"
);

const isSystem = $derived(folder.is_system);

let isMenuOpen = $state(false);
let isDragOver = $state(false);

function handleDragOver(e: DragEvent) {
  e.preventDefault();
  isDragOver = true;
}

function handleDragLeave() {
  isDragOver = false;
}

function handleDrop(e: DragEvent) {
  e.preventDefault();
  isDragOver = false;
  // Let the parent know or open folder
  onopen(folder);
}
</script>

<div
  role="button"
  tabindex="0"
  onclick={() => onopen(folder)}
  onkeydown={(e) => e.key === "Enter" && onopen(folder)}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  class="group relative flex flex-col justify-between p-4 rounded-xl border transition-all duration-200 text-start select-none cursor-pointer
    {isDragOver ? 'border-primary bg-primary/10 ring-2 ring-primary/30 scale-[1.02]' : 'hover:border-border/80 hover:shadow-md hover:-translate-y-0.5'}
    {isCurriculum
      ? 'bg-gradient-to-br from-emerald-500/10 via-emerald-500/5 to-transparent border-emerald-500/30'
      : isStudent
      ? 'bg-gradient-to-br from-indigo-500/10 via-indigo-500/5 to-transparent border-indigo-500/30'
      : 'bg-card/70 border-border/40'}"
>
  <!-- Top Bar: Icon & Badges & Actions -->
  <div class="flex items-start justify-between gap-2">
    <!-- Folder Icon with Badge -->
    <div class="relative">
      {#if isCurriculum}
        <div class="w-10 h-10 rounded-lg flex items-center justify-center bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 shadow-inner">
          <Sparkles class="w-5 h-5" />
        </div>
      {:else if isStudent}
        <div class="w-10 h-10 rounded-lg flex items-center justify-center bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 shadow-inner">
          <AcademicCap class="w-5 h-5" />
        </div>
      {:else}
        <div class="w-10 h-10 rounded-lg flex items-center justify-center bg-amber-500/20 text-amber-600 dark:text-amber-400 shadow-inner">
          <Folder class="w-5 h-5" />
        </div>
      {/if}
    </div>

    <!-- Badges & Action Menu -->
    <div class="flex items-center gap-1">
      {#if isCurriculum}
        <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-medium bg-emerald-500/20 text-emerald-700 dark:text-emerald-300 border border-emerald-500/30">
          <Sparkles class="w-2.5 h-2.5" />
          <span>{m.operations_ui_ai_curriculum_folder()}</span>
        </span>
      {:else if isStudent}
        <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-medium bg-indigo-500/20 text-indigo-700 dark:text-indigo-300 border border-indigo-500/30">
          <span>{m.operations_ui_student_folder()}</span>
        </span>
      {:else if isSystem}
        <span class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-muted text-muted-foreground">
          {m.operations_ui_system()}
        </span>
      {/if}

      {#if !isSystem && (ondelete || onrename)}
        <div class="opacity-0 group-hover:opacity-100 transition-opacity">
          {#if ondelete}
            <Button
              variant="ghost"
              size="icon-xs"
              class="text-muted-foreground hover:text-destructive hover:bg-destructive/10"
              onclick={(e) => {
                e.stopPropagation();
                ondelete?.(folder);
              }}
              title={m.operations_ui_delete_folder()}
            >
              <Trash class="w-3.5 h-3.5" />
            </Button>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Folder Details -->
  <div class="mt-3">
    <h4 class="font-semibold text-sm text-foreground truncate" title={folder.name}>
      {folder.name}
    </h4>
    <div class="flex items-center justify-between gap-2 mt-1 text-[11px] text-muted-foreground">
      <span class="truncate font-mono text-[10px] opacity-75" title={folder.path}>
        {folder.path}
      </span>
      {#if itemCount > 0}
        <span class="shrink-0 font-medium px-1.5 py-0.2 rounded bg-muted/60">
          {m.operations_ui_folder_item_count({ count: itemCount })}
        </span>
      {/if}
    </div>
  </div>
</div>
