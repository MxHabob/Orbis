<!--
  @component LocaleSettings
  A reusable locale/language selection row using `SearchableSelect` and `SettingsRow`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface LocaleOption {
  code: string;
  name: string;
  nativeName: string;
  flag: string;
  region?: string;
}

let {
  title = m.settings_general_locale(),
  description = m.settings_general_locale_desc(),
  currentLocale = $bindable("ar"),
  disabled = false,
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentLocale?: string;
  disabled?: boolean;
  last?: boolean;
  onchange?: (locale: string) => void;
} = $props();

const supportedLocales: LocaleOption[] = [
  { code: "ar", name: "Arabic", nativeName: "العربية", flag: "🌐", region: "اللغة الرسمية للنظام" },
  { code: "en", name: "English", nativeName: "English", flag: "🌐", region: "International" },
  { code: "ar-YE", name: "Arabic (Yemen)", nativeName: "العربية (اليمن)", flag: "🇾🇪", region: "الجمهورية اليمنية" },
  { code: "ar-SA", name: "Arabic (Saudi Arabia)", nativeName: "العربية (السعودية)", flag: "🇸🇦", region: "المملكة العربية السعودية" },
  { code: "ar-EG", name: "Arabic (Egypt)", nativeName: "العربية (مصر)", flag: "🇪🇬", region: "جمهورية مصر العربية" },
  { code: "ar-AE", name: "Arabic (UAE)", nativeName: "العربية (الإمارات)", flag: "🇦🇪", region: "دولة الإمارات العربية المتحدة" },
  { code: "en-US", name: "English (United States)", nativeName: "English (US)", flag: "🇺🇸", region: "United States" },
  { code: "en-GB", name: "English (United Kingdom)", nativeName: "English (UK)", flag: "🇬🇧", region: "United Kingdom" },
];

function getLocaleValue(loc: LocaleOption): string {
  return loc.code;
}

function getLocaleLabel(loc: LocaleOption): string {
  return `${loc.flag} ${loc.nativeName} (${loc.name})`;
}

function getLocaleSubtitle(loc: LocaleOption): string | undefined {
  return loc.region;
}

function getLocaleBadge(loc: LocaleOption): string | undefined {
  return loc.code;
}

function handleChange(val: string | null) {
  const next = val || "ar";
  currentLocale = next;
  onchange?.(next);
}
</script>

<SettingsRow {title} {description} {last}>
  {#snippet children()}
    <SearchableSelect
      bind:value={currentLocale}
      items={supportedLocales}
      placeholder={m.settings_general_locale()}
          searchPlaceholder={m.shared_ui_search_locales_placeholder()}
          emptyText={m.shared_ui_no_matching_locales()}
      {disabled}
      getItemValue={getLocaleValue}
      getItemLabel={getLocaleLabel}
      getItemSubtitle={getLocaleSubtitle}
      getItemBadge={getLocaleBadge}
      onchange={handleChange}
      class="w-full"
    />
  {/snippet}
</SettingsRow>
