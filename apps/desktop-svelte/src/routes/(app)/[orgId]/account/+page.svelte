<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { themeStore } from "$lib/stores";
import { authStore } from "$lib/stores";
import { toast } from "svelte-sonner";
import AvatarUpload from "$lib/components/shared/avatar-upload.svelte";
import LocaleSettings from "$lib/components/shared/locale-settings.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";
import { Checkbox } from "@orbis/ui-svelte/checkbox";

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
    const updatedUser = await commands.users.updateUser(authStore.userId, {
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
    const updatedUser = await commands.users.updateUser(authStore.userId, {
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

const themes = [
  { value: "light", label: m.settings_appearance_theme_light() },
  { value: "dark", label: m.settings_appearance_theme_dark() },
  { value: "system", label: m.settings_appearance_theme_system() },
];
</script>

<svelte:head>
  <title>
    {m.account_settings_title ? m.account_settings_title() : 'إعدادات الحساب'} | Orbis
  </title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="w-full p-6 flex flex-col gap-6">

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
        <!-- Section 1: Account Details -->
        <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card">
          <!-- Card Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.account_details_section ? m.account_details_section() : 'ACCOUNT DETAILS'}
          </div>

          <!-- Card Body -->
          <div class="flex flex-col">
            <!-- Full Name -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="full_name" class="text-xs font-semibold text-foreground flex items-center gap-1">
                  {m.account_details_name ? m.account_details_name() : 'Name'}
                  <span class="text-destructive">*</span>
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">
                  {m.account_details_name_desc ? m.account_details_name_desc() : 'Your full name'}
                </span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="full_name"
                  type="text"
                  bind:value={fullName}
                  required
                  disabled={isSavingDetails}
                  class="h-9 w-full bg-background"
                />
              </div>
            </div>

            <!-- User ID -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label for="user_id" class="text-xs font-semibold text-foreground">
                  {m.account_details_userid ? m.account_details_userid() : 'User ID'}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">
                  {m.account_details_userid_desc ? m.account_details_userid_desc() : 'The unique identifier for your account. It cannot be modified.'}
                </span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Input
                  id="user_id"
                  type="text"
                  value={userId}
                  disabled
                  class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Section 2: Preferences -->
        <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card">
          <!-- Card Header -->
          <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
            {m.account_preferences_section ? m.account_preferences_section() : 'PREFERENCES'}
          </div>

          <!-- Card Body -->
          <div class="flex flex-col">
            <!-- Theme -->
            <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
              <div class="flex flex-col gap-1 text-start">
                <Label class="text-xs font-semibold text-foreground">
                  {m.settings_appearance_theme ? m.settings_appearance_theme() : 'Theme'}
                </Label>
                <span class="text-[11px] text-muted-foreground leading-relaxed">
                  {m.account_preferences_theme_desc ? m.account_preferences_theme_desc() : "Select your theme preference. It can be synced to your system's theme, always light mode, or always dark mode."}
                </span>
              </div>
              <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                <Select
                  type="single"
                  value={selectedTheme}
                  onValueChange={(v: string | undefined) => handleThemeChange(v as "light" | "dark" | "system")}
                  disabled={isSavingDetails}
                >
                  <SelectTrigger class="h-9 w-full">
                    {themes.find(t => t.value === selectedTheme)?.label || selectedTheme}
                  </SelectTrigger>
                  <SelectContent>
                    {#each themes as t}
                      <SelectItem value={t.value}>{t.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <!-- Language (LocaleSettings) -->
            <LocaleSettings
              title={m.settings_appearance_language ? m.settings_appearance_language() : 'Language'}
              description={m.account_preferences_lang_desc ? m.account_preferences_lang_desc() : 'Select your preferred application language.'}
              bind:currentLocale={selectedLanguage}
              onchange={handleLanguageChange}
            />
          </div>
        </div>

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

      <!-- Section 3: Avatar (AvatarUpload) -->
      <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card mt-2">
        <!-- Card Header -->
        <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
          {m.account_avatar_section ? m.account_avatar_section() : 'AVATAR'}
        </div>

        <!-- Card Body -->
        <div class="px-6 py-5 flex flex-col md:flex-row md:items-center justify-between border-b border-border last:border-b-0 gap-6 text-start select-none">
          <div class="flex-1 flex items-center justify-start gap-4">
            <AvatarUpload
              bind:avatarUrl={avatarUrl}
              size={64}
              fallbackText={fullName}
            />
            {#if useGravatar}
              <span class="text-xs font-semibold text-muted-foreground">{m.account_avatar_use_gravatar ? m.account_avatar_use_gravatar() : 'Use Gravatar'} (Active)</span>
            {/if}
          </div>

          <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start md:justify-start gap-3">
            <Checkbox id="use_gravatar" bind:checked={useGravatar} />
            <div class="flex flex-col text-start leading-relaxed">
              <Label for="use_gravatar" class="text-xs font-semibold text-foreground cursor-pointer">
                {m.account_avatar_use_gravatar ? m.account_avatar_use_gravatar() : 'Use Gravatar'}
              </Label>
              <span class="text-[10px] text-muted-foreground leading-normal">
                {m.account_avatar_gravatar_desc ? m.account_avatar_gravatar_desc() : 'Manage your Gravatar on gravatar.com.'}
              </span>
            </div>
          </div>
        </div>

        <!-- Card Footer -->
        <div class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start">
          <Button
            type="button"
            class="font-semibold text-xs h-8 px-4"
            onclick={handleSaveAvatar}
            disabled={isSavingAvatar}
          >
            {isSavingAvatar ? m.settings_general_saving() : (m.account_avatar_save_btn ? m.account_avatar_save_btn() : 'Save Avatar')}
          </Button>
        </div>
      </div>
    {/if}
  </div>
</div>
