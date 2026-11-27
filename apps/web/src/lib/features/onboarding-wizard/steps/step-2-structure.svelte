<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type OrgUnitDto, type RoomDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingSection from "../onboarding-section.svelte";
import OnboardingPresetBar from "../components/onboarding-preset-bar.svelte";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import DeleteRecordButton from "../components/delete-record-button.svelte";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmittingUnit = $state(false);
let isSubmittingRoom = $state(false);
let successMessage = $state<string | null>(null);

// Existing Data
let orgUnits = $state<OrgUnitDto[]>([]);
let rooms = $state<RoomDto[]>([]);

// Branch Creation State
let unitName = $state("");
let unitNameAr = $state("");
let unitCode = $state("");
let unitType = $state("branch");
let unitCapacity = $state<number>(0);

// Room Creation State
let roomName = $state("");
let roomNameAr = $state("");
let roomType = $state("classroom");
let roomCapacity = $state<number>(0);
let isExamHall = $state(false);

const ROOM_TYPES = [
  { value: "classroom", label: m.onboarding_room_type_classroom() },
  { value: "lab", label: m.onboarding_room_type_lab() },
  { value: "auditorium", label: m.onboarding_room_type_auditorium() },
  { value: "library", label: m.onboarding_room_type_library() },
  { value: "sports_hall", label: m.onboarding_room_type_sports_hall() },
  { value: "other", label: m.onboarding_room_type_other() },
];

function applyBranchPreset() {
  unitCode = `BR-${String(orgUnits.length + 1).padStart(2, "0")}`;
  unitType = "branch";
  unitCapacity = 500;
}

function applyRoomPreset() {
  roomName = `R-${String(rooms.length + 101).padStart(3, "0")}`;
  roomType = "classroom";
  roomCapacity = 30;
}

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [unitsRes, roomsRes] = await Promise.all([
      api.listOrgUnits(),
      api.listRooms(),
    ]);
    orgUnits = unitsRes;
    rooms = roomsRes;
  } catch (err: any) {
    console.error("Failed to load structure data:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isLoading = false;
  }
}

async function handleCreateUnit() {
  if (!unitName.trim() && !unitNameAr.trim()) {
    toast.error(m.onboarding_step2_branch_err_name());
    return;
  }

  isSubmittingUnit = true;
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
    unitCode = "";
    unitCapacity = 0;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step2_branch_created_success();
  } catch (err: any) {
    console.error("Failed to create unit:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingUnit = false;
  }
}

async function handleCreateRoom() {
  if (!roomName.trim() && !roomNameAr.trim()) {
    toast.error(m.onboarding_step2_room_err_name());
    return;
  }

  isSubmittingRoom = true;
  successMessage = null;

  try {
    const newRoom = await api.createRoom({
      name: roomName.trim() || roomNameAr.trim(),
      name_ar: roomNameAr.trim() || undefined,
      room_type: roomType,
      is_exam_hall: isExamHall,
      capacity: Math.max(0, roomCapacity),
    });

    rooms = [...rooms, newRoom];
    roomName = "";
    roomNameAr = "";
    roomCapacity = 0;
    isExamHall = false;
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step2_room_created_success();
  } catch (err: any) {
    console.error("Failed to create room:", err);
    toast.error(err?.message || m.common_action_failed());
  } finally {
    isSubmittingRoom = false;
  }
}

async function deleteUnit(unit: OrgUnitDto) {
  await api.deleteOrgUnit(String(unit.id));
  orgUnits = orgUnits.filter((item) => item.id !== unit.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
}

async function deleteRoom(room: RoomDto) {
  await api.deleteRoom(String(room.id));
  rooms = rooms.filter((item) => item.id !== room.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
}
</script>

<div class="space-y-6">
  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
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
        <OnboardingPresetBar title={m.ui_copy_recommended_4917dbf2()}>
          <Button
            variant="outline"
            size="sm"
            disabled={isSubmittingUnit}
            onclick={applyBranchPreset}
            class="text-xs"
          >
            BR-{String(orgUnits.length + 1).padStart(2, "0")} · 500
          </Button>
        </OnboardingPresetBar>

        <!-- Branch Creation Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <BilingualNameFields
            arId="branch-name-ar"
            enId="branch-name-en"
            arLabel={m.onboarding_step2_branch_name_ar()}
            enLabel={m.onboarding_step2_branch_name_en()}
            bind:arValue={unitNameAr}
            bind:enValue={unitName}
            arPlaceholder={m.onboarding_step2_branch_name_ar_placeholder()}
            enPlaceholder={m.onboarding_step2_branch_name_en_placeholder()}
            kind="label"
            requestSuggestion={suggestLocalTranslation}
            replaceUneditedInitialValues
            disabled={isSubmittingUnit}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />
          <div class="grid gap-3 sm:grid-cols-2">
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

          <label
            for="onboarding-room-exam-committee"
            class="flex items-start gap-2 rounded-lg border border-border/50 bg-background/60 p-3 text-xs"
          >
            <Checkbox
              id="onboarding-room-exam-committee"
              bind:checked={isExamHall}
              disabled={isSubmittingRoom}
            />
            <span class="space-y-1">
              <span class="block font-medium">{m.common_rooms_exam_committee()}</span>
              <span class="block text-muted-foreground">{m.common_rooms_exam_committee_help()}</span>
            </span>
          </label>

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
                    <DeleteRecordButton label={unit.name_ar || unit.name} onDelete={() => deleteUnit(unit)} />
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
        <OnboardingPresetBar title={m.ui_copy_recommended_4917dbf2()}>
          <Button
            variant="outline"
            size="sm"
            disabled={isSubmittingRoom}
            onclick={applyRoomPreset}
            class="text-xs"
          >
            R-{String(rooms.length + 101).padStart(3, "0")} · {ROOM_TYPES[0].label} · 30
          </Button>
        </OnboardingPresetBar>

        <!-- Room Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <BilingualNameFields
            arId="room-name-ar"
            enId="room-name-en"
            arLabel={m.common_rooms_label_name_ar()}
            enLabel={m.common_rooms_label_name_en()}
            bind:arValue={roomNameAr}
            bind:enValue={roomName}
            arPlaceholder={m.common_rooms_placeholder_name_ar()}
            enPlaceholder={m.common_rooms_placeholder_name_en()}
            kind="label"
            requestSuggestion={suggestLocalTranslation}
            replaceUneditedInitialValues
            disabled={isSubmittingRoom}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />
          <div class="grid gap-3 sm:grid-cols-3">
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
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingRoom || (!roomName.trim() && !roomNameAr.trim())}
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
                      <div class="font-medium text-foreground">{room.name_ar || room.name}</div>
                      <div class="text-[10px] text-muted-foreground capitalize">
                        {room.room_type || "classroom"}
                      </div>
                      {#if room.is_exam_hall}
                        <Badge variant="outline" class="mt-1 text-[9px]">
                          {m.common_rooms_exam_committee()}
                        </Badge>
                      {/if}
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge variant="secondary" class="text-[10px]">
                      {room.capacity || 30} {m.onboarding_step2_seats_suffix()}
                    </Badge>
                    <DeleteRecordButton label={room.name_ar || room.name} onDelete={() => deleteRoom(room)} />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
