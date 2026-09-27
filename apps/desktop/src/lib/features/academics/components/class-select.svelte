<!--
  @component ClassSelect
  Self-fetching Searchable Class Selector powered by TanStack Query & Academic Service.
  Includes a sticky search bar and optional "Add new class" creation button.
-->
<script lang="ts">
import { page } from "$app/state";
import { l } from "$lib/core/i18n";
import { createClassesQuery } from "$lib/features/academics/queries";
import * as m from "$lib/paraglide/messages";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { authStore } from "$lib/stores";

interface Props {
  value?: string | null;
  orgId?: string;
  placeholder?: string;
  searchPlaceholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  createLabel?: string;
  onCreate?: () => void;
  class?: string;
  onchange?: (val: string | null) => void;
}

let {
  value = $bindable<string | null>(null),
  orgId,
  placeholder = m.common_form_select_class?.() || "اختر الفصل...",
  searchPlaceholder = "ابحث عن فصل أو مرحلة...",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = m.common_filter_all_classes?.() || "جميع الفصول",
  createLabel = "إضافة فصل جديد...",
  onCreate = undefined,
  class: className = "",
  onchange = undefined,
}: Props = $props();

const resolvedOrgId = $derived(
  orgId || page.params.orgId || authStore.orgId || "default",
);

// Self-contained TanStack Query
const classesQuery = createClassesQuery(() => resolvedOrgId);

const classes = $derived(classesQuery.data ?? []);

function getClassLabel(cls: any): string {
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

function getClassSubtitle(cls: any): string | undefined {
  if (cls?.code) return cls.code;
  if (cls?.grade_level || cls?.level) {
    return l(cls.grade_level_ar || cls.level_ar, cls.grade_level || cls.level);
  }
  return undefined;
}
</script>

<SearchableSelect
  bind:value
  items={classes}
  {placeholder}
  {searchPlaceholder}
  emptyText={m.common_classes_empty_title?.() || "لا توجد فصول مطابقة"}
  {disabled}
  {id}
  {showAllOption}
  {allOptionLabel}
  {createLabel}
  {onCreate}
  class={className}
  getItemValue={(cls) => String(cls?.id ?? "")}
  getItemLabel={getClassLabel}
  getItemSubtitle={getClassSubtitle}
  {onchange}
/>
