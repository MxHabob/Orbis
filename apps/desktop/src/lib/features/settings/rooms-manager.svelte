<!--
@component RoomsManager
Lists all rooms and venues with OrbisGrid, CRUD via RoomCreateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Pin, Plus, Search, Refresh } from "@orbis/icons";
import RoomCreateSheet from "$lib/features/academics/components/room-create-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { roomFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

// State
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let isSheetOpen = $state(false);
let editingRoom = $state<any>(null);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

async function loadRooms() {
  isLoading = true;
  try {
    rooms = await api.listRooms();
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load rooms");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadRooms();
});

const gridSource = $derived(
  rooms.map((r, i) => ({
    ...r,
    __idx: i + 1,
    name: l(r.name_ar, r.name) || r.name || "—",
    building: r.building || "—",
    floor: r.floor !== null && r.floor !== undefined ? String(r.floor) : "—",
    capacity: r.capacity ?? "—",
    room_type: r.room_type || "—",
    status: r.is_active !== false ? "active" : "inactive",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_rooms_col_name(), name: m.common_rooms_col_name(), width: 200, size: 200, readonly: true, pin: "left", cellClass: "font-medium" },
  { id: "room_type", prop: "room_type", header: m.common_word_type(), name: m.common_word_type(), width: 130, size: 130, readonly: true },
  { id: "building", prop: "building", header: m.common_rooms_col_location(), name: m.common_rooms_col_location(), width: 150, size: 150, readonly: true },
  { id: "floor", prop: "floor", header: "Floor", name: "Floor", width: 90, size: 90, readonly: true, align: "center" },
  { id: "capacity", prop: "capacity", header: m.common_rooms_col_capacity(), name: m.common_rooms_col_capacity(), width: 100, size: 100, readonly: true, align: "center", type: "number" },
  { id: "status", prop: "status", header: "Status", name: "Status", width: 100, size: 100, type: "badge", align: "center", readonly: true },
];

function openCreateSheet() {
  editingRoom = null;
  isSheetOpen = true;
}

function handleGridAction(event: GridActionEvent) {
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    editingRoom = event.row;
    isSheetOpen = true;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={m.common_rooms_search_placeholder()}
        bind:value={searchQuery}
        class="h-8 ps-8 text-xs bg-card border-border/60"
      />
    </div>

    <div class="flex items-center gap-2">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={roomFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadRooms} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button variant="lime" class="text-xs h-8 px-4 font-semibold shrink-0" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.common_rooms_add()}
      </Button>
    </div>
  </div>

  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      filter={activeFilter}
      emptyTitle={m.common_rooms_empty()}
      emptyDescription={m.common_rooms_empty_desc()}
      rtl={getLocale() === "ar"}
      readonly
      onaction={handleGridAction}
    />
  </div>
</div>

<RoomCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  room={editingRoom}
  onSuccess={loadRooms}
/>
