<!--
  @component SupportDialog
  Builds a prefilled support email with user-provided details and reviewable device diagnostics.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { MULVERSE_URLS } from "@orbis/utils";
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api/client";
import { isDesktopApp } from "$lib/api";
import type { AppInfo } from "$lib/api/types";
import { authStore, orgContext } from "$lib/stores";
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
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";

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
let isLoadingInfo = $state(false);
let infoError = $state<string | null>(null);

const categories = $derived([
  {
    value: "technical_support",
    label: m.ui_system_support_category_technical(),
  },
  {
    value: "onboarding_setup",
    label: m.ui_system_support_category_onboarding(),
  },
  {
    value: "finance_accounting",
    label: m.ui_system_support_category_finance(),
  },
  {
    value: "academics_grading",
    label: m.ui_system_support_category_academics(),
  },
  {
    value: "feature_request",
    label: m.ui_system_support_category_feature_request(),
  },
  { value: "other", label: m.ui_system_support_category_general() },
]);

const userEmail = $derived(authStore.user?.email?.trim() || "");
const userName = $derived(authStore.user?.full_name || "");
const effectiveEmail = $derived(userEmail || email.trim());
const selectedCat = $derived(categories.find((c) => c.value === category));

const diagnosticPayload = $derived({
  application: info?.name,
  version: info?.version,
  build_profile: info?.build_profile,
  target_os: info?.target_os,
  target_arch: info?.target_arch,
  org_id: orgContext.orgId || authStore.orgId || undefined,
  org_name: orgContext.displayName || undefined,
  user_id: authStore.userId || undefined,
  user_name: userName || undefined,
  platform: typeof navigator !== "undefined" ? navigator.platform : undefined,
  locale: typeof navigator !== "undefined" ? navigator.language : undefined,
  timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
  user_agent:
    typeof navigator !== "undefined" ? navigator.userAgent : undefined,
});

function createSupportReport() {
  return `${m.ui_system_support_report_title()}
${m.ui_system_support_category()}: ${selectedCat?.label ?? category}
${m.ui_system_support_subject_title()}: ${subject.trim()}
${m.ui_system_support_contact_email_optional()}: ${effectiveEmail || "-"}

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
    const mailto = new URL(`mailto:${MULVERSE_URLS.supportEmail}`);
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
    void api
      .systemGetAppInfo()
      .then((appInfo) => (info = appInfo))
      .catch((error) => {
        console.error(
          "Failed to load application information for support:",
          error,
        );
        infoError = error instanceof Error ? error.message : String(error);
      })
      .finally(() => (isLoadingInfo = false));
  }
});
</script>

<Dialog bind:open>
  <DialogContent class="sm:max-w-[460px] p-5 gap-4 text-start">
    <DialogHeader class="text-start space-y-0">
      <DialogTitle class="text-base font-bold tracking-tight text-foreground">
        {m.ui_system_support_support_diagnostic_assistance()}
      </DialogTitle>
      <DialogDescription class="sr-only">
        {m.ui_system_support_description()}
      </DialogDescription>
    </DialogHeader>

    {#if isComposeOpened}
      <div class="py-4 text-center space-y-4">
        <div class="space-y-1.5">
          <h3 class="text-base font-semibold text-foreground">
            {m.ui_system_support_compose_ready()}
          </h3>
          <p class="text-xs text-muted-foreground max-w-sm mx-auto leading-relaxed">
            {m.ui_system_support_compose_instructions()}
          </p>
        </div>

        <div class="pt-1 flex items-center justify-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onclick={copyDiagnosticReport}
            class="h-8 text-xs"
          >
            {isCopied ? m.ui_system_support_copied() : m.ui_system_support_copy_full_report()}
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
      <form onsubmit={handleSubmit} class="flex flex-col gap-3.5">
        <div class="grid grid-cols-1 {!userEmail ? 'sm:grid-cols-2' : ''} gap-3.5">
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

          {#if !userEmail}
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
          {/if}
        </div>

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

        {#if infoError}
          <p class="text-xs text-destructive" role="alert">
            {m.ui_system_support_diagnostics_unavailable()} {infoError}
          </p>
        {/if}

        {#if submitError}
          <div class="space-y-1.5 rounded-md border border-destructive/20 bg-destructive/5 p-2.5 text-xs text-destructive" role="alert">
            <p>{m.ui_system_support_compose_failed()} {submitError}</p>
            {#if composeUrl}
              <a href={composeUrl} target="_blank" rel="noreferrer" class="font-medium underline underline-offset-4">
                {m.ui_system_support_open_email_composer()}
              </a>
            {/if}
          </div>
        {/if}

        <div class="pt-1 flex items-center justify-between gap-2">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onclick={copyDiagnosticReport}
            class="h-8 px-2.5 text-xs text-muted-foreground hover:text-foreground"
          >
            {isCopied ? m.ui_system_support_report_copied() : m.ui_system_support_copy_report()}
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
              type="submit"
              variant="default"
              size="sm"
              disabled={isSubmitting || isLoadingInfo || !info || !subject.trim() || !message.trim()}
              class="h-8 px-4 text-xs font-medium"
            >
              {#if isSubmitting}
                <LoadingIndicator class="me-1.5 text-xs" />
                {m.ui_system_support_sending()}
              {:else}
                {m.ui_system_support_send_report()}
              {/if}
            </Button>
          </div>
        </div>
      </form>
    {/if}
  </DialogContent>
</Dialog>
