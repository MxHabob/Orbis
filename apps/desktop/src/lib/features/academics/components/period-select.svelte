<!--
  @component PeriodSelect
  Self-fetching Searchable Academic Period Selector powered by TanStack Query & Academic Service.
  Includes a sticky search bar and optional "Add new period" creation button.
-->
<script lang="ts">
import { page } from "$app/state";
import { formatDate, l } from "$lib/core/i18n";
import { createPeriodsQuery } from "$lib/features/academics/queries";
import * as m from "$lib/paraglide/messages";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { authStore } from "$lib/stores";

interface Props {
  value?: string | null;
  orgId?: string;
  placeholder?: string;
  searchPlaceholder?: string;
  periodType?: "academic_year" | "term" | "semester" | "quarter" | string;
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
  placeholder = m.common_metrics_period(),
  searchPlaceholder = m.shared_ui_search_periods_placeholder(),
  periodType = undefined,
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = m.common_ledger_period_all(),
  createLabel = m.shared_ui_add_academic_period(),
  onCreate = undefined,
  class: className = "",
  onchange = undefined,
}: Props = $props();

const resolvedOrgId = $derived(
  orgId || page.params.orgId || authStore.orgId || "default",
);

// Self-contained TanStack Query
const periodsQuery = createPeriodsQuery(() => resolvedOrgId);

const periods = $derived.by(() => {
  const all = periodsQuery.data ?? [];
  if (!periodType) return all;
  if (periodType === "academic_year" || periodType === "year") {
    return all.filter(
      (p) => p.period_type === "academic_year" || p.period_type === "year",
    );
  }
  return all.filter((p) => p.period_type === periodType);
});

function getPeriodLabel(p: any): string {
  if (!p) return "";
  const name = l(p.name_ar || p.nameAr, p.name || p.nameEn || p.name_en);
  if (name) return name;
  if (p.code) return String(p.code);
  return String(p.id || "");
}

function getPeriodSubtitle(p: any): string | undefined {
  if (p?.is_active || p?.status === "active") return m.shared_ui_current_active_period();
  if (p?.start_date && p?.end_date) {
    return `${formatDate(p.start_date, "date")} — ${formatDate(p.end_date, "date")}`;
  }
  if (p?.start_date) return formatDate(p.start_date, "date");
  return undefined;
}
</script>

<SearchableSelect
  bind:value
  items={periods}
  {placeholder}
  {searchPlaceholder}
  emptyText={m.shared_ui_no_matching_periods()}
  {disabled}
  {id}
  {showAllOption}
  {allOptionLabel}
  {createLabel}
  {onCreate}
  class={className}
  getItemValue={(p) => String(p?.id ?? "")}
  getItemLabel={getPeriodLabel}
  getItemSubtitle={getPeriodSubtitle}
  {onchange}
/>
