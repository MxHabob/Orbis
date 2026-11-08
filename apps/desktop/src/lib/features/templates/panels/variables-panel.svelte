<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { EditorStore } from "../stores/editor-store.svelte";
import type { VariableDefinition } from "@docforge/core";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import {
  Bars3BottomLeft,
  Calendar,
  CheckCircle,
  Clipboard,
  Code,
  Hashtag,
  ListBullet,
  Photo,
  Plus,
  Search,
  Trash,
} from "@orbis/icons";

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
  if (!schema || typeof schema !== "object") return result;

  // If the schema is an object with a `properties` map, unwrap its properties directly
  const props =
    schema.properties && typeof schema.properties === "object"
      ? schema.properties
      : schema;

  for (const [key, value] of Object.entries(props)) {
    // Skip JSON Schema structural keywords
    if (
      props === schema &&
      [
        "type",
        "label",
        "properties",
        "required",
        "$schema",
        "description",
      ].includes(key)
    ) {
      continue;
    }

    const fullPath = prefix ? `${prefix}.${key}` : key;
    const rawLabel = (value as any)?.label;
    const resolvedLabel =
      typeof rawLabel === "function" ? rawLabel() : rawLabel || fullPath;
    const resolvedLabelAr = (value as any)?.labelAr || resolvedLabel;

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
      (value as any).type === "array"
    ) {
      result[fullPath] = {
        source: fullPath,
        type: "array",
        label: resolvedLabel,
        labelAr: resolvedLabelAr,
        isSystem: true,
      };
    } else {
      result[fullPath] = {
        source: fullPath,
        type: (value as any)?.type || "string",
        label: resolvedLabel,
        labelAr: resolvedLabelAr,
        isSystem: true,
      };
    }
  }
  return result;
}

const entityType = $derived(store.document.meta?.entityType || "");
const systemVariables = $derived.by(() => {
  let vars: Record<string, VariableDefinition & { isSystem?: boolean }> = {};
  const baseSchemas = store.systemSchemas || {};
  const schemas: Record<string, any> = { ...baseSchemas };

  // Ensure both org and $org namespaces exist
  if (schemas.$org && !schemas.org) {
    schemas.org = schemas.$org;
  }
  if (schemas.org && !schemas.$org) {
    schemas.$org = schemas.org;
  }

  if (entityType && schemas[entityType]) {
    vars = { ...vars, ...flattenSchema(schemas[entityType], entityType) };
  }
  for (const [key, schema] of Object.entries(schemas)) {
    if (key !== entityType) {
      vars = { ...vars, ...flattenSchema(schema, key) };
    }
  }
  return vars;
});

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
    description: m.shared_ui_add_variable({ name: newVarName }),
    descriptionAr: m.shared_ui_add_variable({ name: newVarName }),
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
    description: m.shared_ui_remove_variable({ name: varName }),
    descriptionAr: m.shared_ui_remove_variable({ name: varName }),
    execute: (doc) => ({ ...doc, variables: remaining }),
    undo: (doc) => ({ ...doc, variables: prevVars }),
  });
}

