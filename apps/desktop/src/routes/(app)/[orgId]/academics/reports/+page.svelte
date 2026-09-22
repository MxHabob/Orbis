<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { ChartBar, DocumentPage, InfoCircle } from "@orbis/icons";

let reports = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

async function loadReports() {
  try {
    isLoading = true;
    error = null;
    reports = await api.listReportDefinitions("academics");
  } catch (e: any) {
    error = e.message || "Failed to load reports";
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadReports();
});
</script>

<div class="p-6 flex flex-col gap-6">
  <div>
    <h1 class="text-xl font-bold">التقارير الأكاديمية</h1>
    <p class="text-sm text-muted-foreground mt-1">اختر التقرير لعرض البيانات وتصديرها.</p>
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each Array(3) as _}
        <Skeleton class="h-36 w-full rounded-xl" />
      {/each}
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <InfoCircle size={32} class="text-destructive opacity-50" />
      <p class="text-sm">{error}</p>
      <Button variant="outline" size="sm" onclick={loadReports}>إعادة المحاولة</Button>
    </div>
  {:else if reports.length === 0}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <DocumentPage size={32} class="opacity-30" />
      <p class="text-sm">لا توجد تقارير متاحة حالياً.</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each reports as report}
        <a
          href={`/${page.params.orgId}/academics/reports/${report.id}`}
          class="flex flex-col gap-2 p-6 rounded-xl border bg-card hover:border-primary hover:shadow-md transition-all"
        >
          <ChartBar class="h-8 w-8 text-primary mb-2" />
          <h3 class="font-medium">{report.name}</h3>
          {#if report.description}
            <p class="text-xs text-muted-foreground">{report.description}</p>
          {/if}
        </a>
      {/each}
    </div>
  {/if}
</div>
