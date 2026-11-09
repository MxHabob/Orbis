<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { marked } from "marked";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import AiLoadingSkeleton from "./ai-loading-skeleton.svelte";
import {
  Check,
  CheckCircle,
  ChevronDown,
  ChevronRight,
  ChevronUp,
  Clipboard,
  Download,
  InfoCircle,
  Pencil,
  Refresh,
  Sparkles,
  Wrench,
  XMark,
} from "@orbis/icons";
import type {
  AiChatMessage,
  ActionWithStatus,
} from "$lib/features/ai/session-store";
import { fade } from "svelte/transition";

// Configure marked with GFM & breaks
marked.setOptions({
  gfm: true,
  breaks: true,
});

let {
  messages = [],
  isThinking = false,
  userName = "",
  orgName = null,
  chatContainer = $bindable(),
  onLinkClick,
  onActionResponse,
  onReportDownload,
  onRetry,
  onEditUserMessage,
  onApplyTemplateDesign,
}: {
  messages: AiChatMessage[];
  isThinking: boolean;
  userName: string;
  orgName: string | null;
  chatContainer?: HTMLDivElement;
  onLinkClick: (href: string) => void;
  onActionResponse?: (
    action: ActionWithStatus,
    status: "executed" | "rejected",
  ) => void;
  onReportDownload?: (action: ActionWithStatus) => void;
  onRetry?: (msgId: string) => void;
  onEditUserMessage?: (msgId: string, newText: string) => void;
  onApplyTemplateDesign?: (content: string) => void;
} = $props();

let expandedTools = $state<Record<string, boolean>>({});
let expandedUserMessages = $state<Record<string, boolean>>({});
let copiedMessageId = $state<string | null>(null);
let editingMessageId = $state<string | null>(null);
let editingText = $state("");
let showScrollBottom = $state(false);

function feeStatusLabel(status: string) {
  switch (status) {
    case "pending":
      return m.finance_ui_fee_status_unpaid();
    case "partially_paid":
      return m.finance_ui_fee_status_partially_paid();
    case "overdue":
      return m.finance_ui_fee_status_overdue();
    default:
      return status;
  }
}

function handleScroll() {
  if (!chatContainer) return;
  const { scrollTop, scrollHeight, clientHeight } = chatContainer;
  showScrollBottom = scrollHeight - scrollTop - clientHeight > 100;
}

function scrollToBottom() {
  if (!chatContainer) return;
  chatContainer.scrollTo({
    top: chatContainer.scrollHeight,
    behavior: "smooth",
  });
  showScrollBottom = false;
}

function isLongMessage(content: string): boolean {
  if (!content) return false;
  return content.length > 240 || content.split("\n").length > 5;
}

function toggleUserMessageExpand(msgId: string) {
  expandedUserMessages[msgId] = !expandedUserMessages[msgId];
}

function toggleTool(key: string) {
  expandedTools[key] = !expandedTools[key];
}

function getTemplateDesignSummary(content: string): { name: string } | null {
  try {
    const result: unknown = JSON.parse(content);
    if (
      typeof result !== "object" ||
      result === null ||
      !("document" in result) ||
      typeof result.document !== "object" ||
      result.document === null ||
      !("meta" in result.document) ||
      typeof result.document.meta !== "object" ||
      result.document.meta === null ||
      !("name" in result.document.meta) ||
      typeof result.document.meta.name !== "string"
    ) {
      return null;
    }
    return { name: result.document.meta.name };
  } catch {
    return null;
  }
}

async function copyText(text: string, id: string) {
  try {
    await navigator.clipboard.writeText(text);
    copiedMessageId = id;
    setTimeout(() => {
      if (copiedMessageId === id) copiedMessageId = null;
    }, 2000);
  } catch (e) {
    console.error("Failed to copy:", e);
  }
}

function startEditing(msg: AiChatMessage) {
  editingMessageId = msg.id;
  editingText = msg.content;
}

function cancelEditing() {
  editingMessageId = null;
  editingText = "";
}

function submitEdit(msgId: string) {
  const trimmed = editingText.trim();
  if (trimmed && onEditUserMessage) {
    onEditUserMessage(msgId, trimmed);
  }
  editingMessageId = null;
  editingText = "";
}

