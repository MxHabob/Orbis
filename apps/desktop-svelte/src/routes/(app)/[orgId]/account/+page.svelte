<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { themeStore, authStore } from "$lib/stores";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";

// Component imports
import {
  AccountDetailsCard,
  AccountPreferencesCard,
  AccountAvatarCard,
} from "$lib/features/auth";

// Form states
let isLoading = $state(true);
let isSavingDetails = $state(false);
let isSavingAvatar = $state(false);

let fullName = $state("");
let email = $derived(authStore.user?.email ?? "");
let userId = $derived(authStore.user?.id ?? "");

// Preferences states
let selectedTheme = $state<"light" | "dark" | "system">("system");
let selectedLanguage = $state("ar");

// Avatar state
let avatarUrl = $state<string | null>(null);
let useGravatar = $state(false);
let gravatarUrl = $state("");

// MD5 / SHA-256 Gravatar URL Generator
async function getGravatarUrl(emailAddress: string) {
  const cleanEmail = emailAddress.trim().toLowerCase();
  const msgBuffer = new TextEncoder().encode(cleanEmail);
  const hashBuffer = await crypto.subtle.digest("SHA-256", msgBuffer);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return `https://gravatar.com/avatar/${hashHex}?d=identicon&s=120`;
}

// Reactively load Gravatar when email changes
$effect(() => {
  if (email) {
    getGravatarUrl(email).then((url) => {
      gravatarUrl = url;
      if (useGravatar) {
        avatarUrl = url;
      }
    });
  }
});

// React to Gravatar checkbox toggle
$effect(() => {
  if (useGravatar && gravatarUrl) {
    avatarUrl = gravatarUrl;
  } else if (!useGravatar && avatarUrl && avatarUrl.includes("gravatar.com")) {
    avatarUrl = null;
  }
});

async function handleLanguageChange(newLocale: string) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";
  selectedLanguage = newLocale;

  if (targetLang === getLocale()) return;
  setLocale(targetLang);
}

function handleThemeChange(theme: "light" | "dark" | "system") {
  selectedTheme = theme;
  themeStore.set(theme);
}

function loadUserAccount() {
  isLoading = true;
  if (authStore.user) {
    fullName = authStore.user.full_name || "";
    selectedTheme = themeStore.current;
    selectedLanguage = getLocale();
    const avatar = authStore.user.avatar || null;
    avatarUrl = avatar;
    if (avatar && avatar.includes("gravatar.com")) {
      useGravatar = true;
    }
  }
  isLoading = false;
}

onMount(() => {
  loadUserAccount();
});

async function handleSaveDetails(e: Event) {
  e.preventDefault();
  if (!authStore.userId) return;
  isSavingDetails = true;
  try {
    // 1. Update full name in database
    const updatedUser = await commands.iam.updateUser(authStore.userId, {
      full_name: fullName,
    });

    // 2. Update client authStore state
    authStore.user = updatedUser;
    localStorage.setItem("orbis_user", JSON.stringify(updatedUser));

    // 3. Save Theme Preference
    themeStore.set(selectedTheme);

    toast.success(
      m.account_save_success
        ? m.account_save_success()
        : "تم حفظ تفاصيل الحساب بنجاح",
    );
  } catch (e) {
    console.error("Failed to update user details:", e);
    const msg = (e as any)?.message || String(e);
    toast.error(
      m.account_save_error
        ? m.account_save_error({ error: msg })
        : `خطأ أثناء حفظ تفاصيل الحساب: ${msg}`,
    );
  } finally {
    isSavingDetails = false;
  }
}

async function handleSaveAvatar(e: Event) {
  e.preventDefault();
  if (!authStore.userId) return;
  isSavingAvatar = true;
  try {
    // Update avatar on backend
    const updatedUser = await commands.iam.updateUser(authStore.userId, {
      avatar: avatarUrl,
    });

    // Update client authStore state
    authStore.user = updatedUser;
    localStorage.setItem("orbis_user", JSON.stringify(updatedUser));

    toast.success(
      m.account_save_success
        ? m.account_save_success()
        : "تم حفظ تفاصيل الحساب بنجاح",
    );
  } catch (e) {
    console.error("Failed to update avatar:", e);
    const msg = (e as any)?.message || String(e);
    toast.error(
      m.account_save_error
        ? m.account_save_error({ error: msg })
        : `خطأ أثناء حفظ تفاصيل الحساب: ${msg}`,
    );
  } finally {
    isSavingAvatar = false;
  }
}
</script>

<svelte:head>
  <title>
    {m.account_settings_title ? m.account_settings_title() : 'إعدادات الحساب'} | Orbis
  </title>
</svelte:head>

<div class="flex flex-col gap-6">
    {#if isLoading}
      <!-- Loading state skeleton -->
      <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card">
        <div class="bg-muted/20 px-6 py-4 border-b border-border">
          <div class="h-4 w-28 bg-muted rounded animate-pulse"></div>
        </div>
        <div class="p-6 flex flex-col gap-4">
          <div class="h-9 w-full bg-muted rounded animate-pulse"></div>
          <div class="h-9 w-full bg-muted rounded animate-pulse"></div>
        </div>
      </div>
    {:else}
      <form onsubmit={handleSaveDetails} class="flex flex-col gap-6">
        <AccountDetailsCard
          bind:fullName
          {userId}
          {isSavingDetails}
        />

        <AccountPreferencesCard
          bind:selectedTheme
          bind:selectedLanguage
          {isSavingDetails}
          onThemeChange={handleThemeChange}
          onLanguageChange={handleLanguageChange}
        />

        <!-- Action Button for Details and Preferences -->
        <div class="flex justify-end gap-3 pb-2">
          <Button
            type="submit"
            class="px-6 font-semibold h-9 text-xs"
            disabled={isSavingDetails}
          >
            {isSavingDetails ? m.settings_general_saving() : (m.account_save_changes_btn ? m.account_save_changes_btn() : 'Save Changes')}
          </Button>
        </div>
      </form>

      <AccountAvatarCard
        bind:avatarUrl
        {fullName}
        bind:useGravatar
        {isSavingAvatar}
        onsave={handleSaveAvatar}
      />
    {/if}
</div>
