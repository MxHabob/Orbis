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

let newRoomName = $state("");
let newRoomNameAr = $state("");
let newRoomCapacity = $state(30);
let newRoomType = $state("classroom");
let newRoomFloor = $state(1);

const roomTypeOptions = [
  { value: "classroom", label: "فصل / قاعة دراسية (Classroom)" },
  { value: "lecture_hall", label: "مدرج / قاعة محاضرات (Lecture Hall)" },
  { value: "computer_lab", label: "معمل حاسب آلي (Computer Lab)" },
  { value: "science_lab", label: "مختبر علمي (Science Lab)" },
  { value: "workshop", label: "ورشة عمل / تدريب (Workshop)" },
  { value: "library", label: "مكتبة / مصادر تعلم (Library)" },
  { value: "studio", label: "استوديو / قاعة افتراضية (Studio)" },
  { value: "activity_hall", label: "صالة أنشطة / متعددة الأغراض (Hall)" },
];

function getRoomTypeLabel(type: string) {
  return roomTypeOptions.find(o => o.value === type)?.label || type;
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
      المباني والمرافق والقاعات التعليمية
    </h3>
    <p class="text-xs text-muted-foreground">
      تحديد المباني والمجمعات والمرافق (فصول، قاعات، معامل ومختبرات) مع طاقتها الاستيعابية وتوزيع الطوابق.
    </p>
  </div>

  <!-- Building Tabs -->
  <Tabs
    value={onboardingState.facilities.activeBuildingTab}
    onValueChange={(v) => (onboardingState.facilities.activeBuildingTab = v ?? onboardingState.facilities.buildings[0])}
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
              {onboardingState.facilities.rooms.filter((r) => r.building === bld).length} مرافق
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
        إضافة مبنى / مقر جديد
      </Button>
    </TabsList>

    {#each onboardingState.facilities.buildings as bld}
      <TabsContent value={bld} class="space-y-4 pt-4">
        <!-- Building Action Bar -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <DoorOpen size={15} class="text-primary" />
            <h4 class="text-xs font-bold text-foreground">
              قاعات ومرافق ({bld})
            </h4>
          </div>
          {#if onboardingState.facilities.buildings.length > 1}
            <Button
              variant="ghost"
              size="sm"
              class="text-muted-foreground hover:text-destructive h-8 px-3 text-xs flex gap-1.5 items-center rounded-lg cursor-pointer"
              onclick={() => handleRemoveBuilding(bld)}
            >
              <Trash2 size={13} />
              <span>حذف المبنى</span>
            </Button>
          {/if}
        </div>

        <!-- Rooms / Facilities Table -->
        <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-xl bg-card shadow-2xs">
          <div class="min-w-[760px] divide-y divide-border/40">
            <!-- Header Row -->
            <div class="grid grid-cols-[1.5fr_1.5fr_1.8fr_1fr_1fr_auto] gap-3 p-3.5 bg-muted/30 text-xs font-semibold text-muted-foreground select-none text-start">
              <span>الاسم بالعربية</span>
              <span>الاسم بالإنجليزية</span>
              <span>نوع المرفق / القاعة</span>
              <span>الطابق</span>
              <span>السعة (طلاب)</span>
              <span class="w-8"></span>
            </div>

            <!-- Inline Add New Room Row -->
            <div class="grid grid-cols-[1.5fr_1.5fr_1.8fr_1fr_1fr_auto] gap-3 px-3.5 py-2.5 items-center bg-primary/5 border-b border-border/40">
              <!-- Arabic Name -->
              <Input
                bind:value={newRoomNameAr}
                placeholder="مثال: قاعة 101"
                class="h-8 text-xs font-sans bg-background"
              />

              <!-- English Name -->
              <Input
                bind:value={newRoomName}
                placeholder="e.g. Room 101"
                class="h-8 text-xs font-mono bg-background"
                style="direction: ltr;"
              />

              <!-- Room Type Select -->
              <Select type="single" value={newRoomType} onValueChange={(v) => { if (v) newRoomType = v; }}>
                <SelectTrigger class="h-8 text-xs bg-background">
                  {getRoomTypeLabel(newRoomType)}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  {#each roomTypeOptions as opt}
                    <SelectItem value={opt.value}>{opt.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <!-- Floor Select -->
              <Select type="single" value={String(newRoomFloor)} onValueChange={(v) => (newRoomFloor = Number(v ?? "1") || 1)}>
                <SelectTrigger class="h-8 text-xs bg-background font-mono">
                  الطابق {newRoomFloor}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="0">الطابق الأرضي</SelectItem>
                  <SelectItem value="1">الطابق الأول</SelectItem>
                  <SelectItem value="2">الطابق الثاني</SelectItem>
                  <SelectItem value="3">الطابق الثالث</SelectItem>
                  <SelectItem value="4">الطابق الرابع</SelectItem>
                </SelectContent>
              </Select>

              <!-- Capacity Input -->
              <Input
                type="number"
                bind:value={newRoomCapacity}
                min="1"
                max="500"
                class="h-8 text-xs text-center font-mono bg-background"
              />

              <!-- Add Button -->
              <Button
                variant="default"
                size="icon-sm"
                class="h-8 w-8 rounded-lg cursor-pointer"
                onclick={handleAddRoom}
                title="إضافة القاعة"
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
                <Select type="single" value={rm.room_type} onValueChange={(v) => { if (v) { rm.room_type = v; onboardingState.save(); } }}>
                  <SelectTrigger class="h-8 text-xs">
                    {getRoomTypeLabel(rm.room_type)}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    {#each roomTypeOptions as opt}
                      <SelectItem value={opt.value}>{opt.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>

                <!-- Floor Select -->
                <Select type="single" value={String(rm.floor)} onValueChange={(v) => { if (v) { rm.floor = Number(v); onboardingState.save(); } }}>
                  <SelectTrigger class="h-8 text-xs font-mono">
                    الطابق {rm.floor}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="0">الطابق الأرضي</SelectItem>
                    <SelectItem value="1">الطابق الأول</SelectItem>
                    <SelectItem value="2">الطابق الثاني</SelectItem>
                    <SelectItem value="3">الطابق الثالث</SelectItem>
                    <SelectItem value="4">الطابق الرابع</SelectItem>
                  </SelectContent>
                </Select>

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
                  title="حذف"
                >
                  <Trash2 size={13} />
                </Button>
              </div>
            {:else}
              <div class="p-8 text-center text-xs text-muted-foreground">
                لا توجد قاعات مسجلة في هذا المبنى. استخدم الصف بالأعلى لإضافة قاعات ومرافق.
              </div>
            {/each}
          </div>
        </div>
      </TabsContent>
    {/each}
  </Tabs>

  <!-- Add Building Dialog -->
  <Dialog bind:open={isAddDialogOpen}>
    <DialogContent class="max-w-md w-full min-w-0">
      <DialogHeader>
        <DialogTitle class="text-sm">إضافة مبنى أو مجمع جديد</DialogTitle>
      </DialogHeader>
      <div class="flex flex-col gap-3 py-2">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs">اسم المبنى / المقر</Label>
          <Input
            bind:value={newBuildingName}
            placeholder="مثال: مبنى المرحلة الابتدائية أو كلية العلوم"
            class="text-xs"
            autofocus
            onkeydown={(e) => { if (e.key === "Enter") confirmAddBuilding(); }}
          />
        </div>
      </div>
      <DialogFooter class="flex items-center justify-end w-full pt-2">
        <Button variant="outline" size="sm" class="text-xs cursor-pointer" onclick={() => (isAddDialogOpen = false)}>
          إلغاء
        </Button>
        <Button size="sm" class="text-xs cursor-pointer" onclick={confirmAddBuilding} disabled={!newBuildingName.trim()}>
          إضافة المبنى
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <!-- Delete Building Alert Dialog -->
  <AlertDialog bind:open={isDeleteDialogOpen}>
    <AlertDialogContent class="max-w-sm">
      <AlertDialogHeader>
        <AlertDialogTitle class="text-sm">حذف المبنى</AlertDialogTitle>
        <AlertDialogDescription class="text-xs leading-relaxed">
          هل أنت متأكد من حذف المبنى ({buildingToDelete}) وجميع القاعات والمرافق التابعة له؟
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter class="flex items-center justify-end gap-2 pt-2">
        <AlertDialogCancel class="text-xs cursor-pointer">إلغاء</AlertDialogCancel>
        <AlertDialogAction class="text-xs bg-destructive text-destructive-foreground hover:bg-destructive/90 cursor-pointer" onclick={confirmRemoveBuilding}>
          تأكيد الحذف
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</div>
