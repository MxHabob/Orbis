<!--
  @component SupportDialog
  Builds a prefilled support email with user-provided details and reviewable device diagnostics.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api/client";
import { isDesktopApp } from "$lib/api";
import type { AppInfo } from "$lib/api/types";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  QuestionMarkCircle,
  Check,
  ChevronDown,
  ChevronUp,
} from "@orbis/icons";

let {
  open = $bindable(false),
}: {
  open?: boolean;
} = $props();

let info = $state<AppInfo | null>(null);
let subject = $state("");
let category = $state("technical_support");
let email = $state("");
let message = $state("");
let isSubmitting = $state(false);
let isComposeOpened = $state(false);
let isCopied = $state(false);
let submitError = $state<string | null>(null);
let composeUrl = $state<string | null>(null);
let showDiagnosticsPreview = $state(false);
let isLoadingInfo = $state(false);
let infoError = $state<string | null>(null);

const categories = $derived([
  { value: "technical_support", label: m.ui_system_support_category_technical() },
  { value: "onboarding_setup", label: m.ui_system_support_category_onboarding() },
  { value: "finance_accounting", label: m.ui_system_support_category_finance() },
  { value: "academics_grading", label: m.ui_system_support_category_academics() },
  { value: "feature_request", label: m.ui_system_support_category_feature_request() },
  { value: "other", label: m.ui_system_support_category_general() },
]);

const selectedCat = $derived(categories.find((c) => c.value === category));

const diagnosticPayload = $derived({
  application: info?.name,
  version: info?.version,
  build_profile: info?.build_profile,
  target_os: info?.target_os,
  target_arch: info?.target_arch,
  platform: typeof navigator !== "undefined" ? navigator.platform : undefined,
  locale: typeof navigator !== "undefined" ? navigator.language : undefined,
  timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
  user_agent: typeof navigator !== "undefined" ? navigator.userAgent : undefined,
});

function createSupportReport() {
  return `${m.ui_system_support_report_title()}
${m.ui_system_support_category()}: ${selectedCat?.label ?? category}
${m.ui_system_support_subject_title()}: ${subject.trim()}
${m.ui_system_support_contact_email_optional()}: ${email.trim() || "-"}

${m.ui_system_support_description_details()}:
${message.trim()}

${m.ui_system_support_auto_attached_diagnostic_telemetry()}:
${JSON.stringify({ ...diagnosticPayload, timestamp: new Date().toISOString() }, null, 2)}`;
}

async function copyDiagnosticReport() {
  submitError = null;
  try {
    await navigator.clipboard.writeText(createSupportReport());
    isCopied = true;
    setTimeout(() => {
      isCopied = false;
    }, 2500);
  } catch (err) {
    console.error("Failed to copy diagnostic report:", err);
    submitError = err instanceof Error ? err.message : String(err);
  }
}

async function handleSubmit(e: Event) {
  e.preventDefault();
  if (!info || !subject.trim() || !message.trim()) return;

  isSubmitting = true;
  submitError = null;
  isComposeOpened = false;

  try {
    const mailto = new URL("mailto:support@mulverse.com");
    mailto.searchParams.set("subject", `[Orbis] ${subject.trim()}`);
    mailto.searchParams.set("body", createSupportReport());
    composeUrl = mailto.toString();

    if (isDesktopApp) {
      const { openUrl } = await import("@tauri-apps/plugin-opener");
      await openUrl(composeUrl);
    } else {
      window.location.href = composeUrl;
    }
    isComposeOpened = true;
  } catch (error) {
    console.error("Failed to open the support email composer:", error);
    submitError = error instanceof Error ? error.message : String(error);
  } finally {
    isSubmitting = false;
  }
}

function resetForm() {
  subject = "";
  category = "technical_support";
  email = "";
  message = "";
  isComposeOpened = false;
  composeUrl = null;
  submitError = null;
}

