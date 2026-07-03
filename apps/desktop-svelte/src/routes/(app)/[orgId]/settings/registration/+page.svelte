<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import { toast } from "svelte-sonner";
import { onMount } from "svelte";
import { Save, HelpCircle, Plus, Trash2 } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let orgId = $derived(page.params.orgId);

let isLoading = $state(true);
let isSaving = $state(false);

// Config fields
let institutionType = $state("school");
let studentNumberPattern = $state("STU-{YEAR}-{SEQ}");
let rawStages = $state<string[]>(["primary", "middle", "high"]);
let stageInput = $state("");
let requiredDocuments = $state<string[]>([
  "Birth Certificate",
  "National ID or Passport",
]);
let documentInput = $state("");

// Field configurations
let requiredFields = $state<Record<string, boolean>>({
  national_id: true,
  email: false,
  phone: true,
  birth_date: true,
  sex: true,
  address: false,
  mother_name: false,
  previous_school: false,
  documents: true,
});

let activeFields = $state<Record<string, boolean>>({
  national_id: true,
  email: true,
  phone: true,
  birth_date: true,
  sex: true,
  address: true,
  mother_name: true,
  previous_school: true,
  documents: true,
});

let preventDuplicates = $state<Record<string, boolean>>({
  national_id: true,
  email: false,
  phone: false,
});

let originalOrg: any = null;

onMount(async () => {
  if (!orgId) return;
  try {
    originalOrg = await commands.users.getOrganization(orgId);
    if (originalOrg && originalOrg.settings) {
      try {
        const settings = JSON.parse(originalOrg.settings);
        const regConfig = settings.registration_config || {};

        institutionType = regConfig.institution_type || "school";
        studentNumberPattern =
          regConfig.student_number_pattern || "STU-{YEAR}-{SEQ}";
        rawStages = regConfig.stages || ["primary", "middle", "high"];
        requiredDocuments = regConfig.required_documents || [
          "Birth Certificate",
          "National ID or Passport",
        ];

        if (regConfig.required_fields) {
          requiredFields = { ...requiredFields, ...regConfig.required_fields };
        }
        if (regConfig.active_fields) {
          activeFields = { ...activeFields, ...regConfig.active_fields };
        }
        if (regConfig.prevent_duplicates) {
          preventDuplicates = {
            ...preventDuplicates,
            ...regConfig.prevent_duplicates,
          };
        }
      } catch (err) {
        console.error("Failed to parse settings JSON:", err);
      }
    }
  } catch (e) {
    console.error("Failed to load organization settings:", e);
    toast.error("فشل في تحميل الإعدادات");
  } finally {
    isLoading = false;
  }
});

function addStage() {
  const clean = stageInput.trim().toLowerCase();
  if (clean && !rawStages.includes(clean)) {
    rawStages = [...rawStages, clean];
    stageInput = "";
  }
}

function removeStage(stage: string) {
  rawStages = rawStages.filter((s) => s !== stage);
}

function addDocument() {
  const clean = documentInput.trim();
  if (clean && !requiredDocuments.includes(clean)) {
    requiredDocuments = [...requiredDocuments, clean];
    documentInput = "";
  }
}

function removeDocument(doc: string) {
  requiredDocuments = requiredDocuments.filter((d) => d !== doc);
}

