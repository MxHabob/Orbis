<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as settings from "$lib/paraglide/messages/settings";
import * as common from "$lib/paraglide/messages/common";
import { onMount } from "svelte";

let {
  orgId,
  unit = null,
  defaultParentId = "",
  onSuccess,
  onCancel,
}: {
  orgId: string;
  unit?: any;
  defaultParentId?: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!unit);

// Form state
let isSubmitting = $state(false);
let unitsList = $state<any[]>([]);
let staffList = $state<any[]>([]);

let name = $state(unit?.name || "");
let nameAr = $state(unit?.name_ar || "");
let code = $state(unit?.code || "");
let unitType = $state(unit?.unit_type || "classroom");
let parentId = $state(unit?.parent_id || defaultParentId || "");
let capacity = $state<number | null>(
  unit?.capacity ? Number(unit.capacity) : null,
);
let roomNumber = $state(unit?.room_number || "");
let floor = $state<number | null>(unit?.floor ? Number(unit.floor) : null);
let building = $state(unit?.building || "");
let headId = $state(unit?.head_id || "");

const unitTypeLabels = {
  branch: "Branch / Site",
  building: "Building",
  department: "Department",
  classroom: "Classroom / Room",
  office: "Office / Lab",
  other: "Other",
};

async function loadReferenceData() {
  if (!orgId) return;
  try {
    const [allUnits, allUsers] = await Promise.all([
      commands.orgUnits.getAll(orgId),
      commands.iam.listUsers(orgId),
    ]);

    // Filter units: exclude current unit if editing to prevent period
    unitsList = allUnits.filter(
      (u: any) => !isEdit || String(u.id) !== String(unit.id),
    );

    // Filter staff / heads: admins, teachers, staff
    staffList = allUsers.filter((u: any) =>
      ["admin", "super_admin", "principal", "teacher", "staff"].includes(
        String(u.primary_role || u.role).toLowerCase(),
      ),
    );
  } catch (e) {
    console.error("Failed to load reference data for org unit:", e);
  }
}

onMount(() => {
  loadReferenceData();
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!name && !nameAr) {
    toast.error("Name is required");
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      code: code || undefined,
      unit_type: unitType,
      parent_id: parentId || undefined,
      capacity: capacity ? Number(capacity) : undefined,
      room_number: roomNumber || undefined,
      floor: floor ? Number(floor) : undefined,
      building: building || undefined,
      head_id: headId || undefined,
    };

    if (isEdit && unit) {
      await commands.orgUnits.update(unit.id, {
        ...request,
        is_active: unit.is_active,
      });
      toast.success(settings.users_update_success());
    } else {
      await commands.orgUnits.create(orgId, request);
      toast.success("Organizational unit created successfully");
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? "Edit Organizational Unit" : "Create Organizational Unit"}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Arabic Name -->
      <div class="flex flex-col gap-2">
        <label for="unit_name_ar" class="text-xs font-semibold text-foreground">
          Arabic Name
        </label>
        <Input
          id="unit_name_ar"
          type="text"
          bind:value={nameAr}
          placeholder="مثلاً: مبنى الإدارة أو الصف الأول"
          disabled={isSubmitting}
        />
      </div>

      <!-- English Name -->
      <div class="flex flex-col gap-2">
        <label for="unit_name_en" class="text-xs font-semibold text-foreground">
          English Name <span class="text-destructive">*</span>
        </label>
        <Input
          id="unit_name_en"
          type="text"
          bind:value={name}
          placeholder="e.g. Administration Building or Grade 1"
          required
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Type & Parent -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Unit Type -->
      <div class="flex flex-col gap-2">
        <label for="unit_type" class="text-xs font-semibold text-foreground">
          Unit Type
        </label>
        <Select type="single" value={unitType} onValueChange={(v) => (unitType = v ?? "classroom")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {unitTypeLabels[unitType as keyof typeof unitTypeLabels] || unitType}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="branch">Branch / Site</SelectItem>
            <SelectItem value="building">Building</SelectItem>
            <SelectItem value="department">Department</SelectItem>
            <SelectItem value="classroom">Classroom / Room</SelectItem>
            <SelectItem value="office">Office / Lab</SelectItem>
            <SelectItem value="other">Other</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Parent Unit -->
      <div class="flex flex-col gap-2">
        <label for="unit_parent" class="text-xs font-semibold text-foreground">
          Parent Unit
        </label>
        <Select type="single" value={parentId} onValueChange={(v) => (parentId = v ?? "")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {unitsList.find(u => String(u.id) === parentId) ? `${unitsList.find(u => String(u.id) === parentId).name_ar || unitsList.find(u => String(u.id) === parentId).name} (${unitsList.find(u => String(u.id) === parentId).unit_type})` : "— None (Root Unit) —"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">— None (Root Unit) —</SelectItem>
            {#each unitsList as u}
              <SelectItem value={String(u.id)}>{u.name_ar || u.name} ({u.unit_type})</SelectItem>
            {:else}
              <div class="p-2 text-xs text-muted-foreground">لا توجد وحدات</div>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Code & Head of Unit -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Code -->
      <div class="flex flex-col gap-2">
        <label for="unit_code" class="text-xs font-semibold text-foreground">
          Code
        </label>
        <Input
          id="unit_code"
          type="text"
          bind:value={code}
          placeholder="e.g. BLD-A"
          disabled={isSubmitting}
        />
      </div>

      <!-- Head of Unit -->
      <div class="flex flex-col gap-2">
        <label for="unit_head" class="text-xs font-semibold text-foreground">
          Head of Unit
        </label>
        <Select type="single" value={headId} onValueChange={(v) => (headId = v ?? "")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {staffList.find(s => String(s.id) === headId) ? (staffList.find(s => String(s.id) === headId).name_ar || staffList.find(s => String(s.id) === headId).full_name) : "— None —"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">— None —</SelectItem>
            {#each staffList as s}
              <SelectItem value={String(s.id)}>{s.name_ar || s.full_name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Building, Floor, Room Number, Capacity -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <!-- Building -->
      <div class="flex flex-col gap-2">
        <label for="unit_bld" class="text-xs font-semibold text-foreground">
          Building
        </label>
        <Input
          id="unit_bld"
          type="text"
          bind:value={building}
          placeholder="e.g. A"
          disabled={isSubmitting}
        />
      </div>

      <!-- Floor -->
      <div class="flex flex-col gap-2">
        <label for="unit_floor" class="text-xs font-semibold text-foreground">
          Floor
        </label>
        <Input
          id="unit_floor"
          type="number"
          bind:value={floor}
          placeholder="e.g. 1"
          disabled={isSubmitting}
        />
      </div>

      <!-- Room Number -->
      <div class="flex flex-col gap-2">
        <label for="unit_room" class="text-xs font-semibold text-foreground">
          Room Number
        </label>
        <Input
          id="unit_room"
          type="text"
          bind:value={roomNumber}
          placeholder="e.g. 102"
          disabled={isSubmitting}
        />
      </div>

      <!-- Capacity -->
      <div class="flex flex-col gap-2">
        <label for="unit_cap" class="text-xs font-semibold text-foreground">
          Capacity
        </label>
        <Input
          id="unit_cap"
          type="number"
          bind:value={capacity}
          placeholder="e.g. 35"
          disabled={isSubmitting}
        />
      </div>
    </div>
  </div>

  <!-- Actions -->
  <div class="px-6 py-4 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={isSubmitting}
    >
      Cancel
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={isSubmitting}
    >
      {isSubmitting ? common.user_form_saving() : common.action_save()}
    </Button>
  </div>
</form>
