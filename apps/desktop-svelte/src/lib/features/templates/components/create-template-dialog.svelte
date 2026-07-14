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
        {m.template_auto_9c31267d()}
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-2">
        <Label for="name-en" class="text-xs font-semibold text-muted-foreground">{m.template_auto_bee31559()}</Label>
        <Input id="name-en" bind:value={name} placeholder="e.g. Student Certificate" />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="name-ar" class="text-xs font-semibold text-muted-foreground">{m.template_auto_4acc9951()}</Label>
        <Input id=m.template_auto_a5ce9b4b() />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="desc" class="text-xs font-semibold text-muted-foreground">{m.template_auto_ec58ae3a()}</Label>
        <Input id=m.template_auto_8aa93a9a() />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_auto_7f136f51()}</Label>
          <Select type="single" bind:value={entityType}>
            <SelectTrigger>
              {({ enrollment: m.template_auto_699544d2(), payment: m.template_auto_9ec74211(), exam: m.template_auto_963944de(), user: m.template_auto_0b3ac748(), organization: m.template_auto_b84adc1b() } as any)[entityType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="enrollment">{m.template_auto_699544d2()}</SelectItem>
              <SelectItem value="payment">{m.template_auto_9ec74211()}</SelectItem>
              <SelectItem value="exam">{m.template_auto_963944de()}</SelectItem>
              <SelectItem value="user">{m.template_auto_0b3ac748()}</SelectItem>
              <SelectItem value="organization">{m.template_auto_b84adc1b()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_auto_3e079611()}</Label>
          <Select type="single" bind:value={docType}>
            <SelectTrigger>
              {({ CERTIFICATE: m.template_auto_4e5975fa(), ID_CARD: m.template_auto_9faed6d0(), RECEIPT: m.template_auto_b51337de(), EXAM_CARD: m.template_auto_7b0a35ae(), REPORT: m.template_auto_b21a743b() } as any)[docType]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="CERTIFICATE">{m.template_auto_4e5975fa()}</SelectItem>
              <SelectItem value="ID_CARD">{m.template_auto_9faed6d0()}</SelectItem>
              <SelectItem value="RECEIPT">{m.template_auto_b51337de()}</SelectItem>
              <SelectItem value="EXAM_CARD">{m.template_auto_7b0a35ae()}</SelectItem>
              <SelectItem value="REPORT">{m.template_auto_b21a743b()}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_auto_b95e12f8()}</Label>
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
          <Label class="text-xs font-semibold text-muted-foreground">{m.template_auto_18ffda00()}</Label>
          <Select type="single" bind:value={orientation}>
            <SelectTrigger>
              {({ portrait: m.template_auto_0605c426(), landscape: m.template_auto_3476da07() } as any)[orientation]}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="portrait">{m.template_auto_0605c426()}</SelectItem>
              <SelectItem value="landscape">{m.template_auto_3476da07()}</SelectItem>
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
