<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
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
let capacity = $state<number | null>(unit?.capacity ? Number(unit.capacity) : null);
let roomNumber = $state(unit?.room_number || "");
let floor = $state<number | null>(unit?.floor ? Number(unit.floor) : null);
let building = $state(unit?.building || "");
let headId = $state(unit?.head_id || "");

async function loadReferenceData() {
  if (!orgId) return;
  try {
    const [allUnits, allUsers] = await Promise.all([
      commands.orgUnits.getAll(orgId),
      commands.users.getUsers(orgId)
    ]);
    
    // Filter units: exclude current unit if editing to prevent cycle
    unitsList = allUnits.filter((u: any) => !isEdit || String(u.id) !== String(unit.id));
    
    // Filter staff / heads: admins, teachers, staff
    staffList = allUsers.filter((u: any) => 
      ["admin", "super_admin", "principal", "teacher", "staff"].includes(String(u.role).toLowerCase())
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
      toast.success(m.settings_users_update_success());
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

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-5 scrollbar-hide text-start">
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
      <select
        id="unit_type"
        bind:value={unitType}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="branch">Branch / Site</option>
        <option value="building">Building</option>
        <option value="department">Department</option>
        <option value="classroom">Classroom / Room</option>
        <option value="office">Office / Lab</option>
        <option value="other">Other</option>
      </select>
    </div>

    <!-- Parent Unit -->
    <div class="flex flex-col gap-2">
      <label for="unit_parent" class="text-xs font-semibold text-foreground">
        Parent Unit
      </label>
      <select
        id="unit_parent"
        bind:value={parentId}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="">— None (Root Unit) —</option>
        {#each unitsList as u}
          <option value={u.id}>{u.name_ar || u.name} ({u.unit_type})</option>
        {/each}
      </select>
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
      <select
        id="unit_head"
        bind:value={headId}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="">— None —</option>
        {#each staffList as s}
          <option value={s.id}>{s.name_ar || s.full_name}</option>
        {/each}
      </select>
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

  <!-- Actions -->
  <div class="flex gap-2 justify-end mt-auto pt-6 border-t border-border/40">
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
      {isSubmitting ? m.user_form_saving() : m.user_form_save()}
    </Button>
  </div>
</form>
