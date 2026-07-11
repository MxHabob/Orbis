<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Plus, DoorOpen } from "lucide-svelte";
import RoomCreateSheet from "$lib/features/academics/components/room-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

import { DataTable } from "$lib/tables";
import { rooms as roomsDef, withIndex } from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let isSheetOpen = $state(false);
let editingRoom = $state<any>(null);

// ── Data Loading ────────────────────────────────────────────────────────

async function loadRooms() {
  isLoading = true;
  try {
    rooms = await commands.scheduling.listRooms(data.orgId);
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load rooms");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadRooms();
});

// ── Filtering ───────────────────────────────────────────────────────────

const filteredRooms = $derived(
  rooms.filter((r) => {
    const q = searchQuery.toLowerCase();
    return (
      !q ||
      (r.name || "").toLowerCase().includes(q) ||
      (r.name_ar || "").toLowerCase().includes(q) ||
      (r.room_type || "").toLowerCase().includes(q) ||
      (r.building || "").toLowerCase().includes(q)
    );
  }),
);

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredRooms.map((r) => ({
      ...r,
      name: l(r.name_ar, r.name) || r.name || "—",
      building: r.building || "—",
      floor: r.floor !== null && r.floor !== undefined ? String(r.floor) : "—",
      capacity: r.capacity ?? "—",
      room_type: r.room_type || "—",
      status: r.is_active !== false ? "active" : "inactive",
    })),
  ),
);

const columns = $derived(
  roomsDef.columns({
    idx: "#",
    name: m.rooms_col_name(),
    type: m.rooms_col_type(),
    capacity: m.rooms_col_capacity(),
    building: m.rooms_col_location(),
    status: "Status",
  }),
);

// ── Sheet Controls ──────────────────────────────────────────────────────

function openCreateSheet() {
  editingRoom = null;
  isSheetOpen = true;
}
</script>

<svelte:head>
  <title>{m.settings_rooms_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide">
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <h1 class="text-lg font-bold text-foreground truncate">{m.settings_rooms_window_title()}</h1>
    <Button variant="lime" class="text-xs h-8 px-4 font-semibold shrink-0" onclick={openCreateSheet}>
      <Plus size={14} class="me-1.5" />
      {m.rooms_add()}
    </Button>
  </div>

  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      searchable
      searchPlaceholder={m.settings_rooms_search_placeholder()}
      bind:searchValue={searchQuery}
      emptyIcon={DoorOpen}
      emptyTitle={m.rooms_empty()}
      emptyDescription={m.rooms_empty_desc()}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadRooms}
    />
  </div>
</div>

<RoomCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  room={editingRoom}
  onSuccess={loadRooms}
/>
