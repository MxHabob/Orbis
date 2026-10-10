<!--
  @component ApprovalsManager
  Next-Generation 2026 ERP Unified Academic & Admission Approvals Center.
  Manages Student Admission Approvals, Capacity Overrides, Grade Modifications,
  Prerequisite Bypasses, and Multi-Step Approval Workflows directly via backend API.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { authStore, workflowInbox } from "$lib/stores";
import { toast } from "@orbis/ui";
import { formatDate, l } from "$lib/core/i18n";
import {
  createPendingApprovalsQuery,
  createOverrideRequestsQuery,
} from "$lib/features/workflow/queries";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  Card,
  CardContent,
} from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { Textarea } from "@orbis/ui/textarea";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import { Skeleton } from "@orbis/ui/skeleton";
import { Separator } from "@orbis/ui/separator";
import {
  AdjustmentsHorizontal,
  ArrowPath,
  ArrowUpCircle,
  CheckCircle,
  CheckSquare,
  ClockRotateRight,
  DocumentCheck,
  Funnel,
  InfoCircle,
  Lock,
  Sparkles,
  Tag,
  User,
  UserBadgeCheck,
  XMark,
} from "@orbis/icons";

import { effectiveContext } from "$lib/stores/effective-context.svelte";

let { orgId }: { orgId: string } = $props();

const userRole = $derived(effectiveContext.effectiveRole || "admin");
const userId = $derived(authStore.userId);

// TanStack Queries
const queryClient = useQueryClient();

const pendingQuery = createOverrideRequestsQuery(
  () => orgId,
  () => "pending",
);

const allRequestsQuery = createOverrideRequestsQuery(
  () => orgId,
  () => "all",
);

const pendingItems = $derived(
  (pendingQuery.data ?? []) as any[],
);
const allRequests = $derived(
  (allRequestsQuery.data ?? []) as any[],
);
const myRequests = $derived(
  allRequests.filter((r) => String(r.requested_by) === String(userId)),
);
const logItems = $derived(
  allRequests.filter((r) => r.status !== "pending"),
);

// Active Tab & Filter Category
let activeTab = $state<"pending" | "admission" | "overrides" | "mine" | "log">("pending");
let typeFilter = $state<string>("all");
let searchQuery = $state("");

// Processing state
let processingId = $state<string | null>(null);
let actionNotes = $state<Record<string, string>>({});

// Filtered pending items by category/type & search query
const filteredPendingItems = $derived.by(() => {
  let list = pendingItems;

  if (activeTab === "admission") {
    list = list.filter((r) => r.override_type === "admission_approval");
  } else if (activeTab === "overrides") {
    list = list.filter((r) => r.override_type !== "admission_approval");
  } else if (typeFilter !== "all") {
    list = list.filter((r) => r.override_type === typeFilter);
  }

  if (searchQuery.trim()) {
    const q = searchQuery.trim().toLowerCase();
    list = list.filter((r) =>
      (r.reason || "").toLowerCase().includes(q) ||
      (r.requested_by_name || "").toLowerCase().includes(q) ||
      (r.override_type || "").toLowerCase().includes(q)
    );
  }

  return list;
});

// Counts for quick metrics
const admissionCount = $derived(
  pendingItems.filter((r) => r.override_type === "admission_approval").length
);
const overrideCount = $derived(
  pendingItems.filter((r) => r.override_type !== "admission_approval").length
);

