<script lang="ts">
import { onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import { formatDate } from "$lib/core/i18n";
import { authStore, effectiveContext, workflowInbox } from "$lib/stores";
import { LoadingIndicator, toast } from "@orbis/ui";
import { createOverrideRequestsQuery } from "$lib/features/workflow/queries";
import { useQueryClient } from "@tanstack/svelte-query";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Tabs, TabsList, TabsTrigger } from "@orbis/ui/tabs";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  AdjustmentsHorizontal,
  CheckCircle,
  CheckSquare,
  ClockRotateRight,
  Lock,
  Refresh,
  Search,
  Tag,
  User,
  UserBadgeCheck,
} from "@orbis/icons";
import type { OverrideRequestDto } from "$lib/api/types";
import EmptyState from "$lib/shared/components/empty-state.svelte";
import OverrideRequestCard from "./components/override-request-card.svelte";

type ApprovalTab = "pending" | "mine" | "history";
type PendingFilter = "all" | "admission" | "override";

let {
  orgId,
  initialTab = "pending",
}: {
  orgId: string;
  initialTab?: ApprovalTab;
} = $props();

const queryClient = useQueryClient();
const userId = $derived(authStore.userId);
const canReadAll = $derived(
  effectiveContext.hasPermission("workflow:read") ||
    effectiveContext.hasPermission("workflow:approve") ||
    effectiveContext.hasPermission("workflow:manage") ||
    effectiveContext.hasPermission("*"),
);
const canApprove = $derived(
  effectiveContext.hasPermission("workflow:approve") ||
    effectiveContext.hasPermission("workflow:manage") ||
    effectiveContext.hasPermission("*"),
);
const requestedTab = $derived(
  !canReadAll && initialTab !== "mine" ? "mine" : initialTab,
);
let activeTab = $derived(requestedTab);
let pendingFilter = $state<PendingFilter>("all");
let searchQuery = $state("");
let deferredSearch = $state("");
let searchDebounceTimer: ReturnType<typeof setTimeout> | undefined;
let processingId = $state<string | null>(null);
let selectedId = $state<string | null>(null);
let actionNotes = $state<Record<string, string>>({});

const requestFilter = $derived({
  status:
    activeTab === "pending"
      ? "pending"
      : activeTab === "history"
        ? "history"
        : undefined,
  mine: activeTab === "mine",
  search: deferredSearch || undefined,
});
const requestsQuery = createOverrideRequestsQuery(
  () => orgId,
  () => requestFilter,
);
const requests = $derived(
  requestsQuery.data?.pages.flatMap((page) => page.items) ?? [],
);
const pendingRequests = $derived(
  requests.filter((request) => request.status === "pending"),
);
const historyRequests = $derived(
  requests.filter((request) => request.status !== "pending"),
);
const hasFilters = $derived(
  Boolean(searchQuery.trim()) ||
    (activeTab === "pending" && pendingFilter !== "all"),
);

const visibleRequests = $derived.by(() => {
  let list: OverrideRequestDto[] =
    activeTab === "mine"
      ? requests
      : activeTab === "history"
        ? historyRequests
        : pendingRequests;

  if (activeTab === "pending" && pendingFilter === "admission") {
    list = list.filter(
      (request) => request.override_type === "admission_approval",
    );
  } else if (activeTab === "pending" && pendingFilter === "override") {
    list = list.filter(
      (request) => request.override_type !== "admission_approval",
    );
  }

  return [...list].sort((a, b) => b.created_at.localeCompare(a.created_at));
});

const selectedRequest = $derived(
  visibleRequests.find((r) => r.id === selectedId) ??
    visibleRequests[0] ??
    null,
);

const emptyTitle = $derived(
  hasFilters
    ? m.approvals_no_search_results()
    : activeTab === "mine"
      ? m.ui_copy_you_have_not_submitted_any_approval_reques_005f0b0e()
      : activeTab === "history"
        ? m.ui_copy_no_approval_history_recorded_cd3090ac()
        : m.ui_copy_no_pending_approvals_eee02c96(),
);

const emptyDescription = $derived(
  hasFilters ? m.approvals_no_search_results_description() : "",
);

function handleTabChange(value: string) {
  if (value === "pending" || value === "mine" || value === "history") {
    activeTab = value;
    selectedId = null;
  }
}

