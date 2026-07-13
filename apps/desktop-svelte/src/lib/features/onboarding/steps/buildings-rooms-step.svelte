
<script lang="ts">
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { Badge } from "@orbis/ui-svelte/badge";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Button } from "@orbis/ui-svelte/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogCancel,
  AlertDialogAction,
} from "@orbis/ui-svelte/alert-dialog";
import { Plus, Trash } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";

let newRoomName = $state("");
let newRoomNameAr = $state("");
let newRoomCapacity = $state(30);
let newRoomType = $state("classroom");
let newRoomFloor = $state(1);

function handleAddRoom() {
  const success = onboardingState.addRoom(
    newRoomName,
    newRoomNameAr,
    newRoomCapacity,
    newRoomType,
    newRoomFloor,
  );
  if (success) {
    newRoomName = "";
    newRoomNameAr = "";
  }
}

let isAddDialogOpen = $state(false);
let newBuildingName = $state("");

let isDeleteDialogOpen = $state(false);
let buildingToDelete = $state("");

function handleAddBuilding() {
  isAddDialogOpen = true;
}

function confirmAddBuilding() {
  if (newBuildingName.trim()) {
    onboardingState.addBuilding(newBuildingName);
    isAddDialogOpen = false;
    newBuildingName = "";
  }
}

function handleRemoveBuilding(bname: string) {
  buildingToDelete = bname;
  isDeleteDialogOpen = true;
}

function confirmRemoveBuilding() {
  if (buildingToDelete) {
    onboardingState.removeBuilding(buildingToDelete);
    isDeleteDialogOpen = false;
    buildingToDelete = "";
  }
}
</script>

