<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import * as common from "$lib/paraglide/messages/common";
import {
  createAgentSessionLogsQuery,
  useApproveAgentAction,
  useRejectAgentAction,
} from "$lib/features/ai/queries";
import { toast } from "svelte-sonner";
import { extractErrorMessage } from "$lib/core/ipc";

let {
  orgId = "",
  sessionId = "",
  onBack,
}: {
  orgId: string;
  sessionId: string;
  onBack: () => void;
} = $props();

const logsQuery = createAgentSessionLogsQuery(
  () => orgId,
  () => sessionId,
);
const logs = $derived($logsQuery.data ?? []);

const approveActionMutation = useApproveAgentAction();
const rejectActionMutation = useRejectAgentAction();

async function handleApprove(actionId: string) {
  try {
    toast.promise(
      $approveActionMutation.mutateAsync({
        orgId,
        actionLogId: actionId,
        approvedBy: "Admin",
      }),
      {
        loading: common.agent_toast_rejecting(),
        success: common.agent_toast_reject_success(),
        error: (e) => extractErrorMessage(e) || common.agent_toast_reject_failed(),
      },
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleReject(actionId: string) {
  try {
    toast.promise(
      $rejectActionMutation.mutateAsync({
        orgId,
        actionLogId: actionId,
        rejectedBy: "Admin",
      }),
      {
        loading: common.agent_toast_rejecting(),
        success: common.agent_toast_reject_success(),
        error: (e) => extractErrorMessage(e) || common.agent_toast_reject_failed(),
      },
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}
</script>

<div class="px-4 py-2 border-b border-border bg-muted/20 flex items-center justify-between shrink-0 select-none">
  <Button 
    variant="ghost" 
    size="xs" 
    class="text-[10px] h-7 font-semibold cursor-pointer"
    onclick={onBack}
  >
    ← {common.action_back()}
  </Button>
  <span class="text-xs font-mono font-bold text-primary flex items-center gap-1">
    #{sessionId.substring(0, 8)}
    {#if logs.some(l => l.status === 'pending')}
      <span class="w-1.5 h-1.5 rounded-full bg-destructive inline-block animate-pulse"></span>
    {/if}
  </span>
</div>

<div class="flex-1 overflow-y-auto p-4 space-y-3 min-h-0 select-text scrollbar-hide bg-background/50">
  {#if $logsQuery.isLoading}
    <div class="space-y-3">
      <Skeleton class="h-20 w-full rounded-lg" />
      <Skeleton class="h-20 w-full rounded-lg" />
    </div>
  {:else if $logsQuery.isError}
    <div class="border border-destructive/20 bg-destructive/5 rounded-lg p-4 text-center space-y-2">
      <p class="text-xs font-semibold text-destructive">
        {common.agent_session_logs_load_failed()}
      </p>
    </div>
  {:else if logs.length === 0}
    <div class="text-center py-8 text-xs text-muted-foreground">
      {common.agent_session_logs_empty()}
    </div>
  {:else}
    <div class="space-y-3 text-start">
      {#each logs as log}
        <div class="border border-border/80 rounded-lg p-3 space-y-2 bg-card text-foreground">
          <div class="flex items-center justify-between w-full">
            <span class="text-xs font-bold font-mono">{log.action_code}</span>
            <div class="flex items-center gap-1.5">
              <Badge 
                variant={log.risk_level === 'critical' || log.risk_level === 'high' ? 'destructive' : log.risk_level === 'medium' ? 'secondary' : 'outline'}
                class="text-[8px] px-1"
              >
                {log.risk_level}
              </Badge>
              {#if log.status === "pending"}
                <Badge variant="secondary" class="text-[8px] px-1 bg-amber-500/10 text-amber-600 dark:text-amber-400 border-none">
                  {common.approvals_tab_pending()}
                </Badge>
              {:else if log.status === "approved" || log.status === "executed"}
                <Badge variant="lime" class="text-[8px] px-1">
                  {common.approval_approved()}
                </Badge>
              {:else if log.status === "rejected"}
                <Badge variant="outline" class="text-[8px] px-1 text-destructive border-destructive/20">
                  {common.approval_rejected()}
                </Badge>
              {:else}
                <Badge variant="outline" class="text-[8px] px-1 text-muted-foreground">{log.status}</Badge>
              {/if}
            </div>
          </div>

          <p class="text-[10px] text-muted-foreground leading-normal">
            <strong class="text-foreground">{common.agent_log_reasoning()}</strong> {log.reasoning}
          </p>

          {#if log.input_data}
            <div class="text-[9px] font-mono bg-muted/40 p-1.5 rounded max-h-16 overflow-y-auto overflow-x-hidden break-all text-muted-foreground">
              {log.input_data}
            </div>
          {/if}

          {#if log.status === "pending"}
            <div class="flex gap-2 pt-1">
              <Button
                variant="default"
                size="xs"
                class="flex-1 text-[10px] h-7 cursor-pointer"
                onclick={() => handleApprove(log.id)}
              >
                {common.agent_log_action_approve()}
              </Button>
              <Button
                variant="destructive"
                size="xs"
                class="flex-1 text-[10px] h-7 cursor-pointer"
                onclick={() => handleReject(log.id)}
              >
                {common.agent_log_action_reject()}
              </Button>
            </div>
          {:else if log.status === "rejected" && log.rejection_reason}
            <div class="text-[9px] text-destructive bg-destructive/5 p-1.5 rounded border border-destructive/10">
              <strong>{common.agent_log_rejection_reason()}</strong> {log.rejection_reason}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>
