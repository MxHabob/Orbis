<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui-svelte/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import { onMount } from "svelte";
import { Save, HelpCircle, CheckSquare, Plus, Trash2 } from "lucide-svelte";
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

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-start text-foreground">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    
    <div class="flex flex-col gap-1">
      <h1 class="text-xl font-bold tracking-tight text-foreground">
        {m.settings_registration_title()}
      </h1>
      <p class="text-xs text-muted-foreground">
        {m.settings_registration_desc()}
      </p>
    </div>

    {#if isLoading}
      <div class="space-y-6">
        <div class="h-32 bg-card border border-border rounded-xl animate-pulse"></div>
        <div class="h-48 bg-card border border-border rounded-xl animate-pulse"></div>
      </div>
    {:else}
      <form onsubmit={handleSave} class="flex flex-col gap-6">
        
        <!-- Section 1: Institution Details & Numbering -->
        <Card class="bg-card border border-border rounded-xl p-6 space-y-6">
          <div class="border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">
              {m.settings_registration_type_and_numbering()}
            </h2>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="flex flex-col gap-2">
              <Label for="inst_type" class="text-xs font-semibold text-muted-foreground">
                {m.settings_registration_inst_type()}
              </Label>
              <Select bind:value={institutionType} type="single">
                <SelectTrigger id="inst_type" class="h-9 text-xs flex justify-between items-center border border-input">
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

            <div class="flex flex-col gap-2">
              <Label for="num_pattern" class="text-xs font-semibold text-muted-foreground">
                {m.settings_registration_num_pattern()}
              </Label>
              <Input id="num_pattern" bind:value={studentNumberPattern} class="h-9 text-xs border border-input" />
              <p class="text-[10px] text-muted-foreground leading-normal flex items-start gap-1">
                <HelpCircle size={12} class="mt-0.5 shrink-0 text-primary" />
                <span>
                  {m.settings_registration_pattern_desc()}
                </span>
              </p>
            </div>
          </div>
        </Card>

        <!-- Section 2: Academic Stages -->
        <Card class="bg-card border border-border rounded-xl p-6 space-y-4">
          <div class="border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">
              {m.settings_registration_stages_title()}
            </h2>
          </div>

          <div class="flex flex-col gap-4">
            <div class="flex gap-2">
              <Input 
                placeholder={m.settings_registration_add_stage_placeholder()} 
                bind:value={stageInput} 
                class="h-9 text-xs flex-1 border border-input"
                onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addStage())}
              />
              <Button type="button" variant="outline" size="sm" onclick={addStage}>
                <Plus size={14} class="me-1.5" />
                {getLocale() === "ar" ? "إضافة" : "Add"}
              </Button>
            </div>

            <div class="flex flex-wrap gap-2 pt-2">
              {#each rawStages as stage}
                <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-muted/45 text-xs text-foreground">
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
                <p class="text-xs text-muted-foreground">
                  {m.settings_registration_no_stages()}
                </p>
              {/if}
            </div>
          </div>
        </Card>

        <!-- Section: Required Documents Checklist -->
        <Card class="bg-card border border-border rounded-xl p-6 space-y-4">
          <div class="border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">
              {m.settings_registration_docs_title()}
            </h2>
          </div>

          <div class="flex flex-col gap-4">
            <div class="flex gap-2">
              <Input 
                placeholder={m.settings_registration_add_doc_placeholder()} 
                bind:value={documentInput} 
                class="h-9 text-xs flex-1 border border-input"
                onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), addDocument())}
              />
              <Button type="button" variant="outline" size="sm" onclick={addDocument}>
                <Plus size={14} class="me-1.5" />
                إضافة
              </Button>
            </div>

            <div class="flex flex-wrap gap-2 pt-2">
              {#each requiredDocuments as doc}
                <div class="flex items-center gap-1.5 px-3 py-1 border border-border rounded-full bg-muted/45 text-xs text-foreground">
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
                <p class="text-xs text-muted-foreground">
                  {m.settings_registration_no_docs()}
                </p>
              {/if}
            </div>
          </div>
        </Card>

        <!-- Section 3: Dynamic Field Visibility & Requirements -->
        <Card class="bg-card border border-border rounded-xl p-6 space-y-4">
          <div class="border-b border-border/60 pb-4">
            <h2 class="text-sm font-semibold text-foreground">
              {m.settings_registration_custom_fields_title()}
            </h2>
          </div>

          <div class="overflow-x-auto">
            <table class="w-full text-xs text-start">
              <thead>
                <tr class="border-b border-border/60 text-muted-foreground">
                  <th class="py-2 text-start font-semibold">{m.settings_registration_table_field()}</th>
                  <th class="py-2 text-center font-semibold">{m.settings_registration_table_active()}</th>
                  <th class="py-2 text-center font-semibold">{m.settings_registration_table_required()}</th>
                  <th class="py-2 text-center font-semibold">{m.settings_registration_table_duplicate()}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/40">
                {#each Object.keys(activeFields) as fieldKey}
                  <tr>
                    <td class="py-3 font-semibold text-foreground text-start">
                      {fieldKey === "national_id" ? m.settings_registration_field_national_id() :
                       fieldKey === "email" ? m.settings_registration_field_email() :
                       fieldKey === "phone" ? m.settings_registration_field_phone() :
                       fieldKey === "birth_date" ? m.settings_registration_field_birth_date() :
                       fieldKey === "sex" ? m.settings_registration_field_sex() :
                       fieldKey === "address" ? m.settings_registration_field_address() :
                       fieldKey === "mother_name" ? m.settings_registration_field_mother_name() :
                       fieldKey === "previous_school" ? m.settings_registration_field_previous_school() :
                       m.settings_registration_field_documents()}
                    </td>
                    <td class="py-3 text-center">
                      <Checkbox 
                        id={`active_${fieldKey}`} 
                        checked={activeFields[fieldKey]} 
                        onCheckedChange={(val) => activeFields[fieldKey] = !!val} 
                      />
                    </td>
                    <td class="py-3 text-center">
                      <Checkbox 
                        id={`required_${fieldKey}`} 
                        checked={requiredFields[fieldKey]} 
                        disabled={!activeFields[fieldKey]}
                        onCheckedChange={(val) => requiredFields[fieldKey] = !!val} 
                      />
                    </td>
                    <td class="py-3 text-center">
                      {#if ["national_id", "email", "phone"].includes(fieldKey)}
                        <Checkbox 
                          id={`duplicate_${fieldKey}`} 
                          checked={preventDuplicates[fieldKey]} 
                          disabled={!activeFields[fieldKey]}
                          onCheckedChange={(val) => preventDuplicates[fieldKey] = !!val} 
                        />
                      {:else}
                        <span class="text-muted-foreground">-</span>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card>

        <!-- Save Button -->
        <div class="flex justify-end gap-3 pb-8">
          <Button variant="lime" type="submit" disabled={isSaving}>
            <Save size={14} class="me-1.5" />
            {isSaving ? m.settings_registration_saving() : m.settings_registration_save()}
          </Button>
        </div>
        
      </form>
    {/if}

  </div>
</div>
