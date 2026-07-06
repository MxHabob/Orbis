<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui-svelte/card";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";
import * as m from "$lib/paraglide/messages";

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
  ...restProps
}: Props & Record<string, any> = $props();

const themes = [
  { value: "light", label: m.settings_appearance_theme_light() },
  { value: "dark", label: m.settings_appearance_theme_dark() },
  { value: "system", label: m.settings_appearance_theme_system() },
];
</script>

<Card {...restProps}>
  <!-- Card Header -->
  <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
    <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
      {m.account_preferences_section ? m.account_preferences_section() : 'PREFERENCES'}
    </CardTitle>
  </CardHeader>

  <!-- Card Body -->
  <CardContent>
    <div class="flex flex-col">
      <!-- Theme -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label class="text-xs font-semibold text-foreground">
            {m.settings_appearance_theme ? m.settings_appearance_theme() : 'Theme'}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.account_preferences_theme_desc ? m.account_preferences_theme_desc() : "Select your theme preference. It can be synced to your system's theme, always light mode, or always dark mode."}
          </span>
        </div>
        <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
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
        </div>
      </div>

      <!-- Language -->
      <LocaleSettings
        title={m.settings_appearance_language ? m.settings_appearance_language() : 'Language'}
        description={m.account_preferences_lang_desc ? m.account_preferences_lang_desc() : 'Select your preferred application language.'}
        bind:currentLocale={selectedLanguage}
        onchange={onLanguageChange}
      />
    </div>
  </CardContent>
</Card>
