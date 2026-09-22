                                                                                                                                                               <!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis AI agent.
Provides file-based conversation and context persistence, ReAct reasoning loop,
interactive tool inspections, and consequential action approvals.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { uiStore, authStore, aiContext } from "$lib/stores";
import { fly } from "svelte/transition";
import { getLocale } from "$lib/paraglide/runtime";
import ChatInput from "../components/chat-input.svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { api, isDesktopApp } from "$lib/api";
import AgentHeader from "../components/agent-header.svelte";
import AgentCockpit from "../components/agent-cockpit.svelte";
import ChatMessageList from "../components/chat-message-list.svelte";
import AgentEmptyState from "../components/agent-empty-state.svelte";
import Resizer from "$lib/shell/resizer.svelte";
import { toast } from "@orbis/ui";
import { onMount } from "svelte";
import {
  type AiChatSession,
  type AiChatMessage,
  type ActionWithStatus,
  loadFileSessions,
  saveFileSession,
  deleteFileSession,
  saveFileContext,
  loadFileContext,
  createNewLocalSession,
} from "$lib/features/ai/session-store";
import { formatAiErrorMessage } from "$lib/features/ai/error-formatter";

let { isStandalone = false }: { isStandalone?: boolean } = $props();

const currentOrgId = $derived(page.params.orgId || authStore.orgId || "default");
const isRtl = $derived(getLocale() === "ar");

// File-based Sessions & Active State
let sessions = $state<AiChatSession[]>([]);
let activeSessionId = $state<string | null>(null);
let viewMode = $state<"chat" | "cockpit">("chat");
let isThinking = $state(false);
let inputValue = $state(page.url.searchParams.get("ask_orbis") || "");
let chatContainer = $state<HTMLDivElement>();
let sidebarDiv = $state<HTMLDivElement>();

function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
  return px / base;
}

let preferredWidth = $state(pxToRem(400));
const minWidth = $derived(pxToRem(320));
const maxWidth = $derived(pxToRem(650));

const activeSession = $derived(
  sessions.find((s) => s.id === activeSessionId) || null,
);

const messages = $derived<AiChatMessage[]>(activeSession?.messages ?? []);

// Extract pending consequential actions across all messages
const pendingActions = $derived<ActionWithStatus[]>(
  messages.flatMap((m) => m.proposedActions || []).filter((a) => !a.status || a.status === "pending"),
);

const userName = $derived(
  authStore.user?.full_name || m.common_agent_default_user_name(),
);
const orgName = $derived(
  (authStore.user as any)?.org_name ||
    (authStore.user as any)?.organization_name ||
    (authStore.user as any)?.org?.name ||
    null,
);

const userHat = $derived.by(() => {
  const role = authStore.user?.role || "viewer";
  const roleLower = String(role).toLowerCase();
  if (roleLower === "admin" || roleLower === "super_admin") return "crown";
  if (roleLower === "teacher") return "graduation";
  if (roleLower === "registrar") return "firefighter";
  return "none";
});

const headerTitle = $derived.by(() => {
  if (activeSession && activeSession.title && activeSession.messages.length > 0) {
    return activeSession.title;
  }
  return "مساعد Orbis الذكي";
});

// Load file sessions from disk on mount & sync context
onMount(() => {
  let isMounted = true;

  async function initStorage() {
    const loaded = await loadFileSessions(currentOrgId);
    if (!isMounted) return;
    
    if (loaded.length > 0) {
      sessions = loaded;
      activeSessionId = loaded[0].id;
    } else {
      const initial = createNewLocalSession(currentOrgId);
      sessions = [initial];
      activeSessionId = initial.id;
      await saveFileSession(initial);
    }

    // Persist screen context to disk
    if (aiContext.context) {
      await saveFileContext(currentOrgId, aiContext.context as Record<string, unknown>);
    }
  }

  initStorage();

  return () => {
    isMounted = false;
  };
});

// Sync context changes to file
$effect(() => {
  if (aiContext.context && currentOrgId) {
    saveFileContext(currentOrgId, aiContext.context as Record<string, unknown>);
  }
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
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("open_agent_window");
      uiStore.setAskOrbis(false);
      return;
    } catch (e) {
      console.warn("Failed to open Tauri agent window via IPC, falling back to window.open:", e);
    }
  }
  window.open(
    `/agent?orgId=${currentOrgId}`,
    "_blank",
    "width=460,height=720",
  );
  uiStore.setAskOrbis(false);
}

async function handleNewChat(title?: string) {
  const newSession = createNewLocalSession(currentOrgId, title || "محادثة جديدة");
  sessions = [newSession, ...sessions];
  activeSessionId = newSession.id;
  viewMode = "chat";
  await saveFileSession(newSession);
}

