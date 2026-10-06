<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
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

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

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
  { value: "classroom", label: "Classroom (فصل دراسي)" },
  { value: "lab", label: "Science / Computer Lab (مختبر)" },
  { value: "auditorium", label: "Auditorium / Theatre (مسرح / قاعة احتفالات)" },
  { value: "library", label: "Library (مكتبة)" },
  { value: "sports_hall", label: "Sports Hall / Gymnasium (صالة رياضية)" },
  { value: "other", label: "Other Facility (مرفق آخر)" },
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
    errorMessage = err?.message || "Failed to load organizational structure";
  } finally {
    isLoading = false;
  }
}

async function handleCreateUnit() {
  if (!unitName.trim() && !unitNameAr.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم الفرع أو المقر" : "Branch/Campus name is required";
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
    successMessage = isArabic ? "تمت إضافة الفرع بنجاح" : "Branch created successfully";
  } catch (err: any) {
    console.error("Failed to create unit:", err);
    errorMessage = err?.message || "Failed to create organizational unit";
  } finally {
    isSubmittingUnit = false;
  }
}

async function handleCreateRoom() {
  if (!roomName.trim() && !roomNameAr.trim() && !roomNumber.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم أو رقم القاعة" : "Room name or number is required";
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
    successMessage = isArabic ? "تمت إضافة المرفق التعليمي بنجاح" : "Room created successfully";
  } catch (err: any) {
    console.error("Failed to create room:", err);
    errorMessage = err?.message || "Failed to create room";
  } finally {
    isSubmittingRoom = false;
  }
}

