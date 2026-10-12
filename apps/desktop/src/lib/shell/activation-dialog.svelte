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
import { confirmStore, licenseStore } from "$lib/stores";
import { Check } from "@orbis/icons";
import { formatLicensePlan } from "$lib/utils/license";

let { open = $bindable(false) }: { open: boolean } = $props();

let productKey = $state("");
let isActivating = $state(false);
let isDeactivating = $state(false);
let activationError = $state<string | null>(null);
let activationSuccess = $state(false);
const planDisplay = $derived(
  formatLicensePlan(licenseStore.planName) ??
    formatLicensePlan(licenseStore.plan) ??
    formatLicensePlan(licenseStore.edition) ??
    m.settings_entitlement_plan_unavailable(),
);

function normalizeProductKey(value: string): string | null {
  let clean = value.replace(/[^A-Za-z0-9]/g, "").toUpperCase();
  if (clean.startsWith("ORBIS")) {
    clean = clean.slice(5);
  }
  if (clean.length !== 20) return null;

  const segments = clean.match(/.{5}/g);
  if (!segments || segments.length !== 4) return null;
  return `ORBIS-${segments.join("-")}`;
}

async function handleActivate() {
  const normalizedKey = normalizeProductKey(productKey);
  if (!normalizedKey) {
    activationError = m.license_invalid_key_format();
    return;
  }
  isActivating = true;
  activationError = null;
  activationSuccess = false;

  try {
    const result = await licenseStore.activateKey(normalizedKey);
    if (result) {
      activationSuccess = true;
      toast.success(m.license_activation_success());
      setTimeout(() => {
        open = false;
        activationSuccess = false;
        productKey = "";
      }, 1500);
    } else {
      activationError = licenseStore.error ?? m.license_activation_failed();
    }
  } finally {
    isActivating = false;
  }
}

async function handleDeactivate() {
  const confirmed = await confirmStore.prompt({
    title: m.license_deactivate_title(),
    description: m.license_change_deactivate_confirm(),
    confirmText: m.license_deactivate_btn(),
    variant: "destructive",
  });
  if (!confirmed) return;

  isDeactivating = true;
  try {
    const result = await licenseStore.deactivate();
    if (result) {
      activationError = null;
      productKey = "";
      toast.info(m.license_deactivated());
    } else if (licenseStore.error) {
      toast.error(licenseStore.error);
    }
  } finally {
    isDeactivating = false;
  }
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
        <DialogTitle>
          {licenseStore.isActive ? m.license_active_title() : m.license_activation_title()}
        </DialogTitle>
        <DialogDescription>
          {licenseStore.isActive ? m.license_active_description() : m.license_activation_desc()}
        </DialogDescription>
      </DialogHeader>

      <div class="flex flex-col gap-5 pt-2">
        {#if licenseStore.isActive}
          <div class="flex flex-col gap-3 rounded-lg border border-emerald-500/25 bg-emerald-500/5 p-4">
            <div class="flex items-center gap-2 text-sm font-semibold text-emerald-700 dark:text-emerald-400">
              <Check size={16} />
              <span>{m.license_active_title()}</span>
            </div>
            <dl class="grid gap-2 text-sm">
              <div class="flex items-center justify-between gap-4">
                <dt class="text-muted-foreground">{m.license_plan_label()}</dt>
                <dd class="text-end font-medium text-foreground">{planDisplay}</dd>
              </div>
              <div class="flex items-center justify-between gap-4">
                <dt class="text-muted-foreground">{m.license_status_title()}</dt>
                <dd class="font-medium text-foreground">
                  {licenseStore.state === "offline_grace"
                    ? m.license_status_offline_grace()
                    : licenseStore.state === "expiring"
                      ? m.license_status_expiring()
                      : m.license_status_active()}
                </dd>
              </div>
            </dl>
            <p class="text-xs leading-relaxed text-muted-foreground">
              {m.license_active_change_plan_hint()}
            </p>
          </div>
        {:else}
          <!-- Product Key Input -->
          <div class="flex flex-col gap-2">
            <Label for="product_key" class="text-[11px] font-semibold">
              {m.license_key_label()}
            </Label>
            <Input
              id="product_key"
              type="text"
              bind:value={productKey}
              placeholder={m.license_key_placeholder()}
              disabled={isActivating || activationSuccess}
              class="h-11 text-center font-mono text-[14px] tracking-[0.08em] uppercase"
              dir="ltr"
              autocapitalize="characters"
              autocorrect="off"
              autocomplete="off"
              spellcheck={false}
            />
          </div>
        {/if}

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
        {#if licenseStore.isActive}
          <Button
            variant="destructive"
            onclick={handleDeactivate}
            disabled={isDeactivating}
            class="text-[11px]"
          >
            {#if isDeactivating}
              <LoadingIndicator size="sm" class="me-1.5" />
              {m.license_deactivating()}
            {:else}
              {m.license_deactivate_btn()}
            {/if}
          </Button>
          <Button variant="outline" onclick={handleClose} disabled={isDeactivating} class="text-[11px]">
            {m.common_btn_cancel()}
          </Button>
        {:else}
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
            disabled={isActivating || !normalizeProductKey(productKey) || activationSuccess}
            class="text-[11px] min-w-[7rem]"
          >
            {#if isActivating}
              <LoadingIndicator size="sm" class="me-1.5" />
              {m.license_activating()}
            {:else}
              {m.license_activate_btn()}
            {/if}
          </Button>
        {/if}
      </div>
    </div>
  </DialogContent>
</Dialog>
