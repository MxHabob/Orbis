<script lang="ts">
import { onDestroy } from "svelte";
import {
  Avatar,
  AvatarController,
  ALL_AVATAR_HATS,
  type AvatarHat,
} from "@orbis/avatar-engine";
import type { AvatarState } from "@orbis/avatar-engine/src/core/avatar-state-machine.js";
import { authStore } from "$lib/stores";
import type { UserDto } from "$lib/api/types";

/**
 * Resolves the appropriate avatar hat based on user role, preferences, or explicit hat name.
 */
export function resolveUserHat(
  roleOrHat?: string | null,
  customHat?: AvatarHat | null
): AvatarHat {
  if (customHat && customHat !== "random") return customHat;
  if (!roleOrHat) return "graduation";

  const lower = String(roleOrHat).toLowerCase().trim();

  // If already a recognized hat name
  if (ALL_AVATAR_HATS.includes(lower as AvatarHat) || lower === "none") {
    return lower as AvatarHat;
  }

  // Role-based hat associations
  if (
    lower.includes("super_admin") ||
    lower.includes("superadmin") ||
    lower.includes("admin") ||
    lower.includes("owner") ||
    lower.includes("principal") ||
    lower.includes("مدير")
  ) {
    return "crown";
  }

  if (
    lower.includes("teacher") ||
    lower.includes("instructor") ||
    lower.includes("educator") ||
    lower.includes("professor") ||
    lower.includes("معلم") ||
    lower.includes("أستاذ")
  ) {
    return "graduation";
  }

  if (
    lower.includes("registrar") ||
    lower.includes("officer") ||
    lower.includes("staff") ||
    lower.includes("coordinator") ||
    lower.includes("مسجل") ||
    lower.includes("شؤون")
  ) {
    return "firefighter";
  }

  if (
    lower.includes("accountant") ||
    lower.includes("finance") ||
    lower.includes("billing") ||
    lower.includes("cashier") ||
    lower.includes("محاسب") ||
    lower.includes("مالي")
  ) {
    return "wizard";
  }

  if (
    lower.includes("auditor") ||
    lower.includes("inspector") ||
    lower.includes("reviewer") ||
    lower.includes("تدقيق")
  ) {
    return "detective";
  }

  if (
    lower.includes("support") ||
    lower.includes("agent") ||
    lower.includes("helpdesk") ||
    lower.includes("دعم")
  ) {
    return "headset";
  }

  if (lower.includes("pilot") || lower.includes("autopilot")) {
    return "pilot";
  }

  if (
    lower.includes("student") ||
    lower.includes("pupil") ||
    lower.includes("طالب")
  ) {
    return "graduation";
  }

  return "graduation";
}

let {
  user = null,
  role = null,
  hat = null,
  state: avatarState = null,
  width = "100%",
  height = "100%",
  controller = undefined,
  class: className = "",
}: {
  user?: UserDto | null;
  role?: string | null;
  hat?: AvatarHat | null;
  state?: AvatarState | null;
  width?: string;
  height?: string;
  controller?: AvatarController;
  class?: string;
} = $props();

const targetUser = $derived(user || authStore.user);

const effectiveHat = $derived.by<AvatarHat>(() => {
  if (hat) return hat;
  const userRole = role || targetUser?.roles || (targetUser as any)?.role || "viewer";
  return resolveUserHat(userRole);
});

// Create single managed controller if state prop is provided
const defaultController = avatarState ? new AvatarController(avatarState) : undefined;
const activeController = $derived(controller ?? defaultController);

$effect(() => {
  if (!controller && defaultController && avatarState && defaultController.state !== avatarState) {
    defaultController.transitionTo(avatarState);
  }
});

onDestroy(() => {
  defaultController?.destroy();
});
</script>

<div class="inline-flex items-center justify-center select-none {className}">
  <Avatar
    hat={effectiveHat}
    {width}
    {height}
    controller={activeController}
  />
</div>
