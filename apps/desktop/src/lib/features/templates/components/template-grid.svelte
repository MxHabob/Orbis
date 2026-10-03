<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { AdjustmentsHorizontal, ArchiveBox, CheckCircle, DocumentPage, Pencil, Plus, Refresh, Trash } from "@orbis/icons";

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
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
    {#each Array(6) as _}
      <Card class="flex flex-col h-full bg-card/40 border-border/40 rounded-2xl">
        <CardHeader class="p-4 pb-2">
          <div class="flex items-start justify-between gap-3">
            <Skeleton class="h-5 w-1/2 rounded" />
            <Skeleton class="h-5 w-16 rounded" />
          </div>
        </CardHeader>
        
        <CardContent class="flex-grow px-4 pb-4 flex flex-col gap-3">
          <Skeleton class="h-4 w-full rounded mt-1" />
          <Skeleton class="h-4 w-3/4 rounded" />
          
          <div class="mt-auto pt-1 flex items-center gap-1.5">
            <Skeleton class="h-5 w-14 rounded" />
            <Skeleton class="h-5 w-20 rounded" />
            <Skeleton class="h-5 w-10 rounded" />
            <Skeleton class="h-3 w-6 rounded ms-auto" />
          </div>
        </CardContent>

        <CardFooter class="p-3 border-t border-border/40 bg-muted/10 flex items-center justify-between gap-2">
          <Skeleton class="h-7 w-7 rounded-md" />
          <div class="flex items-center gap-1.5">
            <Skeleton class="h-7 w-7 rounded-md" />
            <Skeleton class="h-7 w-16 rounded-md" />
          </div>
        </CardFooter>
      </Card>
    {/each}
  </div>
{:else if templates.length === 0}
  <div class="flex flex-col items-center justify-center py-20 px-4 bg-card/20 border border-border/40 border-dashed rounded-[2rem] gap-5 mx-auto w-full max-w-3xl my-8">
    <div class="h-16 w-16 bg-muted/30 rounded-2xl flex items-center justify-center text-muted-foreground">
      <DocumentPage size={32} strokeWidth={1.5} />
    </div>
    <div class="text-center flex flex-col gap-1.5">
      <h3 class="text-base font-bold text-foreground">{m.settings_templates_empty()}</h3>
      <p class="text-sm text-muted-foreground">{m.settings_templates_empty_desc()}</p>
    </div>
    <Button variant="outline" size="sm" class="mt-2 h-9 border-border/40 hover:bg-muted/50 font-medium" onclick={oncreatenew}>
      <Plus size={16} class="me-2" strokeWidth={2} />
      {m.settings_templates_create_btn()}
    </Button>
  </div>
{:else}
  <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6 text-start">
    {#each templates as template (template.id)}
      <Card class="flex flex-col h-full">
        <CardHeader class="p-4 pb-2">
          <div class="flex items-start justify-between gap-3">
            <CardTitle class="text-sm font-bold text-foreground line-clamp-1 leading-snug" title={l(template.nameAr ?? template.name_ar, template.name)}>
              {l(template.nameAr ?? template.name_ar, template.name)}
            </CardTitle>
            <Badge variant={template.status === 'published' ? 'success' : (template.status === 'archived' ? 'outline' : 'secondary')} class="shrink-0 h-5 px-1.5 text-[10px] leading-none">
              {template.status === 'published' ? m.common_phase_active() : (template.status === 'archived' ? m.common_year_status_archived() : m.common_status_draft())}
            </Badge>
          </div>
        </CardHeader>
        
        <CardContent class="grow px-4 pb-4 flex flex-col gap-3">
          <p class="text-xs text-muted-foreground line-clamp-2 leading-relaxed">
            {template.description || m.settings_templates_no_description()}
          </p>
          
          <div class="mt-auto pt-1 flex flex-wrap items-center gap-1.5">
            <Badge variant="secondary" class="h-5 px-1.5 text-[10px] bg-muted/50 font-normal hover:bg-muted/50">{template.documentType ?? template.document_type}</Badge>
            <Badge variant="secondary" class="h-5 px-1.5 text-[10px] bg-muted/50 font-normal hover:bg-muted/50">{template.entityType ?? template.entity_type}</Badge>
            <Badge variant="secondary" class="h-5 px-1.5 text-[10px] bg-muted/50 font-normal hover:bg-muted/50">{template.pageSize ?? template.page_size}</Badge>
            <span class="text-[10px] text-muted-foreground/50 font-mono ms-auto">v{template.version}</span>
          </div>
        </CardContent>

        <CardFooter class="p-3 border-t border-border/40 bg-muted/10 flex items-center justify-between gap-2">
          <Button
            variant="destructive"
            size="icon-xs"
            class="h-7 w-7 rounded-md shrink-0 cursor-pointer"
            onclick={() => onconfirmdelete(template.id)}
            title={m.common_action_delete()}
          >
            <Trash size={13} />
          </Button>
          
          <div class="flex items-center gap-1.5 shrink-0">
            <Button
              variant="outline"
              size="icon-xs"
              class="h-7 w-7 rounded-md cursor-pointer"
              href={`/${orgId}/templates/${template.id}`}
              title={m.common_action_edit_simple()}
            >
              <Pencil size={13} />
            </Button>

            {#if template.status === 'published'}
              <Button
                variant="outline"
                size="icon-xs"
                class="h-7 w-7 rounded-md cursor-pointer"
                onclick={() => onarchive(template.id)}
                title={m.settings_templates_action_archive()}
              >
                <ArchiveBox size={13} />
              </Button>
            {:else}
              {#if template.status === 'archived'}
                <Button
                  variant="outline"
                  size="icon-xs"
                  class="h-7 w-7 rounded-md cursor-pointer"
                  onclick={() => onrestore(template.id)}
                  title={m.common_titlebar_restore()}
                >
                  <Refresh size={13} />
                </Button>
              {:else}
                <Button
                  variant="default"
                  size="xs"
                  class="h-7 px-2.5 rounded-md font-medium cursor-pointer"
                  onclick={() => onpublish(template.id)}
                >
                  <CheckCircle size={13} class="me-1.5" />
                  {m.settings_templates_action_publish()}
                </Button>
              {/if}
            {/if}
          </div>
        </CardFooter>
      </Card>
    {/each}
  </div>
{/if}
