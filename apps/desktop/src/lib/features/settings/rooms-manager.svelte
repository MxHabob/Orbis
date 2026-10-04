<!--
@component RoomsManager
Lists all rooms and venues with OrbisGrid, CRUD via RoomCreateSheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage, type RoomDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { toast } from "@orbis/ui";
import { formatDate, l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Pin, Plus, Search, Refresh } from "@orbis/icons";
import RoomCreateSheet from "$lib/features/academics/components/room-create-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { roomFilterFields } from "$lib/core/filter-configs";
import { confirmStore } from "$lib/stores";

let { orgId }: { orgId: string } = $props();

// State
let rooms = $state<RoomDto[]>([]);
let orgUnits = $state<any[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let isSheetOpen = $state(false);
let editingRoom = $state<RoomDto | null>(null);
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const orgUnitsMap = $derived(new Map(orgUnits.map((u) => [String(u.id), u])));

async function loadRooms() {
  isLoading = true;
  try {
    const [fetchedRooms, fetchedUnits] = await Promise.all([
      api.listRooms(),
      api.listOrgUnits().catch(() => []),
    ]);
    rooms = fetchedRooms;
    orgUnits = fetchedUnits;
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadRooms();
});

const roomActions: ActionItem[] = [
  { id: "edit", label: m.academics_schedule_edit(), variant: "default" },
  { id: "delete", label: m.common_action_delete(), variant: "destructive" },
];

const gridSource = $derived(
  rooms.map((r, i) => {
    const unit = r.org_unit_id ? orgUnitsMap.get(String(r.org_unit_id)) : null;
    const buildingName = unit ? (l(unit.name_ar, unit.name) || unit.name) : (r.building || "—");
    return {
      ...r,
      __idx: i + 1,
      name: l(r.name_ar, r.name) || r.name || "—",
      building: buildingName,
      floor: r.floor !== null && r.floor !== undefined ? String(r.floor) : "—",
      capacity: r.capacity ?? "—",
      room_type: r.room_type || "—",
      status: r.is_active !== false ? "active" : "inactive",
      created_at_display: r.created_at ? formatDate(r.created_at, "datetime") : "—",
      updated_at_display: r.updated_at ? formatDate(r.updated_at, "datetime") : "—",
    };
  }),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "name", prop: "name", header: m.common_rooms_col_name(), name: m.common_rooms_col_name(), width: 200, size: 200, readonly: true, pin: "left", cellClass: "font-medium" },
  { id: "room_type", prop: "room_type", header: m.common_word_type(), name: m.common_word_type(), width: 130, size: 130, readonly: true },
  { id: "building", prop: "building", header: m.common_rooms_col_location(), name: m.common_rooms_col_location(), width: 150, size: 150, readonly: true },
  { id: "floor", prop: "floor", header: m.settings_ui_room_floor(), name: m.settings_ui_room_floor(), width: 90, size: 90, readonly: true, align: "center" },
  { id: "capacity", prop: "capacity", header: m.common_rooms_col_capacity(), name: m.common_rooms_col_capacity(), width: 100, size: 100, readonly: true, align: "center", type: "number" },
  { id: "status", prop: "status", header: m.common_word_status(), name: m.common_word_status(), width: 100, size: 100, type: "badge", align: "center", readonly: true },
  { id: "created_at_display", prop: "created_at_display", header: m.ui_copy_created_at_01f6ea5f(), name: m.ui_copy_created_at_01f6ea5f(), width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "updated_at_display", prop: "updated_at_display", header: m.ui_copy_updated_at_ba0d0a9f(), name: m.ui_copy_updated_at_ba0d0a9f(), width: 130, size: 130, readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  { id: "actions", prop: "actions", header: m.ui_copy_actions_c48127a5(), name: m.ui_copy_actions_c48127a5(), width: 120, size: 120, type: "action", align: "center", readonly: true, meta: { items: roomActions } },
];

function openCreateSheet() {
  editingRoom = null;
  isSheetOpen = true;
}

function promptDelete(room: RoomDto) {
  confirmStore.open({
    title: m.ui_copy_delete_room_b9cc731b(),
    message: m.ui_room_delete_confirmation({ roomName: room.name_ar || room.name }),
    confirmLabel: m.common_action_delete(),
    variant: "destructive",
    onConfirm: async () => {
      try {
        await api.deleteRoom(room.id);
        toast.success(m.ui_copy_room_deleted_successfully_6298d43a());
        loadRooms();
      } catch (err) {
        toast.error(extractErrorMessage(err));
      }
    },
  });
}

function handleGridAction(event: GridActionEvent) {
  const room = event.row as unknown as RoomDto;
  if (!room) return;
  if (event.actionId === "dblclick" || event.actionId === "edit") {
    editingRoom = room;
    isSheetOpen = true;
  } else if (event.actionId === "delete") {
    promptDelete(room);
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