async function processApproval(
  requestId: string,
  action: "approve" | "reject" | "escalate",
) {
  processingId = requestId;
  const notes = actionNotes[requestId]?.trim() || "";

  try {
    if (action === "approve") {
      await api.approveOverride({
        request_id: requestId,
        notes: notes || undefined,
      });
      toast.success(m.ui_copy_request_approved_successfully_0d97b0bc());
    } else if (action === "reject") {
      await api.rejectOverride({
        request_id: requestId,
        reason: notes || "Rejected by administrator",
      });
      toast.error(m.ui_copy_request_rejected_789c8c19());
    } else {
      toast.info(m.ui_copy_request_escalated_4dc69ee1());
    }

    // Clear note for this request
    actionNotes[requestId] = "";
    workflowInbox.decrementApprovals();

    // Invalidate TanStack queries
    queryClient.invalidateQueries({ queryKey: ["pending-approvals"] });
    queryClient.invalidateQueries({ queryKey: ["override-requests"] });
    queryClient.invalidateQueries({ queryKey: ["student"] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    processingId = null;
  }
}

async function cancelRequest(requestId: string) {
  processingId = requestId;
  try {
    await api.rejectOverride({ request_id: requestId, reason: "Cancelled by requester" });
    toast.success(m.ui_copy_request_cancelled_75ffa047());
    queryClient.invalidateQueries({ queryKey: ["override-requests"] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    processingId = null;
  }
}

function getOverrideTypeInfo(typeStr?: string) {
  switch (typeStr) {
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
        label: m.settings_ui_approval_unknown_type({ type: (typeStr || "override request").replace(/_/g, " ") }),
        icon: CheckSquare,
        variant: "outline" as const,
      };
  }
}

function riskBadgeVariant(level?: string | null) {
  switch (level) {
    case "low":
      return "secondary" as const;
    case "medium":
      return "default" as const;
    case "high":
    case "critical":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
}

function statusBadgeVariant(status: string) {
  switch (status) {
    case "approved":
      return "lime" as const;
    case "rejected":
      return "destructive" as const;
    case "pending":
      return "warning" as const;
    case "escalated":
      return "default" as const;
    default:
      return "outline" as const;
  }
}

function parseApprovalChain(chain: any): any[] {
  if (!chain) return [];
  if (Array.isArray(chain)) return chain;
  try {
    return JSON.parse(chain) || [];
  } catch {
    return [];
  }
}
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background text-start">
  <Tabs
    value={activeTab}
    onValueChange={(v) => (activeTab = v as typeof activeTab)}
    class="flex min-h-0 flex-1 flex-col overflow-hidden"
  >
    <header class="shrink-0 border-b border-border/50 bg-card/40 px-4 py-3 backdrop-blur-sm">
      <div class="flex flex-col gap-3">
        <div class="flex flex-wrap items-center gap-2">
          <Badge variant={admissionCount > 0 ? "lime" : "outline"} class="px-2 py-0.5 text-[10px] font-mono font-semibold">
            {admissionCount} {m.ui_copy_pending_admissions_7b6cea59()}
          </Badge>
          <Badge variant={overrideCount > 0 ? "warning" : "outline"} class="px-2 py-0.5 text-[10px] font-mono font-semibold">
            {overrideCount} {m.ui_copy_policy_overrides_e552bc54()}
          </Badge>
        </div>

        <div class="flex min-w-0 flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
          <div class="min-w-0 overflow-x-auto scrollbar-hide">
            <TabsList class="h-9 min-w-max justify-start gap-1 rounded-lg border border-border/60 bg-muted/40 p-1">
          <TabsTrigger value="pending" class="h-7 gap-1.5 px-3 text-xs font-semibold">
            <ClockRotateRight class="w-3.5 h-3.5" />
            <span>{m.ui_copy_all_pending_a89e9229()}</span>
            {#if pendingItems.length > 0}
              <Badge variant="destructive" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
                {pendingItems.length}
              </Badge>
            {/if}
          </TabsTrigger>

          <TabsTrigger value="admission" class="h-7 gap-1.5 px-3 text-xs font-semibold">
            <UserBadgeCheck class="w-3.5 h-3.5" />
            <span>{m.ui_copy_admissions_6dbbec27()}</span>
            {#if admissionCount > 0}
              <Badge variant="lime" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
                {admissionCount}
              </Badge>
            {/if}
          </TabsTrigger>

          <TabsTrigger value="overrides" class="h-7 gap-1.5 px-3 text-xs font-semibold">
            <AdjustmentsHorizontal class="w-3.5 h-3.5" />
            <span>{m.ui_copy_policy_overrides_9ec38c4a()}</span>
            {#if overrideCount > 0}
              <Badge variant="secondary" class="ms-1 px-1.5 py-0 text-[10px] font-mono">
                {overrideCount}
              </Badge>
            {/if}
          </TabsTrigger>

          <TabsTrigger value="mine" class="h-7 gap-1.5 px-3 text-xs font-semibold">
            <User class="w-3.5 h-3.5" />
            <span>{m.ui_copy_my_requests_4a1dba46()}</span>
          </TabsTrigger>

          <TabsTrigger value="log" class="h-7 gap-1.5 px-3 text-xs font-semibold">
            <CheckCircle class="w-3.5 h-3.5" />
            <span>{m.ui_copy_audit_log_97a36552()}</span>
          </TabsTrigger>
            </TabsList>
          </div>

          <div class="w-full sm:w-64 sm:shrink-0">
            <Input
              placeholder={m.ui_copy_search_requests_a2e9521f()}
              bind:value={searchQuery}
              class="h-8 border-border/60 bg-background text-xs"
            />
          </div>
        </div>
      </div>
    </header>

    <main class="min-h-0 flex-1 overflow-y-auto p-4 scrollbar-hide sm:p-6">
      <!-- TAB 1 & 2 & 3: Pending / Admission / Overrides Content -->
      {#if activeTab === "pending" || activeTab === "admission" || activeTab === "overrides"}
        <div class="space-y-3">
          {#if pendingQuery.isLoading}
            <div class="flex flex-col gap-3">
              {#each Array(3) as _}
                <Skeleton class="h-36 w-full rounded-lg" />
              {/each}
            </div>
          {:else if filteredPendingItems.length === 0}
            <Card class="border-border/60 py-16 text-center">
              <CardContent class="flex flex-col items-center justify-center gap-3">
                <div class="w-12 h-12 rounded-full bg-muted/40 border border-border/60 flex items-center justify-center text-muted-foreground">
                  <CheckCircle class="w-6 h-6 text-emerald-500/80" />
                </div>
                <div class="flex flex-col gap-1 max-w-sm">
                  <span class="text-xs font-bold text-foreground">
                    {m.ui_copy_no_pending_approvals_eee02c96()}
                  </span>
                </div>
              </CardContent>
            </Card>
          {:else}
            <div class="flex flex-col gap-3">
              {#each filteredPendingItems as request (request.id)}
                {@const typeInfo = getOverrideTypeInfo(request.override_type)}
                {@const chain = parseApprovalChain(request.approval_chain)}
                {@const isProcessing = processingId === request.id}

                <Card class="border-border/60 shadow-xs transition-colors hover:border-primary/30">
                  <CardContent class="flex flex-col gap-3 p-4">
                    
                    <!-- Header Row: Type Badge, Risk, and Submission Timestamp -->
                    <div class="flex items-start justify-between gap-3 flex-wrap">
                      <div class="flex items-center gap-2 flex-wrap">
                        <Badge variant={typeInfo.variant} class="text-xs font-bold gap-1 px-2.5 py-0.5">
                          {#if typeInfo.icon}
                            {@const Icon = typeInfo.icon}
                            <Icon class="w-3.5 h-3.5" />
                          {/if}
                          <span>{typeInfo.label}</span>
                        </Badge>

                        {#if request.risk_level}
                          <Badge variant={riskBadgeVariant(request.risk_level)} class="text-[10px] font-mono uppercase">
                            {request.risk_level}
                          </Badge>
                        {/if}

                        <Badge variant="outline" class="text-[10px] font-mono text-muted-foreground">
                          {request.entity_type}: {String(request.entity_id).slice(0, 8)}...
                        </Badge>
                      </div>

                      <span class="text-[11px] text-muted-foreground font-mono">
                        {formatDate(request.created_at)}
                      </span>
                    </div>

                    <!-- Request Description & Target Context -->
                    <div class="flex flex-col gap-2 text-start">
                      <p class="text-sm font-semibold text-foreground leading-snug">
                        {request.reason}
                      </p>

                      <div class="flex items-center gap-2 text-xs text-muted-foreground">
                        <span>{m.ui_copy_requested_by_a2679cef()}</span>
                        <strong class="text-foreground font-medium">{request.requested_by_name || request.requested_by || m.settings_ui_system_administrator()}</strong>
                      </div>

                      <!-- Changed Field Values Diff Matrix -->
                      {#if request.old_value || request.new_value}
                        <div class="flex items-center gap-2 mt-1 text-xs bg-muted/30 border border-border/50 rounded-lg p-2.5 font-mono">
                          {#if request.field_name}
                            <span class="text-muted-foreground font-sans font-medium">{request.field_name}:</span>
                          {/if}
                          {#if request.old_value}
                            <span class="line-through text-muted-foreground">{request.old_value}</span>
                            <span class="text-muted-foreground">→</span>
                          {/if}
                          {#if request.new_value}
                            <span class="font-bold text-emerald-600 dark:text-emerald-400">{request.new_value}</span>
                          {/if}
                        </div>
                      {/if}
                    </div>

                    <!-- Approval Chain Progress Stepper -->
                    {#if chain.length > 0}
                      <div class="flex items-center gap-2 text-[11px] pt-1">
                        {#each chain as step, idx}
                          <div class="flex items-center gap-1">
                            {#if step.status === "approved"}
                              <CheckCircle class="w-3.5 h-3.5 text-emerald-500" />
                            {:else if step.status === "rejected"}
                              <XMark class="w-3.5 h-3.5 text-destructive" />
                            {:else}
                              <ClockRotateRight class="w-3.5 h-3.5 text-amber-500" />
                            {/if}
                            <span class="font-semibold text-foreground capitalize">{step.approver_role || step.role}</span>
                          </div>
                          {#if idx < chain.length - 1}
                            <span class="text-muted-foreground">→</span>
                          {/if}
                        {/each}
                      </div>
                    {/if}

                    <Separator class="my-0.5" />

                    <!-- Action Inputs & Decision Buttons -->
                    <div class="flex flex-col items-stretch justify-between gap-3 pt-1 sm:flex-row sm:items-center">
                      <div class="flex-1">
                        <Input
                          placeholder={m.ui_copy_approval_notes_or_decision_reason_optional_a41d972b()}
                          bind:value={actionNotes[request.id]}
                          class="h-8 text-xs bg-background"
                          disabled={isProcessing}
                        />
                      </div>

                      <div class="flex flex-wrap items-center gap-2 sm:shrink-0">
                        <!-- Approve Button -->
                        <Button
                          variant="lime"
                          size="sm"
                          class="h-8 px-4 text-xs font-bold gap-1.5 cursor-pointer"
                          onclick={() => processApproval(request.id, "approve")}
                          disabled={isProcessing}
                        >
                          {#if isProcessing}
                            <LoadingIndicator class="w-3.5 h-3.5"  size={14} />
                            <span>{m.ui_copy_processing_94974c77()}</span>
                          {:else}
                            <CheckCircle class="w-3.5 h-3.5" />
                            <span>{m.ui_copy_approve_7da54ae0()}</span>
                          {/if}
                        </Button>

                        <!-- Reject Button -->
                        <Button
                          variant="destructive"
                          size="sm"
                          class="h-8 px-3.5 text-xs font-bold gap-1.5 cursor-pointer"
                          onclick={() => processApproval(request.id, "reject")}
                          disabled={isProcessing}
                        >
                          <XMark class="w-3.5 h-3.5" />
                          <span>{m.common_approval_reject()}</span>
                        </Button>

                        <!-- Escalate Button -->
                        <Button
                          variant="outline"
                          size="sm"
                          class="h-8 px-3 text-xs font-semibold gap-1 cursor-pointer bg-background"
                          onclick={() => processApproval(request.id, "escalate")}
                          disabled={isProcessing}
                        >
                          <ArrowUpCircle class="w-3.5 h-3.5 text-muted-foreground" />
                          <span>{m.common_approval_escalate()}</span>
                        </Button>
                      </div>
                    </div>

                  </CardContent>
                </Card>
              {/each}
            </div>
          {/if}
        </div>
      {/if}

      <!-- TAB 4: My Requests -->
      <TabsContent value="mine" class="space-y-3 pt-3">
        {#if allRequestsQuery.isLoading}
          <div class="flex flex-col gap-3">
            {#each Array(3) as _}
              <Skeleton class="h-28 w-full rounded-lg" />
            {/each}
          </div>
        {:else if myRequests.length === 0}
          <Card class="border-border/60 py-16 text-center">
            <CardContent class="flex flex-col items-center justify-center gap-3">
              <User class="w-10 h-10 text-muted-foreground/40" />
              <p class="text-xs text-muted-foreground">{m.ui_copy_you_have_not_submitted_any_approval_reques_005f0b0e()}</p>
            </CardContent>
          </Card>
        {:else}
          <div class="flex flex-col gap-3">
            {#each myRequests as request (request.id)}
              {@const typeInfo = getOverrideTypeInfo(request.override_type)}
              <Card class="border-border/80 shadow-xs">
                <CardContent class="p-4 flex flex-col gap-2 text-start">
                  <div class="flex items-center justify-between gap-2">
                    <div class="flex items-center gap-2">
                      <Badge variant={typeInfo.variant} class="text-xs font-bold">
                        {typeInfo.label}
                      </Badge>
                      <Badge variant={statusBadgeVariant(request.status)} class="text-[10px] font-bold uppercase">
                        {request.status}
                      </Badge>
                    </div>
                    <span class="text-[10px] text-muted-foreground font-mono">{formatDate(request.created_at)}</span>
                  </div>
                  <p class="text-xs font-medium text-foreground">{request.reason}</p>
                  {#if request.resolution_notes}
                    <div class="p-2 rounded bg-muted/30 border border-border/50 text-[11px] text-muted-foreground">
                      <strong>{m.ui_copy_resolution_371c8da4()}</strong> {request.resolution_notes}
                    </div>
                  {/if}
                </CardContent>
              </Card>
            {/each}
          </div>
        {/if}
      </TabsContent>

      <!-- TAB 5: Audit Log -->
      <TabsContent value="log" class="space-y-3 pt-3">
        {#if allRequestsQuery.isLoading}
          <div class="flex flex-col gap-3">
            {#each Array(3) as _}
              <Skeleton class="h-20 w-full rounded-lg" />
            {/each}
          </div>
        {:else if logItems.length === 0}
          <Card class="border-border/60 py-16 text-center">
            <CardContent class="flex flex-col items-center justify-center gap-3">
              <ClockRotateRight class="w-10 h-10 text-muted-foreground/40" />
              <p class="text-xs text-muted-foreground">{m.ui_copy_no_approval_history_recorded_cd3090ac()}</p>
            </CardContent>
          </Card>
        {:else}
          <div class="flex flex-col gap-2">
            {#each logItems as request (request.id)}
              {@const typeInfo = getOverrideTypeInfo(request.override_type)}
              <div class="flex items-center justify-between gap-3 p-3 rounded-lg border border-border/60 bg-card text-xs text-start">
                <div class="flex items-center gap-2 min-w-0">
                  <Badge variant={statusBadgeVariant(request.status)} class="text-[10px] font-bold uppercase shrink-0">
                    {request.status}
                  </Badge>
                  <Badge variant="outline" class="text-[10px] shrink-0">
                    {typeInfo.label}
                  </Badge>
                  <span class="truncate font-medium text-foreground">{request.reason}</span>
                </div>
                <div class="flex items-center gap-3 shrink-0 text-[10px] text-muted-foreground font-mono">
                  <span>{request.requested_by_name || m.settings_ui_system()}</span>
                  <span>{formatDate(request.created_at)}</span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </TabsContent>

    </main>
  </Tabs>
</div>
