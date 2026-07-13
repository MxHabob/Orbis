<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Plus, ArrowRight, ArrowLeft } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { authStore } from "$lib/stores";
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import { goto } from "$app/navigation";
import ChatSuggestions from "./chat-suggestions.svelte";

let {
  value = $bindable(""),
  isThinking = false,
  messagesLength = 0,
  isRtl = false,
  onsubmit,
  onnewchat,
}: {
  value?: string;
  isThinking?: boolean;
  messagesLength?: number;
  isRtl?: boolean;
  onsubmit: (query: string) => void;
  onnewchat: () => void;
} = $props();

let inputEl = $state<HTMLDivElement | null>(null);
let suggestionsContainer = $state<HTMLDivElement | null>(null);

const currentOrgId = $derived(page.params.orgId || authStore.orgId || "");

// TanStack query caching for autocomplete using reactive config closures
const enrollmentsQuery = createQuery({
  get queryKey() {
    return ["enrollments", currentOrgId];
  },
  queryFn: () => commands.academic.listEnrollments(currentOrgId),
  get enabled() {
    return !!currentOrgId;
  },
  staleTime: 2 * 60 * 1000,
});

const usersQuery = createQuery({
  get queryKey() {
    return ["users", currentOrgId];
  },
  queryFn: () => commands.iam.listUsers(currentOrgId),
  get enabled() {
    return !!currentOrgId;
  },
  staleTime: 5 * 60 * 1000,
});

const students = $derived($enrollmentsQuery.data || []);
const users = $derived($usersQuery.data || []);

// Autocomplete State
let showSuggestions = $state(false);
let trigger = $state<"/" | "@" | "">("");
let triggerIndex = $state(-1);
let triggerQuery = $state("");
let selectedIndex = $state(0);

const slashPages = [
  { label: "الطلاب", path: "/students", key: "students", cleanLabel: "الطلاب" },
  {
    label: "الصفوف الدراسية",
    path: "/academics/classes",
    key: "classes",
    cleanLabel: "الصفوف الدراسية",
  },
  {
    label: "المواد الدراسية",
    path: "/settings/subjects",
    key: "subjects",
    cleanLabel: "المواد الدراسية",
  },
  {
    label: "الرسوم الدراسية",
    path: "/finance/fees",
    key: "fees",
    cleanLabel: "الرسوم الدراسية",
  },
  {
    label: "المدفوعات والمقبوضات",
    path: "/finance/payments",
    key: "payments",
    cleanLabel: "المدفوعات والمقبوضات",
  },
  {
    label: "الحسابات والصناديق",
    path: "/finance/treasuries",
    key: "treasuries",
    cleanLabel: "الحسابات والصناديق",
  },
  {
    label: "التقارير",
    path: "/reports",
    key: "reports",
    cleanLabel: "التقارير",
  },
  {
    label: "القوالب",
    path: "/templates",
    key: "templates",
    cleanLabel: "القوالب",
  },
  {
    label: "الإعدادات العامة",
    path: "/settings",
    key: "settings",
    cleanLabel: "الإعدادات العامة",
  },
  {
    label: "الجدول الدراسي",
    path: "/academics/schedule",
    key: "schedule",
    cleanLabel: "الجدول الدراسي",
  },
];

const filteredSuggestions = $derived.by(() => {
  if (!showSuggestions) return [];
  const q = triggerQuery.toLowerCase().trim();

  if (trigger === "/") {
    return slashPages
      .filter((p) => p.label.toLowerCase().includes(q))
      .map((p) => ({
        id: `slash-${p.key}`,
        label: p.label,
        subtitle: `/${currentOrgId}${p.path}`,
        path: `/${currentOrgId}${p.path}`,
        insertLabel: p.cleanLabel,
      }));
  } else if (trigger === "@") {
    const matchedStudents = students
      .filter((s: any) => {
        const name = (
          s.full_name ||
          s.name_ar ||
          s.name_en ||
          ""
        ).toLowerCase();
        const num = (s.student_number || "").toLowerCase();
        return name.includes(q) || num.includes(q);
      })
      .map((s: any) => {
        const name = s.full_name || s.name_ar || s.name_en || "Student";
        return {
          id: `student-${s.id}`,
          label: name,
          subtitle: isRtl
            ? `طالب (رقم: ${s.student_number || "—"})`
            : `Student (ID: ${s.student_number || "—"})`,
          path: `/${currentOrgId}/students/${s.id}/grades`,
        };
      });

    const matchedUsers = users
      .filter((u: any) => {
        const name = (
          u.full_name ||
          u.name_ar ||
          u.name_en ||
          ""
        ).toLowerCase();
        const email = (u.email || "").toLowerCase();
        return name.includes(q) || email.includes(q);
      })
      .map((u: any) => {
        const name =
          u.full_name || u.name_ar || u.name_en || m.agent_default_user_name();
        const roleStr =
          u.primary_role || u.role || m.agent_default_role_staff();
        return {
          id: `user-${u.id}`,
          label: name,
          subtitle: m.agent_user_role_label({ role: roleStr }),
          path: `/${currentOrgId}/settings/users/${u.id}/profile`,
        };
      });

    return [...matchedStudents, ...matchedUsers].slice(0, 10);
  }

  return [];
});

