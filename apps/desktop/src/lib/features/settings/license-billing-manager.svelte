<!--
@component LicenseBillingManager
Settings view for license status, plan details, features, plan limits,
product key activation, and device seat deactivation.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { licenseStore, uiStore, confirmStore } from "$lib/stores";
import { Button, LoadingIndicator, toast } from "@orbis/ui";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { Check, ShieldCheck, Bolt, ArrowPath, Trash, InfoCircle } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isStartingTrial = $state(false);
let isDeactivating = $state(false);
let copiedId = $state(false);

async function handleRefresh() {
  isRefreshing = true;
  try {
    const result = await licenseStore.refresh();
    if (result) {
      toast.success(m.license_activation_success());
    } else if (licenseStore.error) {
      toast.error(licenseStore.error);
    }
  } catch (e: any) {
    toast.error(e?.message || m.license_activation_failed());
  } finally {
    isRefreshing = false;
  }
}

async function handleStartTrial() {
  isStartingTrial = true;
  try {
    const res = await licenseStore.startTrial();
    if (res) {
      toast.success(m.license_trial_started());
    } else if (licenseStore.error) {
      toast.error(licenseStore.error);
    }
  } catch (e: any) {
    toast.error(e?.message || m.license_activation_failed());
  } finally {
    isStartingTrial = false;
  }
}

async function handleDeactivate() {
  const confirmed = await confirmStore.prompt({
    title: m.license_deactivate_title(),
    description: m.license_deactivate_confirm(),
    confirmText: m.license_deactivate_btn(),
    variant: "destructive",
  });

  if (!confirmed) return;

  isDeactivating = true;
  try {
    const res = await licenseStore.deactivate();
    if (res) {
      toast.info(m.license_deactivated());
    } else if (licenseStore.error) {
      toast.error(licenseStore.error);
    }
  } catch (e: any) {
    toast.error(e?.message || m.license_activation_failed());
  } finally {
    isDeactivating = false;
  }
}

function handleOpenActivation() {
  uiStore.setActivationDialog(true);
}

async function copyInstallationId() {
  if (!licenseStore.installationId) return;
  try {
    await navigator.clipboard.writeText(licenseStore.installationId);
    copiedId = true;
    toast.success(m.common_copied());
    setTimeout(() => {
      copiedId = false;
    }, 2000);
  } catch {
    toast.error("Failed to copy");
  }
}

function getFeatureLabel(key: string): string {
  switch (key) {
    case "students": return m.license_feature_students();
    case "ai_grading": return m.license_feature_ai_grading();
    case "multi_branch": return m.license_feature_multi_branch();
    case "offline_sync": return m.license_feature_offline_sync();
    case "parent_portal": return m.license_feature_parent_portal();
    case "campus_analytics": return m.license_feature_campus_analytics();
    case "sso": return m.license_feature_sso();
    case "exam_tools": return m.license_feature_exam_tools();
    default: return key.replace(/_/g, " ");
  }
}

function getLimitLabel(key: string): string {
  switch (key) {
    case "students.max": return m.license_limit_students_max();
    case "staff.max": return m.license_limit_staff_max();
    case "devices.max": return m.license_limit_devices_max();
    case "branches.max": return m.license_limit_branches_max();
    case "classes.max": return m.license_limit_classes_max();
    case "storage_gb.max": return m.license_limit_storage_gb_max();
    default: return key.replace(/_/g, " ").replace(/\.max$/, "");
  }
}

function getUnitLabel(unit?: string): string {
  switch (unit) {
    case "students": return m.license_unit_students();
    case "users": return m.license_unit_users();
    case "devices": return m.license_unit_devices();
    case "branches": return m.license_unit_branches();
    case "classes": return m.license_unit_classes();
    case "GB": return m.license_unit_gb();
    default: return unit ?? "";
  }
}