async function handleSave(e: Event) {
  e.preventDefault();
  if (!orgId || !originalOrg) return;
  isSaving = true;

  try {
    let settingsObj: Record<string, any> = {};
    if (originalOrg.settings) {
      try {
        settingsObj = JSON.parse(originalOrg.settings);
      } catch {
        settingsObj = {};
      }
    }

    settingsObj.registration_config = {
      institution_type: institutionType,
      student_number_pattern: studentNumberPattern,
      stages: rawStages,
      required_fields: requiredFields,
      active_fields: activeFields,
      prevent_duplicates: preventDuplicates,
      required_documents: requiredDocuments,
    };

    const request = {
      name: originalOrg.name,
      logo: originalOrg.logo || null,
      primary_color: originalOrg.primary_color,
      currency: originalOrg.currency,
      timezone: originalOrg.timezone,
      locale: originalOrg.locale,
      date_format: originalOrg.date_format,
      hijri_enabled: originalOrg.hijri_enabled,
      settings: JSON.stringify(settingsObj),
    };

    await commands.users.updateOrganization(orgId, request);

    // Update local cache
    originalOrg.settings = request.settings;

    toast.success(m.settings_registration_save_success());
  } catch (e: any) {
    console.error("Failed to save registration settings:", e);
    toast.error(
      m.settings_registration_save_error({ error: e.message || String(e) }),
    );
  } finally {
    isSaving = false;
  }
}
</script>

