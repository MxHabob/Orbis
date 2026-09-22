<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * VariablesPanel — Browse, search, and insert data variables into elements.
 * Supports drag-to-bind and click-to-insert {{variable}} syntax.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import type { VariableDefinition } from "@docforge/core";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import { ScrollArea } from "@orbis/ui/scroll-area";
import { AdjustmentsHorizontal, Bold, Calendar, ChevronDown, ChevronRight, Clipboard, Code, Hashtag, Knobs, Photo, Plus, Search, Trash } from "@orbis/icons";

let {
  store,
}: {
  store: EditorStore;
} = $props();

let searchQuery = $state("");
let showAddForm = $state(false);
let newVarName = $state("");
let newVarSource = $state("");
let newVarType = $state<string>("string");
let expandedGroups = $state<Set<string>>(new Set(["all"]));

// ── Derived ──────────────────────────────────────────────────

function flattenSchema(
  schema: any,
  prefix = "",
): Record<string, VariableDefinition & { isSystem?: boolean }> {
  let result: Record<string, VariableDefinition & { isSystem?: boolean }> = {};
  for (const [key, value] of Object.entries(schema)) {
    const fullPath = prefix ? `${prefix}.${key}` : key;
    if (
      value &&
      typeof value === "object" &&
      (value as any).type === "object" &&
      (value as any).properties
    ) {
      result = {
        ...result,
        ...flattenSchema((value as any).properties, fullPath),
      };
    } else if (
      value &&
      typeof value === "object" &&
      (value as any).type === "array" &&
      (value as any).items?.type === "object"
    ) {
      result[fullPath] = {
        source: fullPath,
        type: "array",
        label: (value as any).label || fullPath,
        labelAr: (value as any).label || fullPath,
        isSystem: true,
      };
    } else {
      result[fullPath] = {
        source: fullPath,
        type: (value as any)?.type || "string",
        label: (value as any)?.label || fullPath,
        labelAr: (value as any)?.label || fullPath,
        isSystem: true,
      };
    }
  }
  return result;
}

const entityType = $derived(store.document.meta?.entityType || "");
const systemSchema = $derived(store.systemSchemas?.[entityType] || {});
const systemVariables = $derived(flattenSchema(systemSchema));

const variables = $derived({ ...systemVariables, ...store.document.variables });

const variableEntries = $derived(
  Object.entries(variables)
    .filter(([key, v]) => {
      if (!searchQuery) return true;
      const q = searchQuery.toLowerCase();
      return (
        key.toLowerCase().includes(q) ||
        v.source.toLowerCase().includes(q) ||
        (v.label?.toLowerCase().includes(q) ?? false) ||
        (v.labelAr?.toLowerCase().includes(q) ?? false)
      );
    })
    .sort((a, b) => a[0].localeCompare(b[0])),
);

// Group variables by source prefix (e.g., "student", "enrollment")
const groupedVariables = $derived(() => {
  const groups: Record<string, [string, VariableDefinition][]> = {};
  for (const entry of variableEntries) {
    const source = entry[1].source;
    const groupKey = source.includes(".") ? source.split(".")[0]! : "general";
    if (!groups[groupKey]) groups[groupKey] = [];
    groups[groupKey]!.push(entry);
  }
  return groups;
});

// ── Actions ──────────────────────────────────────────────────

function insertVariable(varName: string) {
  const selectedNode = store.selectedNode;
  if (selectedNode && selectedNode.type === "Text") {
    const currentContent = (selectedNode as any).props?.content || "";
    store.updateSelectedProps({
      props: {
        ...(selectedNode as any).props,
        content: currentContent + `{{${varName}}}`,
      },
    } as any);
  }
}

function copyVariableTag(varName: string) {
  navigator.clipboard.writeText(`{{${varName}}}`);
}

function addVariable() {
  if (!newVarName.trim() || !newVarSource.trim()) return;
  const previousVariables = { ...store.document.variables };
  const updatedVars = {
    ...previousVariables,
    [newVarName.trim()]: {
      source: newVarSource.trim(),
      type: newVarType as any,
      label: newVarName.trim(),
    },
  };
  store.executeCommand({
    type: "update-variables",
    description: `Add variable ${newVarName}`,
    descriptionAr: `إضافة متغير ${newVarName}`,
    execute: (doc) => ({ ...doc, variables: updatedVars }),
    undo: (doc) => ({ ...doc, variables: previousVariables }),
  });
  newVarName = "";
  newVarSource = "";
  newVarType = "string";
  showAddForm = false;
}

function removeVariable(varName: string) {
  const prevVars = { ...store.document.variables };
  const { [varName]: _, ...remaining } = prevVars;
  store.executeCommand({
    type: "remove-variable",
    description: `Remove variable ${varName}`,
    descriptionAr: `حذف متغير ${varName}`,
    execute: (doc) => ({ ...doc, variables: remaining }),
    undo: (doc) => ({ ...doc, variables: prevVars }),
  });
}

function toggleGroup(group: string) {
  const next = new Set(expandedGroups);
  if (next.has(group)) next.delete(group);
  else next.add(group);
  expandedGroups = next;
}

function getTypeIcon(type: string) {
  switch (type) {
    case "string":
      return TypeIcon;
    case "number":
      return Hashtag;
    case "date":
      return Calendar;
    case "boolean":
      return ToggleLeft;
    case "image":
      return ImageIcon;
    case "array":
      return List;
    default:
      return Variable;
  }
}

