<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import type { IntegrationDto, OrganizationSettingsDto } from "@orbis/sdk";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { AppNotification } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

let isSaving = $state(false);
let isLoading = $state(true);
let isLoadingIntegrations = $state(true);
let rawSettings = $state<OrganizationSettingsDto | null>(null);
let integrations = $state<IntegrationDto[]>([]);
let defaultIntegrationIds = $state<Record<string, string>>({});
let initialSnapshot = $state("");

// Backend notifications settings
let notifyParentsOnAbsence = $state(true);
let notifyParentsOnGradePublish = $state(true);
function channelsForIntegration(integration: IntegrationDto): string[] {
  const settings =
    typeof integration.settings === "object" && integration.settings !== null
      ? (integration.settings as Record<string, unknown>)
      : {};
  const declaredChannels = settings.__notification_channels;
  if (Array.isArray(declaredChannels)) {
    return declaredChannels
      .filter(
        (channel): channel is string =>
          typeof channel === "string" && channel.trim().length > 0,
      )
      .map((channel) => channel.trim().toLowerCase());
  }
  return integration.category.trim().toLowerCase() === "communication"
    ? []
    : [integration.category.trim().toLowerCase()];
}

const notificationChannels = $derived(
  [
    ...new Set([
      ...integrations.flatMap(channelsForIntegration),
      ...Object.keys(defaultIntegrationIds),
    ]),
  ].sort(),
);

function integrationsForChannel(channel: string): IntegrationDto[] {
  return integrations.filter(
    (integration) =>
      integration.is_enabled &&
      (channelsForIntegration(integration).includes(channel) ||
        defaultIntegrationIds[channel] === integration.id),
  );
}

function channelLabel(channel: string): string {
  if (channel === "email") return m.settings_ui_delivery_email_name();
  if (channel === "sms") return m.settings_ui_delivery_sms_name();
  if (channel === "whatsapp") return m.settings_ui_delivery_whatsapp_name();
  return channel;
}

function selectedProviderLabel(channel: string): string {
  const selectedId = defaultIntegrationIds[channel];
  return (
    integrations.find((integration) => integration.id === selectedId)
      ?.provider_name ??
    (selectedId || m.settings_ui_notification_no_default())
  );
}

function setDefaultIntegration(channel: string, integrationId?: string) {
  const next = { ...defaultIntegrationIds };
  if (!integrationId || integrationId === "none") {
    delete next[channel];
  } else {
    next[channel] = integrationId;
  }
  defaultIntegrationIds = next;
}

// Preferences state (hybrid with local channel preferences)
let prefs = $state({
  inApp: {
    enrollment: true,
    grades: true,
    attendance: true,
    finance: true,
    behavior: true,
    system: true,
  },
  email: {
    enrollment: false,
    grades: false,
    attendance: false,
    finance: true,
    behavior: false,
    system: true,
  },
});

onMount(async () => {
  try {
    isLoading = true;
    rawSettings = await api.getSettings();
    if (rawSettings?.notifications) {
      notifyParentsOnAbsence =
        rawSettings.notifications.notify_parents_on_absence ?? true;
      notifyParentsOnGradePublish =
        rawSettings.notifications.notify_parents_on_grade_publish ?? true;
      defaultIntegrationIds = {
        ...(rawSettings.notifications.default_integration_ids ?? {}),
      };
      for (const [channel, integrationId] of [
        ["email", rawSettings.notifications.email_integration_id],
        ["sms", rawSettings.notifications.sms_integration_id],
        ["whatsapp", rawSettings.notifications.whatsapp_integration_id],
      ] as const) {
        if (integrationId && !defaultIntegrationIds[channel]) {
          defaultIntegrationIds[channel] = integrationId;
        }
      }
    }
  } catch (e) {
    console.error("Failed to load backend settings:", e);
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }

  try {
    integrations = await api.listIntegrations();
  } catch (e) {
    console.error("Failed to load notification integrations:", e);
    toast.error(extractErrorMessage(e));
  } finally {
    isLoadingIntegrations = false;
  }

  const saved = localStorage.getItem("orbis_notification_preferences");
  if (saved) {
    try {
      prefs = JSON.parse(saved);
    } catch (e) {
      console.error("Failed to parse notifications preferences:", e);
    }
  }
  initialSnapshot = JSON.stringify({
    prefs,
    notifyParentsOnAbsence,
    notifyParentsOnGradePublish,
    defaultIntegrationIds,
  });
});

const currentSnapshot = $derived(
  JSON.stringify({
    prefs,
    notifyParentsOnAbsence,
    notifyParentsOnGradePublish,
    defaultIntegrationIds,
  }),
);
const isDirty = $derived(
  Boolean(initialSnapshot && currentSnapshot !== initialSnapshot),
);

