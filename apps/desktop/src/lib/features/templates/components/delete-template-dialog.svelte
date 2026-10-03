<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogAction,
  AlertDialogCancel,
} from "@orbis/ui/alert-dialog";
import { InfoCircle } from "@orbis/icons";

interface Props {
  open: boolean;
  onconfirm: () => void;
}

let { open = $bindable(), onconfirm }: Props = $props();
</script>

<AlertDialog bind:open={open}>
  <AlertDialogContent class="text-start">
    <AlertDialogHeader>
      <AlertDialogTitle class="flex items-center gap-2 text-destructive">
        <InfoCircle size={20} />
        <span>{m.template_title_confirm_delete()}</span>
      </AlertDialogTitle>
      <AlertDialogDescription>
        {m.template_msg_confirm_delete()}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter class="gap-2">
      <AlertDialogCancel>{m.template_btn_cancel()}</AlertDialogCancel>
      <AlertDialogAction class="bg-destructive hover:bg-destructive/90 text-white cursor-pointer" onclick={onconfirm}>
        {m.template_btn_final_delete()}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
