<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
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
  isSaving = false,
  getRuleLabel,
  getRoundingLabel,
  onsubmit,
}: {
  open: boolean;
  form: any;
  editingId?: string | null;
  isSaving?: boolean;
  getRuleLabel: (value: string) => string;
  getRoundingLabel: (value: string) => string;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-md w-full min-w-0"><DialogHeader><DialogTitle class="text-sm">{editingId ? m.settings_ui_calculation_policy_edit_title() : m.settings_ui_calculation_policy_create_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-4 py-2 w-full min-w-0"><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">{m.settings_ui_policy_name()}</Label><Input bind:value={form.name} placeholder={m.settings_ui_calculation_policy_name_example()} class="text-xs w-full min-w-0" /></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">{m.settings_ui_calculation_rule_type()}</Label><Select type="single" value={form.rule_type} onValueChange={(v) => { if (v) form.rule_type = v; }}><SelectTrigger class="text-xs h-9 w-full min-w-0">{getRuleLabel(form.rule_type)}</SelectTrigger><SelectContent class="text-xs"><SelectItem value="weighted_sum">{m.settings_ui_calculation_rule_weighted_sum()}</SelectItem><SelectItem value="simple_average">{m.settings_ui_calculation_rule_simple_average()}</SelectItem><SelectItem value="custom_ast">{m.settings_ui_calculation_rule_custom()}</SelectItem></SelectContent></Select></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">{m.settings_ui_calculation_formula()}</Label><Textarea bind:value={form.formula_expression} class="text-xs font-mono min-h-15 w-full min-w-0 max-w-full break-all" placeholder="SUM(score * weight) / SUM(weight)" /></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">{m.settings_ui_calculation_rounding_policy()}</Label><Select type="single" value={form.rounding_policy} onValueChange={(v) => { if (v) form.rounding_policy = v; }}><SelectTrigger class="text-xs h-9 w-full min-w-0">{getRoundingLabel(form.rounding_policy)}</SelectTrigger><SelectContent class="text-xs"><SelectItem value="round_half_up">{m.settings_ui_calculation_round_half_up()}</SelectItem><SelectItem value="round_half_down">{m.settings_ui_calculation_round_half_down()}</SelectItem><SelectItem value="truncate">{m.settings_ui_calculation_truncate()}</SelectItem><SelectItem value="ceiling">{m.settings_ui_calculation_ceiling()}</SelectItem><SelectItem value="floor">{m.settings_ui_calculation_floor()}</SelectItem></SelectContent></Select></div></div><DialogFooter class="flex items-center justify-end w-full pt-2"><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isSaving || !form.name.trim()}>{#if isSaving}<LoadingIndicator class="me-1"  size={12} />{/if}{editingId ? m.common_action_update() : m.common_action_create()}</Button></DialogFooter></DialogContent></Dialog>
