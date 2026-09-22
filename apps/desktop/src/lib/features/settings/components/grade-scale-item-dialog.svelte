<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { AdjustmentsHorizontal, Label } from "@orbis/icons";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
let {
  open = $bindable(false),
  item,
  isCreating = false,
  onsubmit,
}: {
  open: boolean;
  item: any;
  isCreating?: boolean;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">إضافة تقدير جديد</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><div class="flex flex-col gap-1.5"><Label class="text-xs">التقدير</Label><Input bind:value={item.grade_label} placeholder="مثال: ممتاز" class="text-xs" /></div><div class="grid grid-cols-2 gap-3"><div class="flex flex-col gap-1.5"><Label class="text-xs">أدنى درجة</Label><Input type="number" bind:value={item.min_score} class="text-xs font-mono" /></div><div class="flex flex-col gap-1.5"><Label class="text-xs">أعلى درجة</Label><Input type="number" bind:value={item.max_score} class="text-xs font-mono" /></div></div><div class="flex flex-col gap-1.5"><Label class="text-xs">نقاط GPA</Label><Input type="number" step="0.5" bind:value={item.gpa_points} class="text-xs font-mono" /></div><div class="flex items-center justify-between"><Label class="text-xs">ناجح</Label><Switch checked={item.is_passing} onCheckedChange={(v) => (item.is_passing = v)} /></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !item.grade_label.trim()}>{#if isCreating}<Loader2 size={12} class="animate-spin me-1" />{/if}إضافة</Button></DialogFooter></DialogContent></Dialog>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">إضافة تقدير جديد</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><FormField label="التقدير" required>{#snippet children()}<Input bind:value={item.grade_label} placeholder="مثال: ممتاز" class="text-xs" />{/snippet}</FormField><div class="grid grid-cols-2 gap-3"><FormField label="أدنى درجة">{#snippet children()}<Input type="number" bind:value={item.min_score} class="text-xs font-mono" />{/snippet}</FormField><FormField label="أعلى درجة">{#snippet children()}<Input type="number" bind:value={item.max_score} class="text-xs font-mono" />{/snippet}</FormField></div><FormField label="نقاط GPA">{#snippet children()}<Input type="number" step="0.5" bind:value={item.gpa_points} class="text-xs font-mono" />{/snippet}</FormField><div class="flex items-center justify-between"><span class="text-xs">ناجح</span><Switch checked={item.is_passing} onCheckedChange={(v) => (item.is_passing = v)} /></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !item.grade_label.trim()}>{#if isCreating}<Loader2 size={12} class="animate-spin me-1" />{/if}إضافة</Button></DialogFooter></DialogContent></Dialog>
