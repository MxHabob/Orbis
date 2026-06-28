<script lang="ts">
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { Lock, Monitor, Smartphone, Key } from "lucide-svelte";

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
    await commands.users.changeUserPassword(authStore.userId, {
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

// Mock active sessions
const activeSessions = [
  {
    id: "1",
    device: "Windows PC (Tauri Desktop App)",
    location: "Sana'a, Yemen (Current Device)",
    ip: "109.74.32.18",
    active: true,
    icon: Monitor,
  },
  {
    id: "2",
    device: "iPhone 15 Pro (Safari Mobile)",
    location: "Sana'a, Yemen",
    ip: "109.74.32.19",
    active: false,
    icon: Smartphone,
  },
];
</script>

<svelte:head>
  <title>{m.settings_security_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background scrollbar-hide">
  <div class="max-w-3xl mx-auto p-8 flex flex-col gap-10">
    <!-- Header -->
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground">{m.settings_security_title()}</h1>
      <p class="text-xs text-muted-foreground">{m.settings_security_desc()}</p>
    </div>

    <!-- Change Password Form -->
    <div class="flex flex-col gap-6 text-start">
      <div class="border-b border-border/60 pb-3 flex items-center gap-2">
        <Key size={16} class="text-primary" />
        <h2 class="text-sm font-semibold text-foreground">{m.settings_security_change_password()}</h2>
      </div>

      <form onsubmit={handleUpdatePassword} class="flex flex-col gap-4 max-w-lg">
        <!-- Current Password -->
        <div class="flex flex-col gap-2 text-start">
          <Label for="current_pass" class="text-xs font-medium text-muted-foreground">
            {m.settings_security_current_password()}
          </Label>
          <Input
            id="current_pass"
            type="password"
            bind:value={currentPassword}
            placeholder="••••••••"
            required
            disabled={isUpdating}
          />
        </div>

        <!-- New Password -->
        <div class="flex flex-col gap-2 text-start">
          <Label for="new_pass" class="text-xs font-medium text-muted-foreground">
            {m.settings_security_new_password()}
          </Label>
          <Input
            id="new_pass"
            type="password"
            bind:value={newPassword}
            placeholder="••••••••"
            required
            minlength={6}
            disabled={isUpdating}
          />
        </div>

        <!-- Confirm Password -->
        <div class="flex flex-col gap-2 text-start">
          <Label for="confirm_pass" class="text-xs font-medium text-muted-foreground">
            {m.settings_security_confirm_password()}
          </Label>
          <Input
            id="confirm_pass"
            type="password"
            bind:value={confirmPassword}
            placeholder="••••••••"
            required
            disabled={isUpdating}
          />
        </div>

        <div class="flex justify-start mt-2">
          <Button
            type="submit"
            variant="lime"
            class="px-6 font-semibold text-xs h-9"
            disabled={isUpdating}
          >
            {isUpdating ? m.settings_security_updating() : m.settings_security_update_password()}
          </Button>
        </div>
      </form>
    </div>

    <!-- Active Sessions -->
    <div class="flex flex-col gap-6 text-start">
      <div class="border-b border-border/60 pb-3">
        <h2 class="text-sm font-semibold text-foreground">{m.settings_security_sessions()}</h2>
        <p class="text-[10px] text-muted-foreground mt-0.5">{m.settings_security_sessions_desc()}</p>
      </div>

      <div class="flex flex-col gap-4">
        {#each activeSessions as session (session.id)}
          <div class="flex items-center justify-between py-3 border-b border-border/40 last:border-b-0">
            <div class="flex items-center gap-3">
              <div class="p-2.5 rounded-lg bg-muted text-foreground">
                <session.icon size={18} />
              </div>
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-bold text-foreground">{session.device}</span>
                <span class="text-[10px] text-muted-foreground">{session.location} • {session.ip}</span>
              </div>
            </div>
            
            {#if session.active}
              <Badge variant="lime" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full">
                Active Now
              </Badge>
            {:else}
              <Badge variant="secondary" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full">
                Logged out
              </Badge>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
