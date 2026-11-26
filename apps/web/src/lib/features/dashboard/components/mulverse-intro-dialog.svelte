<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type AccountProvisioningResultDto,
} from "$lib/api";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField, toast } from "@orbis/ui";
import { GlobeAlt } from "@orbis/icons";
import { MULVERSE_URLS } from "@orbis/utils";
import { IAM_DOMAINS, Permissions } from "$lib/core/iam/permissions";
import { effectiveContext } from "$lib/stores";
import CredentialDeliveryModal from "$lib/shared/components/credential-delivery-modal.svelte";
import SmoothImage from "$lib/shared/components/smooth-image.svelte";
import mulverseBanner from "$lib/assets/image.png";

let {
  orgId,
  hasOperationalActivity = false,
}: {
  orgId: string;
  hasOperationalActivity?: boolean;
} = $props();

const STORAGE_KEY_PREFIX = "orbis_mulverse_intro_dismissed_";
const queryClient = useQueryClient();

let open = $state(false);
let step = $state<"intro" | "create">("intro");
let mounted = $state(false);
let dismissed = $state(true);

let fullName = $state("");
let email = $state("");
let isSubmitting = $state(false);
let formError = $state<string | null>(null);

let credentialModalOpen = $state(false);
let provisioningResult = $state<AccountProvisioningResultDto | null>(null);
let createdUserName = $state("");

const canProvisionSafeAdmin = $derived(
  effectiveContext.hasPermission("*") ||
    (effectiveContext.hasPermission(Permissions.UserWrite) &&
      effectiveContext.hasPermission(Permissions.UserRoleWrite)),
);

function storageKey(): string {
  return `${STORAGE_KEY_PREFIX}${orgId}`;
}

function markDismissed() {
  dismissed = true;
  open = false;
  if (typeof localStorage !== "undefined" && orgId) {
    localStorage.setItem(storageKey(), "true");
  }
}

onMount(() => {
  if (!orgId) return;
  dismissed = localStorage.getItem(storageKey()) === "true";
  mounted = true;
});

$effect(() => {
  if (!mounted || dismissed || !hasOperationalActivity || !canProvisionSafeAdmin) {
    return;
  }

  const timer = setTimeout(() => {
    if (!dismissed) {
      open = true;
    }
  }, 1200);

  return () => clearTimeout(timer);
});

function buildSafeAdminPermissions(): Record<string, string[]> {
  const perms: Record<string, string[]> = {};
  for (const domain of IAM_DOMAINS) {
    for (const resource of domain.resources) {
      const safeActions = resource.actions
        .map((action) => action.key)
        .filter((key) => key !== "delete");
      if (safeActions.length > 0) {
        perms[resource.key] = safeActions;
      }
    }
  }
  return perms;
}

async function handleCreateSafeAccount(e: SubmitEvent) {
  e.preventDefault();
  formError = null;

  const trimmedName = fullName.trim();
  if (!trimmedName) {
    formError = m.mulverse_intro_name_required();
    return;
  }

  isSubmitting = true;
  try {
    await api.saveRole({
      id: "safe_admin",
      name: "Safe Operational Admin",
      name_ar: "مدير تشغيلي آمن (بدون حذف)",
      permissions: buildSafeAdminPermissions(),
    });

    const result = await api.provisionUser({
      full_name: trimmedName,
      full_name_ar: trimmedName,
      email: email.trim() || undefined,
      roles: ["safe_admin"],
      actor_type: "custom",
      delivery_channel: "manual",
    });

    provisioningResult = result;
    createdUserName = trimmedName;
    markDismissed();
    credentialModalOpen = true;
    toast.success(m.mulverse_intro_create_success());
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.users() }),
      queryClient.invalidateQueries({ queryKey: ["users_paginated", orgId] }),
    ]);
  } catch (err) {
    formError = extractErrorMessage(err);
    toast.error(formError);
  } finally {
    isSubmitting = false;
  }
}

function handleOpenChange(nextOpen: boolean) {
  if (!nextOpen && !isSubmitting) {
    markDismissed();
  }
}
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogContent
    showCloseButton={false}
    class="w-full max-w-lg sm:max-w-[500px] gap-0 overflow-hidden font-sans text-start"
  >
    <DialogTitle class="sr-only">{m.mulverse_intro_title()}</DialogTitle>
    <DialogDescription class="sr-only">{m.mulverse_intro_description()}</DialogDescription>

    <SmoothImage
      src={mulverseBanner}
      alt="Mulverse"
      loading="eager"
      fetchpriority="high"
      containerClass="w-full h-44 sm:h-48 rounded-md border border-border/60 bg-muted"
      class="w-full h-full object-cover select-none"
    />

    {#if step === "intro"}
      <div class="pt-4 px-1 pb-1 flex flex-col gap-1.5 text-start">
        <h2 class="text-lg sm:text-xl font-bold tracking-tight text-foreground">
          {m.mulverse_intro_title()}
        </h2>
        <p class="text-xs sm:text-[13px] leading-relaxed text-muted-foreground">
          {m.mulverse_intro_description()}
        </p>

        <div class="flex items-center justify-between gap-2 pt-3 mt-1">
          <a
            href={MULVERSE_URLS.website}
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-1.5 text-xs font-medium text-muted-foreground hover:text-primary transition-colors"
          >
            <GlobeAlt size={14} />
            <span>{m.mulverse_intro_visit_website()}</span>
          </a>
          <div class="flex items-center gap-2">
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onclick={markDismissed}
            >
              {m.mulverse_intro_later()}
            </Button>
            <Button
              type="button"
              variant="default"
              size="sm"
              onclick={() => (step = "create")}
            >
              {m.mulverse_intro_create_safe_admin()}
            </Button>
          </div>
        </div>
      </div>
    {:else}
      <form onsubmit={handleCreateSafeAccount} class="pt-4 px-1 pb-1 flex flex-col gap-3">
        <h2 class="text-base font-bold tracking-tight text-foreground">
          {m.mulverse_intro_form_title()}
        </h2>

        <FormField
          id="safe_admin_name"
          label={m.mulverse_intro_full_name_label()}
          required
          error={formError ?? undefined}
        >
          {#snippet children()}
            <Input
              id="safe_admin_name"
              type="text"
              bind:value={fullName}
              placeholder={m.mulverse_intro_full_name_placeholder()}
              disabled={isSubmitting}
              required
            />
          {/snippet}
        </FormField>

        <FormField id="safe_admin_email" label={m.mulverse_intro_email_label()}>
          {#snippet children()}
            <Input
              id="safe_admin_email"
              type="email"
              bind:value={email}
              placeholder="ops@school.edu"
              disabled={isSubmitting}
            />
          {/snippet}
        </FormField>

        <div class="pt-2 flex items-center justify-end gap-2">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            disabled={isSubmitting}
            onclick={() => {
              formError = null;
              step = "intro";
            }}
          >
            {m.mulverse_intro_back()}
          </Button>
          <Button
            type="submit"
            variant="default"
            size="sm"
            disabled={isSubmitting}
          >
            {isSubmitting ? m.mulverse_intro_creating() : m.mulverse_intro_create_submit()}
          </Button>
        </div>
      </form>
    {/if}
  </DialogContent>
</Dialog>

<CredentialDeliveryModal
  bind:open={credentialModalOpen}
  provisioning={provisioningResult}
  userName={createdUserName}
/>
