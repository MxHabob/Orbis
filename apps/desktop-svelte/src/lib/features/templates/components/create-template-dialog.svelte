<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

interface Props {
  open: boolean;
  name: string;
  nameAr: string;
  desc: string;
  entityType: string;
  docType: string;
  pageSize: string;
  orientation: string;
  onsubmit: () => void;
}

let {
  open = $bindable(),
  name = $bindable(),
  nameAr = $bindable(),
  desc = $bindable(),
  entityType = $bindable(),
  docType = $bindable(),
  pageSize = $bindable(),
  orientation = $bindable(),
  onsubmit,
}: Props = $props();
</script>

<Dialog bind:open={open}>
  <DialogContent class="sm:max-w-[480px] text-start">
    <DialogHeader>
      <DialogTitle>{m.settings_templates_create_btn()}</DialogTitle>
      <DialogDescription>
        {m.template_desc_new_template()}
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-2">
        <Label for="name-en" class="text-xs font-semibold text-muted-foreground">{m.template_label_name_en()}</Label>
        <Input id="name-en" bind:value={name} placeholder="e.g. Student Certificate" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="name-ar" class="text-xs font-semibold text-muted-foreground">{m.template_label_name_ar()}</Label>
        <Input id=m.template_placeholder_name_ar() />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="desc" class="text-xs font-semibold text-muted-foreground">{m.template_label_desc()}</Label>
        <Input id=m.template_placeholder_desc() />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_entity()}</Label>
          <Select type="single" bind:value={entityType}>
            <SelectTrigger>
              {({ enrollment: m.template_entity_student_reg(), payment: m.template_entity_receipts(), exam: m.template_entity_seat_numbers(), user: m.template_entity_id_cards(), organization: m.template_entity_school() } as any)[entityType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="enrollment">{m.template_entity_student_reg()}</SelectItem>
              <SelectItem value="payment">{m.template_entity_receipts()}</SelectItem>
              <SelectItem value="exam">{m.template_entity_seat_numbers()}</SelectItem>
              <SelectItem value="user">{m.template_entity_id_cards()}</SelectItem>
              <SelectItem value="organization">{m.template_entity_school()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_doc_type()}</Label>
          <Select type="single" bind:value={docType}>
            <SelectTrigger>
              {({ CERTIFICATE: m.template_doc_certificate(), ID_CARD: m.template_doc_id_card(), RECEIPT: m.template_doc_receipt(), EXAM_CARD: m.template_doc_seating(), REPORT: m.template_doc_report() } as any)[docType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="CERTIFICATE">{m.template_doc_certificate()}</SelectItem>
              <SelectItem value="ID_CARD">{m.template_doc_id_card()}</SelectItem>
              <SelectItem value="RECEIPT">{m.template_doc_receipt()}</SelectItem>
              <SelectItem value="EXAM_CARD">{m.template_doc_seating()}</SelectItem>
              <SelectItem value="REPORT">{m.template_doc_report()}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_page_size()}</Label>
          <Select type="single" bind:value={pageSize}>
            <SelectTrigger>
              {({ A4: "A4 (210 x 297 mm)", A5: "A5 (148 x 210 mm)", Letter: "Letter (8.5 x 11 in)", Custom: "Custom" } as any)[pageSize]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="A4">A4 (210 x 297 mm)</SelectItem>
              <SelectItem value="A5">A5 (148 x 210 mm)</SelectItem>
              <SelectItem value="Letter">Letter (8.5 x 11 in)</SelectItem>
              <SelectItem value="Custom">Custom</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_label_orientation()}</Label>
          <Select type="single" bind:value={orientation}>
            <SelectTrigger>
              {({ portrait: m.template_orientation_portrait(), landscape: m.template_orientation_landscape() } as any)[orientation]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="portrait">{m.template_orientation_portrait()}</SelectItem>
              <SelectItem value="landscape">{m.template_orientation_landscape()}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
    </div>

    <DialogFooter class="gap-2">
      <Button variant="outline" size="sm" onclick={() => (open = false)}>{m.common_action_cancel()}</Button>
      <Button variant="lime" size="sm" onclick={onsubmit}>{m.common_form_btn_create()}</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