function getTypeIcon(type: string) {
  switch (type) {
    case "string":
      return Bars3BottomLeft;
    case "number":
      return Hashtag;
    case "date":
      return Calendar;
    case "boolean":
      return CheckCircle;
    case "image":
      return Photo;
    case "array":
      return ListBullet;
    default:
      return Code;
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

  function getTypeLabel(type: string): string {
    switch (type) {
      case "string":
        return m.shared_ui_type_string();
      case "number":
        return m.shared_ui_type_number();
      case "date":
        return m.shared_ui_type_date();
      case "boolean":
        return m.shared_ui_type_boolean();
      case "image":
        return m.shared_ui_type_image();
      case "array":
        return m.shared_ui_type_array();
      default:
        return type;
    }
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
  <div class="px-3 py-2.5 border-b border-border flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Code size={15} class="text-muted-foreground" />
      <h3 class="text-sm font-semibold text-foreground">{m.template_panel_variables()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[10px]">{variableEntries.length}</Badge>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" class="cursor-pointer" onclick={() => showAddForm = !showAddForm}>
            <Plus size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_btn_add_var()}</TooltipContent>
      </Tooltip>
    </div>
  </div>

  <!-- Search -->
      <div class="px-3 pb-2 shrink-0">
        <div class="relative flex items-center">
          <Search
            class="absolute inset-s-2.5 size-3.5 text-muted-foreground pointer-events-none"
          />
          <Input
            placeholder={m.common_header_search()}
            bind:value={searchQuery}
            class="h-9 text-xs ps-8"
          />
        </div>
      </div>

  <!-- Add Variable Form -->
  {#if showAddForm}
    <div class="px-3 py-3 border-b border-border bg-muted/30 flex flex-col gap-2">
      <Input
        type="text"
        placeholder=m.template_var_name()
        bind:value={newVarName}
        class="h-9 text-xs"
      />
      <Input
        type="text"
        placeholder=m.template_var_source()
        bind:value={newVarSource}
        class="h-9 text-xs font-mono"
      />
      <div class="flex items-center gap-1.5">
        <Select type="single" bind:value={newVarType}>
          <SelectTrigger class="h-9 min-w-0 grow rounded-md px-2 text-xs">
            {#if newVarType === "number"}
              {m.shared_ui_type_number()}
            {:else if newVarType === "date"}
              {m.shared_ui_type_date()}
            {:else if newVarType === "boolean"}
              {m.shared_ui_type_boolean()}
            {:else if newVarType === "image"}
              {m.shared_ui_type_image()}
            {:else if newVarType === "array"}
              {m.shared_ui_type_array()}
            {:else}
              {m.shared_ui_type_string()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="string">{m.shared_ui_type_string()}</SelectItem>
            <SelectItem value="number">{m.shared_ui_type_number()}</SelectItem>
            <SelectItem value="date">{m.shared_ui_type_date()}</SelectItem>
            <SelectItem value="boolean">{m.shared_ui_type_boolean()}</SelectItem>
            <SelectItem value="image">{m.shared_ui_type_image()}</SelectItem>
            <SelectItem value="array">{m.shared_ui_type_array()}</SelectItem>
          </SelectContent>
        </Select>
        <Button variant="default" size="sm" class="cursor-pointer" onclick={addVariable}>
          {m.template_btn_add()}
        </Button>
      </div>
    </div>
  {/if}

  <!-- Variable List -->
  <div class="grow overflow-y-auto scrollbar-hide">
    {#if variableEntries.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <Code size={20} class="text-muted-foreground/30" />
        <p class="text-xs text-muted-foreground text-center">
          {searchQuery ? m.template_msg_no_results() : m.template_msg_no_vars()}
        </p>
      </div>
    {:else}
      <div class="py-1">
        {#each variableEntries as [varName, varDef] (varName)}
          {@const TypeIconComp = getTypeIcon(varDef.type)}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="flex items-center gap-2 px-2.5 py-2 mx-1 rounded-md hover:bg-muted/60 cursor-grab group transition-colors"
            draggable="true"
            ondragstart={(e) => handleDragStart(e, varName)}
          >
            <!-- Type icon -->
            <TypeIconComp size={14} class="shrink-0 text-muted-foreground" />

            <!-- Variable info -->
            <div class="flex flex-col grow min-w-0">
              <div class="flex items-center gap-1">
                <span class="text-xs font-medium text-foreground truncate">{varDef.labelAr || varDef.label || varName}</span>
                <!-- <Badge variant={getTypeBadgeVariant(varDef.type)} class="text-[7px] h-3 px-1">{getTypeLabel(varDef.type)}</Badge> -->
              </div>
              <span class="text-[10px] text-muted-foreground font-mono truncate">
                {`{{${varName}}}`}
              </span>
            </div>

            <!-- Actions (on hover) -->
            <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
              <Tooltip>
                <TooltipTrigger>
                  <button type="button" class="rounded p-1 hover:bg-accent cursor-pointer" onclick={() => insertVariable(varName)}>
                    <Plus size={13} class="text-primary" />
                  </button>
                </TooltipTrigger>
                <TooltipContent>{m.template_tooltip_insert_var()}</TooltipContent>
              </Tooltip>

              <Tooltip>
                <TooltipTrigger>
                  <button type="button" class="rounded p-1 hover:bg-accent cursor-pointer" onclick={() => copyVariableTag(varName)}>
                    <Clipboard size={13} class="text-muted-foreground" />
                  </button>
                </TooltipTrigger>
                <TooltipContent>{m.template_tooltip_copy_tag()}</TooltipContent>
              </Tooltip>

              {#if !(varDef as any).isSystem}
                <Tooltip>
                  <TooltipTrigger>
                    <button type="button" class="rounded p-1 hover:bg-accent cursor-pointer" onclick={() => removeVariable(varName)}>
                      <Trash size={13} class="text-destructive" />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent>{m.template_tooltip_delete()}</TooltipContent>
                </Tooltip>
              {:else}
                <Badge variant="outline" class="text-[9px] px-1.5 h-5 font-normal text-muted-foreground">{m.shared_ui_system()}</Badge>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer hint -->
  <div class="px-3 py-2 border-t border-border">
    <p class="text-[10px] text-muted-foreground text-center">
      {m.template_msg_var_drag()}
    </p>
  </div>
</div>
</TooltipProvider>
