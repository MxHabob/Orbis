<!--
  Organization Bootstrap Screen — Create the first organization and super admin.
  Clean 2-step setup built using @orbis/ui Card, Button, Input, Logo, and standard design tokens.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { authStore } from "$lib/stores";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Logo } from "@orbis/ui/logo";
import {
  Building2,
  User,
  Loader2,
  CheckCircle2,
  AlertCircle,
  ArrowRight,
  ArrowLeft,
  ShieldCheck,
} from "@lucide/svelte";

const isRTL = $derived(getLocale().startsWith("ar"));

let step = $state(1); // 1 = Organization Details, 2 = Admin Account
let isSubmitting = $state(false);
let error = $state("");

// Step 1: Organization
let orgName = $state("");
let orgNameAr = $state("");
let orgType = $state("school");
let timezone = $state("Asia/Riyadh");
let locale = $state("ar");
let currency = $state("SAR");

// Step 2: Super Admin
let adminFullName = $state("");
let adminEmail = $state("");
let adminPassword = $state("");
let adminPasswordConfirm = $state("");

const orgTypes = $derived([
  { value: "school", label: m.setup_bootstrap_type_school() },
  { value: "university", label: m.setup_bootstrap_type_university() },
  { value: "institute", label: m.setup_bootstrap_type_institute() },
]);

const currencies = [
  { code: "SAR", label: "Saudi Riyal (SAR)" },
  { code: "YER", label: "Yemeni Rial (YER)" },
  { code: "AED", label: "UAE Dirham (AED)" },
  { code: "EGP", label: "Egyptian Pound (EGP)" },
  { code: "KWD", label: "Kuwaiti Dinar (KWD)" },
  { code: "QAR", label: "Qatari Riyal (QAR)" },
  { code: "USD", label: "US Dollar (USD)" },
  { code: "EUR", label: "Euro (EUR)" },
  { code: "GBP", label: "British Pound (GBP)" },
];

const timezones = [
  { value: "Asia/Riyadh", label: "Riyadh (UTC+3)" },
  { value: "Asia/Aden", label: "Aden / Sana'a (UTC+3)" },
  { value: "Asia/Dubai", label: "Dubai (UTC+4)" },
  { value: "Africa/Cairo", label: "Cairo (UTC+2)" },
  { value: "Europe/London", label: "London (UTC+0)" },
  { value: "America/New_York", label: "New York (UTC-5)" },
];

function handleNext() {
  error = "";
  if (!orgName.trim()) {
    error = m.setup_bootstrap_err_org_name_required();
    return;
  }
  step = 2;
}

function handleBack() {
  error = "";
  step = 1;
}

async function handleBootstrap() {
  error = "";
  if (!adminFullName.trim()) {
    error = m.setup_bootstrap_err_admin_name_required();
    return;
  }
  if (!adminEmail.trim()) {
    error = m.setup_bootstrap_err_admin_email_required();
    return;
  }
  if (adminPassword.length < 8) {
    error = m.setup_bootstrap_err_password_length();
    return;
  }
  if (adminPassword !== adminPasswordConfirm) {
    error = m.setup_bootstrap_err_password_mismatch();
    return;
  }

  isSubmitting = true;
  try {
    const result = await api.bootstrapOrganization({
      orgName: orgName.trim(),
      orgNameAr: orgNameAr.trim() || undefined,
      orgType,
      timezone,
      locale,
      currency,
      adminFullName: adminFullName.trim(),
      adminEmail: adminEmail.trim(),
      adminPassword,
    });

    // Auto-authenticate session
    await authStore.setSession({
      token: result.token,
      user: {
        id: result.admin_user_id,
        email: adminEmail.trim(),
        full_name: adminFullName.trim(),
        role: "super_admin",
        org_id: result.org_id,
        created_at: new Date().toISOString(),
      },
    });

    goto(localizeHref("/onboarding"), { replaceState: true });
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isSubmitting = false;
  }
}
</script>

<svelte:head>
  <title>{m.setup_bootstrap_page_title()}</title>
</svelte:head>

