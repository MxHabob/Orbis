<!--
@component StudentInspector
Renders a slide-over/inspector side panel with details and actions for the selected student.
-->
<script lang="ts">
import { studentsStore } from "$lib/stores/students.svelte";
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { Input } from "@orbis/ui-svelte/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  X,
  ExternalLink,
  RefreshCw,
  LogOut,
  Phone,
  Mail,
  MapPin,
  Calendar,
  CreditCard,
  Award,
  UserMinus,
  ShieldCheck,
  Edit2,
} from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";

let { studentId, onClose }: { studentId: string; onClose: () => void } =
  $props();

// Local reactivity for student data and stats
const student = $derived(studentsStore.list.find((s) => s.id === studentId));
const currentClass = $derived(
  student ? studentsStore.classes.find((c) => c.id === student.class_id) : null,
);

let balanceDue = $state<string>("جاري التحميل...");
let gradeSummary = $state<string>("جاري التحميل...");
let isProcessing = $state(false);

// Actions dropdowns toggles
let isTransferring = $state(false);
let isWithdrawing = $state(false);
let selectedNewClassId = $state("");
let withdrawReason = $state("");

const selectedClassName = $derived(
  studentsStore.classes.find((c) => String(c.id) === selectedNewClassId)?.name ||
    studentsStore.classes.find((c) => String(c.id) === selectedNewClassId)?.name_ar ||
    "اختر الصف الجديد",
);


// Fetch student stats when studentId changes
$effect(() => {
  if (studentId) {
    fetchStats();
  }
});

async function fetchStats() {
  balanceDue = "جاري التحميل...";
  gradeSummary = "جاري التحميل...";
  try {
    const orgId = authStore.orgId;
    if (!orgId) return;

    // Fetch finances
    const fees = await commands.finance.getStudentFees(orgId, studentId);
    const payments = await commands.finance.getPayments(orgId, studentId);

    const totalFees = fees.reduce((sum, f) => sum + Number(f.amount || 0), 0);
    const totalPaid = payments.reduce(
      (sum, p) => sum + Number(p.amount || 0),
      0,
    );
    const diff = totalFees - totalPaid;
    balanceDue = diff <= 0 ? "لا توجد مستحقات" : `${diff.toLocaleString()} ر.س`;

    // Fetch grades report card summary
    const report = await commands.grades.getReportCard(studentId);
    if (report && Array.isArray(report.grades)) {
      const avg =
        report.grades.reduce(
          (sum: number, g: any) => sum + Number(g.score || 0),
          0,
        ) / (report.grades.length || 1);
      gradeSummary = `${avg.toFixed(1)}%`;
    } else {
      gradeSummary = "لا توجد درجات";
    }
  } catch (e) {
    balanceDue = "خطأ في التحميل";
    gradeSummary = "لا توجد درجات";
  }
}

async function handleTransfer() {
  if (!selectedNewClassId) return;
  isProcessing = true;
  try {
    await studentsStore.transfer(studentId, selectedNewClassId);
    toast.success("تم نقل الطالب بنجاح");
    isTransferring = false;
  } catch (e) {
    toast.error(
      "فشل نقل الطالب: " + (e instanceof Error ? e.message : String(e)),
    );
  } finally {
    isProcessing = false;
  }
}

async function handleWithdraw() {
  if (!withdrawReason) {
    toast.error("يرجى إدخال سبب الانسحاب");
    return;
  }
  isProcessing = true;
  try {
    await studentsStore.withdraw(studentId, withdrawReason);
    toast.success("تم تسجيل انسحاب الطالب بنجاح");
    isWithdrawing = false;
  } catch (e) {
    toast.error(
      "فشل تسجيل الانسحاب: " + (e instanceof Error ? e.message : String(e)),
    );
  } finally {
    isProcessing = false;
  }
}
</script>

