<!--
  @component SubjectSelect
  Self-fetching Searchable Subject Selector powered by TanStack Query & Academic Service.
  Includes a sticky search bar and optional "Add new subject" creation button.
-->
<script lang="ts">
import { page } from "$app/state";
import { l } from "$lib/core/i18n";
import { createSubjectsQuery } from "$lib/features/academics/queries";
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
  placeholder = m.common_cs_form_select_subject(),
  searchPlaceholder = m.shared_ui_search_subjects_placeholder(),
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = m.shared_ui_all_subjects(),
  createLabel = m.shared_ui_add_subject(),
  onCreate = undefined,
  class: className = "",
  onchange = undefined,
}: Props = $props();

const resolvedOrgId = $derived(
  orgId || page.params.orgId || authStore.orgId || "default",
);

// Self-contained TanStack Query
const subjectsQuery = createSubjectsQuery(() => resolvedOrgId);

const subjects = $derived(subjectsQuery.data ?? []);

function getSubjectLabel(subj: any): string {
  if (!subj) return "";
  const name = l(
    subj.name_ar || subj.nameAr,
    subj.name || subj.nameEn || subj.name_en,
  );
  if (name) return name;
  if (subj.code) return String(subj.code);
  return String(subj.id || "");
}

function getSubjectSubtitle(subj: any): string | undefined {
  if (subj?.code) return subj.code;
  if (subj?.credits) return m.shared_ui_credit_hours({ count: subj.credits });
  return undefined;
}
</script>

<SearchableSelect
  bind:value
  items={subjects}
  {placeholder}
  {searchPlaceholder}
  emptyText={m.shared_ui_no_matching_subjects()}
  {disabled}
  {id}
  {showAllOption}
  {allOptionLabel}
  {createLabel}
  {onCreate}
  class={className}
  getItemValue={(s) => String(s?.id ?? "")}
  getItemLabel={getSubjectLabel}
  getItemSubtitle={getSubjectSubtitle}
  {onchange}
/>
