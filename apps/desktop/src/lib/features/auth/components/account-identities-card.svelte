<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { localizeHref } from "$lib/paraglide/runtime";
import { api } from "$lib/api";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import { Unlink, ExternalLink, ShieldCheck } from "@lucide/svelte";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  isGoogleLinked?: boolean;
  googleEmail?: string | null;
  isMicrosoftLinked?: boolean;
  microsoftEmail?: string | null;
  onRefresh?: () => void;
}

let {
  isGoogleLinked = false,
  googleEmail = null,
  isMicrosoftLinked = false,
  microsoftEmail = null,
  onRefresh,
}: Props = $props();

let isLinkingGoogle = $state(false);
let isLinkingMicrosoft = $state(false);

let isGoogleOrgEnabled = $state(true);
let isMicrosoftOrgEnabled = $state(true);

const orgId = $derived(page.params.orgId || authStore.orgId || "");
const integrationsUrl = $derived(
  orgId ? `/${orgId}/ai-center/integrations` : "/",
);

onMount(async () => {
  // Check organization integration statuses
  try {
    const googleStatus = await api
      .getIntegrationStatus("google-workspace")
      .catch(() => null);
    if (googleStatus !== null) {
      isGoogleOrgEnabled = !!googleStatus?.connected;
    }
  } catch {
    // default to true
  }

  try {
    const msStatus = await api
      .getIntegrationStatus("microsoft-365")
      .catch(() => null);
    if (msStatus !== null) {
      isMicrosoftOrgEnabled = !!msStatus?.connected;
    }
  } catch {
    // default to true
  }

  const cachedMsLinked =
    typeof window !== "undefined" &&
    localStorage.getItem("orbis_microsoft_linked") === "true";
  const cachedMsEmail =
    typeof window !== "undefined"
      ? localStorage.getItem("orbis_microsoft_email")
      : null;
  if (cachedMsLinked) {
    isMicrosoftLinked = true;
    microsoftEmail = cachedMsEmail;
  }
});

async function handleLinkProvider(provider: "google" | "microsoft") {
  if (provider === "google") isLinkingGoogle = true;
  if (provider === "microsoft") isLinkingMicrosoft = true;

  try {
    const authUrl = `/auth/${provider}?mode=link`;
    await goto(localizeHref(authUrl));
  } catch (err: any) {
    toast.error(err.message || `Failed to connect ${provider} account`);
  } finally {
    if (provider === "google") isLinkingGoogle = false;
    if (provider === "microsoft") isLinkingMicrosoft = false;
  }
}

function handleDisconnectProvider(provider: "google" | "microsoft") {
  if (provider === "google") {
    isGoogleLinked = false;
    googleEmail = null;
    localStorage.removeItem("orbis_google_linked");
    localStorage.removeItem("orbis_google_email");
  } else {
    isMicrosoftLinked = false;
    microsoftEmail = null;
    localStorage.removeItem("orbis_microsoft_linked");
    localStorage.removeItem("orbis_microsoft_email");
  }
  toast.success(m.common_account_identities_disconnect_success());
  onRefresh?.();
}
</script>

