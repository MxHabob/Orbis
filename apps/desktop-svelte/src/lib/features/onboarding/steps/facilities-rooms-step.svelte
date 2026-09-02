<!--
  Step 4: Facilities & Learning Spaces (المباني والمرافق والقاعات التعليمية)
  Adapts dynamically to the chosen Institution Type (School, University, Training Center, etc.)
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { Badge } from "@orbis/ui-svelte/badge";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
 import { FormField } from "@orbis/ui-svelte";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Button } from "@orbis/ui-svelte/button";
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
import {
  Plus,
  Trash2,
  Building2,
  School,
  GraduationCap,
  DoorOpen,
  Layers,
  Sparkles,
  Users,
} from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import AddBuildingDialog from "../components/add-building-dialog.svelte";

let newRoomName = $state("");
let newRoomNameAr = $state("");
let newRoomCapacity = $state(30);
let newRoomType = $state("classroom");
let newRoomFloor = $state(0);

const roomTypeOptions = [
  { value: "classroom", label: () => m.common_onboarding_room_type_classroom() },
  { value: "lecture_hall", label: () => m.common_onboarding_room_type_lecture_hall() },
  { value: "computer_lab", label: () => m.common_onboarding_room_type_computer() },
  { value: "science_lab", label: () => m.common_onboarding_room_type_science() },
  { value: "workshop", label: () => m.common_onboarding_room_type_workshop() },
  { value: "library", label: () => m.common_onboarding_room_type_library() },
  { value: "studio", label: () => m.common_onboarding_room_type_studio() },
  { value: "activity_hall", label: () => m.common_onboarding_room_type_hall() },
];

function getRoomTypeLabel(type: string) {
  return roomTypeOptions.find(o => o.value === type)?.label() || type;
}

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
    onboardingState.addBuilding(newBuildingName.trim());
    isAddDialogOpen = false;
    newBuildingName = "";
  }
}

function handleRemoveBuilding(bname: string) {
  buildingToDelete = bname;
  isDeleteDialogOpen = true;
}

function handleAddDefaultRooms() {
  const building = onboardingState.facilities.activeBuildingTab;
  if (building) onboardingState.addDefaultRoomsToBuilding(building);
}

function confirmRemoveBuilding() {
  if (buildingToDelete) {
    onboardingState.removeBuilding(buildingToDelete);
    isDeleteDialogOpen = false;
    buildingToDelete = "";
  }
}
</script>