function getTypeBadgeVariant(type: string): any {
  switch (type) {
    case "string":
      return "secondary";
    case "number":
      return "brand";
    case "date":
      return "success";
    case "boolean":
      return "outline";
    case "image":
      return "ghost";
    case "array":
      return "destructive";
    default:
      return "secondary";
  }
}

// ── Drag Data ────────────────────────────────────────────────

function handleDragStart(e: DragEvent, varName: string) {
  e.dataTransfer?.setData("text/plain", `{{${varName}}}`);
  e.dataTransfer?.setData("application/x-docforge-variable", varName);
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}
</script>

<TooltipProvider delayDuration={300}>
<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Variable size={13} class="text-primary" />
      <h3 class="text-xs font-bold text-foreground">{m.template_panel_variables()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[8px]">{variableEntries.length}</Badge>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => showAddForm = !showAddForm}>
            <Plus size={11} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_btn_add_var()}</TooltipContent>
      </Tooltip>
    </div>
  </div>

  <!-- Search -->
      <div class="px-4 pb-2 shrink-0">
        <div class="relative flex items-center">
          <Search
            class="absolute inset-s-2.5 size-3.5 text-muted-foreground pointer-events-none"
          />
          <Input
            placeholder={m.common_header_search ? m.common_header_search() : "البحث في المتغيرات..."}
            bind:value={searchQuery}
            class="h-7 text-[10px] ps-7"
          />
        </div>
      </div>

  <!-- Add Variable Form -->
  {#if showAddForm}
    <div class="px-3 py-2 border-b border-border/30 bg-muted/20 flex flex-col gap-1.5">
      <Input
        type="text"
        placeholder=m.template_var_name()
        bind:value={newVarName}
        class="h-6 text-[10px]"
      />
      <Input
        type="text"
        placeholder=m.template_var_source()
        bind:value={newVarSource}
        class="h-6 text-[10px] font-mono"
      />
      <div class="flex items-center gap-1.5">
        <select
          bind:value={newVarType}
          class="h-6 text-[10px] rounded border border-border bg-background px-1.5 grow"
        >
          <option value="string">String</option>
          <option value="number">Number</option>
          <option value="date">Date</option>
          <option value="boolean">Boolean</option>
          <option value="image">Image</option>
          <option value="array">Array</option>
        </select>
        <Button variant="lime" size="xs" class="cursor-pointer h-6 text-[10px]" onclick={addVariable}>
          {m.template_btn_add()}
        </Button>
      </div>
    </div>
  {/if}

  <!-- Variable List -->
  <div class="grow overflow-y-auto scrollbar-hide">
    {#if variableEntries.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <Variable size={20} class="text-muted-foreground/30" />
        <p class="text-[10px] text-muted-foreground text-center">
          {searchQuery ? m.template_msg_no_results() : m.template_msg_no_vars()}
        </p>
      </div>
    {:else}
      <div class="py-1">
        {#each variableEntries as [varName, varDef] (varName)}
          {@const TypeIconComp = getTypeIcon(varDef.type)}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="flex items-center gap-1.5 px-2 py-1.5 mx-1 rounded hover:bg-accent/50 cursor-grab group transition-all"
            draggable="true"
            ondragstart={(e) => handleDragStart(e, varName)}
          >
            <!-- Drag handle -->
            <GripVertical size={9} class="text-muted-foreground/30 group-hover:text-muted-foreground shrink-0" />

            <!-- Type icon -->
            <TypeIconComp size={11} class="shrink-0 text-muted-foreground" />

            <!-- Variable info -->
            <div class="flex flex-col grow min-w-0">
              <div class="flex items-center gap-1">
                <span class="text-[10px] font-medium text-foreground truncate">{varDef.labelAr || varDef.label || varName}</span>
                <Badge variant={getTypeBadgeVariant(varDef.type)} class="text-[7px] h-3 px-1">{varDef.type}</Badge>
              </div>
              <span class="text-[8px] text-muted-foreground font-mono truncate">
                {`{{${varName}}}`}
              </span>
            </div>

            <!-- Actions (on hover) -->
            <div class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
              <Tooltip>
                <TooltipTrigger>
                  <button type="button" class="p-0.5 rounded hover:bg-accent cursor-pointer" onclick={() => insertVariable(varName)}>
                    <Plus size={10} class="text-primary" />
                  </button>
                </TooltipTrigger>
                <TooltipContent>{m.template_tooltip_insert_var()}</TooltipContent>
              </Tooltip>

              <Tooltip>
                <TooltipTrigger>
                  <button type="button" class="p-0.5 rounded hover:bg-accent cursor-pointer" onclick={() => copyVariableTag(varName)}>
                    <Clipboard size={10} class="text-muted-foreground" />
                  </button>
                </TooltipTrigger>
                <TooltipContent>{m.template_tooltip_copy_tag()}</TooltipContent>
              </Tooltip>

              {#if !(varDef as any).isSystem}
                <Tooltip>
                  <TooltipTrigger>
                    <button type="button" class="p-0.5 rounded hover:bg-accent cursor-pointer" onclick={() => removeVariable(varName)}>
                      <Trash size={10} class="text-destructive" />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent>{m.template_tooltip_delete()}</TooltipContent>
                </Tooltip>
              {:else}
                <Badge variant="outline" class="text-[8px] px-1 h-3.5 font-normal text-muted-foreground bg-muted/30">نظام</Badge>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer hint -->
  <div class="px-3 py-1.5 border-t border-border/30 bg-muted/10">
    <p class="text-[8px] text-muted-foreground text-center">
      {m.template_msg_var_drag()}
    </p>
  </div>
</div>
</TooltipProvider>
