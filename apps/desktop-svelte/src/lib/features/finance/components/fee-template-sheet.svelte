<!--
  @component FeeTemplateSheet
  Side panel for creating/editing a fee template.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Receipt } from "lucide-svelte";
import FeeTemplateForm from "../forms/fee-template-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  template,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  template?: any;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md">
    <SheetHeader class="text-start select-none">
      <SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
        <Receipt size={18} class="text-primary" />
        <span>
          {template 
            ? (m.ft_sheet_edit_title ? m.ft_sheet_edit_title() : "تعديل قالب الرسوم") 
            : (m.ft_sheet_create_title ? m.ft_sheet_create_title() : "إنشاء قالب رسوم")}
        </span>
      </SheetTitle>
      <SheetDescription class="text-sm text-muted-foreground mt-1">
        {m.ft_desc ? m.ft_desc() : "إدارة تفاصيل قالب الرسوم وحساباته الدفترية"}
      </SheetDescription>
    </SheetHeader>

    <FeeTemplateForm 
      {orgId} 
      {template}
      onSuccess={() => {
        open = false;
        onSuccess();
      }} 
      onCancel={() => open = false} 
    />
  </SheetContent>
</Sheet>
