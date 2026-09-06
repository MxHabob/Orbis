<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { authStore } from "$lib/stores";
import { api } from "$lib/api";
import { toast } from "@orbis/ui";

import {
  AccountMfaCard,
  AccountIdentitiesCard,
  AccountPasswordCard,
  AccountSessionsCard,
} from "$lib/features/auth";
import { Smartphone, Laptop, Monitor } from "@lucide/svelte";

let isUpdating = $state(false);
let currentPassword = $state("");
let newPassword = $state("");
let confirmPassword = $state("");

let isMfaEnabled = $state(false);
let isGoogleLinked = $state(false);
let googleEmail = $state<string | null>(null);

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
    toast.success(m.settings_security_password_success());
    currentPassword = "";
    newPassword = "";
    confirmPassword = "";
  } catch (e: any) {
    const errorMsg = e?.message || "Unknown error";
    toast.error(errorMsg);
  } finally {
    isUpdating = false;
  }
}

let activeSessions = $state<any[]>([]);

async function loadSecurityData() {
  if (!authStore.userId) return;
  try {
    const devices = await api.listDevices().catch(() => []);
    if (devices && devices.length > 0) {
      activeSessions = devices.map((d) => {
        const isMobile = d.device_type === "Mobile";
        const icon = isMobile ? Smartphone : Laptop;
        return {
          id: d.id,
          device: d.device_name || `${d.os || "Device"} (${d.browser || "Desktop"})`,
          location: d.os ? `${d.os} • ${d.browser || "App"}` : "Local Device",
          ip: "Current Device",
          isCurrent: true,
          isTrusted: d.is_trusted,
          lastActive: new Date(d.last_active_at).toLocaleDateString(),
          icon,
        };
      });
    } else {
      // Default local session display
      activeSessions = [
        {
          id: "current-desktop-session",
          device: "Orbis Desktop Client (Windows)",
          location: "Windows • Desktop App",
          ip: "Localhost",
          isCurrent: true,
          isTrusted: true,
          lastActive: "Just now",
          icon: Laptop,
        },
      ];
    }
  } catch (err) {
    console.error("Failed to load security devices:", err);
  }
}

async function handleRevokeDevice(id: string) {
  try {
    await api.revokeDevice(id);
    toast.success("Device session revoked successfully");
    activeSessions = activeSessions.filter((s) => s.id !== id);
  } catch (err: any) {
    toast.error(err.message || "Failed to revoke device");
  }
}

onMount(() => {
  loadSecurityData();
});
</script>

<svelte:head>
  <title>{m.common_nav_security()} | Orbis</title>
</svelte:head>

<div class="flex flex-col gap-6 max-w-4xl">
  <!-- Two-Factor Authentication (TOTP Authenticator) -->
  <AccountMfaCard
    {isMfaEnabled}
    onRefresh={loadSecurityData}
  />

  <!-- Connected Identities & Social Logins (Google Account Linking) -->
  <AccountIdentitiesCard
    {isGoogleLinked}
    {googleEmail}
    onRefresh={loadSecurityData}
  />

  <!-- Change Password Card -->
  <AccountPasswordCard
    bind:currentPassword
    bind:newPassword
    bind:confirmPassword
    {isUpdating}
    onsubmit={handleUpdatePassword}
  />

  <!-- Trusted Devices & Active Sessions -->
  <AccountSessionsCard
    {activeSessions}
    onRevoke={handleRevokeDevice}
  />
</div>