async function handleContinue() {
  // If user hasn't added any branch and form has data, submit it first
  if (orgUnits.length === 0 && (unitName.trim() || unitNameAr.trim())) {
    await handleCreateUnit();
  }
  await goto(`/${orgId}/onboarding/calendar`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={2}
    title="Organizational Structure & Physical Facilities"
    titleAr="الهيكل التنظيمي والمرافق التعليمية"
    description="Establish your primary operational branches, campuses, buildings, and teaching facilities/rooms."
    descriptionAr="أنشئ الفروع والمقرات الرئيسية وقاعات التدريس والمختبرات والمرافق الملحقة."
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
      <span>{isArabic ? "جاري تحميل البيانات..." : "Loading organizational structure..."}</span>
    </div>
  {:else}
    <!-- Section 1: Branches & Campuses -->
    <OnboardingSection
      title="Campuses & Branches"
      titleAr="الفروع والمقرات الرئيسية"
      description="Define at least one main branch or campus for this institution."
      descriptionAr="أضف فرعاً واحداً على الأقل ليكون المقر الرئيسي أو الفرع التعليمي للطلاب."
      badge={orgUnits.length > 0 ? `${orgUnits.length} ${isArabic ? "فرع مسجل" : "Branches"}` : undefined}
      isComplete={orgUnits.length > 0}
    >
      <div class="space-y-4">
        <!-- Branch Creation Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم الفرع (بالإنكليزية / اللاتينية) *" : "Branch Name (English) *"}
              </Label>
              <Input
                bind:value={unitName}
                placeholder="e.g. Main Campus"
                dir="ltr"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم الفرع (بالعربية)" : "Branch Name (Arabic)"}
              </Label>
              <Input
                bind:value={unitNameAr}
                placeholder="مثال: الفرع الرئيسي - الرياض"
                dir="rtl"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "رمز الفرع" : "Branch Code"}
              </Label>
              <Input
                bind:value={unitCode}
                placeholder="BR-01"
                dir="ltr"
                class="font-mono text-xs"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "الطاقة الاستيعابية التقديرية (طالب)" : "Estimated Student Capacity"}
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
              {isSubmittingUnit ? (isArabic ? "جاري الإضافة..." : "Adding...") : (isArabic ? "+ إضافة الفرع" : "+ Add Branch")}
            </Button>
          </div>
        </div>

        <!-- Existing Branches List -->
        {#if orgUnits.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "الفروع المسجلة" : "Configured Branches"}
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
                        {isArabic && unit.name_ar ? unit.name_ar : unit.name}
                        {#if !isArabic && unit.name_ar}
                          <span class="text-muted-foreground text-[11px] font-sans ml-1">({unit.name_ar})</span>
                        {/if}
                      </div>
                      <div class="text-[11px] font-mono text-muted-foreground">
                        Code: {unit.code || "—"} | {unit.unit_type || "branch"}
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    {#if unit.capacity}
                      <Badge variant="secondary" class="text-[10px]">
                        {unit.capacity} {isArabic ? "طالب" : "capacity"}
                      </Badge>
                    {/if}
                    <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                      {isArabic ? "نشط" : "Active"}
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
      title="Teaching Facilities & Classrooms"
      titleAr="المرافق والقاعات التعليمية"
      description="Add the initial classrooms, lecture halls, or computer labs for student scheduling."
      descriptionAr="أضف الفصول الدراسية وقاعات المحاضرات والمختبرات لإسناد الجداول والحصص الدراسية."
      badge={rooms.length > 0 ? `${rooms.length} ${isArabic ? "قاعة مسجلة" : "Rooms"}` : undefined}
      isComplete={rooms.length > 0}
    >
      <div class="space-y-4">
        <!-- Room Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-3">
            <div class="space-y-1.5 sm:col-span-2">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم القاعة أو الفصل *" : "Room / Classroom Name *"}
              </Label>
              <Input
                bind:value={roomName}
                placeholder="e.g. Classroom 101 / قاعة 101"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "رقم القاعة *" : "Room Number *"}
              </Label>
              <Input
                bind:value={roomNumber}
                placeholder="R-101"
                class="font-mono text-xs"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "نوع المرفق" : "Facility Type"}
              </Label>
              <Select type="single" bind:value={roomType}>
                <SelectTrigger class="w-full">
                  {ROOM_TYPES.find((r) => r.value === roomType)?.label || roomType}
                </SelectTrigger>
                <SelectContent>
                  {#each ROOM_TYPES as r}
                    <SelectItem value={r.value}>{r.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "سعة القاعة (طالب)" : "Capacity (Seats)"}
              </Label>
              <Input
                type="number"
                bind:value={roomCapacity}
                placeholder="30"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "المبنى / الجناح" : "Building / Wing"}
              </Label>
              <Input
                bind:value={roomBuilding}
                placeholder="Building A"
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
              {isSubmittingRoom ? (isArabic ? "جاري الإضافة..." : "Adding...") : (isArabic ? "+ إضافة القاعة" : "+ Add Room")}
            </Button>
          </div>
        </div>

        <!-- Existing Rooms List -->
        {#if rooms.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "القاعات المسجلة" : "Configured Facilities"}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each rooms as room (room.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div class="flex items-center gap-2">
                    <span class="flex size-6 items-center justify-center rounded bg-primary/10 text-primary text-[10px] font-bold">
                      🚪
                    </span>
                    <div>
                      <div class="font-medium text-foreground">{room.name || room.room_number}</div>
                      <div class="text-[10px] text-muted-foreground capitalize">
                        {room.room_type || "classroom"} {room.building ? `(${room.building})` : ""}
                      </div>
                    </div>
                  </div>
                  <Badge variant="secondary" class="text-[10px]">
                    {room.capacity || 30} {isArabic ? "مقعد" : "seats"}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Bottom Actions Bar -->
    <div class="flex items-center justify-between border-t border-border/50 pt-6">
      <Button
        variant="outline"
        onclick={() => goto(`/${orgId}/onboarding/organization`)}
        class="text-xs"
      >
        {isArabic ? "← الخطوة السابقة (المنشأة)" : "← Back to Organization"}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="default"
          disabled={orgUnits.length === 0 && !unitName.trim()}
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {isArabic ? "متابعة إلى التقويم الدراسي →" : "Continue to Academic Calendar →"}
        </Button>
      </div>
    </div>
  {/if}
</div>
