<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui/skeleton";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import AiLoadingSkeleton from "./ai-loading-skeleton.svelte";
import {
  Wrench,
  CheckCircle2,
  XCircle,
  AlertTriangle,
  ChevronDown,
  ChevronRight,
  ExternalLink,
  Copy,
  Check,
} from "@lucide/svelte";
import type { AiChatMessage, ActionWithStatus } from "$lib/features/ai/session-store";
import type { AiToolResult } from "$lib/api/types";

let {
  messages = [],
  isThinking = false,
  userName = "",
  orgName = null,
  chatContainer = $bindable(),
  onLinkClick,
  onApproveAction,
  onRejectAction,
}: {
  messages: AiChatMessage[];
  isThinking: boolean;
  userName: string;
  orgName: string | null;
  chatContainer?: HTMLDivElement;
  onLinkClick: (href: string) => void;
  onApproveAction?: (action: ActionWithStatus, msgId: string, actionIdx: number) => void;
  onRejectAction?: (action: ActionWithStatus, msgId: string, actionIdx: number) => void;
} = $props();

let expandedTools = $state<Record<string, boolean>>({});
let copiedMessageId = $state<string | null>(null);

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

// Markdown parser supporting bold, inline code, code blocks, lists, links, headers, and blockquotes
interface MessagePart {
  type: "text" | "bold" | "code" | "code_block" | "link" | "header" | "list_item" | "blockquote";
  content?: string;
  label?: string;
  href?: string;
  language?: string;
}

