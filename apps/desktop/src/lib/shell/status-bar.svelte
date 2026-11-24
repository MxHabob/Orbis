<script lang="ts">
import { onMount } from "svelte";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import * as m from "$lib/paraglide/messages";
import {
  academicContext,
  authStore,
  effectiveContext,
  licenseStore,
  operationStore,
  uiStore,
} from "$lib/stores";
import { api, isDesktopApp } from "$lib/api";
import {
  createOrganizationQuery,
  createOrgUnitsQuery,
} from "$lib/features/settings/queries";
import { createprogressionJobsQuery } from "$lib/features/academics/promotion-queries";
import { updateManager } from "$lib/features/system/update-manager.svelte";
import type { BulkCommitResult } from "$lib/features/bulk-entry/types";
import type { SolverSetupStatusDto } from "$lib/api/types";
import type {
  BulkRegisterJobDto,
  BulkRegisterEmployeeJobDto,
} from "$lib/api/types";
import { l } from "$lib/core/i18n/locale";
import { Progress } from "@orbis/ui/progress";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import {
  AcademicCap,
  ArrowPath,
  BuildingOffice,
  CheckCircle,
  Lock,
  XMark,
} from "@orbis/icons";
import SystemClock from "./system-clock.svelte";

const orgId = $derived(authStore.orgId);
const organizationQuery = createOrganizationQuery(() => orgId);
const orgUnitsQuery = createOrgUnitsQuery(() => orgId);
const progressionJobsQuery = createprogressionJobsQuery(() => orgId);
let operationOpen = $state(false);
let licenseOpen = $state(false);
let selectedImportResult = $state<BulkCommitResult | null>(null);
let solverSetup = $state<SolverSetupStatusDto | null>(null);
let unlistenSolver: UnlistenFn | undefined;
let solverReadyTimer: ReturnType<typeof setTimeout> | undefined;
let isUpdateActionRunning = $state(false);
let isSolverActionRunning = $state(false);
let isLicenseAlertDismissed = $state(false);

const DISMISSED_UPDATE_VERSION_KEY = "orbis-dismissed-update-version";

function readDismissedUpdateVersion() {
  if (typeof window === "undefined") return null;
  try {
    return window.localStorage.getItem(DISMISSED_UPDATE_VERSION_KEY);
  } catch (error) {
    console.warn("Could not read dismissed update version", error);
    return null;
  }
}

function getErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

function getUpdateErrorMessage(error: unknown) {
  const message = getErrorMessage(error).toLowerCase();
  if (
    message.includes("no internet connection") ||
    message.includes("offline")
  ) {
    return m.ui_update_offline();
  }
  if (
    message.includes("updater does not have any endpoints set") ||
    message.includes("updates are not configured")
  ) {
    return m.ui_update_unavailable();
  }
  return m.ui_update_check_failed();
}

let dismissedUpdateVersion = $state(readDismissedUpdateVersion());

const organizationName = $derived(
  l(organizationQuery.data?.name_ar, organizationQuery.data?.name ?? ""),
);
const branches = $derived(
  (orgUnitsQuery.data ?? []).filter(
    (unit) => unit.is_active && unit.unit_type === "branch",
  ),
);
const branchName = $derived.by(() => {
  if (orgUnitsQuery.isError) return m.common_error_occurred();
  if (!effectiveContext.activeOrgUnitId) return m.shared_ui_all_branches();
  const branch = branches.find(
    (unit) => unit.id === effectiveContext.activeOrgUnitId,
  );
  return branch
    ? l(branch.name_ar, branch.name)
    : m.shared_ui_selected_branch();
});

const academicPeriod = $derived(
  academicContext.periods.find(
    (period) => period.id === academicContext.effectivePeriodId,
  ) ?? null,
);
const academicContextLabel = $derived(
  [
    academicContext.currentYear
      ? l(academicContext.currentYear.name_ar, academicContext.currentYear.name)
      : "",
    academicPeriod ? l(academicPeriod.name_ar, academicPeriod.name) : "",
  ]
    .filter(Boolean)
    .join(" · "),
);

