<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { Save, User } from "lucide-svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);

// Form state
let isSubmitting = $state(false);

let fullName = $state("");
let nameAr = $state("");
let email = $state("");
let password = $state("");
let role = $state("teacher");
let phone = $state("");
let sex = $state("male");
let nationalId = $state("");
let dob = $state("");
let address = $state("");

function getRoleLabel(roleStr: string): string {
  switch (roleStr) {
    case "admin":
      return m.user_form_role_admin();
    case "teacher":
      return m.user_form_role_teacher();
    case "parent":
      return m.user_form_role_guardian();
    case "student":
      return m.user_form_role_student();
    case "staff":
      return m.user_form_role_staff();
    default:
      return roleStr;
  }
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!fullName.trim()) {
    toast.error("Full name is required");
    return;
  }
  if (!email.trim()) {
    toast.error("Email is required");
    return;
  }
  if (!password) {
    toast.error("Password is required");
    return;
  }

  isSubmitting = true;
  try {
    const createdBy = authStore.userId || "unknown";

    // 1. Send the primary create request to backend
    const request = {
      email: email.trim(),
      password,
      full_name: fullName.trim(),
      name_ar: nameAr.trim() || null,
      primary_role: role,
      sex: sex || null,
      phone: phone.trim() || null,
      national_id: nationalId.trim() || null,
      date_of_birth: dob || null,
    };

    const newUser = await commands.iam.createUser(orgId, createdBy, request);

    // 2. Update extra fields (name_en, address) since they are not in the primary createUser endpoint
    const updateRequest: Record<string, any> = {};
    if (fullName.trim()) {
      updateRequest.name_en = fullName.trim();
    }
    if (address.trim()) {
      updateRequest.address = address.trim();
    }

    if (Object.keys(updateRequest).length > 0) {
      await commands.iam.updateUser(newUser.id, updateRequest);
    }

    toast.success(m.settings_users_create_success());

    // Navigate back to the users directory
    goto(localizeHref(`/${orgId}/settings/users`));
  } catch (e) {
    const errorMsg = (e as any)?.message || String(e);
    toast.error(errorMsg);
  } finally {
    isSubmitting = false;
  }
}
</script>

<svelte:head>
  <title>{m.user_form_title_create()} | Orbis</title>
</svelte:head>

