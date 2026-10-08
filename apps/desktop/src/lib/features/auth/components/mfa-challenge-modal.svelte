<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { toast } from "@orbis/ui";
import { api, type LoginResponse } from "$lib/api";
import { Check, UserBadgeCheck } from "@orbis/icons";

interface Props {
  open: boolean;
  userId: string;
  onSuccess: (res: LoginResponse) => void;
  onClose: () => void;
}

let { open = $bindable(false), userId, onSuccess, onClose }: Props = $props();

let code = $state("");
let rememberDevice = $state(true);
let isSubmitting = $state(false);
let isRecoveryMode = $state(false);
let errorMessage = $state<string | null>(null);

async function handleSubmitChallenge() {
  if (!code.trim() || !userId) return;
  isSubmitting = true;
  errorMessage = null;
  try {
    const res = await api.mfaChallenge({
      user_id: userId,
      code: code.trim(),
      remember_device: rememberDevice,
      device_id:
        "desktop-client-" +
        navigator.userAgent.replace(/[^a-zA-Z0-9]/g, "").slice(0, 16),
    });
    toast.success(m.common_mfa_challenge_success());
    open = false;
    onSuccess(res);
  } catch (err: any) {
    errorMessage = err.message || m.common_mfa_challenge_invalid_code();
    toast.error(errorMessage);
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open onOpenChange={(val) => !val && onClose()}>
  <DialogContent class="max-w-md text-start font-sans">
    <DialogHeader>
      <DialogTitle class="text-sm font-bold flex items-center gap-2">
        <UserBadgeCheck class="w-4.5 h-4.5 text-primary" />
        <span>{m.common_mfa_challenge_dialog_title()}</span>
      </DialogTitle>
      <DialogDescription class="text-xs text-muted-foreground">
        {#if isRecoveryMode}
          {m.common_mfa_challenge_recovery_desc()}
        {:else}
          {m.common_mfa_challenge_totp_desc()}
        {/if}
      </DialogDescription>
    </DialogHeader>

    <div class="py-2 flex flex-col gap-4">
      <div class="flex flex-col items-center gap-2 py-2">
        <Input
          type="text"
          maxlength={isRecoveryMode ? 16 : 6}
          placeholder={isRecoveryMode ? "XXXXXXXX" : "000000"}
          bind:value={code}
          class="w-full text-center tracking-[0.35em] font-mono text-xl font-bold h-11"
        />
        <span class="text-[11px] text-muted-foreground">
          {isRecoveryMode ? m.common_mfa_challenge_recovery_hint() : m.common_mfa_enroll_digit_hint()}
        </span>
      </div>

      {#if errorMessage}
        <div class="p-2.5 rounded-lg bg-destructive/10 border border-destructive/20 text-xs text-destructive text-center font-medium">
          {errorMessage}
        </div>
      {/if}

      <!-- Remember this device checkbox -->
      <div class="flex items-center gap-2 text-xs text-foreground select-none p-2.5 rounded-lg bg-muted/40 border border-border">
        <Checkbox id="remember_device" bind:checked={rememberDevice} />
        <Label for="remember_device" class="text-xs text-foreground cursor-pointer font-normal">
          {m.common_mfa_challenge_remember_device()}
        </Label>
      </div>

      <!-- Toggle Recovery code mode -->
      <div class="text-center pt-1">
        <button
          type="button"
          class="text-xs text-primary hover:underline font-medium cursor-pointer"
          onclick={() => {
            isRecoveryMode = !isRecoveryMode;
            code = "";
            errorMessage = null;
          }}
        >
          {isRecoveryMode ? m.common_mfa_challenge_use_totp() : m.common_mfa_challenge_use_recovery()}
        </button>
      </div>
    </div>

    <DialogFooter class="border-t border-border pt-3 mt-2 flex items-center justify-between">
      <Button variant="ghost" size="sm" class="text-xs" onclick={() => { open = false; onClose(); }}>
        {m.common_titlebar_close()}
      </Button>
      <Button
        size="sm"
        class="text-xs font-semibold gap-1.5"
        disabled={code.length < 6 || isSubmitting}
        onclick={handleSubmitChallenge}
      >
        {#if isSubmitting}
          <LoadingIndicator size={14} class="h-3.5 w-3.5 text-primary" />
          <span>...</span>
        {:else}
          <Check class="w-3.5 h-3.5" />
          <span>{m.common_mfa_challenge_submit_btn()}</span>
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
