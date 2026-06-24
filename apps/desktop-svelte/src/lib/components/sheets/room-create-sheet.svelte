<!--
@component RoomCreateSheet
A slide-over Sheet component for creating or editing a room.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import RoomForm from "../forms/room-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  room = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  room?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!room);
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? "Edit Room Details" : m.rooms_add()}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        Configure classrooms, laboratory capacities, and building details.
      </SheetDescription>
    </SheetHeader>

    <RoomForm
      {orgId}
      {room}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
