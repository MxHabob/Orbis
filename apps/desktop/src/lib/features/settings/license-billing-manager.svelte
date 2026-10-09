<!--
@component LicenseBillingManager
Settings view for license status, plan details, features, plan limits,
product key activation, and device seat deactivation.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { licenseStore, uiStore, confirmStore } from "$lib/stores";
import { Button, LoadingIndicator, toast } from "@orbis/ui";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@orbis/ui/card";
import { Check, Bolt, ArrowPath, Trash, CreditCard } from "@orbis/icons";
import { formatLicensePlan } from "$lib/utils/license";
import { formatDate } from "$lib/core/i18n";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isStartingTrial = $state(false);
let isDeactivating = $state(false);
let copiedId = $state(false);
const planDisplay = $derived(
  formatLicensePlan(licenseStore.planName) ??
    formatLicensePlan(licenseStore.plan) ??
    formatLicensePlan(licenseStore.edition) ??
    m.settings_entitlement_plan_unavailable(),
);

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
    case "deactivation_pending":
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

<div class="h-full w-full overflow-y-auto px-5 py-8 md:px-8 scrollbar-hide font-sans text-start">
  <div class="mx-auto flex w-full max-w-5xl flex-col gap-7 pb-8">
    <header class="flex flex-wrap items-end justify-between gap-4">
      <div class="flex flex-col gap-1.5">
        <h1 class="text-2xl font-semibold tracking-tight text-foreground">{m.license_nav_title()}</h1>
      </div>
      <div class="flex flex-wrap items-center gap-2">
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
      </div>
    </header>

    <section aria-labelledby="current-plan-heading" class="flex flex-col gap-2">
      <h2 id="current-plan-heading" class="px-1 text-sm font-medium text-muted-foreground">{m.license_current_plan()}</h2>
      <Card>
        <CardContent class="flex flex-row flex-wrap items-center justify-between gap-4 p-4 text-start">
          <div class="flex min-w-0 flex-col gap-1">
            <div class="flex flex-wrap items-center gap-2">
              <span class="text-base font-semibold text-foreground">
                {planDisplay}
              </span>
              <span class="text-sm text-muted-foreground">{statusBadge.label}</span>
            </div>
            {#if licenseStore.isTrial}
              <p class="text-sm text-muted-foreground">
                {m.license_trial_days_remaining({ count: String(licenseStore.trialDaysRemaining) })}
              </p>
            {:else if licenseStore.daysUntilExpiry !== null}
              <p class="text-sm text-muted-foreground">
                {m.license_days_remaining({ count: String(licenseStore.daysUntilExpiry) })}
              </p>
            {:else if licenseStore.edition && licenseStore.edition !== licenseStore.plan}
              <p class="text-sm text-muted-foreground">{licenseStore.edition}</p>
            {/if}
          </div>
          <div class="flex shrink-0 items-center gap-2">
            <a
              href="http://mulverse.com/p/orbis/pricing"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex h-9 items-center gap-1.5 rounded-md border border-border px-3 text-sm font-medium text-foreground transition-colors hover:bg-muted"
            >
              {m.license_billing_view_plans()}
              <svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" class="size-3.5">
                <path d="M14 3h7v7M10 14 21 3" />
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              </svg>
            </a>
            {#if !licenseStore.isOperational}
              <Button variant="lime" size="sm" onclick={handleOpenActivation} class="font-semibold">
                {m.license_activate_btn()}
              </Button>
            {/if}
          </div>
        </CardContent>
      </Card>
    </section>

    <section aria-labelledby="license-details-heading" class="flex flex-col gap-2">
      <Card>
        <CardHeader class="flex flex-wrap items-center justify-between gap-3 normal-case tracking-normal">
          <div class="flex min-w-0 flex-col">
            <CardTitle id="license-details-heading" class="text-sm font-semibold text-foreground normal-case tracking-normal">
              {m.license_billing_details()}
            </CardTitle>
            <CardDescription class="text-xs">{m.license_billing_details_description()}</CardDescription>
          </div>
          {#if !licenseStore.isOperational}
            <Button variant="outline" size="sm" onclick={handleOpenActivation} class="shrink-0 text-xs">
              <Bolt size={14} class="me-1.5" />
              {m.license_activate_btn()}
            </Button>
          {/if}
        </CardHeader>
        <CardContent class="grid gap-5 p-5 md:grid-cols-[15rem_minmax(0,1fr)] md:items-center">
          <div
            class="group relative flex aspect-[1.55] w-full max-w-[15rem] flex-col justify-between overflow-hidden rounded-2xl border border-primary-foreground/15 bg-gradient-to-br from-primary via-primary to-primary/80 p-4 text-primary-foreground sm:p-5"
          >
            <div
              aria-hidden="true"
              class="license-card-light-ray pointer-events-none absolute inset-y-[-60%] start-[-65%] z-0 w-1/2 -rotate-[22deg] opacity-0"
            ></div>
            <div class="relative z-10 flex items-start justify-between gap-3">
              <span class="text-[10px] font-medium uppercase tracking-widest text-white/70">{m.license_billing_schedule_item()}</span>
              <CreditCard size={22} class="shrink-0 text-white/80" />
            </div>

            <div class="relative z-10 min-w-0">
              <p class="truncate text-xl font-semibold tracking-wide">
                {planDisplay}
              </p>
            </div>
            <div class="relative z-10 flex items-end justify-between gap-3">
              <div class="flex min-w-0 flex-col gap-0.5">
                <span class="text-[9px] uppercase tracking-wider text-white/60">{m.license_valid_until()}</span>
                <span class="truncate text-xs font-medium tabular-nums">
                  {formatDate(licenseStore.validUntil, "datetime") || "—"}
                </span>
              </div>
              <span class="shrink-0 rounded-full border border-white/20 bg-white/10 px-2 py-0.5 text-[10px] font-medium text-white/90">
                {statusBadge.label}
              </span>
            </div>
          </div>

          <style>
            .license-card-light-ray {
              background: linear-gradient(
                90deg,
                transparent,
                color-mix(in srgb, var(--primary-foreground) 3%, transparent) 30%,
                color-mix(in srgb, var(--primary-foreground) 10%, transparent) 50%,
                color-mix(in srgb, var(--primary-foreground) 3%, transparent) 70%,
                transparent
              );
            }

            .group:hover .license-card-light-ray {
              animation: license-card-light-ray 1.8s ease-in-out infinite;
            }

            @keyframes license-card-light-ray {
              0% {
                opacity: 0;
                transform: translateX(-15%) rotate(-22deg);
              }
              20%,
              80% {
                opacity: 1;
              }
              100% {
                opacity: 0;
                transform: translateX(330%) rotate(-22deg);
              }
            }

            @media (prefers-reduced-motion: reduce) {
              .group:hover .license-card-light-ray {
                animation: none;
                opacity: 0;
              }
            }
          </style>
          <dl class="flex min-w-0 flex-col">
            <div class="flex items-center justify-between gap-4 rounded-md px-3 py-2.5 odd:bg-muted/40">
              <dt class="text-sm text-muted-foreground">{m.license_status_title()}</dt>
              <dd class="text-sm font-medium text-foreground">{statusBadge.label}</dd>
            </div>
            <div class="flex items-center justify-between gap-4 rounded-md px-3 py-2.5 odd:bg-muted/40">
              <dt class="text-sm text-muted-foreground">{m.license_valid_until()}</dt>
              <dd class="text-sm font-medium tabular-nums text-foreground">
                {formatDate(licenseStore.validUntil, "datetime") || "—"}
              </dd>
            </div>
            <div class="flex items-center justify-between gap-4 rounded-md px-3 py-2.5 odd:bg-muted/40">
              <dt class="text-sm text-muted-foreground">{m.license_last_sync()}</dt>
              <dd class="text-sm font-medium tabular-nums text-foreground">
                {formatDate(licenseStore.lastSyncAt, "datetime") || m.license_never_synced()}
              </dd>
            </div>
          </dl>
        </CardContent>
      </Card>
    </section>

    <section aria-labelledby="license-schedule-heading" class="flex flex-col gap-2">
      <Card>
        <CardHeader>
          <CardTitle id="license-schedule-heading">{m.license_billing_schedule()}</CardTitle>
        </CardHeader>
        <CardContent class="p-5">
          <div class="grid grid-cols-[1fr_auto] gap-x-6 gap-y-3 text-sm sm:grid-cols-[1fr_1fr_auto]">
            <span class="text-xs text-muted-foreground">{m.license_billing_schedule_item()}</span>
            <span class="hidden text-xs text-muted-foreground sm:block">{m.license_plan_label()}</span>
            <span class="text-end text-xs text-muted-foreground">{m.license_valid_until()}</span>
            <span class="font-medium text-foreground">{statusBadge.label}</span>
            <span class="hidden text-muted-foreground sm:block">{planDisplay}</span>
            <span class="text-end tabular-nums text-foreground">
              {formatDate(licenseStore.validUntil, "datetime") || "—"}
            </span>
            </div>
        </CardContent>
      </Card>
    </section>

    {#if licenseStore.limits && licenseStore.limits.length > 0}
      <section aria-labelledby="license-limits-heading" class="flex flex-col gap-2">
        <Card>
          <CardHeader>
            <CardTitle id="license-limits-heading">{m.license_limits_title()}</CardTitle>
          </CardHeader>
          <CardContent class="p-5">
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
              {#each licenseStore.limits as limit (limit.key)}
                <div class="flex flex-col gap-1 rounded-lg border border-border/70 p-4">
                  <span class="text-xs font-medium text-muted-foreground">{getLimitLabel(limit.key)}</span>
                  <span class="text-xl font-semibold tabular-nums text-foreground">
                    {limit.max_value !== null && limit.max_value !== undefined ? limit.max_value : m.license_limit_unlimited()}
                  </span>
                  <span class="text-xs text-muted-foreground">{getUnitLabel(limit.unit)}</span>
                </div>
              {/each}
            </div>
          </CardContent>
        </Card>
      </section>
    {/if}

    <section aria-labelledby="license-features-heading" class="flex flex-col gap-2">
      <Card>
        <CardHeader>
          <CardTitle id="license-features-heading">{m.license_features_title()}</CardTitle>
        </CardHeader>
        <CardContent class="p-5">
          {#if licenseStore.enabledFeatures && licenseStore.enabledFeatures.length > 0}
            <div class="flex flex-wrap gap-2">
              {#each licenseStore.enabledFeatures as feature (feature)}
                <div class="inline-flex items-center gap-2 rounded-md border border-border bg-background px-3 py-2 text-sm font-medium text-foreground">
                  <Check size={14} class="shrink-0 text-primary" />
                  <span>{getFeatureLabel(feature)}</span>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-muted-foreground">{m.license_billing_no_features()}</p>
          {/if}
        </CardContent>
      </Card>
    </section>

    <section aria-labelledby="organization-finances-heading" class="flex flex-col gap-2">
      <h2 id="organization-finances-heading" class="px-1 text-sm font-medium text-muted-foreground">
        {m.license_billing_financial_records()}
      </h2>
      <Card>
        <CardContent class="flex flex-row flex-wrap items-center justify-between gap-4 p-5 text-start">
          <div class="flex min-w-0 flex-col gap-1">
            <span class="font-medium text-foreground">{m.finance_fees_assigned_title()}</span>
            <p class="max-w-2xl text-sm text-muted-foreground">{m.license_billing_financial_records_description()}</p>
          </div>
          <div class="flex shrink-0 flex-wrap gap-2">
            <a
              href="/{orgId}/finance/fees"
              class="inline-flex h-9 items-center rounded-md border border-border px-3 text-sm font-medium text-foreground transition-colors hover:bg-muted"
            >
              {m.license_billing_manage_fees()}
            </a>
            <a
              href="/{orgId}/finance/payments"
              class="inline-flex h-9 items-center rounded-md border border-border px-3 text-sm font-medium text-foreground transition-colors hover:bg-muted"
            >
              {m.license_billing_view_payments()}
            </a>
          </div>
        </CardContent>
      </Card>
    </section>

    <section aria-labelledby="installation-id-heading" class="flex flex-col gap-2">
      <Card>
        <CardHeader>
          <CardTitle id="installation-id-heading">{m.license_installation_id()}</CardTitle>
          <CardDescription>{m.license_installation_id_desc()}</CardDescription>
        </CardHeader>
        <CardContent class="p-5">
          <div class="flex items-center gap-2">
          <input
            id="install_id"
            type="text"
            readonly
            value={licenseStore.installationId || "—"}
            aria-label={m.license_installation_id()}
            class="h-10 min-w-0 flex-1 rounded-md border border-border bg-muted/40 px-3 font-mono text-xs text-muted-foreground select-all"
          />
          <Button
            variant="outline"
            size="sm"
            onclick={copyInstallationId}
            disabled={!licenseStore.installationId}
            class="h-10 shrink-0 px-3 text-xs"
          >
            {copiedId ? m.common_copied() : m.common_action_copy()}
          </Button>
          </div>
          {#if licenseStore.isActive}
            <div class="mt-5 flex flex-wrap items-center justify-between gap-4 border-t border-border pt-4">
              <div>
                <h3 class="text-sm font-medium text-foreground">{m.license_deactivate_title()}</h3>
                <p class="mt-1 text-xs text-muted-foreground">{m.license_deactivate_desc()}</p>
              </div>
              <Button
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
            </div>
          {/if}
        </CardContent>
      </Card>
    </section>
  </div>
</div>