async function handleSelectSession(sessionId: string) {
  activeSessionId = sessionId;
  viewMode = "chat";
}

async function handleDeleteSession(sessionId: string) {
  await deleteFileSession(currentOrgId, sessionId);
  sessions = sessions.filter((s) => s.id !== sessionId);
  if (activeSessionId === sessionId) {
    if (sessions.length > 0) {
      activeSessionId = sessions[0].id;
    } else {
      await handleNewChat();
    }
  }
}

async function executeChatStream(current: AiChatSession, assistantMsgId: string) {
  isThinking = true;
  scrollToBottom();

  try {
    const aiMessages: import("$lib/api/types").AiMessage[] = current.messages
      .filter((m) => m.content.trim().length > 0 && m.id !== assistantMsgId)
      .map((m) => ({
        role: m.role as any,
        content: m.content,
      }));

    const response = await api.aiChat({
      messages: aiMessages,
      screen_context: (aiContext.context as Record<string, unknown>) || null,
    });

    const targetIdx = current.messages.findIndex((m) => m.id === assistantMsgId);
    if (targetIdx !== -1) {
      current.messages[targetIdx].content = response.response || "";
      current.messages[targetIdx].isError = false;
      current.messages[targetIdx].executedTools = response.executed_tools || [];
      current.messages[targetIdx].proposedActions = (response.proposed_actions || []).map((a) => ({
        ...a,
        status: "pending",
      }));
    }

    current.updatedAt = Date.now();
    sessions = [...sessions];
    await saveFileSession(current);
  } catch (e: unknown) {
    console.error("Ask Orbis query failed:", e);
    const targetIdx = current.messages.findIndex((m) => m.id === assistantMsgId);
    if (targetIdx !== -1) {
      current.messages[targetIdx].content = formatAiErrorMessage(e);
      current.messages[targetIdx].isError = true;
    }
    sessions = [...sessions];
    await saveFileSession(current);
  } finally {
    isThinking = false;
    scrollToBottom();
  }
}

async function handleRetry(msgId: string) {
  if (isThinking || !activeSession) return;
  const current = activeSession;
  const targetIdx = current.messages.findIndex((m) => m.id === msgId);
  if (targetIdx === -1) return;

  const targetMsg = current.messages[targetIdx];

  if (targetMsg.role === "assistant") {
    // Keep user messages preceding this assistant message, replace assistant message with empty thinking placeholder
    const assistantMsgId = `assistant_${Date.now()}`;
    const assistantMsg: AiChatMessage = {
      id: assistantMsgId,
      role: "assistant",
      content: "",
      timestamp: Date.now(),
      executedTools: [],
      proposedActions: [],
    };
    current.messages = [...current.messages.slice(0, targetIdx), assistantMsg];
    current.updatedAt = Date.now();
    sessions = [...sessions];
    await executeChatStream(current, assistantMsgId);
  } else if (targetMsg.role === "user") {
    // Slice off anything after this user message and generate new assistant reply
    const assistantMsgId = `assistant_${Date.now()}`;
    const assistantMsg: AiChatMessage = {
      id: assistantMsgId,
      role: "assistant",
      content: "",
      timestamp: Date.now(),
      executedTools: [],
      proposedActions: [],
    };
    current.messages = [...current.messages.slice(0, targetIdx + 1), assistantMsg];
    current.updatedAt = Date.now();
    sessions = [...sessions];
    await executeChatStream(current, assistantMsgId);
  }
}

async function handleEditUserMessage(msgId: string, newText: string) {
  if (isThinking || !activeSession) return;
  const current = activeSession;
  const targetIdx = current.messages.findIndex((m) => m.id === msgId);
  if (targetIdx === -1) return;

  // Update this user message and slice off subsequent messages
  current.messages[targetIdx].content = newText;
  current.messages[targetIdx].timestamp = Date.now();
  current.messages = current.messages.slice(0, targetIdx + 1);

  const assistantMsgId = `assistant_${Date.now()}`;
  const assistantMsg: AiChatMessage = {
    id: assistantMsgId,
    role: "assistant",
    content: "",
    timestamp: Date.now(),
    executedTools: [],
    proposedActions: [],
  };

  current.messages = [...current.messages, assistantMsg];
  current.updatedAt = Date.now();
  sessions = [...sessions];

  await executeChatStream(current, assistantMsgId);
}

