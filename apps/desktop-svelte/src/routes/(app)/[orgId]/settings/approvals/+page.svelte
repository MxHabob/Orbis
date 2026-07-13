<!--
@component ApprovalsPage
Unified approval inbox for override requests.
Three tabs: Pending (requires action), My Requests (submitted by me), Log (history).
Uses TanStack Query for data, supports approve/reject/escalate actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { authStore, workflowInbox } from "$lib/stores";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import {
  createPendingApprovalsQuery,
  createOverrideRequestsQuery,
} from "$lib/features/workflow/queries";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Textarea } from "@orbis/ui-svelte/textarea";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Separator } from "@orbis/ui-svelte/separator";
import {
  CheckSquare,
  Shield,
  CheckCircle2,
  XCircle,
  ArrowUpCircle,
  Clock,
  AlertTriangle,
  Loader2,
} from "lucide-svelte";
import type { AcademicOverrideRequest, ApprovalAction } from "@orbis/sdk-core";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data?.orgId ?? "");
const userRole = $derived(authStore.user?.role ?? "viewer");
const userId = $derived(authStore.userId);

// TanStack Queries
const queryClient = useQueryClient();
const pendingQuery = createPendingApprovalsQuery(() => orgId, () => userRole);
const myRequestsQuery = createOverrideRequestsQuery(() => orgId, () => ({
  requested_by: userId,
}));
const logQuery = createOverrideRequestsQuery(() => orgId, () => ({}));

const pendingItems = $derived(
  ($pendingQuery.data ?? []) as AcademicOverrideRequest[],
);
const myRequests = $derived(
  ($myRequestsQuery.data ?? []) as AcademicOverrideRequest[],
);
const logItems = $derived(($logQuery.data ?? []) as AcademicOverrideRequest[]);

// Action state
let processingId = $state<string | null>(null);
let actionNotes = $state("");

