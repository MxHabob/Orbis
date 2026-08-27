<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands, OrbisError } from "$lib/core/ipc";
import { createRoomInputSchema } from "@orbis/schemas";
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
import { untrack } from "svelte";

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

const initRoom = untrack(() => room);

// Form state
let isSubmitting = $state(false);
let name = $state(initRoom?.name || "");
let nameAr = $state(initRoom?.name_ar || "");
let capacity = $state<number | null>(
  initRoom?.capacity ? Number(initRoom.capacity) : null,
);
let roomType = $state(initRoom?.room_type || "classroom");
let building = $state(initRoom?.building || "");
let floor = $state<number | null>(
  initRoom?.floor ? Number(initRoom.floor) : null,
);

const roomTypeLabels = {
  classroom: m.common_rooms_type_classroom(),
  laboratory: m.common_rooms_type_laboratory(),
  lecture_hall: m.common_rooms_type_lecture_hall(),
  office: m.common_rooms_type_office(),
  other: m.common_rooms_type_other(),
};

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  const result = createRoomInputSchema.safeParse({
    name,
    name_ar: nameAr,
    capacity: capacity ? Number(capacity) : null,
    room_type: roomType,
    building: building || null,
    floor: floor ? Number(floor) : null,
  });

  if (!result.success) {
    toast.error(m.common_rooms_error_name_required());
    return;
  }

  isSubmitting = true;
  try {
    if (isEdit && room) {
      await commands.scheduling.updateRoom(room.id, {
        ...result.data,
        is_active: room.is_active,
      });
      toast.success(m.common_rooms_update_success());
    } else {
      await commands.scheduling.createRoom(result.data);
      toast.success(m.common_rooms_create_success());
    }

    onSuccess();
  } catch (e) {
    const errorMsg = OrbisError.fromUnknown(e).message;
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? m.common_rooms_edit_title() : m.common_rooms_add()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Arabic Name -->
      <div class="flex flex-col gap-2">
        <label for="room_name_ar" class="text-xs font-semibold text-foreground">
          {m.common_rooms_label_name_ar()}
        </label>
        <Input
          id="room_name_ar"
          type="text"
          bind:value={nameAr}
          placeholder={m.academics_room_form_placeholder()}
          disabled={isSubmitting}
        />
      </div>

      <!-- English Name -->
      <div class="flex flex-col gap-2">
        <label for="room_name_en" class="text-xs font-semibold text-foreground">
          {m.common_rooms_label_name_en()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="room_name_en"
          type="text"
          bind:value={name}
          placeholder={m.common_rooms_placeholder_name_en()}
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
          {m.common_rooms_type_label()}
        </label>
        <Select type="single" value={roomType} onValueChange={(v) => (roomType = v ?? "classroom")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {roomTypeLabels[roomType as keyof typeof roomTypeLabels] || roomType}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="classroom">{m.common_rooms_type_classroom()}</SelectItem>
            <SelectItem value="laboratory">{m.common_rooms_type_laboratory()}</SelectItem>
            <SelectItem value="lecture_hall">{m.common_rooms_type_lecture_hall()}</SelectItem>
            <SelectItem value="office">{m.common_rooms_type_office()}</SelectItem>
            <SelectItem value="other">{m.common_rooms_type_other()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Capacity -->
      <div class="flex flex-col gap-2">
        <label for="room_capacity" class="text-xs font-semibold text-foreground">
          {m.common_rooms_col_capacity()}
        </label>
        <Input
          id="room_capacity"
          type="number"
          bind:value={capacity}
          placeholder={m.common_rooms_placeholder_capacity()}
          disabled={isSubmitting}
        />
      </div>
    </div>

    <!-- Location (Building & Floor) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Building -->
      <div class="flex flex-col gap-2">
        <label for="room_building" class="text-xs font-semibold text-foreground">
          {m.common_rooms_label_building()}
        </label>
        <Input
          id="room_building"
          type="text"
          bind:value={building}
          placeholder={m.common_rooms_placeholder_building()}
          disabled={isSubmitting}
        />
      </div>

      <!-- Floor -->
      <div class="flex flex-col gap-2">
        <label for="room_floor" class="text-xs font-semibold text-foreground">
          {m.common_rooms_label_floor()}
        </label>
        <Input
          id="room_floor"
          type="number"
          bind:value={floor}
          placeholder={m.common_rooms_placeholder_floor()}
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
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={isSubmitting}
    >
      {isSubmitting ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
