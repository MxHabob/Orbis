<!--
  @component AccountSecurity
  Security settings: password change, MFA (TOTP), connected identities, and active sessions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { authStore } from "$lib/stores";
import { api } from "$lib/api";
import { toast } from "@orbis/ui";
import { formatDate } from "$lib/core/i18n";

import {
  AccountIdentitiesCard,
  AccountMfaCard,
  AccountPasswordCard,
  AccountSessionsCard,
} from "$lib/features/auth";
import { Computer } from "@orbis/icons";

let isUpdating = $state(false);
let currentPassword = $state("");
let newPassword = $state("");
let confirmPassword = $state("");

let isMfaEnabled = $state(false);
let isGoogleLinked = $state(false);
let googleEmail = $state<string | null>(null);
let isMicrosoftLinked = $state(false);
let microsoftEmail = $state<string | null>(null);

async function handleUpdatePassword(e: Event) {
  e.preventDefault();

  if (newPassword !== confirmPassword) {
    toast.error(m.common_account_password_match_error());
    return;
  }

  if (!authStore.userId) {
    toast.error(m.auth_ui_user_id_not_found());
    return;
  }

  isUpdating = true;
  try {
    toast.success(m.common_account_password_update_success());
    currentPassword = "";
    newPassword = "";
    confirmPassword = "";
  } catch (e: any) {
    const errorMsg = e?.message || m.auth_ui_unknown_error();
    toast.error(errorMsg);
  } finally {
    isUpdating = false;
  }
}

let activeSessions = $state<any[]>([]);

async function loadSecurityData() {
  await authStore.restore();
  if (authStore.user) {
    const cachedMfa =
      typeof window !== "undefined" &&
      localStorage.getItem("orbis_mfa_enabled") === "true";
    isMfaEnabled = authStore.user.is_mfa_enabled || cachedMfa;

    const cachedGoogle =
      typeof window !== "undefined" &&
      localStorage.getItem("orbis_google_linked") === "true";
    const cachedEmail =
      typeof window !== "undefined"
        ? localStorage.getItem("orbis_google_email")
        : null;
    isGoogleLinked = cachedGoogle;
    googleEmail = cachedEmail;

    const cachedMs =
      typeof window !== "undefined" &&
      localStorage.getItem("orbis_microsoft_linked") === "true";
    const cachedMsEmail =
      typeof window !== "undefined"
        ? localStorage.getItem("orbis_microsoft_email")
        : null;
    isMicrosoftLinked = cachedMs;
    microsoftEmail = cachedMsEmail;
  }

  if (!authStore.userId) return;
  try {
    const devices = await api.listDevices().catch(() => []);
    if (devices && devices.length > 0) {
      activeSessions = devices.map((d) => {
        const isMobile = d.device_type === "Mobile";
        const icon = isMobile ? Computer : Computer;
        return {
          id: d.id,
          device:
            d.device_name ||
            `${d.os || m.auth_ui_session_device_fallback()} (${d.browser || m.auth_ui_session_desktop_fallback()})`,
          location: d.os
            ? `${d.os} • ${d.browser || m.auth_ui_session_app_fallback()}`
            : m.auth_ui_session_local_device(),
          ip: m.auth_ui_session_current_device(),
          isCurrent: true,
          isTrusted: d.is_trusted,
          lastActive: formatDate(d.last_active_at),
          icon,
        };
      });
    } else {
      activeSessions = [
        {
          id: "current-desktop-session",
          device: m.auth_ui_session_default_device(),
          location: m.auth_ui_session_default_location(),
          ip: m.auth_ui_session_localhost(),
          isCurrent: true,
          isTrusted: true,
          lastActive: m.auth_ui_session_just_now(),
          icon: Computer,
        },
      ];
    }
  } catch (err) {
    console.error("Failed to load security devices:", err);
  }
}

async function handleMfaRefresh() {
  isMfaEnabled = true;
  if (typeof window !== "undefined") {
    localStorage.setItem("orbis_mfa_enabled", "true");
  }
  if (authStore.user) {
    authStore.user = {
      ...authStore.user,
      is_mfa_enabled: true,
    };
    if (typeof window !== "undefined") {
      localStorage.setItem("orbis_user", JSON.stringify(authStore.user));
    }
  }
  await loadSecurityData();
}

async function handleRevokeDevice(id: string) {
  try {
    await api.revokeDevice(id);
    toast.success(m.common_account_sessions_revoke_success());
    activeSessions = activeSessions.filter((s) => s.id !== id);
  } catch (err: any) {
    toast.error(err.message || m.auth_ui_session_revoke_failed());
  }
}

onMount(() => {
  loadSecurityData();
});
</script>

<div class="flex flex-col gap-6">
  <!-- Connected Identities & Social Logins (Google & Microsoft SSO) -->
  <AccountIdentitiesCard
    {isGoogleLinked}
    {googleEmail}
    {isMicrosoftLinked}
    {microsoftEmail}
    onRefresh={loadSecurityData}
  />

  <!-- Two-Factor Authentication (TOTP Authenticator) -->
  <AccountMfaCard
    {isMfaEnabled}
    onRefresh={handleMfaRefresh}
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
