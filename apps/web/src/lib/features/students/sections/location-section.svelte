<!--
  @component LocationSection
  Residential Address & Previous Academic Institution Details.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { Home } from "@orbis/icons";

import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();
</script>

<Card class="w-full">
  <CardHeader class="py-3 px-6 border-b border-border/40">
    <div class="flex items-center gap-2">
      <Home class="w-4 h-4 text-primary" />
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {m.common_form_location_title()}
      </CardTitle>
    </div>
  </CardHeader>

  <CardContent class="p-0">
    <div class="flex flex-col divide-y divide-border">
      <!-- Previous School -->
      {#if state.activeFields.previous_school}
        <div class="px-4 py-3 grid items-start @min-[36rem]/registration-panel:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)] border-b border-border gap-3">
          <div class="flex flex-col gap-1 text-start">
            <Label for="previousSchool" class="text-xs font-semibold text-foreground">{m.common_form_previous_school()}
              {#if state.requiredFields.previous_school}<span class="text-destructive">*</span>{/if}
            </Label>
          </div>
          <div class="w-full min-w-0">
            <Input 
              id="previousSchool"
              type="text" 
              placeholder={m.common_form_location_school_ph()} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.previousSchool}
            />
          </div>
        </div>
      {/if}

      <!-- Governorate -->
      <div class="px-4 py-3 grid items-start @min-[36rem]/registration-panel:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)] border-b border-border gap-3">
        <div class="flex flex-col gap-1 text-start">
          <Label for="governorate" class="text-xs font-semibold text-foreground">{m.common_form_governorate()}</Label>
        </div>
        <div class="w-full min-w-0">
          <Input 
            id="governorate"
            type="text" 
            placeholder={m.common_form_location_gov_ph()} 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.governorate}
          />
        </div>
      </div>

      <!-- District -->
      <div class="px-4 py-3 grid items-start @min-[36rem]/registration-panel:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)] border-b border-border gap-3">
        <div class="flex flex-col gap-1 text-start">
          <Label for="district" class="text-xs font-semibold text-foreground">{m.common_form_district()}</Label>
        </div>
        <div class="w-full min-w-0">
          <Input 
            id="district"
            type="text" 
            placeholder={m.common_form_location_dist_ph()} 
            class="h-9 w-full bg-background" 
            bind:value={state.personal.district}
          />
        </div>
      </div>

      <!-- Full Address -->
      {#if state.activeFields.address}
        <div class="px-4 py-3 grid items-start @min-[36rem]/registration-panel:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)] last:border-b-0 gap-3">
          <div class="flex flex-col gap-1 text-start">
            <Label for="address" class="text-xs font-semibold text-foreground">
              {m.common_form_address()}
              {#if state.requiredFields.address}<span class="text-destructive">*</span>{/if}
            </Label>
          </div>
          <div class="w-full min-w-0">
            <Input 
              id="address"
              type="text" 
              placeholder={m.common_form_location_address_ph()} 
              class="h-9 w-full bg-background" 
              bind:value={state.personal.address}
            />
          </div>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
