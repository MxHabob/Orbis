<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Label } from "@orbis/ui-svelte/label";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { StudentFormState } from "./student-form-state.svelte";

// Sections
import ProfileSection from "./sections/ProfileSection.svelte";
import FamilySection from "./sections/FamilySection.svelte";
import LocationSection from "./sections/LocationSection.svelte";
import FinancialsSection from "./sections/FinancialsSection.svelte";
import DocumentsSection from "./sections/DocumentsSection.svelte";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

// Instantiate the form state manager
const state = new StudentFormState(orgId, studentId);

onMount(async () => {
  await state.init();
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-foreground text-start" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    
    <!-- Tabs Interface (Clean minimal design matching Organization Settings layout style) -->
    <Tabs value="profile" class="w-full">
      <TabsList class="flex justify-start border-b border-border bg-transparent gap-6 rounded-none p-0 h-auto mb-6">
        <TabsTrigger 
          value="profile" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.form_tab_profile()}
        </TabsTrigger>
        <TabsTrigger 
          value="family" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.form_tab_family()}
        </TabsTrigger>
        <TabsTrigger 
          value="location" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.form_tab_location()}
        </TabsTrigger>
        <TabsTrigger 
          value="financials" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.form_tab_financials()}
        </TabsTrigger>
        <TabsTrigger 
          value="documents" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.form_tab_documents()}
        </TabsTrigger>
      </TabsList>

      <!-- TAB 1: Student Profile -->
      <TabsContent value="profile" class="mt-0">
        <ProfileSection {state} />
      </TabsContent>

      <!-- TAB 2: Family & Guardians -->
      <TabsContent value="family" class="mt-0">
        <FamilySection {state} />
      </TabsContent>

      <!-- TAB 3: Location & Education -->
      <TabsContent value="location" class="mt-0">
        <LocationSection {state} />
      </TabsContent>

      <!-- TAB 4: Financials & Discounts -->
      <TabsContent value="financials" class="mt-0">
        <FinancialsSection {state} />
      </TabsContent>

      <!-- TAB 5: Documents Status -->
      <TabsContent value="documents" class="mt-0">
        <DocumentsSection {state} />
      </TabsContent>
    </Tabs>

    <!-- Footer Action Bar with Template Selection, Status Choice, and Save/Cancel Actions -->
    <div class="flex flex-col sm:flex-row items-center justify-between gap-4 mt-6 p-4 border border-border bg-card/10 rounded-md">
      
      <!-- Template Selector (Left side) -->
      <div class="flex items-center gap-2 w-full sm:w-auto">
        <Label class="text-[10px] font-semibold text-muted-foreground shrink-0 hidden md:inline">
          {m.form_select_template()}:
        </Label>
        <Select bind:value={state.selectedTemplateId} type="single">
          <SelectTrigger class="w-full sm:w-56 h-9 text-xs flex items-center justify-between bg-background border border-input" id="templateSelect">
            <span class="truncate">
              {state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name_ar || 
               state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name || 
               "قالب استمارة تسجيل طالب"}
            </span>
          </SelectTrigger>
          <SelectContent class="text-start max-h-48 overflow-y-auto" align="start">
            {#each state.availableTemplates as tpl}
              <SelectItem value={tpl.id} class="text-xs cursor-pointer text-start justify-start">
                {tpl.name_ar || tpl.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Action buttons & Lifecycle State Selection -->
      <div class="flex items-center justify-end gap-3 w-full sm:w-auto shrink-0 flex-nowrap">
        
        <!-- Print preview -->
        <Button 
          variant="outline" 
          class="text-xs h-9 px-4 gap-1.5 border-primary/30 text-primary hover:bg-primary/5 hover:text-primary cursor-pointer font-semibold shrink-0" 
          onclick={() => state.handleDownloadRegistrationForm()}
          disabled={state.isSaving || state.isDownloadingForm}
        >
          <span>
            {state.isDownloadingForm ? "جاري التوليد..." : m.form_print_registration_form()}
          </span>
        </Button>

        <!-- Lifecycle selection -->
        {#if !state.isEditMode}
          <Select bind:value={state.registrationStatus} type="single">
            <SelectTrigger class="h-9 text-xs bg-background border border-input min-w-36">
              <span>
                {state.registrationStatus === "draft" ? m.form_save_draft() : 
                 state.registrationStatus === "pending_review" ? m.form_submit_review() : 
                 m.form_direct_enroll()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="draft">{m.form_save_draft()}</SelectItem>
              <SelectItem value="pending_review">{m.form_submit_review()}</SelectItem>
              <SelectItem value="active">{m.form_direct_enroll()}</SelectItem>
            </SelectContent>
          </Select>
        {:else}
          <Select bind:value={state.registrationStatus} type="single">
            <SelectTrigger class="h-9 text-xs bg-background border border-input min-w-36">
              <span>
                {state.registrationStatus === "active" ? m.form_status_active() :
                 state.registrationStatus === "pending_review" ? m.form_status_pending_review() :
                 state.registrationStatus === "draft" ? m.form_status_draft() :
                 state.registrationStatus === "withdrawn" ? m.form_status_withdrawn() :
                 state.registrationStatus === "graduated" ? m.form_status_graduated() :
                 state.registrationStatus}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="draft">{m.form_status_draft()}</SelectItem>
              <SelectItem value="pending_review">{m.form_status_pending_review()}</SelectItem>
              <SelectItem value="approved">{m.form_status_approved()}</SelectItem>
              <SelectItem value="active">{m.form_status_active()}</SelectItem>
              <SelectItem value="withdrawn">{m.form_status_withdrawn()}</SelectItem>
              <SelectItem value="graduated">{m.form_status_graduated()}</SelectItem>
            </SelectContent>
          </Select>
        {/if}

        <Button 
          variant="outline" 
          class="text-xs h-9 px-4 cursor-pointer font-semibold shrink-0" 
          onclick={() => goto(localizeHref(`/${orgId}/students`))}
          disabled={state.isSaving}
        >
          {m.form_cancel()}
        </Button>
        
        <Button 
          variant="lime" 
          class="text-xs h-9 px-5 gap-1.5 cursor-pointer font-bold shrink-0" 
          onclick={() => state.handleSave()}
          disabled={state.isSaving}
        >
          {state.isSaving ? m.form_saving() : m.form_save()}
        </Button>
      </div>
    </div>

  </div>
</div>
