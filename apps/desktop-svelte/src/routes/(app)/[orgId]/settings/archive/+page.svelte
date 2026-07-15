<script lang="ts">
import {
  useDocuments,
  useDeleteDocument,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import { Button } from "@orbis/ui-svelte/button";
import { Trash2, File, Image as ImageIcon, Loader2 } from "lucide-svelte";
import type { Document } from "@orbis/sdk-core";

const documentsQuery = useDocuments({ ownerId: "all" });
const deleteMutation = useDeleteDocument();

function handleDelete(id: string) {
  if (confirm("Are you sure you want to delete this document?")) {
    $deleteMutation.mutate(id);
  }
}

// Flatten pages into a single array of documents
let documents = $derived(
  $documentsQuery.data?.pages.flatMap((p) => p.items) || [],
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

<div class="flex flex-col gap-6 p-6 h-full overflow-auto">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Archive</h1>
      <p class="text-sm text-muted-foreground mt-1">
        Manage all organization documents, files, avatars, and attachments.
      </p>
    </div>
  </div>

  {#if $documentsQuery.isLoading && documents.length === 0}
    <div class="flex items-center justify-center py-12">
      <Loader2 class="h-8 w-8 animate-spin text-primary" />
    </div>
  {:else if documents.length === 0}
    <div class="flex flex-col items-center justify-center py-20 text-center bg-muted/20 rounded-xl border border-dashed">
      <File class="h-12 w-12 text-muted-foreground/50 mb-4" />
      <h3 class="text-lg font-medium">No documents found</h3>
      <p class="text-sm text-muted-foreground">Uploaded files will appear here.</p>
    </div>
  {:else}
    {#each Object.entries(groupedDocs) as [category, docs]}
      <div class="space-y-4">
        <h2 class="text-lg font-medium capitalize border-b pb-2">{category}</h2>
        <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {#each docs as doc}
            <div class="group relative flex flex-col items-center p-2 rounded-xl border bg-card hover:border-primary/50 transition-colors">
              <div class="w-full aspect-square rounded-lg bg-muted flex items-center justify-center overflow-hidden mb-2 relative">
                {#if doc.mime_type?.startsWith("image/")}
                  {#await resolveDocumentUrl(doc.url)}
                    <Loader2 class="h-4 w-4 animate-spin text-muted-foreground" />
                  {:then srcUrl}
                    <img src={srcUrl} alt={doc.name} class="w-full h-full object-cover" />
                  {/await}
                {:else}
                  <File class="h-8 w-8 text-muted-foreground" />
                {/if}
                
                <!-- Overlay Actions -->
                <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity gap-2">
                  {#await resolveDocumentUrl(doc.url) then srcUrl}
                    <a href={srcUrl} target="_blank" rel="noopener noreferrer" class="p-2 bg-background/80 hover:bg-background rounded-full text-foreground shadow-sm transition-colors" title="View">
                      <File class="h-4 w-4" />
                    </a>
                  {/await}
                  <Button variant="destructive" class="p-2" onclick={() => handleDelete(doc.id)} title="Delete">
                    <Trash2 class="h-4 w-4" />
                  </Button>
                </div>
              </div>
              
              <div class="w-full px-1">
                <p class="text-xs font-medium truncate w-full" title={doc.name}>{doc.name}</p>
                <p class="text-[10px] text-muted-foreground truncate w-full flex justify-between">
                  <span>{doc.extension?.toUpperCase() || 'BIN'}</span>
                  <span>{((doc.size || 0) / 1024).toFixed(1)} KB</span>
                </p>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
    
    {#if $documentsQuery.hasNextPage}
      <div class="flex justify-center pt-8 pb-4">
        <Button 
          variant="outline" 
          disabled={$documentsQuery.isFetchingNextPage}
          onclick={() => $documentsQuery.fetchNextPage()}
        >
          {#if $documentsQuery.isFetchingNextPage}
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
            Loading more...
          {:else}
            Load More
          {/if}
        </Button>
      </div>
    {/if}
  {/if}
</div>
