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
  teachers = [],
  placeholder = "",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = "",
  class: className = "",
  onchange = undefined,
}: {
  value?: string | null;
  teachers?: any[];
  placeholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  class?: string;
  onchange?: (val: string | null) => void;
} = $props();

function getTeacherLabel(t: any) {
  if (!t) return "";
  const arName = t.name_ar || t.nameAr || t.full_name;
  const enName = t.name_en || t.nameEn || t.full_name || t.email;
  const localized = l(arName, enName);
  if (localized) return localized;
  return String(t.full_name || t.email || t.id || "");
}

const selectedTeacher = $derived(
  teachers.find((t) => String(t.id) === String(value || "")),
);

const displayLabel = $derived(
  value === "all" || value === "" || !value
    ? showAllOption
      ? allOptionLabel || m.common_cs_form_all_teachers()
      : placeholder || m.common_cs_form_select_teacher()
    : selectedTeacher
      ? getTeacherLabel(selectedTeacher)
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
        {allOptionLabel || m.common_cs_form_all_teachers()}
      </SelectItem>
    {/if}
    {#each teachers as t (t.id)}
      <SelectItem
        value={String(t.id)}
        class="text-xs text-start justify-start cursor-pointer"
      >
        {getTeacherLabel(t)}
      </SelectItem>
    {/each}
    {#if teachers.length === 0 && !showAllOption}
      <div class="px-3 py-2 text-xs text-muted-foreground text-start">
        {m.common_cs_form_no_teachers()}
      </div>
    {/if}
  </SelectContent>
</Select>
