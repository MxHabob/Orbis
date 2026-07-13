<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="flex flex-col gap-6">
  <!-- Section Title & Add Button -->
  <div class="flex justify-between items-center px-2">
    <div class="flex flex-col text-start gap-0.5">
      <span class="text-xs font-semibold text-muted-foreground">{m.common_form_guardian_desc()}</span>
    </div>
    <Button type="button" variant="outline" size="sm" onclick={() => state.guardian.addGuardianField()} class="h-8 text-xs">
      {m.common_form_guardian_add()}
    </Button>
  </div>

  {#each state.guardian.linkedGuardians as guardian, index}
    <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
      <!-- Guardian Block Header -->
      <div class="bg-muted/50 px-6 py-3 border-b border-border flex justify-between items-center select-none">
        <span class="text-[11px] font-bold text-foreground uppercase tracking-wider">
          {m.common_form_guardian_number()} {index + 1}
        </span>
        {#if index > 0}
          <Button 
            type="button" 
            variant="ghost" 
            class="text-destructive hover:bg-destructive/10 h-7 px-2.5 text-xs font-semibold" 
            onclick={() => state.guardian.removeGuardianField(index)}
          >
            {m.common_form_guardian_remove()}
          </Button>
        {/if}
      </div>

      <div class="flex flex-col">
        <!-- Autocomplete Search -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_search_label()}</Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_guardian_search_placeholder()}</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0 relative">
            <Input 
              placeholder="ابحث بالاسم أو رقم الهاتف..." 
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
                    <span>{parent.full_name} ({parent.phone || 'بلا هاتف'})</span>
                    <Badge variant="secondary" class="text-[9px]">ولي أمر مسجل</Badge>
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
            <span class="text-[11px] text-muted-foreground leading-relaxed">الاسم الرباعي الكامل لولي الأمر</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0">
            <Input bind:value={guardian.full_name} class="h-9 w-full bg-background" placeholder="اسم ولي الأمر" />
          </div>
        </div>

        <!-- Phone Number -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground flex items-center gap-1">
              {m.common_word_phone()}
              <span class="text-destructive">*</span>
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">رقم هاتف المحمول الفعّال لولي الأمر للتواصل الطارئ</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0">
            <Input bind:value={guardian.phone} class="h-9 w-full bg-background font-mono text-left" dir="ltr" placeholder="777000000" />
          </div>
        </div>

        <!-- Relationship -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">{m.common_bulk_col_guardian_rel()}</Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">صلة القرابة بين ولي الأمر والطالب</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0">
            <Select bind:value={guardian.relationship} type="single">
              <SelectTrigger class="h-9 w-full bg-background flex justify-between items-center">
                <span>{guardian.relationship === "father" ? "أب" : guardian.relationship === "mother" ? "أم" : guardian.relationship === "uncle" ? "عم" : "آخر"}</span>
              </SelectTrigger>
              <SelectContent class="text-start">
                <SelectItem value="father" onclick={() => guardian.relationship_ar = "أب"}>أب</SelectItem>
                <SelectItem value="mother" onclick={() => guardian.relationship_ar = "أم"}>أم</SelectItem>
                <SelectItem value="uncle" onclick={() => guardian.relationship_ar = "عم"}>عم</SelectItem>
                <SelectItem value="other" onclick={() => guardian.relationship_ar = "قريب/آخر"}>آخر</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        <!-- National ID -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">{m.common_form_guardian_national_id_label()}</Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">رقم الهوية الوطنية أو البطاقة الشخصية لولي الأمر</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0">
            <Input bind:value={guardian.national_id} class="h-9 w-full bg-background font-mono" placeholder="رقم هوية ولي الأمر" />
          </div>
        </div>

        <!-- Occupations / Job -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">المهنة / الوظيفة</Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">مهنة ولي الأمر الحالية وجهة العمل</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0">
            <Input bind:value={guardian.occupation} class="h-9 w-full bg-background" placeholder="مهنة ولي الأمر" />
          </div>
        </div>

        <!-- Permissions Checkboxes -->
        <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">الصلاحيات والمسؤوليات المالية</Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">تحديد أدوار ولي الأمر تجاه الطالب وتوزيع الحصص المالية</span>
          </div>
          <div class="w-full md:w-[480px] shrink-0 flex flex-col gap-4">
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
    </div>
  {/each}
</div>
