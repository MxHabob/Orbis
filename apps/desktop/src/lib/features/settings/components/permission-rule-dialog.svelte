<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
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
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { ShieldAlert, Plus, Check } from "@lucide/svelte";

let {
  open = $bindable(false),
  docType = $bindable(""),
  role = $bindable("super_admin"),
  level = $bindable(0),
  onlyIfCreator = $bindable(false),
  actions = $bindable<string[]>([]),
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

const availableDocActions = $derived(
  getActionsForDocType(docType || "Organization")
);

function toggleAction(action: string) {
  if (actions.includes(action)) {
    actions = actions.filter((a) => a !== action);
  } else {
    actions = [...actions, action];
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-lg text-start bg-card border border-border/80 shadow-2xl p-6">
    <DialogHeader class="space-y-1.5">
      <div class="flex items-center gap-2">
        <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <ShieldAlert class="w-4 h-4" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            {m.settings_permissions_dialog_add_title()}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground">
            تخصيص قاعدة أمان دقيقة لدور معين على مورد مستندي محدد
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-3">
      <!-- Doc Type & Role Row -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5 text-start">
          <Label for="rule_doc_type" class="text-xs font-semibold text-foreground">
            {m.settings_permissions_col_doc_type()}
          </Label>
          <Select
            type="single"
            value={docType}
            onValueChange={(v) => {
              docType = v ?? "";
              actions = [];
            }}
          >
            <SelectTrigger class="h-9 w-full text-xs font-medium">
              {docType || "اختر نوع المستند/المورد"}
            </SelectTrigger>
            <SelectContent class="max-h-56">
              {#each standardDocTypes as type}
                <SelectItem value={type} class="text-xs font-mono">{type}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-1.5 text-start">
          <Label for="rule_role" class="text-xs font-semibold text-foreground">
            {m.settings_permissions_label_role()}
          </Label>
          <Select
            type="single"
            value={role}
            onValueChange={(v) => (role = v ?? "super_admin")}
          >
            <SelectTrigger class="h-9 w-full text-xs font-medium">
              {role}
            </SelectTrigger>
            <SelectContent class="max-h-56">
              {#each standardRoles as item}
                <SelectItem value={item} class="text-xs">{item}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Level & Condition -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 items-center p-3 rounded-lg border border-border/60 bg-muted/20">
        <div class="flex flex-col gap-1 text-start">
          <Label for="rule_level" class="text-xs font-semibold text-foreground">
            مستوى الأسبقية (Level)
          </Label>
          <Input
            id="rule_level"
            type="number"
            bind:value={level}
            min={0}
            max={100}
            class="h-8 text-xs font-mono w-24"
          />
        </div>

        <div class="flex items-center gap-2 pt-3 sm:pt-0">
          <Checkbox
            id="only_creator_check"
            checked={onlyIfCreator}
            onCheckedChange={(checked) => (onlyIfCreator = Boolean(checked))}
          />
          <Label for="only_creator_check" class="text-xs font-medium text-foreground cursor-pointer">
            {m.settings_permissions_only_creator()}
          </Label>
        </div>
      </div>

      <!-- Allowed Actions -->
      <div class="space-y-2 pt-1">
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-foreground">
            {m.settings_permissions_label_allowed_actions()}
          </span>
          <span class="text-[10px] text-muted-foreground font-mono">
            {actions.length} مختار
          </span>
        </div>

        {#if availableDocActions.length === 0}
          <div class="p-3 text-center text-xs text-muted-foreground border border-dashed rounded-lg">
            الرجاء اختيار نوع المستند أولاً لعرض العمليات المتاحة
          </div>
        {:else}
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2 max-h-44 overflow-y-auto pr-1">
            {#each availableDocActions as action}
              {@const isChecked = actions.includes(action)}
              <button
                type="button"
                onclick={() => toggleAction(action)}
                class="flex items-center gap-2 p-2 rounded-lg border text-xs font-medium text-start transition-all cursor-pointer select-none {isChecked
                  ? 'border-primary/40 bg-primary/10 text-primary font-semibold'
                  : 'border-border/60 bg-background/60 text-muted-foreground hover:bg-muted/50 hover:text-foreground'}"
              >
                <div class="w-3.5 h-3.5 rounded flex items-center justify-center {isChecked ? 'bg-primary text-primary-foreground' : 'border border-muted-foreground/40'}">
                  {#if isChecked}
                    <Check class="w-2.5 h-2.5" />
                  {/if}
                </div>
                <span class="truncate font-mono text-[11px]">{capitalizeFirstLetter(action)}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <DialogFooter class="border-t border-border pt-4 mt-2 flex items-center justify-between">
      <Button
        variant="ghost"
        size="sm"
        class="h-8 text-xs font-medium"
        onclick={() => (open = false)}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        variant="default"
        size="sm"
        class="h-8 text-xs font-bold gap-1.5 shadow-sm"
        disabled={!docType || actions.length === 0}
        onclick={onsubmit}
      >
        <Plus class="w-3.5 h-3.5" />
        {m.settings_permissions_btn_add()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
