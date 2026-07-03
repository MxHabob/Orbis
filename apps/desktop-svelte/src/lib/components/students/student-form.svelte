<script lang="ts">
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
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@orbis/ui-svelte/dropdown-menu";
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
const state = new StudentFormState(orgId || "", studentId);

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
        variant="ghost" 
        class="text-xs h-9 px-3 gap-1.5 text-muted-foreground hover:text-foreground cursor-pointer font-semibold shrink-0"
        onclick={() => goto(localizeHref(`/${orgId}/students`))}
      >
        <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
      </Button>

      <div class="flex items-center gap-2">
        <!-- Segmented Action Group -->
        <div class="flex items-center border border-border rounded-md bg-background h-8 divide-x divide-border rtl:divide-x-reverse select-none overflow-hidden shrink-0">
          <!-- Print Template Selector -->
          <Select bind:value={state.selectedTemplateId} type="single">
            <SelectTrigger class="w-44 h-8 text-xs bg-transparent border-0 rounded-none shadow-none px-3 py-1 cursor-pointer focus:ring-0 focus-visible:ring-0">
              <span class="truncate">
                {state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name_ar || 
                 state.availableTemplates.find(t => t.id === state.selectedTemplateId)?.name || 
                 m.form_select_template()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
              {#each state.availableTemplates as tpl}
                <SelectItem value={tpl.id} class="text-xs cursor-pointer text-start justify-start">
                  {tpl.name_ar || tpl.name}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>

          <!-- Actions Dropdown -->
          <DropdownMenu>
            <DropdownMenuTrigger>
              <Button variant="ghost" size="icon" class="h-8 w-8 rounded-none border-0 shadow-none select-none hover:bg-accent/40 cursor-pointer">
                <MoreHorizontal class="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="bg-popover border border-border rounded-xl shadow-lg p-1 min-w-[150px] text-start" align="end">
              <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => state.handleDownloadRegistrationForm()}>
                {state.isDownloadingForm ? m.form_printing_registration_form() : m.form_download()}
              </DropdownMenuItem>
              <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5 text-destructive hover:text-destructive" onclick={() => goto(localizeHref(`/${orgId}/students`))}>
                {m.form_cancel_return()}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </div>

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

    <!-- Footer Action Bar (Sleek minimalist style) -->
    <div class="flex items-center justify-end gap-3 mt-6 p-4 border border-border bg-card/10 rounded-md select-none">
      <Button 
        variant="outline" 
        class="text-xs h-9 px-4 cursor-pointer font-semibold shrink-0" 
        onclick={() => goto(localizeHref(`/${orgId}/students`))}
        disabled={state.isSaving}
      >
        {m.form_cancel()}
      </Button>

      <!-- Save Button with Dropdown Trigger -->
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Button 
            variant="lime" 
            class="text-xs h-9 px-5 font-bold shrink-0 gap-1.5 flex items-center justify-center cursor-pointer select-none"
            disabled={state.isSaving}
          >
            <span>{state.isSaving ? m.form_saving() : m.form_save_form()}</span>
            <ChevronDown class="h-4 w-4 shrink-0 text-lime-950/70" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="bg-popover border border-border rounded-xl shadow-lg p-1 min-w-[160px] text-start" align="end">
          <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground border-b border-border/40 mb-1">
            {m.form_options()}
          </div>
          {#if !state.isEditMode}
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "active"; state.handleSave(); }}>
              {m.form_direct_enroll()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "draft"; state.handleSave(); }}>
              {m.form_save_draft()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "pending_review"; state.handleSave(); }}>
              {m.form_submit_review()}
            </DropdownMenuItem>
          {:else}
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "draft"; state.handleSave(); }}>
              {m.form_status_draft()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "pending_review"; state.handleSave(); }}>
              {m.form_status_pending_review()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "approved"; state.handleSave(); }}>
              {m.form_status_approved()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "active"; state.handleSave(); }}>
              {m.form_status_active()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "withdrawn"; state.handleSave(); }}>
              {state.registrationStatus === "withdrawn" ? "تعديل كـ منسحب" : m.form_status_withdrawn()}
            </DropdownMenuItem>
            <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5" onclick={() => { state.registrationStatus = "graduated"; state.handleSave(); }}>
              {state.registrationStatus === "graduated" ? "تعديل كـ خريج" : m.form_status_graduated()}
            </DropdownMenuItem>
          {/if}
          <DropdownMenuItem class="text-xs cursor-pointer py-1.5 px-2.5 hover:bg-accent/40 rounded flex items-center gap-1.5 border-t border-border/40 mt-1" onclick={() => state.handleDownloadRegistrationForm()}>
            {state.isDownloadingForm ? m.form_printing_registration_form() : m.form_download()}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>

  </div>
</div>
