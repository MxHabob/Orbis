<!--
  @component StepAdmin
  Step 3: Initial Primary Administrator Account Setup.
-->
<script lang="ts">
import type { OnboardingFormModel } from "../types";
import { Eye, EyeSlash, LockClosed, ShieldCheck, User, UserBadgeCheck } from "@orbis/icons";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
  isRTL = false,
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
  isRTL?: boolean;
} = $props();

let showPassword = $state(false);
let showConfirmPassword = $state(false);

const passwordStrength = $derived.by(() => {
  const pwd = form.adminPassword || "";
  let score = 0;
  if (pwd.length >= 8) score++;
  if (pwd.length >= 12) score++;
  if (/[A-Z]/.test(pwd)) score++;
  if (/[0-9]/.test(pwd)) score++;
  if (/[^A-Za-z0-9]/.test(pwd)) score++;
  return score; // 0 to 5
});

const strengthLabel = $derived.by(() => {
  if (!form.adminPassword) return { text: isRTL ? "غير محدد" : "None", color: "bg-muted text-muted-foreground", width: "0%" };
  if (passwordStrength <= 2) return { text: isRTL ? "ضعيفة" : "Weak", color: "bg-destructive text-destructive", width: "30%" };
  if (passwordStrength <= 3) return { text: isRTL ? "متوسطة" : "Fair", color: "bg-amber-500 text-amber-500", width: "60%" };
  if (passwordStrength <= 4) return { text: isRTL ? "جيدة" : "Good", color: "bg-blue-500 text-blue-500", width: "80%" };
  return { text: isRTL ? "قوية جداً" : "Strong", color: "bg-emerald-500 text-emerald-500", width: "100%" };
});

const passwordsMatch = $derived(
  !form.adminConfirmPassword || form.adminPassword === form.adminConfirmPassword
);
</script>

