<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
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
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">{m.settings_ui_grade_item_add_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_grade_item_label()}</Label><Input bind:value={item.grade_label} placeholder={m.settings_ui_grade_item_example()} class="text-xs" /></div><div class="grid grid-cols-2 gap-3"><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_grade_item_min_score()}</Label><Input type="number" bind:value={item.min_score} class="text-xs font-mono" /></div><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_grade_item_max_score()}</Label><Input type="number" bind:value={item.max_score} class="text-xs font-mono" /></div></div><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_grade_item_gpa_points()}</Label><Input type="number" step="0.5" bind:value={item.gpa_points} class="text-xs font-mono" /></div><div class="flex items-center justify-between"><Label class="text-xs">{m.settings_ui_grade_item_passing()}</Label><Switch checked={item.is_passing} onCheckedChange={(v) => (item.is_passing = v)} /></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !item.grade_label.trim()}>{#if isCreating}<LoadingIndicator class="me-1"  size={12} />{/if}{m.common_action_add()}</Button></DialogFooter></DialogContent></Dialog>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">{m.settings_ui_grade_item_add_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><FormField label={m.settings_ui_grade_item_label()} required>{#snippet children()}<Input bind:value={item.grade_label} placeholder={m.settings_ui_grade_item_example()} class="text-xs" />{/snippet}</FormField><div class="grid grid-cols-2 gap-3"><FormField label={m.settings_ui_grade_item_min_score()}>{#snippet children()}<Input type="number" bind:value={item.min_score} class="text-xs font-mono" />{/snippet}</FormField><FormField label={m.settings_ui_grade_item_max_score()}>{#snippet children()}<Input type="number" bind:value={item.max_score} class="text-xs font-mono" />{/snippet}</FormField></div><FormField label={m.settings_ui_grade_item_gpa_points()}>{#snippet children()}<Input type="number" step="0.5" bind:value={item.gpa_points} class="text-xs font-mono" />{/snippet}</FormField><div class="flex items-center justify-between"><span class="text-xs">{m.settings_ui_grade_item_passing()}</span><Switch checked={item.is_passing} onCheckedChange={(v) => (item.is_passing = v)} /></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !item.grade_label.trim()}>{#if isCreating}<LoadingIndicator class="me-1"  size={12} />{/if}{m.common_action_add()}</Button></DialogFooter></DialogContent></Dialog>
