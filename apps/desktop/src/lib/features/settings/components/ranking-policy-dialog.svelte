<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
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
  isSaving = false,
  onsubmit,
}: {
  open: boolean;
  form: any;
  editingId?: string | null;
  isSaving?: boolean;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-md"><DialogHeader><DialogTitle>{editingId ? m.settings_ui_ranking_policy_edit_title() : m.settings_ui_ranking_policy_create_title()}</DialogTitle></DialogHeader><div class="space-y-4 py-2 text-xs"><div class="space-y-1.5"><Label>{m.settings_ui_policy_name()}</Label><Input bind:value={form.name} placeholder={m.settings_ui_ranking_policy_name_example()} /></div><div class="space-y-1.5"><Label>{m.settings_ui_ranking_scope()}</Label><Select type="single" value={form.ranking_scope} onValueChange={(v) => { if (v) form.ranking_scope = v; }}><SelectTrigger class="w-full">{form.ranking_scope === "class" ? m.settings_ui_ranking_scope_class() : form.ranking_scope === "level" ? m.settings_ui_ranking_scope_level() : m.settings_ui_ranking_scope_org()}</SelectTrigger><SelectContent><SelectItem value="class">{m.settings_ui_ranking_scope_class()}</SelectItem><SelectItem value="division">{m.settings_ui_ranking_scope_division()}</SelectItem><SelectItem value="level">{m.settings_ui_ranking_scope_level()}</SelectItem><SelectItem value="org">{m.settings_ui_ranking_scope_org_full()}</SelectItem></SelectContent></Select></div><div class="space-y-1.5"><Label>{m.settings_ui_ranking_metric()}</Label><Select type="single" value={form.ranking_metric} onValueChange={(v) => { if (v) form.ranking_metric = v; }}><SelectTrigger class="w-full">{form.ranking_metric === "total_score" ? m.settings_ui_ranking_metric_total_score() : form.ranking_metric === "percentage" ? m.settings_ui_ranking_metric_percentage() : m.settings_ui_ranking_metric_gpa()}</SelectTrigger><SelectContent><SelectItem value="total_score">{m.settings_ui_ranking_metric_total_score()}</SelectItem><SelectItem value="percentage">{m.settings_ui_ranking_metric_percentage()}</SelectItem><SelectItem value="gpa">{m.settings_ui_ranking_metric_gpa()}</SelectItem></SelectContent></Select></div><div class="space-y-1.5"><Label>{m.settings_ui_tie_handling()}</Label><Select type="single" value={form.tie_handling} onValueChange={(v) => { if (v) form.tie_handling = v; }}><SelectTrigger class="w-full">{form.tie_handling === "same_rank" ? m.settings_ui_tie_same_rank() : form.tie_handling === "sequential" ? m.settings_ui_tie_sequential() : m.settings_ui_tie_skip()}</SelectTrigger><SelectContent><SelectItem value="same_rank">{m.settings_ui_tie_same_rank()}</SelectItem><SelectItem value="sequential">{m.settings_ui_tie_sequential()}</SelectItem><SelectItem value="skip">{m.settings_ui_tie_skip()}</SelectItem></SelectContent></Select></div><div class="flex items-center justify-between pt-2"><Label>{m.settings_ui_ranking_policy_enabled()}</Label><Switch checked={form.is_enabled === 1} onCheckedChange={(c) => (form.is_enabled = c ? 1 : 0)} /></div><div class="flex items-center justify-between"><Label>{m.settings_ui_ranking_include_failed_students()}</Label><Switch checked={form.include_failed_students === 1} onCheckedChange={(c) => (form.include_failed_students = c ? 1 : 0)} /></div></div><DialogFooter><Button variant="outline" onclick={() => (open = false)}>{m.common_action_cancel()}</Button><Button onclick={onsubmit} disabled={isSaving || !form.name.trim()}>{#if isSaving}<ArrowPath class="h-4 w-4 animate-spin mr-1" />{/if}{m.common_action_save()}</Button></DialogFooter></DialogContent></Dialog>