const isLicenseUrgent = $derived(
  licenseStore.state === "expired" ||
    licenseStore.state === "suspended" ||
    licenseStore.state === "revoked",
);
const shouldShowLicenseAlert = $derived.by(() => {
  if (!licenseStore.licenseState) return false;
  if (isLicenseUrgent) return true;
  if (licenseStore.state === "trial" && licenseStore.trialDaysRemaining <= 7) {
    return true;
  }
  return (
    licenseStore.state === "expiring" &&
    (licenseStore.daysUntilExpiry ?? 999) <= 14
  );
});
const licenseAlertMessage = $derived.by(() => {
  if (licenseStore.state === "suspended") return m.license_banner_suspended();
  if (licenseStore.state === "expired" || licenseStore.state === "revoked") {
    return m.license_banner_expired();
  }
  if (licenseStore.state === "trial") {
    return m.license_banner_trial_ending({
      count: String(licenseStore.trialDaysRemaining),
    });
  }
  if (licenseStore.state === "expiring") {
    return m.license_banner_expiring({
      count: String(licenseStore.daysUntilExpiry ?? 0),
    });
  }
  return "";
});
const licenseStatusLabel = $derived.by(() => {
  switch (licenseStore.state) {
    case "active":
      return m.license_status_active();
    case "pending":
    case "deactivation_pending":
      return m.license_status_pending();
    case "trial":
      return m.license_status_trial();
    case "expiring":
      return m.license_status_expiring();
    case "expired":
      return m.license_status_expired();
    case "suspended":
      return m.license_status_suspended();
    case "revoked":
      return m.license_status_revoked();
    case "offline_grace":
      return m.license_status_offline_grace();
    case "unavailable":
    case "invalid":
      return m.license_status_unavailable();
  }
});
const licenseStatusSummary = $derived(
  shouldShowLicenseAlert && !isLicenseAlertDismissed
    ? licenseAlertMessage
    : licenseStatusLabel,
);

const visibleOperations = $derived(
  operationStore.operations.filter(
    (operation) =>
      operation.status === "running" ||
      operation.status === "failed" ||
      operation.status === "interrupted" ||
      operation.status === "completed",
  ),
);
const activeOperation = $derived(
  visibleOperations.find((operation) => operation.status === "running") ?? null,
);
const attentionCount = $derived(
  visibleOperations.filter(
    (operation) =>
      operation.status === "failed" || operation.status === "interrupted",
  ).length,
);
const visiblePromotionJob = $derived(
  progressionJobsQuery.data?.find((job) =>
    ["draft", "preview_ready", "executing"].includes(job.status),
  ) ?? null,
);
const promotionNeedsAttention = $derived(
  visiblePromotionJob?.status === "draft" ||
    visiblePromotionJob?.status === "preview_ready",
);
const showUpdate = $derived.by(() => {
  const latestVersion = updateManager.updateResult?.latest_version;
  if (updateManager.updateResult?.status === "unavailable") return false;
  const isVisible =
    updateManager.isDownloading ||
    updateManager.isApplying ||
    updateManager.isReadyToRestart ||
    !!updateManager.error ||
    !!updateManager.updateResult?.has_update;
  return isVisible && dismissedUpdateVersion !== latestVersion;
});
const showSolverSetup = $derived(
  !!solverSetup && ["preparing", "failed", "ready"].includes(solverSetup.state),
);
const hasOperations = $derived(
  visibleOperations.length > 0 ||
    !!visiblePromotionJob ||
    showUpdate ||
    showSolverSetup,
);
const hasAttention = $derived(
  attentionCount > 0 ||
    promotionNeedsAttention ||
    (showUpdate && !!updateManager.error) ||
    (showSolverSetup && solverSetup?.state === "failed"),
);
const operationLabel = $derived(
  hasAttention
    ? m.ui_shell_status_needs_attention()
    : (activeOperation?.title ??
        visibleOperations[0]?.title ??
        visiblePromotionJob?.name ??
        (showUpdate
          ? updateManager.isReadyToRestart
            ? m.ui_download_manager_update_ready()
            : updateManager.isDownloading
              ? m.ui_download_manager_downloading_update()
              : updateManager.isApplying
                ? m.ui_download_manager_restart_to_update()
                : m.shared_ui_update_available({
                    version: updateManager.updateResult?.latest_version ?? "",
                  })
          : showSolverSetup
            ? solverSetup?.state === "preparing"
              ? m.ai_ui_solver_download_in_progress()
              : solverSetup?.state === "failed"
                ? m.ai_ui_solver_download_failed()
                : m.ai_ui_solver_download_ready()
            : m.ui_shell_status_no_operations())),
);

