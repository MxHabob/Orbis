<script lang="ts">
  import { onMount } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { orgContext } from "$lib/stores/org-context.svelte";
  import { resolveEntityDocumentUrl } from "$lib/features/documents/queries";
  import { formatDate } from "$lib/core/i18n";
  import { ArrowsRightLeft, BuildingOffice, CreditCard, Logo } from "@orbis/icons";

  let {
    planCode,
    planLabel,
    statusLabel,
    validUntil,
  }: {
    planCode: string;
    planLabel: string;
    statusLabel: string;
    validUntil: string | null;
  } = $props();

  const planTheme = $derived.by(() => {
    const plan = planCode.toLowerCase().replace(/[_\s-]+/g, "");

    if (plan.includes("enterprise")) {
      return {
        surface: "border-brand-pink/65 bg-brand-pink/10",
        edge: "bg-brand-pink",
        marker: "bg-brand-pink",
        badge: "border-brand-pink/50 bg-brand-pink/15 text-foreground",
      };
    }
    if (plan.includes("professional")) {
      return {
        surface: "border-brand-purple/65 bg-brand-purple/10",
        edge: "bg-brand-purple",
        marker: "bg-brand-purple",
        badge: "border-brand-purple/50 bg-brand-purple/15 text-foreground",
      };
    }
    if (plan.includes("growth")) {
      return {
        surface: "border-brand-lime/70 bg-brand-lime/10",
        edge: "bg-brand-lime",
        marker: "bg-brand-lime",
        badge: "border-brand-lime/55 bg-brand-lime/15 text-foreground",
      };
    }
    if (plan.includes("starter")) {
      return {
        surface: "border-brand-indigo/65 bg-brand-indigo/10",
        edge: "bg-brand-indigo",
        marker: "bg-brand-indigo",
        badge: "border-brand-indigo/50 bg-brand-indigo/15 text-foreground",
      };
    }
    if (plan.includes("trial") || plan.includes("free")) {
      return {
        surface: "border-border bg-muted/50",
        edge: "bg-muted-foreground/50",
        marker: "bg-muted-foreground",
        badge: "border-border bg-background/70 text-muted-foreground",
      };
    }

    return {
      surface: "border-primary/55 bg-primary/10",
      edge: "bg-primary",
      marker: "bg-primary",
      badge: "border-primary/40 bg-primary/10 text-foreground",
    };
  });

  let organizationLogoUrl = $state("");
  let organizationLogoFailed = $state(false);

  onMount(() => {
    const logo = orgContext.logo;
    const organizationId = orgContext.orgId;
    if (!logo || !organizationId) return;

    let isCurrent = true;
    resolveEntityDocumentUrl(logo, "organization", organizationId)
      .then((url) => {
        if (isCurrent) organizationLogoUrl = url;
      })
      .catch((error: unknown) => {
        console.error("Failed to resolve organization logo for license card:", error);
      });

    return () => {
      isCurrent = false;
    };
  });
</script>

<div
  class="relative flex aspect-[1.55] w-full max-w-[18rem] flex-col justify-between overflow-hidden rounded-2xl border-2 p-4 text-start sm:p-5 {planTheme.surface}"
>
  <span
    aria-hidden="true"
    class="pointer-events-none absolute inset-x-0 top-0 h-1.5 {planTheme.edge}"
  ></span>
  <div class="flex items-center justify-between gap-3">
    <div class="flex size-10 shrink-0 items-center justify-center rounded-xl border border-border/70 bg-background/75 text-primary" aria-label="Orbis">
      <Logo size={24} />
    </div>
    <div class="flex size-8 shrink-0 items-center justify-center rounded-full border border-border/70 bg-background/75 text-muted-foreground" aria-hidden="true">
      <ArrowsRightLeft size={16} />
    </div>
    <div class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-xl border border-border/70 bg-background/75">
      {#if organizationLogoUrl && !organizationLogoFailed}
        <img
          src={organizationLogoUrl}
          alt={orgContext.displayName}
          class="size-full object-contain p-1"
          onerror={() => (organizationLogoFailed = true)}
        />
      {:else}
        <BuildingOffice size={20} class="text-muted-foreground" aria-hidden="true" />
      {/if}
    </div>
  </div>

  <div class="flex min-w-0 items-center gap-2">
    <span class="size-2.5 shrink-0 rounded-full {planTheme.marker}" aria-hidden="true"></span>
    <p class="truncate text-xl font-semibold tracking-wide text-foreground">{planLabel}</p>
  </div>

  <div class="flex items-end justify-between gap-3">
    <div class="flex min-w-0 flex-col gap-1">
      <span class="text-[10px] text-muted-foreground">{m.license_valid_until()}</span>
      <span class="truncate text-xs font-medium tabular-nums text-foreground">
        {formatDate(validUntil, "datetime") || "—"}
      </span>
    </div>
    <span class="shrink-0 rounded-full border px-2.5 py-1 text-[10px] font-medium {planTheme.badge}">
      {statusLabel}
    </span>
  </div>
</div>
