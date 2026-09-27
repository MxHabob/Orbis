<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { marked } from "marked";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import AiLoadingSkeleton from "./ai-loading-skeleton.svelte";
import { Check, CheckCircle, ChevronDown, ChevronRight, Clipboard, InfoCircle, Pencil, Refresh, Sparkles, Wrench, XMark } from "@orbis/icons";
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
  onActionResponse,
  onRetry,
  onEditUserMessage,
}: {
  messages: AiChatMessage[];
  isThinking: boolean;
  userName: string;
  orgName: string | null;
  chatContainer?: HTMLDivElement;
  onLinkClick: (href: string) => void;
  onActionResponse?: (action: ActionWithStatus, status: "executed" | "rejected") => void;
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
  class="flex-1 flex flex-col p-3.5 sm:p-4 overflow-y-auto scrollbar-thin select-text space-y-4"
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
                bind:value={editingText}
                rows={3}
                class="w-full p-2.5 text-[13px] bg-background border border-border rounded-xl focus:outline-none focus:ring-1 focus:ring-primary resize-none text-foreground font-sans leading-relaxed"
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
                  class="text-[11px] h-7 px-2.5 text-muted-foreground hover:bg-muted cursor-pointer"
                  onclick={cancelEditing}
                >
                  <XMark size={12} class="me-1" />
                  إلغاء
                </Button>
                <Button
                  size="xs"
                  variant="default"
                  class="text-[11px] h-7 px-3 bg-primary text-primary-foreground font-semibold cursor-pointer"
                  onclick={() => submitEdit(msg.id)}
                  disabled={!editingText.trim()}
                >
                  حفظ وإرسال
                </Button>
              </div>
            </div>
          {:else}
            <!-- User Text Bubble (Vibrant Gradient / Solid) -->
            <div class="max-w-[88%] bg-primary text-primary-foreground px-4 py-2.5 rounded-2xl rounded-tr-xs rtl:rounded-tr-2xl rtl:rounded-tl-xs text-[13px] leading-relaxed break-words whitespace-pre-wrap font-medium shadow-2xs">
              {msg.content}
            </div>

            <!-- User Hover Action Bar -->
            <div class="flex items-center gap-1 mt-1 opacity-0 group-hover:opacity-100 transition-opacity select-none">
              <!-- Edit Button -->
              <button
                type="button"
                onclick={() => startEditing(msg)}
                class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
                title="تعديل الرسالة"
              >
                <Pencil size={12} />
              </button>

              <!-- Copy Button -->
              <button
                type="button"
                onclick={() => copyText(msg.content, msg.id)}
                class="p-1 rounded-md text-muted-foreground/70 hover:text-foreground hover:bg-muted/70 transition-colors cursor-pointer"
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
                        {tool.is_error ? 'خطأ' : 'مكتمل'}
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
              {/each}
            </div>
          {/if}

          <!-- Assistant Text Content Card -->
          {#if msg.content}
            <div class="relative bg-card/75 hover:bg-card/95 border rounded-2xl p-4 shadow-2xs transition-all {msg.isError ? 'border-destructive/30 bg-destructive/5 dark:bg-destructive/10' : 'border-border/70'}">
              <!-- Assistant Header with Clean Icon -->
              <div class="flex items-center justify-between gap-2 mb-2 pb-1.5 border-b border-border/40 select-none">
                <div class="flex items-center gap-1.5">
                  <div class="w-5 h-5 rounded-md border border-primary/30 bg-primary/10 flex items-center justify-center p-0.5 shrink-0 text-primary shadow-2xs">
                    <Sparkles size={11} />
                  </div>
                  <span class="text-[11.5px] font-bold text-foreground/90 flex items-center gap-1.5">
                    Orbis AI
                    {#if msg.isError}
                      <Badge variant="destructive" class="text-[8.5px] h-4 px-1.5 font-bold">خطأ في الطلب</Badge>
                    {/if}
                  </span>
                </div>

                <!-- Fast Actions (Copy & Retry) -->
                <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                  {#if onRetry && !isThinking}
                    <button
                      type="button"
                      onclick={() => onRetry(msg.id)}
                      class="text-[11px] text-muted-foreground hover:text-foreground flex items-center p-1 rounded-md hover:bg-muted/70 transition-colors cursor-pointer"
                      title="إعادة التوليد"
                    >
                      <Refresh size={12} />
                    </button>
                  {/if}

                  <button
                    type="button"
                    onclick={() => copyText(msg.content, msg.id)}
                    class="text-[11px] text-muted-foreground hover:text-foreground flex items-center p-1 rounded-md hover:bg-muted/70 transition-colors cursor-pointer"
                    title="نسخ الرد"
                  >
                    {#if copiedMessageId === msg.id}
                      <Check size={12} class="text-emerald-500" />
                    {:else}
                      <Clipboard size={12} />
                    {/if}
                  </button>
                </div>
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
            </div>
          {/if}

          <!-- Proposed Consequential Actions -->
          {#if msg.proposedActions && msg.proposedActions.length > 0}
            <div class="space-y-2 mt-1">
              {#each msg.proposedActions as action, aIdx (`${msg.id}_act_${aIdx}`)}
                <div class="border border-amber-500/30 bg-amber-500/5 dark:bg-amber-500/10 rounded-2xl p-3.5 space-y-2.5 shadow-sm">
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
                    <div class="bg-background/80 border border-border/60 rounded-xl p-2.5 font-mono text-[10.5px] space-y-1 text-muted-foreground">
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
                    <div class="text-[10.5px] text-emerald-600 dark:text-emerald-400 font-semibold flex items-center gap-1 pt-1">
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
                        class="text-[11px] h-7.5 px-3.5 font-semibold bg-emerald-600 hover:bg-emerald-700 active:scale-95 text-white cursor-pointer rounded-lg shadow-2xs"
                        onclick={() => onActionResponse?.(action, "executed")}
                      >
                        <CheckCircle size={12} class="me-1" />
                        الموافقة والتنفيذ
                      </Button>
                      <Button
                        size="xs"
                        variant="ghost"
                        class="text-[11px] h-7.5 px-3 text-muted-foreground hover:text-destructive hover:bg-destructive/10 active:scale-95 cursor-pointer rounded-lg"
                        onclick={() => onActionResponse?.(action, "rejected")}
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

    <!-- Thinking / loading skeleton -->
    {#if isThinking && messages.at(-1)?.role === "assistant" && !messages.at(-1)?.content?.trim()}
      <AiLoadingSkeleton class="mt-2" />
    {/if}
  </div>
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