function getOperationStatusLabel(status: string) {
  if (status === "completed") return m.ui_shell_status_completed();
  if (status === "failed") return m.ui_shell_status_failed();
  if (status === "interrupted") return m.ui_shell_status_interrupted();
  return m.ui_shell_status_running();
}

function scheduleSolverReadyDismissal() {
  if (solverReadyTimer) clearTimeout(solverReadyTimer);
  solverReadyTimer = setTimeout(() => {
    if (solverSetup?.state === "ready") solverSetup = null;
  }, 8000);
}

onMount(() => {
  void updateManager.initialize();
  let disposed = false;
  const jobPoll = window.setInterval(() => {
    if (disposed) return;
    for (const operation of operationStore.operations) {
      if (
        operation.kind !== "bulk-import" ||
        !operation.jobId ||
        operation.status === "completed" ||
        operation.status === "failed"
      ) {
        continue;
      }
      const jobRequest =
        operation.jobProfile === "employees"
          ? api.getBulkRegisterEmployeeJob(operation.jobId)
          : api.getBulkRegisterJob(operation.jobId);
      void jobRequest
        .then((job) => {
          if (!job) return;
          if (operation.jobProfile === "employees") {
            operationStore.syncEmployeeJob(
              operation.id,
              job as BulkRegisterEmployeeJobDto,
            );
          } else {
            operationStore.syncJob(operation.id, job as BulkRegisterJobDto);
          }
        })
        .catch((error) => {
          console.error("Failed to sync bulk import job", error);
        });
    }
  }, 750);
  if (!isDesktopApp) {
    return () => {
      disposed = true;
      window.clearInterval(jobPoll);
      window.clearInterval(jobPoll);
    };
  }

  const listenerPromise = listen<SolverSetupStatusDto>(
    "solver-setup-status-changed",
    ({ payload }) => {
      solverSetup = payload;
      if (payload.state === "ready") scheduleSolverReadyDismissal();
    },
  );

  void listenerPromise.then(async (unlisten) => {
    if (disposed) {
      unlisten();
      return;
    }
    unlistenSolver = unlisten;
    try {
      solverSetup = await api.getSolverSetupStatus();
      if (solverSetup.state === "ready") scheduleSolverReadyDismissal();
    } catch (error) {
      console.error("Failed to load solver setup status", error);
      solverSetup = { state: "failed", error: getErrorMessage(error) };
    }
  });

  return () => {
    disposed = true;
    unlistenSolver?.();
    if (solverReadyTimer) clearTimeout(solverReadyTimer);
  };
});

function dismissUpdate() {
  const version = updateManager.updateResult?.latest_version;
  if (!version) return;
  dismissedUpdateVersion = version;
  try {
    window.localStorage.setItem(DISMISSED_UPDATE_VERSION_KEY, version);
  } catch (error) {
    console.warn("Could not save dismissed update version", error);
  }
}

