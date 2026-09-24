<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { ArrowLeft, ArrowRight, Bolt, ChartBar, CheckCircle, InfoCircle, Lock, SettingsAlt, UserBadgeCheck, Wrench } from "@orbis/icons";
import {
  createAiToolsQuery,
  createAutopilotDiagnosticsQuery,
  useConfigureAutopilot,
} from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import type { ActionWithStatus } from "$lib/features/ai/session-store";

let {
  orgId = "",
  pendingActions = [],
  onBack,
  onApproveAction,
  onRejectAction,
}: {
  orgId: string;
  pendingActions?: ActionWithStatus[];
  onBack: () => void;
  onApproveAction?: (action: ActionWithStatus) => void;
  onRejectAction?: (action: ActionWithStatus) => void;
} = $props();

const isRtl = $derived(getLocale() === "ar");

// 1. AI Tools Query
const toolsQuery = createAiToolsQuery();
const tools = $derived(toolsQuery.data ?? []);

// 2. Autopilot Diagnostics Query
const diagnosticsQuery = createAutopilotDiagnosticsQuery(() => orgId);
const diagnostics = $derived(
  diagnosticsQuery.data ?? {
    status: "standby",
    mode: "advisory",
    autoLockGrades: false,
  },
);

const configureAutopilot = useConfigureAutopilot();

let toolSearch = $state("");
const filteredTools = $derived(
  tools.filter((t) => {
    if (!toolSearch.trim()) return true;
    const query = toolSearch.toLowerCase();
    return (
      t.name.toLowerCase().includes(query) ||
      t.description.toLowerCase().includes(query)
    );
  }),
);