<svelte:head>
  <title>قواعد وإعدادات التسجيل | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-start text-foreground" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    {#if isLoading}
      <div class="space-y-6">
        <div class="h-32 bg-card border border-border rounded-xl animate-pulse"></div>
        <div class="h-48 bg-card border border-border rounded-xl animate-pulse"></div>
      </div>
    {:else}
      <form onsubmit={handleSave} class="flex flex-col gap-6">
        
        <Tabs value="general" class="w-full">
          <TabsList class="flex justify-start border-b border-border bg-transparent gap-6 rounded-none p-0 h-auto mb-6">
            <TabsTrigger 
              value="general" 
              class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
            >
              {m.settings_registration_tab_general()}
            </TabsTrigger>
            <TabsTrigger 
              value="stages_docs" 
              class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
            >
              {m.settings_registration_tab_stages_docs()}
            </TabsTrigger>
            <TabsTrigger 
              value="fields" 
              class="text-xs font-semibold py-2 px-1 border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent rounded-none shadow-none text-muted-foreground data-[state=active]:text-foreground bg-transparent transition-all duration-150 cursor-pointer"
            >
              {m.settings_registration_tab_fields()}
            </TabsTrigger>
          </TabsList>

          <!-- TAB 1: General Settings -->
          <TabsContent value="general" class="mt-0">
            <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
              <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                {m.settings_registration_type_and_numbering()}
              </div>

              <div class="flex flex-col">
                <!-- Institution Type -->
                <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
                  <div class="flex flex-col gap-1 text-start">
                    <Label for="inst_type" class="text-xs font-semibold text-foreground">
                      {m.settings_registration_inst_type()}
                    </Label>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">
                      {getLocale() === "ar" ? "تحديد نوع المؤسسة التعليمية لتطبيق المسميات المناسبة" : "Select educational institution type to apply correct naming conventions"}
                    </span>
                  </div>
                  <div class="w-full md:w-[320px] shrink-0">
                    <Select bind:value={institutionType} type="single">
                      <SelectTrigger id="inst_type" class="h-9 text-xs flex justify-between items-center border border-input bg-background w-full">
                        <span>
                          {institutionType === "school" ? m.settings_registration_school() : 
                           institutionType === "university" ? m.settings_registration_university() :
                           institutionType === "institute" ? m.settings_registration_institute() :
                           m.settings_registration_training_center()}
                        </span>
                      </SelectTrigger>
                      <SelectContent class="text-start">
                        <SelectItem value="school">{m.settings_registration_school()}</SelectItem>
                        <SelectItem value="university">{m.settings_registration_university()}</SelectItem>
                        <SelectItem value="institute">{m.settings_registration_institute()}</SelectItem>
                        <SelectItem value="training_center">{m.settings_registration_training_center()}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>

                <!-- Student Number Pattern -->
                <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b-0 gap-4">
                  <div class="flex flex-col gap-1 text-start">
                    <Label for="num_pattern" class="text-xs font-semibold text-foreground">
                      {m.settings_registration_num_pattern()}
                    </Label>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">
                      {m.settings_registration_pattern_desc()}
                    </span>
                  </div>
                  <div class="w-full md:w-[320px] shrink-0">
                    <Input id="num_pattern" bind:value={studentNumberPattern} class="h-9 text-xs border border-input bg-background w-full" />
                  </div>
                </div>
              </div>
            </div>
          </TabsContent>

          <!-- TAB 2: Academic Stages & Required Documents -->
          <TabsContent value="stages_docs" class="mt-0 flex flex-col gap-6">
            <!-- Academic Stages -->
            <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
              <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                {m.settings_registration_stages_title()}
              </div>

              <div class="flex flex-col px-6 py-4 gap-4">
                <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                  <div class="flex flex-col gap-1 text-start">
                    <Label class="text-xs font-semibold text-foreground">
                      {getLocale() === "ar" ? "إضافة مرحلة أكاديمية جديدة" : "Add New Academic Stage"}
                    </Label>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">
                      {getLocale() === "ar" ? "حدد فئة أو مرحلة دراسية لتخصيص الرسوم والقبول لها" : "Define a class category or stage to customize fees and admission rules"}
                    </span>
                  </div>
                  <div class="w-full md:w-[320px] shrink-0 flex gap-2">
                    <Input 
                      placeholder={m.settings_registration_add_stage_placeholder()} 
                      bind:value={stageInput} 
                      class="h-9 text-xs flex-1 border border-input bg-background"
                      onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addStage())}
                    />
                    <Button type="button" variant="outline" size="sm" class="cursor-pointer font-semibold shrink-0" onclick={addStage}>
                      <Plus size={14} class="me-1.5" />
                      {getLocale() === "ar" ? "إضافة" : "Add"}
                    </Button>
                  </div>
                </div>

                <div class="flex flex-wrap gap-2 pt-2 border-t border-border/40">
                  {#each rawStages as stage}
                    <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-background text-xs text-foreground select-none">
                      <span class="font-medium">{stage}</span>
                      <button 
                        type="button" 
                        class="text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                        onclick={() => removeStage(stage)}
                      >
                        <Trash2 size={12} />
                      </button>
                    </div>
                  {/each}
                  {#if rawStages.length === 0}
                    <p class="text-xs text-muted-foreground py-1">
                      {m.settings_registration_no_stages()}
                    </p>
                  {/if}
                </div>
              </div>
            </div>

            <!-- Required Documents Checklist -->
            <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
              <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                {m.settings_registration_docs_title()}
              </div>

              <div class="flex flex-col px-6 py-4 gap-4">
                <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                  <div class="flex flex-col gap-1 text-start">
                    <Label class="text-xs font-semibold text-foreground">
                      {getLocale() === "ar" ? "إضافة مستند مطلوب" : "Add Required Document"}
                    </Label>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">
                      {getLocale() === "ar" ? "حدد المستندات الرسمية التي يجب على الطالب إرفاقها عند التسجيل" : "Specify the official documents that the student must attach during registration"}
                    </span>
                  </div>
                  <div class="w-full md:w-[320px] shrink-0 flex gap-2">
                    <Input 
                      placeholder={m.settings_registration_add_doc_placeholder()} 
                      bind:value={documentInput} 
                      class="h-9 text-xs flex-1 border border-input bg-background"
                      onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addDocument())}
                    />
                    <Button type="button" variant="outline" size="sm" class="cursor-pointer font-semibold shrink-0" onclick={addDocument}>
                      <Plus size={14} class="me-1.5" />
                      {getLocale() === "ar" ? "إضافة" : "Add"}
                    </Button>
                  </div>
                </div>

                <div class="flex flex-wrap gap-2 pt-2 border-t border-border/40">
                  {#each requiredDocuments as doc}
                    <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-background text-xs text-foreground select-none">
                      <span class="font-medium">{doc}</span>
                      <button 
                        type="button" 
                        class="text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                        onclick={() => removeDocument(doc)}
                      >
                        <Trash2 size={12} />
                      </button>
                    </div>
                  {/each}
                  {#if requiredDocuments.length === 0}
                    <p class="text-xs text-muted-foreground py-1">
                      {m.settings_registration_no_docs()}
                    </p>
                  {/if}
                </div>
              </div>
            </div>
          </TabsContent>

          <!-- TAB 3: Registration Fields Table -->
          <TabsContent value="fields" class="mt-0">
            <div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
              <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                {m.settings_registration_custom_fields_title()}
              </div>

              <div class="p-6 overflow-x-auto">
                <Table class="w-full text-xs text-start">
                  <TableHeader>
                    <TableRow class="border-b border-border/60 hover:bg-transparent">
                      <TableHead class="py-2 text-start font-semibold text-muted-foreground">{m.settings_registration_table_field()}</TableHead>
                      <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.settings_registration_table_active()}</TableHead>
                      <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.settings_registration_table_required()}</TableHead>
                      <TableHead class="py-2 text-center font-semibold text-muted-foreground">{m.settings_registration_table_duplicate()}</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {#each Object.keys(activeFields) as fieldKey}
                      <TableRow class="border-b border-border/40 hover:bg-muted/5">
                        <TableCell class="py-3 font-semibold text-foreground text-start">
                          {fieldKey === "national_id" ? m.settings_registration_field_national_id() :
                           fieldKey === "email" ? m.settings_registration_field_email() :
                           fieldKey === "phone" ? m.settings_registration_field_phone() :
                           fieldKey === "birth_date" ? m.settings_registration_field_birth_date() :
                           fieldKey === "sex" ? m.settings_registration_field_sex() :
                           fieldKey === "address" ? m.settings_registration_field_address() :
                           fieldKey === "mother_name" ? m.settings_registration_field_mother_name() :
                           fieldKey === "previous_school" ? m.settings_registration_field_previous_school() :
                           m.settings_registration_field_documents()}
                        </TableCell>
                        <TableCell class="py-3 text-center">
                          <Checkbox 
                            id={`active_${fieldKey}`} 
                            checked={activeFields[fieldKey]} 
                            onCheckedChange={(val) => {
                              activeFields[fieldKey] = !!val;
                              if (!val) {
                                requiredFields[fieldKey] = false;
                                preventDuplicates[fieldKey] = false;
                              }
                            }} 
                            class="mx-auto"
                          />
                        </TableCell>
                        <TableCell class="py-3 text-center">
                          <Checkbox 
                            id={`required_${fieldKey}`} 
                            checked={requiredFields[fieldKey]} 
                            disabled={!activeFields[fieldKey]}
                            onCheckedChange={(val) => requiredFields[fieldKey] = !!val} 
                            class="mx-auto"
                          />
                        </TableCell>
                        <TableCell class="py-3 text-center">
                          {#if ["national_id", "email", "phone"].includes(fieldKey)}
                            <Checkbox 
                              id={`duplicate_${fieldKey}`} 
                              checked={preventDuplicates[fieldKey]} 
                              disabled={!activeFields[fieldKey]}
                              onCheckedChange={(val) => preventDuplicates[fieldKey] = !!val} 
                              class="mx-auto"
                            />
                          {:else}
                            <span class="text-muted-foreground">-</span>
                          {/if}
                        </TableCell>
                      </TableRow>
                    {/each}
                  </TableBody>
                </Table>
              </div>
            </div>
          </TabsContent>
        </Tabs>

        <!-- Footer Action Bar -->
        <div class="flex items-center justify-end gap-3 mt-6 p-4 border border-border bg-card/10 rounded-md select-none">
          <Button 
            variant="lime" 
            type="submit" 
            class="text-xs h-9 px-4 cursor-pointer font-semibold shrink-0" 
            disabled={isSaving}
          >
            <Save size={14} class="me-1.5" />
            {isSaving ? m.settings_registration_saving() : m.settings_registration_save()}
          </Button>
        </div>

      </form>
    {/if}

  </div>
</div>
