<script lang="ts">
import { onMount } from "svelte";
import QRCode from "qrcode";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { toast } from "@orbis/ui";
import { api, type MfaEnrollResponse } from "$lib/api";
import { ShieldCheck, Copy, Check, Download, KeyRound, AlertTriangle, ArrowRight, ArrowLeft } from "@lucide/svelte";

interface Props {
  open: boolean;
  onSuccess: () => void;
  onClose: () => void;
}

let { open = $bindable(false), onSuccess, onClose }: Props = $props();

let step = $state<"qr" | "codes" | "verify">("qr");
let isLoading = $state(false);
let isSubmitting = $state(false);
let enrollData = $state<MfaEnrollResponse | null>(null);
let qrDataUrl = $state<string>("");
let copiedSecret = $state(false);
let copiedCodes = $state(false);
let verifyCode = $state("");
let errorMessage = $state<string | null>(null);

async function startEnrollment() {
  isLoading = true;
  errorMessage = null;
  step = "qr";
  try {
    const res = await api.mfaEnroll({ factor_name: "Authenticator App" });
    enrollData = res;
    if (res.qr_code_uri) {
      qrDataUrl = await QRCode.toDataURL(res.qr_code_uri, {
        width: 200,
        margin: 1.5,
        color: {
          dark: "#000000",
          light: "#ffffff",
        },
      });
    }
  } catch (err: any) {
    errorMessage = err.message || "Failed to initiate MFA enrollment";
    toast.error(errorMessage);
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (open) {
    startEnrollment();
  }
});

async function copyToClipboard(text: string, type: "secret" | "codes") {
  try {
    await navigator.clipboard.writeText(text);
    if (type === "secret") {
      copiedSecret = true;
      setTimeout(() => (copiedSecret = false), 2000);
    } else {
      copiedCodes = true;
      setTimeout(() => (copiedCodes = false), 2000);
    }
    toast.success("Copied to clipboard!");
  } catch {
    toast.error("Failed to copy");
  }
}