<div class="space-y-6">
  <div class="text-center max-w-xl mx-auto space-y-2">
    <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">
      <UserBadgeCheck size={14} />
      <span>{isRTL ? "الخطوة الثالثة" : "Step 3 of 4"}</span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isRTL ? "إنشاء حساب المشرف العام الأول" : "Create Primary Administrator"}
    </h2>
    <p class="text-sm text-muted-foreground">
      {isRTL
        ? "سيتم منح هذا الحساب صلاحيات الإدارة الكاملة (Super Administrator) للتحكم بالنظام."
        : "This account will be provisioned with full system ownership and administrative permissions."}
    </p>
  </div>

  <div class="max-w-2xl mx-auto space-y-6 bg-card/60 border border-border/80 rounded-2xl p-6 shadow-sm backdrop-blur-sm">
    <!-- Admin Name Section -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 pb-2 border-b border-border/60">
        <User size={16} class="text-primary" />
        <h3 class="text-sm font-semibold text-foreground">
          {isRTL ? "الاسم والمعلومات الشخصية" : "Administrator Name & Profile"}
        </h3>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- English First Name -->
        <div class="space-y-1.5">
          <Label for="admin-fn-en" class="text-xs font-medium text-foreground">
            {isRTL ? "الاسم الأول (إنجليزي) *" : "First Name (English) *"}
          </Label>
          <Input
            id="admin-fn-en"
            type="text"
            placeholder="e.g. Tariq"
            bind:value={form.adminFirstName}
            class="h-10 text-sm"
          />
          {#if errors.adminFirstName}
            <p class="text-[11px] text-destructive">{errors.adminFirstName}</p>
          {/if}
        </div>

        <!-- English Last Name -->
        <div class="space-y-1.5">
          <Label for="admin-ln-en" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم العائلة (إنجليزي) *" : "Last Name (English) *"}
          </Label>
          <Input
            id="admin-ln-en"
            type="text"
            placeholder="e.g. Al-Mansoor"
            bind:value={form.adminLastName}
            class="h-10 text-sm"
          />
          {#if errors.adminLastName}
            <p class="text-[11px] text-destructive">{errors.adminLastName}</p>
          {/if}
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Arabic First Name -->
        <div class="space-y-1.5">
          <Label for="admin-fn-ar" class="text-xs font-medium text-foreground">
            {isRTL ? "الاسم الأول (عربي)" : "First Name (Arabic)"}
          </Label>
          <Input
            id="admin-fn-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: طارق"
            bind:value={form.adminFirstNameAr}
            class="h-10 text-sm"
          />
        </div>

        <!-- Arabic Last Name -->
        <div class="space-y-1.5">
          <Label for="admin-ln-ar" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم العائلة (عربي)" : "Last Name (Arabic)"}
          </Label>
          <Input
            id="admin-ln-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: المنصور"
            bind:value={form.adminLastNameAr}
            class="h-10 text-sm"
          />
        </div>
      </div>
    </div>

    <!-- Contact & Credentials Section -->
    <div class="space-y-4 pt-2">
      <div class="flex items-center gap-2 pb-2 border-b border-border/60">
        <ShieldCheck size={16} class="text-primary" />
        <h3 class="text-sm font-semibold text-foreground">
          {isRTL ? "بيانات الدخول والحماية" : "Authentication & Security"}
        </h3>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Email -->
        <div class="space-y-1.5">
          <Label for="admin-email" class="text-xs font-medium text-foreground">
            {isRTL ? "البريد الإلكتروني للإدارة *" : "Administrator Email *"}
          </Label>
          <Input
            id="admin-email"
            type="email"
            placeholder="admin@institution.edu"
            bind:value={form.adminEmail}
            class="h-10 text-sm"
          />
          {#if errors.adminEmail}
            <p class="text-[11px] text-destructive">{errors.adminEmail}</p>
          {/if}
        </div>

        <!-- Phone -->
        <div class="space-y-1.5">
          <Label for="admin-phone" class="text-xs font-medium text-foreground">
            {isRTL ? "رقم الهاتف / الجوال (اختياري)" : "Mobile Phone (Optional)"}
          </Label>
          <Input
            id="admin-phone"
            type="tel"
            placeholder="+966 5X XXX XXXX"
            bind:value={form.adminPhone}
            class="h-10 text-sm"
          />
        </div>
      </div>

      <!-- Password -->
      <div class="space-y-1.5">
        <div class="flex items-center justify-between">
          <Label for="admin-pwd" class="text-xs font-medium text-foreground">
            {isRTL ? "كلمة المرور *" : "Password *"}
          </Label>
          {#if form.adminPassword}
            <span class="text-[10px] font-medium {strengthLabel.color.split(' ')[1]}">
              {isRTL ? "القوة:" : "Strength:"} {strengthLabel.text}
            </span>
          {/if}
        </div>
        <div class="relative">
          <Input
            id="admin-pwd"
            type={showPassword ? "text" : "password"}
            placeholder="••••••••••••"
            bind:value={form.adminPassword}
            class="h-10 text-sm pe-10"
          />
          <button
            type="button"
            onclick={() => (showPassword = !showPassword)}
            class="absolute top-1/2 -translate-y-1/2 {isRTL ? 'left-3' : 'right-3'} text-muted-foreground hover:text-foreground p-1 transition-colors"
          >
            {#if showPassword}
              <EyeSlash size={16} />
            {:else}
              <Eye size={16} />
            {/if}
          </button>
        </div>

        <!-- Strength Meter Bar -->
        {#if form.adminPassword}
          <div class="h-1.5 w-full bg-muted rounded-full overflow-hidden mt-1.5">
            <div
              class="h-full {strengthLabel.color.split(' ')[0]} transition-all duration-300"
              style="width: {strengthLabel.width};"
            ></div>
          </div>
        {/if}

        {#if errors.adminPassword}
          <p class="text-[11px] text-destructive">{errors.adminPassword}</p>
        {/if}
      </div>

      <!-- Confirm Password -->
      <div class="space-y-1.5">
        <Label for="admin-pwd-confirm" class="text-xs font-medium text-foreground">
          {isRTL ? "تأكيد كلمة المرور *" : "Confirm Password *"}
        </Label>
        <div class="relative">
          <Input
            id="admin-pwd-confirm"
            type={showConfirmPassword ? "text" : "password"}
            placeholder="••••••••••••"
            bind:value={form.adminConfirmPassword}
            class="h-10 text-sm pe-10 {!passwordsMatch ? 'border-destructive focus-visible:ring-destructive' : ''}"
          />
          <button
            type="button"
            onclick={() => (showConfirmPassword = !showConfirmPassword)}
            class="absolute top-1/2 -translate-y-1/2 {isRTL ? 'left-3' : 'right-3'} text-muted-foreground hover:text-foreground p-1 transition-colors"
          >
            {#if showConfirmPassword}
              <EyeSlash size={16} />
            {:else}
              <Eye size={16} />
            {/if}
          </button>
        </div>
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
  </div>
</div>
