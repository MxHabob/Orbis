<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { createRoomInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { FormField } from "@orbis/ui-svelte";
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

let {
  orgId,
  room = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  room?: {
    id: string;
    name?: string | null;
    name_ar?: string | null;
    capacity?: number | null;
    room_type?: string | null;
    building?: string | null;
    floor?: number | null;
  } | null;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!room);

const form = new FormController({
  initialValues: {
    name: room?.name || "",
    name_ar: room?.name_ar || "",
    capacity: room?.capacity ?? null,
    room_type: room?.room_type || "classroom",
    building: room?.building || "",
    floor: room?.floor ?? null,
  },
  schema: createRoomInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

const roomTypeLabels = {
  classroom: m.common_rooms_type_classroom(),
  laboratory: m.common_rooms_type_laboratory(),
  lecture_hall: m.common_rooms_type_lecture_hall(),
  office: m.common_rooms_type_office(),
  other: m.common_rooms_type_other(),
};

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    if (isEdit && room) {
      await commands.scheduling.updateRoom(room.id, payload);
      toast.success(m.common_rooms_update_success());
    } else {
      await commands.scheduling.createRoom(payload);
      toast.success(m.common_rooms_create_success());
    }
  });
  if (submitted) {
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(m.common_rooms_error_name_required());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
  <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
    <SheetTitle class="text-xl font-bold text-foreground">
      {isEdit ? m.common_rooms_edit_title() : m.common_rooms_add()}
    </SheetTitle>
  </SheetHeader>

  <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide text-start">
    <!-- Names (Arabic & English) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Arabic Name -->
      <FormField id="room_name_ar" label={m.common_rooms_label_name_ar()} error={fieldError("name_ar")}>
        {#snippet children()}
        <Input
          id="room_name_ar"
          type="text"
          bind:value={form.values.name_ar}
          onblur={() => form.markTouched("name_ar")}
          placeholder={m.academics_room_form_placeholder()}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>

      <!-- English Name -->
      <FormField id="room_name_en" label={m.common_rooms_label_name_en()} error={fieldError("name")} required>
        {#snippet children()}
        <Input
          id="room_name_en"
          type="text"
          bind:value={form.values.name}
          onblur={() => form.markTouched("name")}
          aria-invalid={Boolean(fieldError("name"))}
          aria-describedby={fieldError("name") ? "room_name_error" : undefined}
          placeholder={m.common_rooms_placeholder_name_en()}
          required
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>

    <!-- Type & Capacity -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Room Type -->
      <FormField id="room_type" label={m.common_rooms_type_label()} error={fieldError("room_type")} required>
        {#snippet children()}
        <Select type="single" value={form.values.room_type} onValueChange={(v) => (form.values.room_type = v ?? "classroom")}>
          <SelectTrigger class="h-9 text-sm w-full">
            {roomTypeLabels[form.values.room_type as keyof typeof roomTypeLabels] || form.values.room_type}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="classroom">{m.common_rooms_type_classroom()}</SelectItem>
            <SelectItem value="laboratory">{m.common_rooms_type_laboratory()}</SelectItem>
            <SelectItem value="lecture_hall">{m.common_rooms_type_lecture_hall()}</SelectItem>
            <SelectItem value="office">{m.common_rooms_type_office()}</SelectItem>
            <SelectItem value="other">{m.common_rooms_type_other()}</SelectItem>
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      <!-- Capacity -->
      <FormField id="room_capacity" label={m.common_rooms_col_capacity()} error={fieldError("capacity")}>
        {#snippet children()}
        <Input
          id="room_capacity"
          type="number"
          bind:value={form.values.capacity}
          onblur={() => form.markTouched("capacity")}
          placeholder={m.common_rooms_placeholder_capacity()}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>

    <!-- Location (Building & Floor) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Building -->
      <FormField id="room_building" label={m.common_rooms_label_building()} error={fieldError("building")}>
        {#snippet children()}
        <Input
          id="room_building"
          type="text"
          bind:value={form.values.building}
          onblur={() => form.markTouched("building")}
          placeholder={m.common_rooms_placeholder_building()}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>

      <!-- Floor -->
      <FormField id="room_floor" label={m.common_rooms_label_floor()} error={fieldError("floor")}>
        {#snippet children()}
        <Input
          id="room_floor"
          type="number"
          bind:value={form.values.floor}
          onblur={() => form.markTouched("floor")}
          placeholder={m.common_rooms_placeholder_floor()}
          disabled={form.isSubmitting}
        />
        {/snippet}
      </FormField>
    </div>
  </div>

  <!-- Actions -->
  <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={form.isSubmitting}
    >
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={form.isSubmitting}
    >
      {form.isSubmitting ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </div>
</form>
