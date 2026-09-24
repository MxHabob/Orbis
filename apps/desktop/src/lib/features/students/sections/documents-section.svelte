<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Checkbox } from "@orbis/ui/checkbox";
import { Label } from "@orbis/ui/label";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui/card";

import type { StudentFormState } from "../form-state.svelte";

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
    { id: "birth_certificate", label: m.common_form_doc_birth_certificate() },
    { id: "national_id", label: m.common_form_doc_national_id() },
    {
      id: "previous_record",
      label: m.common_form_doc_previous_record(),
    },
  ];
});
</script>

<Card class="w-full">
  <CardHeader class="py-3 px-6">
    <CardTitle class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider text-start select-none">
      {m.common_form_documents_title()}
    </CardTitle>
  </CardHeader>

  <CardContent class="p-6">
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
  </CardContent>
</Card>
