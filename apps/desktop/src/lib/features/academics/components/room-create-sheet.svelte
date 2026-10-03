<!--
@component RoomCreateSheet
A slide-over Sheet component for creating or editing a room.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import RoomForm from "./room-form.svelte";
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
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden p-0 gap-0">
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
