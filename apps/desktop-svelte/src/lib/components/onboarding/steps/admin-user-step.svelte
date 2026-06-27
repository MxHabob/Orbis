<!--
  Step 8: Administrator User configuration
  Full name (AR/EN), Email, Phone, and Password.
-->
<script lang="ts">
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { UserCheck, Mail, Phone, Lock } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";
import { getLocale } from "$lib/paraglide/runtime";
</script>

<div class="max-w-3xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif flex items-center gap-2">
      <UserCheck class="w-5 h-5 text-primary" />
      <span>{m.onboarding_admin_title()}</span>
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_admin_desc()}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Arabic Name -->
    <div class="space-y-2 text-start">
      <Label for="adminFullName">{m.onboarding_admin_name_ar()}</Label>
      <div class="relative">
        <Input 
          id="adminFullName" 
          bind:value={onboardingState.adminFullName} 
          placeholder="محمد علي أحمد" 
        />
      </div>
    </div>

    <!-- English Name -->
    <div class="space-y-2 text-start">
      <Label for="adminFullNameEn">{m.onboarding_admin_name_en()}</Label>
      <div class="relative">
        <Input 
          id="adminFullNameEn" 
          bind:value={onboardingState.adminFullNameEn} 
          placeholder="Mohamed Ali Ahmed" 
          class="text-left font-sans" 
          style="direction: ltr;" 
        />
      </div>
    </div>

    <!-- Email -->
    <div class="space-y-2 text-start">
      <Label for="adminEmail">{m.onboarding_admin_email()}</Label>
      <div class="relative">
        <Input 
          id="adminEmail" 
          type="email"
          bind:value={onboardingState.adminEmail} 
          placeholder="admin@school.com" 
          class="text-left font-sans" 
          style="direction: ltr;" 
          required
        />
      </div>
    </div>

    <!-- Phone -->
    <div class="space-y-2 text-start">
      <Label for="adminPhone">{m.onboarding_admin_phone()}</Label>
      <div class="relative">
        <Input 
          id="adminPhone" 
          bind:value={onboardingState.adminPhone} 
          placeholder="777777777" 
          class="text-left font-sans" 
          style="direction: ltr;" 
        />
      </div>
    </div>

    <!-- Password -->
    <div class="space-y-2 text-start md:col-span-2">
      <Label for="adminPassword">{m.onboarding_admin_password()}</Label>
      <div class="relative">
        <Input 
          id="adminPassword" 
          type="password"
          bind:value={onboardingState.adminPassword} 
          placeholder="••••••••" 
          class="text-left font-sans" 
          style="direction: ltr;" 
          required
          minlength={6}
        />
      </div>
      <p class="text-[11px] text-muted-foreground">{m.onboarding_admin_password_hint()}</p>
    </div>
  </div>
</div>
