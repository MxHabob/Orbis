<!--
@component RankingPoliciesSettingsPage
Manages student ranking policies (scope, ranking method, tie handling, and metrics).
Uses the Assessment Engine's ranking_policy CRUD operations.
-->
<script lang="ts">
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import { toast } from "svelte-sonner";
import {
  Sliders,
  Plus,
  Trash2,
  Loader2,
  Pencil,
  Award,
} from "@lucide/svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);
const queryClient = useQueryClient();

const policiesQuery = createQuery({
  queryKey: ["ranking-policies", orgId],
  queryFn: () => commands.assessment.listRankingPolicies(),
});

const policies = $derived($policiesQuery.data ?? []);

// ── Create / Edit Dialog ─────────────────────────────────────
let showDialog = $state(false);
let editingId = $state<string | null>(null);
let isSaving = $state(false);
let form = $state({
  name: "",
  is_enabled: 1,
  ranking_scope: "class",
  ranking_method: "standard",
  tie_handling: "same_rank",
  include_failed_students: 0,
  ranking_metric: "total_score",
});

function resetForm() {
  editingId = null;
  form = {
    name: "",
    is_enabled: 1,
    ranking_scope: "class",
    ranking_method: "standard",
    tie_handling: "same_rank",
    include_failed_students: 0,
    ranking_metric: "total_score",
  };
}

function openCreate() {
  resetForm();
  showDialog = true;
}

function openEdit(policy: any) {
  editingId = policy.id;
  form = {
    name: policy.name,
    is_enabled: policy.is_enabled ?? 1,
    ranking_scope: policy.ranking_scope || "class",
    ranking_method: policy.ranking_method || "standard",
    tie_handling: policy.tie_handling || "same_rank",
    include_failed_students: policy.include_failed_students ?? 0,
    ranking_metric: policy.ranking_metric || "total_score",
  };
  showDialog = true;
}

async function handleSave() {
  if (!form.name.trim()) return;
  isSaving = true;
  try {
    if (editingId) {
      await commands.assessment.updateRankingPolicy(editingId, form);
      toast.success("تم تحديث سياسة الترتيب بنجاح");
    } else {
      await commands.assessment.createRankingPolicy(form);
      toast.success("تم إنشاء سياسة الترتيب بنجاح");
    }
    queryClient.invalidateQueries({ queryKey: ["ranking-policies"] });
    showDialog = false;
  } catch (e) {
    toast.error(extractErrorMessage(e));
  } finally {
    isSaving = false;
  }
}

