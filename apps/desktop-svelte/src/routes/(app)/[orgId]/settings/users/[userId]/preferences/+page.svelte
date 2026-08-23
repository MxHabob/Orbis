<!--
@component UserPreferencesPage
Preferences tab for user detail — theme, language, and notification settings.
Extracted from the original monolith TabsContent "settings".
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import { Layout, Globe } from "@lucide/svelte";

let prefTheme = $state("dark");
let prefLanguage = $state("ar");
let notifyEmail = $state(true);
let notifyPush = $state(true);
</script>

<div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Theme -->
    <div class="flex flex-col gap-2 text-start">
      <Label
        class="text-xs font-medium text-muted-foreground flex items-center gap-1.5"
      >
        <Layout size={13} />
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
        <Globe size={13} />
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

  <div class="flex justify-end border-t border-border/40 pt-6 mt-4">
    <Button
      variant="lime"
      size="sm"
      class="font-bold"
      onclick={() =>
        toast.success(m.settings_user_detail_settings_saved())}
    >
      {m.settings_action_save_changes()}
    </Button>
  </div>
</div>
