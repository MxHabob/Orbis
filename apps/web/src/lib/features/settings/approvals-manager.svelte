<script lang="ts">
import { onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import { authStore, effectiveContext, workflowInbox } from "$lib/stores";
import { toast } from "@orbis/ui";
import { createOverrideRequestsQuery } from "$lib/features/workflow/queries";
import { useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { Skeleton } from "@orbis/ui/skeleton";
import { ArrowPath, CheckCircle, ClockRotateRight, User } from "@orbis/icons";
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
    list = list.filter((request) => request.override_type === "admission_approval");
  } else if (activeTab === "pending" && pendingFilter === "override") {
    list = list.filter((request) => request.override_type !== "admission_approval");
  }

  return [...list].sort((a, b) => b.created_at.localeCompare(a.created_at));
});

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
  if (
    value === "pending" ||
    value === "mine" ||
    value === "history"
  ) {
    activeTab = value;
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

<div class="flex h-full w-full flex-col overflow-hidden bg-background text-start">
  <Tabs
    value={activeTab}
    onValueChange={handleTabChange}
    class="flex min-h-0 flex-1 flex-col overflow-hidden"
  >
    <header class="shrink-0 space-y-3 border-b border-border/50 bg-card/40 px-3 py-3 sm:px-5">
      <div class="flex min-w-0 flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
        <div class="min-w-0 overflow-x-auto scrollbar-hide">
          <TabsList class="h-9 min-w-max justify-start gap-1 rounded-lg border border-border/60 bg-muted/40 p-1">
            {#if canReadAll}
              <TabsTrigger value="pending" class="h-7 gap-1.5 px-3 text-xs font-semibold">
                <ClockRotateRight size={14} />
                <span>{m.ui_copy_all_pending_a89e9229()}</span>
              </TabsTrigger>
            {/if}
            <TabsTrigger value="mine" class="h-7 gap-1.5 px-3 text-xs font-semibold">
              <User size={14} />
              <span>{m.ui_copy_my_requests_4a1dba46()}</span>
            </TabsTrigger>
            {#if canReadAll}
              <TabsTrigger value="history" class="h-7 gap-1.5 px-3 text-xs font-semibold">
                <CheckCircle size={14} />
                <span>{m.ui_copy_audit_log_97a36552()}</span>
              </TabsTrigger>
            {/if}
          </TabsList>
        </div>
        <div class="flex min-w-0 gap-2 sm:w-80 sm:shrink-0">
          <Input
            placeholder={m.ui_copy_search_requests_a2e9521f()}
            value={searchQuery}
            oninput={handleSearchInput}
            class="h-9 min-w-0 flex-1"
            aria-label={m.ui_copy_search_requests_a2e9521f()}
          />
          <Button
            variant="outline"
            size="icon-sm"
            class="shrink-0"
            aria-label={m.common_refresh()}
            title={m.common_refresh()}
            disabled={requestsQuery.isFetching}
            onclick={() => void requestsQuery.refetch()}
          >
            <ArrowPath
              class={requestsQuery.isFetching ? "animate-spin" : ""}
              size={14}
            />
          </Button>
        </div>
      </div>

      {#if activeTab === "pending" && canReadAll}
        <div class="flex flex-wrap items-center gap-2">
          <Button
            variant={pendingFilter === "all" ? "secondary" : "ghost"}
            size="sm"
            class="h-7 px-3 text-xs"
            onclick={() => (pendingFilter = "all")}
          >
            {m.approvals_filter_all()}
          </Button>
          <Button
            variant={pendingFilter === "admission" ? "secondary" : "ghost"}
            size="sm"
            class="h-7 px-3 text-xs"
            onclick={() => (pendingFilter = "admission")}
          >
            {m.ui_copy_admissions_6dbbec27()}
          </Button>
          <Button
            variant={pendingFilter === "override" ? "secondary" : "ghost"}
            size="sm"
            class="h-7 px-3 text-xs"
            onclick={() => (pendingFilter = "override")}
          >
            {m.ui_copy_policy_overrides_9ec38c4a()}
          </Button>
        </div>
      {/if}
    </header>

    <main
      class="min-h-0 flex-1 overflow-y-auto p-4 scrollbar-hide sm:p-6"
      onscroll={handleListScroll}
    >
      <div class="mx-auto w-full max-w-6xl">
        <TabsContent value={activeTab} class="space-y-3 pt-1">
          {#if requestsQuery.isLoading}
            <div class="flex flex-col gap-3">
              {#each Array(3) as _, index (index)}
                <Skeleton class="h-32 w-full rounded-lg" />
              {/each}
            </div>
          {:else if requestsQuery.isError && requests.length === 0}
            <EmptyState
              icon={ClockRotateRight}
              title={m.approvals_load_error_title()}
              description={m.approvals_load_error_description()}
              actionLabel={m.common_retry()}
              onaction={() => void requestsQuery.refetch()}
            />
          {:else if requestsQuery.isFetchNextPageError && visibleRequests.length === 0}
            <EmptyState
              icon={ClockRotateRight}
              title={m.approvals_load_error_title()}
              description={m.approvals_load_error_description()}
              actionLabel={m.common_retry()}
              onaction={fetchNextPage}
            />
          {:else if visibleRequests.length === 0 && requestsQuery.hasNextPage}
            <EmptyState
              icon={activeTab === "mine" ? User : CheckCircle}
              title={emptyTitle}
              description={emptyDescription}
              actionLabel={m.template_load_more()}
              onaction={fetchNextPage}
            />
          {:else if visibleRequests.length === 0}
            <EmptyState
              icon={activeTab === "mine" ? User : CheckCircle}
              title={emptyTitle}
              description={emptyDescription}
            />
          {:else}
            <div class="flex flex-col gap-3">
              {#each visibleRequests as request (request.id)}
                {@const isProcessing = processingId === request.id}
                {@const canReviewRequest = canDecide(request)}
                <OverrideRequestCard
                  {request}
                  showRequester={activeTab !== "mine"}
                  canReview={canReviewRequest}
                  processing={isProcessing}
                  bind:actionNotes={actionNotes[request.id]}
                  onapprove={() => void processDecision(request, "approve")}
                  onreject={() => void processDecision(request, "reject")}
                />
              {/each}
            </div>
          {/if}
          {#if requestsQuery.isFetchNextPageError && visibleRequests.length > 0}
            <div class="flex justify-center py-4">
              <Button
                variant="outline"
                size="sm"
                onclick={fetchNextPage}
              >
                {m.common_retry()}
              </Button>
            </div>
          {/if}
          {#if requestsQuery.isFetchingNextPage}
            <div class="flex justify-center py-4">
              <ArrowPath size={16} class="animate-spin text-muted-foreground" />
            </div>
          {/if}
        </TabsContent>
      </div>
    </main>
  </Tabs>
</div>
