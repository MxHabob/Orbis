<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

let {
  value = $bindable<string | null>(null),
  classes = [],
  placeholder = "",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = "",
  class: className = "",
  onchange = undefined,
}: {
  value?: string | null;
  classes?: any[];
  placeholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  class?: string;
  onchange?: (val: string | null) => void;
} = $props();

function getClassLabel(cls: any) {
  if (!cls) return "";
  const name = l(cls.name_ar || cls.nameAr, cls.name || cls.nameEn);
  if (name) return name;
  const level = l(
    cls.grade_level_ar || cls.level_ar,
    cls.grade_level || cls.level,
  );
  if (level) return level;
  return String(cls.id || "");
}

const selectedClass = $derived(
  classes.find((c) => String(c.id) === String(value || "")),
);

const displayLabel = $derived(
  value === "all" || value === "" || !value
    ? showAllOption
      ? allOptionLabel || (m.common_filter_all_classes ? m.common_filter_all_classes() : "جميع الصفوف")
      : placeholder || (m.common_form_select_class ? m.common_form_select_class() : "اختر الصف الدراسي...")
    : selectedClass
      ? getClassLabel(selectedClass)
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
        {allOptionLabel || (m.common_filter_all_classes ? m.common_filter_all_classes() : "جميع الصفوف")}
      </SelectItem>
    {/if}
    {#each classes as cls (cls.id)}
      <SelectItem
        value={String(cls.id)}
        class="text-xs text-start justify-start cursor-pointer"
      >
        {getClassLabel(cls)}
      </SelectItem>
    {/each}
    {#if classes.length === 0 && !showAllOption}
      <div class="px-3 py-2 text-xs text-muted-foreground text-start">
        {m.common_status_undefined() === "غير محدد" ? "لا توجد صفوف مضافة" : "No classes available"}
      </div>
    {/if}
  </SelectContent>
</Select>
