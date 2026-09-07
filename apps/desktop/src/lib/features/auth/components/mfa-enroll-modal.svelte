<script lang="ts">
import * as m from "$lib/paraglide/messages";
import QRCode from "qrcode";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import { api, type MfaEnrollResponse } from "$lib/api";
import { ShieldCheck, Copy, Check, Download, AlertTriangle, ArrowRight, ArrowLeft } from "@lucide/svelte";

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
        width: 170,
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
    toast.success(m.common_mfa_enroll_copied());
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
  toast.success(m.common_mfa_enroll_download_txt());
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
      toast.success(m.common_mfa_enroll_success());
      open = false;
      onSuccess();
    } else {
      errorMessage = m.common_mfa_enroll_invalid_code();
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
  <DialogContent class="max-w-md text-start font-sans">
    <DialogHeader>
      <DialogTitle class="text-sm font-bold flex items-center gap-2">
        <ShieldCheck class="w-4.5 h-4.5 text-primary" />
        <span>{m.common_mfa_enroll_dialog_title()}</span>
      </DialogTitle>
      <DialogDescription class="text-xs text-muted-foreground">
        {m.common_mfa_enroll_dialog_desc()}
      </DialogDescription>

      <!-- Steps Progress -->
      <div class="flex items-center gap-1.5 pt-2">
        <div class="flex-1 h-1 rounded-full transition-all {step === 'qr' ? 'bg-primary' : 'bg-primary/40'}"></div>
        <div class="flex-1 h-1 rounded-full transition-all {step === 'codes' ? 'bg-primary' : (step === 'verify' ? 'bg-primary/40' : 'bg-muted')}"></div>
        <div class="flex-1 h-1 rounded-full transition-all {step === 'verify' ? 'bg-primary' : 'bg-muted'}"></div>
      </div>
    </DialogHeader>

    <div class="py-2">
      {#if isLoading}
        <div class="py-12 flex flex-col items-center justify-center gap-3 text-muted-foreground">
          <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
          <span class="text-xs">Generating keys...</span>
        </div>
      {:else if enrollData}
        <!-- Step 1: QR Code & Secret Key -->
        {#if step === 'qr'}
          <div class="flex flex-col items-center gap-3 text-center">
            <p class="text-xs text-muted-foreground text-start w-full">
              {m.common_mfa_enroll_qr_step_info()}
            </p>

            <div class="p-2 bg-white rounded-lg border border-border shadow-xs">
              {#if qrDataUrl}
                <img src={qrDataUrl} alt="TOTP QR Code" class="w-36 h-36 rounded object-contain" />
              {/if}
            </div>

            <div class="w-full flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-[11px] font-semibold text-muted-foreground">
                {m.common_mfa_enroll_manual_key()}
              </Label>
              <div class="flex items-center gap-2">
                <Input
                  type="text"
                  readonly
                  value={enrollData.secret}
                  class="flex-1 font-mono text-xs bg-muted/40 h-9"
                />
                <Button
                  variant="outline"
                  size="sm"
                  class="h-9 px-3 gap-1.5 text-xs font-medium"
                  onclick={() => copyToClipboard(enrollData!.secret, "secret")}
                >
                  {#if copiedSecret}
                    <Check class="w-3.5 h-3.5 text-emerald-500" />
                    <span>{m.common_mfa_enroll_copied()}</span>
                  {:else}
                    <Copy class="w-3.5 h-3.5" />
                    <span>{m.common_mfa_enroll_copy()}</span>
                  {/if}
                </Button>
              </div>
            </div>
          </div>

        <!-- Step 2: Emergency Recovery Codes -->
        {:else if step === 'codes'}
          <div class="flex flex-col gap-3 text-start">
            <div class="p-3 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-start gap-2.5 text-amber-600 dark:text-amber-400">
              <AlertTriangle class="w-4 h-4 shrink-0 mt-0.5" />
              <div class="text-xs leading-relaxed">
                <strong class="font-semibold">{m.common_mfa_enroll_codes_warning_title()}</strong>
                <p class="text-[11px] text-muted-foreground mt-0.5">
                  {m.common_mfa_enroll_codes_warning_desc()}
                </p>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-1.5 p-2.5 bg-muted/30 border border-border rounded-lg font-mono text-xs">
              {#each enrollData.recovery_codes as code, index}
                <div class="flex items-center gap-2 py-1 px-2 rounded bg-background border border-border/50">
                  <span class="text-[10px] text-muted-foreground w-3.5">{index + 1}.</span>
                  <span class="font-semibold text-foreground tracking-wider">{code}</span>
                </div>
              {/each}
            </div>

            <div class="flex items-center gap-2 pt-1">
              <Button
                variant="outline"
                size="sm"
                class="flex-1 h-9 gap-1.5 text-xs font-medium"
                onclick={() => copyToClipboard(enrollData!.recovery_codes.join("\n"), "codes")}
              >
                {#if copiedCodes}
                  <Check class="w-3.5 h-3.5 text-emerald-500" />
                  <span>{m.common_mfa_enroll_copied()}</span>
                {:else}
                  <Copy class="w-3.5 h-3.5" />
                  <span>{m.common_mfa_enroll_copy_all_codes()}</span>
                {/if}
              </Button>
              <Button
                variant="outline"
                size="sm"
                class="flex-1 h-9 gap-1.5 text-xs font-medium"
                onclick={downloadRecoveryCodes}
              >
                <Download class="w-3.5 h-3.5" />
                <span>{m.common_mfa_enroll_download_txt()}</span>
              </Button>
            </div>
          </div>

        <!-- Step 3: Confirmation Code Verification -->
        {:else if step === 'verify'}
          <div class="flex flex-col gap-3 text-center">
            <p class="text-xs text-muted-foreground text-start">
              {m.common_mfa_enroll_verify_info()}
            </p>

            <div class="flex flex-col items-center gap-2 py-2">
              <Input
                type="text"
                maxlength={6}
                placeholder="000000"
                bind:value={verifyCode}
                class="w-48 text-center tracking-[0.4em] font-mono text-xl font-bold h-11"
              />
              <span class="text-[11px] text-muted-foreground">{m.common_mfa_enroll_digit_hint()}</span>
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
    <DialogFooter class="border-t border-border pt-3 mt-2 flex items-center justify-between">
      {#if step === 'qr'}
        <Button variant="ghost" size="sm" class="text-xs" onclick={() => { open = false; onClose(); }}>
          {m.common_titlebar_close ? m.common_titlebar_close() : "Cancel"}
        </Button>
        <Button size="sm" class="text-xs font-semibold gap-1.5" onclick={() => (step = 'codes')}>
          <span>{m.common_mfa_enroll_step_codes_btn()}</span>
          <ArrowRight class="w-3.5 h-3.5 rtl:rotate-180" />
        </Button>
      {:else if step === 'codes'}
        <Button variant="ghost" size="sm" class="text-xs gap-1" onclick={() => (step = 'qr')}>
          <ArrowLeft class="w-3.5 h-3.5 rtl:rotate-180" />
          <span>{m.common_titlebar_close ? m.common_titlebar_close() : "Back"}</span>
        </Button>
        <Button size="sm" class="text-xs font-semibold gap-1.5" onclick={() => (step = 'verify')}>
          <span>{m.common_mfa_enroll_step_verify_btn()}</span>
          <ArrowRight class="w-3.5 h-3.5 rtl:rotate-180" />
        </Button>
      {:else if step === 'verify'}
        <Button variant="ghost" size="sm" class="text-xs gap-1" onclick={() => (step = 'codes')}>
          <ArrowLeft class="w-3.5 h-3.5 rtl:rotate-180" />
          <span>{m.common_titlebar_close ? m.common_titlebar_close() : "Back"}</span>
        </Button>
        <Button
          size="sm"
          class="text-xs font-semibold gap-1.5"
          disabled={verifyCode.length < 6 || isSubmitting}
          onclick={handleVerifyAndActivate}
        >
          {#if isSubmitting}
            <div class="h-3.5 w-3.5 animate-spin rounded-full border-2 border-primary-foreground border-t-transparent"></div>
            <span>...</span>
          {:else}
            <ShieldCheck class="w-3.5 h-3.5" />
            <span>{m.common_mfa_enroll_confirm_btn()}</span>
          {/if}
        </Button>
      {/if}
    </DialogFooter>
  </DialogContent>
</Dialog>
