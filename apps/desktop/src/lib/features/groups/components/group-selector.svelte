<!--
  @component GroupSelector
  Reusable, searchable User Group multi-selector component.
  Integrates seamlessly with TanStack Query and Orbis IAM design tokens.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { ArrowPath, CheckCircle, Search, UserGroup, XMark } from "@orbis/icons";
import { formatNumber } from "$lib/core/i18n";
import { createGroupsQuery } from "../queries";

interface Props {
  orgId: string;
  selectedGroupIds?: string[];
  onchange?: (groupIds: string[]) => void;
  maxHeight?: string;
}

let {
  orgId,
  selectedGroupIds = $bindable([]),
  onchange,
  maxHeight = "max-h-60",
}: Props = $props();

let searchQuery = $state("");

const groupsQuery = createGroupsQuery(() => orgId);
const groups = $derived(
  (groupsQuery.data ?? []).filter((g) => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return true;
    const name = (g.name || "").toLowerCase();
    const nameAr = (g.name_ar || "").toLowerCase();
    const desc = (g.description || "").toLowerCase();
    return name.includes(q) || nameAr.includes(q) || desc.includes(q);
  }),
);

function toggleGroup(groupId: string) {
  let next: string[];
  if (selectedGroupIds.includes(groupId)) {
    next = selectedGroupIds.filter((id) => id !== groupId);
  } else {
    next = [...selectedGroupIds, groupId];
  }
  selectedGroupIds = next;
  onchange?.(next);
}

function selectAll() {
  const next = groups.map((g) => g.id);
  selectedGroupIds = next;
  onchange?.(next);
}

function clearAll() {
  selectedGroupIds = [];
  onchange?.([]);
}
</script>

<div class="flex flex-col gap-2.5 w-full">
  <!-- Search & Quick Selection Header -->
  <div class="flex items-center justify-between gap-3">
    <div class="relative flex-1">
      <Search class="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder={m.ui_copy_search_user_groups_6409355a()}
        class="input-sunken ps-8 text-xs py-1.5 h-8"
        bind:value={searchQuery}
      />
    </div>

    <div class="flex items-center gap-2 text-xs text-muted-foreground shrink-0">
      <Badge variant="outline" class="font-mono text-[10px]">
        {formatNumber(selectedGroupIds.length)} {m.ui_copy_selected_d24fa4a6()}
      </Badge>
      <button
        type="button"
        class="text-primary hover:underline text-[11px] cursor-pointer"
        onclick={selectAll}
      >
        {m.ui_copy_select_all_99dda350()}
      </button>
      <span>•</span>
      <button
        type="button"
        class="text-muted-foreground hover:underline text-[11px] cursor-pointer"
        onclick={clearAll}
      >
        {m.ui_copy_clear_9bb7e538()}
      </button>
    </div>
  </div>

  <!-- Groups Grid List -->
  <div class="border border-border/70 rounded-lg overflow-y-auto {maxHeight} p-1.5 bg-muted/10 grid grid-cols-2 gap-2">
    {#if groupsQuery.isLoading}
      <div class="col-span-2 p-6 text-center text-xs text-muted-foreground flex items-center justify-center gap-2">
        <LoadingIndicator class="text-primary"  size={14} />
        <span>{m.ui_copy_loading_groups_a0b56ad6()}</span>
      </div>
    {:else if groups.length === 0}
      <div class="col-span-2 p-6 text-center text-xs text-muted-foreground">
        {m.ui_copy_no_matching_user_groups_found_399e3059()}
      </div>
    {:else}
      {#each groups as group (group.id)}
        {@const isSelected = selectedGroupIds.includes(group.id)}
        <button
          type="button"
          class="flex items-start gap-2.5 p-2.5 rounded-lg border text-start transition-all cursor-pointer {isSelected ? 'border-primary bg-primary/10 text-foreground shadow-2xs' : 'border-border/60 bg-card text-muted-foreground hover:border-border hover:bg-muted/20'}"
          onclick={() => toggleGroup(group.id)}
        >
          <div class="flex size-5 shrink-0 items-center justify-center rounded-xs mt-0.5 {isSelected ? 'bg-primary text-primary-foreground' : 'border border-border bg-muted/40'}">
            {#if isSelected}
              <CheckCircle size={12} />
            {/if}
          </div>

          <div class="flex flex-col min-w-0 flex-1">
            <div class="flex items-center justify-between gap-1">
              <span class="text-xs font-semibold text-foreground truncate">
                {group.name_ar || group.name}
              </span>
              {#if group.is_system}
                <Badge variant="secondary" class="text-[9px] px-1 py-0 shrink-0">
                  {m.groups_system_badge()}
                </Badge>
              {/if}
            </div>

            {#if group.description}
              <span class="text-[10px] text-muted-foreground line-clamp-1 pt-0.5">
                {group.description}
              </span>
            {/if}
          </div>
        </button>
      {/each}
    {/if}
  </div>
</div>
