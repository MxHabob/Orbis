<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
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

function handleConfirm() {
  confirmStore.respond(true);
}

function handleCancel() {
  confirmStore.respond(false);
}

// To support clicking outside or pressing Escape, we update the store when the dialog closes natively
function handleOpenChange(open: boolean) {
  if (!open && confirmStore.isOpen) {
    confirmStore.respond(false);
  }
}
</script>

<AlertDialog open={confirmStore.isOpen} onOpenChange={handleOpenChange}>
  <AlertDialogContent>
    {#if confirmStore.options}
      <AlertDialogHeader>
        <AlertDialogTitle>{confirmStore.options.title}</AlertDialogTitle>
        <AlertDialogDescription>
          {confirmStore.options.description}
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel onclick={handleCancel}>
          {confirmStore.options.cancelText || m.common_action_cancel()}
        </AlertDialogCancel>
        <AlertDialogAction
          onclick={handleConfirm}
          class={confirmStore.options.variant === "destructive" ? "bg-destructive text-destructive-foreground hover:bg-destructive/90" : ""}
        >
          {confirmStore.options.confirmText}
        </AlertDialogAction>
      </AlertDialogFooter>
    {/if}
  </AlertDialogContent>
</AlertDialog>
