<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  api,
  extractErrorMessage,
  type NotificationPreferenceResponseDto,
} from "$lib/api";
import { authStore, themeStore } from "$lib/stores";
import { getLocale, setLocale } from "$lib/paraglide/runtime";
import { Card } from "@orbis/ui/card";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import EmptyState from "$lib/shared/components/empty-state.svelte";
import { GlobeAlt } from "@orbis/icons";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const queryClient = useQueryClient();
const isOwnAccount = $derived(Boolean(userId) && authStore.userId === userId);

const prefsQuery = createQuery<NotificationPreferenceResponseDto[]>(() => ({
  queryKey: ["notifications", "preferences", orgId, userId],
  queryFn: () => api.getNotificationPreferences(),
  enabled: isOwnAccount,
  staleTime: 60 * 1000,
}));

const initialTheme = $derived(themeStore.current);
const initialLocale = $derived<"ar" | "en">(getLocale() === "en" ? "en" : "ar");

let prefTheme = $state<"light" | "dark" | "system">(themeStore.current);
let prefLanguage = $state<"ar" | "en">(getLocale() === "en" ? "en" : "ar");
let notifyEmail = $state(true);
let notifyPush = $state(true);
let initializedNotifs = $state(false);
let isSaving = $state(false);

const serverNotifyEmail = $derived(
  prefsQuery.data?.some((p) => p.email_enabled) ?? true,
);
const serverNotifyPush = $derived(
  prefsQuery.data?.some((p) => p.push_enabled) ?? true,
);

$effect(() => {
  if (!prefsQuery.data || initializedNotifs) return;
  notifyEmail = serverNotifyEmail;
  notifyPush = serverNotifyPush;
  initializedNotifs = true;
});

const notificationsDirty = $derived(
  Boolean(prefsQuery.data) &&
    (notifyEmail !== serverNotifyEmail || notifyPush !== serverNotifyPush),
);

const isDirty = $derived(
  prefTheme !== initialTheme ||
    prefLanguage !== initialLocale ||
    notificationsDirty,
);

function handleReset() {
  prefTheme = initialTheme;
  prefLanguage = initialLocale;
  notifyEmail = serverNotifyEmail;
  notifyPush = serverNotifyPush;
}

async function handleSave() {
  isSaving = true;
  try {
    if (prefTheme !== initialTheme) {
      themeStore.set(prefTheme);
    }

    if (notificationsDirty && prefsQuery.data) {
      const updated = prefsQuery.data.map((p) => ({
        category: p.category,
        in_app_enabled: p.in_app_enabled,
        email_enabled: notifyEmail,
        push_enabled: notifyPush,
      }));
      await api.updateNotificationPreferences(updated);
      await queryClient.invalidateQueries({
        queryKey: ["notifications", "preferences", orgId, userId],
      });
    }

    toast.success(m.settings_user_detail_settings_saved());

    if (prefLanguage !== initialLocale) {
      setLocale(prefLanguage);
    }
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || m.ui_copy_save_failed_38ef99fb());
  } finally {
    isSaving = false;
  }
}
</script>

{#if !isOwnAccount}
  <EmptyState
    icon={GlobeAlt}
    title={m.settings_user_detail_tab_settings()}
  />
{:else}
  <div class="flex flex-col gap-5 text-start">
    <Card>
      <div class="border-b border-border bg-muted/20 px-4 py-3 sm:px-6">
        <h2 class="text-xs font-semibold text-muted-foreground">
          {m.settings_user_detail_tab_settings()}
        </h2>
      </div>

      <SettingsRow title={m.settings_user_detail_pref_theme()}>
        <Select
          type="single"
          value={prefTheme}
          onValueChange={(v) => {
            if (v === "system" || v === "light" || v === "dark") {
              prefTheme = v;
            }
          }}
        >
          <SelectTrigger class="h-10 w-full">
            {#if prefTheme === "system"}
              {m.settings_user_detail_theme_system()}
            {:else if prefTheme === "light"}
              {m.settings_user_detail_theme_light()}
            {:else}
              {m.settings_user_detail_theme_dark()}
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
      </SettingsRow>

      <SettingsRow title={m.settings_general_locale()} last>
        <Select
          type="single"
          value={prefLanguage}
          onValueChange={(v) => {
            if (v === "ar" || v === "en") {
              prefLanguage = v;
            }
          }}
        >
          <SelectTrigger class="h-10 w-full">
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
      </SettingsRow>
    </Card>

    <Card>
      <div class="border-b border-border bg-muted/20 px-4 py-3 sm:px-6">
        <h2 class="text-xs font-semibold text-muted-foreground">
          {m.settings_user_detail_pref_notifications()}
        </h2>
      </div>

      {#if prefsQuery.isLoading}
        <div class="flex flex-col gap-3 px-4 py-4 sm:px-6">
          <Skeleton class="h-6 w-full rounded-md" />
          <Skeleton class="h-6 w-full rounded-md" />
        </div>
      {:else if prefsQuery.isError}
        <p class="px-4 py-4 text-xs text-destructive sm:px-6">
          {extractErrorMessage(prefsQuery.error)}
        </p>
      {:else}
        <SettingsRow title={m.settings_user_detail_notify_email()}>
          <Switch bind:checked={notifyEmail} />
        </SettingsRow>

        <SettingsRow title={m.settings_user_detail_notify_push()} last>
          <Switch bind:checked={notifyPush} />
        </SettingsRow>
      {/if}
    </Card>
  </div>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={handleSave}
    onreset={handleReset}
  />
{/if}