function getActiveMentionOrSlash(text: string, cursorIndex: number) {
  let index = cursorIndex - 1;
  while (index >= 0) {
    const char = text[index];
    if (char === "@" || char === "/") {
      if (index === 0 || /\s/.test(text[index - 1])) {
        const query = text.substring(index + 1, cursorIndex);
        if (!/\s/.test(query)) {
          return { trigger: char, query, index };
        }
      }
      break;
    }
    if (/\s/.test(char)) {
      break;
    }
    index--;
  }
  return null;
}

function getPlaintextFromHtml(html: string): string {
  if (typeof document === "undefined") return html;
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;

  const spans = tempDiv.querySelectorAll("span[data-type='mention']");
  spans.forEach((span) => {
    const isMention = span.getAttribute("data-mention") === "true";
    const label = span.textContent || "";
    const textNode = document.createTextNode(
      isMention ? `[@${label}]` : `[${label}]`,
    );
    span.parentNode?.replaceChild(textNode, span);
  });

  return tempDiv.textContent || "";
}

function getCaretPosition(element: HTMLElement): number {
  const selection = window.getSelection();
  if (!selection || selection.rangeCount === 0) return 0;

  const range = selection.getRangeAt(0);
  if (!element.contains(range.endContainer)) return 0;

  const preCaretRange = range.cloneRange();
  preCaretRange.selectNodeContents(element);

  try {
    preCaretRange.setEnd(range.endContainer, range.endOffset);
    return preCaretRange.toString().length;
  } catch (err) {
    return element.textContent?.length || 0;
  }
}

function checkTrigger(target: HTMLDivElement) {
  const text = target.textContent || "";
  const cursorIndex = getCaretPosition(target);
  const active = getActiveMentionOrSlash(text, cursorIndex);

  if (active) {
    showSuggestions = true;
    trigger = active.trigger as "/" | "@";
    triggerQuery = active.query;
    triggerIndex = active.index;

    const filteredLength = filteredSuggestions.length;
    if (selectedIndex >= filteredLength) {
      selectedIndex = Math.max(0, filteredLength - 1);
    }
  } else {
    showSuggestions = false;
    trigger = "";
    triggerQuery = "";
    triggerIndex = -1;
    selectedIndex = 0;
  }
}

function insertHtmlAtCaret(html: string) {
  const sel = window.getSelection();
  if (sel && sel.rangeCount > 0) {
    const range = sel.getRangeAt(0);
    range.deleteContents();

    const el = document.createElement("div");
    el.innerHTML = html;
    const frag = document.createDocumentFragment();
    let node;
    let lastNode;
    while ((node = el.firstChild)) {
      lastNode = frag.appendChild(node);
    }

    range.insertNode(frag);

    if (lastNode) {
      const nextRange = range.cloneRange();
      nextRange.setStartAfter(lastNode);
      nextRange.collapse(true);
      sel.removeAllRanges();
      sel.addRange(nextRange);
    }
  } else if (inputEl) {
    inputEl.innerHTML += html;
  }
}

function selectSuggestion(item: {
  label: string;
  path: string;
  insertLabel?: string;
}) {
  if (!inputEl) return;

  const sel = window.getSelection();
  if (sel && sel.rangeCount > 0) {
    const range = sel.getRangeAt(0);
    const node = range.startContainer;
    const offset = range.startOffset;

    const queryLength = triggerQuery.length + 1;
    const startReplaceOffset = Math.max(0, offset - queryLength);

    if (node.nodeType === Node.TEXT_NODE) {
      node.textContent =
        node.textContent!.substring(0, startReplaceOffset) +
        node.textContent!.substring(offset);

      range.setStart(node, startReplaceOffset);
      range.collapse(true);
    }
  }

  const isMention = trigger === "@";
  const insertText = item.insertLabel || item.label;
  const displayLabel = isMention ? `@${insertText}` : insertText;

  const badgeHtml = `<span data-type="mention" data-mention="${isMention}" data-path="${item.path}" contenteditable="false" class="border border-transparent border-b-dotted border-b-foreground/60 rounded px-1.5 py-0.5 bg-transparent hover:bg-card hover:border-border/60 hover:border-solid text-foreground transition-all duration-150 inline mx-0.5 cursor-pointer select-none font-semibold">${displayLabel}</span>&nbsp;`;

  insertHtmlAtCaret(badgeHtml);

  showSuggestions = false;
  trigger = "";
  triggerQuery = "";
  triggerIndex = -1;
  selectedIndex = 0;

  value = getPlaintextFromHtml(inputEl.innerHTML);

  setTimeout(() => {
    if (inputEl) inputEl.focus();
  }, 10);
}

