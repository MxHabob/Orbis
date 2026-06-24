<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import { Plus, Search, Pencil, Trash2, DoorOpen, Landmark } from "lucide-svelte";
import RoomCreateSheet from "$lib/components/sheets/room-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters
let searchQuery = $state("");

// Sheet controls
let isSheetOpen = $state(false);
let editingRoom = $state<any>(null);

async function loadRooms() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.schedule.getRooms(data.orgId);
    rooms = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error("Failed to load rooms");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadRooms();
  }
});

const filteredRooms = $derived(
  rooms.filter((r) => {
    return (
      (r.name || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (r.name_ar || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (r.room_type || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (r.building || "").toLowerCase().includes(searchQuery.toLowerCase())
    );
  })
);

function openCreateSheet() {
  editingRoom = null;
  isSheetOpen = true;
}

function openEditSheet(room: any) {
  editingRoom = room;
  isSheetOpen = true;
}

async function handleDeleteRoom(room: any) {
  if (!confirm(m.rooms_delete_confirm())) return;

  try {
    await commands.schedule.deleteRoom(data.orgId, room.id);
    toast.success(m.rooms_delete_success());
    loadRooms();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getRoomTypeBadgeClass(typeStr: string): string {
  switch (String(typeStr).toLowerCase()) {
    case "classroom":
      return "bg-lime-500/10 text-lime-500 border-lime-500/20";
    case "laboratory":
      return "bg-blue-500/10 text-blue-500 border-blue-500/20";
    case "lecture_hall":
      return "bg-purple-500/10 text-purple-500 border-purple-500/20";
    case "office":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    default:
      return "bg-slate-500/10 text-slate-500 border-slate-500/20";
  }
}
</script>

<svelte:head>
  <title>{m.rooms_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <span>{m.rooms_title()}</span>
        {#if !isLoading}
          <span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {filteredRooms.length}
          </span>
        {/if}
      </h1>
      <p class="text-xs text-muted-foreground">{m.rooms_desc()}</p>
    </div>

    <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      {m.rooms_add()}
    </Button>
  </div>

  <!-- Search Bar -->
  <div class="relative w-full">
    <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
    <Input
      type="text"
      bind:value={searchQuery}
      placeholder="Search rooms by name, type or location..."
      class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs"
    />
  </div>

  <!-- Table/Grid Content -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
      <p class="text-xs text-destructive font-semibold">Failed to load rooms</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadRooms}>{m.error_retry()}</Button>
    </div>
  {:else if filteredRooms.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <DoorOpen size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">{m.rooms_empty()}</p>
        <p class="text-[10px] text-muted-foreground/80">{m.rooms_empty_desc()}</p>
      </div>
    </div>
  {:else}
    <div class="border border-border rounded-xl bg-card/20 overflow-hidden" in:fade={{ duration: 150 }}>
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-transparent">
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.rooms_col_name()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.rooms_col_type()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.rooms_col_capacity()}</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">{m.rooms_col_location()}</TableHead>
              <TableHead class="text-end text-xs font-bold text-muted-foreground px-6">{m.rooms_col_actions()}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredRooms as room (room.id)}
              <TableRow class="hover:bg-accent/5">
                <!-- Name -->
                <TableCell>
                  <div class="flex flex-col text-start">
                    <span class="text-xs font-bold text-foreground">
                      {getLocale() === "ar" && room.name_ar ? room.name_ar : room.name}
                    </span>
                    {#if getLocale() === "ar" && room.name_ar && room.name}
                      <span class="text-[9px] text-muted-foreground leading-none mt-0.5">{room.name}</span>
                    {/if}
                  </div>
                </TableCell>
                
                <!-- Room Type -->
                <TableCell>
                  <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase {getRoomTypeBadgeClass(room.room_type)}">
                    {room.room_type}
                  </span>
                </TableCell>
                
                <!-- Capacity -->
                <TableCell class="text-xs text-muted-foreground font-mono">
                  {room.capacity !== null && room.capacity !== undefined ? room.capacity : "—"}
                </TableCell>
                
                <!-- Location details -->
                <TableCell class="text-xs text-muted-foreground">
                  {#if room.building || room.floor !== null}
                    <div class="flex items-center gap-1.5 flex-wrap">
                      {#if room.building}
                        <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-bold">Bld {room.building}</span>
                      {/if}
                      {#if room.floor !== null}
                        <span class="bg-muted px-1.5 py-0.5 rounded text-[10px]">Floor {room.floor}</span>
                      {/if}
                    </div>
                  {:else}
                    <span class="text-muted-foreground/60">—</span>
                  {/if}
                </TableCell>
                
                <!-- Actions -->
                <TableCell class="text-end px-6">
                  <div class="flex items-center justify-end gap-1.5">
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title="Edit"
                      onclick={() => openEditSheet(room)}
                    >
                      <Pencil size={12} class="text-muted-foreground hover:text-foreground" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title="Delete"
                      onclick={() => handleDeleteRoom(room)}
                    >
                      <Trash2 size={12} class="text-muted-foreground hover:text-destructive" />
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    </div>
  {/if}
</div>

<!-- Modal Sheets -->
<RoomCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  room={editingRoom}
  onSuccess={loadRooms}
/>
