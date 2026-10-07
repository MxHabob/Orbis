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
  name = $bindable(""),
  scaleType = $bindable("percentage"),
  isCreating = false,
  onsubmit,
}: {
  open: boolean;
  name: string;
  scaleType: string;
  isCreating?: boolean;
  onsubmit: () => void;
} = $props();
const options = [
  { v: "percentage", l: m.settings_ui_grade_scale_percentage() },
  { v: "gpa_4_0", l: m.settings_ui_grade_scale_gpa() },
  { v: "pass_fail", l: m.settings_ui_grade_scale_pass_fail() },
];
</script>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">{m.settings_ui_grade_scale_create_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-4 py-2"><div class="flex flex-col gap-2"><Label class="text-xs">{m.settings_ui_grade_scale_name()}</Label><Input bind:value={name} placeholder={m.settings_ui_grade_scale_example()} class="text-xs" /></div><div class="flex flex-col gap-2"><Label class="text-xs">{m.settings_ui_grade_scale_type()}</Label><div class="flex gap-2">{#each options as opt}<button class="flex-1 text-[10px] py-2 rounded-md border transition-all cursor-pointer {scaleType === opt.v ? 'border-primary bg-primary/10 text-primary font-semibold' : 'border-border/40 text-muted-foreground hover:border-border'}" onclick={() => (scaleType = opt.v)}>{opt.l}</button>{/each}</div></div></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !name.trim()}>{#if isCreating}<LoadingIndicator class="me-1"  size={12} />{/if}{m.common_action_create()}</Button></DialogFooter></DialogContent></Dialog>
<Dialog bind:open><DialogContent class="max-w-sm"><DialogHeader><DialogTitle class="text-sm">{m.settings_ui_grade_scale_create_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-4 py-2"><FormField label={m.settings_ui_grade_scale_name()} required>{#snippet children()}<Input bind:value={name} placeholder={m.settings_ui_grade_scale_example()} class="text-xs" />{/snippet}</FormField><FormField label={m.settings_ui_grade_scale_type()} required>{#snippet children()}<div class="flex gap-2">{#each options as opt}<button type="button" class="flex-1 text-[10px] py-2 rounded-md border transition-all cursor-pointer {scaleType === opt.v ? 'border-primary bg-primary/10 text-primary font-semibold' : 'border-border/40 text-muted-foreground hover:border-border'}" onclick={() => (scaleType = opt.v)}>{opt.l}</button>{/each}</div>{/snippet}</FormField></div><DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !name.trim()}>{#if isCreating}<LoadingIndicator class="me-1"  size={12} />{/if}{m.common_action_create()}</Button></DialogFooter></DialogContent></Dialog>