{#if student}
	<div class="h-full flex flex-col overflow-hidden text-xs">
		<!-- Header -->
		<div class="flex items-center justify-between p-4 border-b border-border bg-card">
			<div class="flex items-center gap-2">
				<Button
					href="/{authStore.orgId}/students/{studentId}/grades"
					variant="ghost"
					size="icon"
					class="h-7 w-7 text-muted-foreground hover:text-foreground"
					title="عرض الملف الكامل"
				>
					<ExternalLink size={14} />
				</Button>
				<Button
					href="/{authStore.orgId}/students/{studentId}/edit"
					variant="ghost"
					size="icon"
					class="h-7 w-7 text-muted-foreground hover:text-foreground"
					title="تعديل سجل الطالب"
				>
					<Edit2 size={13} />
				</Button>
				<span class="font-bold text-muted-foreground uppercase text-[10px] ms-1">مفتش الطالب</span>
			</div>
			
			<Button
				variant="ghost"
				size="icon"
				class="h-7 w-7 text-muted-foreground hover:text-foreground"
				onclick={onClose}
			>
				<X size={14} />
			</Button>
		</div>

		<!-- Main Profile Section -->
		<div class="flex-1 overflow-y-auto p-5 flex flex-col gap-6 scrollbar-hide select-none">
			<div class="flex flex-col items-center text-center gap-3">
				<Avatar class="h-16 w-16 ring-2 ring-primary/10 glow-primary">
					<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
					<AvatarFallback class="bg-primary/5 text-primary text-lg font-bold">
						{String(student.full_name || "ط").slice(0, 1)}
					</AvatarFallback>
				</Avatar>
				<div>
					<h3 class="text-sm font-bold text-foreground">{student.full_name || student.name_ar || ''}</h3>
					{#if student.name_en}
						<p class="text-[10px] text-muted-foreground font-mono mt-0.5">{student.name_en}</p>
					{/if}
				</div>
				<div>
					{#if String(student.status).toLowerCase() === 'active'}
						<span class="badge-brand-lime py-0.5 px-2.5 text-[9px] font-semibold">قيد دراسي نشط</span>
					{:else if String(student.status).toLowerCase() === 'withdrawn'}
						<span class="badge-brand-destructive py-0.5 px-2.5 text-[9px] font-semibold">منسحب</span>
					{:else}
						<span class="badge-brand-default py-0.5 px-2.5 text-[9px] font-semibold">{student.status}</span>
					{/if}
				</div>
			</div>

			<div class="h-px bg-border"></div>

			<!-- Quick Indicators Cards -->
			<div class="grid grid-cols-2 gap-2 text-center">
				<div class="bg-card/50 border border-border/80 p-3 rounded-lg flex flex-col items-center justify-center">
					<Award size={14} class="text-primary mb-1" />
					<span class="text-[9px] text-muted-foreground">معدل الدرجات</span>
					<span class="font-bold text-foreground mt-0.5">{gradeSummary}</span>
				</div>
				<div class="bg-card/50 border border-border/80 p-3 rounded-lg flex flex-col items-center justify-center">
					<CreditCard size={14} class="text-emerald-500 mb-1" />
					<span class="text-[9px] text-muted-foreground">الرصيد المستحق</span>
					<span class="font-bold text-foreground mt-0.5">{balanceDue}</span>
				</div>
			</div>

			<!-- Dynamic Actions Box -->
			<div class="flex flex-col gap-2">
				<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider px-1">العمليات السريعة</span>
				
				<!-- Transfer Class Action -->
				{#if isTransferring}
					<div class="bg-card border border-border p-3 rounded-lg flex flex-col gap-2">
						<span class="font-semibold text-[10px]">اختر الصف الجديد</span>
						<Select bind:value={selectedNewClassId} type="single" disabled={isProcessing}>
							<SelectTrigger class="w-full h-8 text-xs text-right flex items-center justify-between border border-input/60" id="classSelect">
								<span>{selectedClassName}</span>
							</SelectTrigger>
							<SelectContent class="text-right" align="end">
								{#each studentsStore.classes.filter(c => c.id !== student.class_id) as cls (cls.id)}
									<SelectItem value={String(cls.id)} class="text-xs text-right justify-start">
										{cls.name || cls.name_ar || cls.id}
									</SelectItem>
								{/each}
							</SelectContent>
						</Select>
						<div class="flex gap-2 justify-end mt-1">
							<Button variant="ghost" size="sm" class="h-7 text-[10px]" onclick={() => isTransferring = false}>إلغاء</Button>
							<Button variant="default" size="sm" class="h-7 btn-tactile text-[10px]" onclick={handleTransfer} disabled={isProcessing}>تأكيد النقل</Button>
						</div>
					</div>
				{:else if isWithdrawing}
					<div class="bg-card border border-border p-3 rounded-lg flex flex-col gap-2">
						<span class="font-semibold text-[10px]">أدخل سبب الانسحاب</span>
						<Input
							type="text"
							placeholder="سبب الانسحاب..."
							class="input-sunken text-xs py-1.5"
							bind:value={withdrawReason}
							disabled={isProcessing}
						/>
						<div class="flex gap-2 justify-end mt-1">
							<Button variant="ghost" size="sm" class="h-7 text-[10px]" onclick={() => isWithdrawing = false}>إلغاء</Button>
							<Button variant="destructive" size="sm" class="h-7 btn-tactile-destructive text-[10px]" onclick={handleWithdraw} disabled={isProcessing}>تأكيد الانسحاب</Button>
						</div>
					</div>
				{:else}
					<div class="grid grid-cols-2 gap-2">
						<Button
							variant="ghost"
							class="btn-tactile text-center justify-center"
							onclick={() => isTransferring = true}
							disabled={String(student.status).toLowerCase() === 'withdrawn'}
						>
							<RefreshCw size={12} class="me-1.5" />
							نقل فصل دراسي
						</Button>
						<Button
							variant="ghost"
							class="btn-tactile-destructive text-center justify-center"
							onclick={() => isWithdrawing = true}
							disabled={String(student.status).toLowerCase() === 'withdrawn'}
						>
							<UserMinus size={12} class="me-1.5" />
							تسجيل انسحاب
						</Button>
					</div>
				{/if}
			</div>

			<!-- Static Personal Details -->
			<div class="flex flex-col gap-3">
				<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider px-1">تفاصيل الاتصال</span>
				
				<div class="bg-card/30 border border-border/60 rounded-lg p-3 flex flex-col gap-2.5">
					<div class="flex items-center gap-2">
						<Phone size={12} class="text-muted-foreground" />
						<span class="font-mono text-foreground">{student.phone_number || 'لا يوجد هاتف'}</span>
					</div>
					<div class="flex items-center gap-2">
						<Mail size={12} class="text-muted-foreground" />
						<span class="font-mono text-foreground">{student.email || 'لا يوجد بريد'}</span>
					</div>
					{#if student.address}
						<div class="flex items-center gap-2">
							<MapPin size={12} class="text-muted-foreground" />
							<span>{student.address}</span>
						</div>
					{/if}
					<div class="flex items-center gap-2">
						<Calendar size={12} class="text-muted-foreground" />
						<span>سجل في: {student.created_at ? new Date(String(student.created_at)).toLocaleDateString('ar-SA') : '-'}</span>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