<div class="w-full max-w-3xl">
  <Card class="bg-card border border-border shadow-2xl rounded-2xl overflow-hidden flex flex-col md:flex-row">
    <!-- Left Brand Column -->
    <div class="w-full md:w-20 bg-primary flex md:flex-col items-center justify-between md:justify-start p-4 md:py-8 gap-4 text-primary-foreground shrink-0 select-none">
      <div class="flex items-center justify-center text-primary-foreground">
        <Logo class="size-8 text-primary-foreground" dotClass="fill-white" />
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 p-6 sm:p-8 lg:p-10 flex flex-col justify-between min-w-0">
      <div>
        <!-- Stepper Indicator -->
        <div class="flex items-center justify-between mb-6 pb-4 border-b border-border/60">
          <div>
            <h2 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
              {m.setup_bootstrap_title()}
            </h2>
            <p class="text-xs text-muted-foreground mt-0.5">
              {step === 1
                ? m.setup_bootstrap_step_1_desc()
                : m.setup_bootstrap_step_2_desc()}
            </p>
          </div>

          <div class="flex items-center gap-1.5">
            <span class="w-2.5 h-2.5 rounded-full {step === 1 ? 'bg-primary' : 'bg-emerald-500'}"></span>
            <span class="w-2.5 h-2.5 rounded-full {step === 2 ? 'bg-primary' : 'bg-muted'}"></span>
          </div>
        </div>

        {#if error}
          <div class="mb-5 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{error}</span>
          </div>
        {/if}

        <!-- STEP 1: Organization Details -->
        {#if step === 1}
          <div class="space-y-4">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label for="orgName" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_org_name_label()}
                </label>
                <Input
                  id="orgName"
                  type="text"
                  bind:value={orgName}
                  class="h-10 text-xs bg-background border-input"
                  placeholder="e.g. Al-Amal Academy"
                />
              </div>

              <div>
                <label for="orgNameAr" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_org_name_ar_label()}
                </label>
                <Input
                  id="orgNameAr"
                  type="text"
                  bind:value={orgNameAr}
                  class="h-10 text-xs bg-background border-input text-right"
                  placeholder="مثال: أكاديمية الأمل"
                />
              </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <div>
                <label for="orgType" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_org_type_label()}
                </label>
                <select
                  id="orgType"
                  bind:value={orgType}
                  class="w-full bg-background border border-input rounded-md px-3 h-10 text-xs focus:outline-none focus:ring-2 focus:ring-primary/40 text-foreground"
                >
                  {#each orgTypes as t}
                    <option value={t.value}>{t.label}</option>
                  {/each}
                </select>
              </div>

              <div>
                <label for="currency" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_currency_label()}
                </label>
                <select
                  id="currency"
                  bind:value={currency}
                  class="w-full bg-background border border-input rounded-md px-3 h-10 text-xs focus:outline-none focus:ring-2 focus:ring-primary/40 text-foreground"
                >
                  {#each currencies as c}
                    <option value={c.code}>{c.label}</option>
                  {/each}
                </select>
              </div>

              <div>
                <label for="timezone" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_timezone_label()}
                </label>
                <select
                  id="timezone"
                  bind:value={timezone}
                  class="w-full bg-background border border-input rounded-md px-3 h-10 text-xs focus:outline-none focus:ring-2 focus:ring-primary/40 text-foreground"
                >
                  {#each timezones as tz}
                    <option value={tz.value}>{tz.label}</option>
                  {/each}
                </select>
              </div>
            </div>
          </div>

        <!-- STEP 2: Super Admin Account -->
        {:else}
          <div class="space-y-4">
            <div>
              <label for="adminFullName" class="block text-xs font-medium text-foreground mb-1.5">
                {m.setup_bootstrap_admin_name_label()}
              </label>
              <Input
                id="adminFullName"
                type="text"
                bind:value={adminFullName}
                class="h-10 text-xs bg-background border-input"
                placeholder="e.g. Abdullah Al-Mansoor"
              />
            </div>

            <div>
              <label for="adminEmail" class="block text-xs font-medium text-foreground mb-1.5">
                {m.setup_bootstrap_admin_email_label()}
              </label>
              <Input
                id="adminEmail"
                type="email"
                bind:value={adminEmail}
                class="h-10 text-xs bg-background border-input"
                placeholder="admin@school.edu"
              />
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label for="adminPassword" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_admin_password_label()}
                </label>
                <Input
                  id="adminPassword"
                  type="password"
                  bind:value={adminPassword}
                  class="h-10 text-xs bg-background border-input"
                  placeholder="••••••••"
                />
              </div>

              <div>
                <label for="adminPasswordConfirm" class="block text-xs font-medium text-foreground mb-1.5">
                  {m.setup_bootstrap_admin_password_confirm_label()}
                </label>
                <Input
                  id="adminPasswordConfirm"
                  type="password"
                  bind:value={adminPasswordConfirm}
                  class="h-10 text-xs bg-background border-input"
                  placeholder="••••••••"
                />
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Action Footer -->
      <div class="pt-6 mt-6 border-t border-border/40 flex items-center justify-between">
        {#if step === 2}
          <Button
            type="button"
            variant="outline"
            onclick={handleBack}
            disabled={isSubmitting}
            class="inline-flex items-center gap-1.5 px-4 h-10 text-xs font-medium cursor-pointer border-border"
          >
            {#if isRTL}
              <ArrowRight class="w-3.5 h-3.5" />
            {:else}
              <ArrowLeft class="w-3.5 h-3.5" />
            {/if}
            <span>{m.setup_bootstrap_prev_btn()}</span>
          </Button>
        {:else}
          <div></div>
        {/if}

        {#if step === 1}
          <Button
            type="button"
            variant="default"
            onclick={handleNext}
            class="inline-flex items-center gap-2 px-6 h-10 text-xs font-semibold cursor-pointer"
          >
            <span>{m.setup_bootstrap_next_btn()}</span>
            {#if isRTL}
              <ArrowLeft class="w-4 h-4" />
            {:else}
              <ArrowRight class="w-4 h-4" />
            {/if}
          </Button>
        {:else}
          <Button
            type="button"
            variant="default"
            onclick={handleBootstrap}
            disabled={isSubmitting}
            class="inline-flex items-center gap-2 px-6 h-10 text-xs font-semibold cursor-pointer disabled:opacity-50"
          >
            {#if isSubmitting}
              <Loader2 class="w-4 h-4 animate-spin" />
              <span>{m.setup_bootstrap_submitting()}</span>
            {:else}
              <ShieldCheck class="w-4 h-4" />
              <span>{m.setup_bootstrap_submit_btn()}</span>
            {/if}
          </Button>
        {/if}
      </div>
    </div>
  </Card>
</div>

