<script lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";

let {
  open = $bindable(false),
  title = "تعارض في تحديث البيانات / Version Conflict",
  message = "تم تعديل هذا السجل بواسطة مستخدم آخر أو عملية أخرى بالتزامن أثناء قيامك بالتعديل. للحفاظ على تكامل البيانات، يرجى إعادة تحميل السجل للحصول على أحدث نسخة.",
  onreload,
  oncancel,
}: {
  open: boolean;
  title?: string;
  message?: string;
  onreload?: () => void;
  oncancel?: () => void;
} = $props();

function handleReload() {
  open = false;
  onreload?.();
}

function handleCancel() {
  open = false;
  oncancel?.();
}
</script>

<Dialog bind:open>
  <DialogContent class="sm:max-w-110 z-50">
    <DialogHeader class="gap-2">
      <div class="flex items-center gap-2.5 text-amber-600 dark:text-amber-500">
        <span class="text-2xl">⚠️</span>
        <DialogTitle class="text-base font-bold">{title}</DialogTitle>
      </div>
      <DialogDescription class="text-xs text-muted-foreground leading-relaxed pt-1">
        {message}
      </DialogDescription>
    </DialogHeader>

    <div class="p-3 bg-amber-500/10 border border-amber-500/20 rounded-md text-xs text-amber-800 dark:text-amber-300">
      تنبيه: التعديلات غير المحفوظة قد يتم استبدالها بأحدث البيانات المخزنة على الخادم.
    </div>

    <DialogFooter class="flex items-center justify-end gap-2 pt-2">
      <Button variant="outline" size="sm" onclick={handleCancel}>
        إلغاء / Cancel
      </Button>
      <Button variant="default" size="sm" class="bg-amber-600 hover:bg-amber-700 text-white" onclick={handleReload}>
        🔄 إعادة تحميل البيانات / Reload
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
