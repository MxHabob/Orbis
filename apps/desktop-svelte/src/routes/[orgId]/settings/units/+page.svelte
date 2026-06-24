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
import { Plus, Search, Pencil, Trash2, Network, School, Building2 } from "lucide-svelte";
import OrgUnitSheet from "$lib/components/sheets/org-unit-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let units = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters
let searchQuery = $state("");

// Sheet controls
let isSheetOpen = $state(false);
let editingUnit = $state<any>(null);

async function loadUnits() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.orgUnits.getAll(data.orgId);
    units = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error("Failed to load organizational units");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadUnits();
  }
});

const filteredUnits = $derived(
  units.filter((u) => {
    return (
      (u.name || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (u.name_ar || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (u.code || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
      (u.unit_type || "").toLowerCase().includes(searchQuery.toLowerCase())
    );
  })
);

function openCreateSheet() {
  editingUnit = null;
  isSheetOpen = true;
}

function openEditSheet(unit: any) {
  editingUnit = unit;
  isSheetOpen = true;
}

async function handleDeleteUnit(unit: any) {
  if (!confirm(`Are you sure you want to delete org unit "${unit.name_ar || unit.name}"?`)) return;

  try {
    await commands.orgUnits.delete(unit.id);
    toast.success("Organizational unit deleted successfully");
    loadUnits();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getUnitTypeBadgeClass(typeStr: string): string {
  switch (String(typeStr).toLowerCase()) {
    case "branch":
      return "bg-sky-500/10 text-sky-500 border-sky-500/20";
    case "building":
      return "bg-teal-500/10 text-teal-500 border-teal-500/20";
    case "department":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "classroom":
      return "bg-lime-500/10 text-lime-500 border-lime-500/20";
    case "office":
      return "bg-indigo-500/10 text-indigo-500 border-indigo-500/20";
    default:
      return "bg-slate-500/10 text-slate-500 border-slate-500/20";
  }
}
</script>

<svelte:head>
  <title>Organizational Units | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <span>{m.settings_nav_units()}</span>
        {#if !isLoading}
          <span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {filteredUnits.length}
          </span>
        {/if}
      </h1>
      <p class="text-xs text-muted-foreground">Manage organization units, sites, departments, and class locations</p>
    </div>

    <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      Add Unit
    </Button>
  </div>

  <!-- Search Bar -->
  <div class="relative w-full">
    <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
    <Input
      type="text"
      bind:value={searchQuery}
      placeholder="Search units by name, code or type..."
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
      <p class="text-xs text-destructive font-semibold">Failed to load organizational units</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadUnits}>{m.error_retry()}</Button>
    </div>
  {:else if filteredUnits.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <Network size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">No units found</p>
        <p class="text-[10px] text-muted-foreground/80">Add your first site, building, department, or classroom location.</p>
      </div>
    </div>
  {:else}
    <div class="border border-border rounded-xl bg-card/20 overflow-hidden" in:fade={{ duration: 150 }}>
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-transparent">
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Code</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Name</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Type</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Capacity</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Location</TableHead>
              <TableHead class="text-end text-xs font-bold text-muted-foreground px-6">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each filteredUnits as unit (unit.id)}
              <TableRow class="hover:bg-accent/5">
                <!-- Code -->
                <TableCell class="text-xs font-mono font-bold text-foreground">
                  {unit.code || "—"}
                </TableCell>
                
                <!-- Name -->
                <TableCell>
                  <div class="flex flex-col text-start">
                    <span class="text-xs font-bold text-foreground">
                      {getLocale() === "ar" && unit.name_ar ? unit.name_ar : unit.name}
                    </span>
                    {#if getLocale() === "ar" && unit.name_ar && unit.name}
                      <span class="text-[9px] text-muted-foreground leading-none mt-0.5">{unit.name}</span>
                    {/if}
                  </div>
                </TableCell>
                
                <!-- Unit Type -->
                <TableCell>
                  <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase {getUnitTypeBadgeClass(unit.unit_type)}">
                    {unit.unit_type}
                  </span>
                </TableCell>
                
                <!-- Capacity -->
                <TableCell class="text-xs text-muted-foreground font-mono">
                  {unit.capacity !== null && unit.capacity !== undefined ? unit.capacity : "—"}
                </TableCell>
                
                <!-- Location details -->
                <TableCell class="text-xs text-muted-foreground">
                  {#if unit.building || unit.floor !== null || unit.room_number}
                    <div class="flex items-center gap-1.5 flex-wrap">
                      {#if unit.building}
                        <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-bold">Bld {unit.building}</span>
                      {/if}
                      {#if unit.floor !== null}
                        <span class="bg-muted px-1.5 py-0.5 rounded text-[10px]">Floor {unit.floor}</span>
                      {/if}
                      {#if unit.room_number}
                        <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-mono">Room {unit.room_number}</span>
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
                      onclick={() => openEditSheet(unit)}
                    >
                      <Pencil size={12} class="text-muted-foreground hover:text-foreground" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      title="Delete"
                      onclick={() => handleDeleteUnit(unit)}
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
<OrgUnitSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  unit={editingUnit}
  onSuccess={loadUnits}
/>
