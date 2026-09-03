<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
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
  docType = $bindable(""),
  role = $bindable("Academics User"),
  level = $bindable(0),
  onlyIfCreator = $bindable(false),
  actions = $bindable([] as string[]),
  standardDocTypes,
  standardRoles,
  getActionsForDocType,
  capitalizeFirstLetter,
  onsubmit,
}: {
  open: boolean;
  docType: string;
  role: string;
  level: number;
  onlyIfCreator: boolean;
  actions: string[];
  standardDocTypes: string[];
  standardRoles: string[];
  getActionsForDocType: (type: string) => string[];
  capitalizeFirstLetter: (value: string) => string;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-lg text-start"><DialogHeader><DialogTitle>{m.settings_permissions_dialog_add_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-4 py-2"><div class="flex flex-col gap-1.5 text-start"><Label for="rule_doc_type" class="text-xs font-medium text-muted-foreground">{m.settings_permissions_col_doc_type()}</Label><Select type="single" value={docType} onValueChange={(v) => (docType = v ?? "")}><SelectTrigger class="h-9 w-full text-xs">{docType || "Select Document Type"}</SelectTrigger><SelectContent>{#each standardDocTypes as type}<SelectItem value={type}>{type}</SelectItem>{/each}</SelectContent></Select></div><div class="flex flex-col gap-1.5 text-start"><Label for="rule_role" class="text-xs font-medium text-muted-foreground">{m.settings_permissions_label_role()}</Label><Select type="single" value={role} onValueChange={(v) => (role = v ?? "Academics User")}><SelectTrigger class="h-9 w-full text-xs">{role}</SelectTrigger><SelectContent>{#each standardRoles as item}<SelectItem value={item}>{item}</SelectItem>{/each}</SelectContent></Select></div><div class="grid grid-cols-2 gap-4 items-end"><div class="flex flex-col gap-1.5 text-start"><Label for="rule_level" class="text-xs font-medium text-muted-foreground">{m.academics_exams_level()}</Label><Input id="rule_level" type="number" bind:value={level} class="h-9 text-xs" /></div><div class="pb-2"><label class="inline-flex items-center gap-2 cursor-pointer select-none"><Checkbox checked={onlyIfCreator} onCheckedChange={(checked) => (onlyIfCreator = Boolean(checked))} /><span class="text-xs text-muted-foreground">{m.settings_permissions_only_creator()}</span></label></div></div><div class="flex flex-col gap-2 border-t border-border/40 pt-4 mt-2 text-start"><span class="text-xs font-semibold text-muted-foreground mb-1">{m.settings_permissions_label_allowed_actions()}</span><div class="grid grid-cols-3 gap-2.5 max-h-40 overflow-y-auto pr-1">{#each getActionsForDocType(docType || "Default") as action}<label class="inline-flex items-center gap-2 text-xs text-muted-foreground hover:text-foreground cursor-pointer select-none font-medium"><Checkbox checked={actions.includes(action)} onCheckedChange={(checked) => { actions = checked ? [...actions, action] : actions.filter((item) => item !== action); }} /><span>{capitalizeFirstLetter(action)}</span></label>{/each}</div></div></div><DialogFooter class="border-t border-border pt-4 mt-2"><Button variant="ghost" size="sm" onclick={() => (open = false)}>{m.common_action_cancel()}</Button><Button variant="lime" size="sm" class="font-bold" onclick={onsubmit}>{m.settings_permissions_btn_add()}</Button></DialogFooter></DialogContent></Dialog>
