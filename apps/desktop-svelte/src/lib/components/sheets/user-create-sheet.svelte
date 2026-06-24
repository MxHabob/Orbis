<!--
@component UserCreateSheet
A slide-over Sheet component for creating or editing a user.
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
import UserForm from "../forms/user-form.svelte";
import * as m from "$lib/paraglide/messages";
import type { User } from "$lib/services/ipc";

let {
  open = $bindable(false),
  orgId,
  user = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  user?: User | null;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!user);
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? m.user_form_title_edit() : m.user_form_title_create()}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        {isEdit ? m.user_form_desc_edit() : m.user_form_desc_create()}
      </SheetDescription>
    </SheetHeader>

    <UserForm
      {orgId}
      {user}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
