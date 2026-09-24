<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import { toast } from "@orbis/ui";

import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { FloppyDisk } from "@orbis/icons";
import { userCreateInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let { orgId }: { orgId: string } = $props();

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

const form = new FormController({
  initialValues: {
    full_name: "",
    name_ar: null as string | null,
    email: "",
    password: "",
    primary_role: "teacher" as
      | "admin"
      | "teacher"
      | "parent"
      | "student"
      | "staff",
    sex: "male" as "male" | "female" | null,
    phone: null as string | null,
    national_id: null as string | null,
    date_of_birth: null as string | null,
    address: null as string | null,
  },
  schema: userCreateInputSchema,
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

function getRoleLabel(roleStr: string): string {
  switch (roleStr) {
    case "admin":
      return m.common_users_filter_admin();
    case "teacher":
      return m.common_schedule_page_teacher();
    case "parent":
      return m.common_users_filter_guardian();
    case "student":
      return m.common_word_student_single();
    case "staff":
      return m.common_user_form_role_staff();
    default:
      return roleStr;
  }
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();

  isSubmitting = true;
  try {
    form.values = {
      full_name: fullName,
      name_ar: nameAr,
      email,
      password,
      primary_role: role as
        | "admin"
        | "teacher"
        | "parent"
        | "student"
        | "staff",
      sex: sex as "male" | "female" | null,
      phone,
      national_id: nationalId,
      date_of_birth: dob,
      address,
    };
    const submitted = await form.submit(async (request) => {
      const newUser = await api.registerUser(request);
      const updateRequest: Record<string, any> = {};
      if (request.full_name.trim())
        updateRequest.name_en = request.full_name.trim();
      if (request.address) updateRequest.address = request.address;
      if (Object.keys(updateRequest).length > 0) {
        await api.updateUser(newUser.id, updateRequest as any);
      }
    });
    if (!submitted) {
      toast.error(
        Object.values(form.errors).flat()[0] || "بيانات المستخدم غير صالحة",
      );
      return;
    }
    toast.success(m.settings_users_create_success());
    goto(localizeHref(`/${orgId}/settings/users`));
  } catch (e) {
    toast.error(e instanceof Error ? e.message : String(e));
  } finally {
    isSubmitting = false;
  }
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden">
  <DetailPageHeader
    backHref={`/${orgId}/settings/users`}
    backLabel={m.settings_nav_users()}
  >
    <div class="flex flex-col text-start">
      <h1 class="text-sm font-bold tracking-tight text-foreground select-none">
        {m.common_user_form_title_create()}
      </h1>
      <p class="text-[10px] text-muted-foreground select-none">
        {m.common_user_form_desc_create()}
      </p>
    </div>
  </DetailPageHeader>

  <!-- Scrollable Form Container -->
  <div class="flex-1 overflow-y-auto scrollbar-hide">
    <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
      <form onsubmit={handleSubmit} class="flex flex-col gap-6 text-start">
        
        <!-- Section 1: Personal Information -->
        <Card>
          <CardHeader>
            <CardTitle>{m.common_user_form_personal_info()}</CardTitle>
          </CardHeader>
          
          <CardContent class="p-6 grid grid-cols-1 md:grid-cols-2 gap-6">
            <FormField id="usr_name_ar" label={m.common_word_fullname()} error={fieldError("name_ar")}>
              {#snippet children()}
                <Input id="usr_name_ar" type="text" bind:value={nameAr} placeholder={m.common_user_form_full_name_placeholder()} disabled={isSubmitting} class="h-9 text-xs" />
              {/snippet}
            </FormField>
            <FormField id="usr_full_name" label={m.common_user_form_name_en()} error={fieldError("full_name")} required>
              {#snippet children()}
                <Input id="usr_full_name" type="text" bind:value={fullName} placeholder={m.common_form_full_name_en_placeholder()} required disabled={isSubmitting} class="h-9 text-xs font-mono" />
              {/snippet}
            </FormField>
            <FormField id="usr_sex" label={m.common_form_sex()} error={fieldError("sex")} required>
              {#snippet children()}
                <Select type="single" value={sex} onValueChange={(v: string | undefined) => (sex = v ?? "male")} disabled={isSubmitting}>
                  <SelectTrigger id="usr_sex" class="h-9 w-full text-xs">{sex === "male" ? m.common_form_male() : m.common_form_female()}</SelectTrigger>
                  <SelectContent>
                    <SelectItem value="male" class="text-xs">{m.common_form_male()}</SelectItem>
                    <SelectItem value="female" class="text-xs">{m.common_form_female()}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>
            <FormField id="usr_dob" label={m.common_bulk_col_dob()} error={fieldError("date_of_birth")}>
              {#snippet children()}
                <Input id="usr_dob" type="date" bind:value={dob} disabled={isSubmitting} class="h-9 text-xs" />
              {/snippet}
            </FormField>
            <FormField id="usr_nid" label={m.common_user_form_national_id()} error={fieldError("national_id")}>
              {#snippet children()}
                <Input id="usr_nid" type="text" bind:value={nationalId} placeholder="100020003" disabled={isSubmitting} class="h-9 text-xs font-mono" />
              {/snippet}
            </FormField>
          </CardContent>
        </Card>

        <!-- Section 2: Account & Role -->
        <Card>
          <CardHeader>
            <CardTitle>{m.common_user_form_account_info()}</CardTitle>
          </CardHeader>
          
          <CardContent class="p-6 grid grid-cols-1 md:grid-cols-2 gap-6">
            <FormField id="usr_email" label={m.common_word_email()} error={fieldError("email")} required>
              {#snippet children()}
                <Input id="usr_email" type="email" bind:value={email} placeholder={m.common_user_form_email_placeholder()} required disabled={isSubmitting} class="h-9 text-xs font-mono" />
              {/snippet}
            </FormField>
            <FormField id="usr_password" label={m.common_user_form_password()} error={fieldError("password")} required>
              {#snippet children()}
                <Input id="usr_password" type="password" bind:value={password} placeholder={m.common_user_form_password_placeholder()} required disabled={isSubmitting} class="h-9 text-xs" />
              {/snippet}
            </FormField>
            <FormField id="usr_role" label={m.common_user_form_role()} error={fieldError("primary_role")} required>
              {#snippet children()}
                <Select type="single" value={role} onValueChange={(v: string | undefined) => (role = v ?? "teacher")} disabled={isSubmitting}>
                  <SelectTrigger id="usr_role" class="h-9 w-full text-xs">{getRoleLabel(role)}</SelectTrigger>
                  <SelectContent>
                    <SelectItem value="admin" class="text-xs">{m.common_users_filter_admin()}</SelectItem>
                    <SelectItem value="teacher" class="text-xs">{m.common_schedule_page_teacher()}</SelectItem>
                    <SelectItem value="parent" class="text-xs">{m.common_users_filter_guardian()}</SelectItem>
                    <SelectItem value="student" class="text-xs">{m.common_word_student_single()}</SelectItem>
                    <SelectItem value="staff" class="text-xs">{m.common_user_form_role_staff()}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>
          </CardContent>
        </Card>

        <!-- Section 3: Contact Information -->
        <Card>
          <CardHeader>
            <CardTitle>{m.common_user_form_contact_info()}</CardTitle>
          </CardHeader>
          
          <CardContent class="p-6 grid grid-cols-1 md:grid-cols-2 gap-6">
            <FormField id="usr_phone" label={m.common_word_phone()} error={fieldError("phone")}>
              {#snippet children()}
                <Input id="usr_phone" type="tel" bind:value={phone} placeholder="770000000" disabled={isSubmitting} class="h-9 text-xs font-mono" />
              {/snippet}
            </FormField>
            <FormField id="usr_address" label={m.common_word_address()} error={fieldError("address")}>
              {#snippet children()}
                <Input id="usr_address" type="text" bind:value={address} placeholder={m.common_user_form_address_placeholder()} disabled={isSubmitting} class="h-9 text-xs" />
              {/snippet}
            </FormField>
          </CardContent>
        </Card>

        <!-- Bottom Actions Bar -->
        <div class="flex justify-end gap-3 pb-8">
          <Button
            type="button"
            variant="outline"
            class="text-xs h-9 px-5 cursor-pointer"
            onclick={() => goto(localizeHref(`/${orgId}/settings/users`))}
            disabled={isSubmitting}
          >
            {m.common_action_cancel()}
          </Button>
          <Button
            type="submit"
            variant="lime"
            class="text-xs h-9 px-6 gap-1.5 font-semibold"
            disabled={isSubmitting}
          >
            <FloppyDisk size={14} />
            {isSubmitting ? m.common_state_saving() : m.common_action_save()}
          </Button>
        </div>
      </form>
    </div>
  </div>
</div>
