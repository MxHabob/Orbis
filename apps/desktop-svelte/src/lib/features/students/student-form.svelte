<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { ArrowLeft, MoreHorizontal, ChevronDown } from "lucide-svelte";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@orbis/ui-svelte/dropdown-menu";

import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { StudentFormState } from "./form-state.svelte";
import { createClassesQuery } from "$lib/features/academics";
import { useQueryClient } from "@tanstack/svelte-query";

// Sections
import ProfileSection from "./sections/profile-section.svelte";
import FamilySection from "./sections/family-section.svelte";
import LocationSection from "./sections/location-section.svelte";
import FinancialsSection from "./sections/financials-section.svelte";
import DocumentsSection from "./sections/documents-section.svelte";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

// Instantiate the form state manager
const state = new StudentFormState(orgId || "", studentId);
const queryClient = useQueryClient();

async function handleSave(status?: string) {
  if (status) state.registrationStatus = status;
  await state.handleSave(queryClient);
}

// Fetch classes via TanStack Query (complying with AGENTS.md §8.1)
const classesQuery = createClassesQuery(() => orgId);

$effect(() => {
  if ($classesQuery.data) {
    state.classes = $classesQuery.data;
  }
});

onMount(async () => {
  if (orgId) {
    state.orgId = orgId;
  }
  if (studentId) {
    state.studentId = studentId;
    state.isEditMode = true;
  }
  await state.init();
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-foreground text-start" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    
    <!-- Header: Simple Back button and Actions dropdown -->
    <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0 border-b border-border/60 pb-3 mb-2 select-none">
      <Button 
        variant="secondary" 
        class="text-xs h-8 px-2 gap-1.5 shrink-0"
        onclick={() => goto(localizeHref(`/${orgId}/students`))}
      >
        <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
      </Button>

      <div class="flex items-center gap-2">
        <!-- Segmented Print Template Selector & Actions Dropdown -->
        <DropdownMenu>
          <DropdownMenuTrigger>
            <div class="flex items-center border border-border rounded-md bg-background h-8 divide-x divide-border rtl:divide-x-reverse select-none overflow-hidden shrink-0 cursor-pointer hover:bg-muted/30">
              <span class="text-xs px-3 py-1 text-foreground font-medium truncate max-w-40">
                {state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name_ar || 
                 state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name || 
                 m.common_form_select_template()}
              </span>
              <div class="h-8 w-8 flex items-center justify-center text-muted-foreground">
                <MoreHorizontal class="h-4 w-4" />
              </div>
            </div>
          </DropdownMenuTrigger>
          <DropdownMenuContent class="p-1 min-w-[180px] text-start" align="end">
            {#each state.availableTemplates as tpl}
              <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center justify-between gap-1.5" onclick={async () => { state.selectedTemplateId = tpl.id; await state.handleDownloadRegistrationForm(); }}>
                <span>{tpl.name_ar || tpl.name}</span>
                {#if state.selectedTemplateId === tpl.id}
                  <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check text-primary h-3.5 w-3.5"><path d="M20 6 9 17l-5-5"/></svg>
                {/if}
              </DropdownMenuItem>
            {/each}
            {#if state.availableTemplates.length === 0}
              <div class="px-3 py-4 text-center text-xs text-muted-foreground">
                {m.common_form_no_templates_available()}
              </div>
            {/if}
            <div class="border-t border-border/40 my-1"></div>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5 text-destructive hover:text-destructive" onclick={() => goto(localizeHref(`/${orgId}/students`))}>
              {m.common_action_cancel()}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>

    <!-- Tabs Interface (Clean minimal design matching Organization Settings layout style) -->
    <Tabs value="profile" class="w-full">
      <TabsList class="flex justify-start border-b border-border bg-transparent gap-6 rounded-none p-0 h-auto mb-6">
        <TabsTrigger 
          value="profile" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.common_form_tab_profile()}
        </TabsTrigger>
        <TabsTrigger 
          value="family" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.common_form_tab_family()}
        </TabsTrigger>
        <TabsTrigger 
          value="location" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.common_form_tab_location()}
        </TabsTrigger>
        <TabsTrigger 
          value="financials" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.common_form_tab_financials()}
        </TabsTrigger>
        <TabsTrigger 
          value="documents" 
          class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
        >
          {m.common_form_tab_documents()}
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

    <!-- Footer Action Bar (Sleek minimalist style) -->
    <div class="flex items-center justify-end gap-3 mt-6 p-4 border border-border bg-card/10 rounded-md select-none">
      <Button 
        variant="outline" 
        class="text-xs h-9 px-4 cursor-pointer font-semibold shrink-0" 
        onclick={() => goto(localizeHref(`/${orgId}/students`))}
        disabled={state.isSaving}
      >
        {m.common_action_cancel()}
      </Button>

      <!-- Separate dropdown button for changing status -->
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Button 
            size="icon"
            class="text-xs h-9 w-9 shrink-0"
            disabled={state.isSaving}
          >
            <ChevronDown class="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="p-1 min-w-[160px] text-start" align="end">
          {#if !state.isEditMode}
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("draft")}>
              {m.common_action_save()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("pending_review")}>
              {m.common_form_submit_review()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("approved")}>
              {m.common_form_direct_enroll()}
            </DropdownMenuItem>
          {:else}
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("draft")}>
              {m.common_status_draft()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("pending_review")}>
              {m.common_form_status_pending_review()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("approved")}>
              {m.common_form_status_approved()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("withdrawn")}>
              {state.registrationStatus === "withdrawn" ? m.common_form_edit_as_withdrawn() : m.common_status_withdrawn()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5" onclick={() => handleSave("graduated")}>
              {state.registrationStatus === "graduated" ? m.common_form_edit_as_graduated() : m.common_form_status_graduated()}
            </DropdownMenuItem>
          {/if}
          <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 flex items-center gap-1.5 border-t border-border/40 mt-1" onclick={() => state.handleDownloadRegistrationForm()}>
            {state.isDownloadingForm ? m.common_form_printing_registration_form() : m.common_form_download()}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>

  </div>
</div>
