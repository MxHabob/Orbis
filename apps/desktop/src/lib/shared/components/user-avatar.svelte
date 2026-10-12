<!--
  @component UserAvatar
  Canonical user avatar component using @orbis/ui/avatar primitives.
  Supports image loading, initials extraction, and deterministic color assignment.
-->
<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui/avatar";
import { authStore } from "$lib/stores";
import type { UserDto } from "$lib/api/types";
import {
  resolveDocumentUrl,
  resolveEntityDocumentUrl,
} from "$lib/features/documents/queries";

interface Props {
  user?: UserDto | { name?: string; full_name?: string; email?: string; avatar_url?: string } | null;
  name?: string | null;
  avatarUrl?: string | null;
  role?: string | null;
  size?: "sm" | "default" | "lg";
  class?: string;
}

let {
  user = null,
  name = null,
  avatarUrl = null,
  role = null,
  size = "default",
  class: className = "",
}: Props = $props();

const targetUser = $derived(user || (!name ? authStore.user : null));

const displayName = $derived.by(() => {
  if (name) return name;
  if (!targetUser) return "User";
  return (targetUser as any).full_name || (targetUser as any).name || (targetUser as any).email || "User";
});

const src = $derived(avatarUrl || (targetUser as any)?.avatar_url || undefined);
let resolvedSrc = $state<string | undefined>();

$effect(() => {
  const source = src;
  const userId = (targetUser as any)?.id as string | undefined;
  if (!source) {
    resolvedSrc = undefined;
    return;
  }

  let isCurrent = true;
  const resolve = userId
    ? resolveEntityDocumentUrl(source, "user", userId)
    : resolveDocumentUrl(source);
  resolve
    .then((url) => {
      if (isCurrent) resolvedSrc = url;
    })
    .catch((error: unknown) => {
      console.error("Failed to load user avatar:", error);
    });

  return () => {
    isCurrent = false;
  };
});

const initials = $derived.by(() => {
  const clean = displayName.trim();
  if (!clean) return "U";
  const parts = clean.split(/\s+/).filter(Boolean);
  if (parts.length === 1) {
    return parts[0].substring(0, 2).toUpperCase();
  }
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
});

// Deterministic pleasing color based on name hash
function getHue(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }
  return Math.abs(hash % 360);
}

const hue = $derived(getHue(displayName));
</script>

<Avatar {size} class="shrink-0 {className}">
  {#if src}
    <AvatarImage src={resolvedSrc} alt={displayName} />
  {/if}
  <AvatarFallback
    class="font-semibold text-white tracking-wider flex items-center justify-center"
    style="background: hsl({hue}, 60%, 45%);"
  >
    {initials}
  </AvatarFallback>
</Avatar>
