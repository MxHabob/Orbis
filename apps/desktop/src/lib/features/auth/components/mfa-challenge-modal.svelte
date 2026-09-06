<script lang="ts">
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import { api, type LoginResponse } from "$lib/api";
import { ShieldCheck, Smartphone, KeyRound, Check } from "@lucide/svelte";

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
      device_id: "desktop-client-" + navigator.userAgent.replace(/[^a-zA-Z0-9]/g, "").slice(0, 16),
    });
    toast.success("Identity verified successfully!");
    open = false;
    onSuccess(res);
  } catch (err: any) {
    errorMessage = err.message || "Invalid authentication code";
    toast.error(errorMessage);
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open onOpenChange={(val) => !val && onClose()}>
  <DialogContent class="sm:max-w-[420px] p-0 overflow-hidden border border-border/80 shadow-2xl bg-card rounded-2xl">
    <DialogHeader class="px-6 pt-6 pb-4 border-b border-border/50 bg-muted/20">
      <div class="flex items-center gap-3">
        <div class="p-2.5 rounded-xl bg-primary/10 text-primary">
          <ShieldCheck size={22} class="stroke-[2.2]" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            Two-Factor Verification
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            {#if isRecoveryMode}
              Enter one of your 8-character recovery codes
            {:else}
              Enter the 6-digit code from your Authenticator app
            {/if}
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <div class="p-6 flex flex-col gap-4">
      <div class="flex flex-col items-center gap-2 py-2">
        <input
          type="text"
          maxlength={isRecoveryMode ? 16 : 6}
          placeholder={isRecoveryMode ? "XXXXXXXX" : "000000"}
          bind:value={code}
          class="w-full text-center tracking-[0.35em] font-mono text-2xl font-bold py-3 px-4 rounded-xl border-2 border-primary/40 focus:border-primary focus:outline-none bg-background text-foreground"
        />
        <span class="text-[11px] text-muted-foreground">
          {isRecoveryMode ? "One-time emergency recovery code" : "6-digit dynamic code"}
        </span>
      </div>

      {#if errorMessage}
        <div class="p-2.5 rounded-xl bg-destructive/10 border border-destructive/20 text-xs text-destructive text-center font-medium">
          {errorMessage}
        </div>
      {/if}

      <!-- Remember this device checkbox -->
      <label class="flex items-center gap-2.5 text-xs text-foreground cursor-pointer select-none p-2 rounded-xl bg-muted/30 border border-border/40 hover:bg-muted/50 transition-colors">
        <input
          type="checkbox"
          bind:checked={rememberDevice}
          class="h-4 w-4 rounded border-border text-primary focus:ring-primary/20 accent-primary"
        />
        <span>Trust this device for 30 days</span>
      </label>

      <!-- Toggle Recovery code mode -->
      <div class="text-center pt-1">
        <button
          type="button"
          class="text-xs text-primary hover:underline font-medium"
          onclick={() => {
            isRecoveryMode = !isRecoveryMode;
            code = "";
            errorMessage = null;
          }}
        >
          {isRecoveryMode ? "Use Authenticator app code instead" : "Lost access? Use a backup recovery code"}
        </button>
      </div>
    </div>

    <DialogFooter class="px-6 py-4 border-t border-border/50 bg-muted/20 flex items-center justify-between">
      <Button variant="ghost" size="sm" class="rounded-xl text-xs" onclick={() => { open = false; onClose(); }}>
        Cancel
      </Button>
      <Button
        size="sm"
        class="rounded-xl gap-1.5 text-xs font-bold bg-primary text-primary-foreground"
        disabled={code.length < 6 || isSubmitting}
        onclick={handleSubmitChallenge}
      >
        {#if isSubmitting}
          <div class="h-3.5 w-3.5 animate-spin rounded-full border-2 border-primary-foreground border-t-transparent"></div>
          <span>Verifying...</span>
        {:else}
          <Check size={14} />
          <span>Verify & Login</span>
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
