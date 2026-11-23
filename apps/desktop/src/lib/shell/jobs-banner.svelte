<script lang="ts">
import { onMount } from "svelte";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { authStore } from "$lib/stores";
import { operationStore } from "$lib/stores";
import { createprogressionJobsQuery } from "$lib/features/academics/promotion-queries";
import { updateManager } from "$lib/features/system/update-manager.svelte";
import { api, isDesktopApp } from "$lib/api";
import type { SolverSetupStatusDto } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";
import { Progress } from "@orbis/ui/progress";
import { Button } from "@orbis/ui/button";
import {
  AppNotification,
  ArrowPath,
  CheckCircle,
  XMark,
} from "@orbis/icons";
import { fly } from "svelte/transition";

const orgId = $derived(authStore.orgId);
const progressionJobsQuery = createprogressionJobsQuery(() => orgId);
const progressionJobs = $derived(progressionJobsQuery.data ?? []);
let solverSetup = $state<SolverSetupStatusDto | null>(null);
let unlistenSolver: UnlistenFn | undefined;
let solverReadyTimer: ReturnType<typeof setTimeout> | undefined;
let isUpdateActionRunning = $state(false);
let isSolverActionRunning = $state(false);
let isCollapsed = $state(false);

const COLLAPSED_PREFERENCE_KEY = "orbis-operation-notifications-collapsed";
const DISMISSED_UPDATE_VERSION_KEY = "orbis-dismissed-update-version";

function readLocalPreference(key: string) {
  if (typeof window === "undefined") return null;
  try {
    return window.localStorage.getItem(key);
  } catch (error) {
    console.warn(`Could not read local preference: ${key}`, error);
    return null;
  }
}

function saveLocalPreference(key: string, value: string) {
  try {
    window.localStorage.setItem(key, value);
  } catch (error) {
    console.warn(`Could not save local preference: ${key}`, error);
  }
}

function getErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

let dismissedUpdateVersion = $state(
  readLocalPreference(DISMISSED_UPDATE_VERSION_KEY),
);
const bulkOperations = $derived(
  operationStore.operations.filter(
    (operation) =>
      operation.status === "running" ||
      operation.status === "interrupted" ||
      operation.status === "failed" ||
      operation.status === "completed",
  ),
);

const activeProgressionJob = $derived(
  progressionJobs.find((job) =>
    ["draft", "preview_ready", "executing"].includes(job.status),
  ) ?? null,
);

const visibleJob = $derived(
  activeProgressionJob
    ? {
        id: activeProgressionJob.id,
        title: activeProgressionJob.name,
        status: activeProgressionJob.status,
        processed: activeProgressionJob.promoted_count +
          activeProgressionJob.retained_count +
          activeProgressionJob.graduated_count +
          activeProgressionJob.excluded_count,
        total: activeProgressionJob.total_students,
        failed: 0,
      }
    : null,
);

const showUpdate = $derived.by(() => {
  const latestVersion = updateManager.updateResult?.latest_version;
  const isOperationVisible =
    updateManager.isDownloading ||
    updateManager.isApplying ||
    updateManager.isReadyToRestart ||
    !!updateManager.error ||
    !!updateManager.updateResult?.has_update;
  return isOperationVisible && dismissedUpdateVersion !== latestVersion;
});

const showSolverSetup = $derived(
  !!solverSetup &&
    ["preparing", "failed", "ready"].includes(solverSetup.state),
);
const notificationCount = $derived(
  bulkOperations.length +
    Number(showUpdate) +
    Number(showSolverSetup) +
    Number(!!visibleJob),
);

