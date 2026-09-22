<!--
  Step 3: Facilities & Learning Spaces (المباني والمرافق والقاعات التعليمية)
  Clean, minimalist layout for configuring buildings and rooms.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Button } from "@orbis/ui/button";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogCancel,
  AlertDialogAction,
} from "@orbis/ui/alert-dialog";
import { Grid, Plus, Sparkles, Trash } from "@orbis/icons";

import { onboardingState } from "../state.svelte";
import AddBuildingDialog from "../components/add-building-dialog.svelte";
import { l } from "$lib/core/i18n";

let newRoomName = $state("");
let newRoomNameAr = $state("");
let newRoomCapacity = $state(30);
let newRoomType = $state("classroom");
let newRoomFloor = $state(0);

const roomTypeOptions = [
  { value: "classroom", label: () => m.common_onboarding_room_type_classroom() },
  { value: "virtual_classroom", label: () => l("قاعة افتراضية (عن بُعد)", "Virtual Classroom (Online)") },
  { value: "lecture_hall", label: () => m.common_onboarding_room_type_lecture_hall() },
  { value: "computer_lab", label: () => m.common_onboarding_room_type_computer() },
  { value: "science_lab", label: () => m.common_onboarding_room_type_science() },
  { value: "workshop", label: () => m.common_onboarding_room_type_workshop() },
  { value: "library", label: () => m.common_onboarding_room_type_library() },
  { value: "studio", label: () => m.common_onboarding_room_type_studio() },
  { value: "activity_hall", label: () => m.common_onboarding_room_type_hall() },
];

function getRoomTypeLabel(type: string) {
  return roomTypeOptions.find((o) => o.value === type)?.label() || type;
}

