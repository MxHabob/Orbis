<!--
@component LicenseBillingManager
Refined billing & license management view following Orbis design tokens:
1. Hero plan overview with status, key dates, and primary actions.
2. Unified limits & enabled features summary.
3. Financial shortcuts, installation ID, and seat deactivation.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { resolve } from "$app/paths";
import { licenseStore, uiStore, confirmStore } from "$lib/stores";
import { Button, LoadingIndicator, toast } from "@orbis/ui";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { Bolt, ArrowPath, Check, Trash } from "@orbis/icons";
import { MULVERSE_URLS } from "@orbis/utils";
import { formatLicensePlan } from "$lib/utils/license";
import { formatDate } from "$lib/core/i18n";
import LicensePlanCard from "./components/license-plan-card.svelte";
import CopyableInput from "$lib/shared/components/copyable-input.svelte";

let { orgId }: { orgId: string } = $props();

let isRefreshing = $state(false);
let isStartingTrial = $state(false);
let isDeactivating = $state(false);

const planDisplay = $derived(
  formatLicensePlan(licenseStore.planName) ??
    formatLicensePlan(licenseStore.plan) ??
    formatLicensePlan(licenseStore.edition) ??
    m.settings_entitlement_plan_unavailable(),
);

async function handleRefresh() {
  isRefreshing = true;
  const result = await licenseStore.refresh();
  if (result) {
    toast.success(m.license_activation_success());
  } else if (licenseStore.error) {
    toast.error(licenseStore.error);
  }
  isRefreshing = false;
}

async function handleLoadState() {
  const result = await licenseStore.loadState();
  if (!result && licenseStore.error) {
    toast.error(licenseStore.error);
  }
}

async function handleStartTrial() {
  isStartingTrial = true;
  const result = await licenseStore.startTrial();
  if (result) {
    toast.success(m.license_trial_started());
  } else if (licenseStore.error) {
    toast.error(licenseStore.error);
  }
  isStartingTrial = false;
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
  const result = await licenseStore.deactivate();
  if (result) {
    toast.info(m.license_deactivated());
  } else if (licenseStore.error) {
    toast.error(licenseStore.error);
  }
  isDeactivating = false;
}

function handleOpenActivation() {
  uiStore.setActivationDialog(true);
}

function getFeatureLabel(key: string): string {
  switch (key) {
    case "students":
      return m.license_feature_students();
    case "ai_grading":
      return m.license_feature_ai_grading();
    case "multi_branch":
      return m.license_feature_multi_branch();
    case "offline_sync":
      return m.license_feature_offline_sync();
    case "parent_portal":
      return m.license_feature_parent_portal();
    case "campus_analytics":
      return m.license_feature_campus_analytics();
    case "sso":
      return m.license_feature_sso();
    case "exam_tools":
      return m.license_feature_exam_tools();
    default:
      return key.replace(/_/g, " ");
  }
}

function getLimitLabel(key: string): string {
  switch (key) {
    case "students.max":
      return m.license_limit_students_max();
    case "staff.max":
      return m.license_limit_staff_max();
    case "devices.max":
      return m.license_limit_devices_max();
    case "branches.max":
      return m.license_limit_branches_max();
    case "classes.max":
      return m.license_limit_classes_max();
    case "storage_gb.max":
      return m.license_limit_storage_gb_max();
    default:
      return key.replace(/_/g, " ").replace(/\.max$/, "");
  }
}

function getUnitLabel(unit?: string): string {
  switch (unit) {
    case "students":
      return m.license_unit_students();
    case "users":
      return m.license_unit_users();
    case "devices":
      return m.license_unit_devices();
    case "branches":
      return m.license_unit_branches();
    case "classes":
      return m.license_unit_classes();
    case "GB":
      return m.license_unit_gb();
    default:
      return unit ?? "";
  }
}