async function handleDelete(id: string) {
  try {
    await commands.assessment.deleteRankingPolicy(id);
    toast.success("تم حذف سياسة الترتيب بنجاح");
    queryClient.invalidateQueries({ queryKey: ["ranking-policies"] });
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}
</script>

<svelte:head>
  <title>سياسات الترتيب | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 select-none scrollbar-hide">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="space-y-1">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <Award class="h-5 w-5 text-primary" />
        سياسات الترتيب
      </h1>
      <p class="text-xs text-muted-foreground">
        تحديد قواعد وخوارزميات ترتيب الطلاب على مستوى الصف والمرحلة وطريقة التعامل مع التعادل
      </p>
    </div>
    <Button onclick={openCreate} size="sm" class="gap-1.5 rounded-lg">
      <Plus class="h-4 w-4" />
      إضافة سياسة
    </Button>
  </div>

  <!-- Content -->
  {#if $policiesQuery.isLoading}
    <SettingsFormSkeleton />
  {:else if policies.length === 0}
    <div class="flex flex-col items-center justify-center p-12 text-center rounded-xl border border-dashed border-border bg-card/50">
      <Award class="h-10 w-10 text-muted-foreground/50 mb-3" />
      <p class="text-sm font-medium text-foreground">لا توجد سياسات ترتيب</p>
      <p class="text-xs text-muted-foreground mt-1">
        قم بإنشاء سياسة ترتيب لتحديد آليات احتساب ترتيب الطلاب
      </p>
      <Button onclick={openCreate} variant="outline" size="sm" class="mt-4 gap-1.5">
        <Plus class="h-4 w-4" />
        إضافة سياسة
      </Button>
    </div>
  {:else}
    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
      {#each policies as policy (policy.id)}
        <div class="p-4 rounded-xl border border-border/60 bg-card hover:border-primary/40 transition-colors flex flex-col justify-between gap-3 shadow-2xs">
          <div class="space-y-2">
            <div class="flex items-start justify-between gap-2">
              <h3 class="font-semibold text-sm text-foreground truncate">{policy.name}</h3>
              <Badge variant={policy.is_enabled ? "default" : "secondary"} class="text-[10px]">
                {policy.is_enabled ? "مفعّلة" : "معطلة"}
              </Badge>
            </div>

            <div class="space-y-1 text-xs text-muted-foreground">
              <div class="flex justify-between">
                <span>نطاق الترتيب:</span>
                <span class="font-medium text-foreground capitalize">{policy.ranking_scope}</span>
              </div>
              <div class="flex justify-between">
                <span>الخوارزمية:</span>
                <span class="font-medium text-foreground capitalize">{policy.ranking_method}</span>
              </div>
              <div class="flex justify-between">
                <span>معيار الترتيب:</span>
                <span class="font-medium text-foreground">{policy.ranking_metric}</span>
              </div>
              <div class="flex justify-between">
                <span>التعادل:</span>
                <span class="font-medium text-foreground">{policy.tie_handling}</span>
              </div>
            </div>
          </div>

          <div class="flex items-center justify-end gap-1 pt-2 border-t border-border/40">
            <Button
              variant="ghost"
              size="sm"
              class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
              onclick={() => openEdit(policy)}
            >
              <Pencil class="h-3.5 w-3.5" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              class="h-7 w-7 p-0 text-destructive/70 hover:text-destructive"
              onclick={() => handleDelete(policy.id)}
            >
              <Trash2 class="h-3.5 w-3.5" />
            </Button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Create / Edit Dialog -->
<Dialog bind:open={showDialog}>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle>{editingId ? "تعديل سياسة الترتيب" : "إضافة سياسة ترتيب جديدة"}</DialogTitle>
    </DialogHeader>

    <div class="space-y-4 py-2 text-xs">
      <div class="space-y-1.5">
        <Label>اسم السياسة</Label>
        <Input bind:value={form.name} placeholder="مثال: الترتيب العام للصفوف" />
      </div>

      <div class="space-y-1.5">
        <Label>نطاق الترتيب (Scope)</Label>
        <Select
          type="single"
          value={form.ranking_scope}
          onValueChange={(val) => { if (val) form.ranking_scope = val; }}
        >
          <SelectTrigger class="w-full">
            {form.ranking_scope === "class" ? "على مستوى الصف (Class)" : form.ranking_scope === "level" ? "على مستوى المرحلة (Level)" : "على مستوى المؤسسة (Org)"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="class">على مستوى الصف (Class)</SelectItem>
            <SelectItem value="division">على مستوى الشعبة (Division)</SelectItem>
            <SelectItem value="level">على مستوى المرحلة (Level)</SelectItem>
            <SelectItem value="org">على مستوى المؤسسة بالكامل (Org)</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <Label>معيار الترتيب (Metric)</Label>
        <Select
          type="single"
          value={form.ranking_metric}
          onValueChange={(val) => { if (val) form.ranking_metric = val; }}
        >
          <SelectTrigger class="w-full">
            {form.ranking_metric === "total_score" ? "المجموع الكلي (Total Score)" : form.ranking_metric === "percentage" ? "النسبة المئوية (Percentage)" : "المعدل التراكمي (GPA)"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="total_score">المجموع الكلي (Total Score)</SelectItem>
            <SelectItem value="percentage">النسبة المئوية (Percentage)</SelectItem>
            <SelectItem value="gpa">المعدل التراكمي (GPA)</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <Label>طريقة التعامل مع التعادل (Tie Handling)</Label>
        <Select
          type="single"
          value={form.tie_handling}
          onValueChange={(val) => { if (val) form.tie_handling = val; }}
        >
          <SelectTrigger class="w-full">
            {form.tie_handling === "same_rank" ? "نفس الرتبة (Same Rank: 1, 2, 2, 4)" : form.tie_handling === "sequential" ? "ترتيب تسلسلي (Sequential: 1, 2, 3, 4)" : "تخطي الرتبة (Skip)"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="same_rank">نفس الرتبة (Same Rank: 1, 2, 2, 4)</SelectItem>
            <SelectItem value="sequential">ترتيب تسلسلي (Sequential: 1, 2, 3, 4)</SelectItem>
            <SelectItem value="skip">تخطي الرتبة (Skip)</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex items-center justify-between pt-2">
        <Label>تفعيل السياسة</Label>
        <Switch
          checked={form.is_enabled === 1}
          onCheckedChange={(c) => { form.is_enabled = c ? 1 : 0; }}
        />
      </div>

      <div class="flex items-center justify-between">
        <Label>شمل الطلاب الراسبين في الترتيب</Label>
        <Switch
          checked={form.include_failed_students === 1}
          onCheckedChange={(c) => { form.include_failed_students = c ? 1 : 0; }}
        />
      </div>
    </div>

    <DialogFooter>
      <Button variant="outline" onclick={() => { showDialog = false; }}>إلغاء</Button>
      <Button onclick={handleSave} disabled={isSaving || !form.name.trim()}>
        {#if isSaving}
          <Loader2 class="h-4 w-4 animate-spin mr-1" />
        {/if}
        حفظ
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
