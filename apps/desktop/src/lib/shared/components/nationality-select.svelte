<!--
  @component NationalitySelect
  Dedicated Nationality & Country selector powered by @orbis/location and SearchableSelect.
  Supports bilingual Arabic/English nationality names, flags, search, and ISO codes.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { countries } from "@orbis/location/countries-intl";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";

interface Props {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  id?: string;
  onchange?: (val: string | null) => void;
}

let {
  value = $bindable(""),
  placeholder = m.common_select_nationality(),
  disabled = false,
  size = "default",
  class: className = "",
  id = undefined,
  onchange,
}: Props = $props();

// Common Arabic nationalities dictionary mapping ISO alpha2 to Arabic nationality / demonym
const NATIONALITY_MESSAGES: Record<string, () => string> = {
  SA: m.nationality_sa,
  YE: m.nationality_ye,
  EG: m.nationality_eg,
  AE: m.nationality_ae,
  JO: m.nationality_jo,
  SY: m.nationality_sy,
  LB: m.nationality_lb,
  PS: m.nationality_ps,
  KW: m.nationality_kw,
  QA: m.nationality_qa,
  BH: m.nationality_bh,
  OM: m.nationality_om,
  IQ: m.nationality_iq,
  SD: m.nationality_sd,
  MA: m.nationality_ma,
  DZ: m.nationality_dz,
  TN: m.nationality_tn,
  LY: m.nationality_ly,
  SO: m.nationality_so,
  DJ: m.nationality_dj,
  MR: m.nationality_mr,
  IN: m.nationality_in,
  PK: m.nationality_pk,
  BD: m.nationality_bd,
  PH: m.nationality_ph,
  ID: m.nationality_id,
  MY: m.nationality_my,
  TR: m.nationality_tr,
  US: m.nationality_us,
  GB: m.nationality_gb,
  CA: m.nationality_ca,
  AU: m.nationality_au,
  FR: m.nationality_fr,
  DE: m.nationality_de,
  IT: m.nationality_it,
  ES: m.nationality_es,
  RU: m.nationality_ru,
  CN: m.nationality_cn,
  JP: m.nationality_jp,
  KR: m.nationality_kr,
  BR: m.nationality_br,
  MX: m.nationality_mx,
  NG: m.nationality_ng,
  ET: m.nationality_et,
  ZA: m.nationality_za,
};

// Priority countries displayed at top of list
const PRIORITY_CODES = [
  "YE",
  "SA",
  "EG",
  "AE",
  "JO",
  "SY",
  "LB",
  "PS",
  "KW",
  "QA",
  "BH",
  "OM",
  "IQ",
  "SD",
  "MA",
  "IN",
  "PK",
  "BD",
  "PH",
  "US",
  "GB",
];

interface CountryItem {
  alpha2: string;
  alpha3: string;
  name: string;
  localDemonym: string;
  emoji: string;
  demonym: string;
}

const countryList = $derived.by<CountryItem[]>(() => {
  const mapped: CountryItem[] = countries.map((c) => {
    const known = NATIONALITY_MESSAGES[c.alpha2];
    const demonym = known ? known() : c.name;
    const localDemonym = demonym;
    return {
      alpha2: c.alpha2,
      alpha3: c.alpha3,
      name: c.name,
      localDemonym,
      emoji: c.emoji || "🌐",
      demonym,
    };
  });

  return mapped.sort((a, b) => {
    const aPri = PRIORITY_CODES.indexOf(a.alpha2);
    const bPri = PRIORITY_CODES.indexOf(b.alpha2);
    if (aPri !== -1 && bPri !== -1) return aPri - bPri;
    if (aPri !== -1) return -1;
    if (bPri !== -1) return 1;
    return a.demonym.localeCompare(b.demonym);
  });
});

function getCountryValue(c: CountryItem): string {
  return c.demonym;
}

function getCountryLabel(c: CountryItem): string {
  return `${c.emoji} ${c.demonym} (${c.name})`;
}

function getCountrySubtitle(c: CountryItem): string | undefined {
  return `${c.name} — ${c.localDemonym !== c.name ? c.localDemonym : ""}`;
}

function getCountryBadge(c: CountryItem): string | undefined {
  return c.alpha2;
}

function handleChange(val: string | null) {
  const next = val || "";
  value = next;
  onchange?.(next);
}
</script>

<SearchableSelect
  bind:value
  items={countryList}
  {placeholder}
  searchPlaceholder={m.ui_copy_search_nationality_9c3a6b9d()}
  emptyText={m.ui_copy_no_matching_nationalities_found_65a71c81()}
  {disabled}
  {size}
  {id}
  getItemValue={getCountryValue}
  getItemLabel={getCountryLabel}
  getItemSubtitle={getCountrySubtitle}
  getItemBadge={getCountryBadge}
  onchange={handleChange}
  class={className}
/>