onDestroy(() => {
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
});

function handleSearchInput(event: Event) {
  const target = event.currentTarget as HTMLInputElement;
  searchQuery = target.value;
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
  const value = searchQuery.trim();
  if (!value) {
    deferredSearch = "";
    searchDebounceTimer = undefined;
    return;
  }
  searchDebounceTimer = setTimeout(() => {
    deferredSearch = value;
    searchDebounceTimer = undefined;
  }, 250);
}

function fetchNextPage() {
  if (requestsQuery.hasNextPage && !requestsQuery.isFetchingNextPage) {
    void requestsQuery.fetchNextPage();
  }
}

function handleListScroll(event: Event) {
  const element = event.currentTarget;
  if (!(element instanceof HTMLElement)) return;
  if (element.scrollHeight - element.scrollTop - element.clientHeight < 240) {
    fetchNextPage();
  }
}

function canDecide(request: OverrideRequestDto): boolean {
  return (
    canApprove &&
    request.status === "pending" &&
    request.requested_by !== userId
  );
}

function getTypeMeta(type: string) {
  switch (type) {
    case "admission_approval":
      return {
        label: m.settings_ui_approval_admission(),
        icon: UserBadgeCheck,
        variant: "lime" as const,
      };
    case "capacity_override":
      return {
        label: m.settings_ui_approval_capacity_override(),
        icon: User,
        variant: "warning" as const,
      };
    case "grade_change":
      return {
        label: m.settings_ui_approval_locked_grade_change(),
        icon: AdjustmentsHorizontal,
        variant: "default" as const,
      };
    case "prerequisite_bypass":
      return {
        label: m.settings_ui_approval_prerequisite_bypass(),
        icon: Lock,
        variant: "destructive" as const,
      };
    case "late_enrollment":
      return {
        label: m.settings_ui_approval_late_enrollment(),
        icon: ClockRotateRight,
        variant: "outline" as const,
      };
    case "discount_override":
      return {
        label: m.settings_ui_approval_discount_override(),
        icon: Tag,
        variant: "secondary" as const,
      };
    default:
      return {
        label: type.replace(/_/g, " "),
        icon: CheckSquare,
        variant: "outline" as const,
      };
  }
}

function shortRequester(request: OverrideRequestDto): string {
  if (request.requested_by_name) return request.requested_by_name;
  return request.requested_by ? `${request.requested_by.slice(0, 8)}…` : "";
}

