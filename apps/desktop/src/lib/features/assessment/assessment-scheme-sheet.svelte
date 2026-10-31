<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { api, type AssessmentSchemeDto } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { InfoCircle, SettingsAlt } from "@orbis/icons";

let {
  open = $bindable(false),
  SectionSubjectId,
  periodId,
}: { open: boolean; SectionSubjectId: string; periodId: string } = $props();

const schemesQuery = createQuery(() => ({
  queryKey: ["assessment-schemes", SectionSubjectId, periodId],
  queryFn: (): Promise<AssessmentSchemeDto[]> => api.listAssessmentSchemes(),
  enabled: !!SectionSubjectId && !!periodId && open,
}));

const schemes = $derived(schemesQuery.data ?? []);
const locale = $derived(getLocale());

function componentWeight(
  scheme: AssessmentSchemeDto,
  weightValue: string,
  maxScoreValue: string,
) {
  const weight = Number(weightValue);
  if (!Number.isFinite(weight)) return 0;
  if (weight !== 1) return weight * 100;

  const schemeMax = Number(scheme.max_score);
  return schemeMax > 0 ? (Number(maxScoreValue) / schemeMax) * 100 : 0;
}
</script>

<Sheet bind:open>
  <SheetContent
    side="end"
    contentClass="data-[side=end]:sm:!max-w-md"
    class="w-full p-0 flex flex-col h-full border-border/50"
  >
    <div class="p-6 border-b border-border/40 bg-muted/20">
      <SheetHeader>
        <SheetTitle class="flex items-center gap-2 text-lg">
          <SettingsAlt size={18} class="text-primary" />
          {m.common_assessment_scheme()}
        </SheetTitle>
      </SheetHeader>
    </div>

    <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
      {#if schemesQuery.isLoading}
        <div class="flex flex-col gap-3">
          {#each Array(3) as _, index (index)}
            <Skeleton class="h-24 w-full rounded-lg" />
          {/each}
        </div>
      {:else if schemesQuery.error}
        <div class="flex flex-col items-center justify-center py-12 text-destructive gap-2 text-center border border-destructive/20 bg-destructive/5 rounded-xl">
          <InfoCircle size={24} />
          <h3 class="text-sm font-bold">{m.common_load_error()}</h3>
          <p class="text-xs opacity-80">{schemesQuery.error.message}</p>
          <Button
            variant="outline"
            size="sm"
            class="mt-2"
            onclick={() => schemesQuery.refetch()}
          >
            {m.common_retry()}
          </Button>
        </div>
      {:else if schemes.length === 0}
        <div class="flex flex-col items-center justify-center py-16 text-muted-foreground text-center border border-dashed border-border/60 rounded-xl bg-muted/10">
          <p class="text-sm">{m.common_assessment_no_scheme()}</p>
        </div>
      {:else}
        <div class="flex flex-col gap-4">
          {#each schemes as scheme (scheme.id)}
            <section class="rounded-xl border border-border/50 bg-card shadow-sm">
              <header class="flex items-start justify-between gap-3 border-b border-border/40 p-4">
                <div class="min-w-0">
                  <h3 class="truncate text-sm font-bold text-foreground">
                    {locale === "ar"
                      ? scheme.name_ar || scheme.name
                      : scheme.name || scheme.name_ar}
                  </h3>
                  {#if scheme.description}
                    <p class="mt-1 text-xs text-muted-foreground">
                      {scheme.description}
                    </p>
                  {/if}
                </div>
                <div class="shrink-0 text-end">
                  <span class="block text-[10px] text-muted-foreground">
                    {m.academics_assessment_max_score()}
                  </span>
                  <span class="text-sm font-bold text-foreground">
                    {scheme.max_score}
                  </span>
                </div>
              </header>

              {#if scheme.components.length > 0}
                <div class="divide-y divide-border/40">
                  {#each [...scheme.components].sort((a, b) => a.sortOrder - b.sortOrder) as component (component.code)}
                    <div class="flex items-center justify-between gap-3 px-4 py-3">
                      <div class="min-w-0">
                        <p class="truncate text-xs font-medium text-foreground">
                          {locale === "ar"
                            ? component.nameAr || component.name
                            : component.name || component.nameAr}
                        </p>
                        <p class="mt-0.5 text-[10px] text-muted-foreground">
                          {m.academics_assessment_component_weight({
                            weight: componentWeight(
                              scheme,
                              component.weight,
                              component.maxScore,
                            ),
                          })}
                        </p>
                      </div>
                      <div class="shrink-0 text-end">
                        <span class="block text-[10px] text-muted-foreground">
                          {m.academics_assessment_max_score()}
                        </span>
                        <span class="text-xs font-semibold text-foreground">
                          {component.maxScore}
                        </span>
                      </div>
                    </div>
                  {/each}
                </div>
              {:else}
                <p class="p-4 text-xs text-muted-foreground">
                  {m.academics_assessment_no_components()}
                </p>
              {/if}
            </section>
          {/each}
        </div>
      {/if}
    </div>

    <div class="p-4 border-t border-border/40 bg-muted/20 flex items-center justify-end">
      <Button variant="outline" onclick={() => (open = false)} class="text-xs h-8">
        {m.common_action_close()}
      </Button>
    </div>
  </SheetContent>
</Sheet>
