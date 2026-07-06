<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@orbis/ui-svelte/card";
import {
  FileText,
  Plus,
  Trash2,
  Edit,
  Archive,
  RefreshCw,
  CheckCircle,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";

interface Props {
  templates: any[];
  loading: boolean;
  orgId: string;
  onconfirmdelete: (id: string) => void;
  onpublish: (id: string) => void;
  onarchive: (id: string) => void;
  onrestore: (id: string) => void;
  oncreatenew: () => void;
}

let {
  templates,
  loading,
  orgId,
  onconfirmdelete,
  onpublish,
  onarchive,
  onrestore,
  oncreatenew,
}: Props = $props();
</script>

{#if loading}
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    {#each Array(3) as _}
      <div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 animate-pulse">
        <div class="h-6 w-2/3 bg-muted rounded"></div>
        <div class="h-4 w-full bg-muted rounded"></div>
        <div class="h-8 w-1/2 bg-muted rounded mt-2"></div>
      </div>
    {/each}
  </div>
{:else if templates.length === 0}
  <div class="flex flex-col items-center justify-center py-16 px-4 bg-card border border-border border-dashed rounded-xl gap-4">
    <div class="bg-muted p-4 rounded-full text-muted-foreground">
      <FileText size={32} />
    </div>
    <div class="flex flex-col gap-1">
      <h3 class="text-sm font-semibold text-foreground">{m.templates_empty()}</h3>
      <p class="text-xs text-muted-foreground">{m.templates_empty_desc()}</p>
    </div>
    <Button variant="outline" size="sm" onclick={oncreatenew}>
      <Plus size={14} class="me-1" />
      {m.templates_create_btn()}
    </Button>
  </div>
{:else}
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-start">
    {#each templates as template (template.id)}
      <Card class="flex flex-col h-full">
        <CardHeader class="pb-3">
          <div class="flex items-start justify-between gap-2">
            <Badge variant={template.status === 'published' ? 'success' : (template.status === 'archived' ? 'outline' : 'secondary')}>
              {template.status === 'published' ? m.templates_status_active() : (template.status === 'archived' ? m.templates_status_archived() : m.templates_status_draft())}
            </Badge>
            <div class="text-[10px] text-muted-foreground">v{template.version}</div>
          </div>
          <CardTitle class="text-sm font-bold text-foreground mt-2">
            {l(template.name_ar, template.name)}
          </CardTitle>
          <CardDescription class="text-xs text-muted-foreground line-clamp-2 mt-1">
            {template.description || m.templates_no_description()}
          </CardDescription>
        </CardHeader>
        
        <CardContent class="flex-grow pb-4 flex flex-col gap-2">
          <div class="flex items-center justify-between text-xs text-muted-foreground border-t border-border/40 pt-3">
            <span>{m.templates_label_doc_type()}</span>
            <span class="font-medium text-foreground">{template.document_type}</span>
          </div>
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{m.templates_label_entity_type()}</span>
            <span class="font-medium text-foreground">{template.entity_type}</span>
          </div>
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{m.templates_label_size()}</span>
            <span class="font-medium text-foreground">{template.page_size} ({template.orientation})</span>
          </div>
        </CardContent>

        <CardFooter class="border-t border-border/40 pt-3 flex justify-between gap-2 bg-muted/10 flex-nowrap overflow-x-auto">
          <div class="flex items-center gap-1 shrink-0">
            <Button
              variant="destructive"
              size="icon-xs"
              class="h-8 w-8 rounded-lg cursor-pointer"
              onclick={() => onconfirmdelete(template.id)}
            >
              <Trash2 size={13} />
            </Button>
          </div>
          
          <div class="flex items-center gap-1.5 shrink-0">
            <Button
              variant="outline"
              size="xs"
              class="cursor-pointer"
              href={`/${orgId}/templates/${template.id}`}
            >
              <Edit size={13} class="me-1" />
              {m.templates_action_edit()}
            </Button>

            {#if template.status === 'published'}
              <Button
                variant="outline"
                size="xs"
                class="cursor-pointer"
                onclick={() => onarchive(template.id)}
              >
                <Archive size={13} class="me-1" />
                {m.templates_action_archive()}
              </Button>
            {:else}
              {#if template.status === 'archived'}
                <Button
                  variant="outline"
                  size="xs"
                  class="cursor-pointer"
                  onclick={() => onrestore(template.id)}
                >
                  <RefreshCw size={13} class="me-1" />
                  {m.templates_action_restore()}
                </Button>
              {:else}
                <Button
                  variant="lime"
                  size="xs"
                  class="cursor-pointer"
                  onclick={() => onpublish(template.id)}
                >
                  <CheckCircle size={13} class="me-1" />
                  {m.templates_action_publish()}
                </Button>
              {/if}
            {/if}
          </div>
        </CardFooter>
      </Card>
    {/each}
  </div>
{/if}