async function downloadUpdate() {
  isUpdateActionRunning = true;
  try {
    await updateManager.downloadUpdate();
  } catch (error) {
    console.error("Failed to download update", error);
  } finally {
    isUpdateActionRunning = false;
  }
}

async function installUpdate() {
  isUpdateActionRunning = true;
  try {
    await updateManager.installUpdate();
  } catch (error) {
    console.error("Failed to install update", error);
  } finally {
    isUpdateActionRunning = false;
  }
}

async function retryUpdate() {
  isUpdateActionRunning = true;
  try {
    await updateManager.retry();
  } catch (error) {
    console.error("Failed to retry update", error);
  } finally {
    isUpdateActionRunning = false;
  }
}

async function retrySolverSetup() {
  isSolverActionRunning = true;
  try {
    solverSetup = await api.startSolverSetup();
  } catch (error) {
    console.error("Failed to restart solver setup", error);
    solverSetup = { state: "failed", error: getErrorMessage(error) };
  } finally {
    isSolverActionRunning = false;
  }
}
</script>

<footer
  class="flex h-7 min-h-7 w-full shrink-0 items-center justify-between border-t border-border/70 bg-background text-[11px] text-muted-foreground select-none"
  aria-label={m.ui_shell_status_bar()}
>
  <div class="flex min-w-0 flex-1 items-center overflow-hidden">
    <div
      class="flex h-7 min-w-0 max-w-[42vw] shrink items-center gap-1.5 border-e border-border/60 px-2.5"
      title={orgUnitsQuery.error
        ? `${organizationName} · ${String(orgUnitsQuery.error)}`
        : `${organizationName} · ${branchName}`}
    >
      {#if orgUnitsQuery.isError}
        <XMark size={13} class="shrink-0 text-destructive" />
      {:else}
        <BuildingOffice size={13} class="shrink-0" />
      {/if}
      <span class="hidden truncate lg:inline">{organizationName}</span>
      <span class="truncate">{branchName}</span>
    </div>

    {#if academicContextLabel}
      <div
        class="hidden h-7 min-w-0 max-w-[32vw] shrink items-center gap-1.5 border-e border-border/60 px-2.5 sm:flex"
        title={`${m.ui_shell_status_academic_context()}: ${academicContextLabel}`}
      >
        <AcademicCap size={13} class="shrink-0" />
        <span class="truncate">{academicContextLabel}</span>
      </div>
    {/if}

    {#if licenseStore.licenseState}
      <Popover bind:open={licenseOpen}>
        <PopoverTrigger>
          {#snippet child({ props }: { props: any })}
            <button
              {...props}
              type="button"
              class="flex h-7 min-w-0 max-w-[30vw] items-center gap-1.5 border-e border-border/60 px-2.5 transition-colors hover:bg-accent hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring {isLicenseUrgent
                ? 'bg-destructive/10 text-destructive'
                : shouldShowLicenseAlert && !isLicenseAlertDismissed
                  ? 'bg-amber-500/10 text-amber-700 dark:text-amber-400'
                  : ''}"
              title={`${m.license_status_title()}: ${licenseStatusLabel}`}
              aria-label={`${m.license_status_title()}: ${licenseStatusSummary}`}
            >
              {#if licenseStore.isLoading}
                <ArrowPath size={13} class="shrink-0 animate-spin" />
              {:else if isLicenseUrgent}
                <XMark size={13} class="shrink-0" />
              {:else}
                <Lock size={13} class="shrink-0" />
              {/if}
              <span class="hidden truncate md:inline">
                {licenseStatusSummary}
              </span>
            </button>
          {/snippet}
        </PopoverTrigger>
        <PopoverContent
          side="top"
          align="start"
          sideOffset={6}
          class="w-[min(360px,calc(100dvw-1rem))] rounded-md border border-border bg-card p-0 text-foreground shadow-md"
        >
          <div class="flex items-start gap-2.5 border-b border-border px-3 py-2.5">
            {#if isLicenseUrgent}
              <XMark size={16} class="mt-0.5 shrink-0 text-destructive" />
            {:else}
              <Lock size={16} class="mt-0.5 shrink-0 text-muted-foreground" />
            {/if}
            <div class="min-w-0 flex-1">
              <h2 class="text-xs font-semibold">
                {m.license_status_title()}
              </h2>
              <p class="mt-0.5 text-[11px] text-muted-foreground">
                {licenseStatusLabel}
                {#if licenseStore.planName}
                  · {licenseStore.planName}
                {/if}
              </p>
            </div>
          </div>
          <div class="flex flex-col gap-2 px-3 py-3">
            {#if shouldShowLicenseAlert}
              <p
                class="text-xs {isLicenseUrgent
                  ? 'text-destructive'
                  : 'text-amber-700 dark:text-amber-400'}"
              >
                {licenseAlertMessage}
              </p>
            {/if}
            {#if licenseStore.error}
              <p class="break-words text-xs text-destructive">
                {licenseStore.error}
              </p>
            {/if}
            <div class="flex items-center justify-between gap-2">
              {#if shouldShowLicenseAlert ||
                licenseStore.needsActivation ||
                licenseStore.isRestricted}
                <Button
                  variant={isLicenseUrgent ? "destructive" : "outline"}
                  size="sm"
                  class="h-7 px-2.5 text-[11px] font-semibold"
                  onclick={() => {
                    licenseOpen = false;
                    uiStore.setActivationDialog(true);
                  }}
                >
                  {m.license_banner_activate()}
                </Button>
              {/if}
              {#if shouldShowLicenseAlert &&
                !isLicenseUrgent &&
                !isLicenseAlertDismissed}
                <Button
                  variant="ghost"
                  size="icon-xs"
                  title={m.common_action_close()}
                  aria-label={m.common_action_close()}
                  onclick={() => (isLicenseAlertDismissed = true)}
                >
                  <XMark size={14} />
                </Button>
              {/if}
            </div>
          </div>
        </PopoverContent>
      </Popover>
    {/if}

    <Popover bind:open={operationOpen}>
      <PopoverTrigger>
        {#snippet child({ props }: { props: any })}
          <button
            {...props}
            type="button"
            class="flex h-7 min-w-0 max-w-[42vw] items-center gap-1.5 border-e border-border/60 px-2.5 transition-colors hover:bg-accent hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
            class:text-destructive={hasAttention}
            title={m.ui_shell_status_operations()}
            aria-label={`${m.ui_shell_status_operations()}: ${operationLabel}`}
          >
            {#if hasAttention}
              <XMark size={13} class="shrink-0" />
            {:else if
              activeOperation ||
              visiblePromotionJob?.status === "executing" ||
              updateManager.isDownloading ||
              updateManager.isApplying ||
              solverSetup?.state === "preparing"}
              <ArrowPath size={13} class="shrink-0 animate-spin" />
            {:else if showUpdate || showSolverSetup}
              <ArrowPath size={13} class="shrink-0" />
            {:else}
              <CheckCircle size={13} class="shrink-0" />
            {/if}
            <span class="hidden truncate md:inline">
              {activeOperation
                ? `${activeOperation.title} · ${activeOperation.total > 0 ? Math.round((activeOperation.processed / activeOperation.total) * 100) : 0}%`
                : visiblePromotionJob?.name ?? operationLabel}
            </span>
            {#if hasOperations}
              <span class="shrink-0 rounded-sm bg-background/70 px-1 tabular-nums">
                {visibleOperations.length +
                  Number(!!visiblePromotionJob) +
                  Number(showUpdate) +
                  Number(showSolverSetup)}
              </span>
            {/if}
          </button>
        {/snippet}
      </PopoverTrigger>

      <PopoverContent
        side="top"
        align="start"
        sideOffset={6}
        class="flex max-h-[min(24rem,calc(100dvh-3rem))] w-[min(360px,calc(100dvw-1rem))] flex-col gap-0 overflow-y-auto rounded-md border border-border bg-card p-0 text-foreground shadow-md"
      >
        <h2 class="border-b border-border px-3 py-2 text-xs font-semibold">
          {m.ui_shell_status_operations()}
        </h2>
        {#if !hasOperations}
          <p class="px-3 py-4 text-xs text-muted-foreground">
            {m.ui_shell_status_no_operations()}
          </p>
        {:else}
          <div class="flex flex-col divide-y divide-border/60">
            {#each visibleOperations as operation (operation.id)}
              {@const result = operationStore.resultFor(operation.id)}
              <div
                class="flex min-w-0 items-start gap-2.5 px-3 py-2.5"
                role="group"
                aria-label={`${operation.title}: ${getOperationStatusLabel(operation.status)}`}
              >
                {#if operation.status === "running"}
                  <ArrowPath
                    size={15}
                    class="mt-0.5 shrink-0 animate-spin text-primary"
                  />
                {:else if operation.status === "completed"}
                  <CheckCircle
                    size={15}
                    class="mt-0.5 shrink-0 text-emerald-600 dark:text-emerald-400"
                  />
                {:else if operation.status === "interrupted"}
                  <XMark
                    size={15}
                    class="mt-0.5 shrink-0 text-amber-600 dark:text-amber-400"
                  />
                {:else}
                  <XMark size={15} class="mt-0.5 shrink-0 text-destructive" />
                {/if}
                <div class="min-w-0 flex-1">
                  <div class="flex min-w-0 items-center justify-between gap-2">
                    <span class="truncate text-xs font-medium">
                      {operation.title}
                    </span>
                    <span
                      class="shrink-0 text-[10px] text-muted-foreground"
                    >
                      {getOperationStatusLabel(operation.status)}
                    </span>
                  </div>
                  {#if operation.status === "running"}
                    {#if operation.processed > 0}
                      <div class="mt-1.5 flex items-center gap-2">
                        <Progress
                          value={operation.processed}
                          max={Math.max(operation.total, 1)}
                          class="h-1 flex-1"
                          aria-label={m.ui_shell_status_progress()}
                        />
                        <span class="shrink-0 tabular-nums text-[10px] text-muted-foreground">
                          {operation.total > 0
                            ? Math.round((operation.processed / operation.total) * 100)
                            : 0}%
                        </span>
                      </div>
                    {:else}
                      <div class="mt-1.5 h-1 overflow-hidden rounded-full bg-muted">
                        <div class="h-full w-1/3 rounded-full bg-primary/70 animate-pulse"></div>
                      </div>
                    {/if}
                    <p class="mt-1 text-[10px] text-muted-foreground">
                      {operation.processed > 0
                        ? `${operation.processed} / ${operation.total}`
                        : m.ui_shell_status_running()}
                    </p>
                  {:else}
                    <div
                      class="mt-1 flex items-center justify-between gap-2 text-[10px] text-muted-foreground"
                    >
                      <span class="tabular-nums">
                        {operation.committed} / {operation.total}
                      </span>
                      {#if operation.failed > 0}
                        <span class="text-destructive">
                          {operation.failed}
                        </span>
                      {/if}
                    </div>
                    {#if operation.status === "interrupted"}
                      <p class="mt-1.5 text-[10px] leading-4 text-amber-700 dark:text-amber-300">
                        {m.ui_shell_status_interrupted_after_refresh()}
                      </p>
                    {/if}
                  {/if}
                </div>
                <div class="flex shrink-0 items-center gap-1">
                  {#if result}
                    <Button
                      variant="outline"
                      size="sm"
                      class="h-6 px-2 text-[10px]"
                      onclick={() => {
                        operationOpen = false;
                        selectedImportResult = result;
                      }}
                    >
                      {m.bulk_entry_view_results()}
                    </Button>
                  {/if}
                  {#if operation.status !== "running"}
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    class="shrink-0"
                    title={m.common_action_close()}
                    aria-label={m.common_action_close()}
                    onclick={() => operationStore.dismiss(operation.id)}
                  >
                    <XMark size={14} />
                  </Button>
                  {/if}
                </div>
              </div>
            {/each}
            {#if showUpdate}
              <section
                class="flex min-w-0 items-start gap-2.5 px-3 py-2.5"
                role="status"
                aria-live="polite"
              >
                <div class="mt-0.5 shrink-0">
                  {#if updateManager.isDownloading || updateManager.isApplying}
                    <ArrowPath size={15} class="animate-spin text-primary" />
                  {:else if updateManager.error}
                    <XMark size={15} class="text-destructive" />
                  {:else if updateManager.isReadyToRestart}
                    <CheckCircle size={15} class="text-primary" />
                  {:else}
                    <ArrowPath size={15} class="text-primary" />
                  {/if}
                </div>
                <div class="min-w-0 flex-1">
                  <div class="flex min-w-0 items-center justify-between gap-2">
                    <span class="truncate text-xs font-medium">
                      {updateManager.isReadyToRestart
                        ? m.ui_download_manager_update_ready()
                        : updateManager.isDownloading
                          ? m.ui_download_manager_downloading_update()
                          : updateManager.isApplying
                            ? m.ui_download_manager_restart_to_update()
                            : updateManager.error
                              ? m.ui_download_manager_update_failed()
                              : m.shared_ui_update_available({
                                  version:
                                    updateManager.updateResult
                                      ?.latest_version ?? "",
                                })}
                    </span>
                    {#if updateManager.progress?.total_bytes}
                      <span class="shrink-0 text-[10px] tabular-nums text-muted-foreground">
                        {Math.min(
                          100,
                          Math.round(
                            (updateManager.progress.downloaded_bytes /
                              updateManager.progress.total_bytes) *
                              100,
                          ),
                        )}%
                      </span>
                    {/if}
                  </div>
                  {#if updateManager.error}
                    <p class="mt-1 break-words text-[10px] text-destructive">
                      {getUpdateErrorMessage(updateManager.error)}
                    </p>
                  {/if}
                  {#if updateManager.isDownloading}
                    <Progress
                      value={updateManager.progress?.downloaded_bytes ?? 0}
                      max={Math.max(
                        updateManager.progress?.total_bytes ?? 1,
                        1,
                      )}
                      class="mt-1.5 h-1 w-full"
                      aria-label={m.ui_download_manager_downloading_update()}
                    />
                  {/if}
                </div>
                {#if updateManager.error}
                  <Button
                    variant="outline"
                    size="sm"
                    class="h-6 shrink-0 px-2.5 text-[10px] font-bold"
                    disabled={isUpdateActionRunning}
                    onclick={() => void retryUpdate()}
                  >
                    {m.common_retry()}
                  </Button>
                {:else if updateManager.isReadyToRestart}
                  <Button
                    variant="default"
                    size="sm"
                    class="h-6 shrink-0 px-2.5 text-[10px] font-bold"
                    disabled={isUpdateActionRunning}
                    onclick={() => void installUpdate()}
                  >
                    {m.ui_download_manager_restart_to_update()}
                  </Button>
                {:else if updateManager.isDownloading || updateManager.isApplying}
                  <span class="shrink-0 text-[10px] text-muted-foreground">
                    {m.common_loading()}
                  </span>
                {:else}
                  <Button
                    variant="default"
                    size="sm"
                    class="h-6 shrink-0 px-2.5 text-[10px] font-bold"
                    disabled={isUpdateActionRunning}
                    onclick={() => void downloadUpdate()}
                  >
                    {m.ui_download_manager_download_update()}
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    class="shrink-0"
                    title={m.common_action_cancel()}
                    aria-label={m.common_action_cancel()}
                    onclick={dismissUpdate}
                  >
                    <XMark size={14} />
                  </Button>
                {/if}
              </section>
            {/if}
            {#if showSolverSetup}
              <section
                class="flex min-w-0 items-start gap-2.5 px-3 py-2.5"
                role="status"
                aria-live="polite"
              >
                <div class="mt-0.5 shrink-0">
                  {#if solverSetup?.state === "preparing"}
                    <ArrowPath size={15} class="animate-spin text-primary" />
                  {:else if solverSetup?.state === "failed"}
                    <XMark size={15} class="text-destructive" />
                  {:else}
                    <CheckCircle size={15} class="text-primary" />
                  {/if}
                </div>
                <div class="min-w-0 flex-1">
                  <p class="truncate text-xs font-medium">
                    {solverSetup?.state === "preparing"
                      ? m.ai_ui_solver_download_in_progress()
                      : solverSetup?.state === "failed"
                        ? m.ai_ui_solver_download_failed()
                        : m.ai_ui_solver_download_ready()}
                  </p>
                  {#if solverSetup?.state === "preparing"}
                    <Progress
                      value={0}
                      class="download-progress-indeterminate mt-1.5 h-1 w-full"
                      aria-label={m.ai_ui_solver_download_in_progress()}
                    />
                  {:else if solverSetup?.state === "failed" && solverSetup.error}
                    <p class="mt-1 break-words text-[10px] text-destructive">
                      {solverSetup.error}
                    </p>
                  {/if}
                </div>
                {#if solverSetup?.state === "failed"}
                  <Button
                    variant="outline"
                    size="sm"
                    class="h-6 shrink-0 px-2.5 text-[10px] font-bold"
                    disabled={isSolverActionRunning}
                    onclick={() => void retrySolverSetup()}
                  >
                    {m.common_retry()}
                  </Button>
                {:else if solverSetup?.state === "ready"}
                  <Button
                    variant="ghost"
                    size="sm"
                    class="h-6 shrink-0 px-2.5 text-[10px]"
                    onclick={() => (solverSetup = null)}
                  >
                    {m.common_action_close()}
                  </Button>
                {/if}
              </section>
            {/if}
            {#if visiblePromotionJob}
              {@const processed =
                visiblePromotionJob.promoted_count +
                visiblePromotionJob.retained_count +
                visiblePromotionJob.graduated_count +
                visiblePromotionJob.excluded_count}
              <div class="flex min-w-0 items-start gap-2.5 px-3 py-2.5">
                {#if promotionNeedsAttention}
                  <XMark size={15} class="mt-0.5 shrink-0 text-primary" />
                {:else}
                  <ArrowPath
                    size={15}
                    class="mt-0.5 shrink-0 animate-spin text-primary"
                  />
                {/if}
                <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between gap-2">
                    <span class="min-w-0 truncate text-xs font-medium">
                      {visiblePromotionJob.name}
                    </span>
                    <span class="shrink-0 text-[10px] text-muted-foreground">
                      {promotionNeedsAttention
                        ? m.ui_shell_status_needs_attention()
                        : m.ui_shell_status_running()}
                    </span>
                  </div>
                  <Progress
                    value={processed}
                    max={Math.max(visiblePromotionJob.total_students, 1)}
                    class="mt-1.5 h-1 w-full"
                    aria-label={visiblePromotionJob.name}
                  />
                  <p class="mt-1 text-[10px] tabular-nums text-muted-foreground">
                    {processed} / {visiblePromotionJob.total_students}
                  </p>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </PopoverContent>
    </Popover>
  </div>

  <SystemClock />
</footer>

{#if selectedImportResult}
  {#await import("$lib/features/bulk-entry/components/import-result-dialog.svelte") then { default: ImportResultDialog }}
    <ImportResultDialog
      open={true}
      result={selectedImportResult}
      onclose={() => (selectedImportResult = null)}
    />
  {/await}
{/if}
