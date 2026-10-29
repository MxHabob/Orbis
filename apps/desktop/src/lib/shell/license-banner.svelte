<!--
@component LicenseBanner
A thin warning banner displayed at the top of the app when:
- License is expired
- Trial is ending (≤ 7 days)
- License is expiring soon (≤ 14 days)
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { licenseStore, uiStore } from "$lib/stores";
import { Button } from "@orbis/ui";
import { XMark } from "@orbis/icons";
import { slide } from "svelte/transition";

let dismissed = $state(false);

const shouldShow = $derived.by(() => {
  if (dismissed || !licenseStore.licenseState) return false;
  const s = licenseStore.state;
  if (s === "expired" || s === "suspended" || s === "revoked") return true;
  if (s === "trial" && licenseStore.trialDaysRemaining <= 7) return true;
  if (s === "expiring" && (licenseStore.daysUntilExpiry ?? 999) <= 14)
    return true;
  return false;
});

const bannerMessage = $derived.by(() => {
  const s = licenseStore.state;
  if (s === "suspended") return m.license_banner_suspended();
  if (s === "expired" || s === "revoked") return m.license_banner_expired();
  if (s === "trial")
    return m.license_banner_trial_ending({
      count: String(licenseStore.trialDaysRemaining),
    });
  if (s === "expiring")
    return m.license_banner_expiring({
      count: String(licenseStore.daysUntilExpiry ?? 0),
    });
  return "";
});

const isUrgent = $derived(
  licenseStore.state === "expired" ||
    licenseStore.state === "suspended" ||
    licenseStore.state === "revoked",
);

function handleActivate() {
  uiStore.setActivationDialog(true);
}
</script>

{#if shouldShow}
  <div
    transition:slide={{ duration: 200, axis: "y" }}
    class="flex items-center justify-center gap-3 px-4 py-1.5 text-[11px] font-medium shrink-0 select-none {isUrgent ? 'bg-destructive/10 text-destructive border-b border-destructive/15' : 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-b border-amber-500/15'}"
  >
    <span>{bannerMessage}</span>
    <Button
      variant="outline"
      size="sm"
      onclick={handleActivate}
      class="h-6 px-2.5 text-[10px] font-bold"
    >
      {m.license_banner_activate()}
    </Button>
    {#if !isUrgent}
      <button
        onclick={() => dismissed = true}
        class="ms-auto p-0.5 rounded hover:bg-foreground/10 transition-colors"
        aria-label="Dismiss"
      >
        <XMark size={12} />
      </button>
    {/if}
  </div>
{/if}
