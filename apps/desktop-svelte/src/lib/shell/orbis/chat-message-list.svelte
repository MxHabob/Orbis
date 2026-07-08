<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import * as m from "$lib/paraglide/messages";

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

function parseMarkdownLinks(text: string) {
  const regex = /\[([^\]]+)\]\(([^)]+)\)/g;
  const parts: { type: "text" | "link"; content?: string; label?: string; href?: string }[] = [];
  let lastIndex = 0;
  let match;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push({ type: "text", content: text.substring(lastIndex, match.index) });
    }
    const fullLabel = match[1];
    const href = match[2];
    parts.push({ type: "link", label: fullLabel, href });
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
            {#each parseMarkdownLinks(msg.content) as part}
              {#if part.type === 'link'}
                <button
                  type="button"
                  onclick={() => onLinkClick(part.href!)}
                  class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                >
                  {part.label}
                </button>
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
              {#each parseMarkdownLinks(msg.content) as part}
                {#if part.type === 'link'}
                  <button
                    type="button"
                    onclick={() => onLinkClick(part.href!)}
                    class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                  >
                    {part.label}
                  </button>
                {:else}
                  {part.content}
                {/if}
              {/each}
            </div>
          {:else}
            <p>
              {#if orgName}
                {m.agent_response_greeting_with_org({
                  user: userName,
                  org: orgName,
                })}
              {:else}
                {m.agent_response_greeting_without_org({
                  user: userName,
                })}
              {/if}
            </p>
            <p class="mt-1">
              {m.agent_response_question()}
            </p>
            <ul class="list-disc ps-5 flex flex-col gap-2 mt-1">
              <li>{m.agent_response_bullet_1()}</li>
              <li>{m.agent_response_bullet_2()}</li>
              <li>{m.agent_response_bullet_3()}</li>
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
