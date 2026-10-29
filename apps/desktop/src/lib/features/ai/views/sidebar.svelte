<!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis AI agent.
Provides file-based conversation and context persistence, ReAct reasoning loop,
interactive tool inspections, and consequential action approvals.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { api, isDesktopApp } from "$lib/api";
import type { AiCustomReportRequest } from "$lib/api/types";
import { formatAiErrorMessage } from "$lib/features/ai/error-formatter";
import {
  type ActionWithStatus,
  type AiChatMessage,
  type AiChatSession,
  createNewLocalSession,
  deleteFileSession,
  loadFileSessions,
  parseAiUiData,
  saveFileContext,
  saveFileSession,
} from "$lib/features/ai/session-store";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import Resizer from "$lib/shell/resizer.svelte";
import { academicContext, aiContext, authStore, uiStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import {
  applyTemplateAiDesign,
  getTemplateAiEditorContext,
} from "$lib/features/templates/template-ai-bridge";
import { onMount } from "svelte";
import { fly } from "svelte/transition";
import AgentCockpit from "../components/agent-cockpit.svelte";
import AgentEmptyState from "../components/agent-empty-state.svelte";
import AgentHeader from "../components/agent-header.svelte";
import ChatInput from "../components/chat-input.svelte";
import ChatMessageList from "../components/chat-message-list.svelte";

let {
  isStandalone = false,
  preferredWidth = $bindable(25),
}: {
  isStandalone?: boolean;
  preferredWidth?: number;
} = $props();

const currentOrgId = $derived(
  page.params.orgId || authStore.orgId || "default",
);

// File-based Sessions & Active State
let sessions = $state<AiChatSession[]>([]);
let activeSessionId = $state<string | null>(null);
let viewMode = $state<"chat" | "cockpit">("chat");
let isThinking = $state(false);
let inputValue = $state(page.url.searchParams.get("ask_orbis") || "");
const aiPreferences = $derived(uiStore.getAiProviderModel(currentOrgId));
const selectedModel = $derived(aiPreferences.model);
const selectedProvider = $derived(aiPreferences.provider);
let chatContainer = $state<HTMLDivElement>();
let sidebarDiv = $state<HTMLDivElement>();

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function pxToRem(px: number): number {
  if (typeof document === "undefined") return px / 16;
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
  return px / base;
}

const minWidth = $derived(pxToRem(320));
const maxWidth = $derived(pxToRem(1470));

const activeSession = $derived(
  sessions.find((s) => s.id === activeSessionId) || null,
);

const messages = $derived<AiChatMessage[]>(activeSession?.messages ?? []);

// Extract pending consequential actions across all messages
const pendingActions = $derived<ActionWithStatus[]>(
  messages
    .flatMap((m) => m.proposedActions || [])
    .filter((a) => !a.status || a.status === "pending"),
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

const headerTitle = $derived.by(() => {
  if (
    activeSession &&
    activeSession.title &&
    activeSession.messages.length > 0
  ) {
    return activeSession.title;
  }
  return "Orbis";
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
      console.warn(
        "Failed to open Tauri agent window via IPC, falling back to window.open:",
        e,
      );
    }
  }
  window.open(`/agent?orgId=${currentOrgId}`, "_blank", "width=460,height=720");
  uiStore.setAskOrbis(false);
}

async function handleNewChat(title?: string) {
  const newSession = createNewLocalSession(
    currentOrgId,
    title || m.ai_ui_new_chat(),
  );
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

async function executeChatStream(
  current: AiChatSession,
  assistantMsgId: string,
) {
  isThinking = true;
  scrollToBottom();

  try {
    const aiMessages: import("$lib/api/types").AiMessage[] = current.messages
      .filter((m) => m.content.trim().length > 0 && m.id !== assistantMsgId)
      .map((m) => ({
        role: m.role as any,
        content: m.content,
      }));

    const templateEditorContext = getTemplateAiEditorContext();
    const baseUiContext = aiContext.context
      ? {
          route: page.url.pathname,
          screen: aiContext.context.screen || aiContext.context.pageName,
          domain: aiContext.context.domain,
          entity_type: aiContext.context.entity_type,
          entity_id: aiContext.context.entity_id || aiContext.context.recordId,
          selected_entity_ids: aiContext.context.selected_entity_ids || [],
          active_filters: aiContext.context.active_filters,
          metadata: aiContext.context.metadata,
          period_id:
            aiContext.context.period_id ||
            academicContext.selectedPeriodId ||
            undefined,
          class_id:
            aiContext.context.class_id ||
            academicContext.selectedClassId ||
            undefined,
          subject_id:
            aiContext.context.subject_id ||
            academicContext.selectedSubjectId ||
            undefined,
        }
      : {
          route: page.url.pathname,
          period_id: academicContext.selectedPeriodId || undefined,
          class_id: academicContext.selectedClassId || undefined,
          subject_id: academicContext.selectedSubjectId || undefined,
        };
    const activeUiContext = templateEditorContext
      ? {
          ...baseUiContext,
          screen: "Template Editor",
          domain: "template",
          entity_type: templateEditorContext.entityType,
          metadata: { template_editor_active: true },
        }
      : baseUiContext;

    const activeAcademicContext = {
      calendar_id: academicContext.currentYear?.id || null,
      academic_calendar_name: academicContext.yearLabel || null,
      period_id: academicContext.selectedPeriodId || null,
      period_name: academicContext.activePeriod?.name || null,
      class_id: academicContext.selectedClassId || null,
      subject_id: academicContext.selectedSubjectId || null,
    };

    const response = await api.aiChat({
      messages: aiMessages,
      screen_context: templateEditorContext
        ? { template_editor: templateEditorContext }
        : (aiContext.context as Record<string, unknown>) || null,
      ui_context: activeUiContext,
      academic_context: activeAcademicContext,
      model: selectedModel ? selectedModel : undefined,
    });

    const targetIdx = current.messages.findIndex(
      (m) => m.id === assistantMsgId,
    );
    if (targetIdx !== -1) {
      current.messages[targetIdx].content = response.response || "";
      current.messages[targetIdx].isError = false;
      current.messages[targetIdx].executedTools = response.executed_tools || [];
      current.messages[targetIdx].uiData = Array.isArray(response.ui_data)
        ? response.ui_data
            .map((item) => parseAiUiData(item))
            .filter((item): item is NonNullable<typeof item> => item !== null)
        : [];
      current.messages[targetIdx].proposedActions = (
        response.proposed_actions || []
      ).map((a) => ({
        ...a,
        status: "pending",
      }));
    }

    current.updatedAt = Date.now();
    sessions = [...sessions];
    await saveFileSession(current);
  } catch (e: unknown) {
    console.error("Ask Orbis query failed:", e);
    const targetIdx = current.messages.findIndex(
      (m) => m.id === assistantMsgId,
    );
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

function handleApplyTemplateDesign(content: string) {
  try {
    applyTemplateAiDesign(content);
    toast.success(m.template_ai_design_applied());
  } catch (error) {
    toast.error(
      error instanceof Error ? error.message : m.template_ai_no_active_editor(),
    );
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
    current.messages = [
      ...current.messages.slice(0, targetIdx + 1),
      assistantMsg,
    ];
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

async function handleActionResponse(
  action: ActionWithStatus,
  status: "executed" | "rejected",
) {
  const persistAction = async () => {
    if (!activeSession) return;
    activeSession.updatedAt = Date.now();
    sessions = [...sessions];
    await saveFileSession(activeSession);
  };

  if (status === "rejected") {
    action.status = "rejected";
    action.executedAt = Date.now();
    action.resultMessage = m.ai_ui_action_cancelled_title({
      title: action.title,
    });
    toast.info(m.ai_ui_action_cancelled_title({ title: action.title }));
    await persistAction();
    return;
  }

  if (action.tool_name === "create_custom_report") {
    action.status = "processing";
    action.reportChecking = true;
    action.resultMessage = m.ai_ui_report_submitted();
    await persistAction();
    try {
      if (!action.reportJobId) {
        const request = parseCustomReportRequest(action.payload);
        if (!request) {
          throw new Error("The proposed report request is invalid.");
        }
        const created = await api.aiCreateCustomReport(request);
        action.reportJobId = created.job_id;
        action.reportOutputFormat = created.output_format;
        await persistAction();
      }

      let reportStatus = await api.getReportJobStatus(action.reportJobId);
      for (
        let attempt = 0;
        attempt < 60 &&
        (reportStatus.status === "pending" ||
          reportStatus.status === "processing");
        attempt += 1
      ) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
        reportStatus = await api.getReportJobStatus(action.reportJobId);
      }

      if (reportStatus.status === "completed") {
        action.status = "executed";
        action.executedAt = Date.now();
        action.resultMessage = m.ai_ui_report_ready();
        toast.success(m.ai_ui_report_ready());
      } else if (
        reportStatus.status === "failed" ||
        reportStatus.status === "cancelled"
      ) {
        const error = reportStatus.error_message || m.ai_ui_failed();
        action.status = "failed";
        action.executedAt = Date.now();
        action.resultMessage = m.ai_ui_report_creation_failed({ error });
        toast.error(action.resultMessage);
      } else {
        action.status = "processing";
        action.resultMessage = m.ai_ui_report_still_processing();
      }
    } catch (error) {
      const message = formatAiErrorMessage(error);
      if (action.reportJobId) {
        action.status = "processing";
        action.resultMessage = message;
        toast.error(message);
      } else {
        action.status = "failed";
        action.executedAt = Date.now();
        action.resultMessage = m.ai_ui_report_creation_failed({
          error: message,
        });
        toast.error(action.resultMessage);
      }
    } finally {
      action.reportChecking = false;
      await persistAction();
    }
    return;
  }

  action.status = "executed";
  action.executedAt = Date.now();
  action.resultMessage = m.ai_ui_action_approved({ title: action.title });
  toast.success(m.ai_ui_action_approved({ title: action.title }));
  await persistAction();
}

function parseCustomReportRequest(
  payload: unknown,
): AiCustomReportRequest | null {
  if (!isRecord(payload)) return null;
  const request = payload;
  const columns = Array.isArray(request.columns)
    ? request.columns.filter(
        (column): column is string => typeof column === "string",
      )
    : [];
  if (
    typeof request.report_code !== "string" ||
    typeof request.title !== "string" ||
    !Array.isArray(request.columns) ||
    columns.length !== request.columns.length ||
    columns.length === 0 ||
    typeof request.output_format !== "string"
  ) {
    return null;
  }
  const parameters = request.parameters;
  if (
    parameters !== undefined &&
    parameters !== null &&
    !isRecord(parameters)
  ) {
    return null;
  }
  const templateId = request.template_id;
  if (
    templateId !== undefined &&
    templateId !== null &&
    typeof templateId !== "string"
  ) {
    return null;
  }

  return {
    report_code: request.report_code,
    title: request.title,
    columns,
    parameters: isRecord(parameters) ? parameters : {},
    output_format: request.output_format,
    locale: typeof request.locale === "string" ? request.locale : getLocale(),
    template_id: typeof templateId === "string" ? templateId : null,
  };
}

async function downloadActionReport(action: ActionWithStatus) {
  if (!action.reportJobId) return;
  try {
    const bytes = await api.downloadReportFile(action.reportJobId);
    const format = action.reportOutputFormat || "pdf";
    const mimeTypes: Record<string, string> = {
      pdf: "application/pdf",
      xlsx: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
      csv: "text/csv;charset=utf-8",
      json: "application/json",
      html: "text/html;charset=utf-8",
    };
    const url = URL.createObjectURL(
      new Blob([bytes], {
        type: mimeTypes[format] || "application/octet-stream",
      }),
    );
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = `orbis-report-${action.reportJobId}.${format}`;
    anchor.click();
    window.setTimeout(() => URL.revokeObjectURL(url), 1000);
  } catch (error) {
    const message = formatAiErrorMessage(error);
    toast.error(message);
  }
}

function handleLinkClick(href: string) {
  goto(href);
}
</script>

<div
  bind:this={sidebarDiv}
  class={isStandalone
    ? "relative flex w-full h-full flex-col overflow-hidden"
    : "absolute end-3.5 top-3.5 bottom-3.5 flex shrink-0 flex-col overflow-hidden shadow-2xl z-50 select-none backdrop-blur-md"}
  style:width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:min-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  style:max-width={isStandalone ? "100%" : `${preferredWidth}rem`}
  transition:fly={isStandalone
    ? undefined
    : { y: 12, opacity: 0, duration: 200 }}
>
  <!-- Resize Handle on the start edge -->
  {#if !isStandalone}
    <Resizer
      viewport={sidebarDiv}
      edge="start"
      inside
      collapsible={false}
      {minWidth}
      {maxWidth}
      defaultValue={pxToRem(400)}
      persistId="ask-orbis-sidebar-width"
      onWidth={(w) => (preferredWidth = w)}
    />
  {/if}

  <!-- Header Section -->
  <AgentHeader
    {isStandalone}
    {headerTitle}
    messagesLength={messages.length}
    {currentOrgId}
    {viewMode}
    {sessions}
    currentSessionId={activeSessionId}
    pendingActionsCount={pendingActions.length}
    {selectedModel}
    {selectedProvider}
    onToggleView={() => (viewMode = viewMode === "chat" ? "cockpit" : "chat")}
    onClose={handleClose}
    onOpenStandalone={openStandaloneWindow}
    onSelectSession={handleSelectSession}
    onDeleteSession={handleDeleteSession}
    onNewChat={() => handleNewChat()}
    onOpenSettings={() => {
      goto(`/${currentOrgId}/ai-center/integrations`);
    }}
  />

  {#if viewMode === "cockpit"}
    <!-- Backend Connected Autopilot & System Tools Cockpit View -->
    <AgentCockpit
      orgId={currentOrgId}
      {pendingActions}
      onBack={() => {
        viewMode = "chat";
      }}
    />
  {:else}
    <!-- Chat View -->
    {#if messages.length === 0}
      <AgentEmptyState onselectSuggestion={(sug) => submitMessage(sug)} />
    {:else}
      <ChatMessageList
        {messages}
        {isThinking}
        {userName}
        {orgName}
        bind:chatContainer
        onLinkClick={handleLinkClick}
        onActionResponse={handleActionResponse}
        onReportDownload={downloadActionReport}
        onRetry={handleRetry}
        onEditUserMessage={handleEditUserMessage}
        onApplyTemplateDesign={handleApplyTemplateDesign}
      />
    {/if}

    <!-- Chat Input Area with Integrated Model Picker -->
    <ChatInput
      bind:value={inputValue}
      {isThinking}
      messagesLength={messages.length}
      orgId={currentOrgId}
      onsubmit={submitMessage}
      onnewchat={() => handleNewChat()}
      onOpenSettings={() => {
        goto(`/${currentOrgId}/ai-center/integrations`);
      }}
    />
  {/if}
</div>
