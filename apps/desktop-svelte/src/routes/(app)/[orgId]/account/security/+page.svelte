<script lang="ts">
import { onMount } from "svelte";
import { authStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import type { UserSession } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { AccountPasswordCard, AccountSessionsCard } from "$lib/features/auth";
import { Smartphone, Monitor } from "lucide-svelte";

let isUpdating = $state(false);
let currentPassword = $state("");
let newPassword = $state("");
let confirmPassword = $state("");

async function handleUpdatePassword(e: Event) {
  e.preventDefault();

  if (newPassword !== confirmPassword) {
    toast.error(m.settings_security_password_mismatch());
    return;
  }

  if (!authStore.userId) {
    toast.error("User ID not found");
    return;
  }

  isUpdating = true;
  try {
    await commands.iam.changePassword(authStore.userId, {
      new_password: newPassword,
    });
    toast.success(m.settings_security_password_success());
    currentPassword = "";
    newPassword = "";
    confirmPassword = "";
  } catch (e) {
    const errorMsg = (e as any)?.message || "Unknown error";
    toast.error(errorMsg);
  } finally {
    isUpdating = false;
  }
}

let activeSessions = $state<any[]>([]);

async function loadSessions() {
  if (!authStore.userId) return;
  try {
    const rawSessions = await commands.iam.listSessions(authStore.userId);
    activeSessions = rawSessions.map((s: UserSession) => {
      const isMobile =
        s.device_type === "mobile" ||
        (s.user_agent && /mobile|android|iphone|ipad/i.test(s.user_agent));
      const icon = isMobile ? Smartphone : Monitor;

      let device = "Unknown Device";
      if (s.device_name) {
        device = s.device_name;
        if (s.browser) device += ` (${s.browser})`;
      } else if (s.os) {
        device = s.os;
        if (s.browser) device += ` (${s.browser})`;
      }

      const isCurrentDevice = s.token === authStore.token;
      const location = isCurrentDevice
        ? m.settings_user_session_current_device()
        : m.settings_user_session_active_device();

      return {
        id: s.id,
        device,
        location,
        ip: s.ip_address || "127.0.0.1",
        active: s.is_current === 1,
        icon,
      };
    });
  } catch (err) {
    console.error("Failed to load user sessions:", err);
  }
}

onMount(() => {
  loadSessions();
});
</script>

<svelte:head>
  <title>{m.settings_security_window_title()} | Orbis</title>
</svelte:head>

<div class="flex flex-col gap-6">
  <!-- Change Password Form -->
  <AccountPasswordCard
    bind:currentPassword
    bind:newPassword
    bind:confirmPassword
    {isUpdating}
    onsubmit={handleUpdatePassword}
  />

  <!-- Active Sessions -->
  <AccountSessionsCard
    {activeSessions}
  />
</div>
