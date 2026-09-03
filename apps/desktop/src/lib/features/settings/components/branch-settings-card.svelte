<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Label } from "@orbis/ui/label";
  import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui/select";
  import { Switch } from "@orbis/ui/switch";
  import { commands } from "$lib/core/ipc";
  import { extractErrorMessage } from "$lib/core/ipc";
  import { toast } from "@orbis/ui";

  type BranchPolicy = {
    enabled: boolean;
    selection_required: boolean;
    data_isolation: "shared" | "isolated";
    max_branches: number | null;
    creation_requires_approval: boolean;
    default_branch_id: string | null;
  };

  type Branch = { id: string; name: string; name_ar?: string | null; is_active: number; unit_type: string };

  let { org, isSaving = false }: { org: any; isSaving?: boolean } = $props();

  let branches = $state<Branch[]>([]);
  let isLoadingBranches = $state(true);
  let isSavingPolicy = $state(false);
  let savedPolicy = $state<BranchPolicy | null>(null);

  const defaultPolicy = (): BranchPolicy => ({
    enabled: true,
    selection_required: false,
    data_isolation: "shared",
    max_branches: null,
    creation_requires_approval: false,
    default_branch_id: null,
  });

  function readPolicy(): BranchPolicy {
    const settings = typeof org?.settings === "string"
      ? (() => { try { return JSON.parse(org.settings); } catch { return {}; } })()
      : (org?.settings || {});
    const policy = settings.branch_policy || {};
    return {
      ...defaultPolicy(),
      ...policy,
      max_branches: policy.max_branches == null ? null : Number(policy.max_branches),
    };
  }

  let policy = $state<BranchPolicy>(defaultPolicy());
  let maxBranchesInput = $state("");

  $effect(() => {
    if (org && !savedPolicy) {
      policy = readPolicy();
      maxBranchesInput = policy.max_branches == null ? "" : String(policy.max_branches);
      savedPolicy = { ...policy };
    }
  });

  const isDirty = $derived(
    savedPolicy !== null && JSON.stringify(policy) !== JSON.stringify(savedPolicy),
  );

  const activeBranchCount = $derived(branches.length);

  onMount(async () => {
    try {
      const units = await commands.organization.listOrgUnits();
      branches = units.filter((unit: Branch) => unit.is_active === 1 && unit.unit_type === "branch");
    } catch {
      branches = [];
    } finally {
      isLoadingBranches = false;
    }
  });

  function updateMaxBranches(value: string) {
    maxBranchesInput = value;
    const normalized = value.trim();
    policy.max_branches = normalized === "" ? null : Math.max(1, Number.parseInt(normalized, 10) || 1);
  }

  async function savePolicy() {
    if (policy.max_branches !== null && policy.max_branches < activeBranchCount) {
      toast.error("الحد الأقصى لا يمكن أن يقل عن عدد الفروع الحالية");
      return;
    }
    if (policy.default_branch_id && !branches.some((branch) => branch.id === policy.default_branch_id)) {
      toast.error("الفرع الافتراضي غير صالح");
      return;
    }

    try {
      isSavingPolicy = true;
      await commands.organization.updateSettings({ branch_policy: policy });
      savedPolicy = { ...policy };
      toast.success("تم حفظ إعدادات الفروع");
    } catch (error) {
      toast.error(extractErrorMessage(error));
    } finally {
      isSavingPolicy = false;
    }
  }

  function resetPolicy() {
    if (!savedPolicy) return;
    policy = { ...savedPolicy };
    maxBranchesInput = policy.max_branches == null ? "" : String(policy.max_branches);
  }
</script>