<div class="w-full space-y-6 text-start">
  <!-- Header Description -->
  <div class="flex flex-col gap-1">
    <h3 class="text-sm font-bold text-foreground flex items-center gap-2">
      <Building2 size={18} class="text-primary" />
      {m.common_onboarding_facilities_title()}
    </h3>
    <p class="text-xs text-muted-foreground">
      {m.common_onboarding_facilities_desc()}
    </p>
  </div>

  <!-- Building Tabs -->
  <Tabs
    value={onboardingState.facilities.activeBuildingTab}
          onValueChange={(v: string | undefined) => (onboardingState.facilities.activeBuildingTab = v ?? onboardingState.facilities.buildings[0])}
    class="w-full"
  >
    <TabsList class="w-full justify-start bg-transparent border-b border-border/60 rounded-none p-0 h-auto gap-2 overflow-x-auto flex-nowrap scrollbar-hide">
      {#each onboardingState.facilities.buildings as bld}
        <TabsTrigger
          value={bld}
          class="px-5 py-3 font-semibold text-xs rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:text-foreground text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        >
          <div class="flex items-center gap-2">
            <Building2 size={14} class="text-primary" />
            <span>{bld}</span>
            <Badge variant="secondary" class="text-[10px] px-1.5 py-0">
              {onboardingState.facilities.rooms.filter((r) => r.building === bld).length} {m.common_onboarding_rooms_count_suffix()}
            </Badge>
          </div>
        </TabsTrigger>
      {/each}
      <Button
        variant="ghost"
        size="sm"
        class="shrink-0 h-9 my-auto ms-2 text-muted-foreground hover:text-primary gap-1.5 text-xs cursor-pointer"
        onclick={handleAddBuilding}
      >
        <Plus size={14} />
        {m.common_onboarding_add_building()}
      </Button>
    </TabsList>

    {#each onboardingState.facilities.buildings as bld}
      <TabsContent value={bld} class="space-y-4 pt-4">
        <!-- Building Action Bar -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <DoorOpen size={15} class="text-primary" />
            <h4 class="text-xs font-bold text-foreground">
              {m.common_onboarding_building_rooms({ building: bld })}
            </h4>
          </div>
          <div class="flex items-center gap-2">
            <Button variant="outline" size="sm" class="h-8 px-3 text-xs flex gap-1.5 items-center rounded-lg cursor-pointer" onclick={handleAddDefaultRooms}>
              <Sparkles size={13} />
              <span>{m.common_onboarding_room_reset_defaults()}</span>
            </Button>
            <Button
              variant="ghost"
              size="sm"
              class="text-muted-foreground hover:text-destructive h-8 px-3 text-xs flex gap-1.5 items-center rounded-lg cursor-pointer"
              onclick={() => handleRemoveBuilding(bld)}
            >
              <Trash2 size={13} />
              <span>{m.common_onboarding_delete_building()}</span>
            </Button>
          </div>
        </div>

        <!-- Rooms / Facilities Table -->
        <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-xl bg-card shadow-2xs">
          <div class="min-w-190 divide-y divide-border/40">
            <!-- Header Row -->
            <div class="grid grid-cols-[1.5fr_1.5fr_1.8fr_1fr_1fr_auto] gap-3 p-3.5 bg-muted/30 text-xs font-semibold text-muted-foreground select-none text-start">
              <span>{m.common_onboarding_rooms_table_name_ar()}</span>
              <span>{m.common_onboarding_rooms_table_name_en()}</span>
              <span>{m.common_onboarding_room_type()}</span>
              <span>{m.common_onboarding_rooms_table_floor()}</span>
              <span>{m.common_onboarding_room_capacity()}</span>
              <span class="w-8"></span>
            </div>

            <!-- Inline Add New Room Row -->
            <div class="grid grid-cols-[1.5fr_1.5fr_1.8fr_1fr_1fr_auto] gap-3 px-3.5 py-2.5 items-center bg-primary/5 border-b border-border/40">
              <!-- Arabic Name -->
              <FormField label={m.common_onboarding_rooms_table_name_ar()} required>{#snippet children()}<Input bind:value={newRoomNameAr} placeholder={m.common_onboarding_rooms_placeholder_ar()} class="h-8 text-xs font-sans bg-background" />{/snippet}</FormField>

              <!-- English Name -->
              <FormField label={m.common_onboarding_rooms_table_name_en()} required>{#snippet children()}<Input bind:value={newRoomName} placeholder={m.common_onboarding_rooms_placeholder_en()} class="h-8 text-xs font-mono bg-background" style="direction: ltr;" />{/snippet}</FormField>

              <!-- Room Type Select -->
              <FormField label={m.common_onboarding_room_type()} required>{#snippet children()}<Select type="single" value={newRoomType} onValueChange={(v: string | undefined) => { if (v) newRoomType = v; }}>
                <SelectTrigger class="h-8 text-xs bg-background">
                  {getRoomTypeLabel(newRoomType)}
                </SelectTrigger>
                <SelectContent class="text-xs">
                    {#each roomTypeOptions as opt}
                    <SelectItem value={opt.value}>{opt.label()}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>{/snippet}</FormField>

              <FormField label={m.common_onboarding_rooms_table_floor()} required>{#snippet children()}<Input type="number" bind:value={newRoomFloor} min="0" step="1" placeholder={m.common_onboarding_room_floor_placeholder()} class="h-8 text-xs text-center font-mono bg-background" />{/snippet}</FormField>

              <!-- Capacity Input -->
              <FormField label={m.common_onboarding_room_capacity()} required>{#snippet children()}<Input type="number" bind:value={newRoomCapacity} min="1" max="500" class="h-8 text-xs text-center font-mono bg-background" />{/snippet}</FormField>

              <!-- Add Button -->
              <Button
                variant="default"
                size="icon-sm"
                class="h-8 w-8 rounded-lg cursor-pointer"
                onclick={handleAddRoom}
                  title={m.common_onboarding_add_room()}
              >
                <Plus size={14} />
              </Button>
            </div>

            <!-- Existing Rooms in this building -->
            {#each onboardingState.facilities.rooms.filter((r) => r.building === bld) as rm (rm.id)}
              <div class="grid grid-cols-[1.5fr_1.5fr_1.8fr_1fr_1fr_auto] gap-3 px-3.5 py-2.5 items-center hover:bg-muted/10 transition-colors">
                <!-- Arabic Name -->
                <Input
                  bind:value={rm.name_ar}
                  class="h-8 text-xs font-sans"
                  onchange={() => onboardingState.save()}
                />

                <!-- English Name -->
                <Input
                  bind:value={rm.name}
                  class="h-8 text-xs font-mono"
                  style="direction: ltr;"
                  onchange={() => onboardingState.save()}
                />

                <!-- Room Type Select -->
                <Select type="single" value={rm.room_type} onValueChange={(v: string | undefined) => { if (v) { rm.room_type = v; onboardingState.save(); } }}>
                  <SelectTrigger class="h-8 text-xs">
                    {getRoomTypeLabel(rm.room_type)}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    {#each roomTypeOptions as opt}
                    <SelectItem value={opt.value}>{opt.label()}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>

                <Input type="number" bind:value={rm.floor} min="0" step="1" placeholder={m.common_onboarding_room_floor_placeholder()} class="h-8 text-xs text-center font-mono" onchange={() => onboardingState.save()} />

                <!-- Capacity Input -->
                <Input
                  type="number"
                  bind:value={rm.capacity}
                  min="1"
                  max="500"
                  class="h-8 text-xs text-center font-mono"
                  onchange={() => onboardingState.save()}
                />

                <!-- Delete action -->
                <Button
                  variant="ghost"
                  size="icon-sm"
                  class="text-muted-foreground hover:text-destructive h-8 w-8 hover:bg-destructive/10 cursor-pointer"
                  onclick={() => onboardingState.removeRoom(rm.id)}
                  title={m.common_onboarding_delete_room()}
                >
                  <Trash2 size={13} />
                </Button>
              </div>
            {:else}
              <div class="p-8 text-center text-xs text-muted-foreground">
                {m.common_onboarding_no_rooms_in_building()}
              </div>
            {/each}
          </div>
        </div>
      </TabsContent>
    {/each}
  </Tabs>

  <!-- Add Building Dialog -->
  <AddBuildingDialog bind:open={isAddDialogOpen} bind:name={newBuildingName} onsubmit={confirmAddBuilding} />

  <!-- Delete Building Alert Dialog -->
  <AlertDialog bind:open={isDeleteDialogOpen}>
    <AlertDialogContent class="max-w-sm">
      <AlertDialogHeader>
        <AlertDialogTitle class="text-sm">{m.common_onboarding_delete_building()}</AlertDialogTitle>
        <AlertDialogDescription class="text-xs leading-relaxed">
          {m.common_onboarding_delete_building_desc({ building: buildingToDelete })}
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter class="flex items-center justify-end gap-2 pt-2">
        <AlertDialogCancel class="text-xs cursor-pointer">{m.common_action_cancel()}</AlertDialogCancel>
        <AlertDialogAction class="text-xs bg-destructive text-destructive-foreground hover:bg-destructive/90 cursor-pointer" onclick={confirmRemoveBuilding}>
          {m.common_onboarding_confirm_delete()}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</div>
