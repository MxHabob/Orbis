<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { fade } from "svelte/transition";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { orgId, userId }: { orgId: string; userId: string } = $props();

let prefTheme = $state("dark");
let prefLanguage = $state("ar");
let notifyEmail = $state(true);
let notifyPush = $state(true);

const initialPrefs = {
  theme: "dark",
  language: "ar",
  notifyEmail: true,
  notifyPush: true,
};

let isSaving = $state(false);
const isDirty = $derived(
  prefTheme !== initialPrefs.theme ||
  prefLanguage !== initialPrefs.language ||
  notifyEmail !== initialPrefs.notifyEmail ||
  notifyPush !== initialPrefs.notifyPush
);

function handleReset() {
  prefTheme = initialPrefs.theme;
  prefLanguage = initialPrefs.language;
  notifyEmail = initialPrefs.notifyEmail;
  notifyPush = initialPrefs.notifyPush;
}

function handleSave() {
  isSaving = true;
  setTimeout(() => {
    isSaving = false;
    toast.success(m.settings_user_detail_settings_saved());
  }, 300);
}
</script>

<div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Theme -->
    <div class="flex flex-col gap-2 text-start">
      <Label
        class="text-xs font-medium text-muted-foreground flex items-center gap-1.5"
      >
        <Grid size={13} />
        {m.settings_user_detail_pref_theme()}
      </Label>
      <Select
        type="single"
        value={prefTheme}
        onValueChange={(v) => (prefTheme = v ?? "")}
      >
        <SelectTrigger class="h-9 w-full">
          {#if prefTheme === "system"}
            {m.settings_user_detail_theme_system()}
          {:else}
            {prefTheme === "light"
              ? m.settings_user_detail_theme_light()
              : m.settings_user_detail_theme_dark()}
          {/if}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="system"
            >{m.settings_user_detail_theme_system()}</SelectItem
          >
          <SelectItem value="light"
            >{m.settings_user_detail_theme_light()}</SelectItem
          >
          <SelectItem value="dark"
            >{m.settings_user_detail_theme_dark()}</SelectItem
          >
        </SelectContent>
      </Select>
    </div>

    <!-- Language -->
    <div class="flex flex-col gap-2 text-start">
      <Label
        class="text-xs font-medium text-muted-foreground flex items-center gap-1.5"
      >
        <GlobeAlt size={13} />
        {m.settings_general_locale()}
      </Label>
      <Select
        type="single"
        value={prefLanguage}
        onValueChange={(v) => (prefLanguage = v ?? "")}
      >
        <SelectTrigger class="h-9 w-full">
          {prefLanguage === "ar"
            ? m.settings_user_detail_lang_ar()
            : m.settings_user_detail_lang_en()}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="ar"
            >{m.settings_user_detail_lang_ar()}</SelectItem
          >
          <SelectItem value="en"
            >{m.settings_user_detail_lang_en()}</SelectItem
          >
        </SelectContent>
      </Select>
    </div>
  </div>

  <!-- Notifications -->
  <div class="flex flex-col gap-4 border-t border-border/40 pt-4 mt-2">
    <span class="text-xs font-semibold text-muted-foreground">
      {m.settings_user_detail_pref_notifications()}
    </span>
    <div class="flex flex-col gap-3">
      <div class="flex items-center justify-between">
        <Label class="text-xs text-muted-foreground cursor-pointer select-none">
          {m.settings_user_detail_notify_email()}
        </Label>
        <Switch bind:checked={notifyEmail} />
      </div>
      <div class="flex items-center justify-between">
        <Label class="text-xs text-muted-foreground cursor-pointer select-none">
          {m.settings_user_detail_notify_push()}
        </Label>
        <Switch bind:checked={notifyPush} />
      </div>
    </div>
  </div>
</div>

<SettingsBottomBar
  visible={isDirty}
  {isDirty}
  {isSaving}
  onsave={handleSave}
  onreset={handleReset}
/>