<section class="border border-border rounded-md overflow-hidden flex flex-col">
  <div class="bg-muted/50 px-6 py-3 border-b border-border flex items-center justify-between gap-4">
    <div class="text-start">
      <h2 class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase">إدارة الفروع</h2>
      <p class="text-xs text-muted-foreground mt-1">سياسة الفروع ونطاق ظهور البيانات داخل المؤسسة</p>
    </div>
    <div class="text-end shrink-0">
      <span class="text-2xl font-semibold tabular-nums">{activeBranchCount}</span>
      <span class="block text-[10px] text-muted-foreground">فرع نشط</span>
    </div>
  </div>

  <div class="p-6 grid grid-cols-1 md:grid-cols-2 gap-5 text-start">
    <div class="flex items-center justify-between gap-4 md:col-span-2">
      <div>
        <Label for="branches-enabled" class="text-xs font-semibold">تفعيل إدارة الفروع</Label>
        <p class="text-[11px] text-muted-foreground mt-1">عند التعطيل تستخدم المؤسسة نطاقاً واحداً مشتركاً</p>
      </div>
      <Switch id="branches-enabled" checked={policy.enabled} onCheckedChange={(value: boolean) => (policy.enabled = value)} disabled={isSaving || isSavingPolicy} />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="branches-isolation" class="text-xs font-semibold">نطاق ظهور البيانات</Label>
      <Select type="single" value={policy.data_isolation} onValueChange={(value: string | undefined) => (policy.data_isolation = (value as "shared" | "isolated") || "shared")} disabled={!policy.enabled || isSaving || isSavingPolicy}>
        <SelectTrigger id="branches-isolation" class="h-9 text-xs">{policy.data_isolation === "isolated" ? "معزول حسب الفرع" : "مشترك بين الفروع"}</SelectTrigger>
        <SelectContent>
          <SelectItem value="shared">مشترك بين الفروع</SelectItem>
          <SelectItem value="isolated">معزول حسب الفرع</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <div class="flex flex-col gap-2">
      <Label for="branches-limit" class="text-xs font-semibold">الحد الأقصى للفروع</Label>
      <Input id="branches-limit" type="number" min="1" value={maxBranchesInput} oninput={(event: Event) => updateMaxBranches((event.currentTarget as HTMLInputElement).value)} placeholder="غير محدود" disabled={!policy.enabled || isSaving || isSavingPolicy} class="h-9 text-xs" />
      <span class="text-[10px] text-muted-foreground">اتركه فارغاً للسماح بعدد غير محدود</span>
    </div>

    <label class="flex items-center justify-between gap-4 border border-border rounded-md p-3">
      <span>
        <span class="block text-xs font-semibold">إلزام اختيار الفرع</span>
        <span class="block text-[10px] text-muted-foreground mt-1">يجب تحديد فرع قبل فتح البيانات</span>
      </span>
      <Switch checked={policy.selection_required} onCheckedChange={(value: boolean) => (policy.selection_required = value)} disabled={!policy.enabled || isSaving || isSavingPolicy} />
    </label>

    <label class="flex items-center justify-between gap-4 border border-border rounded-md p-3">
      <span>
        <span class="block text-xs font-semibold">موافقة على إنشاء فرع</span>
        <span class="block text-[10px] text-muted-foreground mt-1">يحوّل الإنشاء إلى طلب موافقة</span>
      </span>
      <Switch checked={policy.creation_requires_approval} onCheckedChange={(value: boolean) => (policy.creation_requires_approval = value)} disabled={!policy.enabled || isSaving || isSavingPolicy} />
    </label>

    <div class="flex flex-col gap-2 md:col-span-2">
      <Label for="default-branch" class="text-xs font-semibold">الفرع الافتراضي</Label>
      <Select type="single" value={policy.default_branch_id || "none"} onValueChange={(value: string | undefined) => (policy.default_branch_id = value === "none" ? null : value || null)} disabled={!policy.enabled || isLoadingBranches || isSaving || isSavingPolicy}>
        <SelectTrigger id="default-branch" class="h-9 text-xs">{branches.find((branch) => branch.id === policy.default_branch_id)?.name_ar || branches.find((branch) => branch.id === policy.default_branch_id)?.name || "بدون فرع افتراضي"}</SelectTrigger>
        <SelectContent>
          <SelectItem value="none">بدون فرع افتراضي</SelectItem>
          {#each branches as branch}
            <SelectItem value={branch.id}>{branch.name_ar || branch.name}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>
  </div>

  {#if isDirty}
    <div class="px-6 py-3 border-t border-border flex items-center justify-end gap-2">
      <Button type="button" variant="outline" size="sm" onclick={resetPolicy} disabled={isSavingPolicy}>إلغاء</Button>
      <Button type="button" size="sm" onclick={savePolicy} disabled={isSaving || isSavingPolicy}>حفظ إعدادات الفروع</Button>
    </div>
  {/if}
</section>
