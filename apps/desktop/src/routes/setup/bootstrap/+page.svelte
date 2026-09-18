<!--
  Organization Bootstrap Screen — Create the first organization and super admin.
  Clean 2-step setup built using FormController, FormField, @orbis/ui Card, Button, Input, Select, Logo, and standard design tokens.
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
import { FormField } from "@orbis/ui";
import { Logo } from "@orbis/ui/logo";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Loader2,
  AlertCircle,
  ArrowRight,
  ArrowLeft,
  ShieldCheck,
} from "@lucide/svelte";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

const isRTL = $derived(getLocale().startsWith("ar"));

let step = $state(1); // 1 = Organization Details, 2 = Admin Account

const bootstrapSchema = z.object({
  orgName: z.string().trim().min(1, m.setup_bootstrap_err_org_name_required()),
  orgNameAr: z.string().optional(),
  orgType: z.string().min(1),
  timezone: z.string().min(1),
  locale: z.string().min(1),
  currency: z.string().min(1),
  adminFullName: z.string().trim().min(1, m.setup_bootstrap_err_admin_name_required()),
  adminEmail: z.string().trim().email(m.setup_bootstrap_err_admin_email_required()),
  adminPassword: z.string().min(8, m.setup_bootstrap_err_password_length()),
  adminPasswordConfirm: z.string().min(1),
}).refine((data) => data.adminPassword === data.adminPasswordConfirm, {
  message: m.setup_bootstrap_err_password_mismatch(),
  path: ["adminPasswordConfirm"],
});

type BootstrapFormValues = z.infer<typeof bootstrapSchema>;

const form = new FormController<BootstrapFormValues>({
  initialValues: {
    orgName: "",
    orgNameAr: "",
    orgType: "school",
    timezone: "Asia/Riyadh",
    locale: "ar",
    currency: "SAR",
    adminFullName: "",
    adminEmail: "",
    adminPassword: "",
    adminPasswordConfirm: "",
  },
  schema: bootstrapSchema,
});

function fieldError(field: keyof BootstrapFormValues): string | undefined {
  return form.errors[field]?.[0];
}

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
  form.errors = {};
  form.formError = undefined;
  if (!form.values.orgName.trim()) {
    form.errors.orgName = [m.setup_bootstrap_err_org_name_required()];
    return;
  }
  step = 2;
}

function handleBack() {
  form.formError = undefined;
  step = 1;
}

