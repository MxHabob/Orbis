<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import { commands } from "$lib/core/ipc";
  import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription } from "@orbis/ui-svelte/sheet";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Button } from "@orbis/ui-svelte/button";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { Settings2, AlertCircle } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

  let { open = $bindable(false), classSubjectId, periodId }: { open: boolean, classSubjectId: string, periodId: string } = $props();

  const isArabic = $derived(getLocale() === "ar");

  const componentsQuery = createQuery(() => ({
    queryKey: ["assessment-components", classSubjectId, periodId],
    queryFn: () => commands.assessment.listComponentsForClassSubject(classSubjectId, periodId),
    enabled: !!classSubjectId && !!periodId && open,
  }));

  const components = $derived((componentsQuery.data || []) as any[]);
</script>

<Sheet bind:open>
  <SheetContent side={isArabic ? "left" : "right"} class="w-full sm:max-w-md p-0 flex flex-col h-full bg-background border-border/50">
    <div class="p-6 border-b border-border/40 bg-muted/20">
      <SheetHeader>
        <SheetTitle class="flex items-center gap-2 text-lg">
          <Settings2 size={18} class="text-primary" />
          Assessment Configuration
        </SheetTitle>
        <SheetDescription class="text-xs">
          View the assessment components and weights assigned to this subject.
        </SheetDescription>
      </SheetHeader>
    </div>

    <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
      {#if componentsQuery.isLoading}
        <div class="flex flex-col gap-3">
          {#each Array(4) as _}
            <Skeleton class="h-16 w-full rounded-lg" />
          {/each}
        </div>
      {:else if componentsQuery.error}
        <div class="flex flex-col items-center justify-center py-12 text-destructive gap-2 text-center border border-destructive/20 bg-destructive/5 rounded-xl">
          <AlertCircle size={24} />
          <h3 class="text-sm font-bold">Failed to load</h3>
          <p class="text-xs opacity-80">{componentsQuery.error.message}</p>
        </div>
      {:else if components.length === 0}
        <div class="flex flex-col items-center justify-center py-16 text-muted-foreground text-center border border-dashed border-border/60 rounded-xl bg-muted/10">
          <p class="text-sm">No components configured.</p>
          <p class="text-[10px] mt-1">This subject does not have an active assessment scheme for this period.</p>
        </div>
      {:else}
        <div class="flex flex-col gap-3">
          {#each components as comp (comp.id)}
            <div class="flex flex-col p-4 rounded-xl border border-border/50 bg-card hover:border-primary/30 transition-colors shadow-sm">
              <div class="flex items-start justify-between mb-2">
                <h4 class="font-bold text-sm text-foreground">{comp.name_en || comp.name_ar || comp.exam_type}</h4>
                <Badge variant={comp.is_active === 0 ? "outline" : "secondary"} class="text-[9px] uppercase">
                  {comp.is_active === 0 ? "Inactive" : "Active"}
                </Badge>
              </div>
              
              <div class="flex items-center gap-4 text-[10px] text-muted-foreground mt-2 bg-muted/20 p-2 rounded border border-border/40">
                <div class="flex flex-col">
                  <span class="uppercase tracking-wider opacity-70 mb-0.5">Weight</span>
                  <span class="font-bold text-foreground font-sans">{comp.weight}%</span>
                </div>
                <div class="w-px h-6 bg-border/60"></div>
                <div class="flex flex-col">
                  <span class="uppercase tracking-wider opacity-70 mb-0.5">Max Score</span>
                  <span class="font-bold text-foreground font-sans">{comp.max_score}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <div class="p-4 border-t border-border/40 bg-muted/20 flex items-center justify-end">
      <Button variant="outline" onclick={() => open = false} class="text-xs h-8">Close</Button>
    </div>
  </SheetContent>
</Sheet>
