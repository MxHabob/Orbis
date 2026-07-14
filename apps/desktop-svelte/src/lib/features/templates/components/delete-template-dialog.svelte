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
} from "@orbis/ui-svelte/alert-dialog";
import { AlertCircle } from "lucide-svelte";

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
        <AlertCircle size={20} />
        <span>{m.template_auto_02a47dff()}</span>
      </AlertDialogTitle>
      <AlertDialogDescription>
        {m.template_auto_a51b56ea()}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter class="gap-2">
      <AlertDialogCancel>{m.template_auto_c7b6c276()}</AlertDialogCancel>
      <AlertDialogAction class="bg-destructive hover:bg-destructive/90 text-white cursor-pointer" onclick={onconfirm}>
        {m.template_auto_a0438ab1()}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