$effect(() => {
  if (open && !info && !isLoadingInfo) {
    isLoadingInfo = true;
    infoError = null;
    void api.systemGetAppInfo()
      .then((appInfo) => (info = appInfo))
      .catch((error) => {
        console.error("Failed to load application information for support:", error);
        infoError = error instanceof Error ? error.message : String(error);
      })
      .finally(() => (isLoadingInfo = false));
  }
});
</script>

<Dialog bind:open>
  <DialogContent class="max-w-2xl max-h-[90vh] p-0 gap-0 overflow-hidden flex flex-col bg-background/95 backdrop-blur-md border-border/60 shadow-2xl">
    <!-- Header -->
    <DialogHeader class="px-6 py-4 border-b border-border/40 bg-muted/20 text-start space-y-1">
      <div class="flex items-center gap-2">
        <div class="size-6 rounded-md bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <QuestionMarkCircle size={14} />
        </div>
        <DialogTitle class="text-base font-semibold text-foreground tracking-tight">
          {m.ui_system_support_support_diagnostic_assistance()}
        </DialogTitle>
      </div>
      <DialogDescription class="text-xs text-muted-foreground">
        {m.ui_system_support_description()}
      </DialogDescription>
    </DialogHeader>

    {#if isComposeOpened}
      <div class="p-8 text-center space-y-4 my-auto">
        <div class="size-12 rounded-full bg-emerald-500/10 text-emerald-500 flex items-center justify-center mx-auto border border-emerald-500/20">
          <Check size={24} strokeWidth={2.5} />
        </div>
        <div class="space-y-1.5">
          <h3 class="text-base font-bold text-foreground">
            {m.ui_system_support_compose_ready()}
          </h3>
          <p class="text-xs text-muted-foreground max-w-md mx-auto leading-relaxed">
            {m.ui_system_support_compose_instructions()}
          </p>
        </div>

        <div class="pt-2 flex items-center justify-center gap-3">
          <Button
            variant="outline"
            size="sm"
            onclick={copyDiagnosticReport}
            class="h-8 text-xs font-normal gap-1.5 px-3"
          >
            {#if isCopied}
              <Check size={13} class="text-emerald-500" />
              <span>{m.ui_system_support_copied()}</span>
            {:else}
              <span>{m.ui_system_support_copy_full_report()}</span>
            {/if}
          </Button>

          <Button
            variant="default"
            size="sm"
            onclick={() => {
              resetForm();
              open = false;
            }}
            class="h-8 px-4 text-xs font-medium"
          >
            {m.ui_system_support_done()}
          </Button>
        </div>
      </div>
    {:else}
      <!-- Form Body -->
      <form onsubmit={handleSubmit} class="flex-1 overflow-y-auto p-6 space-y-4">
        <!-- Category & Email -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="space-y-1.5">
            <Label class="text-xs font-medium text-foreground">
              {m.ui_system_support_category()}
            </Label>
            <Select
              type="single"
              value={category}
              onValueChange={(v) => (category = v ?? "technical_support")}
            >
              <SelectTrigger class="w-full h-9 text-xs">
                {selectedCat ? selectedCat.label : category}
              </SelectTrigger>
              <SelectContent>
                {#each categories as cat (cat.value)}
                  <SelectItem value={cat.value}>
                    {cat.label}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <div class="space-y-1.5">
            <Label for="support-email" class="text-xs font-medium text-foreground">
              {m.ui_system_support_contact_email_optional()}
            </Label>
            <Input
              id="support-email"
              type="email"
              placeholder="admin@institution.edu"
              bind:value={email}
              class="h-9 text-xs"
            />
          </div>
        </div>

        <!-- Subject -->
        <div class="space-y-1.5">
          <Label for="support-subj" class="text-xs font-medium text-foreground">
            {m.ui_system_support_subject_title()}
          </Label>
          <Input
            id="support-subj"
            type="text"
            required
            placeholder={m.ui_system_support_e_g_issue_with_double_entry_journal_balance()}
            maxlength={160}
            bind:value={subject}
            class="h-9 text-xs"
          />
        </div>

        <!-- Description -->
        <div class="space-y-1.5">
          <Label for="support-msg" class="text-xs font-medium text-foreground">
            {m.ui_system_support_description_details()}
          </Label>
          <Textarea
            id="support-msg"
            rows={4}
            maxlength={4000}
            required
            placeholder={m.ui_system_support_describe_the_steps_taken_and_what_you_expected_to_happen()}
            bind:value={message}
            class="text-xs resize-none"
          />
        </div>

        <!-- Attached Diagnostic Telemetry Box -->
        <div class="rounded-lg border border-border/50 bg-muted/20 overflow-hidden text-xs">
          <button
            type="button"
            onclick={() => (showDiagnosticsPreview = !showDiagnosticsPreview)}
            class="w-full px-3.5 py-2.5 flex items-center justify-between text-muted-foreground hover:text-foreground transition-colors cursor-pointer text-start"
          >
            <div class="flex items-center gap-2">
              <span class="font-medium text-[11px]">
                {m.ui_system_support_auto_attached_diagnostic_telemetry()}
              </span>
              {#if info}
                <Badge variant="outline" class="text-[9px] font-mono px-1 py-0 h-3.5 border-border">
                  {info.target_os} • v{info.version}
                </Badge>
              {/if}
            </div>

            <span>
              {#if showDiagnosticsPreview}
                <ChevronUp size={13} />
              {:else}
                <ChevronDown size={13} />
              {/if}
            </span>
          </button>

          {#if showDiagnosticsPreview}
            <div class="p-3 border-t border-border/40 bg-background/50 font-mono text-[10px] text-muted-foreground space-y-1 max-h-36 overflow-y-auto">
              <pre>{JSON.stringify(diagnosticPayload, null, 2)}</pre>
            </div>
          {/if}
          {#if isLoadingInfo}
            <p class="px-3 pb-3 text-xs text-muted-foreground">{m.ui_system_support_loading_diagnostics()}</p>
          {:else if infoError}
            <p class="px-3 pb-3 text-xs text-destructive" role="alert">
              {m.ui_system_support_diagnostics_unavailable()} {infoError}
            </p>
          {/if}
        </div>
        {#if submitError}
          <div class="space-y-2 rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-destructive" role="alert">
            <p>{m.ui_system_support_compose_failed()} {submitError}</p>
            {#if composeUrl}
              <a href={composeUrl} target="_blank" rel="noreferrer" class="font-medium underline underline-offset-4">
                {m.ui_system_support_open_email_composer()}
              </a>
            {/if}
          </div>
        {/if}
      </form>

      <!-- Footer Action Bar -->
      <div class="px-6 py-3 border-t border-border/40 bg-muted/20 flex items-center justify-between">
        <Button
          type="button"
          variant="outline"
          size="sm"
          onclick={copyDiagnosticReport}
          class="h-8 text-xs font-normal gap-1.5 px-3 border-border/60"
        >
          {#if isCopied}
            <Check size={13} class="text-emerald-500" />
            <span class="text-emerald-600 dark:text-emerald-400 font-medium">
              {m.ui_system_support_report_copied()}
            </span>
          {:else}
            <span>{m.ui_system_support_copy_report()}</span>
          {/if}
        </Button>

        <div class="flex items-center gap-2">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onclick={() => (open = false)}
            class="h-8 px-3 text-xs"
          >
            {m.common_action_cancel()}
          </Button>

          <Button
            type="button"
            variant="default"
            size="sm"
            disabled={isSubmitting || isLoadingInfo || !info || !subject.trim() || !message.trim()}
            onclick={handleSubmit}
            class="h-8 px-4 text-xs font-medium gap-1.5"
          >
            {#if isSubmitting}
              <LoadingIndicator class="text-xs" />
              <span>{m.ui_system_support_sending()}</span>
            {:else}
              <span>{m.ui_system_support_send_report()}</span>
            {/if}
          </Button>
        </div>
      </div>
    {/if}
  </DialogContent>
</Dialog>
