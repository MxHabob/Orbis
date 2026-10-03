<!--
  @component StepAdmin
  Step 4: Initial Primary Administrator Account.
  Spacious 2-column layout with live @orbis/avatar-engine preview, real-time password strength,
  and elevated design tokens.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OnboardingFormModel } from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Avatar, AvatarController } from "@orbis/avatar-engine";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
  isRTL = false,
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
  isRTL?: boolean;
} = $props();

const avatarController = new AvatarController();

const passwordStrength = $derived.by(() => {
  const pwd = form.adminPassword || "";
  let score = 0;
  if (pwd.length >= 8) score++;
  if (pwd.length >= 12) score++;
  if (/[A-Z]/.test(pwd)) score++;
  if (/[0-9]/.test(pwd)) score++;
  if (/[^A-Za-z0-9]/.test(pwd)) score++;
  return score;
});

const strengthLabel = $derived.by(() => {
  if (!form.adminPassword) return { text: "", color: "bg-muted", width: "0%" };
  if (passwordStrength <= 2) return { text: m.ui_onboarding_admin_weak(), color: "bg-destructive", width: "30%" };
  if (passwordStrength <= 3) return { text: m.ui_onboarding_admin_fair(), color: "bg-amber-500", width: "60%" };
  if (passwordStrength <= 4) return { text: m.ui_onboarding_admin_good(), color: "bg-blue-500", width: "80%" };
  return { text: m.ui_onboarding_admin_strong(), color: "bg-emerald-500", width: "100%" };
});
</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {m.ui_onboarding_admin_step_4_of_6()}
      </span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {m.ui_onboarding_admin_primary_administrator_account()}
    </h2>
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {isRTL
        ? "سيتم منح هذا الحساب صلاحيات الإشراف والتحكم الكاملة بجميع وحدات وقواعد بيانات المنظومة."
        : "This master account will possess unrestricted authority across security policies and core operations."}
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 text-start">
    <!-- Left Column: Avatar & Role Identity Preview -->
    <div class="lg:col-span-4 rounded-2xl border border-border/70 bg-card p-6 flex flex-col items-center text-center justify-between shadow-xs">
      <div class="space-y-4 w-full flex flex-col items-center">
        <div class="size-20 rounded-full bg-background border-2 border-primary/30 flex items-center justify-center overflow-hidden shadow-sm ring-4 ring-primary/10">
          <Avatar
            controller={avatarController}
            width="72px"
            height="72px"
          />
        </div>

        <div class="space-y-1 w-full">
          <div class="text-sm font-bold text-foreground truncate">
            {form.adminFirstNameAr || form.adminFirstName || (m.ui_onboarding_admin_administrator())} {form.adminLastNameAr || form.adminLastName || ""}
          </div>
          <div class="text-xs font-mono text-muted-foreground truncate">
            {form.adminEmail || "admin@institution.edu"}
          </div>
        </div>
      </div>

      <div class="w-full mt-6 pt-4 border-t border-border/40 text-xs text-muted-foreground leading-relaxed">
        {isRTL
          ? "يتم تشفير وتأمين كلمات المرور محلياً عبر خوارزمية Bcrypt مع توثيق كامل للعمليات الإدارية."
          : "Credentials are cryptographically hashed on-device using Bcrypt with full audit logging."}
      </div>
    </div>

    <!-- Right Column: Form Inputs -->
    <div class="lg:col-span-8 rounded-2xl border border-border/70 bg-card p-6 space-y-5 shadow-xs">
      <div class="border-b border-border/40 pb-3">
        <h3 class="text-sm font-bold text-foreground uppercase tracking-wider font-mono">
          {m.ui_onboarding_admin_administrator_credentials()}
        </h3>
      </div>

      <!-- Arabic Names -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-fn-ar" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_first_name_arabic()}
          </Label>
          <Input
            id="admin-fn-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: طارق"
            bind:value={form.adminFirstNameAr}
            class="h-9 text-xs"
          />
        </div>

        <div class="space-y-1.5">
          <Label for="admin-ln-ar" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_last_name_arabic()}
          </Label>
          <Input
            id="admin-ln-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: المنصور"
            bind:value={form.adminLastNameAr}
            class="h-9 text-xs"
          />
        </div>
      </div>

      <!-- English Names -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-fn-en" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_first_name_english()}
          </Label>
          <Input
            id="admin-fn-en"
            type="text"
            placeholder="e.g. Tariq"
            bind:value={form.adminFirstName}
            class="h-9 text-xs"
          />
          {#if errors.adminFirstName}
            <p class="text-[11px] text-destructive">{errors.adminFirstName}</p>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="admin-ln-en" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_last_name_english()}
          </Label>
          <Input
            id="admin-ln-en"
            type="text"
            placeholder="e.g. Al-Mansoor"
            bind:value={form.adminLastName}
            class="h-9 text-xs"
          />
          {#if errors.adminLastName}
            <p class="text-[11px] text-destructive">{errors.adminLastName}</p>
          {/if}
        </div>
      </div>

      <!-- Email Address -->
      <div class="space-y-1.5">
        <Label for="admin-email" class="text-xs font-medium text-foreground">
          {m.ui_onboarding_admin_primary_email_address()}
        </Label>
        <Input
          id="admin-email"
          type="email"
          placeholder="admin@institution.edu"
          bind:value={form.adminEmail}
          class="h-9 text-xs font-mono"
        />
        {#if errors.adminEmail}
          <p class="text-[11px] text-destructive">{errors.adminEmail}</p>
        {/if}
      </div>

      <!-- Password and Confirmation -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-pwd" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_password()}
          </Label>
          <Input
            id="admin-pwd"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminPassword}
            class="h-9 text-xs"
          />
          {#if errors.adminPassword}
            <p class="text-[11px] text-destructive">{errors.adminPassword}</p>
          {/if}

          <!-- Real-Time Password Strength Meter -->
          {#if form.adminPassword}
            <div class="space-y-1 pt-1">
              <div class="h-1.5 w-full bg-muted rounded-full overflow-hidden">
                <div
                  class="h-full {strengthLabel.color} transition-all duration-300"
                  style="width: {strengthLabel.width};"
                ></div>
              </div>
              <div class="flex justify-between items-center text-[10px] text-muted-foreground font-mono">
                <span>{m.ui_onboarding_admin_strength()}</span>
                <span class="font-bold">{strengthLabel.text}</span>
              </div>
            </div>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="admin-pwd-confirm" class="text-xs font-medium text-foreground">
            {m.ui_onboarding_admin_confirm_password()}
          </Label>
          <Input
            id="admin-pwd-confirm"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminConfirmPassword}
            class="h-9 text-xs"
          />
          {#if errors.adminConfirmPassword}
            <p class="text-[11px] text-destructive">{errors.adminConfirmPassword}</p>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