<div class="max-w-5xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_buildings_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_buildings_desc()}
    </p>
  </div>

  <Tabs
    value={onboardingState.activeBuildingTab}
    onValueChange={(v) => (onboardingState.activeBuildingTab = v ?? onboardingState.buildings[0])}
    class="w-full"
  >
    <TabsList class="w-full justify-start bg-muted/50 border-b border-border rounded-none p-0 h-auto gap-2 overflow-x-auto flex-nowrap scrollbar-hide">
      {#each onboardingState.buildings as bld}
        <TabsTrigger
          value={bld}
          class="px-5 py-3 font-semibold text-sm rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:text-foreground text-muted-foreground hover:text-foreground shrink-0"
        >
          {bld}
          <Badge variant="secondary" class="mr-2 ml-2 text-[10px]">
            {onboardingState.rooms.filter((r) => r.building === bld).length} {m.onboarding_rooms_count_suffix()}
          </Badge>
        </TabsTrigger>
      {/each}
      <Button variant="ghost" size="sm" class="shrink-0 h-9 my-auto ml-2 text-muted-foreground hover:text-primary gap-1" onclick={handleAddBuilding}>
        <Plus size={16} />
        {m.onboarding_add_building()}
      </Button>
    </TabsList>

    {#each onboardingState.buildings as bld}
      <TabsContent value={bld} class="space-y-6 pt-4">
        {#if onboardingState.buildings.length > 1}
          <div class="flex justify-end">
            <Button variant="destructive" size="sm" class="gap-1 h-8 text-xs" onclick={() => handleRemoveBuilding(bld)}>
              <Trash size={14} />
              {m.onboarding_delete_building()}
            </Button>
          </div>
        {/if}
        <!-- Add Room Form for current Building -->
        <div class="p-6 bg-muted/40 rounded-xl border border-border grid grid-cols-1 md:grid-cols-5 gap-4 items-end">
          <div class="space-y-2 md:col-span-2">
            <Label class="text-xs font-semibold">
              {m.onboarding_room_name_hint()}
            </Label>
            <div class="grid grid-cols-2 gap-2">
              <Input bind:value={newRoomNameAr} placeholder="فصل 1-05" class="text-xs h-9" />
              <Input bind:value={newRoomName} placeholder="Classroom 1-05" class="text-xs h-9 text-left font-sans" style="direction: ltr;" />
            </div>
          </div>

          <div class="space-y-2 text-start">
            <Label class="text-xs font-semibold">{m.onboarding_room_type()}</Label>
            <Select type="single" value={newRoomType} onValueChange={(v) => (newRoomType = v ?? "classroom")}>
              <SelectTrigger class="w-full h-9 text-xs">
                {#if newRoomType === "classroom"}{m.onboarding_room_type_classroom()}
                {:else if newRoomType === "computer_lab"}{m.onboarding_room_type_computer()}
                {:else if newRoomType === "chemistry_lab"}{m.onboarding_room_type_chemistry()}
                {:else if newRoomType === "physics_lab"}{m.onboarding_room_type_physics()}
                {:else}{m.onboarding_room_type_hall()}
                {/if}
              </SelectTrigger>
              <SelectContent class="text-xs">
                <SelectItem value="classroom">{m.onboarding_room_type_classroom()}</SelectItem>
                <SelectItem value="computer_lab">{m.onboarding_room_type_computer()}</SelectItem>
                <SelectItem value="chemistry_lab">{m.onboarding_room_type_chemistry()}</SelectItem>
                <SelectItem value="physics_lab">{m.onboarding_room_type_physics()}</SelectItem>
                <SelectItem value="hall">{m.onboarding_room_type_hall()}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div class="space-y-2 text-start">
            <Label class="text-xs font-semibold">
              {m.onboarding_room_floor_capacity()}
            </Label>
            <div class="grid grid-cols-2 gap-2">
              <Select type="single" value={String(newRoomFloor)} onValueChange={(v) => (newRoomFloor = Number(v ?? "1"))}>
                <SelectTrigger class="w-full h-9 text-xs">
                  {m.onboarding_room_floor_n({ n: newRoomFloor })}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="1">{m.onboarding_room_floor_n({ n: 1 })}</SelectItem>
                  <SelectItem value="2">{m.onboarding_room_floor_n({ n: 2 })}</SelectItem>
                  <SelectItem value="3">{m.onboarding_room_floor_n({ n: 3 })}</SelectItem>
                  <SelectItem value="4">{m.onboarding_room_floor_n({ n: 4 })}</SelectItem>
                </SelectContent>
              </Select>
              <Input type="number" bind:value={newRoomCapacity} min="10" max="100" class="text-xs h-9 font-sans" />
            </div>
          </div>

          <Button onclick={handleAddRoom} class="w-full h-9 text-xs flex gap-1.5 items-center justify-center">
            <Plus class="w-4 h-4" />
            <span>{m.onboarding_add_room_title()}</span>
          </Button>
        </div>

        <!-- Rooms List for the Active Building -->
        <div class="space-y-3">
          <h4 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider text-start">
            {m.onboarding_room_approved({ building: bld })}
          </h4>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each onboardingState.rooms.filter((r) => r.building === bld) as rm (rm.id)}
              <div class="bg-card border border-border rounded-xl p-4 flex justify-between items-start hover:border-muted-foreground/30 transition-colors">
                <div class="space-y-1 text-start">
                  <div class="flex items-center gap-2">
                    <span class="font-semibold text-foreground text-sm">{rm.name_ar}</span>
                    <span class="text-xs text-muted-foreground font-mono">({rm.name})</span>
                  </div>
                  <div class="flex items-center gap-2 mt-2">
                    <Badge variant="outline" class="text-[10px] text-primary border-primary/20 bg-primary/5">
                      {#if rm.room_type === "classroom"}{m.onboarding_room_type_classroom()}
                      {:else if rm.room_type === "computer_lab"}{m.onboarding_room_type_computer()}
                      {:else if rm.room_type === "chemistry_lab"}{m.onboarding_room_type_chemistry()}
                      {:else if rm.room_type === "physics_lab"}{m.onboarding_room_type_physics()}
                      {:else}{m.onboarding_room_type_hall()}
                      {/if}
                    </Badge>
                    <span class="text-xs text-muted-foreground">
                      {m.onboarding_room_floor_label({ floor: rm.floor })}
                    </span>
                    <span class="text-xs text-muted-foreground">
                      {rm.capacity} {m.onboarding_room_capacity_unit()}
                    </span>
                  </div>
                </div>

                <button
                  onclick={() => onboardingState.removeRoom(rm.id)}
                  class="text-muted-foreground hover:text-destructive p-1.5 hover:bg-destructive/10 rounded-lg transition-colors"
                >
                  <Trash class="w-4 h-4" />
                </button>
              </div>
            {/each}
          </div>

          <div class="flex justify-end pt-2">
            <Button
              variant="ghost"
              onclick={() => onboardingState.resetRoomsToDefault()}
              class="text-muted-foreground hover:text-foreground text-xs"
            >
              {m.onboarding_room_reset_defaults()}
            </Button>
          </div>
        </div>
      </TabsContent>
    {/each}
  </Tabs>

  <!-- Add Building Dialog -->
  <Dialog bind:open={isAddDialogOpen}>
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle>{m.onboarding_add_building()}</DialogTitle>
      </DialogHeader>
      <div class="py-4 space-y-4">
        <div class="space-y-2">
          <Label>{m.onboarding_add_building_name()}</Label>
          <Input bind:value={newBuildingName} placeholder={m.onboarding_add_building_placeholder()} class="text-sm" />
        </div>
      </div>
      <DialogFooter>
        <Button variant="outline" onclick={() => (isAddDialogOpen = false)}>{m.onboarding_cancel()}</Button>
        <Button onclick={confirmAddBuilding}>{m.onboarding_add()}</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <!-- Delete Building Alert -->
  <AlertDialog bind:open={isDeleteDialogOpen}>
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>{m.onboarding_delete_building_confirm()}</AlertDialogTitle>
        <AlertDialogDescription>
          {@html m.onboarding_delete_building_desc({ building: buildingToDelete })}
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>{m.onboarding_cancel()}</AlertDialogCancel>
        <AlertDialogAction class="bg-destructive text-destructive-foreground hover:bg-destructive/90" onclick={confirmRemoveBuilding}>
          {m.onboarding_delete()}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</div>
