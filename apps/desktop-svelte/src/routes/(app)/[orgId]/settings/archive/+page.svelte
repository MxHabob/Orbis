<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  useDocuments,
  useDeleteDocument,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { confirmStore } from "$lib/stores";
import { Trash2, File, Image as ImageIcon, Loader2, ExternalLink } from "@lucide/svelte";
import type { Documents } from "@orbis/sdk-core";

const documentsQuery = useDocuments({ ownerId: "all" });
const deleteMutation = useDeleteDocument();

async function handleDelete(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.common_docs_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (confirmed) {
    deleteMutation.mutate(id);
  }
}

// Flatten pages into a single array of documents
let documents = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
);
let groupedDocs = $derived(
  documents.reduce(
    (acc: Record<string, Document[]>, doc: Document) => {
      const cat = doc.category || "uncategorized";
      if (!acc[cat]) acc[cat] = [];
      acc[cat].push(doc);
      return acc;
    },
    {} as Record<string, Document[]>,
  ),
);
</script>

<div class="flex flex-col gap-6 p-6 h-full overflow-auto scrollbar-hide">
  <!-- Page Header -->
  <div class="flex items-center justify-between pb-4 border-b border-border/40">
    <div>
      <h1 class="text-xl font-bold tracking-tight text-foreground">{m.settings_archive_title()}</h1>
      <p class="text-xs text-muted-foreground mt-1">
        {m.settings_archive_desc()}
      </p>
    </div>
  </div>

  <!-- Loading Skeletons (Uses base Skeleton component styles) -->
  {#if documentsQuery.isLoading && documents.length === 0}
    <div class="space-y-6">
      <div class="space-y-4">
        <Skeleton class="h-6 w-36 rounded-md" />
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {#each Array(12) as _}
            <div class="flex flex-col p-2.5 rounded-xl border border-border/40 bg-card gap-2.5 shadow-2xs">
              <Skeleton class="w-full aspect-square rounded-lg" />
              <div class="space-y-1.5 px-0.5">
                <Skeleton class="h-3.5 w-4/5 rounded" />
                <div class="flex items-center justify-between pt-0.5">
                  <Skeleton class="h-2.5 w-1/4 rounded" />
                  <Skeleton class="h-2.5 w-1/3 rounded" />
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if documents.length === 0}
    <!-- Empty State -->
    <div class="flex flex-col items-center justify-center py-20 text-center bg-muted/10 rounded-2xl border border-dashed border-border/60">
      <div class="w-12 h-12 rounded-2xl bg-muted/40 flex items-center justify-center mb-3">
        <File class="h-6 w-6 text-muted-foreground/60" />
      </div>
      <h3 class="text-base font-semibold text-foreground">{m.settings_archive_empty_title()}</h3>
      <p class="text-xs text-muted-foreground max-w-sm mt-1">{m.settings_archive_empty_desc()}</p>
    </div>
  {:else}
    <!-- Document Groups -->
    {#each Object.entries(groupedDocs) as [category, docs]}
      <div class="space-y-4">
        <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider border-b border-border/40 pb-2">{category}</h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {#each docs as doc}
            <div class="group relative flex flex-col items-center p-2.5 rounded-xl border border-border/40 bg-card hover:border-primary/40 hover:shadow-md transition-all duration-200">
              <div class="w-full aspect-square rounded-lg bg-muted/30 flex items-center justify-center overflow-hidden mb-2 relative">
                {#if doc.mime_type?.startsWith("image/")}
                  {#await resolveDocumentUrl(doc.url)}
                    <Skeleton class="w-full h-full rounded-lg" />
                  {:then srcUrl}
                    <img src={srcUrl} alt={doc.name} class="w-full h-full object-cover transition-transform group-hover:scale-105" />
                  {/await}
                {:else}
                  <File class="h-8 w-8 text-muted-foreground/70" />
                {/if}
                
                <!-- Overlay Actions -->
                <div class="absolute inset-0 bg-background/60 backdrop-blur-xs opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity gap-2">
                  {#await resolveDocumentUrl(doc.url) then srcUrl}
                    <a
                      href={srcUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="p-2 bg-background/90 hover:bg-background rounded-lg text-foreground shadow-sm transition-all hover:scale-105"
                      title={m.settings_archive_action_view()}
                    >
                      <ExternalLink class="h-3.5 w-3.5" />
                    </a>
                  {/await}
                  <Button
                    variant="destructive"
                    size="sm"
                    class="h-7 w-7 p-0 rounded-lg cursor-pointer shadow-sm"
                    onclick={() => handleDelete(doc.id)}
                    title={m.common_action_delete()}
                  >
                    <Trash2 class="h-3.5 w-3.5" />
                  </Button>
                </div>
              </div>
              
              <div class="w-full px-0.5">
                <p class="text-xs font-medium truncate w-full text-foreground" title={doc.name}>{doc.name}</p>
                <p class="text-[10px] text-muted-foreground truncate w-full flex justify-between mt-0.5 font-mono">
                  <span>{doc.extension?.toUpperCase() || 'BIN'}</span>
                  <span>{((doc.size || 0) / 1024).toFixed(1)} KB</span>
                </p>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
    
    <!-- Load More -->
    {#if documentsQuery.hasNextPage}
      <div class="flex justify-center pt-6 pb-4">
        <Button 
          variant="outline" 
          size="sm"
          disabled={documentsQuery.isFetchingNextPage}
          onclick={() => documentsQuery.fetchNextPage()}
          class="text-xs gap-2 rounded-xl"
        >
          {#if documentsQuery.isFetchingNextPage}
            <Loader2 class="h-3.5 w-3.5 animate-spin" />
            <span>{m.settings_archive_loading_more()}</span>
          {:else}
            <span>{m.settings_archive_load_more()}</span>
          {/if}
        </Button>
      </div>
    {/if}
  {/if}
</div>
