<!--
  @component FamilySection
  High-speed, responsive Family & Guardians management section for student registration.
  Uses Orbis ERP standard form controls, real-time parent search, and multi-guardian support.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Plus, Trash, User } from "@orbis/icons";
import { l } from "$lib/core/i18n";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="flex flex-col gap-6">
  <!-- Section Title & Add Button -->
  <div class="flex justify-between items-center px-1">
    <div class="flex flex-col text-start gap-0.5">
      <span class="text-xs font-semibold text-muted-foreground">{m.common_form_guardian_desc()}</span>
    </div>
    <Button 
      type="button" 
      variant="outline" 
      size="sm" 
      onclick={() => state.guardian.addGuardian()} 
      class="h-8 text-xs font-semibold gap-1.5 cursor-pointer"
    >
      <Plus class="w-3.5 h-3.5" />
      <span>{m.common_form_guardian_add()}</span>
    </Button>
  </div>

  {#each state.guardian.linkedGuardians as guardian, index}
    <Card class="w-full">
      <!-- Guardian Block Header -->
      <CardHeader class="py-3 px-6 flex flex-row items-center justify-between border-b border-border/40">
        <div class="flex items-center gap-2">
          <User class="w-4 h-4 text-primary" />
          <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
            {m.common_form_guardian_number()} {index + 1}
            {#if guardian.is_primary}
              <Badge variant="lime" class="ms-2 text-[10px] font-bold">
                {m.common_form_guardian_role_primary()}
              </Badge>
            {/if}
          </CardTitle>
        </div>
        <CardAction>
          {#if state.guardian.linkedGuardians.length > 1}
            <Button 
              type="button" 
              variant="ghost" 
              class="text-destructive hover:bg-destructive/10 h-7 px-2.5 text-xs font-semibold gap-1 cursor-pointer" 
              onclick={() => state.guardian.removeGuardian(index)}
            >
              <Trash class="w-3.5 h-3.5" />
              <span>{m.common_form_guardian_remove()}</span>
            </Button>
          {/if}
        </CardAction>
      </CardHeader>

      <CardContent class="p-0">
        <div class="flex flex-col divide-y divide-border">
          <!-- Autocomplete Search -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_search_label()}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_search_placeholder()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0 relative">
              <Input 
                placeholder={m.common_form_guardian_search_placeholder_ext()} 
                class="h-9 w-full bg-background"
                oninput={(e) => state.guardian.searchParent((e.target as HTMLInputElement).value, index)}
              />
              <!-- Search Results Dropdown -->
              {#if state.guardian.activeSearchIndex === index && state.guardian.parentSearchResults.length > 0}
                <div class="absolute top-full left-0 right-0 z-50 mt-1 bg-popover border border-border rounded-md shadow-lg max-h-48 overflow-y-auto p-1 text-start">
                  {#each state.guardian.parentSearchResults as parent}
                    <button 
                      type="button" 
                      class="w-full text-start p-2 hover:bg-accent/40 rounded text-xs flex justify-between items-center text-foreground font-medium cursor-pointer"
                      onclick={() => state.guardian.selectExistingParent(parent, index)}
                    >
                      <span>{parent.full_name || parent.name} ({parent.phone || m.common_form_guardian_no_phone()})</span>
                      <Badge variant="secondary" class="text-[9px]">{m.common_form_guardian_registered()}</Badge>
                    </button>
                  {/each}
                </div>
              {/if}
            </div>
          </div>

          <!-- Guardian Full Name -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground flex items-center gap-1">
                {m.common_word_fullname()}
                <span class="text-destructive">*</span>
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_fullname_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0">
              <Input bind:value={guardian.full_name} class="h-9 w-full bg-background" placeholder={m.common_guardians_col_name()} />
            </div>
          </div>

          <!-- Phone Number -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground flex items-center gap-1">
                {m.common_word_phone()}
                <span class="text-destructive">*</span>
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_phone_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0">
              <Input bind:value={guardian.phone} class="h-9 w-full bg-background font-mono text-left" placeholder="777000000" />
            </div>
          </div>

          <!-- Relationship -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{m.common_bulk_col_guardian_rel()}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_rel_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0">
              <Select 
                type="single"
                value={guardian.relationship}
                onValueChange={(val) => {
                  guardian.relationship = val || "other";
                  guardian.relationship_ar = val === "father" ? "أب" : val === "mother" ? "أم" : val === "uncle" ? "عم" : "آخر";
                }}
              >
                <SelectTrigger class="h-9 w-full bg-background flex justify-between items-center text-xs">
                  <span>
                    {#if guardian.relationship === "father"}
                      {m.common_guardians_relationship_father()}
                    {:else if guardian.relationship === "mother"}
                      {m.common_guardians_relationship_mother()}
                    {:else if guardian.relationship === "uncle"}
                      {m.students_family_uncle()}
                    {:else}
                      {m.common_guardians_relationship_other()}
                    {/if}
                  </span>
                </SelectTrigger>
                <SelectContent class="text-start">
                  <SelectItem value="father" class="text-xs">{m.common_guardians_relationship_father()}</SelectItem>
                  <SelectItem value="mother" class="text-xs">{m.common_guardians_relationship_mother()}</SelectItem>
                  <SelectItem value="uncle" class="text-xs">{m.students_family_uncle()}</SelectItem>
                  <SelectItem value="other" class="text-xs">{m.common_guardians_relationship_other()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- National ID -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_national_id_label()}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_nid_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0">
              <Input bind:value={guardian.national_id} class="h-9 w-full bg-background font-mono" placeholder={m.common_form_guardian_national_id_label()} />
            </div>
          </div>

          <!-- Occupations / Job & Employer -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_job_label()}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_job_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0 grid grid-cols-1 sm:grid-cols-2 gap-2">
              <Input bind:value={guardian.occupation} class="h-9 w-full bg-background" placeholder={m.common_form_guardian_job_placeholder()} />
              <Input bind:value={guardian.employer} class="h-9 w-full bg-background" placeholder={l("جهة العمل / الشركة", "Employer / Company")} />
            </div>
          </div>

          <!-- Work Phone -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{l("هاتف العمل", "Work Phone")}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{l("رقم هاتف مقر عمل ولي الأمر للضرورة", "Guardian workplace phone number")}</span>
            </div>
            <div class="w-full md:w-120 shrink-0">
              <Input bind:value={guardian.work_phone} class="h-9 w-full bg-background font-mono text-left" placeholder="01-234567" />
            </div>
          </div>

          <!-- Permissions Checkboxes -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_perms_label()}</Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_perms_desc()}</span>
            </div>
            <div class="w-full md:w-120 shrink-0 flex flex-col gap-4">
              <div class="grid grid-cols-3 gap-3">
                <div class="flex items-center gap-2">
                  <Checkbox id={`primary_${index}`} checked={guardian.is_primary} onCheckedChange={(val) => guardian.is_primary = !!val} />
                  <Label for={`primary_${index}`} class="text-xs cursor-pointer">{m.common_form_guardian_role_primary()}</Label>
                </div>

                <div class="flex items-center gap-2">
                  <Checkbox id={`emergency_${index}`} checked={guardian.is_emergency_contact} onCheckedChange={(val) => guardian.is_emergency_contact = !!val} />
                  <Label for={`emergency_${index}`} class="text-xs cursor-pointer">{m.common_form_guardian_role_emergency()}</Label>
                </div>

                <div class="flex items-center gap-2">
                  <Checkbox id={`payer_${index}`} checked={guardian.is_payer} onCheckedChange={(val) => guardian.is_payer = !!val} />
                  <Label for={`payer_${index}`} class="text-xs cursor-pointer">{m.common_form_guardian_role_payer()}</Label>
                </div>

                <div class="flex items-center gap-2">
                  <Checkbox id={`pickup_${index}`} checked={guardian.is_authorized_pickup} onCheckedChange={(val) => guardian.is_authorized_pickup = !!val} />
                  <Label for={`pickup_${index}`} class="text-xs cursor-pointer">{l("مصرح بالاستلام", "Authorized Pickup")}</Label>
                </div>
              </div>

              {#if guardian.is_payer}
                <div class="flex items-center justify-start gap-2 pt-2 border-t border-border/50">
                  <Label class="text-xs text-muted-foreground">{m.common_form_guardian_payment_share()}</Label>
                  <div class="flex items-center gap-1">
                    <Input type="number" min="0" max="100" class="h-8 w-16 text-center font-bold px-1 bg-background" bind:value={guardian.payment_share_percent} />
                    <span class="text-xs text-muted-foreground">%</span>
                  </div>
                </div>
              {/if}
            </div>
          </div>

        </div>
      </CardContent>
    </Card>
  {/each}
</div>
