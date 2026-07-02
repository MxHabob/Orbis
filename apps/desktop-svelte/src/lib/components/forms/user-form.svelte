<script lang="ts">
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import type { User } from "$lib/services/ipc";

let {
  orgId,
  user = null,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  user?: User | null;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

const isEdit = $derived(!!user);

// Form state
let isSubmitting = $state(false);
let fullName = $state(user?.full_name || "");
let nameAr = $state(user?.name_ar || "");
let email = $state(user?.email || "");
let password = $state("");
let role = $state(user?.primary_role || user?.role || "teacher");
let phone = $state((user?.phone as string) || "");
let sex = $state((user?.sex as string) || "male");
let nationalId = $state((user?.national_id as string) || "");
let dob = $state((user?.date_of_birth as string) || "");

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!fullName) {
    toast.error("Full name is required");
    return;
  }

  isSubmitting = true;
  try {
    if (isEdit && user) {
      const request = {
        full_name: fullName,
        name_ar: nameAr || null,
        email: email || null,
        phone: phone || null,
        sex: sex || null,
        national_id: nationalId || null,
        date_of_birth: dob || null,
      };
      await commands.users.updateUser(user.id, request);

      // Update role if changed
      if (role !== (user.primary_role || user.role)) {
        await commands.users.changeUserRole(user.id, { role });
      }

      toast.success(m.settings_users_update_success());
    } else {
      if (!password) {
        toast.error("Password is required for new users");
        isSubmitting = false;
        return;
      }

      const createdBy = authStore.userId || "unknown";
      const request = {
        email,
        password,
        full_name: fullName,
        name_ar: nameAr || null,
        primary_role: role,
        sex: sex || null,
        phone: phone || null,
        national_id: nationalId || null,
        date_of_birth: dob || null,
      };
      await commands.users.createUser(orgId, createdBy, request);
      toast.success(m.settings_users_create_success());
    }

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-5 scrollbar-hide text-start">
  <!-- Names (Arabic & English) -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Arabic Name -->
    <div class="flex flex-col gap-2">
      <label for="usr_name_ar" class="text-xs font-semibold text-foreground">
        {m.user_form_full_name()}
      </label>
      <Input
        id="usr_name_ar"
        type="text"
        bind:value={nameAr}
        placeholder={m.user_form_full_name_placeholder()}
        disabled={isSubmitting}
      />
    </div>

    <!-- English / Full Name -->
    <div class="flex flex-col gap-2">
      <label for="usr_full_name" class="text-xs font-semibold text-foreground">
        {m.user_form_name_en()} <span class="text-destructive">*</span>
      </label>
      <Input
        id="usr_full_name"
        type="text"
        bind:value={fullName}
        placeholder={m.user_form_name_en_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>
  </div>

  <!-- Email & Password -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Email -->
    <div class="flex flex-col gap-2">
      <label for="usr_email" class="text-xs font-semibold text-foreground">
        {m.user_form_email()} <span class="text-destructive">*</span>
      </label>
      <Input
        id="usr_email"
        type="email"
        bind:value={email}
        placeholder={m.user_form_email_placeholder()}
        required
        disabled={isSubmitting}
      />
    </div>

    <!-- Password (only for creation) -->
    {#if !isEdit}
      <div class="flex flex-col gap-2">
        <label for="usr_password" class="text-xs font-semibold text-foreground">
          {m.user_form_password()} <span class="text-destructive">*</span>
        </label>
        <Input
          id="usr_password"
          type="password"
          bind:value={password}
          placeholder={m.user_form_password_placeholder()}
          required={!isEdit}
          disabled={isSubmitting}
        />
      </div>
    {:else}
      <div class="flex flex-col gap-2 opacity-50">
        <label class="text-xs font-semibold text-foreground">
          {m.user_form_password()}
        </label>
        <Input
          type="text"
          value="•••••••• (Manage under Security)"
          disabled={true}
        />
      </div>
    {/if}
  </div>

  <!-- Role & Phone -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- Role Selection -->
    <div class="flex flex-col gap-2">
      <label for="usr_role" class="text-xs font-semibold text-foreground">
        {m.user_form_role()}
      </label>
      <select
        id="usr_role"
        bind:value={role}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="admin">{m.user_form_role_admin()}</option>
        <option value="teacher">{m.user_form_role_teacher()}</option>
        <option value="parent">{m.user_form_role_guardian()}</option>
        <option value="student">{m.user_form_role_student()}</option>
        <option value="staff">Staff</option>
      </select>
    </div>

    <!-- Phone -->
    <div class="flex flex-col gap-2">
      <label for="usr_phone" class="text-xs font-semibold text-foreground">
        {m.user_form_phone()}
      </label>
      <Input
        id="usr_phone"
        type="tel"
        bind:value={phone}
        placeholder="770000000"
        disabled={isSubmitting}
      />
    </div>
  </div>

  <!-- Sex, National ID, DOB -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <!-- Sex -->
    <div class="flex flex-col gap-2">
      <label for="usr_sex" class="text-xs font-semibold text-foreground">
        {m.user_form_sex()}
      </label>
      <select
        id="usr_sex"
        bind:value={sex}
        class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
        disabled={isSubmitting}
      >
        <option value="male">{m.user_form_sex_male()}</option>
        <option value="female">{m.user_form_sex_female()}</option>
      </select>
    </div>

    <!-- National ID -->
    <div class="flex flex-col gap-2">
      <label for="usr_nid" class="text-xs font-semibold text-foreground">
        {m.user_form_national_id()}
      </label>
      <Input
        id="usr_nid"
        type="text"
        bind:value={nationalId}
        placeholder="100020003"
        disabled={isSubmitting}
      />
    </div>

    <!-- DOB -->
    <div class="flex flex-col gap-2">
      <label for="usr_dob" class="text-xs font-semibold text-foreground">
        {m.user_form_dob()}
      </label>
      <Input
        id="usr_dob"
        type="date"
        bind:value={dob}
        disabled={isSubmitting}
      />
    </div>
  </div>

  <!-- Actions -->
  <div class="flex gap-2 justify-end mt-auto pt-6 border-t border-border/40">
    <Button
      type="button"
      variant="ghost"
      onclick={onCancel}
      disabled={isSubmitting}
    >
      {m.user_form_cancel()}
    </Button>
    <Button
      type="submit"
      variant="lime"
      class="font-semibold px-6"
      disabled={isSubmitting}
    >
      {isSubmitting ? m.user_form_saving() : m.user_form_save()}
    </Button>
  </div>
</form>