const statusBadge = $derived.by(() => {
  const state = licenseStore.state;
  switch (state) {
    case "active":
      return {
        label: m.license_status_active(),
        class: "bg-success/10 text-success border-success/20",
      };
    case "trial":
      return {
        label: m.license_status_trial(),
        class: "bg-info/10 text-info border-info/20",
      };
    case "expiring":
    case "offline_grace":
      return {
        label:
          state === "expiring"
            ? m.license_status_expiring()
            : m.license_status_offline_grace(),
        class: "bg-warning/10 text-warning border-warning/20",
      };
    case "expired":
    case "suspended":
    case "deactivation_pending":
    case "revoked":
      return {
        label:
          state === "expired"
            ? m.license_status_expired()
            : state === "revoked"
              ? m.license_status_revoked()
              : m.license_status_suspended(),
        class: "bg-destructive/10 text-destructive border-destructive/20",
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

const hasLimits = $derived(
  Boolean(licenseStore.limits && licenseStore.limits.length > 0),
);
const hasFeatures = $derived(
  Boolean(licenseStore.enabledFeatures && licenseStore.enabledFeatures.length > 0),
);
</script>

<div class="h-full w-full overflow-y-auto px-5 py-7 md:px-8 scrollbar-hide font-sans text-start">
  <div class="mx-auto flex w-full max-w-4xl flex-col gap-6 pb-10">
    <!-- Page Header -->
    <header class="flex flex-wrap items-center justify-between gap-4">
      <div class="flex flex-col gap-1">
        <div class="flex items-center gap-2.5">
          <h1 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
            {m.license_nav_title()}
          </h1>
          <span
            class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-[11px] font-medium {statusBadge.class}"
          >
            {statusBadge.label}
          </span>
        </div>
        {#if !licenseStore.hasLoaded && licenseStore.error}
          <p class="text-xs text-destructive" role="alert">{licenseStore.error}</p>
        {/if}
      </div>

      <div class="flex flex-wrap items-center gap-2">
        {#if !licenseStore.hasLoaded && licenseStore.error}
          <Button
            variant="outline"
            size="sm"
            onclick={handleLoadState}
            disabled={licenseStore.isLoading}
            class="text-xs"
          >
            {#if licenseStore.isLoading}
              <LoadingIndicator size="sm" class="me-1.5" />
            {/if}
            {m.common_retry()}
          </Button>
        {/if}

        {#if licenseStore.hasLoaded && licenseStore.state === "unavailable"}
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

    <!-- 1. Plan & License Overview -->
    <Card>
      <CardContent class="grid gap-6 p-5 md:grid-cols-[14rem_minmax(0,1fr)] md:items-center">
        <LicensePlanCard
          planCode={licenseStore.isTrial
            ? "trial"
            : licenseStore.plan ?? licenseStore.planName ?? licenseStore.edition ?? ""}
          planLabel={planDisplay}
          validUntil={licenseStore.validUntil}
        />

        <div class="flex min-w-0 flex-col justify-between gap-5">
          <div class="flex flex-col gap-1">
            <span class="text-xs font-medium text-muted-foreground">
              {m.license_current_plan()}
            </span>
            <h2 class="text-xl font-bold tracking-tight text-foreground">
              {planDisplay}
            </h2>
            {#if licenseStore.isTrial}
              <p class="text-xs text-muted-foreground">
                {m.license_trial_days_remaining({ count: String(licenseStore.trialDaysRemaining) })}
              </p>
            {:else if licenseStore.daysUntilExpiry !== null}
              <p class="text-xs text-muted-foreground">
                {m.license_days_remaining({ count: String(licenseStore.daysUntilExpiry) })}
              </p>
            {:else if licenseStore.edition && licenseStore.edition !== licenseStore.plan}
              <p class="text-xs text-muted-foreground">{licenseStore.edition}</p>
            {/if}
          </div>

          <dl class="grid grid-cols-1 gap-3 border-y border-border/60 py-3.5 sm:grid-cols-2">
            <div class="flex flex-col gap-0.5">
              <dt class="text-[11px] text-muted-foreground">{m.license_valid_until()}</dt>
              <dd class="text-sm font-medium tabular-nums text-foreground">
                {formatDate(licenseStore.validUntil, "datetime") || "—"}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[11px] text-muted-foreground">{m.license_last_sync()}</dt>
              <dd class="text-sm font-medium tabular-nums text-foreground">
                {formatDate(licenseStore.lastSyncAt, "datetime") || m.license_never_synced()}
              </dd>
            </div>
          </dl>

          <div class="flex flex-wrap items-center gap-2">
            <Button variant="default" size="sm" onclick={handleOpenActivation}>
              {#if !licenseStore.isOperational}
                <Bolt size={14} class="me-1.5" />
                {m.license_activate_btn()}
              {:else}
                {m.license_manage_btn()}
              {/if}
            </Button>
            <a
              href={MULVERSE_URLS.orbisPricing}
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex h-8 items-center gap-1.5 rounded-md border border-border px-3 text-xs font-medium text-foreground transition-colors hover:bg-muted"
            >
              {m.license_billing_view_plans()}
              <svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" class="size-3.5 text-muted-foreground">
                <path d="M14 3h7v7M10 14 21 3" />
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              </svg>
            </a>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 2. Quotas & Enabled Features -->
    {#if hasLimits || hasFeatures}
      <Card>
        <CardContent class="flex flex-col gap-6 p-5">
          {#if hasLimits}
            <div class="flex flex-col gap-3">
              <h3 class="text-sm font-semibold text-foreground">{m.license_limits_title()}</h3>
              <div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
                {#each licenseStore.limits as limit (limit.key)}
                  <div class="flex flex-col gap-0.5 rounded-md bg-muted/40 px-3.5 py-3">
                    <span class="text-xs text-muted-foreground">{getLimitLabel(limit.key)}</span>
                    <div class="flex items-baseline gap-1.5">
                      <span class="text-lg font-bold tabular-nums text-foreground">
                        {limit.max_value !== null && limit.max_value !== undefined
                          ? limit.max_value
                          : m.license_limit_unlimited()}
                      </span>
                      <span class="text-[11px] text-muted-foreground">
                        {getUnitLabel(limit.unit)}
                      </span>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          {#if hasFeatures}
            <div class="flex flex-col gap-3 {hasLimits ? 'border-t border-border/60 pt-5' : ''}">
              <h3 class="text-sm font-semibold text-foreground">{m.license_features_title()}</h3>
              <div class="flex flex-wrap gap-2">
                {#each licenseStore.enabledFeatures as feature (feature)}
                  <span class="inline-flex items-center gap-1.5 rounded-md border border-border/70 bg-background px-2.5 py-1.5 text-xs font-medium text-foreground">
                    <Check size={13} class="shrink-0 text-primary" />
                    <span>{getFeatureLabel(feature)}</span>
                  </span>
                {/each}
              </div>
            </div>
          {/if}
        </CardContent>
      </Card>
    {/if}

    <!-- 3. Financial Shortcuts & Device Seat -->
    <Card>
      <CardHeader class="pb-3">
        <CardTitle class="text-sm font-semibold">{m.license_billing_financial_records()}</CardTitle>
        <CardDescription class="text-xs">
          {m.license_billing_financial_records_description()}
        </CardDescription>
      </CardHeader>
      <CardContent class="flex flex-col gap-5 p-5 pt-0">
        <div class="flex flex-wrap items-center gap-2">
          <a
            href={resolve(`/${orgId}/finance/fees`)}
            class="inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground transition-colors hover:bg-muted"
          >
            {m.license_billing_manage_fees()}
          </a>
          <a
            href={resolve(`/${orgId}/finance/payments`)}
            class="inline-flex h-8 items-center rounded-md border border-border px-3 text-xs font-medium text-foreground transition-colors hover:bg-muted"
          >
            {m.license_billing_view_payments()}
          </a>
        </div>

        <div class="border-t border-border/60 pt-4">
          <CopyableInput
            id="install_id"
            label={m.license_installation_id()}
            value={licenseStore.installationId}
          />
        </div>

        {#if licenseStore.isActive}
          <div class="flex flex-wrap items-center justify-between gap-4 border-t border-border/60 pt-4">
            <div>
              <h3 class="text-sm font-medium text-foreground">{m.license_deactivate_title()}</h3>
              <p class="mt-0.5 text-xs text-muted-foreground">{m.license_deactivate_desc()}</p>
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
  </div>
</div>
