<!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis agent, mirroring the Seer design.
Allows users to ask questions, select prompt suggestions, and collapse/resize.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { uiStore, authStore, aiContext } from "$lib/stores";
import { fly } from "svelte/transition";
import { getLocale } from "$lib/paraglide/runtime";
import { isDesktopApp } from "$lib/core/ipc";
import { toast } from "@orbis/ui";
import ChatInput from "../components/chat-input.svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { commands } from "$lib/core/ipc";
import { createAgentSessionsQuery } from "$lib/features/ai/queries";
import AgentHeader from "../components/agent-header.svelte";
import AgentCockpit from "../components/agent-cockpit.svelte";
import ChatMessageList from "../components/chat-message-list.svelte";
import AgentEmptyState from "../components/agent-empty-state.svelte";
import Resizer from "$lib/shell/resizer.svelte";

let { isStandalone = false }: { isStandalone?: boolean } = $props();

const currentOrgId = $derived(page.params.orgId || authStore.orgId || "");

// Agent Session & Action Log Cockpit States
let selectedSessionId = $state<string | null>(null);
let viewMode = $state<"chat" | "cockpit">("chat");

const sessionsQuery = createAgentSessionsQuery(() => currentOrgId);
const sessions = $derived<any[]>(sessionsQuery.data ?? []);

const effectiveSessionId = $derived(
  selectedSessionId || (sessions.length > 0 ? sessions[0].id : null),
);

function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
  return px / base;
}

let sidebarDiv = $state<HTMLDivElement>();
let preferredWidth = $state(pxToRem(380));
const minWidth = $derived(pxToRem(320));
const maxWidth = $derived(pxToRem(650));

const isRtl = $derived(getLocale() === "ar");

const userName = $derived(
  authStore.user?.full_name || m.common_agent_default_user_name(),
);
const orgName = $derived(
  (authStore.user as any)?.org_name ||
    (authStore.user as any)?.organization_name ||
    (authStore.user as any)?.org?.name ||
    null,
);

let inputValue = $state(page.url.searchParams.get("ask_orbis") || "");
const suggestions = $derived([
  m.common_agent_suggest_1(),
  m.common_agent_suggest_2(),
  m.common_agent_suggest_3(),
]);

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
}

let messages = $state<Message[]>([]);
let isThinking = $state(false);
let chatContainer = $state<HTMLDivElement>();

const currentSession = $derived(
  sessions.find((s: any) => s.id === effectiveSessionId),
);

const userHat = $derived.by(() => {
  const role = authStore.user?.primary_role || "viewer";
  const roleLower = String(role).toLowerCase();
  if (roleLower === "admin" || roleLower === "super_admin") return "crown";
  if (roleLower === "teacher") return "graduation";
  if (roleLower === "registrar") return "firefighter";
  return "none";
});

const headerTitle = $derived.by(() => {
  if (messages.length === 0) return m.common_agent_title();
  if (currentSession) {
    const type = currentSession.agent_type;
    return type.charAt(0).toUpperCase() + type.slice(1);
  }
  return m.common_agent_title();
});

function scrollToBottom() {
  const container = chatContainer;
  if (container) {
    setTimeout(() => {
      container.scrollTo({
        top: container.scrollHeight,
        behavior: "smooth",
      });
    }, 50);
  }
}

function handleSuggestionClick(suggestion: string) {
  submitMessage(suggestion);
}

function handleClose() {
  uiStore.setAskOrbis(false);
  if (page.url.searchParams.has("ask_orbis")) {
    const newUrl = new URL(page.url.href);
    newUrl.searchParams.delete("ask_orbis");
    goto(newUrl.pathname + newUrl.search, {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
    });
  }
}

async function openStandaloneWindow() {
  if (isDesktopApp) {
    try {
      await commands.ai.agentOpenStandalone();
      uiStore.setAskOrbis(false);
    } catch (e: unknown) {
      console.error("Failed to open Ask Orbis standalone window:", e);
      toast.error(m.agent_toast_open_standalone_failed());
    }
  } else {
    window.open(
      `/agent?orgId=${currentOrgId}`,
      "_blank",
      "width=400,height=600",
    );
  }
}

async function handleNewChat(agentType: string = "default") {
  try {
    const newSession = await commands.ai.agentCreateSession(agentType);
    selectedSessionId = newSession.id;
    messages = [];
  } catch (e: unknown) {
    console.error("Failed to create Ask Orbis session:", e);
    toast.error(m.agent_toast_new_chat_failed());
  }
}