function renderMarkdown(content: string): string {
  if (!content) return "";
  try {
    return marked.parse(content) as string;
  } catch (e) {
    console.error("Markdown parse error:", e);
    return content;
  }
}

function handleContentClick(e: MouseEvent) {
  const target = (e.target as HTMLElement)?.closest("a");
  if (target && target.getAttribute("href")) {
    const href = target.getAttribute("href") || "";
    e.preventDefault();
    onLinkClick(href);
  }
}
</script>

<div class="relative flex-1 flex flex-col min-h-0 overflow-hidden">
  <div 
    bind:this={chatContainer}
    onscroll={handleScroll}
    class="flex-1 flex flex-col p-3.5 sm:p-4 overflow-y-auto scrollbar-hide select-text space-y-4"
  >
  <div class="flex flex-col gap-4 w-full">
    {#each messages as msg (msg.id)}
      {#if msg.role === 'user'}
        <!-- User message bubble with Hover Actions -->
        <div class="group relative flex flex-col items-end w-full">
          {#if editingMessageId === msg.id}
            <!-- Inline Edit Form -->
            <div class="w-full max-w-[95%] flex flex-col gap-2 bg-card border border-primary/40 rounded-2xl p-3 shadow-md">
              <textarea
                dir="auto"
                bind:value={editingText}
                rows={3}
                class="w-full p-2.5 text-[13px] bg-background border border-border rounded-xl focus:outline-none focus:ring-1 focus:ring-primary resize-none text-foreground font-sans leading-relaxed"
                placeholder={m.ai_ui_edit_message()}
                onkeydown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    submitEdit(msg.id);
                  }
                  if (e.key === 'Escape') {
                    cancelEditing();
                  }
                }}
              ></textarea>
              <div class="flex items-center justify-end gap-2">
                <Button
                  size="xs"
                  variant="ghost"
                  class="text-[11px] h-7 px-2.5 text-muted-foreground hover:bg-muted cursor-pointer"
                  onclick={cancelEditing}
                >
                  <XMark size={12} class="me-1" />
                  {m.ai_ui_cancel()}
                </Button>
                <Button
                  size="xs"
                  variant="default"
                  class="text-[11px] h-7 px-3 bg-primary text-primary-foreground font-semibold cursor-pointer"
                  onclick={() => submitEdit(msg.id)}
                  disabled={!editingText.trim()}
                >
                  {m.ai_ui_save_and_send()}
                </Button>
              </div>
            </div>
          {:else}
            {@const isLong = isLongMessage(msg.content)}
            {@const isExpanded = !!expandedUserMessages[msg.id]}

            <!-- User Text Bubble (Vibrant Gradient / Solid with Expandable Long Message) -->
            <div class="max-w-[88%] bg-primary text-primary-foreground px-4 py-3 rounded-2xl rounded-tr-xs rtl:rounded-tr-2xl rtl:rounded-tl-xs text-[13px] leading-relaxed break-words whitespace-pre-wrap font-medium shadow-2xs relative flex flex-col">
              <!-- Content area with max-height truncation and bottom gradient -->
              <div class="relative transition-all duration-200 {isLong && !isExpanded ? 'max-h-[140px] sm:max-h-[160px] overflow-hidden' : ''}">
                {msg.content}

                {#if isLong && !isExpanded}
                  <!-- Soft bottom gradient fade-out -->
                  <div class="pointer-events-none absolute inset-x-0 bottom-0 h-16 bg-gradient-to-t from-primary via-primary/80 to-transparent"></div>
                {/if}
              </div>

              {#if isLong}
                <!-- Show More / Show Less Toggle Button -->
                <button
                  type="button"
                  onclick={() => toggleUserMessageExpand(msg.id)}
                  class="mt-2 pt-1.5 border-t border-primary-foreground/15 inline-flex items-center gap-1.5 text-xs font-semibold text-primary-foreground/90 hover:text-primary-foreground transition-colors cursor-pointer select-none self-start"
                >
                  <span>{isExpanded ? m.ai_ui_show_less() : m.ai_ui_show_more()}</span>
                  {#if isExpanded}
                    <ChevronUp size={13} />
                  {:else}
                    <ChevronDown size={13} />
                  {/if}
                </button>
              {/if}
            </div>

            <!-- User Hover Action Bar -->
            <div class="flex items-center gap-1 mt-1 opacity-0 group-hover:opacity-100 transition-opacity select-none">
              <!-- Edit Button -->
              <button
                type="button"
                onclick={() => startEditing(msg)}
                class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title={m.ai_ui_edit_message()}
              >
                <Pencil size={12} />
              </button>

              <!-- Copy Button -->
              <button
                type="button"
                onclick={() => copyText(msg.content, msg.id)}
                class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title={m.ai_ui_copy_text()}
              >
                {#if copiedMessageId === msg.id}
                  <Check size={12} class="text-emerald-500" />
                {:else}
                  <Clipboard size={12} />
                {/if}
              </button>
            </div>
          {/if}
        </div>
      {:else}
        <!-- Assistant response bubble -->
        <div class="group relative flex flex-col gap-2.5 w-full text-start text-[13px] text-foreground/90 leading-relaxed font-normal">
          
          <!-- Executed Tools Badges (Modern Sleek Agent Accordion) -->
          {#if msg.executedTools && msg.executedTools.length > 0}
            <div class="flex flex-col gap-1.5 w-full">
              {#each msg.executedTools as tool, tIdx (`${msg.id}_tool_${tIdx}`)}
                {@const toolKey = `${msg.id}_tool_${tIdx}`}
                {@const isExpanded = !!expandedTools[toolKey]}
                <div class="border border-border/60 rounded-xl bg-card/60 hover:bg-card/90 overflow-hidden text-xs transition-colors shadow-2xs">
                  <button
                    type="button"
                    onclick={() => toggleTool(toolKey)}
                    class="w-full px-3 py-1.5 flex items-center justify-between hover:bg-muted/60 transition-colors cursor-pointer text-start select-none"
                  >
                    <div class="flex items-center gap-2 min-w-0">
                      <div class="w-2 h-2 rounded-full {tool.is_error ? 'bg-destructive ring-2 ring-destructive/20' : 'bg-emerald-500 ring-2 ring-emerald-500/20'} shrink-0"></div>
                      <span class="font-mono font-medium text-[11px] text-foreground/80 truncate">{tool.tool_name}</span>
                      <span class="text-[9px] px-1.5 py-0.2 rounded-full font-medium {tool.is_error ? 'bg-destructive/15 text-destructive' : 'bg-emerald-500/15 text-emerald-600 dark:text-emerald-400'}">
                        {tool.is_error ? m.ai_ui_error_status() : m.ai_ui_completed_status()}
                      </span>
                    </div>
                    <div class="text-muted-foreground/70 shrink-0">
                      {#if isExpanded}
                        <ChevronDown size={12} />
                      {:else}
                        <ChevronRight size={12} />
                      {/if}
                    </div>
                  </button>

                  {#if isExpanded}
                    <div class="p-2.5 border-t border-border/40 bg-background/90 font-mono text-[10.5px] leading-relaxed max-h-48 overflow-y-auto whitespace-pre-wrap break-words text-foreground/80 scrollbar-thin">
                      {tool.content}
                    </div>
                  {/if}
                </div>
                {#if tool.tool_name === "design_template" && !tool.is_error}
                  {@const designSummary = getTemplateDesignSummary(tool.content)}
                  <div class="flex items-center justify-between gap-3 rounded-xl border border-primary/20 bg-primary/5 px-3 py-2.5">
                    <div class="min-w-0">
                      <p class="text-xs font-semibold text-foreground">{m.template_ai_design_ready()}</p>
                      {#if designSummary}
                        <p class="mt-0.5 truncate text-[11px] text-muted-foreground">{designSummary.name}</p>
                      {/if}
                    </div>
                    <Button
                      variant="outline"
                      size="xs"
                      class="shrink-0"
                      disabled={!onApplyTemplateDesign}
                      onclick={() => onApplyTemplateDesign?.(tool.content)}
                    >
                      <Sparkles size={12} class="me-1" />
                      {m.template_ai_apply_design()}
                    </Button>
                  </div>
                {/if}
              {/each}
            </div>
          {/if}

          {#if msg.uiData?.length}
            {#each msg.uiData as data, uiIndex (`${msg.id}_ui_${uiIndex}`)}
              {#if data.type === "unpaid_fees"}
                <section class="w-full overflow-hidden rounded-2xl border border-border/70 bg-card/70 shadow-2xs">
                  <h3 class="border-b border-border/60 px-4 py-3 text-sm font-semibold">
                    {m.ai_ui_unpaid_fees_heading()} ({data.items.length})
                  </h3>
                  <div class="max-h-72 overflow-auto">
                    <table class="w-full min-w-[560px] border-collapse text-start text-xs">
                      <thead class="sticky top-0 bg-muted/90 text-muted-foreground">
                        <tr>
                          <th class="px-3 py-2 font-medium">{m.ai_ui_unpaid_fees_student()}</th>
                          <th class="px-3 py-2 font-medium">{m.ai_ui_unpaid_fees_balance()}</th>
                          <th class="px-3 py-2 font-medium">{m.ai_ui_unpaid_fees_status()}</th>
                          <th class="px-3 py-2 font-medium">{m.ai_ui_unpaid_fees_due_date()}</th>
                        </tr>
                      </thead>
                      <tbody>
                        {#each data.items as fee, rowIndex (`${msg.id}_${uiIndex}_${rowIndex}`)}
                          <tr class="border-t border-border/50">
                            <td class="px-3 py-2 font-medium">{fee.student_name}</td>
                            <td class="px-3 py-2 tabular-nums">{fee.balance_due}</td>
                            <td class="px-3 py-2">{feeStatusLabel(fee.status)}</td>
                            <td class="px-3 py-2 tabular-nums">{fee.due_date}</td>
                          </tr>
                        {/each}
                      </tbody>
                    </table>
                  </div>
                </section>
              {/if}
            {/each}
          {/if}

          <!-- Assistant Text Content Card -->
          {#if msg.content}
            <div class="relative bg-card/65 hover:bg-card/90 border border-border/70 rounded-2xl p-3.5 sm:p-4 shadow-2xs transition-all">
              <!-- Rendered Markdown Container -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                class="prose-ai text-[13px] leading-relaxed"
                onclick={handleContentClick}
              >
                {@html renderMarkdown(msg.content)}
              </div>

              {#if msg.isError && onRetry && !isThinking}
                <div class="pt-3 mt-3 border-t border-border/40 flex items-center justify-between">
                  <button
                    type="button"
                    onclick={() => onRetry(msg.id)}
                    class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-medium text-foreground/80 bg-muted/50 hover:bg-muted border border-border/60 hover:border-border transition-all cursor-pointer select-none"
                  >
                    <Refresh size={12} class="shrink-0 text-muted-foreground" />
                    <span>{m.ai_ui_retry()}</span>
                  </button>
                </div>
              {/if}
            </div>

            <!-- Assistant Actions on Hover -->
            <div class="flex items-center gap-1.5 ps-1 select-none opacity-0 group-hover:opacity-100 transition-opacity">
              {#if onRetry && !isThinking}
                <button
                  type="button"
                  onclick={() => onRetry(msg.id)}
                  class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                  title={m.ai_ui_regenerate()}
                >
                  <Refresh size={12} />
                </button>
              {/if}

              <button
                type="button"
                onclick={() => copyText(msg.content, msg.id)}
                class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title={m.ai_ui_copy_response()}
              >
                {#if copiedMessageId === msg.id}
                  <Check size={12} class="text-emerald-500" />
                {:else}
                  <Clipboard size={12} />
                {/if}
              </button>
            </div>
          {/if}

          <!-- Proposed Consequential Actions -->
          {#if msg.proposedActions && msg.proposedActions.length > 0}
            <div class="mt-1 min-w-0 space-y-2">
              {#each msg.proposedActions as action, aIdx (`${msg.id}_act_${aIdx}`)}
                <div class="w-full min-w-0 max-w-full space-y-2.5 overflow-hidden rounded-md border border-amber-500/30 bg-amber-500/5 p-3 dark:bg-amber-500/10">
                  <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                    <div class="flex min-w-0 items-center gap-2">
                      <span class="min-w-0 text-[12.5px] font-bold text-foreground [overflow-wrap:anywhere]">{action.title}</span>
                    </div>
                    {#if action.status === 'executed'}
                      <Badge variant="lime" class="text-[9px] px-1.5 py-0.5 flex items-center gap-1">
                        <CheckCircle size={10} />
                        {m.ai_ui_action_executed()}
                      </Badge>
                    {:else if action.status === 'rejected'}
                      <Badge variant="outline" class="text-[9px] px-1.5 py-0.5 text-destructive border-destructive/30 flex items-center gap-1">
                        <XMark size={10} />
                        {m.ai_ui_action_cancelled()}
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[9px] px-1.5 py-0.5 bg-amber-500/15 text-amber-700 dark:text-amber-300 font-semibold border-amber-500/20">
                        {m.ai_ui_approval_required()}
                      </Badge>
                    {/if}
                  </div>

                  <p class="min-w-0 text-xs leading-relaxed text-foreground/80 [overflow-wrap:anywhere]">
                    {action.description}
                  </p>

                  <!-- Payload details preview -->
                  {#if action.payload && Object.keys(action.payload).length > 0}
                    <div class="grid min-w-0 max-w-full grid-cols-[minmax(0,0.7fr)_minmax(0,1.3fr)] gap-x-3 gap-y-1 overflow-hidden rounded-md border border-border bg-background/80 p-2.5 font-mono text-[10.5px] text-muted-foreground">
                      {#each Object.entries(action.payload) as [key, val] (key)}
                        <span class="min-w-0 font-semibold text-foreground/70 [overflow-wrap:anywhere]">{key}:</span>
                        <span class="min-w-0 text-start text-foreground [overflow-wrap:anywhere]">{typeof val === 'object' ? JSON.stringify(val) : String(val)}</span>
                      {/each}
                    </div>
                  {/if}

                  <!-- Action Buttons -->
                  {#if action.status === 'executed'}
                    <div class="text-[10.5px] text-emerald-600 dark:text-emerald-400 font-semibold flex items-center gap-1 pt-1">
                      <CheckCircle size={12} />
                      {action.resultMessage || m.ai_ui_action_executed_successfully()}
                    </div>
                    {#if action.tool_name === "create_custom_report" && action.reportJobId}
                      <Button
                        size="xs"
                        variant="outline"
                        class="mt-2 h-7.5 rounded-lg px-3 text-[11px]"
                        onclick={() => onReportDownload?.(action)}
                      >
                        <Download size={12} class="me-1" />
                        {m.ai_ui_report_download()}
                      </Button>
                    {/if}
                  {:else if action.status === 'rejected'}
                    <div class="text-[10px] text-muted-foreground italic pt-1">
                      {m.ai_ui_action_dismissed_by_request()}
                    </div>
                  {:else if action.status === 'failed'}
                    <div class="text-[10.5px] text-destructive font-semibold pt-1">
                      {action.resultMessage || m.ai_ui_failed()}
                    </div>
                  {:else if action.status === 'processing'}
                    <div class="text-[10.5px] text-muted-foreground font-medium pt-1">
                      {action.resultMessage || m.ai_ui_report_submitted()}
                    </div>
                    {#if action.tool_name === "create_custom_report" && action.reportJobId}
                      <Button
                        size="xs"
                        variant="outline"
                        class="mt-2 h-7.5 rounded-lg px-3 text-[11px]"
                        disabled={action.reportChecking}
                        onclick={() => onActionResponse?.(action, "executed")}
                      >
                        <Refresh size={12} class="me-1" />
                        {m.ai_ui_report_check_status()}
                      </Button>
                    {/if}
                  {:else}
                    <div class="flex flex-wrap items-center gap-2 pt-1">
                      <Button
                        size="xs"
                        variant="default"
                        class="h-7.5 rounded-md px-3.5 text-[11px] font-semibold text-white shadow-2xs active:scale-95"
                        onclick={() => onActionResponse?.(action, "executed")}
                      >
                        <CheckCircle size={12} class="me-1" />
                        {m.ai_ui_approve_and_execute()}
                      </Button>
                      <Button
                        size="xs"
                        variant="ghost"
                        class="h-7.5 rounded-md px-3 text-[11px] text-muted-foreground hover:bg-destructive/10 hover:text-destructive active:scale-95"
                        onclick={() => onActionResponse?.(action, "rejected")}
                      >
                        {m.ai_ui_cancel()}
                      </Button>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}

        </div>
      {/if}
    {/each}

    <!-- Thinking / loading skeleton -->
    {#if isThinking && messages.at(-1)?.role === "assistant" && !messages.at(-1)?.content?.trim()}
      <AiLoadingSkeleton class="mt-2" />
    {/if}
  </div>
</div>

  <!-- Center Floating Scroll-to-bottom Button -->
  {#if showScrollBottom}
    <div
      class="absolute bottom-3 inset-x-0 flex justify-center pointer-events-none z-20"
      transition:fade={{ duration: 150 }}
    >
      <button
        type="button"
        onclick={scrollToBottom}
        class="pointer-events-auto shadow-md border border-border/80 bg-card/95 hover:bg-card backdrop-blur-md text-foreground px-3.5 py-1.5 rounded-full flex items-center gap-1.5 text-xs font-semibold cursor-pointer transition-all hover:scale-105 active:scale-95 group hover:border-primary/50"
        aria-label={m.ai_ui_scroll_to_bottom()}
      >
        <ChevronDown size={14} class="text-primary group-hover:translate-y-0.5 transition-transform" />
        <span class="text-[11px] font-medium">{m.ai_ui_scroll_to_bottom()}</span>
      </button>
    </div>
  {/if}
</div>

<style>
  :global(.prose-ai) {
    line-height: 1.7;
    color: var(--foreground);
    word-break: break-word;
  }

  :global(.prose-ai p) {
    margin-bottom: 0.6rem;
  }

  :global(.prose-ai p:last-child) {
    margin-bottom: 0;
  }

  :global(.prose-ai strong) {
    font-weight: 700;
    color: var(--foreground);
  }

  :global(.prose-ai em) {
    font-style: italic;
  }

  :global(.prose-ai code) {
    font-family: var(--font-mono);
    font-size: 0.88em;
    padding: 0.15rem 0.35rem;
    border-radius: 0.375rem;
    background-color: var(--muted);
    color: var(--primary);
    border: 1px solid color-mix(in oklch, var(--border) 60%, transparent);
  }

  :global(.prose-ai pre) {
    font-family: var(--font-mono);
    font-size: 0.82em;
    padding: 0.75rem 1rem;
    border-radius: 0.625rem;
    background-color: color-mix(in oklch, var(--muted) 80%, transparent);
    border: 1px solid var(--border);
    overflow-x: auto;
    margin: 0.6rem 0;
  }

  :global(.prose-ai pre code) {
    padding: 0;
    border: none;
    background: transparent;
    color: inherit;
  }

  :global(.prose-ai ul) {
    list-style-type: disc;
    padding-inline-start: 1.25rem;
    margin: 0.5rem 0;
  }

  :global(.prose-ai ol) {
    list-style-type: decimal;
    padding-inline-start: 1.25rem;
    margin: 0.5rem 0;
  }

  :global(.prose-ai li) {
    margin-bottom: 0.25rem;
  }

  :global(.prose-ai blockquote) {
    border-inline-start: 3px solid var(--primary);
    padding-inline-start: 0.75rem;
    margin: 0.5rem 0;
    color: var(--muted-foreground);
    background-color: color-mix(in oklch, var(--muted) 40%, transparent);
    border-radius: 0 0.5rem 0.5rem 0;
    padding-top: 0.35rem;
    padding-bottom: 0.35rem;
  }

  :global(.prose-ai table) {
    width: 100%;
    border-collapse: collapse;
    margin: 0.75rem 0;
    font-size: 0.88em;
  }

  :global(.prose-ai th), :global(.prose-ai td) {
    border: 1px solid var(--border);
    padding: 0.45rem 0.65rem;
    text-align: start;
  }

  :global(.prose-ai th) {
    background-color: var(--muted);
    font-weight: 600;
  }

  :global(.prose-ai h1) {
    font-size: 1.25rem;
    font-weight: 700;
    margin-top: 0.75rem;
    margin-bottom: 0.5rem;
  }

  :global(.prose-ai h2) {
    font-size: 1.1rem;
    font-weight: 700;
    margin-top: 0.65rem;
    margin-bottom: 0.4rem;
  }

  :global(.prose-ai h3) {
    font-size: 0.98rem;
    font-weight: 700;
    margin-top: 0.5rem;
    margin-bottom: 0.35rem;
  }

  :global(.prose-ai hr) {
    border: 0;
    border-top: 1px solid var(--border);
    margin: 1rem 0;
  }

  :global(.prose-ai a) {
    color: var(--primary);
    text-decoration: underline;
    font-weight: 500;
    cursor: pointer;
  }
</style>
