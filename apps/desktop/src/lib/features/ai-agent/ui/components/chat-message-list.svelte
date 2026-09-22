<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { marked } from "marked";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import AiLoadingSkeleton from "./ai-loading-skeleton.svelte";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";
import { BarsArrowUp, Check, CheckCircle, ChevronDown, ChevronRight, Clipboard, InfoCircle, Pencil, Refresh, Wrench, XMark } from "@orbis/icons";
import type { AiChatMessage, ActionWithStatus } from "$lib/features/ai/session-store";

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
  onApproveAction,
  onRejectAction,
  onRetry,
  onEditUserMessage,
}: {
  messages: AiChatMessage[];
  isThinking: boolean;
  userName: string;
  orgName: string | null;
  chatContainer?: HTMLDivElement;
  onLinkClick: (href: string) => void;
  onApproveAction?: (action: ActionWithStatus, msgId: string, actionIdx: number) => void;
  onRejectAction?: (action: ActionWithStatus, msgId: string, actionIdx: number) => void;
  onRetry?: (msgId: string) => void;
  onEditUserMessage?: (msgId: string, newText: string) => void;
} = $props();

let expandedTools = $state<Record<string, boolean>>({});
let copiedMessageId = $state<string | null>(null);
let editingMessageId = $state<string | null>(null);
let editingText = $state("");

