<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
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
  form,
  editingId = null,
  componentTypes,
  isSaving = false,
  onsubmit,
}: {
  open: boolean;
  form: any;
  editingId?: string | null;
  componentTypes: { v: string; l: string }[];
  isSaving?: boolean;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">{editingId ? m.common_action_edit() : m.settings_ui_assessment_component_add()}</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_assessment_component_name()}</Label><Input bind:value={form.name} placeholder={m.settings_ui_assessment_component_example()} class="text-xs" /></div><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_assessment_type()}</Label><div class="grid grid-cols-3 gap-1.5">{#each componentTypes as ct}<button class="text-[10px] py-1.5 rounded-md border transition-all cursor-pointer {form.component_type === ct.v ? 'border-primary bg-primary/10 text-primary font-semibold' : 'border-border/40 text-muted-foreground hover:border-border'}" onclick={() => (form.component_type = ct.v)}>{ct.l}</button>{/each}</div></div><div class="grid grid-cols-3 gap-3"><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_assessment_weight_percent()}</Label><Input type="number" min="0" max="100" bind:value={form.weight} class="text-xs font-mono" /></div><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_assessment_max_score()}</Label><Input type="number" min="0" bind:value={form.max_score} class="text-xs font-mono" /></div><div class="flex flex-col gap-1.5"><Label class="text-xs">{m.settings_ui_assessment_passing_score()}</Label><Input type="number" min="0" bind:value={form.passing_score} class="text-xs font-mono" /></div></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isSaving || !form.name.trim()}>{#if isSaving}<LoadingIndicator class="me-1"  size={12} />{/if}{editingId ? m.common_action_update() : m.common_action_add()}</Button></DialogFooter></DialogContent></Dialog>