function parseMarkdown(text: string): MessagePart[] {
  if (!text) return [];
  const parts: MessagePart[] = [];
  
  // Split lines to handle headers and lists cleanly
  const lines = text.split("\n");
  let inCodeBlock = false;
  let codeBlockLang = "";
  let codeBlockBuffer: string[] = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.startsWith("```")) {
      if (inCodeBlock) {
        parts.push({
          type: "code_block",
          content: codeBlockBuffer.join("\n"),
          language: codeBlockLang,
        });
        codeBlockBuffer = [];
        inCodeBlock = false;
      } else {
        inCodeBlock = true;
        codeBlockLang = line.replace("```", "").trim();
      }
      continue;
    }

    if (inCodeBlock) {
      codeBlockBuffer.push(line);
      continue;
    }

    // Header
    if (line.startsWith("### ")) {
      parts.push({ type: "header", content: line.replace("### ", "").trim() });
      continue;
    } else if (line.startsWith("## ")) {
      parts.push({ type: "header", content: line.replace("## ", "").trim() });
      continue;
    } else if (line.startsWith("# ")) {
      parts.push({ type: "header", content: line.replace("# ", "").trim() });
      continue;
    }

    // Blockquote
    if (line.startsWith("> ")) {
      parts.push({ type: "blockquote", content: line.replace(/^>\s*/, "").trim() });
      continue;
    }

    // List item
    if (line.startsWith("- ") || line.startsWith("* ") || /^\d+\.\s/.test(line)) {
      const cleanLine = line.replace(/^[-*]\s+|\d+\.\s+/, "");
      parts.push({ type: "list_item", content: cleanLine });
      continue;
    }

    // Parse inline elements (bold, links, code)
    const inlineRegex = /(\*\*([^*]+)\*\*|`([^`]+)`|\[([^\]]+)\]\(([^)]+)\))/g;
    let lastIdx = 0;
    let match;

    while ((match = inlineRegex.exec(line)) !== null) {
      if (match.index > lastIdx) {
        parts.push({
          type: "text",
          content: line.substring(lastIdx, match.index),
        });
      }

      if (match[2]) {
        // Bold
        parts.push({ type: "bold", content: match[2] });
      } else if (match[3]) {
        // Inline code
        parts.push({ type: "code", content: match[3] });
      } else if (match[4] && match[5]) {
        // Link
        parts.push({ type: "link", label: match[4], href: match[5] });
      }

      lastIdx = inlineRegex.lastIndex;
    }

    if (lastIdx < line.length) {
      parts.push({ type: "text", content: line.substring(lastIdx) });
    }

    // Add newline if not last line
    if (i < lines.length - 1) {
      parts.push({ type: "text", content: "\n" });
    }
  }

  return parts;
}
</script>

<div 
  bind:this={chatContainer}
  class="flex-1 flex flex-col p-4 overflow-y-auto scrollbar-hide select-text space-y-5"
>
  <div class="flex flex-col gap-5 w-full">
    {#each messages as msg (msg.id)}
      {#if msg.role === 'user'}
        <!-- User message bubble -->
        <div class="flex justify-end w-full">
          <div class="max-w-[85%] bg-primary text-primary-foreground px-4 py-2.5 rounded-2xl rounded-tr-sm text-[13px] leading-relaxed break-words whitespace-pre-wrap font-medium shadow-sm">
            {msg.content}
          </div>
        </div>
      {:else}
        <!-- Assistant response bubble -->
        <div class="flex flex-col gap-3 w-full text-start text-[13px] text-foreground/90 leading-relaxed font-normal">
          
          <!-- Executed Tools Badges (ReAct steps) -->
          {#if msg.executedTools && msg.executedTools.length > 0}
            <div class="flex flex-col gap-1.5 w-full">
              {#each msg.executedTools as tool, tIdx}
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

          <!-- Assistant Text Content -->
          {#if msg.content}
            <div class="group relative bg-card/60 hover:bg-card/90 border rounded-xl p-3.5 shadow-2xs transition-colors {msg.isError ? 'border-destructive/30 bg-destructive/5 dark:bg-destructive/10' : 'border-border/50'}">
              <div class="leading-relaxed space-y-1.5 wrap-break-word">
                {#each parseMarkdown(msg.content) as part}
                  {#if part.type === 'bold'}
                    <strong class="font-bold text-foreground">{part.content}</strong>
                  {:else if part.type === 'code'}
                    <code class="px-1.5 py-0.5 rounded bg-muted font-mono text-[11.5px] text-primary border border-border/60">{part.content}</code>
                  {:else if part.type === 'code_block'}
                    <pre class="my-2 p-3 rounded-lg bg-muted/80 font-mono text-[11px] overflow-x-auto border border-border/70 text-foreground">{part.content}</pre>
                  {:else if part.type === 'blockquote'}
                    <blockquote class="my-1.5 ps-2.5 pe-2 py-1.5 rounded-e-md border-s-2 border-amber-500/60 bg-muted/50 font-mono text-[11px] text-muted-foreground break-words whitespace-pre-wrap">
                      {part.content}
                    </blockquote>
                  {:else if part.type === 'header'}
                    <div class="text-sm font-bold text-foreground pt-2 pb-1 border-b border-border/40">{part.content}</div>
                  {:else if part.type === 'list_item'}
                    <div class="flex items-start gap-2 ps-2">
                      <span class="text-primary font-bold">•</span>
                      <span>{part.content}</span>
                    </div>
                  {:else if part.type === 'link'}
                    <button
                      type="button"
                      onclick={() => onLinkClick(part.href || "")}
                      class="inline-flex items-center gap-1 text-primary hover:underline font-semibold cursor-pointer align-baseline px-1 py-0.5 rounded hover:bg-primary/10 transition-colors"
                    >
                      {part.label}
                      <ExternalLink size={10} />
                    </button>
                  {:else}
                    <span class="whitespace-pre-wrap">{part.content}</span>
                  {/if}
                {/each}
              </div>

              <!-- Action Bar (Copy) -->
              <div class="flex items-center justify-end gap-1.5 mt-2 pt-1 border-t border-border/30 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  type="button"
                  onclick={() => copyText(msg.content, msg.id)}
                  class="text-[10px] text-muted-foreground hover:text-foreground flex items-center gap-1 px-1.5 py-0.5 rounded hover:bg-muted transition-colors cursor-pointer"
                  title="نسخ النص"
                >
                  {#if copiedMessageId === msg.id}
                    <Check size={11} class="text-emerald-500" />
                    <span>تم النسخ</span>
                  {:else}
                    <Copy size={11} />
                    <span>نسخ</span>
                  {/if}
                </button>
              </div>
            </div>
          {/if}

          <!-- Proposed Consequential Actions -->
          {#if msg.proposedActions && msg.proposedActions.length > 0}
            <div class="space-y-2 mt-1">
              {#each msg.proposedActions as action, aIdx}
                <div class="border border-amber-500/30 bg-amber-500/5 dark:bg-amber-500/10 rounded-xl p-3.5 space-y-2.5 shadow-sm">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                      <AlertTriangle size={15} class="text-amber-500 shrink-0" />
                      <span class="text-[12.5px] font-bold text-foreground">{action.title}</span>
                    </div>
                    {#if action.status === 'executed'}
                      <Badge variant="lime" class="text-[9px] px-1.5 py-0.5 flex items-center gap-1">
                        <CheckCircle2 size={10} />
                        تم التنفيذ
                      </Badge>
                    {:else if action.status === 'rejected'}
                      <Badge variant="outline" class="text-[9px] px-1.5 py-0.5 text-destructive border-destructive/30 flex items-center gap-1">
                        <XCircle size={10} />
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
                      {#each Object.entries(action.payload) as [key, val]}
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
                      <CheckCircle2 size={12} />
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
                        <CheckCircle2 size={12} class="me-1" />
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