function handleInput(e: Event) {
  const target = e.currentTarget as HTMLDivElement;
  value = getPlaintextFromHtml(target.innerHTML);
  checkTrigger(target);
}

function handleInputClick(e: MouseEvent) {
  const target = e.target as HTMLElement;
  const mentionSpan = target.closest('span[data-type="mention"]');
  if (mentionSpan) {
    const path = mentionSpan.getAttribute("data-path");
    if (path) {
      goto(path);
      return;
    }
  }

  const currentTarget = e.currentTarget as HTMLDivElement;
  checkTrigger(currentTarget);
}

function handleKeyUp(e: KeyboardEvent) {
  const target = e.currentTarget as HTMLDivElement;
  checkTrigger(target);
}

function handleSubmit(e: SubmitEvent | KeyboardEvent | Event) {
  if (e) e.preventDefault();
  const trimmed = value.trim();
  if (trimmed && !isThinking) {
    onsubmit(trimmed);
    value = "";
    if (inputEl) {
      inputEl.innerHTML = "";
    }
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (showSuggestions && filteredSuggestions.length > 0) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % filteredSuggestions.length;
      return;
    }
    if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex =
        (selectedIndex - 1 + filteredSuggestions.length) %
        filteredSuggestions.length;
      return;
    }
    if (e.key === "Enter" || e.key === "Tab") {
      e.preventDefault();
      selectSuggestion(filteredSuggestions[selectedIndex]);
      return;
    }
    if (e.key === "Escape") {
      e.preventDefault();
      showSuggestions = false;
      return;
    }
  }

  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSubmit(e);
  }
}

function handleNewChat() {
  value = "";
  if (inputEl) {
    inputEl.innerHTML = "";
  }
  onnewchat();
}

$effect(() => {
  if (value === "" && inputEl && inputEl.innerHTML !== "") {
    inputEl.innerHTML = "";
  }
});

$effect(() => {
  if (showSuggestions && suggestionsContainer) {
    const activeEl = suggestionsContainer.querySelector(
      '[aria-selected="true"]',
    );
    if (activeEl) {
      activeEl.scrollIntoView({ block: "nearest", behavior: "smooth" });
    }
  }
});
</script>

<div class="relative p-3.5 shrink-0 select-none bg-card">
  <ChatSuggestions 
    {filteredSuggestions} 
    {selectedIndex} 
    {showSuggestions} 
    bind:suggestionsContainer 
    onSelect={selectSuggestion} 
    onHover={(idx) => (selectedIndex = idx)} 
  />

  <form onsubmit={handleSubmit} class="flex flex-col gap-2">
    <div
      bind:this={inputEl}
      contenteditable={!isThinking}
      onkeydown={handleKeyDown}
      oninput={handleInput}
      onkeyup={handleKeyUp}
      onclick={handleInputClick}
      data-placeholder={m.agent_input_placeholder()}
      class="contenteditable-input input-sunken dark:bg-input/30 rounded-md px-2.5 py-2 text-xs placeholder:text-muted-foreground block w-full min-h-16 max-h-[120px] overflow-y-auto scrollbar-hide outline-none break-words text-start whitespace-pre-wrap leading-relaxed"
    ></div>
    
    <div class="flex items-center justify-between px-1">
      <!-- Input Tools (Left-side/Right-side depending on RTL) -->
      <div class="flex items-center gap-1.5">
        <!-- Clear/New Chat button -->
        <Button
          variant="ghost"
          size="icon-xs"
          onclick={handleNewChat}
          type="button"
          class="h-7 w-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/50 cursor-pointer"
          title={m.agent_input_new_chat()}
          disabled={messagesLength === 0}
        >
          <Plus size={14} />
        </Button>
        
        <!-- Tip/Helper text or indicator -->
        {#if value.trim().length > 0}
          <span class="text-[9px] text-muted-foreground/60 font-medium select-none">
            {value.length} ch
          </span>
        {:else}
          <span class="text-[9px] text-muted-foreground/50 font-medium select-none">
            {m.agent_input_newline_tip()}
          </span>
        {/if}
      </div>

      <!-- Submit Button -->
      <Button
        type="submit"
        disabled={!value.trim() || isThinking}
        class="h-7 w-7 rounded-lg bg-primary hover:bg-primary/95 text-primary-foreground disabled:bg-muted/40 disabled:text-muted-foreground/60 disabled:opacity-50 cursor-pointer flex items-center justify-center transition-colors shrink-0"
      >
        {#if isRtl}
          <ArrowLeft size={13} strokeWidth={2.5} />
        {:else}
          <ArrowRight size={13} strokeWidth={2.5} />
        {/if}
      </Button>
    </div>
  </form>
</div>

<style>
  .contenteditable-input:empty::before {
    content: attr(data-placeholder);
    color: hsl(var(--muted-foreground));
    opacity: 0.7;
    pointer-events: none;
    display: inline-block;
  }
</style>