<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { Switch } from "@orbis/ui/switch";
import { toast } from "@orbis/ui";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { appearanceStore } from "$lib/stores/appearance.svelte";
import { themeStore } from "$lib/stores/theme.svelte";

let fontSize = $derived(appearanceStore.fontSize);
let highContrast = $derived(appearanceStore.highContrast);
let motion = $derived(appearanceStore.motion);
let toastPosition = $derived(appearanceStore.toastPosition);

function resetPreferences() {
  appearanceStore.reset();
  themeStore.set("system");
}
</script>

<div class="flex h-full w-full flex-col gap-6 overflow-y-auto p-4 sm:p-6">
  <Card>
    <CardHeader class="flex flex-row items-center justify-between gap-4">
      <div class="flex flex-col gap-1">
        <CardTitle>{m.appearance_visual_title()}</CardTitle>
        <p class="text-xs text-muted-foreground">{m.appearance_description()}</p>
      </div>
      <Button variant="outline" size="sm" onclick={resetPreferences}>
        {m.appearance_reset()}
      </Button>
    </CardHeader>
    <CardContent>
      <SettingsRow
        title={m.appearance_theme_label()}
        labelFor="appearance-theme"
      >
        {#snippet children()}
          <Select
            type="single"
            value={themeStore.current}
            onValueChange={(value) => {
              if (value === "light" || value === "dark" || value === "system") {
                themeStore.set(value);
              }
            }}
          >
            <SelectTrigger id="appearance-theme" class="h-9 w-full">
              {#if themeStore.current === "system"}
                {m.settings_user_detail_theme_system()}
              {:else if themeStore.current === "light"}
                {m.settings_user_detail_theme_light()}
              {:else}
                {m.settings_user_detail_theme_dark()}
              {/if}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="system">{m.settings_user_detail_theme_system()}</SelectItem>
              <SelectItem value="light">{m.settings_user_detail_theme_light()}</SelectItem>
              <SelectItem value="dark">{m.settings_user_detail_theme_dark()}</SelectItem>
            </SelectContent>
          </Select>
        {/snippet}
      </SettingsRow>

      <SettingsRow
        title={m.appearance_font_size_label()}
        description={m.appearance_font_size_description()}
        labelFor="appearance-font-size"
      >
        {#snippet children()}
          <Select
            type="single"
            value={fontSize}
            onValueChange={(value) => {
              if (value === "default" || value === "large" || value === "larger") {
                appearanceStore.setFontSize(value);
              }
            }}
          >
            <SelectTrigger id="appearance-font-size" class="h-9 w-full">
              {#if fontSize === "large"}
                {m.appearance_font_size_large()}
              {:else if fontSize === "larger"}
                {m.appearance_font_size_larger()}
              {:else}
                {m.appearance_font_size_default()}
              {/if}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="default">{m.appearance_font_size_default()}</SelectItem>
              <SelectItem value="large">{m.appearance_font_size_large()}</SelectItem>
              <SelectItem value="larger">{m.appearance_font_size_larger()}</SelectItem>
            </SelectContent>
          </Select>
        {/snippet}
      </SettingsRow>

      <SettingsRow
        title={m.appearance_toast_position_label()}
        description={m.appearance_toast_position_description()}
        labelFor="appearance-toast-position"
        last
      >
        {#snippet children()}
          <div class="flex w-full flex-col items-start gap-2">
            <Select
              type="single"
              value={toastPosition}
              onValueChange={(value) => {
                if (
                  value === "top-left" ||
                  value === "top-center" ||
                  value === "top-right" ||
                  value === "bottom-left" ||
                  value === "bottom-center" ||
                  value === "bottom-right"
                ) {
                  appearanceStore.setToastPosition(value);
                }
              }}
            >
              <SelectTrigger id="appearance-toast-position" class="h-9 w-full">
                {#if toastPosition.startsWith("top-")}
                  {m.appearance_toast_position_top()}
                {:else}
                  {m.appearance_toast_position_bottom()}
                {/if}
                {#if toastPosition.endsWith("left")}
                  {m.appearance_toast_position_left()}
                {:else if toastPosition.endsWith("right")}
                  {m.appearance_toast_position_right()}
                {:else}
                  {m.appearance_toast_position_center()}
                {/if}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="top-left">{m.appearance_toast_position_top()} {m.appearance_toast_position_left()}</SelectItem>
                <SelectItem value="top-center">{m.appearance_toast_position_top()} {m.appearance_toast_position_center()}</SelectItem>
                <SelectItem value="top-right">{m.appearance_toast_position_top()} {m.appearance_toast_position_right()}</SelectItem>
                <SelectItem value="bottom-left">{m.appearance_toast_position_bottom()} {m.appearance_toast_position_left()}</SelectItem>
                <SelectItem value="bottom-center">{m.appearance_toast_position_bottom()} {m.appearance_toast_position_center()}</SelectItem>
                <SelectItem value="bottom-right">{m.appearance_toast_position_bottom()} {m.appearance_toast_position_right()}</SelectItem>
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              onclick={() => toast.info(m.appearance_toast_preview_message())}
            >
              {m.appearance_toast_preview()}
            </Button>
          </div>
        {/snippet}
      </SettingsRow>
    </CardContent>
  </Card>

  <Card>
    <CardHeader>
      <CardTitle>{m.appearance_accessibility_title()}</CardTitle>
    </CardHeader>
    <CardContent>
      <SettingsRow
        title={m.appearance_high_contrast_label()}
        description={m.appearance_high_contrast_description()}
        labelFor="appearance-high-contrast"
      >
        {#snippet children()}
          <Switch
            id="appearance-high-contrast"
            checked={highContrast}
            onCheckedChange={(checked) => appearanceStore.setHighContrast(checked)}
          />
        {/snippet}
      </SettingsRow>

      <SettingsRow
        title={m.appearance_motion_label()}
        description={m.appearance_motion_description()}
        labelFor="appearance-motion"
        last
      >
        {#snippet children()}
          <Select
            type="single"
            value={motion}
            onValueChange={(value) => {
              if (value === "system" || value === "reduce" || value === "full") {
                appearanceStore.setMotion(value);
              }
            }}
          >
            <SelectTrigger id="appearance-motion" class="h-9 w-full">
              {#if motion === "reduce"}
                {m.appearance_motion_reduce()}
              {:else if motion === "full"}
                {m.appearance_motion_full()}
              {:else}
                {m.appearance_motion_system()}
              {/if}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="system">{m.appearance_motion_system()}</SelectItem>
              <SelectItem value="reduce">{m.appearance_motion_reduce()}</SelectItem>
              <SelectItem value="full">{m.appearance_motion_full()}</SelectItem>
            </SelectContent>
          </Select>
        {/snippet}
      </SettingsRow>
    </CardContent>
  </Card>
</div>
