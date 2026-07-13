
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

<div class="w-full space-y-6">


  <Tabs
    value={onboardingState.activeBuildingTab}
    onValueChange={(v) => (onboardingState.activeBuildingTab = v ?? onboardingState.buildings[0])}
    class="w-full"
  >
    <TabsList class="w-full justify-start bg-transparent border-b border-border/60 rounded-none p-0 h-auto gap-2 overflow-x-auto flex-nowrap scrollbar-hide">
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
        <!-- Rooms List for the Active Building -->
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <h4 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider text-start">
              {m.onboarding_room_approved({ building: bld })}
            </h4>
            {#if onboardingState.buildings.length > 1}
              <Button
                variant="ghost"
                size="sm"
                class="text-muted-foreground hover:text-destructive h-8 px-3 text-xs flex gap-1.5 items-center rounded-lg"
                onclick={() => handleRemoveBuilding(bld)}
              >
                <Trash size={12} />
                <span>{m.onboarding_delete_building ? m.onboarding_delete_building() : "Delete Building"}</span>
              </Button>
            {/if}
          </div>

          <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-2xl bg-card shadow-sm">
            <div class="min-w-[800px] divide-y divide-border/60">
              <!-- Header Row -->
              <div class="grid grid-cols-[1.5fr_1.5fr_1.2fr_1fr_1fr_auto] gap-4 p-4 bg-muted/20 text-xs font-semibold text-muted-foreground select-none text-start">
                <span>{m.onboarding_rooms_table_name_ar()}</span>
                <span>{m.onboarding_rooms_table_name_en()}</span>
                <span>{m.onboarding_rooms_table_type()}</span>
                <span>{m.onboarding_rooms_table_floor()}</span>
                <span>{m.onboarding_rooms_table_capacity()}</span>
                <span class="w-8"></span>
              </div>

              <!-- Inline Add New Room Row -->
              <div class="grid grid-cols-[1.5fr_1.5fr_1.2fr_1fr_1fr_auto] gap-4 px-4 py-2.5 items-center bg-muted/5 transition-colors">
                <!-- Arabic Name -->
                <Input bind:value={newRoomNameAr} placeholder={m.onboarding_rooms_placeholder_ar()} class="h-8 text-xs font-bold font-sans bg-background/50 border-dashed border-border/80" />
                
                <!-- English Name -->
                <Input bind:value={newRoomName} placeholder={m.onboarding_rooms_placeholder_en()} class="h-8 text-xs font-mono text-left font-sans bg-background/50 border-dashed border-border/80" style="direction: ltr;" />
                
                <!-- Room Type Select -->
                <Select type="single" value={newRoomType} onValueChange={(v) => (newRoomType = v ?? "classroom")}>
                  <SelectTrigger class="h-8 text-xs bg-background/50 border-dashed border-border/80">
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

                <!-- Floor Select -->
                <Select type="single" value={String(newRoomFloor)} onValueChange={(v) => (newRoomFloor = Number(v ?? "1") || 1)}>
                  <SelectTrigger class="h-8 text-xs bg-background/50 border-dashed border-border/80">
                    {m.onboarding_room_floor_n({ n: newRoomFloor })}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="1">{m.onboarding_room_floor_n({ n: 1 })}</SelectItem>
                    <SelectItem value="2">{m.onboarding_room_floor_n({ n: 2 })}</SelectItem>
                    <SelectItem value="3">{m.onboarding_room_floor_n({ n: 3 })}</SelectItem>
                    <SelectItem value="4">{m.onboarding_room_floor_n({ n: 4 })}</SelectItem>
                  </SelectContent>
                </Select>

                <!-- Capacity Input -->
                <Input type="number" bind:value={newRoomCapacity} min="1" max="150" class="h-8 text-xs text-center font-bold font-mono bg-background/50 border-dashed border-border/80" />

                <!-- Add Button -->
                <Button variant="default" size="icon-sm" class="h-8 w-8 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 animate-pulse" onclick={handleAddRoom}>
                  <Plus size={14} />
                </Button>
              </div>

              {#each onboardingState.rooms.filter((r) => r.building === bld) as rm (rm.id)}
                <div class="grid grid-cols-[1.5fr_1.5fr_1.2fr_1fr_1fr_auto] gap-4 px-4 py-3 items-center hover:bg-muted/10 transition-colors">
                  <!-- Arabic Name -->
                  <Input bind:value={rm.name_ar} class="h-8 text-xs font-bold font-sans" onchange={() => onboardingState.save()} />
                  
                  <!-- English Name -->
                  <Input bind:value={rm.name} class="h-8 text-xs font-mono text-left font-sans" style="direction: ltr;" onchange={() => onboardingState.save()} />
                  
                  <!-- Room Type Select -->
                  <Select type="single" value={rm.room_type} onValueChange={(v) => { if (v) { rm.room_type = v; onboardingState.save(); } }}>
                    <SelectTrigger class="h-8 text-xs">
                      {#if rm.room_type === "classroom"}{m.onboarding_room_type_classroom()}
                      {:else if rm.room_type === "computer_lab"}{m.onboarding_room_type_computer()}
                      {:else if rm.room_type === "chemistry_lab"}{m.onboarding_room_type_chemistry()}
                      {:else if rm.room_type === "physics_lab"}{m.onboarding_room_type_physics()}
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

                  <!-- Floor Select -->
                  <Select type="single" value={String(rm.floor)} onValueChange={(v) => { if (v) { rm.floor = Number(v); onboardingState.save(); } }}>
                    <SelectTrigger class="h-8 text-xs">
                      {m.onboarding_room_floor_n({ n: rm.floor })}
                    </SelectTrigger>
                    <SelectContent class="text-xs">
                      <SelectItem value="1">{m.onboarding_room_floor_n({ n: 1 })}</SelectItem>
                      <SelectItem value="2">{m.onboarding_room_floor_n({ n: 2 })}</SelectItem>
                      <SelectItem value="3">{m.onboarding_room_floor_n({ n: 3 })}</SelectItem>
                      <SelectItem value="4">{m.onboarding_room_floor_n({ n: 4 })}</SelectItem>
                    </SelectContent>
                  </Select>

                  <!-- Capacity Input -->
                  <Input type="number" bind:value={rm.capacity} min="1" max="150" class="h-8 text-xs text-center font-bold font-mono" onchange={() => onboardingState.save()} />

                  <!-- Delete action -->
                  <Button variant="ghost" size="icon-sm" class="text-muted-foreground hover:text-destructive h-8 w-8 hover:bg-destructive/10" onclick={() => onboardingState.removeRoom(rm.id)}>
                    <Trash size={14} />
                  </Button>
                </div>
              {/each}
            </div>
          </div>

          <div class="flex justify-end pt-2">
            <Button
              variant="ghost"
              onclick={() => onboardingState.addDefaultRoomsToBuilding(bld)}
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
