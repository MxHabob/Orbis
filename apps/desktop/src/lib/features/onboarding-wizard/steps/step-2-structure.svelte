<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, type OrgUnitDto, type RoomDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingUnit = $state(false);
let isSubmittingRoom = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Existing Data
let orgUnits = $state<OrgUnitDto[]>([]);
let rooms = $state<RoomDto[]>([]);

// Branch Creation State
let unitName = $state("");
let unitNameAr = $state("");
let unitCode = $state("BR-01");
let unitType = $state("branch");
let unitCapacity = $state<number>(500);

// Room Creation State
let roomName = $state("");
let roomNameAr = $state("");
let roomNumber = $state("R-101");
let roomType = $state("classroom");
let roomCapacity = $state<number>(30);
let roomBuilding = $state("");

const ROOM_TYPES = [
  { value: "classroom", label: m.onboarding_room_type_classroom() },
  { value: "lab", label: m.onboarding_room_type_lab() },
  { value: "auditorium", label: m.onboarding_room_type_auditorium() },
  { value: "library", label: m.onboarding_room_type_library() },
  { value: "sports_hall", label: m.onboarding_room_type_sports_hall() },
  { value: "other", label: m.onboarding_room_type_other() },
];

const stepTasks = $derived(getTasksForStep("structure", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [unitsRes, roomsRes] = await Promise.all([
      api.listOrgUnits().catch(() => []),
      api.listRooms().catch(() => []),
    ]);
    orgUnits = unitsRes;
    rooms = roomsRes;
  } catch (err: any) {
    console.error("Failed to load structure data:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isLoading = false;
  }
}

async function handleCreateUnit() {
  if (!unitName.trim() && !unitNameAr.trim()) {
    errorMessage = m.onboarding_step2_branch_err_name();
    return;
  }

  isSubmittingUnit = true;
  errorMessage = null;
  successMessage = null;

  try {
    const newUnit = await api.createOrgUnit({
      name: unitName.trim() || unitNameAr.trim(),
      name_ar: unitNameAr.trim() || undefined,
      code: unitCode.trim() || undefined,
      unit_type: unitType,
      capacity: unitCapacity > 0 ? unitCapacity : undefined,
      is_active: true,
    });

    orgUnits = [...orgUnits, newUnit];
    unitName = "";
    unitNameAr = "";
    unitCode = `BR-0${orgUnits.length + 1}`;
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step2_branch_created_success();
  } catch (err: any) {
    console.error("Failed to create unit:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingUnit = false;
  }
}

async function handleCreateRoom() {
  if (!roomName.trim() && !roomNameAr.trim() && !roomNumber.trim()) {
    errorMessage = m.onboarding_step2_room_err_name();
    return;
  }

  isSubmittingRoom = true;
  errorMessage = null;
  successMessage = null;

  try {
    const newRoom = await api.createRoom({
      name: roomName.trim() || roomNameAr.trim() || roomNumber.trim(),
      name_ar: roomNameAr.trim() || undefined,
      room_number: roomNumber.trim() || "R-101",
      room_type: roomType,
      capacity: roomCapacity > 0 ? roomCapacity : 30,
      building: roomBuilding.trim() || undefined,
      is_active: true,
    });

    rooms = [...rooms, newRoom];
    roomName = "";
    roomNameAr = "";
    roomNumber = `R-10${rooms.length + 1}`;
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step2_room_created_success();
  } catch (err: any) {
    console.error("Failed to create room:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSubmittingRoom = false;
  }
}

async function handleContinue() {
  if (orgUnits.length === 0 && (unitName.trim() || unitNameAr.trim())) {
    await handleCreateUnit();
  }
  await navigateOnboardingRoute(`/${orgId}/onboarding/calendar`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={2}
    title={m.onboarding_step2_title()}
    description={m.onboarding_step2_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : orgUnits.length > 0 ? "in_progress" : "not_started"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <span class="size-4 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
      <span>{m.common_control_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Branches & Campuses -->
    <OnboardingSection
      title={m.onboarding_step2_branch_title()}
      description={m.onboarding_step2_branch_desc()}
      badge={orgUnits.length > 0 ? `${orgUnits.length}` : undefined}
      isComplete={orgUnits.length > 0}
    >
      <div class="space-y-4">
        <!-- Branch Creation Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_branch_name_en()}
              </Label>
              <Input
                bind:value={unitName}
                placeholder={m.onboarding_step2_branch_name_en_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_branch_name_ar()}
              </Label>
              <Input
                bind:value={unitNameAr}
                placeholder={m.onboarding_step2_branch_name_ar_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_branch_code()}
              </Label>
              <Input
                bind:value={unitCode}
                placeholder="BR-01"
                class="font-mono text-xs"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_branch_capacity()}
              </Label>
              <Input
                type="number"
                bind:value={unitCapacity}
                placeholder="500"
              />
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingUnit || (!unitName.trim() && !unitNameAr.trim())}
              onclick={handleCreateUnit}
              class="text-xs font-semibold"
            >
              {isSubmittingUnit ? m.onboarding_step2_adding_btn() : m.onboarding_step2_add_branch_btn()}
            </Button>
          </div>
        </div>

        <!-- Existing Branches List -->
        {#if orgUnits.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step2_configured_branches()}
            </span>
            <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
              {#each orgUnits as unit (unit.id)}
                <div class="flex items-center justify-between p-3 text-xs">
                  <div class="flex items-center gap-3">
                    <span class="flex size-7 items-center justify-center rounded-md bg-primary/10 font-bold text-primary text-[11px]">
                      🏢
                    </span>
                    <div>
                      <div class="font-medium text-foreground">
                        {unit.name_ar || unit.name}
                      </div>
                      <div class="text-[11px] font-mono text-muted-foreground">
                        {unit.code || "—"} | {unit.unit_type || "branch"}
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    {#if unit.capacity}
                      <Badge variant="secondary" class="text-[10px]">
                        {unit.capacity} {m.onboarding_step2_students_suffix()}
                      </Badge>
                    {/if}
                    <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                      {m.onboarding_active_badge()}
                    </Badge>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Section 2: Rooms & Physical Facilities -->
    <OnboardingSection
      title={m.onboarding_step2_room_title()}
      description={m.onboarding_step2_room_desc()}
      badge={rooms.length > 0 ? `${rooms.length}` : undefined}
      isComplete={rooms.length > 0}
    >
      <div class="space-y-4">
        <!-- Room Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-3">
            <div class="space-y-1.5 sm:col-span-2">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_room_name()}
              </Label>
              <Input
                bind:value={roomName}
                placeholder={m.onboarding_step2_room_name_placeholder()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_room_number()}
              </Label>
              <Input
                bind:value={roomNumber}
                placeholder="R-101"
                class="font-mono text-xs"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_room_type()}
              </Label>
              <Select type="single" bind:value={roomType}>
                <SelectTrigger class="w-full">
                  {ROOM_TYPES.find((r) => r.value === roomType)?.label || roomType}
                </SelectTrigger>
                <SelectContent>
                  {#each ROOM_TYPES as r (r.value)}
                    <SelectItem value={r.value}>{r.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_room_capacity()}
              </Label>
              <Input
                type="number"
                bind:value={roomCapacity}
                placeholder="30"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step2_room_building()}
              </Label>
              <Input
                bind:value={roomBuilding}
                placeholder={m.onboarding_step2_room_building_placeholder()}
              />
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingRoom || (!roomName.trim() && !roomNumber.trim())}
              onclick={handleCreateRoom}
              class="text-xs font-semibold"
            >
              {isSubmittingRoom ? m.onboarding_step2_adding_btn() : m.onboarding_step2_add_room_btn()}
            </Button>
          </div>
        </div>

        <!-- Existing Rooms List -->
        {#if rooms.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step2_configured_rooms()}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each rooms as room (room.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div class="flex items-center gap-2">
                    <span class="flex size-6 items-center justify-center rounded bg-primary/10 text-primary text-[10px] font-bold">
                      🚪
                    </span>
                    <div>
                      <div class="font-medium text-foreground">{room.name_ar || room.name || room.room_number}</div>
                      <div class="text-[10px] text-muted-foreground capitalize">
                        {room.room_type || "classroom"} {room.building ? `(${room.building})` : ""}
                      </div>
                    </div>
                  </div>
                  <Badge variant="secondary" class="text-[10px]">
                    {room.capacity || 30} {m.onboarding_step2_seats_suffix()}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Bottom Actions Bar -->
    <div class="sticky bottom-0 z-10 -mx-4 flex items-center justify-between border-t border-border/60 bg-background/95 px-4 py-4 backdrop-blur sm:-mx-8 sm:px-8 lg:-mx-12 lg:px-12">
      <Button
        variant="outline"
        onclick={() => navigateOnboardingRoute(`/${orgId}/onboarding/organization`)}
        class="text-xs"
      >
        {m.onboarding_step2_back_btn()}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="default"
          disabled={orgUnits.length === 0 && !unitName.trim()}
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {m.onboarding_step2_continue_btn()}
        </Button>
      </div>
    </div>
  {/if}
</div>
