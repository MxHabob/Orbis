<!--
  @component DashboardGreeting
  Dedicated, time-aware greeting component for the Dashboard.
  Features localized user names via `l()`, random avatar hats, interactive moods,
  and conversational chat/copilot style subtitles.
-->
<script lang="ts">
import { onMount, onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import { authStore, orgContext } from "$lib/stores";
import { l } from "$lib/core/i18n";
import type { DashboardCountsDto } from "$lib/api/types";
import {
  Avatar,
  AvatarController,
  ALL_AVATAR_HATS,
  type AvatarHat,
} from "@orbis/avatar-engine";

let {
  showSubtitle = true,
  stats = null,
}: {
  showSubtitle?: boolean;
  stats?: DashboardCountsDto | null;
} = $props();

const avatarController = new AvatarController();

// Random Initial Hat from available hats
let currentHat = $state<AvatarHat>(
  ALL_AVATAR_HATS[Math.floor(Math.random() * ALL_AVATAR_HATS.length)] || "graduation"
);

function randomizeHat() {
  const otherHats = ALL_AVATAR_HATS.filter((h) => h !== currentHat);
  currentHat = otherHats[Math.floor(Math.random() * otherHats.length)] || ALL_AVATAR_HATS[0];
}

function handleAvatarClick() {
  randomizeHat();
  const rand = Math.random();
  if (rand < 0.33) {
    avatarController.happy();
  } else if (rand < 0.66) {
    avatarController.think();
  } else {
    avatarController.blink();
  }
}

// Time-aware greeting calculations
let hourState = $state(new Date().getHours());
let minuteInterval: ReturnType<typeof setInterval> | undefined;

onMount(() => {
  minuteInterval = setInterval(() => {
    hourState = new Date().getHours();
  }, 60000);
});

onDestroy(() => {
  if (minuteInterval) clearInterval(minuteInterval);
  avatarController.destroy();
});

// Localized user name via `l()`
const fullName = $derived(
  l(authStore.user?.full_name_ar, authStore.user?.full_name) || ""
);
const firstName = $derived(fullName ? fullName.trim().split(" ")[0] : "");

// Institution differentiation from orgContext
const instType = $derived(orgContext.institutionType);

// Real operational status: Is the organization newly initialized / in setup phase?
const isNewOrg = $derived(
  stats != null ? (stats.total_students ?? 0) === 0 && (stats.total_classes ?? 0) === 0 : false
);

// Dynamic time-based Greeting & Conversational Subtitle
const greeting = $derived.by(() => {
  if (hourState >= 4 && hourState < 12) return m.common_greeting_morning();
  if (hourState >= 12 && hourState < 17) return m.common_greeting_afternoon();
  return m.common_greeting_evening();
});

const subtitle = $derived.by(() => {
  // 1. Initial setup / onboarding phase (meaningful operational directive)
  if (isNewOrg) {
    if (instType === "university") return m.greeting_university_setup_sub();
    if (instType === "institute") return m.greeting_institute_setup_sub();
    return m.greeting_school_setup_sub();
  }

  // 2. Active morning operations (04:00 - 12:00)
  if (hourState >= 4 && hourState < 12) {
    if (instType === "university") return m.greeting_university_morning_sub();
    if (instType === "institute") return m.greeting_institute_morning_sub();
    return m.greeting_school_morning_sub();
  }

  // 3. Active afternoon operations (12:00 - 17:00)
  if (hourState >= 12 && hourState < 17) {
    if (instType === "university") return m.greeting_university_afternoon_sub();
    if (instType === "institute") return m.greeting_institute_afternoon_sub();
    return m.greeting_school_afternoon_sub();
  }

  // 4. Evening wrap-up & reconciliations (17:00 - 04:00)
  if (instType === "university") return m.greeting_university_evening_sub();
  if (instType === "institute") return m.greeting_institute_evening_sub();
  return m.greeting_school_evening_sub();
});
</script>

<div class="flex flex-col items-center justify-center text-center select-none gap-2">
  <!-- Avatar & Greeting Heading -->
  <div class="flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-2 text-center sm:text-start">
    <button
      type="button"
      onclick={handleAvatarClick}
      class="cursor-pointer focus:outline-none transition-all duration-150 ease-out -translate-y-1 hover:-translate-y-2 active:translate-y-0 shrink-0"
      aria-label="Interactive Avatar"
      title="Click to change hat & mood"
    >
      <Avatar
        controller={avatarController}
        hat={currentHat}
        width="0px"
        height="90px"
      />
    </button>

    <h1 class="text-[34px] sm:text-[38px] font-serif leading-tight text-foreground tracking-tight">
      {greeting}{#if firstName}, <span class="text-muted-foreground font-sans font-medium">{firstName}</span>{/if}
    </h1>
  </div>

  <!-- Modern Chat / Copilot style conversational subtitle -->
  {#if showSubtitle}
    <p class="text-xs sm:text-sm text-muted-foreground max-w-md mx-auto leading-relaxed mt-0.5">
      {subtitle}
    </p>
  {/if}
</div>
