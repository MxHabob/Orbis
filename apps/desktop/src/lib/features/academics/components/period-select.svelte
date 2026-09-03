<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";

let {
  value = $bindable<string | null>(null),
  periods = [],
  placeholder = "",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = "",
  class: className = "",
  onchange = undefined,
}: {
  value?: string | null;
  periods?: any[];
  placeholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  class?: string;
  onchange?: (val: string | null) => void;
} = $props();

function getPeriodLabel(p: any) {
  if (!p) return "";
  const name = l(p.name_ar || p.nameAr, p.name || p.nameEn || p.name_en);
  if (name) return name;
  if (p.code) return String(p.code);
  return String(p.id || "");
}

const selectedPeriod = $derived(
  periods.find((p) => String(p.id) === String(value || "")),
);

const displayLabel = $derived(
  value === "all" || value === "" || !value
    ? showAllOption
      ? allOptionLabel || (m.common_ledger_period_all ? m.common_ledger_period_all() : "جميع الفترات")
      : placeholder || (m.common_metrics_period ? m.common_metrics_period() : "اختر الفترة الدراسية...")
    : selectedPeriod
      ? getPeriodLabel(selectedPeriod)
      : value,
);

function handleValueChange(newVal: string | undefined) {
  const val = newVal ?? null;
  value = val;
  if (onchange) {
    onchange(val);
  }
}
</script>

<Select
  type="single"
  value={value ?? ""}
  onValueChange={handleValueChange}
  {disabled}
>
  <SelectTrigger
    {id}
    class="w-full h-9 text-xs text-start flex items-center justify-between bg-background {className}"
  >
    <span class="truncate">{displayLabel}</span>
  </SelectTrigger>
  <SelectContent class="text-start max-h-56 overflow-y-auto" align="end">
    {#if showAllOption}
      <SelectItem
        value="all"
        class="text-xs text-start justify-start cursor-pointer"
      >
        {allOptionLabel || (m.common_ledger_period_all ? m.common_ledger_period_all() : "جميع الفترات")}
      </SelectItem>
    {/if}
    {#each periods as p (p.id)}
      <SelectItem
        value={String(p.id)}
        class="text-xs text-start justify-start cursor-pointer"
      >
        {getPeriodLabel(p)}
      </SelectItem>
    {/each}
    {#if periods.length === 0 && !showAllOption}
      <div class="px-3 py-2 text-xs text-muted-foreground text-start">
        {m.common_status_undefined() === "غير محدد" ? "لا توجد فترات مضافة" : "No periods available"}
      </div>
    {/if}
  </SelectContent>
</Select>
