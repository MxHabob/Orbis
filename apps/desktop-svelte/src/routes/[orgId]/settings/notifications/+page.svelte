<script lang="ts">
import { onMount } from "svelte";
import { Switch } from "@orbis/ui-svelte/switch";
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { Bell, Mail } from "lucide-svelte";

let isSaving = $state(false);

// Preferences state
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

onMount(() => {
  const saved = localStorage.getItem("orbis_notification_preferences");
  if (saved) {
    try {
      prefs = JSON.parse(saved);
    } catch (e) {
      console.error("Failed to parse notifications preferences:", e);
    }
  }
});

async function handleSave(e: Event) {
  e.preventDefault();
  isSaving = true;

  // Simulate delay for premium feel
  await new Promise((resolve) => setTimeout(resolve, 600));

  localStorage.setItem("orbis_notification_preferences", JSON.stringify(prefs));
  toast.success(m.settings_general_save_success());
  isSaving = false;
}
</script>

<svelte:head>
  <title>{getLocale() === "ar" ? "الإشعارات" : "Notifications"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
    <form onsubmit={handleSave} class="flex flex-col gap-10">
      <!-- Header Actions -->
      <div class="flex items-center justify-end gap-3 pb-5 border-b border-border/40 w-full select-none font-sans">
        <Button
          type="submit"
          variant="lime"
          class="px-6 font-semibold h-9 text-xs"
          disabled={isSaving}
        >
          {isSaving ? m.settings_general_saving() : m.settings_general_save()}
        </Button>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
        <!-- In-App Notifications -->
        <div class="flex flex-col gap-6 text-start">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <Bell size={16} class="text-primary" />
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
                <span class="text-xs font-semibold text-foreground">{m.settings_notifications_attendance()}</span>
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
                <span class="text-xs font-semibold text-foreground">{m.settings_notifications_behavior()}</span>
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
            <Mail size={16} class="text-primary" />
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
                <span class="text-xs font-semibold text-foreground">{m.settings_notifications_attendance()}</span>
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
                <span class="text-xs font-semibold text-foreground">{m.settings_notifications_behavior()}</span>
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
</div>