<SettingsCard title={m.common_account_identities_section()}>
  {#snippet headerAction()}
    {#if orgId}
      <a
        href={localizeHref(integrationsUrl)}
        class="inline-flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors font-medium"
      >
        <span>{m.common_account_identities_configure_in_integrations()}</span>
        <ExternalLink class="w-3 h-3" />
      </a>
    {/if}
  {/snippet}

  <!-- Google Workspace Identity Row -->
  <SettingsRow
    description={isGoogleLinked
      ? m.common_account_identities_google_linked_desc({ email: googleEmail || "Google Workspace" })
      : m.common_account_identities_google_desc()}
  >
    {#snippet headerSlot()}
      <div class="flex items-center gap-2">
        <!-- Google Colored SVG Icon -->
        <svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24">
          <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
          <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
          <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
          <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
        </svg>
        <Label class="text-xs font-semibold text-foreground">
          {m.common_account_identities_google_title()}
        </Label>
        {#if isGoogleLinked}
          <Badge variant="default" class="text-[10px] font-medium py-0 px-2 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
            {m.common_account_identities_connected()}
          </Badge>
        {:else if isGoogleOrgEnabled}
          <Badge variant="secondary" class="text-[10px] font-normal py-0 px-2 text-muted-foreground border">
            {m.common_account_identities_org_enabled()}
          </Badge>
        {/if}
      </div>
    {/snippet}

    {#snippet children()}
      {#if isGoogleLinked}
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="h-9 px-4 text-xs font-medium text-destructive hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
          onclick={() => handleDisconnectProvider("google")}
        >
          <Unlink class="w-3.5 h-3.5 me-1.5" />
          <span>{m.common_account_identities_disconnect()}</span>
        </Button>
      {:else}
        <Button
          type="button"
          variant="outline"
          size="sm"
          disabled={isLinkingGoogle}
          class="h-9 px-4 text-xs font-semibold gap-2"
          onclick={() => handleLinkProvider("google")}
        >
          <svg class="w-3.5 h-3.5 shrink-0" viewBox="0 0 24 24">
            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
          </svg>
          <span>{isLinkingGoogle ? m.common_account_identities_connecting() : m.common_account_identities_connect_google()}</span>
        </Button>
      {/if}
    {/snippet}
  </SettingsRow>

  <!-- Microsoft 365 Identity Row -->
  <SettingsRow
    description={isMicrosoftLinked
      ? m.common_account_identities_microsoft_linked_desc({ email: microsoftEmail || "Microsoft 365" })
      : m.common_account_identities_microsoft_desc()}
    last
  >
    {#snippet headerSlot()}
      <div class="flex items-center gap-2">
        <!-- Microsoft Colored SVG Icon -->
        <svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24">
          <path fill="#F25022" d="M1 1h10v10H1z" />
          <path fill="#00A4EF" d="M1 13h10v10H1z" />
          <path fill="#7FBA00" d="M13 1h10v10H13z" />
          <path fill="#FFB900" d="M13 13h10v10H13z" />
        </svg>
        <Label class="text-xs font-semibold text-foreground">
          {m.common_account_identities_microsoft_title()}
        </Label>
        {#if isMicrosoftLinked}
          <Badge variant="default" class="text-[10px] font-medium py-0 px-2 bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20">
            {m.common_account_identities_connected()}
          </Badge>
        {:else if isMicrosoftOrgEnabled}
          <Badge variant="secondary" class="text-[10px] font-normal py-0 px-2 text-muted-foreground border">
            {m.common_account_identities_org_enabled()}
          </Badge>
        {/if}
      </div>
    {/snippet}

    {#snippet children()}
      {#if isMicrosoftLinked}
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="h-9 px-4 text-xs font-medium text-destructive hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
          onclick={() => handleDisconnectProvider("microsoft")}
        >
          <Unlink class="w-3.5 h-3.5 me-1.5" />
          <span>{m.common_account_identities_disconnect()}</span>
        </Button>
      {:else}
        <Button
          type="button"
          variant="outline"
          size="sm"
          disabled={isLinkingMicrosoft}
          class="h-9 px-4 text-xs font-semibold gap-2"
          onclick={() => handleLinkProvider("microsoft")}
        >
          <svg class="w-3.5 h-3.5 shrink-0" viewBox="0 0 24 24">
            <path fill="#F25022" d="M1 1h10v10H1z" />
            <path fill="#00A4EF" d="M1 13h10v10H1z" />
            <path fill="#7FBA00" d="M13 1h10v10H13z" />
            <path fill="#FFB900" d="M13 13h10v10H13z" />
          </svg>
          <span>{isLinkingMicrosoft ? m.common_account_identities_connecting() : m.common_account_identities_connect_microsoft()}</span>
        </Button>
      {/if}
    {/snippet}
  </SettingsRow>
</SettingsCard>