function toggleTool(key: string) {
  expandedTools[key] = !expandedTools[key];
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

<div 
  bind:this={chatContainer}
  class="flex-1 flex flex-col p-4 overflow-y-auto scrollbar-hide select-text space-y-5"
>
  <div class="flex flex-col gap-5 w-full">
    {#each messages as msg (msg.id)}
      {#if msg.role === 'user'}
        <!-- User message bubble with Hover Action Buttons -->
        <div class="group relative flex flex-col items-end w-full">
          {#if editingMessageId === msg.id}
            <!-- Inline Pencil Form -->
            <div class="w-full max-w-[90%] flex flex-col gap-2 bg-card border border-primary/40 rounded-2xl p-3 shadow-md">
              <textarea
                bind:value={editingText}
                rows={3}
                class="w-full p-2 text-[13px] bg-background border border-border rounded-xl focus:outline-none focus:ring-1 focus:ring-primary resize-none text-foreground font-sans leading-relaxed"
                placeholder="تعديل الرسالة..."
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
                  class="text-[11px] h-7 px-2.5 text-muted-foreground hover:bg-muted"
                  onclick={cancelEditing}
                >
                  <XMark size={12} class="me-1" />
                  إلغاء
                </Button>
                <Button
                  size="xs"
                  variant="default"
                  class="text-[11px] h-7 px-3 bg-primary text-primary-foreground font-semibold"
                  onclick={() => submitEdit(msg.id)}
                  disabled={!editingText.trim()}
                >
                  <BarsArrowUp size={12} class="me-1" />
                  حفظ وإرسال
                </Button>
              </div>
            </div>
          {:else}
            <!-- User Text Bubble -->
            <div class="max-w-[85%] bg-primary text-primary-foreground px-4 py-2.5 rounded-2xl rounded-tr-sm text-[13px] leading-relaxed break-words whitespace-pre-wrap font-medium shadow-sm">
              {msg.content}
            </div>

            <!-- User Hover Action Bar -->
            <div class="flex items-center gap-1 mt-1 opacity-0 group-hover:opacity-100 transition-opacity select-none">
              <!-- Pencil Button -->
              <button
                type="button"
                onclick={() => startEditing(msg)}
                class="p-1 rounded-md text-muted-foreground/80 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title="تعديل السؤال"
              >
                <Pencil size={12} />
              </button>

              <!-- Clipboard Button -->
              <button
                type="button"
                onclick={() => copyText(msg.content, msg.id)}
                class="p-1 rounded-md text-muted-foreground/80 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title="نسخ النص"
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
        <div class="group relative flex flex-col gap-3 w-full text-start text-[13px] text-foreground/90 leading-relaxed font-normal">
          
          <!-- Executed Tools Badges (ReAct steps) -->
          {#if msg.executedTools && msg.executedTools.length > 0}
            <div class="flex flex-col gap-1.5 w-full">
              {#each msg.executedTools as tool, tIdx (`${msg.id}_tool_${tIdx}`)}
                {@const toolKey = `${msg.id}_tool_${tIdx}`}
                {@const isExpanded = !!expandedTools[toolKey]}
                <div class="border border-border/70 rounded-lg bg-muted/40 overflow-hidden text-xs">
                  <button
                    type="button"
                    onclick={() => toggleTool(toolKey)}
                    class="w-full px-2.5 py-1.5 flex items-center justify-between hover:bg-muted/70 transition-colors cursor-pointer text-start select-none"
                  >
                    <div class="flex items-center gap-1.5 text-foreground/80 font-mono text-[11px]">
                      <Wrench size={12} class="text-primary" />
                      <span class="font-bold">{tool.tool_name}</span>
                      {#if tool.is_error}
                        <Badge variant="destructive" class="text-[8px] h-3.5 px-1">خطأ</Badge>
                      {:else}
                        <Badge variant="lime" class="text-[8px] h-3.5 px-1 font-mono">تم التنفيذ</Badge>
                      {/if}
                    </div>
                    <div class="text-muted-foreground">
                      {#if isExpanded}
                        <ChevronDown size={13} />
                      {:else}
                        <ChevronRight size={13} />
                      {/if}
                    </div>
                  </button>

                  {#if isExpanded}
                    <div class="p-2 border-t border-border/50 bg-background/80 font-mono text-[10.5px] leading-relaxed max-h-48 overflow-y-auto whitespace-pre-wrap break-words text-foreground/80">
                      {tool.content}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}

          <!-- Assistant Text Content with Full GFM Markdown Support -->
          {#if msg.content}
            <div class="relative bg-card/60 hover:bg-card/90 border rounded-xl p-3.5 shadow-2xs transition-colors {msg.isError ? 'border-destructive/30 bg-destructive/5 dark:bg-destructive/10' : 'border-border/50'}">
              <!-- Assistant Header Badge with Expressive Avatar -->
              <div class="flex items-center gap-2 mb-2 pb-1 border-b border-border/30 select-none">
                <div class="flex items-center justify-center shrink-0">
                  <UserAvatar
                    hat="graduation"
                    state={msg.isError ? "error" : "idle"}
                    width="20px"
                    height="20px"
                    class="shrink-0"
                  />
                </div>
                <span class="text-[11px] font-bold text-foreground/80 flex items-center gap-1.5">
                  <span>Orbis</span>
                  {#if msg.isError}
                    <Badge variant="destructive" class="text-[8px] h-3.5 px-1 font-bold">خطأ في الطلب</Badge>
                  {/if}
                </span>
              </div>

              <!-- Rendered Markdown Container -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                class="prose-ai text-[13px] leading-relaxed"
                onclick={handleContentClick}
              >
                {@html renderMarkdown(msg.content)}
              </div>

              <!-- Action Bar (Clipboard & Retry on hover) -->
              <div class="flex items-center justify-end gap-1.5 mt-2.5 pt-1.5 border-t border-border/30 opacity-0 group-hover:opacity-100 transition-opacity select-none">
                <!-- Retry / Regenerate button -->
                {#if onRetry && !isThinking}
                  <button
                    type="button"
                    onclick={() => onRetry(msg.id)}
                    class="text-[11px] text-muted-foreground hover:text-foreground flex items-center gap-1 px-1.5 py-0.5 rounded hover:bg-muted transition-colors cursor-pointer"
                    title="إعادة التوليد"
                  >
                    <Refresh size={11} />
                  </button>
                {/if}

                <!-- Clipboard button -->
                <button
                  type="button"
                  onclick={() => copyText(msg.content, msg.id)}
                  class="text-[11px] text-muted-foreground hover:text-foreground flex items-center gap-1 px-1.5 py-0.5 rounded hover:bg-muted transition-colors cursor-pointer"
                  title="نسخ الرد"
                >
                  {#if copiedMessageId === msg.id}
                    <Check size={11} class="text-emerald-500" />
                  {:else}
                    <Clipboard size={11} />
                  {/if}
                </button>
              </div>
            </div>
          {/if}

          <!-- Proposed Consequential Actions -->
          {#if msg.proposedActions && msg.proposedActions.length > 0}
            <div class="space-y-2 mt-1">
              {#each msg.proposedActions as action, aIdx (`${msg.id}_act_${aIdx}`)}
                <div class="border border-amber-500/30 bg-amber-500/5 dark:bg-amber-500/10 rounded-xl p-3.5 space-y-2.5 shadow-sm">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                      <InfoCircle size={15} class="text-amber-500 shrink-0" />
                      <span class="text-[12.5px] font-bold text-foreground">{action.title}</span>
                    </div>
                    {#if action.status === 'executed'}
                      <Badge variant="lime" class="text-[9px] px-1.5 py-0.5 flex items-center gap-1">
                        <CheckCircle size={10} />
                        تم التنفيذ
                      </Badge>
                    {:else if action.status === 'rejected'}
                      <Badge variant="outline" class="text-[9px] px-1.5 py-0.5 text-destructive border-destructive/30 flex items-center gap-1">
                        <XMark size={10} />
                        تم الإلغاء
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[9px] px-1.5 py-0.5 bg-amber-500/15 text-amber-700 dark:text-amber-300 font-semibold border-amber-500/20">
                        يتطلب الموافقة
                      </Badge>
                    {/if}
                  </div>

                  <p class="text-xs text-foreground/80 leading-relaxed">
                    {action.description}
                  </p>

                  <!-- Payload details preview -->
                  {#if action.payload && Object.keys(action.payload).length > 0}
                    <div class="bg-background/80 border border-border/60 rounded-lg p-2 font-mono text-[10.5px] space-y-1 text-muted-foreground">
                      {#each Object.entries(action.payload) as [key, val] (key)}
                        <div class="flex items-center justify-between">
                          <span class="font-semibold text-foreground/70">{key}:</span>
                          <span class="text-foreground">{typeof val === 'object' ? JSON.stringify(val) : String(val)}</span>
                        </div>
                      {/each}
                    </div>
                  {/if}

                  <!-- Action Buttons -->
                  {#if action.status === 'executed'}
                    <div class="text-[10px] text-emerald-600 dark:text-emerald-400 font-semibold flex items-center gap-1 pt-1">
                      <CheckCircle size={12} />
                      {action.resultMessage || "تم تنفيذ الإجراء بنجاح في النظام"}
                    </div>
                  {:else if action.status === 'rejected'}
                    <div class="text-[10px] text-muted-foreground italic pt-1">
                      تم تجاهل هذا الإجراء بناءً على طلبك.
                    </div>
                  {:else}
                    <div class="flex items-center gap-2 pt-1">
                      <Button
                        size="xs"
                        variant="default"
                        class="text-[11px] h-7 px-3 font-semibold bg-emerald-600 hover:bg-emerald-700 text-white cursor-pointer"
                        onclick={() => onApproveAction?.(action, msg.id, aIdx)}
                      >
                        <CheckCircle size={12} class="me-1" />
                        الموافقة والتنفيذ
                      </Button>
                      <Button
                        size="xs"
                        variant="ghost"
                        class="text-[11px] h-7 px-3 text-muted-foreground hover:text-destructive hover:bg-destructive/10 cursor-pointer"
                        onclick={() => onRejectAction?.(action, msg.id, aIdx)}
                      >
                        إلغاء
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

    <!-- Thinking/loading skeleton -->
    {#if isThinking && messages.at(-1)?.role === "assistant" && !messages.at(-1)?.content?.trim()}
      <AiLoadingSkeleton class="mt-2" />
    {/if}
  </div>
</div>

<style>
  :global(.prose-ai) {
    line-height: 1.65;
    color: var(--foreground);
    word-break: break-word;
  }

  :global(.prose-ai p) {
    margin-bottom: 0.5rem;
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
    border-radius: 0.5rem;
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
    border-radius: 0 0.375rem 0.375rem 0;
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
    padding: 0.4rem 0.6rem;
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
