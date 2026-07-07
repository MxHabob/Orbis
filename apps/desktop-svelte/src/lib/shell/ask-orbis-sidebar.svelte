<!--
@component AskOrbisSidebar
A resizable right-hand sidebar overlay for Ask Orbis agent, mirroring the Seer design.
Allows users to ask questions, select prompt suggestions, and collapse/resize.
-->
<script lang="ts">
import { uiStore, authStore } from "$lib/stores";
import Resizer from "./resizer.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "$lib/features/onboarding/state.svelte";
import {
  X,
  MoreHorizontal,
  Columns,
  Plus,
  ArrowRight,
  ArrowLeft,
} from "lucide-svelte";
import { fly } from "svelte/transition";
import { getLocale } from "$lib/paraglide/runtime";
import { isDesktopApp } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import ChatHistoryPopover from "./chat-history-popover.svelte";
import ChatInput from "./chat-input.svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import { Badge } from "@orbis/ui-svelte/badge";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui-svelte/tabs";
import {
  createAgentSessionsQuery,
  createAgentSessionLogsQuery,
  useApproveAgentAction,
  useRejectAgentAction,
} from "$lib/features/ai/queries";
import { extractErrorMessage } from "$lib/core/ipc/error";
import { l, formatDate } from "$lib/core/i18n";

let { isStandalone = false }: { isStandalone?: boolean } = $props();

const currentOrgId = $derived(page.params.orgId || authStore.orgId || "");

// TanStack query caching for autocomplete using reactive config closures
const enrollmentsQuery = createQuery(() => ({
  queryKey: ["enrollments", currentOrgId],
  queryFn: () => commands.enrollments.getAll(currentOrgId),
  enabled: !!currentOrgId,
  staleTime: 2 * 60 * 1000,
}));

const usersQuery = createQuery(() => ({
  queryKey: ["users", currentOrgId],
  queryFn: () => commands.users.getUsers(currentOrgId),
  enabled: !!currentOrgId,
  staleTime: 5 * 60 * 1000,
}));

const students = $derived($enrollmentsQuery.data || []);
const users = $derived($usersQuery.data || []);

// Agent Session & Action Log Cockpit States
let selectedSessionId = $state<string | null>(null);

const sessionsQuery = createAgentSessionsQuery(currentOrgId);
const sessions = $derived($sessionsQuery.data ?? []);

const effectiveSessionId = $derived(selectedSessionId || (sessions.length > 0 ? sessions[0].id : null));

const logsQuery = createAgentSessionLogsQuery(currentOrgId, effectiveSessionId || "");
const logs = $derived($logsQuery.data ?? []);

const approveActionMutation = useApproveAgentAction();
const rejectActionMutation = useRejectAgentAction();

