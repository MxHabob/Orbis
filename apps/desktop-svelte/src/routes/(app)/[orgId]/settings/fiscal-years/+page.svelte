<script lang="ts">
import { onMount } from "svelte";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardDescription,
} from "@orbis/ui-svelte/card";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import { Calendar, Plus, CalendarRange } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

let fiscalYears = $state<any[]>([]);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    fiscalYears = await commands.finance.listFiscalYears(data.orgId);
  } catch (e) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

async function handleCreateYear() {
  try {
    throw new Error("This feature has been deprecated.");
    toast.success("تم إنشاء وتهيئة السنة المالية الجديدة وفتراتها بنجاح");
    await loadData();
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-start font-sans">
  <div class="max-w-4xl mx-auto p-4 md:p-6 flex flex-col gap-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-foreground">السنوات المالية</h1>
        <p class="text-sm text-muted-foreground">عرض وإدارة الفترات المالية المحاسبية.</p>
      </div>
      <Button onclick={handleCreateYear} class="flex items-center gap-2">
        <Plus class="h-4 w-4" />
        تهيئة سنة مالية جديدة
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Calendar class="h-5 w-5 text-muted-foreground" />
          قائمة السنوات المالية
        </CardTitle>
        <CardDescription>هذه القائمة تديرها مهام الطيار الآلي بشكل افتراضي لضمان استمرارية القيود المحاسبية.</CardDescription>
      </CardHeader>
      <CardContent>
        {#if isLoading}
          <div class="flex items-center justify-center p-8 text-muted-foreground">جاري التحميل...</div>
        {:else if fiscalYears.length === 0}
          <div class="text-center p-12 border border-dashed rounded-lg bg-background/50 flex flex-col items-center justify-center gap-3">
            <CalendarRange class="h-10 w-10 text-muted-foreground opacity-50" />
            <h3 class="text-sm font-semibold">لا يوجد سنوات مالية</h3>
            <p class="text-xs text-muted-foreground max-w-sm">قم بالنقر على تهيئة السنة المالية ليتم إنشاؤها تلقائياً.</p>
          </div>
        {:else}
          <div class="rounded-md border overflow-hidden">
            <Table>
              <TableHeader class="bg-muted/40">
                <TableRow>
                  <TableHead>اسم السنة</TableHead>
                  <TableHead>تاريخ البداية</TableHead>
                  <TableHead>تاريخ النهاية</TableHead>
                  <TableHead>الحالة</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each fiscalYears as year}
                  <TableRow>
                    <TableCell class="font-medium">{year.name}</TableCell>
                    <TableCell class="text-muted-foreground font-mono text-sm">{year.start_date}</TableCell>
                    <TableCell class="text-muted-foreground font-mono text-sm">{year.end_date}</TableCell>
                    <TableCell>
                      {#if year.status === "open"}
                        <Badge variant="lime">مفتوحة</Badge>
                      {:else}
                        <Badge variant="secondary">مغلقة</Badge>
                      {/if}
                    </TableCell>
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>