async function handleBootstrap(e: SubmitEvent | MouseEvent) {
  if (e) e.preventDefault();
  
  await form.submit(async (values) => {
    const result = await api.bootstrapOrganization({
      orgName: values.orgName.trim(),
      orgNameAr: values.orgNameAr?.trim() || undefined,
      orgType: values.orgType,
      timezone: values.timezone,
      locale: values.locale,
      currency: values.currency,
      adminFullName: values.adminFullName.trim(),
      adminEmail: values.adminEmail.trim(),
      adminPassword: values.adminPassword,
    });

    // Auto-authenticate session with effective context
    await authStore.setSession({
      token: result.token,
      user: {
        id: result.admin_user_id,
        org_id: result.org_id,
        person_id: result.admin_user_id,
        email: values.adminEmail.trim(),
        full_name: values.adminFullName.trim(),
        full_name_ar: values.adminFullName.trim(),
        status: "active",
        is_email_verified: true,
        is_phone_verified: false,
        is_mfa_enabled: false,
      },
      effective_context: {
        effective_role: "super_admin",
        roles: ["super_admin"],
        permissions: ["*"],
        dependents: [],
      },
    });

    goto(localizeHref("/onboarding"), { replaceState: true });
  });
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
    <form onsubmit={handleBootstrap} class="flex-1 p-6 sm:p-8 lg:p-10 flex flex-col justify-between min-w-0">
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

        {#if form.formError}
          <div class="mb-5 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        <!-- STEP 1: Organization Details -->
        {#if step === 1}
          <div class="space-y-4">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <FormField
                id="orgName"
                label={m.setup_bootstrap_org_name_label()}
                error={fieldError("orgName")}
                required
              >
                {#snippet children()}
                  <Input
                    id="orgName"
                    type="text"
                    bind:value={form.values.orgName}
                    disabled={form.isSubmitting}
                    class="h-10 text-xs bg-background border-input"
                    placeholder="e.g. Al-Amal Academy"
                  />
                {/snippet}
              </FormField>

              <FormField
                id="orgNameAr"
                label={m.setup_bootstrap_org_name_ar_label()}
                error={fieldError("orgNameAr")}
              >
                {#snippet children()}
                  <Input
                    id="orgNameAr"
                    type="text"
                    bind:value={form.values.orgNameAr}
                    disabled={form.isSubmitting}
                    class="h-10 text-xs bg-background border-input text-right"
                    placeholder="مثال: أكاديمية الأمل"
                  />
                {/snippet}
              </FormField>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <FormField
                id="orgType"
                label={m.setup_bootstrap_org_type_label()}
                error={fieldError("orgType")}
                required
              >
                {#snippet children()}
                  <Select
                    type="single"
                    value={form.values.orgType}
                    onValueChange={(v : any) => { if (v) form.values.orgType = v; }}
                    disabled={form.isSubmitting}
                  >
                    <SelectTrigger id="orgType" class="h-10 text-xs w-full bg-background border-input">
                      {orgTypes.find(t => t.value === form.values.orgType)?.label || form.values.orgType}
                    </SelectTrigger>
                    <SelectContent>
                      {#each orgTypes as t}
                        <SelectItem value={t.value} class="text-xs">{t.label}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                {/snippet}
              </FormField>

              <FormField
                id="currency"
                label={m.setup_bootstrap_currency_label()}
                error={fieldError("currency")}
                required
              >
                {#snippet children()}
                  <Select
                    type="single"
                    value={form.values.currency}
                    onValueChange={(v : any ) => { if (v) form.values.currency = v; }}
                    disabled={form.isSubmitting}
                  >
                    <SelectTrigger id="currency" class="h-10 text-xs w-full bg-background border-input font-sans">
                      {currencies.find(c => c.code === form.values.currency)?.label || form.values.currency}
                    </SelectTrigger>
                    <SelectContent class="max-h-56">
                      {#each currencies as c}
                        <SelectItem value={c.code} class="text-xs">{c.label}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                {/snippet}
              </FormField>

              <FormField
                id="timezone"
                label={m.setup_bootstrap_timezone_label()}
                error={fieldError("timezone")}
                required
              >
                {#snippet children()}
                  <Select
                    type="single"
                    value={form.values.timezone}
                    onValueChange={(v : any) => { if (v) form.values.timezone = v; }}
                    disabled={form.isSubmitting}
                  >
                    <SelectTrigger id="timezone" class="h-10 text-xs w-full bg-background border-input font-sans">
                      {timezones.find(tz => tz.value === form.values.timezone)?.label || form.values.timezone}
                    </SelectTrigger>
                    <SelectContent class="max-h-56">
                      {#each timezones as tz}
                        <SelectItem value={tz.value} class="text-xs">{tz.label}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                {/snippet}
              </FormField>
            </div>
          </div>

        <!-- STEP 2: Super Admin Account -->
        {:else}
          <div class="space-y-4">
            <FormField
              id="adminFullName"
              label={m.setup_bootstrap_admin_name_label()}
              error={fieldError("adminFullName")}
              required
            >
              {#snippet children()}
                <Input
                  id="adminFullName"
                  type="text"
                  bind:value={form.values.adminFullName}
                  disabled={form.isSubmitting}
                  class="h-10 text-xs bg-background border-input"
                  placeholder="e.g. Abdullah Al-Mansoor"
                />
              {/snippet}
            </FormField>

            <FormField
              id="adminEmail"
              label={m.setup_bootstrap_admin_email_label()}
              error={fieldError("adminEmail")}
              required
            >
              {#snippet children()}
                <Input
                  id="adminEmail"
                  type="email"
                  bind:value={form.values.adminEmail}
                  disabled={form.isSubmitting}
                  class="h-10 text-xs bg-background border-input"
                  placeholder="admin@school.edu"
                />
              {/snippet}
            </FormField>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <FormField
                id="adminPassword"
                label={m.setup_bootstrap_admin_password_label()}
                error={fieldError("adminPassword")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminPassword"
                    type="password"
                    bind:value={form.values.adminPassword}
                    disabled={form.isSubmitting}
                    class="h-10 text-xs bg-background border-input"
                    placeholder="••••••••"
                  />
                {/snippet}
              </FormField>

              <FormField
                id="adminPasswordConfirm"
                label={m.setup_bootstrap_admin_password_confirm_label()}
                error={fieldError("adminPasswordConfirm")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminPasswordConfirm"
                    type="password"
                    bind:value={form.values.adminPasswordConfirm}
                    disabled={form.isSubmitting}
                    class="h-10 text-xs bg-background border-input"
                    placeholder="••••••••"
                  />
                {/snippet}
              </FormField>
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
            disabled={form.isSubmitting}
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
            type="submit"
            variant="default"
            disabled={form.isSubmitting}
            class="inline-flex items-center gap-2 px-6 h-10 text-xs font-semibold cursor-pointer disabled:opacity-50"
          >
            {#if form.isSubmitting}
              <Loader2 class="w-4 h-4 animate-spin" />
              <span>{m.setup_bootstrap_submitting()}</span>
            {:else}
              <ShieldCheck class="w-4 h-4" />
              <span>{m.setup_bootstrap_submit_btn()}</span>
            {/if}
          </Button>
        {/if}
      </div>
    </form>
  </Card>
</div>
