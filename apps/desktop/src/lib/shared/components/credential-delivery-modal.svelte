<!--
  @component CredentialDeliveryModal
  Displays one-time generated temporary credentials for a user account (Identity & Access Management).
  Adheres strictly to the invariant: temporary passwords are never stored in plaintext
  and cannot be retrieved once dismissed.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { toast } from "@orbis/ui";
import {
  Check,
  Clipboard,
  Eye,
  EyeSlash,
  InfoCircle,
  LockClosed,
  ShieldCheck,
  User,
} from "@orbis/icons";
import type { AccountProvisioningResultDto } from "$lib/api/types";

interface Props {
  open: boolean;
  provisioning?: AccountProvisioningResultDto | null;
  userName?: string;
  onclose?: () => void;
}

let {
  open = $bindable(false),
  provisioning = null,
  userName = "",
  onclose,
}: Props = $props();

let showPassword = $state(false);
let copiedPassword = $state(false);
let copiedUsername = $state(false);
let copiedAll = $state(false);

async function copyToClipboard(
  text: string,
  type: "password" | "username" | "all",
) {
  try {
    await navigator.clipboard.writeText(text);
    if (type === "password") {
      copiedPassword = true;
      setTimeout(() => (copiedPassword = false), 2000);
    } else if (type === "username") {
      copiedUsername = true;
      setTimeout(() => (copiedUsername = false), 2000);
    } else if (type === "all") {
      copiedAll = true;
      setTimeout(() => (copiedAll = false), 2000);
    }
    toast.success(m.credentials_copied());
  } catch (err) {
    console.error("Clipboard write error:", err);
    toast.error(m.ui_copy_failed_to_copy_to_clipboard_99a3869e());
  }
}

function handleCopyAll() {
  if (!provisioning) return;
  const text = [
    `Orbis Workspace User Credentials`,
    `User: ${userName || "User"}`,
    `Username: ${provisioning.username}`,
    provisioning.temporary_password
      ? `Temporary Password: ${provisioning.temporary_password}`
      : "",
    `Delivery: ${provisioning.delivery_channel || "manual"}`,
    `Note: Password must be changed upon first login.`,
  ]
    .filter(Boolean)
    .join("\n");

  copyToClipboard(text, "all");
}

function handleClose() {
  open = false;
  onclose?.();
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md p-6 text-start">
    <DialogHeader>
      <div class="flex items-center gap-3">
        <div class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">
          <ShieldCheck size={22} strokeWidth={2} />
        </div>
        <div class="flex flex-col">
          <DialogTitle class="text-base font-bold text-foreground">
            {m.credentials_modal_title()}
          </DialogTitle>
          <span class="text-xs text-muted-foreground">
            {userName ? `${userName}` : (m.credentials_modal_desc())}
          </span>
        </div>
      </div>
    </DialogHeader>

    {#if provisioning}
      <div class="flex flex-col gap-4 py-3">
        <!-- Username Row -->
        <div class="flex flex-col gap-1.5 rounded-lg border border-border/70 bg-card p-3 shadow-xs">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
              <User size={13} />
              {m.credentials_username()}
            </span>
            <button
              type="button"
              class="flex items-center gap-1 text-[11px] text-primary hover:underline cursor-pointer"
              onclick={() => copyToClipboard(provisioning?.username || "", "username")}
            >
              {#if copiedUsername}
                <Check size={12} class="text-emerald-500" />
                <span class="text-emerald-500">{m.common_mfa_enroll_copied()}</span>
              {:else}
                <Clipboard size={12} />
                <span>{m.common_mfa_enroll_copy()}</span>
              {/if}
            </button>
          </div>
          <div class="font-mono text-sm font-semibold tracking-wide text-foreground select-all">
            {provisioning.username}
          </div>
        </div>

        <!-- Temporary Password Row (if present) -->
        {#if provisioning.temporary_password}
          <div class="flex flex-col gap-1.5 rounded-lg border border-amber-500/30 bg-amber-500/5 dark:bg-amber-950/10 p-3 shadow-xs">
            <div class="flex items-center justify-between">
              <span class="text-xs font-semibold text-amber-700 dark:text-amber-400 flex items-center gap-1.5">
                <LockClosed size={13} />
                {m.credentials_password()}
              </span>
              <div class="flex items-center gap-2">
                <button
                  type="button"
                  class="flex items-center text-muted-foreground hover:text-foreground cursor-pointer p-0.5"
                  onclick={() => (showPassword = !showPassword)}
                  aria-label={showPassword ? m.shared_ui_hide_password() : m.shared_ui_show_password()}
                >
                  {#if showPassword}
                    <EyeSlash size={13} />
                  {:else}
                    <Eye size={13} />
                  {/if}
                </button>
                <button
                  type="button"
                  class="flex items-center gap-1 text-[11px] text-amber-700 dark:text-amber-400 hover:underline cursor-pointer"
                  onclick={() => copyToClipboard(provisioning?.temporary_password || "", "password")}
                >
                  {#if copiedPassword}
                    <Check size={12} class="text-emerald-500" />
                    <span class="text-emerald-500">{m.common_mfa_enroll_copied()}</span>
                  {:else}
                    <Clipboard size={12} />
                    <span>{m.common_mfa_enroll_copy()}</span>
                  {/if}
                </button>
              </div>
            </div>
            <div class="font-mono text-base font-bold tracking-wider text-amber-900 dark:text-amber-200 select-all">
              {showPassword ? provisioning.temporary_password : "••••••••••••••••"}
            </div>
          </div>
        {/if}

        <!-- Security Warning Box -->
        <div class="flex items-start gap-2.5 rounded-lg bg-muted/60 p-3 border border-border/50 text-xs text-muted-foreground">
          <InfoCircle size={15} class="shrink-0 text-amber-500 mt-0.5" />
          <div class="flex flex-col gap-0.5">
            <span class="font-semibold text-foreground">
              {m.credentials_must_change()}
            </span>
            <span class="text-[11px] leading-relaxed">
              {m.credentials_close_warning()}
            </span>
          </div>
        </div>

        <!-- Delivery Channel Badge -->
        <div class="flex items-center justify-between text-xs text-muted-foreground pt-1 px-1">
          <span>{m.employees_delivery_channel()}:</span>
          <Badge variant="outline" class="font-mono text-[11px] uppercase">
            {provisioning.delivery_channel || "manual"}
          </Badge>
        </div>
      </div>
    {/if}

    <DialogFooter class="flex items-center justify-between w-full pt-3 border-t border-border/60">
      <Button
        type="button"
        variant="outline"
        size="sm"
        class="text-xs cursor-pointer gap-1.5"
        onclick={handleCopyAll}
      >
        {#if copiedAll}
          <Check size={13} class="text-emerald-500" />
          <span class="text-emerald-500">{m.common_mfa_enroll_copied()}</span>
        {:else}
          <Clipboard size={13} />
          <span>{m.credentials_copy_all()}</span>
        {/if}
      </Button>

      <Button
        type="button"
        variant="default"
        size="sm"
        class="text-xs px-4 cursor-pointer"
        onclick={handleClose}
      >
        {m.common_done()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
