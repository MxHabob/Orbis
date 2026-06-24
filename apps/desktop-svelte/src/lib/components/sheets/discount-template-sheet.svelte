<!--
  @component DiscountTemplateSheet
  Side panel for creating/editing a discount template.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Percent } from "lucide-svelte";
import DiscountTemplateForm from "../forms/discount-template-form.svelte";
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
        <Percent size={18} class="text-primary" />
        <span>
          {template 
            ? (m.dt_sheet_edit_title ? m.dt_sheet_edit_title() : "تعديل قالب الخصم") 
            : (m.dt_sheet_create_title ? m.dt_sheet_create_title() : "إنشاء قالب خصم")}
        </span>
      </SheetTitle>
      <SheetDescription class="text-sm text-muted-foreground mt-1">
        {m.dt_desc ? m.dt_desc() : "إدارة تفاصيل الخصم والمنح وشروط الاستحقاق"}
      </SheetDescription>
    </SheetHeader>

    <DiscountTemplateForm 
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