function handleAddRoom() {
  const nameAr = newRoomNameAr.trim() || newRoomName.trim();
  const nameEn = newRoomName.trim() || newRoomNameAr.trim();
  if (!nameAr) return;

  const success = onboardingState.addRoom(
    nameEn,
    nameAr,
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

<div class="w-full h-full flex flex-col min-h-0 space-y-3 text-start animate-in fade-in duration-300">
  <!-- Building Tabs -->
  <Tabs
    value={onboardingState.facilities.activeBuildingTab}
    onValueChange={(v: string | undefined) => (onboardingState.facilities.activeBuildingTab = v ?? onboardingState.facilities.buildings[0])}
    class="w-full h-full flex flex-col min-h-0"
  >
    <div class="flex items-center justify-between border-b border-border/60 pb-1 shrink-0">
      <TabsList class="justify-start bg-transparent p-0 h-auto gap-2 overflow-x-auto flex-nowrap scrollbar-hide">
        {#each onboardingState.facilities.buildings as bld}
          <TabsTrigger
            value={bld}
            class="px-3.5 py-1.5 font-bold text-xs rounded-xl border border-transparent data-[state=active]:border-primary/30 data-[state=active]:bg-primary/10 data-[state=active]:text-primary text-muted-foreground hover:text-foreground shrink-0 cursor-pointer transition-all duration-150"
          >
            <div class="flex items-center gap-1.5">
              <span>{bld}</span>
              <Badge variant="secondary" class="text-[9px] px-1.5 py-0 font-mono">
                {onboardingState.facilities.rooms.filter((r) => r.building === bld).length}
              </Badge>
            </div>
          </TabsTrigger>
        {/each}
      </TabsList>

      <Button
        variant="ghost"
        size="sm"
        class="shrink-0 h-8 text-xs text-muted-foreground hover:text-primary gap-1 cursor-pointer font-bold"
        onclick={handleAddBuilding}
      >
        <Plus size={13} />
        {m.common_onboarding_add_building()}
      </Button>
    </div>

    {#each onboardingState.facilities.buildings as bld}
      <TabsContent value={bld} class="flex-1 flex flex-col min-h-0 space-y-2.5 pt-1.5">
        <!-- Building Action Bar -->
        <div class="flex items-center justify-between shrink-0">
          <h4 class="text-xs font-bold text-foreground">
            {m.common_onboarding_building_rooms({ building: bld })}
          </h4>

          <div class="flex items-center gap-2">
            <Button variant="outline" size="xs" class="h-7 text-[11px] flex gap-1.5 items-center rounded-lg cursor-pointer" onclick={handleAddDefaultRooms}>
              <Sparkles size={12} />
              <span>{m.common_onboarding_room_reset_defaults()}</span>
            </Button>
            {#if onboardingState.facilities.buildings.length > 1}
              <Button
                variant="ghost"
                size="xs"
                class="text-muted-foreground hover:text-destructive h-7 text-[11px] flex gap-1 items-center rounded-lg cursor-pointer"
                onclick={() => handleRemoveBuilding(bld)}
              >
                <Trash size={12} />
                <span>{m.common_onboarding_delete_building()}</span>
              </Button>
            {/if}
          </div>
        </div>

        <!-- Rooms / Facilities Grid with flex-1 and scrollable body filling available height -->
        <div class="w-full flex-1 flex flex-col min-h-0 border border-border/70 rounded-2xl bg-card shadow-2xs overflow-hidden">
          
          <!-- Inline Add New Room Row (Fixed top header) -->
          <div class="grid grid-cols-[1.5fr_1.5fr_1.5fr_0.8fr_0.8fr_auto] gap-2.5 p-2.5 items-center bg-muted/20 border-b border-border/40 shrink-0">
            <FormField label={m.common_onboarding_rooms_table_name_ar()} required>
              {#snippet children()}<Input bind:value={newRoomNameAr} placeholder="قاعة 101" class="h-7.5 text-xs font-bold font-sans bg-background" />{/snippet}
            </FormField>

            <FormField label={m.common_onboarding_rooms_table_name_en()}>
              {#snippet children()}<Input bind:value={newRoomName} placeholder="Room 101" class="h-7.5 text-xs font-sans bg-background text-left" style="direction: ltr;" />{/snippet}
            </FormField>

            <FormField label={m.common_onboarding_room_type()} required>
              {#snippet children()}
                <Select type="single" value={newRoomType} onValueChange={(v: string | undefined) => { if (v) newRoomType = v; }}>
                  <SelectTrigger class="h-7.5 text-xs bg-background">
                    {getRoomTypeLabel(newRoomType)}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    {#each roomTypeOptions as opt}
                      <SelectItem value={opt.value}>{opt.label()}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>

            <FormField label={m.common_onboarding_rooms_table_floor()} required>
              {#snippet children()}<Input type="number" bind:value={newRoomFloor} min="0" step="1" class="h-7.5 text-xs text-center font-mono bg-background" />{/snippet}
            </FormField>

            <FormField label={m.common_onboarding_room_capacity()} required>
              {#snippet children()}<Input type="number" bind:value={newRoomCapacity} min="1" max="500" class="h-7.5 text-xs text-center font-mono bg-background" />{/snippet}
            </FormField>

            <div class="flex items-end pt-4">
              <Button
                variant="default"
                size="icon-sm"
                class="h-7.5 w-7.5 rounded-lg cursor-pointer shrink-0"
                onclick={handleAddRoom}
                disabled={!newRoomNameAr.trim() && !newRoomName.trim()}
                title={m.common_onboarding_add_room()}
              >
                <Plus size={13} />
              </Button>
            </div>
          </div>

          <!-- Existing Rooms in this building -->
          <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide divide-y divide-border/30">
            {#each onboardingState.facilities.rooms.filter((r) => r.building === bld) as rm (rm.id)}
              <div class="grid grid-cols-[1.5fr_1.5fr_1.5fr_0.8fr_0.8fr_auto] gap-2.5 px-3 py-2 items-center hover:bg-muted/10 transition-colors text-xs">
                <Input
                  bind:value={rm.name_ar}
                  class="h-7.5 text-xs font-bold font-sans bg-background"
                  onchange={() => onboardingState.save()}
                />

                <Input
                  bind:value={rm.name}
                  class="h-7.5 text-xs font-sans bg-background text-left"
                  style="direction: ltr;"
                  onchange={() => onboardingState.save()}
                />

                <Select type="single" value={rm.room_type} onValueChange={(v: string | undefined) => { if (v) { rm.room_type = v; onboardingState.save(); } }}>
                  <SelectTrigger class="h-7.5 text-xs bg-background">
                    {getRoomTypeLabel(rm.room_type)}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    {#each roomTypeOptions as opt}
                      <SelectItem value={opt.value}>{opt.label()}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>

                <Input type="number" bind:value={rm.floor} min="0" step="1" class="h-7.5 text-xs text-center font-mono bg-background" onchange={() => onboardingState.save()} />

                <Input
                  type="number"
                  bind:value={rm.capacity}
                  min="1"
                  max="500"
                  class="h-7.5 text-xs text-center font-mono bg-background"
                  onchange={() => onboardingState.save()}
                />

                <Button
                  variant="ghost"
                  size="icon-sm"
                  class="text-muted-foreground hover:text-destructive h-7.5 w-7.5 rounded-md cursor-pointer"
                  onclick={() => onboardingState.removeRoom(rm.id)}
                  title={m.common_onboarding_delete_room()}
                >
                  <Trash size={13} />
                </Button>
              </div>
            {:else}
              <div class="py-12 text-center text-xs text-muted-foreground">
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