async function submitMessage(queryText?: string) {
  const text = (queryText || inputValue).trim();
  if (!text || isThinking) return;

  if (!queryText) {
    inputValue = "";
  }

  messages = [
    ...messages,
    { id: Math.random().toString(), role: "user", content: text },
  ];
  isThinking = true;
  scrollToBottom();

  try {
    let activeId = effectiveSessionId;
    if (!activeId) {
      const newSession = await commands.ai.agentCreateSession("greeting");
      selectedSessionId = newSession.id;
      activeId = newSession.id;
    }

    const assistantMessageId = Math.random().toString();
    messages = [
      ...messages,
      { id: assistantMessageId, role: "assistant", content: "" },
    ];

    const response = await commands.ai.agentQuery(
      activeId,
      text,
      aiContext.context || undefined,
      (chunk: string) => {
        const msgIndex = messages.findIndex((m) => m.id === assistantMessageId);
        if (msgIndex !== -1) {
          messages[msgIndex].content = chunk;
          scrollToBottom();
        }
      },
    );

    // Ensure final response is set in case streaming didn't fire properly or at the end
    const finalMsgIndex = messages.findIndex(
      (m) => m.id === assistantMessageId,
    );
    if (finalMsgIndex !== -1) {
      messages[finalMsgIndex].content = response;
    }
  } catch (e: unknown) {
    console.error("Ask Orbis query failed:", e);
    messages = [
      ...messages,
      {
        id: Math.random().toString(),
        role: "assistant",
        content: m.agent_error_response_failed(),
      },
    ];
  } finally {
    isThinking = false;
    scrollToBottom();
  }
}

function handleLinkClick(e: MouseEvent | null, href: string) {
  if (e) e.preventDefault();
  goto(href);
}
</script>

<div
  bind:this={sidebarDiv}
  class={isStandalone 
    ? "relative flex w-full h-full flex-col bg-card text-card-foreground overflow-hidden select-none"
    : "absolute " + (isRtl ? 'left-3.5 border-r' : 'right-3.5 border-l') + " top-3.5 bottom-3.5 flex shrink-0 flex-col bg-card text-card-foreground rounded-xl overflow-hidden border border-border shadow-2xl z-50 select-none"}
  style:width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:min-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:max-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  transition:fly={isStandalone ? undefined : { x: isRtl ? -400 : 400, duration: 250 }}
>
  <!-- Resize Handle on the left/right edge -->
  {#if !isStandalone}
    <Resizer
      viewport={sidebarDiv}
      direction="start"
      {minWidth}
      {maxWidth}
      defaultValue={pxToRem(425)}
      persistId="ask-orbis-sidebar-width"
      onWidth={(w) => (preferredWidth = w)}
    />
  {/if}

  <!-- Header Section -->
  <AgentHeader
    isStandalone={isStandalone}
    userHat={userHat}
    headerTitle={headerTitle}
    messagesLength={messages.length}
    currentOrgId={currentOrgId}
    viewMode={viewMode}
    onToggleView={() => viewMode = viewMode === "chat" ? "cockpit" : "chat"}
    onClose={handleClose}
    onOpenStandalone={openStandaloneWindow}
    onSelectSession={(id) => { selectedSessionId = id; viewMode = "cockpit"; }}
    onNewChat={() => handleNewChat("default")}
  />

  {#if effectiveSessionId && viewMode === "cockpit"}
    <!-- Session Activity Cockpit View -->
    <AgentCockpit
      orgId={currentOrgId}
      sessionId={effectiveSessionId}
      onBack={() => { viewMode = "chat"; }}
    />
  {:else}
    <!-- Content Section -->
    {#if messages.length === 0}
      <AgentEmptyState
        suggestions={suggestions}
        onselectSuggestion={handleSuggestionClick}
      />
    {:else}
      <ChatMessageList
        messages={messages}
        isThinking={isThinking}
        userName={userName}
        orgName={orgName}
        bind:chatContainer={chatContainer}
        onLinkClick={(href) => handleLinkClick(null, href)}
      />
    {/if}

    <!-- Footer Input Area -->
    <ChatInput
      bind:value={inputValue}
      isThinking={isThinking}
      messagesLength={messages.length}
      onsubmit={submitMessage}
      onnewchat={handleNewChat}
    />
  {/if}
</div>