async function processApproval(
  requestId: string,
  action: "approve" | "reject" | "escalate",
) {
  processingId = requestId;
  try {
    const approvalAction: ApprovalAction = {
      action,
      user_id: userId,
      user_name: authStore.user?.full_name ?? null,
      user_role: userRole,
      notes: actionNotes.trim() || null,
    };
    await commands.platform.processApproval(requestId, approvalAction);

    const actionLabel =
      action === "approve"
        ? m.approval_approved
          ? m.approval_approved()
          : "Approved"
        : action === "reject"
          ? m.approval_rejected
            ? m.approval_rejected()
            : "Rejected"
          : m.approval_escalated
            ? m.approval_escalated()
            : "Escalated";

    toast.success(actionLabel);
    actionNotes = "";
    workflowInbox.decrementApprovals();

    // Invalidate all approval queries
    queryClient.invalidateQueries({ queryKey: ["pending-approvals"] });
    queryClient.invalidateQueries({ queryKey: ["override-requests"] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    processingId = null;
  }
}

async function cancelRequest(requestId: string) {
  processingId = requestId;
  try {
    await commands.platform.cancelOverride(requestId, userId);
    toast.success(
      m.approval_cancelled ? m.approval_cancelled() : "Request cancelled",
    );
    queryClient.invalidateQueries({ queryKey: ["override-requests"] });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    processingId = null;
  }
}

function riskBadgeVariant(level?: string | null) {
  switch (level) {
    case "low":
      return "secondary" as const;
    case "medium":
      return "default" as const;
    case "high":
      return "destructive" as const;
    case "critical":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
}

function statusBadgeVariant(status: string) {
  switch (status) {
    case "approved":
      return "success" as const;
    case "rejected":
      return "destructive" as const;
    case "pending":
      return "secondary" as const;
    case "escalated":
      return "brand" as const;
    default:
      return "outline" as const;
  }
}

function parseApprovalChain(
  chainJson?: string | null,
): Array<{ role: string; status: string; by: string; at: string }> {
  try {
    if (!chainJson) return [];
    return JSON.parse(chainJson) || [];
  } catch {
    return [];
  }
}
</script>

<svelte:head>
  <title>{m.settings_nav_approvals ? m.settings_nav_approvals() : "Approvals"}</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <!-- Header -->
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-10 rounded-xl bg-primary/10">
        <CheckSquare class="size-5 text-primary" />
      </div>
      <div>
        <h1 class="text-lg font-bold text-foreground">
          {m.settings_nav_approvals ? m.settings_nav_approvals() : "Approvals"}
        </h1>
        <p class="text-xs text-muted-foreground">
          {m.approvals_desc ? m.approvals_desc() : "Manage override requests and approval workflows"}
        </p>
      </div>
    </div>

    <!-- Tabs -->
    <Tabs value="pending">
      <TabsList>
        <TabsTrigger value="pending">
          {m.approvals_tab_pending ? m.approvals_tab_pending() : "Pending"}
          {#if pendingItems.length > 0}
            <Badge variant="destructive" class="text-[10px] px-1.5 h-4 ms-1.5">{pendingItems.length}</Badge>
          {/if}
        </TabsTrigger>
        <TabsTrigger value="mine">
          {m.approvals_tab_mine ? m.approvals_tab_mine() : "My Requests"}
        </TabsTrigger>
        <TabsTrigger value="log">
          {m.approvals_tab_log ? m.approvals_tab_log() : "Log"}
        </TabsTrigger>
      </TabsList>

      <!-- Pending Tab -->
      <TabsContent value="pending">
        {#if $pendingQuery.isLoading}
          <div class="flex flex-col gap-3 mt-4">
            {#each Array(3) as _}
              <Skeleton class="h-32 w-full rounded-lg" />
            {/each}
          </div>
        {:else if pendingItems.length === 0}
          <div class="flex flex-col items-center justify-center py-16 text-center gap-3">
            <CheckCircle2 class="size-10 text-muted-foreground/40" />
            <p class="text-sm text-muted-foreground">
              {m.approvals_empty_pending ? m.approvals_empty_pending() : "No pending approvals"}
            </p>
          </div>
        {:else}
          <div class="flex flex-col gap-4 mt-4">
            {#each pendingItems as request (request.id)}
              {@const chain = parseApprovalChain(request.approval_chain)}
              <Card>
                <CardContent class="p-4">
                  <div class="flex flex-col gap-3">
                    <!-- Header row -->
                    <div class="flex items-start justify-between">
                      <div class="flex items-center gap-2">
                        <Badge variant="outline" class="text-[10px] capitalize">
                          {(request.override_type || "").replace(/_/g, " ")}
                        </Badge>
                        <Badge variant={riskBadgeVariant(request.risk_level)} class="text-[10px]">
                          {#if request.risk_level === "high" || request.risk_level === "critical"}
                            <AlertTriangle class="size-3 me-0.5" />
                          {/if}
                          {request.risk_level}
                        </Badge>
                      </div>
                      <span class="text-[10px] text-muted-foreground">
                        {new Date(request.created_at).toLocaleDateString()}
                      </span>
                    </div>

                    <!-- Details -->
                    <div class="text-sm">
                      <p class="font-medium">{request.reason}</p>
                      <p class="text-xs text-muted-foreground mt-1">
                        {m.approvals_requested_by ? m.approvals_requested_by() : "Requested by"}: {request.requested_by_name ?? request.requested_by}
                      </p>
                      {#if request.old_value || request.new_value}
                        <div class="flex items-center gap-2 mt-2 text-xs bg-muted/30 rounded p-2">
                          {#if request.field_name}
                            <span class="text-muted-foreground">{request.field_name}:</span>
                          {/if}
                          {#if request.old_value}
                            <span class="line-through text-muted-foreground">{request.old_value}</span>
                            <span>→</span>
                          {/if}
                          {#if request.new_value}
                            <span class="font-medium">{request.new_value}</span>
                          {/if}
                        </div>
                      {/if}
                    </div>

                    <!-- Approval chain progress -->
                    {#if chain.length > 0}
                      <div class="flex items-center gap-2 text-[10px]">
                        {#each chain as step}
                          <div class="flex items-center gap-1">
                            {#if step.status === "approved"}
                              <CheckCircle2 class="size-3 text-primary" />
                            {:else if step.status === "rejected"}
                              <XCircle class="size-3 text-destructive" />
                            {:else}
                              <Clock class="size-3 text-muted-foreground" />
                            {/if}
                            <span class="capitalize">{step.role}</span>
                          </div>
                          {#if chain.indexOf(step) < chain.length - 1}
                            <span class="text-muted-foreground">→</span>
                          {/if}
                        {/each}
                      </div>
                    {/if}

                    <Separator />

                    <!-- Actions -->
                    <div class="flex items-center gap-2 flex-nowrap shrink-0">
                      <Textarea
                        bind:value={actionNotes}
                        placeholder={m.approvals_notes_placeholder ? m.approvals_notes_placeholder() : "Notes (optional)..."}
                        rows={1}
                        class="flex-1 text-xs"
                      />
                      <Button
                        variant="lime" size="sm"
                        onclick={() => processApproval(request.id, "approve")}
                        disabled={processingId === request.id}
                      >
                        {#if processingId === request.id}
                          <Loader2 class="size-3.5 animate-spin me-1" />
                        {:else}
                          <CheckCircle2 class="size-3.5 me-1" />
                        {/if}
                        {m.approval_approve ? m.approval_approve() : "Approve"}
                      </Button>
                      <Button
                        variant="destructive" size="sm"
                        onclick={() => processApproval(request.id, "reject")}
                        disabled={processingId === request.id}
                      >
                        <XCircle class="size-3.5 me-1" />
                        {m.approval_reject ? m.approval_reject() : "Reject"}
                      </Button>
                      <Button
                        variant="outline" size="sm"
                        onclick={() => processApproval(request.id, "escalate")}
                        disabled={processingId === request.id}
                      >
                        <ArrowUpCircle class="size-3.5 me-1" />
                        {m.approval_escalate ? m.approval_escalate() : "Escalate"}
                      </Button>
                    </div>
                  </div>
                </CardContent>
              </Card>
            {/each}
          </div>
        {/if}
      </TabsContent>

      <!-- My Requests Tab -->
      <TabsContent value="mine">
        {#if $myRequestsQuery.isLoading}
          <div class="flex flex-col gap-3 mt-4">
            {#each Array(3) as _}
              <Skeleton class="h-24 w-full rounded-lg" />
            {/each}
          </div>
        {:else if myRequests.length === 0}
          <div class="flex flex-col items-center justify-center py-16 text-center gap-3">
            <Shield class="size-10 text-muted-foreground/40" />
            <p class="text-sm text-muted-foreground">
              {m.approvals_empty_mine ? m.approvals_empty_mine() : "You haven't submitted any override requests"}
            </p>
          </div>
        {:else}
          <div class="flex flex-col gap-3 mt-4">
            {#each myRequests as request (request.id)}
              <Card>
                <CardContent class="p-4">
                  <div class="flex items-start justify-between">
                    <div class="flex items-center gap-2">
                      <Badge variant="outline" class="text-[10px] capitalize">
                        {request.override_type?.replace(/_/g, " ") ?? "Unknown"}
                      </Badge>
                      <Badge variant={statusBadgeVariant(request.status)} class="text-[10px] capitalize">
                        {request.status}
                      </Badge>
                    </div>
                    <div class="flex items-center gap-2">
                      <span class="text-[10px] text-muted-foreground">
                        {new Date(request.created_at).toLocaleDateString()}
                      </span>
                      {#if request.status === "pending"}
                        <Button
                          variant="ghost" size="icon-xs"
                          onclick={() => cancelRequest(request.id)}
                          disabled={processingId === request.id}
                          class="text-destructive"
                        >
                          <XCircle class="size-3.5" />
                        </Button>
                      {/if}
                    </div>
                  </div>
                  <p class="text-sm mt-2">{request.reason}</p>
                  {#if request.resolution_notes}
                    <p class="text-xs text-muted-foreground mt-1 bg-muted/30 rounded p-2">
                      {request.resolution_notes}
                    </p>
                  {/if}
                </CardContent>
              </Card>
            {/each}
          </div>
        {/if}
      </TabsContent>

      <!-- Log Tab -->
      <TabsContent value="log">
        {#if $logQuery.isLoading}
          <div class="flex flex-col gap-3 mt-4">
            {#each Array(3) as _}
              <Skeleton class="h-16 w-full rounded-lg" />
            {/each}
          </div>
        {:else if logItems.length === 0}
          <div class="flex flex-col items-center justify-center py-16 text-center gap-3">
            <Clock class="size-10 text-muted-foreground/40" />
            <p class="text-sm text-muted-foreground">
              {m.approvals_empty_log ? m.approvals_empty_log() : "No approval history yet"}
            </p>
          </div>
        {:else}
          <div class="flex flex-col gap-2 mt-4">
            {#each logItems as request (request.id)}
              <div class="flex items-center gap-3 p-3 rounded-lg border border-border/60 bg-card text-sm">
                <Badge variant={statusBadgeVariant(request.status)} class="text-[10px] capitalize shrink-0">
                  {request.status}
                </Badge>
                <Badge variant="outline" class="text-[10px] capitalize shrink-0">
                  {request.override_type?.replace(/_/g, " ") ?? "Unknown"}
                </Badge>
                <span class="flex-1 text-xs truncate">{request.reason}</span>
                <span class="text-[10px] text-muted-foreground shrink-0">
                  {request.requested_by_name ?? request.requested_by}
                </span>
                <span class="text-[10px] text-muted-foreground shrink-0">
                  {new Date(request.created_at).toLocaleDateString()}
                </span>
              </div>
            {/each}
          </div>
        {/if}
      </TabsContent>
    </Tabs>
  </div>
</div>
