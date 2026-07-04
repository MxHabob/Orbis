<script lang="ts">
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
        <span>تأكيد الحذف / Confirm Delete</span>
      </AlertDialogTitle>
      <AlertDialogDescription>
        هل أنت متأكد من رغبتك في حذف هذا القالب نهائياً؟ لن تتمكن من استرجاع البيانات أو استخدام هذا القالب للطباعة مجدداً.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter class="gap-2">
      <AlertDialogCancel>إلغاء / Cancel</AlertDialogCancel>
      <AlertDialogAction class="bg-destructive hover:bg-destructive/90 text-white cursor-pointer" onclick={onconfirm}>
        حذف نهائي / Delete
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
