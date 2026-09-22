<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { AdjustmentsHorizontal } from "@orbis/icons";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";

let {
  open = $bindable(false),
  name = $bindable(""),
  description = $bindable(""),
  isCreating = false,
  onsubmit,
}: {
  open: boolean;
  name: string;
  description: string;
  isCreating?: boolean;
  onsubmit: () => void;
} = $props();
</script>

<Dialog bind:open>
  <DialogContent class="max-w-sm">
    <DialogHeader><DialogTitle class="text-sm">إنشاء هيكل تقييم جديد</DialogTitle></DialogHeader>
    <div class="flex flex-col gap-4 py-2">
      <FormField label="اسم الهيكل" required>{#snippet children()}<Input bind:value={name} placeholder="مثال: هيكل التقييم الابتدائي" class="text-xs" />{/snippet}</FormField>
      <FormField label="الوصف (اختياري)">{#snippet children()}<Input bind:value={description} placeholder="وصف مختصر لهيكل التقييم" class="text-xs" />{/snippet}</FormField>
    </div>
    <DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !name.trim()}>{#if isCreating}<Loader2 size={12} class="animate-spin me-1" />{/if}إنشاء</Button></DialogFooter>
  </DialogContent>
</Dialog>
