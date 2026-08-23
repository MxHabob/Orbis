<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * InspectorPanel — Dynamic property editor for the selected node.
 * Shows different fields based on the selected node's type.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Textarea } from "@orbis/ui-svelte/textarea";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui-svelte/accordion";
import { Separator } from "@orbis/ui-svelte/separator";
import { Badge } from "@orbis/ui-svelte/badge";
import { Sliders } from "@lucide/svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

const node = $derived(store.selectedNode);
const nodeType = $derived(node?.type ?? null);
const nodeProps = $derived(
  node && "props" in node ? (node as any).props || {} : {},
);
const position = $derived(
  node && "position" in node ? (node as any).position || {} : {},
);

function updateProp(key: string, value: unknown) {
  store.updateSelectedProps({ props: { ...nodeProps, [key]: value } } as any);
}

function updatePosition(key: string, value: number) {
  store.updateSelectedProps({ position: { ...position, [key]: value } } as any);
}
</script>

<div class="flex flex-col h-full overflow-y-auto scrollbar-hide text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center gap-2">
    <Sliders size={13} class="text-primary shrink-0" />
    <h3 class="text-xs font-bold text-foreground">{m.template_title_properties()}</h3>
  </div>

  {#if !node}
    <!-- No selection -->
    <div class="flex-grow flex items-center justify-center p-6">
      <p class="text-[10px] text-muted-foreground text-center">
        {m.template_msg_select_element()}
      </p>
    </div>
  {:else}
    <div class="p-3 flex flex-col gap-3">
      <!-- Node Identity -->
      <div class="flex items-center justify-between">
        <Badge variant="secondary" class="text-[9px]">{nodeType}</Badge>
        <span class="text-[9px] text-muted-foreground font-mono truncate max-w-[120px]">{node.id}</span>
      </div>

      <!-- Name -->
      <div class="flex flex-col gap-1">
        <Label class="text-[10px] text-muted-foreground">{m.template_prop_name()}</Label>
        <Input
          type="text"
          value={node.name || ""}
          oninput={(e) => store.updateSelectedProps({ name: e.currentTarget.value })}
          class="h-7 text-xs"
          placeholder="Element name..."
        />
      </div>

      <Separator />

      <Accordion type="multiple" value={["position", "appearance", "content"]}>
        <!-- Position & Size -->
        <AccordionItem value="position">
          <AccordionTrigger class="text-[10px] font-semibold py-1.5">
            {m.template_prop_position()}
          </AccordionTrigger>
          <AccordionContent>
            <div class="grid grid-cols-2 gap-2 pt-1">
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground">X (mm)</span>
                <Input
                  type="number"
                  step="0.5"
                  value={position.x ?? 0}
                  oninput={(e) => updatePosition("x", parseFloat(e.currentTarget.value) || 0)}
                  class="h-6 text-[10px]"
                />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground">Y (mm)</span>
                <Input
                  type="number"
                  step="0.5"
                  value={position.y ?? 0}
                  oninput={(e) => updatePosition("y", parseFloat(e.currentTarget.value) || 0)}
                  class="h-6 text-[10px]"
                />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground">W (mm)</span>
                <Input
                  type="number"
                  step="0.5"
                  value={typeof position.width === "number" ? position.width : ""}
                  placeholder={typeof position.width === "string" ? position.width : "auto"}
                  oninput={(e) => {
                    const v = parseFloat(e.currentTarget.value);
                    if (!isNaN(v)) updatePosition("width", v);
                  }}
                  class="h-6 text-[10px]"
                />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="text-[8px] text-muted-foreground">H (mm)</span>
                <Input
                  type="number"
                  step="0.5"
                  value={typeof position.height === "number" ? position.height : ""}
                  placeholder={typeof position.height === "string" ? position.height : "auto"}
                  oninput={(e) => {
                    const v = parseFloat(e.currentTarget.value);
                    if (!isNaN(v)) updatePosition("height", v);
                  }}
                  class="h-6 text-[10px]"
                />
              </div>
            </div>

            {#if node.rotation !== undefined || nodeType === "Shape" || nodeType === "Image"}
              <div class="flex flex-col gap-0.5 mt-2">
                <span class="text-[8px] text-muted-foreground">Rotation (°)</span>
                <Input
                  type="number"
                  step="1"
                  value={node.rotation ?? 0}
                  oninput={(e) => store.updateSelectedProps({ rotation: parseFloat(e.currentTarget.value) || 0 })}
                  class="h-6 text-[10px]"
                />
              </div>
            {/if}
          </AccordionContent>
        </AccordionItem>

        <!-- Appearance -->
        <AccordionItem value="appearance">
          <AccordionTrigger class="text-[10px] font-semibold py-1.5">
            {m.template_prop_appearance()}
          </AccordionTrigger>
          <AccordionContent>
            <div class="flex flex-col gap-2 pt-1">
              <div class="flex items-center justify-between">
                <span class="text-[9px] text-muted-foreground">{m.template_prop_opacity()}</span>
                <Input
                  type="number"
                  step="0.1"
                  min="0"
                  max="1"
                  value={node.opacity ?? 1}
                  oninput={(e) => store.updateSelectedProps({ opacity: parseFloat(e.currentTarget.value) || 1 })}
                  class="h-6 text-[10px] w-16"
                />
              </div>

              <div class="flex items-center justify-between">
                <span class="text-[9px] text-muted-foreground">{m.template_prop_visible()}</span>
                <Switch
                  checked={node.visible !== false}
                  onCheckedChange={(v) => store.updateSelectedProps({ visible: v })}
                />
              </div>

              <div class="flex items-center justify-between">
                <span class="text-[9px] text-muted-foreground">{m.template_prop_locked()}</span>
                <Switch
                  checked={node.locked === true}
                  onCheckedChange={(v) => store.updateSelectedProps({ locked: v })}
                />
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>

        <!-- Content (type-specific) -->
        <AccordionItem value="content">
          <AccordionTrigger class="text-[10px] font-semibold py-1.5">
            {m.template_prop_content()}
          </AccordionTrigger>
          <AccordionContent>
            <div class="flex flex-col gap-2 pt-1">
              {#if nodeType === "Text"}
                <!-- Text Content -->
                <div class="flex flex-col gap-1">
                  <Label class="text-[9px] text-muted-foreground">{m.template_prop_content()}</Label>
                  <Textarea
                    value={nodeProps.content || ""}
                    oninput={(e) => updateProp("content", e.currentTarget.value)}
                    class="text-xs min-h-[50px]"
                  />
                </div>

                <div class="grid grid-cols-2 gap-2">
                  <div class="flex flex-col gap-0.5">
                    <span class="text-[8px] text-muted-foreground">{m.template_prop_font_size()}</span>
                    <Input type="number" step="1" value={nodeProps.fontSize ?? 12} oninput={(e) => updateProp("fontSize", parseFloat(e.currentTarget.value))} class="h-6 text-[10px]" />
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-[8px] text-muted-foreground">{m.template_prop_font_weight()}</span>
                    <Select type="single" value={nodeProps.fontWeight || "400"} onValueChange={(v) => { if (v) updateProp("fontWeight", v); }}>
                      <SelectTrigger class="h-6 text-[10px]">{nodeProps.fontWeight || "400"}</SelectTrigger>
                      <SelectContent>
                        <SelectItem value="300">Light</SelectItem>
                        <SelectItem value="400">Regular</SelectItem>
                        <SelectItem value="600">SemiBold</SelectItem>
                        <SelectItem value="700">Bold</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>

                <div class="grid grid-cols-2 gap-2">
                  <div class="flex flex-col gap-0.5">
                    <span class="text-[8px] text-muted-foreground">{m.template_prop_font_family()}</span>
                    <Select type="single" value={nodeProps.fontFamily || "Rubik"} onValueChange={(v) => { if (v) updateProp("fontFamily", v); }}>
                      <SelectTrigger class="h-6 text-[10px]">{nodeProps.fontFamily || "Rubik"}</SelectTrigger>
                      <SelectContent>
                        <SelectItem value="Rubik">Rubik</SelectItem>
                        <SelectItem value="Tahoma">Tahoma</SelectItem>
                        <SelectItem value="Arial">Arial</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-[8px] text-muted-foreground">{m.template_prop_alignment()}</span>
                    <Select type="single" value={nodeProps.textAlign || "start"} onValueChange={(v) => { if (v) updateProp("textAlign", v); }}>
                      <SelectTrigger class="h-6 text-[10px]">{nodeProps.textAlign || "start"}</SelectTrigger>
                      <SelectContent>
                        <SelectItem value="start">Start</SelectItem>
                        <SelectItem value="center">Center</SelectItem>
                        <SelectItem value="end">End</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>

                <div class="flex flex-col gap-0.5">
                  <span class="text-[8px] text-muted-foreground">{m.template_prop_color()}</span>
                  <div class="flex items-center gap-2">
                    <input type="color" value={nodeProps.color || "#000000"} oninput={(e) => updateProp("color", e.currentTarget.value)} class="w-7 h-6 rounded border border-border cursor-pointer" />
                    <Input type="text" value={nodeProps.color || "#000000"} oninput={(e) => updateProp("color", e.currentTarget.value)} class="h-6 text-[10px] font-mono flex-grow" />
                  </div>
                </div>

              {:else if nodeType === "Shape"}
                <!-- Shape properties -->
                <div class="flex flex-col gap-0.5">
                  <span class="text-[8px] text-muted-foreground">{m.template_prop_shape()}</span>
                  <Select type="single" value={nodeProps.shape || "rectangle"} onValueChange={(v) => { if (v) updateProp("shape", v); }}>
                    <SelectTrigger class="h-6 text-[10px]">{nodeProps.shape || "rectangle"}</SelectTrigger>
                    <SelectContent>
                      <SelectItem value="rectangle">Rectangle</SelectItem>
                      <SelectItem value="circle">Circle</SelectItem>
                      <SelectItem value="ellipse">Ellipse</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

                <div class="flex flex-col gap-0.5">
                  <span class="text-[8px] text-muted-foreground">{m.template_prop_fill_color()}</span>
                  <div class="flex items-center gap-2">
                    <input type="color" value={nodeProps.fillColor || "#e2e8f0"} oninput={(e) => updateProp("fillColor", e.currentTarget.value)} class="w-7 h-6 rounded border border-border cursor-pointer" />
                    <Input type="text" value={nodeProps.fillColor || "#e2e8f0"} oninput={(e) => updateProp("fillColor", e.currentTarget.value)} class="h-6 text-[10px] font-mono flex-grow" />
                  </div>
                </div>

                <div class="flex flex-col gap-0.5">
                  <span class="text-[8px] text-muted-foreground">{m.template_prop_border_color()}</span>
                  <div class="flex items-center gap-2">
                    <input type="color" value={nodeProps.strokeColor || "#94a3b8"} oninput={(e) => updateProp("strokeColor", e.currentTarget.value)} class="w-7 h-6 rounded border border-border cursor-pointer" />
                    <Input type="text" value={nodeProps.strokeColor || "#94a3b8"} oninput={(e) => updateProp("strokeColor", e.currentTarget.value)} class="h-6 text-[10px] font-mono flex-grow" />
                  </div>
                </div>

              {:else if nodeType === "Barcode"}
                <div class="flex flex-col gap-1">
                  <Label class="text-[9px] text-muted-foreground">{m.template_prop_data()}</Label>
                  <Input type="text" value={nodeProps.data || ""} oninput={(e) => updateProp("data", e.currentTarget.value)} class="h-7 text-xs font-mono" />
                </div>
                <div class="flex flex-col gap-0.5">
                  <span class="text-[8px] text-muted-foreground">{m.template_prop_format()}</span>
                  <Select type="single" value={nodeProps.format || "Code128"} onValueChange={(v) => { if (v) updateProp("format", v); }}>
                    <SelectTrigger class="h-6 text-[10px]">{nodeProps.format || "Code128"}</SelectTrigger>
                    <SelectContent>
                      <SelectItem value="Code128">Code128</SelectItem>
                      <SelectItem value="EAN8">EAN8</SelectItem>
                      <SelectItem value="EAN13">EAN13</SelectItem>
                      <SelectItem value="Code39">Code39</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

              {:else if nodeType === "QRCode"}
                <div class="flex flex-col gap-1">
                  <Label class="text-[9px] text-muted-foreground">{m.template_prop_data()}</Label>
                  <Input type="text" value={nodeProps.data || ""} oninput={(e) => updateProp("data", e.currentTarget.value)} class="h-7 text-xs font-mono" />
                </div>

              {:else}
                <p class="text-[9px] text-muted-foreground">
                  {m.template_msg_no_content_props()}
                </p>
              {/if}
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  {/if}
</div>
