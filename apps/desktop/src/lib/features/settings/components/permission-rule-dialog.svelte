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
import { ShieldAlert, Plus, Check, SlidersHorizontal, Lock } from "@lucide/svelte";
import { l } from "$lib/core/i18n";
import {
  IAM_DOMAINS,
  SYSTEM_ROLE_DEFINITIONS,
  getActionsForResource,
  getResourceLabel,
  getActionBadgeVariant,
} from "$lib/core/iam";

let {
  open = $bindable(false),
  docType = $bindable("organization"),
  role = $bindable("super_admin"),
  level = $bindable(10),
  onlyIfCreator = $bindable(false),
  actions = $bindable<string[]>([]),
  availableRoles = [],
  onsubmit,
}: {
  open: boolean;
  docType: string;
  role: string;
  level: number;
  onlyIfCreator: boolean;
  actions: string[];
  availableRoles: Array<{ id: string; name: string; name_ar?: string }>;
  onsubmit: () => void;
} = $props();

// All resources flattened with domain label
const allResources = $derived.by(() => {
  const list: Array<{ key: string; labelAr: string; labelEn: string; domainAr: string; domainEn: string }> = [];
  for (const domain of IAM_DOMAINS) {
    for (const res of domain.resources) {
      list.push({
        key: res.key,
        labelAr: res.labelAr,
        labelEn: res.labelEn,
        domainAr: domain.nameAr,
        domainEn: domain.nameEn,
      });
    }
  }
  return list;
});

// Allowed actions for current docType
const availableActions = $derived(getActionsForResource(docType || "organization"));

function toggleAction(actionKey: string) {
  if (actions.includes(actionKey)) {
    actions = actions.filter((a) => a !== actionKey);
  } else {
    actions = [...actions, actionKey];
  }
}

function selectAllActions() {
  actions = availableActions.map((a) => a.key);
}

function clearActions() {
  actions = [];
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-lg text-start bg-card border border-border/80 shadow-2xl p-6">
    <!-- Header -->
    <DialogHeader class="space-y-1.5">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <ShieldAlert class="w-4 h-4" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            {l("إضافة قاعدة أمان متقدمة", "Add Security Rule")}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            {l(
              "تخصيص سياسة وصول دقيقة لدور محدد على مورد معين بالمنظومة",
              "Define granular access policies for a specific role and resource"
            )}
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-3">
      <!-- Role & Resource Pickers -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <!-- Target Role -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-semibold text-foreground">
            {l("الدور المستهدف", "Target Role")} <span class="text-destructive">*</span>
          </Label>
          <Select
            type="single"
            value={role}
            onValueChange={(v) => {
              if (v) role = v;
            }}
          >
            <SelectTrigger class="h-9 w-full text-xs font-medium">
              {role || l("اختر الدور", "Select Role")}
            </SelectTrigger>
            <SelectContent class="max-h-56">
              {#each availableRoles as r}
                <SelectItem value={r.id} class="text-xs">
                  {l(r.name_ar || r.name, r.name)}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- Resource (DocType) -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label class="text-xs font-semibold text-foreground">
            {l("المورد / المستند", "Resource / DocType")} <span class="text-destructive">*</span>
          </Label>
          <Select
            type="single"
            value={docType}
            onValueChange={(v) => {
              if (v) {
                docType = v;
                actions = [];
              }
            }}
          >
            <SelectTrigger class="h-9 w-full text-xs font-medium">
              {getResourceLabel(docType) || docType}
            </SelectTrigger>
            <SelectContent class="max-h-60">
              {#each allResources as res}
                <SelectItem value={res.key} class="text-xs">
                  <div class="flex items-center justify-between gap-3 w-full">
                    <span>{l(res.labelAr, res.labelEn)}</span>
                    <span class="text-[10px] text-muted-foreground font-mono">({res.key})</span>
                  </div>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Precedence Level & Conditions Card -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/60 bg-muted/20 items-center">
        <div class="flex flex-col gap-1 text-start">
          <Label for="rule_precedence_level" class="text-xs font-semibold text-foreground">
            {l("مستوى الأسبقية (Precedence Level)", "Precedence Level")}
          </Label>
          <Input
            id="rule_precedence_level"
            type="number"
            bind:value={level}
            min={0}
            max={100}
            class="h-8 text-xs font-mono w-28"
          />
        </div>

        <div class="flex items-center gap-2 pt-2 sm:pt-0">
          <Checkbox
            id="only_creator_check"
            checked={onlyIfCreator}
            onCheckedChange={(checked) => (onlyIfCreator = Boolean(checked))}
          />
          <Label for="only_creator_check" class="text-xs font-medium text-foreground cursor-pointer">
            {l("سريان الصلاحية لمنشئ السجل فقط", "Only apply if user is creator")}
          </Label>
        </div>
      </div>

      <!-- Actions Toggle Section -->
      <div class="space-y-2 pt-1">
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-foreground">
            {l("العمليات المسموح بها:", "Allowed Actions:")}
          </span>
          <div class="flex items-center gap-2">
            <Button
              variant="ghost"
              size="sm"
              class="h-6 text-[11px] px-2 text-muted-foreground hover:text-foreground"
              onclick={selectAllActions}
            >
              {l("تحديد الكل", "All")}
            </Button>
            <Button
              variant="ghost"
              size="sm"
              class="h-6 text-[11px] px-2 text-muted-foreground hover:text-destructive"
              onclick={clearActions}
            >
              {l("إلغاء", "None")}
            </Button>
          </div>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-3 gap-2 max-h-48 overflow-y-auto pr-1 py-1">
          {#each availableActions as act}
            {@const isChecked = actions.includes(act.key)}
            {@const badgeVariant = getActionBadgeVariant(act.key)}

            <button
              type="button"
              onclick={() => toggleAction(act.key)}
              class="flex items-center justify-between gap-2 p-2.5 rounded-lg border text-xs font-medium text-start transition-all cursor-pointer select-none {isChecked
                ? 'border-primary/50 bg-primary/10 text-foreground ring-1 ring-primary/20'
                : 'border-border/60 bg-card/60 text-muted-foreground hover:bg-muted/40 hover:text-foreground'}"
            >
              <div class="flex items-center gap-2 min-w-0">
                <div
                  class="w-3.5 h-3.5 rounded flex items-center justify-center shrink-0 {isChecked
                    ? 'bg-primary text-primary-foreground'
                    : 'border border-muted-foreground/30'}"
                >
                  {#if isChecked}
                    <Check class="w-2.5 h-2.5" />
                  {/if}
                </div>
                <span class="truncate">{l(act.labelAr, act.labelEn)}</span>
              </div>

              <Badge variant={badgeVariant} class="text-[9px] font-mono px-1 py-0 uppercase shrink-0">
                {act.key}
              </Badge>
            </button>
          {/each}
        </div>
      </div>
    </div>

    <!-- Footer -->
    <DialogFooter class="border-t border-border pt-4 mt-2 flex items-center justify-between">
      <Button
        variant="ghost"
        size="sm"
        class="h-8 text-xs font-medium"
        onclick={() => (open = false)}
      >
        {m.common_action_cancel ? m.common_action_cancel() : l("إلغاء", "Cancel")}
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="h-8 text-xs font-bold gap-1.5 px-4 shadow-sm"
        disabled={!docType || actions.length === 0}
        onclick={onsubmit}
      >
        <Plus class="w-3.5 h-3.5" />
        {l("إضافة القاعدة", "Add Rule")}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