function downloadRecoveryCodes() {
  if (!enrollData) return;
  const content = `ORBIS SECURITY RECOVERY CODES\nGenerated on: ${new Date().toISOString()}\n\nKeep these codes in a safe place. Each code can only be used once.\n\n` +
    enrollData.recovery_codes.map((c, i) => `${i + 1}. ${c}`).join("\n");
  
  const blob = new Blob([content], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = `orbis-recovery-codes-${Date.now()}.txt`;
  link.click();
  URL.revokeObjectURL(url);
  toast.success("Recovery codes saved to file");
}

async function handleVerifyAndActivate() {
  if (!enrollData || !verifyCode.trim()) return;
  isSubmitting = true;
  errorMessage = null;
  try {
    const ok = await api.mfaVerifyEnrollment({
      factor_id: enrollData.factor_id,
      code: verifyCode.trim(),
    });
    if (ok) {
      toast.success("Two-Factor Authentication activated successfully!");
      open = false;
      onSuccess();
    } else {
      errorMessage = "Invalid verification code. Please try again.";
      toast.error(errorMessage);
    }
  } catch (err: any) {
    errorMessage = err.message || "Verification failed";
    toast.error(errorMessage);
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open onOpenChange={(val) => !val && onClose()}>
  <DialogContent class="sm:max-w-[500px] p-0 overflow-hidden border border-border/80 shadow-2xl bg-card rounded-2xl">
    <DialogHeader class="px-6 pt-6 pb-4 border-b border-border/50 bg-muted/20">
      <div class="flex items-center gap-3">
        <div class="p-2.5 rounded-xl bg-primary/10 text-primary">
          <ShieldCheck size={22} class="stroke-[2.2]" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            Setup Two-Factor Authentication
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            Protect your account with Google or Microsoft Authenticator
          </DialogDescription>
        </div>
      </div>
      <!-- Steps Indicator -->
      <div class="flex items-center gap-2 mt-4">
        <div class="flex-1 h-1.5 rounded-full transition-all duration-300 {step === 'qr' ? 'bg-primary' : 'bg-primary/40'}"></div>
        <div class="flex-1 h-1.5 rounded-full transition-all duration-300 {step === 'codes' ? 'bg-primary' : (step === 'verify' ? 'bg-primary/40' : 'bg-muted')}"></div>
        <div class="flex-1 h-1.5 rounded-full transition-all duration-300 {step === 'verify' ? 'bg-primary' : 'bg-muted'}"></div>
      </div>
    </DialogHeader>

    <div class="p-6">
      {#if isLoading}
        <div class="py-12 flex flex-col items-center justify-center gap-3 text-muted-foreground">
          <div class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
          <span class="text-xs font-medium">Generating secure keys...</span>
        </div>
      {:else if enrollData}
        <!-- Step 1: QR Code & Secret Key -->
        {#if step === 'qr'}
          <div class="flex flex-col items-center gap-4 text-center">
            <p class="text-xs text-muted-foreground">
              Scan this QR code in your Authenticator app (e.g. Google Authenticator, 1Password, Microsoft Authenticator):
            </p>

            <div class="p-3 bg-white rounded-2xl border border-border shadow-sm">
              {#if qrDataUrl}
                <img src={qrDataUrl} alt="TOTP QR Code" class="w-44 h-44 rounded-lg object-contain" />
              {/if}
            </div>

            <div class="w-full flex flex-col gap-1.5 text-start">
              <label for="manual-key" class="text-[11px] font-semibold text-muted-foreground uppercase tracking-wider">
                Or enter key manually
              </label>
              <div class="flex items-center gap-2">
                <input
                  id="manual-key"
                  type="text"
                  readonly
                  value={enrollData.secret}
                  class="flex-1 px-3 py-2 text-xs font-mono bg-muted/40 border border-border rounded-xl text-foreground focus:outline-none select-all"
                />
                <Button
                  variant="outline"
                  size="sm"
                  class="h-9 px-3 rounded-xl gap-1.5 text-xs font-medium"
                  onclick={() => copyToClipboard(enrollData!.secret, "secret")}
                >
                  {#if copiedSecret}
                    <Check size={14} class="text-emerald-500" />
                    <span>Copied</span>
                  {:else}
                    <Copy size={14} />
                    <span>Copy</span>
                  {/if}
                </Button>
              </div>
            </div>
          </div>

        <!-- Step 2: Emergency Recovery Codes -->
        {:else if step === 'codes'}
          <div class="flex flex-col gap-4 text-start">
            <div class="p-3.5 rounded-xl bg-amber-500/10 border border-amber-500/20 flex items-start gap-3 text-amber-600 dark:text-amber-400">
              <AlertTriangle size={18} class="shrink-0 mt-0.5" />
              <div class="text-xs leading-relaxed">
                <strong class="font-bold">Save your recovery codes!</strong>
                <p class="mt-0.5 text-muted-foreground">If you lose access to your authenticator app, these 8 one-time codes are the only way to recover your account.</p>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2 p-3 bg-muted/30 border border-border rounded-xl font-mono text-xs text-foreground">
              {#each enrollData.recovery_codes as code, index}
                <div class="flex items-center gap-2 py-1 px-2 rounded bg-background/50 border border-border/40">
                  <span class="text-[10px] text-muted-foreground w-4">{index + 1}.</span>
                  <span class="font-bold tracking-wider">{code}</span>
                </div>
              {/each}
            </div>

            <div class="flex items-center gap-2 pt-1">
              <Button
                variant="outline"
                size="sm"
                class="flex-1 h-9 rounded-xl gap-1.5 text-xs font-medium"
                onclick={() => copyToClipboard(enrollData!.recovery_codes.join("\n"), "codes")}
              >
                {#if copiedCodes}
                  <Check size={14} class="text-emerald-500" />
                  <span>Codes Copied</span>
                {:else}
                  <Copy size={14} />
                  <span>Copy All</span>
                {/if}
              </Button>
              <Button
                variant="outline"
                size="sm"
                class="flex-1 h-9 rounded-xl gap-1.5 text-xs font-medium"
                onclick={downloadRecoveryCodes}
              >
                <Download size={14} />
                <span>Download .txt</span>
              </Button>
            </div>
          </div>

        <!-- Step 3: Confirmation Code Verification -->
        {:else if step === 'verify'}
          <div class="flex flex-col gap-4 text-center">
            <div class="p-3 rounded-xl bg-primary/5 border border-primary/10 text-xs text-muted-foreground">
              Enter the 6-digit code currently shown in your Authenticator app to confirm everything is configured properly.
            </div>

            <div class="flex flex-col items-center gap-2 py-2">
              <input
                type="text"
                maxlength={6}
                placeholder="000000"
                bind:value={verifyCode}
                class="w-48 text-center tracking-[0.4em] font-mono text-2xl font-bold py-2.5 px-4 rounded-xl border-2 border-primary/40 focus:border-primary focus:outline-none bg-background text-foreground"
              />
              <span class="text-[11px] text-muted-foreground">6-digit dynamic code</span>
            </div>

            {#if errorMessage}
              <div class="text-xs text-destructive font-medium">
                {errorMessage}
              </div>
            {/if}
          </div>
        {/if}
      {/if}
    </div>

    <!-- Footer Actions -->
    <DialogFooter class="px-6 py-4 border-t border-border/50 bg-muted/20 flex items-center justify-between">
      {#if step === 'qr'}
        <Button variant="ghost" size="sm" class="rounded-xl text-xs" onclick={() => { open = false; onClose(); }}>
          Cancel
        </Button>
        <Button size="sm" class="rounded-xl gap-1.5 text-xs font-bold" onclick={() => (step = 'codes')}>
          <span>Next: Save Codes</span>
          <ArrowRight size={14} />
        </Button>
      {:else if step === 'codes'}
        <Button variant="ghost" size="sm" class="rounded-xl gap-1 text-xs" onclick={() => (step = 'qr')}>
          <ArrowLeft size={14} />
          <span>Back</span>
        </Button>
        <Button size="sm" class="rounded-xl gap-1.5 text-xs font-bold" onclick={() => (step = 'verify')}>
          <span>Next: Verify Code</span>
          <ArrowRight size={14} />
        </Button>
      {:else if step === 'verify'}
        <Button variant="ghost" size="sm" class="rounded-xl gap-1 text-xs" onclick={() => (step = 'codes')}>
          <ArrowLeft size={14} />
          <span>Back</span>
        </Button>
        <Button
          size="sm"
          class="rounded-xl gap-1.5 text-xs font-bold bg-primary text-primary-foreground"
          disabled={verifyCode.length < 6 || isSubmitting}
          onclick={handleVerifyAndActivate}
        >
          {#if isSubmitting}
            <div class="h-3.5 w-3.5 animate-spin rounded-full border-2 border-primary-foreground border-t-transparent"></div>
            <span>Verifying...</span>
          {:else}
            <ShieldCheck size={15} />
            <span>Confirm & Enable</span>
          {/if}
        </Button>
      {/if}
    </DialogFooter>
  </DialogContent>
</Dialog>