onMount(() => {
  isCollapsed = readLocalPreference(COLLAPSED_PREFERENCE_KEY) === "true";

  void updateManager.initialize();

  if (!isDesktopApp) return;

  let disposed = false;
  const listenerPromise = listen<SolverSetupStatusDto>(
    "solver-setup-status-changed",
    ({ payload }) => {
      solverSetup = payload;
      if (payload.state === "ready") {
        if (solverReadyTimer) clearTimeout(solverReadyTimer);
        solverReadyTimer = setTimeout(() => {
          if (solverSetup?.state === "ready") solverSetup = null;
        }, 8000);
      }
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
      if (solverSetup.state === "ready") {
        if (solverReadyTimer) clearTimeout(solverReadyTimer);
        solverReadyTimer = setTimeout(() => {
          if (solverSetup?.state === "ready") solverSetup = null;
        }, 8000);
      }
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

function collapseNotifications() {
  if (isCollapsed) return;
  isCollapsed = true;
  saveLocalPreference(COLLAPSED_PREFERENCE_KEY, "true");
}

function expandNotifications() {
  if (!isCollapsed) return;
  isCollapsed = false;
  saveLocalPreference(COLLAPSED_PREFERENCE_KEY, "false");
}

function handleCollapseKeydown(event: KeyboardEvent) {
  if (event.key !== "Enter" && event.key !== " ") return;
  event.preventDefault();
  collapseNotifications();
}

function notificationGesture(node: HTMLElement) {
  let pointerStart: { id: number; x: number; y: number } | null = null;

  function onPointerDown(event: PointerEvent) {
    if (
      !event.isPrimary ||
      (event.pointerType === "mouse" && event.button !== 0) ||
      (event.target instanceof Element &&
        event.target.closest("button, a, input, select, textarea"))
    ) {
      pointerStart = null;
      return;
    }

    pointerStart = {
      id: event.pointerId,
      x: event.clientX,
      y: event.clientY,
    };
    node.setPointerCapture(event.pointerId);
  }

  function onPointerUp(event: PointerEvent) {
    if (!pointerStart || event.pointerId !== pointerStart.id) return;
    const deltaX = event.clientX - pointerStart.x;
    const deltaY = event.clientY - pointerStart.y;
    pointerStart = null;

    const isSwipe =
      Math.abs(deltaX) >= 70 && Math.abs(deltaX) > Math.abs(deltaY);
    const isClick =
      Math.hypot(deltaX, deltaY) < 8 &&
      !(event.target instanceof Element &&
        event.target.closest('[role="button"]'));
    if (isSwipe || isClick) collapseNotifications();
  }

  function clearPointer() {
    pointerStart = null;
  }

  node.addEventListener("pointerdown", onPointerDown);
  node.addEventListener("pointerup", onPointerUp);
  node.addEventListener("pointercancel", clearPointer);
  return {
    destroy() {
      node.removeEventListener("pointerdown", onPointerDown);
      node.removeEventListener("pointerup", onPointerUp);
      node.removeEventListener("pointercancel", clearPointer);
    },
  };
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

function dismissUpdate() {
  if (!updateManager.updateResult?.latest_version) return;
  dismissedUpdateVersion = updateManager.updateResult.latest_version;
  saveLocalPreference(
    DISMISSED_UPDATE_VERSION_KEY,
    updateManager.updateResult.latest_version,
  );
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

{#if notificationCount > 0 && isCollapsed}
  <div class="pointer-events-none fixed inset-e-0 bottom-3 z-[60]">
    <button
      type="button"
      class="pointer-events-auto relative flex size-10 cursor-pointer appearance-none items-center justify-center rounded-md border border-border/80 bg-card/95 text-foreground shadow-lg backdrop-blur transition-[translate,transform,background-color,box-shadow] duration-200 ease-out ltr:translate-x-7 rtl:-translate-x-7 ltr:hover:translate-x-0 rtl:hover:translate-x-0 ltr:focus-visible:translate-x-0 rtl:focus-visible:translate-x-0 hover:scale-105 hover:bg-accent hover:shadow-xl active:scale-95 focus-visible:scale-105 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      onclick={expandNotifications}
      aria-label={m.common_nav_notifications()}
      title={m.common_nav_notifications()}
      aria-expanded="false"
      aria-controls="operation-notifications"
    >
      <AppNotification size={18} />
      <span
        class="absolute -top-1 -inset-s-1 flex h-5 min-w-5 items-center justify-center rounded-full border-2 border-card bg-primary px-1 text-[10px] font-bold leading-none text-primary-foreground shadow-sm"
      >
        {notificationCount > 99 ? "99+" : notificationCount}
      </span>
    </button>
  </div>
{/if}

<div
  id="operation-notifications"
  class:hidden={isCollapsed || notificationCount === 0}
  class="pointer-events-none fixed inset-e-3 bottom-3 z-[60] flex max-h-[calc(100dvh-1.5rem)] w-[min(320px,calc(100vw-1.5rem))] flex-col gap-1.5 overflow-y-auto"
  aria-label={m.common_nav_notifications()}
>
{#each bulkOperations as operation, index (operation.id)}
  {@const percentage = operation.total > 0
    ? Math.min(100, Math.round((operation.processed / operation.total) * 100))
    : 0}
  <section
    in:fly={{ y: 16, opacity: 0, duration: 180 }}
    out:fly={{ y: 8, opacity: 0, duration: 120 }}
    class="pointer-events-auto flex w-full items-start gap-2 rounded-md border border-border bg-card p-2.5 text-foreground shadow-md"
    role="status"
    aria-live="polite"
    style="touch-action: pan-y"
    use:notificationGesture
  >
    <div class="flex size-5 shrink-0 items-center justify-center">
      {#if operation.status === "completed"}
        <CheckCircle class="size-4 text-primary" />
      {:else if operation.status === "failed" || operation.status === "interrupted"}
        <XMark class="size-4 text-destructive" />
      {:else}
        <ArrowPath class="size-4 animate-spin text-primary" />
      {/if}
    </div>

    <div class="min-w-0 flex-1">
      <div
        class="flex cursor-pointer items-start justify-between gap-2 rounded-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        role="button"
        tabindex="0"
        aria-label={m.common_action_collapse()}
        onclick={collapseNotifications}
        onkeydown={handleCollapseKeydown}
      >
        <p class="min-w-0 truncate text-xs font-semibold">
          {operation.title}
          {#if operation.status === "interrupted"}
            — العملية لم تكتمل بعد إعادة فتح التطبيق
          {:else if operation.status === "completed"}
            — اكتملت
          {:else if operation.status === "failed"}
            — تعذر إكمال العملية
          {/if}
        </p>
        <p class="shrink-0 text-xs tabular-nums text-muted-foreground">
          {operation.processed} / {operation.total}
        </p>
      </div>
      {#if operation.status === "running"}
        <Progress
          value={operation.processed}
          max={Math.max(operation.total, 1)}
          class="mt-2 h-1.5 w-full"
          aria-label={operation.title}
        />
      {:else}
        <Progress
          value={percentage}
          class="mt-2 h-1.5 w-full"
          aria-label={operation.title}
        />
      {/if}
    </div>

    {#if operation.failed > 0}
      <span class="shrink-0 rounded-md bg-destructive/10 px-2 py-1 text-xs text-destructive">
        {operation.failed}
      </span>
    {/if}
    {#if operation.status !== "running"}
      <Button
        variant="ghost"
        size="icon-xs"
        title={m.common_action_close()}
        aria-label={m.common_action_close()}
        onclick={() => operationStore.dismiss(operation.id)}
      >
        <XMark class="size-4" />
      </Button>
    {/if}
  </section>
{/each}

{#if showUpdate}
  <section
    in:fly={{ y: 16, opacity: 0, duration: 180 }}
    out:fly={{ y: 8, opacity: 0, duration: 120 }}
    class="pointer-events-auto flex w-full items-start gap-2 rounded-md border border-border bg-card p-2.5 text-foreground shadow-md"
    role="status"
    aria-live="polite"
    style="touch-action: pan-y"
    use:notificationGesture
  >
    <div class="flex size-5 shrink-0 items-center justify-center">
      {#if updateManager.isDownloading || updateManager.isApplying}
        <ArrowPath class="size-4 animate-spin text-primary" />
      {:else if updateManager.error}
        <XMark class="size-4 text-destructive" />
      {:else if updateManager.isReadyToRestart}
        <CheckCircle class="size-4 text-primary" />
      {:else}
        <ArrowPath class="size-4 text-primary" />
      {/if}
    </div>

    <div class="min-w-0 flex-1">
      <div
        class="flex cursor-pointer items-start justify-between gap-2 rounded-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        role="button"
        tabindex="0"
        aria-label={m.common_action_collapse()}
        onclick={collapseNotifications}
        onkeydown={handleCollapseKeydown}
      >
        <p class="min-w-0 truncate text-xs font-semibold">
          {updateManager.isReadyToRestart
            ? m.ui_download_manager_update_ready()
            : updateManager.isDownloading
              ? m.ui_download_manager_downloading_update()
              : updateManager.isApplying
                ? m.ui_download_manager_restart_to_update()
                : updateManager.error
                  ? m.ui_download_manager_update_failed()
                  : m.shared_ui_update_available({
                      version: updateManager.updateResult?.latest_version ?? "",
                    })}
        </p>
        {#if updateManager.progress?.total_bytes}
          <p class="shrink-0 text-xs tabular-nums text-muted-foreground">
            {Math.min(
              100,
              Math.round(
                (updateManager.progress.downloaded_bytes /
                  updateManager.progress.total_bytes) *
                  100,
              ),
            )}%
          </p>
        {/if}
      </div>

      {#if updateManager.isDownloading}
        <Progress
          value={updateManager.progress?.downloaded_bytes ?? 0}
          max={Math.max(updateManager.progress?.total_bytes ?? 1, 1)}
          class="mt-2 h-1.5 w-full"
          aria-label={m.ui_download_manager_downloading_update()}
        />
      {/if}
    </div>

    {#if updateManager.error}
      <Button
        variant="outline"
        size="sm"
        class="h-6 px-2.5 text-[10px] font-bold"
        disabled={isUpdateActionRunning}
        onclick={() => void retryUpdate()}
      >
        {m.common_retry()}
      </Button>
    {:else if updateManager.isReadyToRestart}
      <Button
        variant="default"
        size="sm"
        class="h-6 px-2.5 text-[10px] font-bold"
        disabled={isUpdateActionRunning}
        onclick={() => void installUpdate()}
      >
        {m.ui_download_manager_restart_to_update()}
      </Button>
    {:else if updateManager.isDownloading || updateManager.isApplying}
      <span class="shrink-0 text-xs text-muted-foreground">
        {m.common_loading()}
      </span>
    {:else}
      <Button
        variant="default"
        size="sm"
        class="h-6 px-2.5 text-[10px] font-bold"
        disabled={isUpdateActionRunning}
        onclick={() => void downloadUpdate()}
      >
        {m.ui_download_manager_download_update()}
      </Button>
      <Button
        variant="ghost"
        size="icon-xs"
        title={m.common_action_cancel()}
        aria-label={m.common_action_cancel()}
        onclick={dismissUpdate}
      >
        <XMark class="size-4" />
      </Button>
    {/if}
  </section>
{/if}

{#if solverSetup && ["preparing", "failed", "ready"].includes(solverSetup.state)}
  <section
    in:fly={{ y: 16, opacity: 0, duration: 180 }}
    out:fly={{ y: 8, opacity: 0, duration: 120 }}
    class="pointer-events-auto flex w-full items-start gap-2 rounded-md border border-border bg-card p-2.5 text-foreground shadow-md"
    role="status"
    aria-live="polite"
    style="touch-action: pan-y"
    use:notificationGesture
  >
    <div class="flex size-5 shrink-0 items-center justify-center">
      {#if solverSetup.state === "preparing"}
        <ArrowPath class="size-4 animate-spin text-primary" />
      {:else if solverSetup.state === "failed"}
        <XMark class="size-4 text-destructive" />
      {:else}
        <CheckCircle class="size-4 text-primary" />
      {/if}
    </div>

    <div class="min-w-0 flex-1">
      <div
        class="flex cursor-pointer items-start justify-between gap-2 rounded-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        role="button"
        tabindex="0"
        aria-label={m.common_action_collapse()}
        onclick={collapseNotifications}
        onkeydown={handleCollapseKeydown}
      >
        <p class="min-w-0 truncate text-[11px] font-medium">
        {solverSetup.state === "preparing"
          ? m.ai_ui_solver_download_in_progress()
          : solverSetup.state === "failed"
            ? m.ai_ui_solver_download_failed()
            : m.ai_ui_solver_download_ready()}
        </p>
      </div>
      {#if solverSetup.state === "preparing"}
        <Progress
          value={0}
          class="download-progress-indeterminate mt-2 h-1.5 w-full"
          aria-label={m.ai_ui_solver_download_in_progress()}
        />
      {:else if solverSetup.state === "failed" && solverSetup.error}
        <p class="mt-1 truncate text-xs text-destructive">{solverSetup.error}</p>
      {/if}
    </div>

    {#if solverSetup.state === "failed"}
      <Button
        variant="outline"
        size="sm"
        class="h-6 px-2.5 text-[10px] font-bold"
        disabled={isSolverActionRunning}
        onclick={() => void retrySolverSetup()}
      >
        {m.common_retry()}
      </Button>
    {:else if solverSetup.state === "ready"}
      <Button
        variant="ghost"
        size="sm"
        class="h-6 px-2.5 text-[10px]"
        onclick={() => (solverSetup = null)}
      >
        {m.common_action_close()}
      </Button>
    {/if}
  </section>
{/if}

{#if visibleJob}
  <section
    in:fly={{ y: 16, opacity: 0, duration: 180 }}
    out:fly={{ y: 8, opacity: 0, duration: 120 }}
    class="pointer-events-auto flex w-full items-start gap-2 rounded-md border border-border bg-card p-2.5 text-foreground shadow-md"
    role="status"
    aria-live="polite"
    style="touch-action: pan-y"
    use:notificationGesture
  >
    <div class="flex size-5 shrink-0 items-center justify-center">
      {#if visibleJob.status === "completed"}
        <CheckCircle class="size-4 text-primary" />
      {:else if visibleJob.status === "failed"}
        <XMark class="size-4 text-destructive" />
      {:else}
        <ArrowPath class="size-4 animate-spin text-primary" />
      {/if}
    </div>

    <div class="min-w-0 flex-1">
      <div
        class="flex cursor-pointer items-start justify-between gap-2 rounded-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        role="button"
        tabindex="0"
        aria-label={m.common_action_collapse()}
        onclick={collapseNotifications}
        onkeydown={handleCollapseKeydown}
      >
        <p class="min-w-0 truncate text-xs font-semibold">{visibleJob.title}</p>
        <p class="shrink-0 text-xs tabular-nums text-muted-foreground">
          {visibleJob.processed} / {visibleJob.total}
        </p>
      </div>
      <Progress
        value={visibleJob.processed}
        max={Math.max(visibleJob.total, 1)}
        class="mt-2 h-1.5 w-full"
        aria-label={visibleJob.title}
      />
    </div>

    {#if visibleJob.failed > 0}
      <span class="shrink-0 rounded-md bg-destructive/10 px-2 py-1 text-xs text-destructive">
        {visibleJob.failed}
      </span>
    {/if}
  </section>
{/if}
</div>