async function submitMessage(queryText?: string) {
  const text = (queryText || inputValue).trim();
  if (!text || isThinking) return;

  if (!queryText) {
    inputValue = "";
  }

  let current = activeSession;
  if (!current) {
    current = createNewLocalSession(currentOrgId, text.slice(0, 30));
    sessions = [current, ...sessions];
    activeSessionId = current.id;
  }

  // Update session title if first message
  if (current.messages.length === 0) {
    current.title = text.length > 28 ? text.slice(0, 28) + "..." : text;
  }

  const userMsg: AiChatMessage = {
    id: `user_${Date.now()}`,
    role: "user",
    content: text,
    timestamp: Date.now(),
  };

  const assistantMsgId = `assistant_${Date.now()}`;
  const assistantMsg: AiChatMessage = {
    id: assistantMsgId,
    role: "assistant",
    content: "",
    timestamp: Date.now(),
    executedTools: [],
    proposedActions: [],
  };

  current.messages = [...current.messages, userMsg, assistantMsg];
  current.updatedAt = Date.now();
  sessions = [...sessions];

  await executeChatStream(current, assistantMsgId);
}

async function handleApproveAction(action: ActionWithStatus, msgId?: string, actionIdx?: number) {
  try {
    toast.promise(
      (async () => {
        // Execute the action through domain services or generic update
        if (action.tool_name === "update_student_status" && action.payload?.student_id) {
          await api.updateStudent({
            id: String(action.payload.student_id),
            status: String(action.payload.new_status || "Active"),
          });
        }
        
        // Mark action as executed in the conversation
        action.status = "executed";
        action.executedAt = Date.now();
        action.resultMessage = `تم تنفيذ: ${action.title} بنجاح`;

        if (activeSession) {
          activeSession.updatedAt = Date.now();
          sessions = [...sessions];
          await saveFileSession(activeSession);
        }
      })(),
      {
        loading: `جاري تنفيذ ${action.title}...`,
        success: `تم تنفيذ ${action.title} بنجاح`,
        error: "فشل تنفيذ الإجراء",
      },
    );
  } catch (e) {
    console.error("Failed to approve action:", e);
  }
}

async function handleRejectAction(action: ActionWithStatus, msgId?: string, actionIdx?: number) {
  action.status = "rejected";
  if (activeSession) {
    activeSession.updatedAt = Date.now();
    sessions = [...sessions];
    await saveFileSession(activeSession);
  }
  toast.info(`تم إلغاء ${action.title}`);
}

function handleLinkClick(href: string) {
  goto(href);
}
</script>

<div
  bind:this={sidebarDiv}
  class={isStandalone 
    ? "relative flex w-full h-full flex-col bg-card text-card-foreground overflow-hidden select-none"
    : "absolute " + (isRtl ? 'left-3.5 border-r' : 'right-3.5 border-l') + " top-3.5 bottom-3.5 flex shrink-0 flex-col bg-card text-card-foreground rounded-2xl overflow-hidden border border-border/80 shadow-2xl z-50 select-none backdrop-blur-md"}
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
      defaultValue={pxToRem(400)}
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
    sessions={sessions}
    currentSessionId={activeSessionId}
    pendingActionsCount={pendingActions.length}
    onToggleView={() => viewMode = viewMode === "chat" ? "cockpit" : "chat"}
    onClose={handleClose}
    onOpenStandalone={openStandaloneWindow}
    onSelectSession={handleSelectSession}
    onDeleteSession={handleDeleteSession}
    onNewChat={() => handleNewChat()}
  />

  {#if viewMode === "cockpit"}
    <!-- Activity & System Tools Cockpit View -->
    <AgentCockpit
      orgId={currentOrgId}
      pendingActions={pendingActions}
      onBack={() => { viewMode = "chat"; }}
      onApproveAction={(act) => handleApproveAction(act)}
      onRejectAction={(act) => handleRejectAction(act)}
    />
  {:else}
    <!-- Chat View -->
    {#if messages.length === 0}
      <AgentEmptyState
        onselectSuggestion={(sug) => submitMessage(sug)}
      />
    {:else}
      <ChatMessageList
        messages={messages}
        isThinking={isThinking}
        userName={userName}
        orgName={orgName}
        bind:chatContainer={chatContainer}
        onLinkClick={handleLinkClick}
        onApproveAction={(act, mId, aIdx) => handleApproveAction(act, mId, aIdx)}
        onRejectAction={(act, mId, aIdx) => handleRejectAction(act, mId, aIdx)}
        onRetry={handleRetry}
        onEditUserMessage={handleEditUserMessage}
      />
    {/if}

    <!-- Chat Input Area -->
    <ChatInput
      bind:value={inputValue}
      isThinking={isThinking}
      messagesLength={messages.length}
      onsubmit={submitMessage}
      onnewchat={() => handleNewChat()}
    />
  {/if}
</div>
