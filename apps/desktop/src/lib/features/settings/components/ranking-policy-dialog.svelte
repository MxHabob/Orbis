<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { AdjustmentsHorizontal } from "@orbis/icons";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
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
  onsubmit,
}: {
  open: boolean;
  form: any;
  editingId?: string | null;
  isSaving?: boolean;
  onsubmit: () => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-md"><DialogHeader><DialogTitle>{editingId ? "تعديل سياسة الترتيب" : "إضافة سياسة ترتيب جديدة"}</DialogTitle></DialogHeader><div class="space-y-4 py-2 text-xs"><div class="space-y-1.5"><Label>اسم السياسة</Label><Input bind:value={form.name} placeholder="مثال: الترتيب العام للصفوف" /></div><div class="space-y-1.5"><Label>نطاق الترتيب (Scope)</Label><Select type="single" value={form.ranking_scope} onValueChange={(v) => { if (v) form.ranking_scope = v; }}><SelectTrigger class="w-full">{form.ranking_scope === "class" ? "على مستوى الصف (Class)" : form.ranking_scope === "level" ? "على مستوى المرحلة (Level)" : "على مستوى المؤسسة (Org)"}</SelectTrigger><SelectContent><SelectItem value="class">على مستوى الصف (Class)</SelectItem><SelectItem value="division">على مستوى الشعبة (Division)</SelectItem><SelectItem value="level">على مستوى المرحلة (Level)</SelectItem><SelectItem value="org">على مستوى المؤسسة بالكامل (Org)</SelectItem></SelectContent></Select></div><div class="space-y-1.5"><Label>معيار الترتيب (Metric)</Label><Select type="single" value={form.ranking_metric} onValueChange={(v) => { if (v) form.ranking_metric = v; }}><SelectTrigger class="w-full">{form.ranking_metric === "total_score" ? "المجموع الكلي (Total Score)" : form.ranking_metric === "percentage" ? "النسبة المئوية (Percentage)" : "المعدل التراكمي (GPA)"}</SelectTrigger><SelectContent><SelectItem value="total_score">المجموع الكلي (Total Score)</SelectItem><SelectItem value="percentage">النسبة المئوية (Percentage)</SelectItem><SelectItem value="gpa">المعدل التراكمي (GPA)</SelectItem></SelectContent></Select></div><div class="space-y-1.5"><Label>طريقة التعامل مع التعادل (Tie Handling)</Label><Select type="single" value={form.tie_handling} onValueChange={(v) => { if (v) form.tie_handling = v; }}><SelectTrigger class="w-full">{form.tie_handling === "same_rank" ? "نفس الرتبة (Same Rank: 1, 2, 2, 4)" : form.tie_handling === "sequential" ? "ترتيب تسلسلي (Sequential: 1, 2, 3, 4)" : "تخطي الرتبة (Skip)"}</SelectTrigger><SelectContent><SelectItem value="same_rank">نفس الرتبة (Same Rank: 1, 2, 2, 4)</SelectItem><SelectItem value="sequential">ترتيب تسلسلي (Sequential: 1, 2, 3, 4)</SelectItem><SelectItem value="skip">تخطي الرتبة (Skip)</SelectItem></SelectContent></Select></div><div class="flex items-center justify-between pt-2"><Label>تفعيل السياسة</Label><Switch checked={form.is_enabled === 1} onCheckedChange={(c) => (form.is_enabled = c ? 1 : 0)} /></div><div class="flex items-center justify-between"><Label>شمل الطلاب الراسبين في الترتيب</Label><Switch checked={form.include_failed_students === 1} onCheckedChange={(c) => (form.include_failed_students = c ? 1 : 0)} /></div></div><DialogFooter><Button variant="outline" onclick={() => (open = false)}>إلغاء</Button><Button onclick={onsubmit} disabled={isSaving || !form.name.trim()}>{#if isSaving}<Loader2 class="h-4 w-4 animate-spin mr-1" />{/if}حفظ</Button></DialogFooter></DialogContent></Dialog>