<div class="h-full w-full flex flex-col overflow-hidden">
  <!-- Sticky Page Header -->
  <DetailPageHeader
    backHref={`/${orgId}/settings/users`}
    backLabel={m.settings_nav_users()}
  >
    <div class="flex flex-col text-start">
      <h1 class="text-sm font-bold tracking-tight text-foreground select-none">
        {m.user_form_title_create()}
      </h1>
      <p class="text-[10px] text-muted-foreground select-none">
        {m.user_form_desc_create()}
      </p>
    </div>
  </DetailPageHeader>

  <!-- Scrollable Form Container -->
  <div class="flex-1 overflow-y-auto scrollbar-hide">
    <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
      <form onsubmit={handleSubmit} class="flex flex-col gap-6 text-start">
        
        <!-- Section 1: Personal Information -->
        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
          <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">{m.user_form_personal_info()}</h2>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Name in Arabic -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_name_ar" class="text-xs font-medium text-muted-foreground">
                {m.user_form_full_name()}
              </Label>
              <Input
                id="usr_name_ar"
                type="text"
                bind:value={nameAr}
                placeholder={m.user_form_full_name_placeholder()}
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs"
              />
            </div>

            <!-- Name in English -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_full_name" class="text-xs font-medium text-muted-foreground">
                {m.user_form_name_en()} <span class="text-destructive">*</span>
              </Label>
              <Input
                id="usr_full_name"
                type="text"
                bind:value={fullName}
                placeholder={m.user_form_name_en_placeholder()}
                required
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs font-mono"
                dir="ltr"
              />
            </div>

            <!-- Sex / Gender -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_sex" class="text-xs font-medium text-muted-foreground">
                {m.user_form_sex()}
              </Label>
              <Select
                type="single"
                value={sex}
                onValueChange={(v) => (sex = v ?? "male")}
                disabled={isSubmitting}
              >
                <SelectTrigger id="usr_sex" class="h-9 w-full text-xs bg-background/50">
                  {sex === "male" ? m.user_form_sex_male() : m.user_form_sex_female()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="male" class="text-xs">{m.user_form_sex_male()}</SelectItem>
                  <SelectItem value="female" class="text-xs">{m.user_form_sex_female()}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Date of Birth -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_dob" class="text-xs font-medium text-muted-foreground">
                {m.user_form_dob()}
              </Label>
              <Input
                id="usr_dob"
                type="date"
                bind:value={dob}
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs"
              />
            </div>

            <!-- National ID -->
            <div class="flex flex-col gap-2 text-start md:col-span-2">
              <Label for="usr_nid" class="text-xs font-medium text-muted-foreground">
                {m.user_form_national_id()}
              </Label>
              <Input
                id="usr_nid"
                type="text"
                bind:value={nationalId}
                placeholder="100020003"
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs"
              />
            </div>
          </div>
        </div>

        <!-- Section 2: Account & Role -->
        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
          <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">{m.user_form_account_info()}</h2>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Email -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_email" class="text-xs font-medium text-muted-foreground">
                {m.user_form_email()} <span class="text-destructive">*</span>
              </Label>
              <Input
                id="usr_email"
                type="email"
                bind:value={email}
                placeholder={m.user_form_email_placeholder()}
                required
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs font-mono"
                dir="ltr"
              />
            </div>

            <!-- Password -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_password" class="text-xs font-medium text-muted-foreground">
                {m.user_form_password()} <span class="text-destructive">*</span>
              </Label>
              <Input
                id="usr_password"
                type="password"
                bind:value={password}
                placeholder={m.user_form_password_placeholder()}
                required
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs"
              />
            </div>

            <!-- Role Selection -->
            <div class="flex flex-col gap-2 text-start md:col-span-2">
              <Label for="usr_role" class="text-xs font-medium text-muted-foreground">
                {m.user_form_role()}
              </Label>
              <Select
                type="single"
                value={role}
                onValueChange={(v) => (role = v ?? "teacher")}
                disabled={isSubmitting}
              >
                <SelectTrigger id="usr_role" class="h-9 w-full text-xs bg-background/50">
                  {getRoleLabel(role)}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="admin" class="text-xs">{m.user_form_role_admin()}</SelectItem>
                  <SelectItem value="teacher" class="text-xs">{m.user_form_role_teacher()}</SelectItem>
                  <SelectItem value="parent" class="text-xs">{m.user_form_role_guardian()}</SelectItem>
                  <SelectItem value="student" class="text-xs">{m.user_form_role_student()}</SelectItem>
                  <SelectItem value="staff" class="text-xs">{m.user_form_role_staff()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

        <!-- Section 3: Contact Information -->
        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
          <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">{m.user_form_contact_info()}</h2>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Phone Number -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_phone" class="text-xs font-medium text-muted-foreground">
                {m.user_form_phone()}
              </Label>
              <Input
                id="usr_phone"
                type="tel"
                bind:value={phone}
                placeholder="770000000"
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs font-mono"
                dir="ltr"
              />
            </div>

            <!-- Address -->
            <div class="flex flex-col gap-2 text-start">
              <Label for="usr_address" class="text-xs font-medium text-muted-foreground">
                {m.user_form_address()}
              </Label>
              <Input
                id="usr_address"
                type="text"
                bind:value={address}
                placeholder={m.user_form_address_placeholder()}
                disabled={isSubmitting}
                class="bg-background/50 h-9 text-xs"
              />
            </div>
          </div>
        </div>

        <!-- Bottom Actions Bar -->
        <div class="flex justify-end gap-3 pb-8">
          <Button
            type="button"
            variant="outline"
            class="text-xs h-9 px-5 cursor-pointer"
            onclick={() => goto(localizeHref(`/${orgId}/settings/users`))}
            disabled={isSubmitting}
          >
            {m.user_form_cancel()}
          </Button>
          <Button
            type="submit"
            variant="lime"
            class="text-xs h-9 px-6 gap-1.5 font-semibold"
            disabled={isSubmitting}
          >
            <Save size={14} />
            {isSubmitting ? m.user_form_saving() : m.user_form_save()}
          </Button>
        </div>
      </form>
    </div>
  </div>
</div>