function handleReset() {
  if (!initialSnapshot) return;
  try {
    const parsed = JSON.parse(initialSnapshot);
    prefs = parsed.prefs;
    notifyParentsOnAbsence = parsed.notifyParentsOnAbsence;
    notifyParentsOnGradePublish = parsed.notifyParentsOnGradePublish;
    defaultIntegrationIds = parsed.defaultIntegrationIds;
  } catch (e) {
    console.error("Failed to reset notifications preferences:", e);
  }
}

async function handleSave(e?: Event) {
  e?.preventDefault();
  isSaving = true;
  try {
    localStorage.setItem(
      "orbis_notification_preferences",
      JSON.stringify(prefs),
    );
    if (rawSettings) {
      const updatedSettings: OrganizationSettingsDto = {
        ...rawSettings,
        notifications: {
          ...rawSettings.notifications,
          default_integration_ids: defaultIntegrationIds,
          email_integration_id: defaultIntegrationIds.email ?? null,
          sms_integration_id: defaultIntegrationIds.sms ?? null,
          whatsapp_integration_id: defaultIntegrationIds.whatsapp ?? null,
          notify_parents_on_absence: notifyParentsOnAbsence,
          notify_parents_on_grade_publish: notifyParentsOnGradePublish,
          finance_reminder_days: rawSettings.notifications
            ?.finance_reminder_days ?? [7, 3, 1],
        },
      };
      rawSettings = await api.updateSettings(updatedSettings);
    }
    initialSnapshot = currentSnapshot;
    toast.success(m.settings_general_save_success());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 pb-28 flex flex-col gap-6 scrollbar-hide">
  <form onsubmit={handleSave} class="flex flex-col gap-10">
    <section class="flex flex-col gap-3 text-start">
      <div>
        <h2 class="text-sm font-semibold text-foreground">
          {m.settings_ui_notification_default_providers()}
        </h2>
        <p class="mt-1 text-[10px] text-muted-foreground">
          {m.settings_ui_notification_default_providers_description()}
        </p>
      </div>
      {#if isLoadingIntegrations}
        <p class="text-xs text-muted-foreground">{m.common_loading()}</p>
      {:else if notificationChannels.length === 0}
        <p class="text-xs text-muted-foreground">
          {m.settings_ui_delivery_channel_integrations_hint()}
        </p>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          {#each notificationChannels as channel (channel)}
            <label class="flex flex-col gap-2 text-xs font-medium text-foreground">
              {channelLabel(channel)}
              <Select
                type="single"
                value={defaultIntegrationIds[channel] ?? "none"}
                onValueChange={(value) => setDefaultIntegration(channel, value)}
                disabled={isSaving}
              >
                <SelectTrigger class="h-9 w-full text-xs">
                  {selectedProviderLabel(channel)}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">{m.settings_ui_notification_no_default()}</SelectItem>
                  {#each integrationsForChannel(channel) as integration (integration.id)}
                    <SelectItem value={integration.id}>{integration.provider_name}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </label>
          {/each}
        </div>
      {/if}
    </section>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
      <!-- In-App Notifications -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <AppNotification size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">{m.settings_notifications_in_app()}</h2>
        </div>

        <div class="flex flex-col gap-4">
          <!-- Enrollment -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_enrollment()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_enrollment_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.enrollment} disabled={isSaving} />
          </div>

          <!-- Grades -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_grades()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_grades_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.grades} disabled={isSaving} />
          </div>

          <!-- Attendance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_promo_attendance()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_attendance_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.attendance} disabled={isSaving} />
          </div>

          <!-- Finance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_finance()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_finance_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.finance} disabled={isSaving} />
          </div>

          <!-- Behavior -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_breadcrumb_behavior()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_behavior_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.behavior} disabled={isSaving} />
          </div>

          <!-- System -->
          <div class="flex items-center justify-between py-2">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_system()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_system_description()}</span>
            </div>
            <Switch bind:checked={prefs.inApp.system} disabled={isSaving} />
          </div>
        </div>
      </div>

      <!-- Email Notifications -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <AppNotification size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">{m.settings_notifications_email()}</h2>
        </div>

        <div class="flex flex-col gap-4">
          <!-- Enrollment -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_enrollment()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_enrollment_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.enrollment} disabled={isSaving} />
          </div>

          <!-- Grades -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_grades()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_grades_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.grades} disabled={isSaving} />
          </div>

          <!-- Attendance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_promo_attendance()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_attendance_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.attendance} disabled={isSaving} />
          </div>

          <!-- Finance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_finance()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_finance_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.finance} disabled={isSaving} />
          </div>

          <!-- Behavior -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_breadcrumb_behavior()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_behavior_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.behavior} disabled={isSaving} />
          </div>

          <!-- System -->
          <div class="flex items-center justify-between py-2">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_system()}</span>
              <span class="text-[10px] text-muted-foreground">{m.settings_ui_notification_system_description()}</span>
            </div>
            <Switch bind:checked={prefs.email.system} disabled={isSaving} />
          </div>
        </div>
      </div>
    </div>
  </form>
  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={handleSave}
    onreset={handleReset}
  />
</div>
