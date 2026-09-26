<!--
  @component DocumentsSection
  Dynamic document requirements checklist and verification status.
  Evaluates mandatory requirements and blocks enrollment indicators.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Checkbox } from "@orbis/ui/checkbox";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui/card";
import { CheckCircle, DocumentCheck, Lock } from "@orbis/icons";
import { l } from "$lib/core/i18n";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();

const docItems = $derived.by(() => {
  if (state.document.fulfillments.length > 0) {
    return state.document.fulfillments.map((f) => ({
      id: f.requirement.code,
      label: f.requirement.name_ar || f.requirement.name_en || f.requirement.code,
      isMandatory: f.requirement.is_mandatory,
      blocksEnrollment: f.requirement.blocks_enrollment,
      isFulfilled: f.is_fulfilled,
      condition: f.requirement.condition,
      fileName: f.document_name,
    }));
  }

  if (state.document.requirements.length > 0) {
    return state.document.requirements.map((r) => ({
      id: r.code,
      label: r.name_ar || r.name_en || r.code,
      isMandatory: r.is_mandatory,
      blocksEnrollment: r.blocks_enrollment,
      isFulfilled: state.document.submittedDocs.includes(r.code),
      condition: r.condition,
      fileName: undefined,
    }));
  }

  if (state.document.orgRequiredDocuments.length > 0) {
    return state.document.orgRequiredDocuments.map((name) => ({
      id: name,
      label: name,
      isMandatory: true,
      blocksEnrollment: false,
      isFulfilled: state.document.submittedDocs.includes(name),
      condition: "all_students",
      fileName: undefined,
    }));
  }

  return [
    { id: "birth_certificate", label: m.common_form_doc_birth_certificate(), isMandatory: true, blocksEnrollment: true, isFulfilled: state.document.submittedDocs.includes("birth_certificate"), condition: "all_students", fileName: undefined },
    { id: "national_id", label: m.common_form_doc_national_id(), isMandatory: true, blocksEnrollment: true, isFulfilled: state.document.submittedDocs.includes("national_id"), condition: "all_students", fileName: undefined },
    { id: "previous_record", label: m.common_form_doc_previous_record(), isMandatory: false, blocksEnrollment: false, isFulfilled: state.document.submittedDocs.includes("previous_record"), condition: "all_students", fileName: undefined },
  ];
});
</script>

<Card class="w-full">
  <CardHeader class="py-3 px-6 flex flex-row items-center justify-between border-b border-border/40">
    <div class="flex items-center gap-2">
      <DocumentCheck class="h-4 w-4 text-primary" />
      <CardTitle class="text-xs font-bold text-foreground uppercase tracking-wider text-start select-none">
        {m.common_form_documents_title()}
      </CardTitle>
    </div>
    {#if state.document.fulfillments.length > 0}
      {@const fulfilledCount = state.document.fulfillments.filter(f => f.is_fulfilled).length}
      <Badge variant="outline" class="text-[10px] font-mono">
        {fulfilledCount} / {state.document.fulfillments.length} {l("مكتمل", "fulfilled")}
      </Badge>
    {/if}
  </CardHeader>

  <CardContent class="p-6">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each docItems as doc (doc.id)}
        {@const isChecked = doc.isFulfilled || state.document.submittedDocs.includes(doc.id)}
        <div 
          class="flex flex-col p-3.5 border rounded-lg bg-background hover:bg-accent/5 transition-all duration-150 gap-2 text-start select-none {isChecked ? 'border-primary/40 bg-primary/5' : 'border-border'}"
        >
          <div class="flex items-start justify-between gap-2">
            <div class="flex items-center gap-3">
              <Checkbox 
                id={`doc_${doc.id}`} 
                checked={isChecked}
                onCheckedChange={() => state.document.toggleDoc(doc.id)}
              />
              <Label for={`doc_${doc.id}`} class="text-xs font-semibold text-foreground cursor-pointer leading-tight">
                {doc.label}
              </Label>
            </div>
            {#if isChecked}
              <CheckCircle class="h-4 w-4 text-emerald-500 shrink-0" />
            {/if}
          </div>

          <div class="flex items-center gap-1.5 flex-wrap ms-7">
            {#if doc.isMandatory}
              <Badge variant="destructive" class="text-[9px] px-1.5 py-0 h-4 font-bold">
                {l("إلزامي", "Mandatory")}
              </Badge>
            {:else}
              <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4 font-medium">
                {l("اختياري", "Optional")}
              </Badge>
            {/if}

            {#if doc.blocksEnrollment}
              <Badge variant="outline" class="text-[9px] px-1.5 py-0 h-4 text-amber-500 border-amber-500/30 flex items-center gap-0.5 font-semibold">
                <Lock class="h-2.5 w-2.5" />
                {l("يحجب القيد", "Blocks")}
              </Badge>
            {/if}

            {#if doc.fileName}
              <span class="text-[10px] text-muted-foreground truncate max-w-35 font-mono">
                {doc.fileName}
              </span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </CardContent>
</Card>