async function handleApprove(actionId: string) {
  try {
    toast.promise($approveActionMutation.mutateAsync({
      orgId: currentOrgId,
      actionLogId: actionId,
      approvedBy: authStore.user?.id || "Admin",
    }), {
      loading: m.agent_toast_approving(),
      success: m.agent_toast_approve_success(),
      error: (e) => extractErrorMessage(e) || m.agent_toast_approve_failed(),
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleReject(actionId: string) {
  const reason = prompt(m.agent_rejection_prompt_reason());
  if (reason === null) return;
  
  try {
    toast.promise($rejectActionMutation.mutateAsync({
      orgId: currentOrgId,
      actionLogId: actionId,
      rejectedBy: authStore.user?.id || "Admin",
      reason: reason || m.agent_rejection_reason_fallback(),
    }), {
      loading: m.agent_toast_rejecting(),
      success: m.agent_toast_reject_success(),
      error: (e) => extractErrorMessage(e) || m.agent_toast_reject_failed(),
    });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

const slashPages = [
  { label: "الطلاب", path: "/students", cleanLabel: "الطلاب" },
  {
    label: "الصفوف الدراسية",
    path: "/academics/classes",
    cleanLabel: "الصفوف الدراسية",
  },
  {
    label: "المواد الدراسية",
    path: "/settings/subjects",
    cleanLabel: "المواد الدراسية",
  },
  {
    label: "الرسوم الدراسية",
    path: "/finance/fees",
    cleanLabel: "الرسوم الدراسية",
  },
  {
    label: "المدفوعات والمقبوضات",
    path: "/finance/payments",
    cleanLabel: "المدفوعات والمقبوضات",
  },
  {
    label: "الحسابات والصناديق",
    path: "/finance/treasuries",
    cleanLabel: "الحسابات والصناديق",
  },
  { label: "التقارير", path: "/reports", cleanLabel: "التقارير" },
  { label: "القوالب", path: "/templates", cleanLabel: "القوالب" },
  {
    label: "الإعدادات العامة",
    path: "/settings",
    cleanLabel: "الإعدادات العامة",
  },
  {
    label: "الجدول الدراسي",
    path: "/academics/schedule",
    cleanLabel: "الجدول الدراسي",
  },
];

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
  authStore.user?.full_name || m.agent_default_user_name(),
);
const orgName = $derived(
  (authStore.user as any)?.org_name ||
    (authStore.user as any)?.organization_name ||
    (authStore.user as any)?.org?.name ||
    null,
);

let inputValue = $state(page.url.searchParams.get("ask_orbis") || "");
const suggestions = $derived([
  m.agent_suggest_1(),
  m.agent_suggest_2(),
  m.agent_suggest_3(),
]);

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
}

let messages = $state<Message[]>([]);
let isThinking = $state(false);
let chatContainer = $state<HTMLDivElement>();

function scrollToBottom() {
  if (chatContainer) {
    setTimeout(() => {
      chatContainer.scrollTo({
        top: chatContainer.scrollHeight,
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
  if (!isDesktopApp()) return;
  try {
    const { commands } = await import("$lib/core/ipc");

    // Close the current sidebar overlay
    uiStore.setAskOrbis(false);

    // Create standalone window securely via Rust backend
    await commands.window.openAgent();
  } catch (e: any) {
    console.error("Failed to open standalone window:", e);
    toast.error(`Failed to open window: ${e?.message || e}`);
  }
}

async function submitMessage(query: string) {
  if (!query.trim()) return;

  inputValue = "";

  // Clear query parameter from URL so it doesn't persist in history
  if (page.url.searchParams.has("ask_orbis")) {
    const newUrl = new URL(page.url.href);
    newUrl.searchParams.delete("ask_orbis");
    goto(newUrl.pathname + newUrl.search, {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
    });
  }

  const isNewChat = messages.length === 0;

  messages.push({
    id: Math.random().toString(36).substring(7),
    role: "user",
    content: query,
  });

  scrollToBottom();
  isThinking = true;

  // Simulate a realistic agent thinking time
  await new Promise((resolve) => setTimeout(resolve, 800));

  isThinking = false;

  messages.push({
    id: Math.random().toString(36).substring(7),
    role: "assistant",
    content: "",
  });

  scrollToBottom();
}

function handleNewChat() {
  messages = [];
}

function handleSelectHistoryItem(messagesToLoad: Message[]) {
  messages = [...messagesToLoad];
  scrollToBottom();
}

interface ParsedLinkPart {
  type: "text" | "link";
  content?: string;
  label?: string;
  href?: string;
}

function parseMarkdownLinks(text: string): ParsedLinkPart[] {
  const regex = /\[([^\]]+)\]/g;
  const parts: ParsedLinkPart[] = [];
  let lastIndex = 0;
  let match;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push({
        type: "text",
        content: text.substring(lastIndex, match.index),
      });
    }

    const fullLabel = match[1];
    let isMention = false;
    let label = fullLabel;

    if (fullLabel.startsWith("@")) {
      isMention = true;
      label = fullLabel.substring(1);
    }

    let resolvedHref = "";

    if (isMention) {
      // Look up student by name
      const studentMatch = students.find(
        (s: any) =>
          (s.full_name || s.name_ar || s.name_en || "").toLowerCase() ===
          label.toLowerCase(),
      );
      if (studentMatch) {
        resolvedHref = `/${currentOrgId}/students/${studentMatch.id}/grades`;
      } else {
        // Look up user by name
        const userMatch = users.find(
          (u: any) =>
            (u.full_name || u.name_ar || u.name_en || "").toLowerCase() ===
            label.toLowerCase(),
        );
        if (userMatch) {
          resolvedHref = `/${currentOrgId}/settings/users/${userMatch.id}/profile`;
        }
      }
    } else {
      // Look up slash pages by label
      const matchedPage = slashPages.find(
        (p) =>
          p.cleanLabel.toLowerCase() === label.toLowerCase() ||
          p.label.toLowerCase() === label.toLowerCase(),
      );
      if (matchedPage) {
        resolvedHref = `/${currentOrgId}${matchedPage.path}`;
      }
    }

    if (resolvedHref) {
      parts.push({
        type: "link",
        label: fullLabel, // Keep the @ if it was a mention, or clean label
        href: resolvedHref,
      });
    } else {
      // Not resolved to a route, treat as normal text but keep brackets
      parts.push({ type: "text", content: match[0] });
    }

    lastIndex = regex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push({ type: "text", content: text.substring(lastIndex) });
  }

  return parts.length > 0 ? parts : [{ type: "text", content: text }];
}

function handleLinkClick(e: MouseEvent, href: string) {
  e.preventDefault();
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
      direction={isRtl ? "left" : "right"}
      {minWidth}
      {maxWidth}
      defaultValue={pxToRem(425)}
      persistId="ask-orbis-sidebar-width"
      onWidth={(w) => (preferredWidth = w)}
    />
  {/if}

  <!-- Header Section -->
  <div class="flex items-center justify-between px-3.5 h-11 shrink-0 border-b border-border select-none bg-card">
    <div class="flex items-center gap-2">
      <!-- Close Button -->
      {#if !isStandalone}
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={handleClose}
          class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Close sidebar"
        >
          <X size={14} strokeWidth={2} />
        </Button>
      {/if}
      
      <!-- Title -->
      <span class="text-[12px] font-semibold tracking-wide text-foreground flex items-center gap-1.5">
        {m.agent_title()} 🧙
      </span>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-1">
      <Button
        variant="ghost"
        size="icon-xs"
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="More options"
      >
        <MoreHorizontal size={14} />
      </Button>
      
      {#if !isStandalone}
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={openStandaloneWindow}
          class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Split layout"
        >
          <Columns size={13} />
        </Button>
      {/if}

      <ChatHistoryPopover
        orgId={currentOrgId}
        onSelectSession={(sessionId) => selectedSessionId = sessionId}
      />

      <Button
        variant="ghost"
        size="icon-xs"
        onclick={handleNewChat}
        class="w-6 h-6 flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="New Chat"
      >
        <Plus size={14} />
      </Button>
    </div>
  </div>

  {#if effectiveSessionId}
    <!-- Session Activity Cockpit View -->
    <div class="px-4 py-2 border-b border-border bg-muted/20 flex items-center justify-between shrink-0 select-none">
      <Button 
        variant="ghost" 
        size="xs" 
        class="text-[10px] h-7 font-semibold cursor-pointer"
        onclick={() => { selectedSessionId = null; }}
      >
        ← {m.agent_back_to_chat()}
      </Button>
      <span class="text-xs font-mono font-bold text-primary flex items-center gap-1">
        #{effectiveSessionId.substring(0, 8)}
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
            {m.agent_session_logs_load_failed()}
          </p>
        </div>
      {:else if logs.length === 0}
        <div class="text-center py-8 text-xs text-muted-foreground">
          {m.agent_session_logs_empty()}
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
                    <Badge variant="secondary" class="text-[8px] px-1 bg-amber-500/10 text-amber-600 dark:text-amber-400 border-none">Pending</Badge>
                  {:else if log.status === "approved" || log.status === "executed"}
                    <Badge variant="lime" class="text-[8px] px-1">Approved</Badge>
                  {:else if log.status === "rejected"}
                    <Badge variant="outline" class="text-[8px] px-1 text-destructive border-destructive/20">Rejected</Badge>
                  {:else}
                    <Badge variant="outline" class="text-[8px] px-1 text-muted-foreground">{log.status}</Badge>
                  {/if}
                </div>
              </div>

              <p class="text-[10px] text-muted-foreground leading-normal">
                <strong class="text-foreground">{m.agent_log_reasoning()}</strong> {log.reasoning}
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
                    {m.agent_log_action_approve()}
                  </Button>
                  <Button
                    variant="destructive"
                    size="xs"
                    class="flex-1 text-[10px] h-7 cursor-pointer"
                    onclick={() => handleReject(log.id)}
                  >
                    {m.agent_log_action_reject()}
                  </Button>
                </div>
              {:else if log.status === "rejected" && log.rejection_reason}
                <div class="text-[9px] text-destructive bg-destructive/5 p-1.5 rounded border border-destructive/10">
                  <strong>{m.agent_log_rejection_reason()}</strong> {log.rejection_reason}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <!-- Content Section -->
    <div 
      bind:this={chatContainer}
      class="flex-1 flex flex-col p-4 overflow-y-auto scrollbar-hide select-text"
    >
      {#if messages.length === 0}
        <div class="flex-1 flex flex-col items-center justify-center px-6 pb-8 select-none">
          <!-- Eye-Triangle Agent Logo -->
          <div class="text-muted-foreground mb-6 drop-shadow-[0_0_15px_rgba(0,0,0,0.02)]">
            <svg width="68" height="68" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 3L2 20h20L12 3z" stroke-linejoin="round" />
              <path d="M7.2 14.2c2.4-2.2 7.2-2.2 9.6 0" />
              <path d="M7.2 14.2c2.4 2.2 7.2 2.2 9.6 0" />
              <circle cx="12" cy="14.2" r="2.2" fill="currentColor" />
            </svg>
          </div>

          <!-- Subtitle -->
          <p class="text-[13px] text-foreground/90 text-center font-medium max-w-[280px] leading-relaxed mb-6 select-none">
            {m.agent_subtitle()}
          </p>

          <!-- Suggestion Buttons -->
          <div class="flex flex-col gap-2 w-full max-w-[340px]">
            {#each suggestions as sug}
              <Button
                variant="outline"
                onclick={() => handleSuggestionClick(sug)}
                class="w-full text-center py-3 px-4 text-[11.5px] font-medium text-foreground/80 bg-card border border-border/80 hover:border-zinc-400 dark:hover:border-zinc-600 hover:bg-muted/40 hover:text-foreground rounded-lg cursor-pointer transition-all duration-150 active:scale-[0.99] leading-relaxed whitespace-normal h-auto wrap-break-word shadow-sm"
              >
                {sug}
              </Button>
            {/each}
          </div>
        </div>
      {:else}
        <div class="flex flex-col gap-5 w-full">
          {#each messages as msg}
            {#if msg.role === 'user'}
              <!-- User message bubble on the right -->
              <div class="flex justify-end w-full">
                <div class="max-w-[85%] bg-muted text-foreground px-4 py-2.5 rounded-2xl text-[12.5px] leading-relaxed break-words font-medium shadow-sm">
                  {#each parseMarkdownLinks(msg.content) as part}
                    {#if part.type === 'link'}
                      <a
                        href={part.href}
                        onclick={(e) => handleLinkClick(e, part.href!)}
                        class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                      >
                        {part.label}
                      </a>
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
                        <a
                          href={part.href}
                          onclick={(e) => handleLinkClick(e, part.href!)}
                          class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold align-baseline"
                        >
                          {part.label}
                        </a>
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
      {/if}
    </div>

    <!-- Footer Input Area -->
    <ChatInput
      bind:value={inputValue}
      isThinking={isThinking}
      messagesLength={messages.length}
      isRtl={isRtl}
      onsubmit={submitMessage}
      onnewchat={handleNewChat}
    />
  {/if}
</div>
