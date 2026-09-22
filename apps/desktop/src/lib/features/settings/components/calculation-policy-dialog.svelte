<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
let {
  open = $bindable(false),
  form,
  editingId = null,
  isSaving = false,
  getRuleLabel,
  getRoundingLabel,
  onsubmit,
}: {
  open: boolean;
  form: any;
  editingId?: string | null;
  isSaving?: boolean;
  getRuleLabel: (value: string) => string;
  getRoundingLabel: (value: string) => string;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-md w-full min-w-0"><DialogHeader><DialogTitle class="text-sm">{editingId ? "تعديل سياسة الحساب" : "إنشاء سياسة حساب جديدة"}</DialogTitle></DialogHeader><div class="flex flex-col gap-4 py-2 w-full min-w-0"><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">اسم السياسة</Label><Input bind:value={form.name} placeholder="مثال: المجموع الموزون الافتراضي" class="text-xs w-full min-w-0" /></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">نوع القاعدة</Label><Select type="single" value={form.rule_type} onValueChange={(v) => { if (v) form.rule_type = v; }}><SelectTrigger class="text-xs h-9 w-full min-w-0">{getRuleLabel(form.rule_type)}</SelectTrigger><SelectContent class="text-xs"><SelectItem value="weighted_sum">المجموع الموزون</SelectItem><SelectItem value="simple_average">المتوسط البسيط</SelectItem><SelectItem value="custom_ast">صيغة مخصصة (AST)</SelectItem></SelectContent></Select></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">صيغة الحساب</Label><Textarea bind:value={form.formula_expression} class="text-xs font-mono min-h-15 w-full min-w-0 max-w-full break-all" placeholder="SUM(score * weight) / SUM(weight)" /></div><div class="flex flex-col gap-1.5 w-full min-w-0"><Label class="text-xs">سياسة التقريب</Label><Select type="single" value={form.rounding_policy} onValueChange={(v) => { if (v) form.rounding_policy = v; }}><SelectTrigger class="text-xs h-9 w-full min-w-0">{getRoundingLabel(form.rounding_policy)}</SelectTrigger><SelectContent class="text-xs"><SelectItem value="round_half_up">تقريب للأعلى</SelectItem><SelectItem value="round_half_down">تقريب للأسفل</SelectItem><SelectItem value="truncate">قطع الكسور</SelectItem><SelectItem value="ceiling">تقريب للسقف</SelectItem><SelectItem value="floor">تقريب للقاع</SelectItem></SelectContent></Select></div></div><DialogFooter class="flex items-center justify-end w-full pt-2"><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isSaving || !form.name.trim()}>{#if isSaving}<ArrowPath size={12} class="animate-spin me-1" />{/if}{editingId ? "تحديث" : "إنشاء"}</Button></DialogFooter></DialogContent></Dialog>
