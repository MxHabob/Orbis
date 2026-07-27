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

const GRADE_LEVEL_NAMES: Record<string, { en: string; ar: string }> = {
  kg1: { en: "KG 1", ar: "روضة أولى" },
  kg2: { en: "KG 2", ar: "روضة ثانية" },
  grade1: { en: "Grade 1", ar: "الصف الأول" },
  grade2: { en: "Grade 2", ar: "الصف الثاني" },
  grade3: { en: "Grade 3", ar: "الصف الثالث" },
  grade4: { en: "Grade 4", ar: "الصف الرابع" },
  grade5: { en: "Grade 5", ar: "الصف الخامس" },
  grade6: { en: "Grade 6", ar: "الصف السادس" },
  grade7: { en: "Grade 7", ar: "الصف السابع" },
  grade8: { en: "Grade 8", ar: "الصف الثامن" },
  grade9: { en: "Grade 9", ar: "الصف التاسع" },
  grade10: { en: "Grade 10", ar: "الصف العاشر" },
  grade11: { en: "Grade 11", ar: "الصف الحادي عشر" },
  grade12: { en: "Grade 12", ar: "الصف الثاني عشر" },
};

function formatGradeLevel(level?: string | null): string {
  if (!level) return "";
  const key = String(level).trim().toLowerCase();
  const found = GRADE_LEVEL_NAMES[key];
  return found ? l(found.ar, found.en) : String(level);
}

function getClassLabel(cls: any) {
  if (!cls) return "";
  const name = l(cls.name_ar || cls.nameAr, cls.name || cls.nameEn);
  if (name) return name;
  if (cls.level) return formatGradeLevel(cls.level);
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
