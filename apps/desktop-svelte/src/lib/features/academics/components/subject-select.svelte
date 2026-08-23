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
  subjects = [],
  placeholder = "",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = "",
  class: className = "",
  onchange = undefined,
}: {
  value?: string | null;
  subjects?: any[];
  placeholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  class?: string;
  onchange?: (val: string | null) => void;
} = $props();

function getSubjectLabel(subj: any) {
  if (!subj) return "";
  const name = l(subj.name_ar || subj.nameAr, subj.name || subj.nameEn || subj.name_en);
  if (name) return name;
  if (subj.code) return String(subj.code);
  return String(subj.id || "");
}

const selectedSubject = $derived(
  subjects.find((s) => String(s.id) === String(value || "")),
);

const displayLabel = $derived(
  value === "all" || value === "" || !value
    ? showAllOption
      ? allOptionLabel || "جميع المواد"
      : placeholder || (m.common_cs_form_select_subject ? m.common_cs_form_select_subject() : "اختر المادة...")
    : selectedSubject
      ? getSubjectLabel(selectedSubject)
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
        {allOptionLabel || "جميع المواد"}
      </SelectItem>
    {/if}
    {#each subjects as subj (subj.id)}
      <SelectItem
        value={String(subj.id)}
        class="text-xs text-start justify-start cursor-pointer"
      >
        {getSubjectLabel(subj)}
      </SelectItem>
    {/each}
    {#if subjects.length === 0 && !showAllOption}
      <div class="px-3 py-2 text-xs text-muted-foreground text-start">
        {m.common_status_undefined() === "غير محدد" ? "لا توجد مواد مضافة" : "No subjects available"}
      </div>
    {/if}
  </SelectContent>
</Select>