async function processDecision(
  request: OverrideRequestDto,
  action: "approve" | "reject",
) {
  if (!canDecide(request) || processingId) return;

  processingId = request.id;
  const notes = actionNotes[request.id]?.trim() || "";

  try {
    if (action === "approve") {
      await api.approveOverride({
        request_id: request.id,
        notes: notes || undefined,
      });
      toast.success(m.ui_copy_request_approved_successfully_0d97b0bc());
    } else {
      await api.rejectOverride({
        request_id: request.id,
        reason: notes || m.approvals_default_rejection_reason(),
      });
      toast.success(m.ui_copy_request_rejected_789c8c19());
    }

    actionNotes[request.id] = "";
    workflowInbox.decrementApprovals();
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all }),
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all }),
    ]);
  } catch (error: unknown) {
    toast.error(extractErrorMessage(error));
  } finally {
    processingId = null;
  }
}
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background font-sans text-start">
  <Tabs
    value={activeTab}
    onValueChange={handleTabChange}
    class="flex min-h-0 flex-1 flex-col overflow-hidden"
  >
    <!-- Full-Width Compact Toolbar -->
    <header class="shrink-0 border-b border-border/60 bg-card/50 px-4 py-2.5 sm:px-5">
      <div class="flex w-full flex-wrap items-center justify-between gap-2.5">
        <div class="flex flex-wrap items-center gap-2 min-w-0">
          <div class="min-w-0 overflow-x-auto scrollbar-hide">
            <TabsList class="h-8 min-w-max justify-start gap-1 rounded-md border border-border/60 bg-muted/50 p-0.5">
              {#if canReadAll}
                <TabsTrigger value="pending" class="h-7 gap-1.5 rounded-sm px-2.5 text-xs font-semibold">
                  <ClockRotateRight size={13} />
                  <span>{m.ui_copy_all_pending_a89e9229()}</span>
                  {#if pendingRequests.length > 0 && activeTab === "pending"}
                    <Badge variant="warning" class="ms-0.5 h-4 rounded-full px-1.5 text-[10px] font-bold">
                      {pendingRequests.length}
                    </Badge>
                  {/if}
                </TabsTrigger>
              {/if}
              <TabsTrigger value="mine" class="h-7 gap-1.5 rounded-sm px-2.5 text-xs font-semibold">
                <User size={13} />
                <span>{m.ui_copy_my_requests_4a1dba46()}</span>
              </TabsTrigger>
              {#if canReadAll}
                <TabsTrigger value="history" class="h-7 gap-1.5 rounded-sm px-2.5 text-xs font-semibold">
                  <CheckCircle size={13} />
                  <span>{m.ui_copy_audit_log_97a36552()}</span>
                </TabsTrigger>
              {/if}
            </TabsList>
          </div>

          {#if activeTab === "pending" && canReadAll}
            <div class="hidden md:flex items-center gap-1 border-s border-border/60 ps-2">
              {#each [
                { id: "all" as const, label: m.approvals_filter_all() },
                { id: "admission" as const, label: m.ui_copy_admissions_6dbbec27() },
                { id: "override" as const, label: m.ui_copy_policy_overrides_9ec38c4a() },
              ] as filterItem (filterItem.id)}
                <Button
                  variant={pendingFilter === filterItem.id ? "secondary" : "ghost"}
                  size="sm"
                  class="h-7 rounded-md px-2.5 text-xs font-medium {pendingFilter === filterItem.id ? 'border border-border/60 shadow-2xs' : 'text-muted-foreground'}"
                  onclick={() => (pendingFilter = filterItem.id)}
                >
                  {filterItem.label}
                </Button>
              {/each}
            </div>
          {/if}
        </div>

        <div class="flex items-center gap-2 ms-auto w-full sm:w-auto">
          <div class="relative flex-1 sm:w-64">
            <Search
              size={14}
              class="pointer-events-none absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground"
            />
            <Input
              placeholder={m.ui_copy_search_requests_a2e9521f()}
              value={searchQuery}
              oninput={handleSearchInput}
              class="h-8 border-border/60 bg-background ps-8 text-xs"
              aria-label={m.ui_copy_search_requests_a2e9521f()}
            />
          </div>
          <Button
            variant="ghost"
            size="icon-sm"
            class="size-8 shrink-0 text-muted-foreground"
            aria-label={m.common_refresh()}
            title={m.common_refresh()}
            disabled={requestsQuery.isFetching}
            onclick={() => void requestsQuery.refetch()}
          >
            {#if requestsQuery.isFetching}
              <LoadingIndicator size={14} />
            {:else}
              <Refresh size={14} />
            {/if}
          </Button>
        </div>
      </div>

      {#if activeTab === "pending" && canReadAll}
        <div class="mt-2 flex flex-wrap items-center gap-1 md:hidden">
          {#each [
            { id: "all" as const, label: m.approvals_filter_all() },
            { id: "admission" as const, label: m.ui_copy_admissions_6dbbec27() },
            { id: "override" as const, label: m.ui_copy_policy_overrides_9ec38c4a() },
          ] as filterItem (filterItem.id)}
            <Button
              variant={pendingFilter === filterItem.id ? "secondary" : "ghost"}
              size="sm"
              class="h-7 rounded-md px-2.5 text-xs font-medium {pendingFilter === filterItem.id ? 'border border-border/60 shadow-2xs' : 'text-muted-foreground'}"
              onclick={() => (pendingFilter = filterItem.id)}
            >
              {filterItem.label}
            </Button>
          {/each}
        </div>
      {/if}
    </header>

    <!-- Full-Screen Master-Detail Split Workspace -->
    <main class="min-h-0 flex-1 overflow-hidden">
      {#if requestsQuery.isLoading}
        <div class="grid h-full lg:grid-cols-[minmax(340px,40%)_1fr]">
          <div class="border-e border-border/60 p-4 space-y-2.5">
            {#each Array(6) as _, index (index)}
              <Skeleton class="h-18 w-full rounded-md" />
            {/each}
          </div>
          <div class="hidden lg:block p-6">
            <Skeleton class="h-64 w-full rounded-md" />
          </div>
        </div>
      {:else if requestsQuery.isError && requests.length === 0}
        <div class="flex h-full items-center justify-center p-6">
          <EmptyState
            icon={ClockRotateRight}
            title={m.approvals_load_error_title()}
            description={m.approvals_load_error_description()}
            actionLabel={m.common_retry()}
            onaction={() => void requestsQuery.refetch()}
          />
        </div>
      {:else if visibleRequests.length === 0}
        <div class="flex h-full items-center justify-center p-6">
          <EmptyState
            icon={activeTab === "mine" ? User : CheckCircle}
            title={emptyTitle}
            description={emptyDescription}
            actionLabel={requestsQuery.hasNextPage ? m.template_load_more() : undefined}
            onaction={requestsQuery.hasNextPage ? fetchNextPage : undefined}
          />
        </div>
      {:else}
        <div class="grid h-full min-h-0 lg:grid-cols-[minmax(340px,40%)_1fr]">
          <!-- Master Queue List -->
          <div
            class="h-full min-h-0 overflow-y-auto border-e border-border/60 bg-card/30 divide-y divide-border/50"
            onscroll={handleListScroll}
          >
            {#each visibleRequests as request (request.id)}
              {@const isSelected = selectedRequest?.id === request.id}
              {@const meta = getTypeMeta(request.override_type)}
              {@const Icon = meta.icon}
              <button
                type="button"
                onclick={() => (selectedId = request.id)}
                class="w-full text-start p-3.5 transition-colors flex flex-col gap-2 cursor-pointer {isSelected
                  ? 'bg-accent/70 border-s-2 border-s-primary'
                  : 'hover:bg-accent/40'}"
              >
                <div class="flex items-center justify-between gap-2">
                  <Badge variant={meta.variant} class="h-5 gap-1 rounded-full px-2 text-[11px] font-semibold">
                    <Icon size={11} />
                    {meta.label}
                  </Badge>
                  <time datetime={request.created_at} class="font-mono text-[11px] text-muted-foreground shrink-0">
                    {formatDate(request.created_at)}
                  </time>
                </div>

                <p class="line-clamp-2 text-xs font-medium text-foreground leading-relaxed">
                  {request.reason || meta.label}
                </p>

                <div class="flex items-center justify-between gap-2 text-[11px] text-muted-foreground">
                  {#if activeTab !== "mine"}
                    <span class="inline-flex items-center gap-1 truncate">
                      <User size={11} class="shrink-0" />
                      <span class="truncate">{shortRequester(request)}</span>
                    </span>
                  {:else}
                    <span></span>
                  {/if}
                  {#if request.approval_chain.length > 0}
                    <span class="truncate font-medium">
                      {request.approval_chain.map((s) => s.approver_name || s.approver_role).join(" › ")}
                    </span>
                  {/if}
                </div>
              </button>
            {/each}

            {#if requestsQuery.isFetchNextPageError}
              <div class="flex justify-center py-3">
                <Button variant="outline" size="sm" onclick={fetchNextPage}>
                  {m.common_retry()}
                </Button>
              </div>
            {/if}
            {#if requestsQuery.isFetchingNextPage}
              <div class="flex justify-center py-3">
                <LoadingIndicator size={16} />
              </div>
            {/if}
          </div>

          <!-- Detail & Decision Inspector Pane -->
          <div class="h-full min-h-0 overflow-y-auto bg-background p-4 sm:p-6">
            {#if selectedRequest}
              {@const isProcessing = processingId === selectedRequest.id}
              {@const canReviewRequest = canDecide(selectedRequest)}
              <OverrideRequestCard
                request={selectedRequest}
                showRequester={activeTab !== "mine"}
                canReview={canReviewRequest}
                processing={isProcessing}
                bind:actionNotes={actionNotes[selectedRequest.id]}
                onapprove={() => void processDecision(selectedRequest, "approve")}
                onreject={() => void processDecision(selectedRequest, "reject")}
              />
            {/if}
          </div>
        </div>
      {/if}
    </main>
  </Tabs>
</div>
