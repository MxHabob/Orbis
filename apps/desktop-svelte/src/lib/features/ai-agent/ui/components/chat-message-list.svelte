<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui-svelte/skeleton";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
}

let {
  messages = [],
  isThinking = false,
  userName = "",
  orgName = null,
  chatContainer = $bindable(),
  onLinkClick,
}: {
  messages: Message[];
  isThinking: boolean;
  userName: string;
  orgName: string | null;
  chatContainer?: HTMLDivElement;
  onLinkClick: (href: string) => void;
} = $props();

function parseMessageParts(text: string) {
  const regex = /\[([^\]]+)\]\(([^)]+)\)|\[نتيجة الأداة ([^:]+):\s*([^\]]+)\]/g;
  const parts: any[] = [];
  let lastIndex = 0;
  let match;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push({
        type: "text",
        content: text.substring(lastIndex, match.index),
      });
    }
    
    if (match[1] && match[2]) {
      // Link
      parts.push({ type: "link", label: match[1], href: match[2] });
    } else if (match[3] && match[4]) {
      // Tool result
      parts.push({ type: "tool", toolName: match[3].trim(), result: match[4].trim() });
    }
    
    lastIndex = regex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push({ type: "text", content: text.substring(lastIndex) });
  }

  return parts.length > 0 ? parts : [{ type: "text", content: text }];
}
</script>

<div 
  bind:this={chatContainer}
  class="flex-1 flex flex-col p-4 overflow-y-auto scrollbar-hide select-text"
>
  <div class="flex flex-col gap-5 w-full">
    {#each messages as msg}
      {#if msg.role === 'user'}
        <!-- User message bubble on the right -->
        <div class="flex justify-end w-full">
          <div class="max-w-[85%] bg-muted text-foreground px-4 py-2.5 rounded-2xl text-[12.5px] leading-relaxed break-words font-medium shadow-sm">
            {#each parseMessageParts(msg.content) as part}
              {#if part.type === 'link'}
                <button
                  type="button"
                  onclick={() => onLinkClick(part.href!)}
                  class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                >
                  {part.label}
                </button>
              {:else if part.type === 'tool'}
                <div class="my-2 p-2 rounded-lg bg-foreground/5 border border-foreground/10 text-xs flex flex-col gap-1">
                  <div class="font-bold text-primary flex items-center gap-1">
                    <svg class="w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>
                    {part.toolName}
                  </div>
                  <div class="text-foreground/80 font-mono text-[11px] whitespace-pre-wrap">{part.result}</div>
                </div>
              {:else}
                {part.content}
              {/if}
            {/each}
          </div>
        </div>
      {:else}
        <!-- Agent response on the left -->
        <div class="flex flex-col gap-3 w-full text-start text-[13px] text-foreground/90 leading-relaxed font-normal">
          {#if msg.content}
            <div class="wrap-break-word">
              {#each parseMessageParts(msg.content) as part}
                {#if part.type === 'link'}
                  <button
                    type="button"
                    onclick={() => onLinkClick(part.href!)}
                    class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                  >
                    {part.label}
                  </button>
                {:else if part.type === 'tool'}
                  <div class="my-2 p-2.5 rounded-lg bg-muted border border-border shadow-sm text-xs flex flex-col gap-1.5">
                    <div class="font-bold text-primary flex items-center gap-1.5">
                      <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>
                      إجراء النظام: {part.toolName}
                    </div>
                    <div class="text-foreground/80 font-mono text-[11px] whitespace-pre-wrap px-1">{part.result}</div>
                  </div>
                {:else}
                  {part.content}
                {/if}
              {/each}
            </div>
          {:else}
            <p>
              {#if orgName}
                {m.common_agent_response_greeting_with_org({
                  user: userName,
                  org: orgName,
                })}
              {:else}
                {m.common_agent_response_greeting_without_org({
                  user: userName,
                })}
              {/if}
            </p>
            <p class="mt-1">
              {m.common_agent_response_question()}
            </p>
            <ul class="list-disc ps-5 flex flex-col gap-2 mt-1">
              <li>{m.common_agent_response_bullet_1()}</li>
              <li>{m.common_agent_response_bullet_2()}</li>
              <li>{m.common_agent_response_bullet_3()}</li>
            </ul>
          {/if}
        </div>
      {/if}
    {/each}

    <!-- Thinking/loading skeleton -->
    {#if isThinking}
      <div class="flex flex-col gap-3 w-full text-start mt-2">
        <Skeleton class="h-4 w-3/4 rounded-md" />
        <Skeleton class="h-4 w-5/6 rounded-md" />
        <Skeleton class="h-4 w-1/2 rounded-md" />
        <div class="flex flex-col gap-2 ps-5 mt-2">
          <Skeleton class="h-3.5 w-2/3 rounded-md" />
          <Skeleton class="h-3.5 w-3/4 rounded-md" />
          <Skeleton class="h-3.5 w-3/5 rounded-md" />
        </div>
      </div>
    {/if}
  </div>
</div>
