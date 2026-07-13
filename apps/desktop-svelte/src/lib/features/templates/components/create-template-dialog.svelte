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
        أدخل تفاصيل قالب المستندات الجديد لبدء تصميمه في المحرر.
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-2">
        <Label for="name-en" class="text-xs font-semibold text-muted-foreground">اسم القالب بالإنجليزية (Name EN) *</Label>
        <Input id="name-en" bind:value={name} placeholder="e.g. Student Certificate" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="name-ar" class="text-xs font-semibold text-muted-foreground">اسم القالب بالعربية (Name AR)</Label>
        <Input id="name-ar" bind:value={nameAr} placeholder="مثال: شهادة طالب" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="desc" class="text-xs font-semibold text-muted-foreground">وصف القالب / Description</Label>
        <Input id="desc" bind:value={desc} placeholder="وصف موجز للقالب..." />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">نوع السجل / Entity</Label>
          <Select type="single" bind:value={entityType}>
            <SelectTrigger>
              {({ enrollment: "تسجيل الطلاب", payment: "سندات القبض", exam: "أرقام الجلوس", user: "بطاقات التعريف", organization: "المؤسسة / المدرسة" } as any)[entityType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="enrollment">تسجيل الطلاب</SelectItem>
              <SelectItem value="payment">سندات القبض</SelectItem>
              <SelectItem value="exam">أرقام الجلوس</SelectItem>
              <SelectItem value="user">بطاقات التعريف</SelectItem>
              <SelectItem value="organization">المؤسسة / المدرسة</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">نوع المستند / Document</Label>
          <Select type="single" bind:value={docType}>
            <SelectTrigger>
              {({ CERTIFICATE: "شهادات (Certificate)", ID_CARD: "بطاقات (ID Card)", RECEIPT: "سندات (Receipt)", EXAM_CARD: "بطاقات جلوس (Seating)", REPORT: "تقارير (Report)" } as any)[docType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="CERTIFICATE">شهادات (Certificate)</SelectItem>
              <SelectItem value="ID_CARD">بطاقات (ID Card)</SelectItem>
              <SelectItem value="RECEIPT">سندات (Receipt)</SelectItem>
              <SelectItem value="EXAM_CARD">بطاقات جلوس (Seating)</SelectItem>
              <SelectItem value="REPORT">تقارير (Report)</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">حجم الصفحة / Size</Label>
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
          <Label class="text-xs font-semibold text-muted-foreground">الاتجاه / Orientation</Label>
          <Select type="single" bind:value={orientation}>
            <SelectTrigger>
              {({ portrait: "طولي (Portrait)", landscape: "عرضي (Landscape)" } as any)[orientation]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="portrait">طولي (Portrait)</SelectItem>
              <SelectItem value="landscape">عرضي (Landscape)</SelectItem>
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
