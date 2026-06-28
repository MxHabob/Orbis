<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  room = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  room?: any;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!room);

// Form state
let isSubmitting = $state(false);
let name = $state(room?.name || "");
let nameAr = $state(room?.name_ar || "");
let capacity = $state<number | null>(
  room?.capacity ? Number(room.capacity) : null,
);
let roomType = $state(room?.room_type || "classroom");
let building = $state(room?.building || "");
let floor = $state<number | null>(room?.floor ? Number(room.floor) : null);

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!name && !nameAr) {
    toast.error("Room Name is required");
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      capacity: capacity ? Number(capacity) : null,
      room_type: roomType,
      building: building || null,
      floor: floor ? Number(floor) : null,
    };

    if (isEdit && room) {
      await commands.schedule.updateRoom(orgId, room.id, {
        ...request,
        is_active: room.is_active,
      });
      toast.success(m.rooms_update_success());
    } else {
      await commands.schedule.createRoom(orgId, request);
      toast.success(m.rooms_create_success());
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
      <label for="room_name_ar" class="text-xs font-semibold text-foreground">
        Arabic Name
      </label>
      <Input
        id="room_name_ar"
        type="text"
        bind:value={nameAr}
        placeholder="مثلاً: قاعة 101 أو مختبر الحاسوب"
        disabled={isSubmitting}
      />
    </div>

    <!-- English Name -->
    <div class="flex flex-col gap-2">
      <label for="room_name_en" class="text-xs font-semibold text-foreground">
        English Name <span class="text-destructive">*</span>
      </label>
      <Input
        id="room_name_en"
        type="text"
        bind:value={name}
        placeholder="e.g. Room 101 or Computer Lab"
        required
        disabled={isSubmitting}
      />
    </div>
  </div>

  <!-- Type & Capacity -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Room Type -->
    <div class="flex flex-col gap-2">
      <label for="room_type" class="text-xs font-semibold text-foreground">
        Room Type
      </label>
      <select
        id="room_type"
        bind:value={roomType}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="classroom">Classroom</option>
        <option value="laboratory">Laboratory</option>
        <option value="lecture_hall">Lecture Hall</option>
        <option value="office">Staff Office</option>
        <option value="other">Other</option>
      </select>
    </div>

    <!-- Capacity -->
    <div class="flex flex-col gap-2">
      <label for="room_capacity" class="text-xs font-semibold text-foreground">
        {m.rooms_col_capacity()}
      </label>
      <Input
        id="room_capacity"
        type="number"
        bind:value={capacity}
        placeholder="e.g. 40"
        disabled={isSubmitting}
      />
    </div>
  </div>

  <!-- Location (Building & Floor) -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Building -->
    <div class="flex flex-col gap-2">
      <label for="room_building" class="text-xs font-semibold text-foreground">
        Building
      </label>
      <Input
        id="room_building"
        type="text"
        bind:value={building}
        placeholder="e.g. Main Building"
        disabled={isSubmitting}
      />
    </div>

    <!-- Floor -->
    <div class="flex flex-col gap-2">
      <label for="room_floor" class="text-xs font-semibold text-foreground">
        Floor
      </label>
      <Input
        id="room_floor"
        type="number"
        bind:value={floor}
        placeholder="e.g. 1"
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
      {m.user_form_cancel()}
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