async function toggleAutopilotMode() {
  const newMode = diagnostics.mode === "advisory" ? "autonomous" : "advisory";
  try {
    await configureAutopilot.mutateAsync({
      orgId,
      isEnabled: true,
      paused: false,
      mode: newMode,
    });
    toast.success(`تم تغيير وضع الطيار الآلي إلى ${newMode === "autonomous" ? "المستقل" : "الاستشاري"}`);
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}
</script>

<div class="px-4 py-2.5 border-b border-border bg-muted/30 flex items-center justify-between shrink-0 select-none">
  <Button 
    variant="ghost" 
    size="xs" 
    class="text-xs h-7 font-semibold cursor-pointer text-muted-foreground hover:text-foreground flex items-center gap-1"
    onclick={onBack}
  >
    {#if isRtl}
      <ArrowRight size={13} />
    {:else}
      <ArrowLeft size={13} />
    {/if}
    العودة للمحادثة
  </Button>
  <span class="text-xs font-bold text-foreground flex items-center gap-1.5">
    <ChartBar size={13} class="text-primary" />
    مركز التحكم والأدوات
  </span>
</div>

<div class="flex-1 overflow-y-auto p-4 space-y-5 min-h-0 select-text scrollbar-hide bg-background/50">
  
  <!-- 1. Pending Consequential Actions Section -->
  {#if pendingActions.length > 0}
    <div class="space-y-2.5">
      <div class="flex items-center justify-between">
        <span class="text-xs font-bold text-foreground flex items-center gap-1.5">
          <InfoCircle size={13} class="text-amber-500" />
          الإجراءات المعلقة ({pendingActions.length})
        </span>
      </div>

      <div class="space-y-2">
        {#each pendingActions as action}
          <div class="border border-amber-500/30 bg-amber-500/5 rounded-xl p-3 space-y-2 text-start">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-foreground">{action.title}</span>
              <Badge variant="secondary" class="text-[8px] bg-amber-500/20 text-amber-700 dark:text-amber-300">
                يتطلب موافقة
              </Badge>
            </div>
            <p class="text-[11px] text-muted-foreground leading-relaxed">
              {action.description}
            </p>
            <div class="flex items-center gap-2 pt-1">
              <Button
                size="xs"
                variant="default"
                class="text-[10.5px] h-6 px-2.5 bg-emerald-600 hover:bg-emerald-700 text-white cursor-pointer"
                onclick={() => onApproveAction?.(action)}
              >
                <CheckCircle size={11} class="me-1" />
                موافقة وتنفيذ
              </Button>
              <Button
                size="xs"
                variant="ghost"
                class="text-[10.5px] h-6 px-2.5 text-muted-foreground hover:text-destructive cursor-pointer"
                onclick={() => onRejectAction?.(action)}
              >
                إلغاء
              </Button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- 2. Autopilot Diagnostic Panel -->
  <div class="border border-border/80 rounded-xl p-3.5 bg-card text-foreground space-y-3 shadow-2xs">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Bolt size={15} class="text-primary" />
        <span class="text-xs font-bold">محرك الطيار الآلي (Autopilot Engine)</span>
      </div>
      <Badge 
        variant={diagnostics.status === 'active' ? 'lime' : 'secondary'}
        class="text-[9px] px-1.5 font-mono"
      >
        {diagnostics.status === 'active' ? 'نشط (Active)' : 'في وضع الاستعداد'}
      </Badge>
    </div>

    <div class="grid grid-cols-2 gap-2 text-[11px]">
      <div class="bg-muted/40 p-2 rounded-lg border border-border/50">
        <span class="text-muted-foreground block text-[9.5px]">وضع التشغيل الحالي:</span>
        <strong class="text-foreground capitalize">{diagnostics.mode === 'autonomous' ? 'مستقل (Autonomous)' : 'استشاري (Advisory)'}</strong>
      </div>
      <div class="bg-muted/40 p-2 rounded-lg border border-border/50">
        <span class="text-muted-foreground block text-[9.5px]">قفل الدرجات التلقائي:</span>
        <strong class="text-foreground">{diagnostics.autoLockGrades ? 'مفعّل' : 'معطّل'}</strong>
      </div>
    </div>

    <Button
      variant="outline"
      size="xs"
      class="w-full text-xs h-7.5 font-semibold cursor-pointer border-border hover:bg-muted"
      onclick={toggleAutopilotMode}
      disabled={configureAutopilot.isPending}
    >
      <Settings2 size={12} class="me-1.5" />
      تبديل الوضع إلى {diagnostics.mode === 'advisory' ? 'المستقل (Autonomous)' : 'الاستشاري (Advisory)'}
    </Button>
  </div>

  <!-- 3. AI Platform Tools Capabilities -->
  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Wrench size={14} class="text-primary" />
        <span class="text-xs font-bold text-foreground">أدوات النظام المتاحة للوكيل ({tools.length})</span>
      </div>
    </div>

    {#if toolsQuery.isLoading}
      <div class="space-y-2">
        <Skeleton class="h-16 w-full rounded-lg" />
        <Skeleton class="h-16 w-full rounded-lg" />
        <Skeleton class="h-16 w-full rounded-lg" />
      </div>
    {:else if toolsQuery.isError}
      <div class="border border-destructive/20 bg-destructive/5 rounded-lg p-3 text-center text-xs text-destructive">
        تعذر تحميل قائمة أدوات النظام
      </div>
    {:else if filteredTools.length === 0}
      <div class="text-center py-6 text-xs text-muted-foreground">
        لا توجد أدوات مطابقة
      </div>
    {:else}
      <div class="space-y-2">
        {#each filteredTools as tool}
          <div class="border border-border/70 rounded-xl p-3 bg-card space-y-1.5 text-start hover:border-primary/40 transition-colors shadow-2xs">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold font-mono text-primary flex items-center gap-1.5">
                <Wrench size={11} />
                {tool.name}
              </span>
              <div class="flex items-center gap-1">
                {#if tool.is_consequential}
                  <Badge variant="secondary" class="text-[8px] bg-amber-500/15 text-amber-700 dark:text-amber-300 border-amber-500/20">
                    تعديل بيانات
                  </Badge>
                {:else}
                  <Badge variant="outline" class="text-[8px] text-muted-foreground">
                    قراءة فقط
                  </Badge>
                {/if}
              </div>
            </div>

            <p class="text-[11px] text-muted-foreground leading-normal">
              {tool.description}
            </p>

            {#if tool.required_permission}
              <div class="text-[9.5px] text-muted-foreground/80 flex items-center gap-1 pt-0.5">
                <Lock size={10} />
                الصلاحية المطلوبة: <code class="font-mono text-foreground/80">{tool.required_permission}</code>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>

</div>
