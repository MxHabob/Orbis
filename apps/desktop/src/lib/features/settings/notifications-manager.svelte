<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Switch } from "@orbis/ui/switch";
import { toast } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import type { OrganizationSettingsDto } from "@orbis/sdk";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { AppNotification } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

let isSaving = $state(false);
let isLoading = $state(true);
let rawSettings = $state<OrganizationSettingsDto | null>(null);
let initialSnapshot = $state("");

// Backend notifications settings
let notifyParentsOnAbsence = $state(true);
let notifyParentsOnGradePublish = $state(true);

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
      notifyParentsOnAbsence = rawSettings.notifications.notify_parents_on_absence ?? true;
      notifyParentsOnGradePublish = rawSettings.notifications.notify_parents_on_grade_publish ?? true;
    }
  } catch (e) {
    console.error("Failed to load backend settings:", e);
  } finally {
    isLoading = false;
  }

  const saved = localStorage.getItem("orbis_notification_preferences");
  if (saved) {
    try {
      prefs = JSON.parse(saved);
    } catch (e) {
      console.error("Failed to parse notifications preferences:", e);
    }
  }
  initialSnapshot = JSON.stringify({ prefs, notifyParentsOnAbsence, notifyParentsOnGradePublish });
});

const currentSnapshot = $derived(
  JSON.stringify({ prefs, notifyParentsOnAbsence, notifyParentsOnGradePublish }),
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
  } catch (e) {
    console.error("Failed to reset notifications preferences:", e);
  }
}

async function handleSave(e?: Event) {
  e?.preventDefault();
  isSaving = true;
  try {
    localStorage.setItem("orbis_notification_preferences", JSON.stringify(prefs));
    if (rawSettings) {
      const updatedSettings: OrganizationSettingsDto = {
        ...rawSettings,
        notifications: {
          ...rawSettings.notifications,
          notify_parents_on_absence: notifyParentsOnAbsence,
          notify_parents_on_grade_publish: notifyParentsOnGradePublish,
          finance_reminder_days: rawSettings.notifications?.finance_reminder_days ?? [7, 3, 1],
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
              <span class="text-[10px] text-muted-foreground">Receive updates when students register</span>
            </div>
            <Switch bind:checked={prefs.inApp.enrollment} disabled={isSaving} />
          </div>

          <!-- Grades -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_grades()}</span>
              <span class="text-[10px] text-muted-foreground">Receive updates on examinations and grading sheets</span>
            </div>
            <Switch bind:checked={prefs.inApp.grades} disabled={isSaving} />
          </div>

          <!-- Attendance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_promo_attendance()}</span>
              <span class="text-[10px] text-muted-foreground">Receive logs on student absence or tardiness</span>
            </div>
            <Switch bind:checked={prefs.inApp.attendance} disabled={isSaving} />
          </div>

          <!-- Finance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_finance()}</span>
              <span class="text-[10px] text-muted-foreground">Receive invoices and payment confirmations</span>
            </div>
            <Switch bind:checked={prefs.inApp.finance} disabled={isSaving} />
          </div>

          <!-- Behavior -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_breadcrumb_behavior()}</span>
              <span class="text-[10px] text-muted-foreground">Receive behavior points or infraction warnings</span>
            </div>
            <Switch bind:checked={prefs.inApp.behavior} disabled={isSaving} />
          </div>

          <!-- System -->
          <div class="flex items-center justify-between py-2">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_system()}</span>
              <span class="text-[10px] text-muted-foreground">Receive platform notifications & logs</span>
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
              <span class="text-[10px] text-muted-foreground">Receive updates when students register</span>
            </div>
            <Switch bind:checked={prefs.email.enrollment} disabled={isSaving} />
          </div>

          <!-- Grades -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_grades()}</span>
              <span class="text-[10px] text-muted-foreground">Receive updates on examinations and grading sheets</span>
            </div>
            <Switch bind:checked={prefs.email.grades} disabled={isSaving} />
          </div>

          <!-- Attendance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_promo_attendance()}</span>
              <span class="text-[10px] text-muted-foreground">Receive logs on student absence or tardiness</span>
            </div>
            <Switch bind:checked={prefs.email.attendance} disabled={isSaving} />
          </div>

          <!-- Finance -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_finance()}</span>
              <span class="text-[10px] text-muted-foreground">Receive invoices and payment confirmations</span>
            </div>
            <Switch bind:checked={prefs.email.finance} disabled={isSaving} />
          </div>

          <!-- Behavior -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.common_breadcrumb_behavior()}</span>
              <span class="text-[10px] text-muted-foreground">Receive behavior points or infraction warnings</span>
            </div>
            <Switch bind:checked={prefs.email.behavior} disabled={isSaving} />
          </div>

          <!-- System -->
          <div class="flex items-center justify-between py-2">
            <div class="flex flex-col text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_notifications_system()}</span>
              <span class="text-[10px] text-muted-foreground">Receive platform notifications & logs</span>
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
