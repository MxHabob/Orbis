<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";

import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    العنوان والسكن والدراسة السابقة
  </div>

  <div class="flex flex-col">
    <!-- Previous School -->
    {#if state.activeFields.previous_school}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="previousSchool" class="text-xs font-semibold text-foreground">{m.common_form_previous_school()}</Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_previous_school_placeholder()}</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0">
          <Input 
            id="previousSchool"
            type="text" 
            placeholder="اسم المدرسة السابقة..." 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.previousSchool}
          />
        </div>
      </div>
    {/if}

    <!-- Governorate -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="governorate" class="text-xs font-semibold text-foreground">{m.common_form_governorate()}</Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_governorate_placeholder()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0">
        <Input 
          id="governorate"
          type="text" 
          placeholder="مثال: صنعاء، عدن..." 
          class="h-9 w-full bg-background" 
          bind:value={state.personal.governorate}
        />
      </div>
    </div>

    <!-- District -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="district" class="text-xs font-semibold text-foreground">{m.common_form_district()}</Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_district_placeholder()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0">
        <Input 
          id="district"
          type="text" 
          placeholder="مثال: مديرية السبعين..." 
          class="h-9 w-full bg-background" 
          bind:value={state.personal.district}
        />
      </div>
    </div>

    <!-- Full Address -->
    {#if state.activeFields.address}
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="address" class="text-xs font-semibold text-foreground">
            {m.common_form_address()}
            {#if state.requiredFields.address}<span class="text-destructive">*</span>{/if}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_form_address_placeholder()}</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0">
          <Input 
            id="address"
            type="text" 
            placeholder="الشارع، الحي، المعلم القريب..." 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.address}
          />
        </div>
      </div>
    {/if}
  </div>
</div>
