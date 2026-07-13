<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Label } from "@orbis/ui-svelte/label";
import * as m from "$lib/paraglide/messages";
import type { StudentFormState } from "../student-form-state.svelte";

let { state }: { state: StudentFormState } = $props();

const docTypes = $derived.by(() => {
  if (state.document.orgRequiredDocuments.length > 0) {
    return state.document.orgRequiredDocuments.map((name) => ({
      id: name,
      label: name,
    }));
  }
  // Fallback
  return [
    { id: "birth_certificate", label: m.form_doc_birth_certificate() },
    { id: "national_id", label: m.form_doc_national_id() },
    {
      id: "previous_record",
      label: m.form_doc_previous_record(),
    },
  ];
});
</script>

<div class="border border-border rounded-md overflow-hidden flex flex-col bg-card/10">
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.form_documents_title()}
  </div>

  <div class="p-6">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each docTypes as doc (doc.id)}
        {@const isChecked = state.document.submittedDocs.includes(doc.id)}
        <div 
          class="flex items-center justify-between p-3.5 border border-border rounded-lg bg-background hover:bg-accent/10 transition-all duration-150 gap-4 text-start select-none"
        >
          <div class="flex items-center gap-3">
            <Checkbox 
              id={`doc_${doc.id}`} 
              checked={isChecked}
              onCheckedChange={() => state.document.toggleDoc(doc.id)}
            />
            <Label for={`doc_${doc.id}`} class="text-xs font-semibold text-foreground cursor-pointer">
              {doc.label}
            </Label>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
