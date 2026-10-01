<!--
  @component StepAdmin
  Step 4: Initial Primary Administrator Account.
  Minimalist form using @orbis/ui components, design tokens, and avatar preview.
-->
<script lang="ts">
import type { OnboardingFormModel } from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Card, CardContent } from "@orbis/ui/card";
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
  if (passwordStrength <= 2) return { text: isRTL ? "ضعيفة" : "Weak", color: "bg-destructive", width: "30%" };
  if (passwordStrength <= 3) return { text: isRTL ? "متوسطة" : "Fair", color: "bg-amber-500", width: "60%" };
  if (passwordStrength <= 4) return { text: isRTL ? "جيدة" : "Good", color: "bg-blue-500", width: "80%" };
  return { text: isRTL ? "قوية" : "Strong", color: "bg-emerald-500", width: "100%" };
});

const passwordsMatch = $derived(
  !form.adminConfirmPassword || form.adminPassword === form.adminConfirmPassword
);
</script>

<div class="space-y-6 max-w-2xl mx-auto py-1">
  <div class="space-y-1 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {isRTL ? "المرحلة ٤ من ٥" : "Step 4 of 5"}
      </span>
    </div>
    <h2 class="text-xl font-bold tracking-tight text-foreground">
      {isRTL ? "إنشاء حساب المشرف العام" : "Primary Administrator Account"}
    </h2>
    <p class="text-xs text-muted-foreground">
      {isRTL
        ? "سيتم منح هذا الحساب صلاحيات الإدارة الكاملة للتحكم بجميع وحدات النظام."
        : "This account will be granted full administrative authority across all modules."}
    </p>
  </div>

  <Card class="border-border/60 shadow-md bg-card/95">
    <CardContent class="p-6 space-y-5 text-start">
      <!-- Avatar Header Preview -->
      <div class="flex items-center gap-4 p-3.5 rounded-xl bg-muted/20 border border-border/50">
        <div class="size-12 rounded-full bg-background border border-border/60 flex items-center justify-center shrink-0 overflow-hidden shadow-xs">
          <Avatar
            controller={avatarController}
            width="44px"
            height="44px"
          />
        </div>
        <div class="space-y-0.5 text-start overflow-hidden">
          <div class="text-xs font-bold text-foreground truncate">
            {form.adminFirstNameAr || form.adminFirstName || (isRTL ? "المشرف العام" : "Administrator")} {form.adminLastNameAr || form.adminLastName || ""}
          </div>
          <div class="text-[11px] font-mono text-muted-foreground truncate">
            {form.adminEmail || "admin@institution.edu"}
          </div>
        </div>
      </div>

      <!-- Admin Names -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-fn-ar" class="text-xs font-medium text-foreground">
            {isRTL ? "الاسم الأول (عربي) *" : "First Name (Arabic) *"}
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
            {isRTL ? "اسم العائلة (عربي) *" : "Last Name (Arabic) *"}
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

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="admin-fn-en" class="text-xs font-medium text-foreground">
            {isRTL ? "الاسم الأول (إنجليزي) *" : "First Name (English) *"}
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
            {isRTL ? "اسم العائلة (إنجليزي) *" : "Last Name (English) *"}
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

      <div class="border-t border-border/40 pt-4 grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Email -->
        <div class="space-y-1.5">
          <Label for="admin-email" class="text-xs font-medium text-foreground">
            {isRTL ? "البريد الإلكتروني *" : "Email Address *"}
          </Label>
          <Input
            id="admin-email"
            type="email"
            placeholder="admin@institution.edu"
            bind:value={form.adminEmail}
            class="h-9 text-xs"
          />
          {#if errors.adminEmail}
            <p class="text-[11px] text-destructive">{errors.adminEmail}</p>
          {/if}
        </div>

        <!-- Phone -->
        <div class="space-y-1.5">
          <Label for="admin-phone" class="text-xs font-medium text-foreground">
            {isRTL ? "رقم الهاتف (اختياري)" : "Phone (Optional)"}
          </Label>
          <Input
            id="admin-phone"
            type="tel"
            placeholder="+966 5X XXX XXXX"
            bind:value={form.adminPhone}
            class="h-9 text-xs"
          />
        </div>
      </div>

      <!-- Password & Confirm -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-1">
        <div class="space-y-1.5">
          <div class="flex items-center justify-between">
            <Label for="admin-pwd" class="text-xs font-medium text-foreground">
              {isRTL ? "كلمة المرور *" : "Password *"}
            </Label>
            {#if form.adminPassword && strengthLabel.text}
              <span class="text-[10px] text-muted-foreground">{strengthLabel.text}</span>
            {/if}
          </div>
          <Input
            id="admin-pwd"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminPassword}
            class="h-9 text-xs"
          />
          {#if form.adminPassword}
            <div class="h-1 w-full bg-muted rounded-full overflow-hidden mt-1">
              <div
                class="h-full {strengthLabel.color} transition-all duration-300"
                style="width: {strengthLabel.width};"
              ></div>
            </div>
          {/if}
          {#if errors.adminPassword}
            <p class="text-[11px] text-destructive">{errors.adminPassword}</p>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="admin-pwd-confirm" class="text-xs font-medium text-foreground">
            {isRTL ? "تأكيد كلمة المرور *" : "Confirm Password *"}
          </Label>
          <Input
            id="admin-pwd-confirm"
            type="password"
            placeholder="••••••••••••"
            bind:value={form.adminConfirmPassword}
            class="h-9 text-xs {!passwordsMatch ? 'border-destructive' : ''}"
          />
          {#if !passwordsMatch}
            <p class="text-[11px] text-destructive">
              {isRTL ? "كلمتا المرور غير متطابقتين" : "Passwords do not match"}
            </p>
          {/if}
          {#if errors.adminConfirmPassword}
            <p class="text-[11px] text-destructive">{errors.adminConfirmPassword}</p>
          {/if}
        </div>
      </div>
    </CardContent>
  </Card>
</div>
