<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";

interface Props {
  selectedTheme: "light" | "dark" | "system";
  selectedLanguage: string;
  isSavingDetails?: boolean;
  onThemeChange: (theme: "light" | "dark" | "system") => void;
  onLanguageChange: (locale: string) => void;
}

let {
  selectedTheme = $bindable(),
  selectedLanguage = $bindable(),
  isSavingDetails = false,
  onThemeChange,
  onLanguageChange,
}: Props = $props();

const themes = [
  { value: "light", label: "فاتح (Light)" },
  { value: "dark", label: "داكن (Dark)" },
  { value: "system", label: "تلقائي حسب النظام (System)" },
];
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.common_account_preferences_section()}</CardTitle>
  </CardHeader>
  <CardContent>
    <!-- Theme -->
    <SettingsRow
      title={m.settings_appearance_theme()}
      description={m.common_account_preferences_theme_desc()}
    >
      {#snippet children()}
        <Select
          type="single"
          value={selectedTheme}
          onValueChange={(v: string | undefined) => onThemeChange(v as "light" | "dark" | "system")}
          disabled={isSavingDetails}
        >
          <SelectTrigger class="h-9 w-full">
            {themes.find(t => t.value === selectedTheme)?.label || selectedTheme}
          </SelectTrigger>
          <SelectContent>
            {#each themes as t}
              <SelectItem value={t.value}>{t.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      {/snippet}
    </SettingsRow>

    <!-- Language -->
    <LocaleSettings
      title={m.settings_appearance_language()}
      description={m.common_account_preferences_lang_desc()}
      bind:currentLocale={selectedLanguage}
      last
      onchange={onLanguageChange}
    />
  </CardContent>
</Card>
