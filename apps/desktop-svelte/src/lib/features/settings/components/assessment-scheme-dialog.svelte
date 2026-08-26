<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Loader2 } from "@lucide/svelte";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@orbis/ui-svelte/dialog";

let { open = $bindable(false), name = $bindable(""), description = $bindable(""), isCreating = false, onsubmit }: { open: boolean; name: string; description: string; isCreating?: boolean; onsubmit: () => void } = $props();
</script>

<Dialog bind:open>
  <DialogContent class="max-w-sm">
    <DialogHeader><DialogTitle class="text-sm">إنشاء هيكل تقييم جديد</DialogTitle></DialogHeader>
    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-2"><Label class="text-xs">اسم الهيكل</Label><Input bind:value={name} placeholder="مثال: هيكل التقييم الابتدائي" class="text-xs" /></div>
      <div class="flex flex-col gap-2"><Label class="text-xs">الوصف (اختياري)</Label><Input bind:value={description} placeholder="وصف مختصر لهيكل التقييم" class="text-xs" /></div>
    </div>
    <DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !name.trim()}>{#if isCreating}<Loader2 size={12} class="animate-spin me-1" />{/if}إنشاء</Button></DialogFooter>
  </DialogContent>
</Dialog>
