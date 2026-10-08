<!--
@component ActivationDialog
Product key activation dialog. Can be opened from:
- Upgrade dialog
- Settings → License & Billing
- License expired banner
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orbis/ui/dialog";
import { Button, LoadingIndicator } from "@orbis/ui";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import { licenseStore } from "$lib/stores";
import { Check } from "@orbis/icons";

let { open = $bindable(false) }: { open: boolean } = $props();

let productKey = $state("");
let isActivating = $state(false);
let activationError = $state<string | null>(null);
let activationSuccess = $state(false);

// Format key as user types (ORBIS-XXXXX-XXXXX-XXXXX-XXXXX, 29 chars, Base32)
function formatKey(value: string): string {
  let clean = value.replace(/[^A-Za-z0-9]/g, "").toUpperCase();
  if (!clean || clean === "ORBIS") return "";

  if (clean.startsWith("ORBIS")) {
    clean = clean.slice(5);
  }

  if (!clean) return "";

  clean = clean.slice(0, 20);
  const parts = clean.match(/.{1,5}/g) || [];
  return "ORBIS-" + parts.join("-");
}

function handleInput(e: Event) {
  const target = e.target as HTMLInputElement;
  productKey = formatKey(target.value);
}

async function handleActivate() {
  if (!productKey || productKey.length < 29) return;
  
  isActivating = true;
  activationError = null;
  activationSuccess = false;

  const result = await licenseStore.activateKey(productKey);

  if (result.success) {
    activationSuccess = true;
    toast.success(m.license_activation_success());
    setTimeout(() => {
      open = false;
      activationSuccess = false;
      productKey = "";
    }, 1500);
  } else {
    activationError = result.message;
  }

  isActivating = false;
}

function handleClose() {
  open = false;
  activationError = null;
  activationSuccess = false;
  productKey = "";
}
</script>

<Dialog bind:open onOpenChange={(v) => { if (!v) handleClose(); }}>
  <DialogContent
    showCloseButton={true}
    class="sm:max-w-[30rem] w-full"
  >
    <DialogHeader>
      <DialogTitle>{m.license_activation_title()}</DialogTitle>
      <DialogDescription>{m.license_activation_desc()}</DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-5 pt-2">
      <!-- Product Key Input -->
      <div class="flex flex-col gap-2">
        <Label for="product_key" class="text-[11px] font-semibold">
          {m.license_key_label()}
        </Label>
        <Input
          id="product_key"
          type="text"
          value={productKey}
          oninput={handleInput}
          placeholder={m.license_key_placeholder()}
          disabled={isActivating || activationSuccess}
          class="h-11 text-center font-mono text-[14px] tracking-[0.08em] uppercase"
          maxlength={29}
          autocomplete="off"
          spellcheck={false}
        />
      </div>

      <!-- Error Message -->
      {#if activationError}
        <div class="flex items-start gap-2 rounded-md bg-destructive/10 border border-destructive/20 p-3">
          <span class="text-[11px] text-destructive leading-relaxed">{activationError}</span>
        </div>
      {/if}

      <!-- Success Message -->
      {#if activationSuccess}
        <div class="flex items-center gap-2 rounded-md bg-emerald-500/10 border border-emerald-500/20 p-3">
          <Check size={14} class="text-emerald-600 dark:text-emerald-400 shrink-0" />
          <span class="text-[11px] text-emerald-700 dark:text-emerald-300 font-medium">
            {m.license_activation_success()}
          </span>
        </div>
      {/if}

      <!-- Actions -->
      <div class="flex items-center justify-end gap-3 pt-1">
        <Button
          variant="outline"
          onclick={handleClose}
          disabled={isActivating}
          class="text-[11px]"
        >
          {m.common_btn_cancel()}
        </Button>
        <Button
          onclick={handleActivate}
          disabled={isActivating || productKey.length < 29 || activationSuccess}
          class="text-[11px] min-w-[7rem]"
        >
          {#if isActivating}
            <LoadingIndicator size="sm" class="me-1.5" />
            {m.license_activating()}
          {:else}
            {m.license_activate_btn()}
          {/if}
        </Button>
      </div>
    </div>
  </DialogContent>
</Dialog>