const statusBadge = $derived.by(() => {
  const state = licenseStore.state;
  switch (state) {
    case "active":
      return {
        label: m.license_status_active(),
        class: "bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border-emerald-500/20",
      };
    case "trial":
      return {
        label: m.license_status_trial(),
        class: "bg-blue-500/10 text-blue-700 dark:text-blue-400 border-blue-500/20",
      };
    case "expiring":
      return {
        label: m.license_status_expiring(),
        class: "bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20",
      };
    case "expired":
      return {
        label: m.license_status_expired(),
        class: "bg-destructive/10 text-destructive border-destructive/20",
      };
    case "suspended":
      return {
        label: m.license_status_suspended(),
        class: "bg-destructive/10 text-destructive border-destructive/20",
      };
    case "revoked":
      return {
        label: m.license_status_revoked(),
        class: "bg-destructive/10 text-destructive border-destructive/20",
      };
    case "offline_grace":
      return {
        label: m.license_status_offline_grace(),
        class: "bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20",
      };
    case "pending":
      return {
        label: m.license_status_pending(),
        class: "bg-muted text-muted-foreground border-border",
      };
    default:
      return {
        label: m.license_status_unavailable(),
        class: "bg-muted text-muted-foreground border-border",
      };
  }
});
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide font-sans text-start">
  <!-- Top Header / Quick Actions -->
  <div class="flex items-center justify-between gap-4 flex-wrap shrink-0">
    <div class="flex flex-col gap-1">
      <h1 class="text-xl font-bold text-foreground">{m.license_nav_title()}</h1>
      <p class="text-xs text-muted-foreground">{m.license_upgrade_plan()}</p>
    </div>

    <div class="flex items-center gap-2.5">
      {#if licenseStore.state === "unavailable"}
        <Button
          variant="outline"
          size="sm"
          onclick={handleStartTrial}
          disabled={isStartingTrial}
          class="text-xs"
        >
          {#if isStartingTrial}
            <LoadingIndicator size="sm" class="me-1.5" />
          {/if}
          {m.license_start_trial_btn()}
        </Button>
      {/if}

      <Button
        variant="outline"
        size="sm"
        onclick={handleRefresh}
        disabled={isRefreshing}
        class="text-xs"
      >
        {#if isRefreshing}
          <LoadingIndicator size="sm" class="me-1.5" />
          {m.license_refreshing()}
        {:else}
          <ArrowPath size={14} class="me-1.5" />
          {m.license_refresh_btn()}
        {/if}
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={handleOpenActivation}
        class="text-xs font-semibold"
      >
        <Bolt size={14} class="me-1.5" />
        {m.license_activate_btn()}
      </Button>
    </div>
  </div>

  <div class="flex flex-col gap-6 max-w-4xl">
    <!-- License Overview Card -->
    <Card>
      <CardHeader>
        <div class="flex items-center justify-between">
          <CardTitle>{m.license_status_title()}</CardTitle>
          <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold border {statusBadge.class}">
            {statusBadge.label}
          </span>
        </div>
        <CardDescription>
          {#if licenseStore.isTrial}
            {m.license_trial_days_remaining({ count: String(licenseStore.trialDaysRemaining) })}
          {:else if licenseStore.daysUntilExpiry !== null}
            {m.license_days_remaining({ count: String(licenseStore.daysUntilExpiry) })}
          {:else}
            {licenseStore.product} {licenseStore.edition ?? ""}
          {/if}
        </CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x rtl:md:divide-x-reverse divide-border">
          <div class="px-6 py-4 flex flex-col gap-1">
            <span class="text-xs font-semibold text-muted-foreground">{m.license_plan_label()}</span>
            <span class="text-lg font-bold text-foreground capitalize">{licenseStore.plan ?? "—"}</span>
            <span class="text-[11px] text-muted-foreground">{licenseStore.edition ?? licenseStore.product}</span>
          </div>

          <div class="px-6 py-4 flex flex-col gap-1">
            <span class="text-xs font-semibold text-muted-foreground">{m.license_valid_until()}</span>
            <span class="text-lg font-bold text-foreground font-mono">
              {#if licenseStore.validUntil}
                {new Date(licenseStore.validUntil).toLocaleDateString()}
              {:else}
                —
              {/if}
            </span>
            <span class="text-[11px] text-muted-foreground">
              {#if licenseStore.isTrial}
                {m.license_trial_days_remaining({ count: String(licenseStore.trialDaysRemaining) })}
              {:else if licenseStore.daysUntilExpiry !== null}
                {m.license_days_remaining({ count: String(licenseStore.daysUntilExpiry) })}
              {:else}
                —
              {/if}
            </span>
          </div>

          <div class="px-6 py-4 flex flex-col gap-1">
            <span class="text-xs font-semibold text-muted-foreground">{m.license_last_sync()}</span>
            <span class="text-lg font-bold text-foreground font-mono">
              {#if licenseStore.lastSyncAt}
                {new Date(licenseStore.lastSyncAt).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              {:else}
                {m.license_never_synced()}
              {/if}
            </span>
            <span class="text-[11px] text-muted-foreground">
              {licenseStore.isOperational ? "Operational" : "Restricted"}
            </span>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Plan Limits Card -->
    {#if licenseStore.limits && licenseStore.limits.length > 0}
      <Card>
        <CardHeader>
          <CardTitle>{m.license_limits_title()}</CardTitle>
        </CardHeader>
        <CardContent class="p-0">
          <div class="grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x rtl:md:divide-x-reverse divide-border">
            {#each licenseStore.limits as limit}
              <div class="px-6 py-4 flex flex-col gap-1">
                <span class="text-xs font-semibold text-muted-foreground">
                  {getLimitLabel(limit.key)}
                </span>
                <span class="text-2xl font-bold text-foreground font-mono tabular-nums">
                  {limit.max_value !== null && limit.max_value !== undefined ? limit.max_value : m.license_limit_unlimited()}
                </span>
                <span class="text-[11px] text-muted-foreground">
                  {getUnitLabel(limit.unit)}
                </span>
              </div>
            {/each}
          </div>
        </CardContent>
      </Card>
    {/if}

    <!-- Enabled Features Card -->
    <Card>
      <CardHeader>
        <CardTitle>{m.license_features_title()}</CardTitle>
      </CardHeader>
      <CardContent>
        {#if licenseStore.enabledFeatures && licenseStore.enabledFeatures.length > 0}
          <div class="flex flex-wrap gap-2.5">
            {#each licenseStore.enabledFeatures as feature}
              <div class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-border bg-card text-xs font-semibold text-foreground">
                <Check size={13} class="text-primary shrink-0" />
                <span>{getFeatureLabel(feature)}</span>
              </div>
            {/each}
          </div>
        {:else}
          <p class="text-xs text-muted-foreground italic">No specific feature flags registered.</p>
        {/if}
      </CardContent>
    </Card>

    <!-- Installation & Seat Management Card -->
    <Card>
      <CardHeader>
        <CardTitle>{m.license_installation_id()}</CardTitle>
        <CardDescription>{m.license_installation_id_desc()}</CardDescription>
      </CardHeader>
      <CardContent class="flex flex-col gap-4">
        <SettingsRow
          title={m.license_installation_id()}
          description={m.license_installation_id_desc()}
          labelFor="install_id"
        >
          {#snippet children()}
            <div class="flex items-center gap-2 w-full">
              <input
                id="install_id"
                type="text"
                readonly
                value={licenseStore.installationId || "—"}
                class="h-9 flex-1 bg-muted/40 text-muted-foreground border border-border/80 rounded-md px-3 font-mono text-xs select-all"
              />
              <Button
                variant="outline"
                size="sm"
                onclick={copyInstallationId}
                disabled={!licenseStore.installationId}
                class="text-xs h-9 px-3 shrink-0"
              >
                {copiedId ? m.common_copied() : m.common_action_copy()}
              </Button>
            </div>
          {/snippet}
        </SettingsRow>

        {#if licenseStore.isActive}
          <SettingsRow
            title={m.license_deactivate_title()}
            description={m.license_deactivate_desc()}
            labelFor="btn_deactivate"
            last
          >
            {#snippet children()}
              <Button
                id="btn_deactivate"
                variant="destructive"
                size="sm"
                onclick={handleDeactivate}
                disabled={isDeactivating}
                class="text-xs"
              >
                {#if isDeactivating}
                  <LoadingIndicator size="sm" class="me-1.5" />
                  {m.common_saving()}
                {:else}
                  <Trash size={14} class="me-1.5" />
                  {m.license_deactivate_btn()}
                {/if}
              </Button>
            {/snippet}
          </SettingsRow>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>
