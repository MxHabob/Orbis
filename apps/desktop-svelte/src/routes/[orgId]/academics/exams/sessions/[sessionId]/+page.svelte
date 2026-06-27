<!--
@component ExamSessionDetail
Session detail page with tabs: Subjects, Schedule, Seating, Halls, Proctors.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { toast } from "svelte-sonner";
import { localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  ArrowLeft,
  BookOpen,
  Calendar,
  Hash,
  Building2,
  UserCheck,
  Plus,
  Zap,
  Lock,
  Shuffle,
  Play,
  CheckCircle2,
  Pencil,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import ExamStatusBadge from "$lib/components/exams/status-badge.svelte";

let { data }: { data: { orgId: string } } = $props();

// Extract sessionId from route params
const sessionId = $derived(page.params.sessionId);

// State
let session = $state<any>(null);
let subjects = $state<any[]>([]);
let schedules = $state<any[]>([]);
let seatingNumbers = $state<any[]>([]);
let halls = $state<any[]>([]);
let proctors = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state("subjects");

const tabs = $derived([
  { id: "subjects", label: m.exams_detail_subjects(), icon: BookOpen, count: subjects.length },
  { id: "schedule", label: m.exams_detail_schedule(), icon: Calendar, count: schedules.length },
  { id: "seating", label: m.exams_detail_seating(), icon: Hash, count: seatingNumbers.length },
  { id: "halls", label: m.exams_detail_halls(), icon: Building2, count: halls.length },
  { id: "proctors", label: m.exams_detail_proctors(), icon: UserCheck, count: proctors.length },
]);

async function loadSession() {
  isLoading = true;
  try {
    // TODO: Wire to IPC commands
    session = null;
    subjects = [];
    schedules = [];
    seatingNumbers = [];
    halls = [];
    proctors = [];
  } catch (e) {
    toast.error("Failed to load session details");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId && sessionId) {
    loadSession();
  }
});

// Quick actions
async function handleGenerateSeating() {
  toast.info(m.exams_actions_generate_seating());
  // TODO: Wire to API
}

async function handleGenerateSecrets() {
  toast.info(m.exams_actions_generate_secrets());
}

async function handleAutoSchedule() {
  toast.info(m.exams_actions_auto_schedule());
}

async function handleAllocateHalls() {
  toast.info(m.exams_actions_allocate_halls());
}

async function handleActivate() {
  toast.info(m.exams_detail_activate());
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-5 select-none">
	{#if isLoading}
		<Skeleton class="h-20 w-full rounded-xl" />
		<Skeleton class="h-10 w-full rounded-lg" />
		<Skeleton class="h-64 w-full rounded-xl" />
	{:else}
		<!-- Back + Header -->
		<div class="flex flex-col gap-3" in:fade={{ duration: 150 }}>
			<Button
				href={localizeHref(`/${data.orgId}/academics/exams/sessions`)}
				variant="ghost"
				class="w-fit h-7 px-2 text-[10px] text-muted-foreground flex items-center gap-1 cursor-pointer"
			>
				<ArrowLeft size={12} />
				<span>{m.exams_detail_back()}</span>
			</Button>

			<!-- Session Info Banner -->
			<div class="card-tactile bg-card p-5 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
				<div class="flex flex-col gap-1.5">
					<div class="flex items-center gap-2">
						<h1 class="text-sm font-bold text-foreground">
							{session?.name_ar || session?.name || `Session ${sessionId}`}
						</h1>
						<ExamStatusBadge status={session?.status || "draft"} />
					</div>
					{#if session}
						<div class="flex items-center gap-3 text-[10px] text-muted-foreground">
							<span class="flex items-center gap-1">
								<Calendar size={10} />
								{session.start_date} → {session.end_date}
							</span>
							<span class="px-1.5 py-0.5 rounded bg-muted/60 border border-border/40 text-[9px]">
								{session.session_type}
							</span>
						</div>
					{/if}
				</div>
				<div class="flex items-center gap-2">
					<Button
						variant="outline"
						class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
						onclick={() => toast.info("Edit sheet coming soon")}
					>
						<Pencil size={10} />
						{m.exams_detail_edit()}
					</Button>
					<Button
						variant="default"
						class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
						onclick={handleActivate}
					>
						<Play size={10} />
						{m.exams_detail_activate()}
					</Button>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="flex items-center gap-1 border-b border-border pb-0" in:fade={{ delay: 50, duration: 200 }}>
			{#each tabs as tab (tab.id)}
				<button
					class="px-3 py-2 text-[10px] font-medium transition-all duration-150 border-b-2 cursor-pointer flex items-center gap-1.5
						{activeTab === tab.id
							? 'border-primary text-primary font-semibold'
							: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border'}"
					onclick={() => activeTab = tab.id}
				>
					<tab.icon size={12} />
					<span>{tab.label}</span>
					{#if tab.count > 0}
						<span class="text-[8px] px-1 py-0.5 rounded bg-muted/60 font-mono">{tab.count}</span>
					{/if}
				</button>
			{/each}
		</div>

		<!-- Tab Content -->
		<div class="flex-1" in:fade={{ delay: 100, duration: 200 }}>

			<!-- ═══ Subjects Tab ═══ -->
			{#if activeTab === "subjects"}
				<div class="flex flex-col gap-4">
					{#if subjects.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<BookOpen size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم إضافة مواد بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => toast.info("Add subject coming soon")}>
								<Plus size={12} class="me-1" />
								{m.exams_subjects_add()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10">#</TableHead>
										<TableHead>{m.exams_subjects_name()}</TableHead>
										<TableHead class="w-20">{m.exams_subjects_score()}</TableHead>
										<TableHead class="w-24">{m.exams_subjects_duration()}</TableHead>
										<TableHead class="w-24">{m.exams_subjects_type()}</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each subjects as subject, i (subject.id)}
										<TableRow class="text-xs">
											<TableCell class="font-mono text-muted-foreground">{i + 1}</TableCell>
											<TableCell class="font-medium">{subject.subject_id}</TableCell>
											<TableCell class="font-mono">{subject.max_score}</TableCell>
											<TableCell class="font-mono">{subject.duration_minutes}د</TableCell>
											<TableCell>
												<span class="px-1.5 py-0.5 rounded bg-muted/60 border border-border/40 text-[9px]">
													{subject.exam_type}
												</span>
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
						<Button variant="outline" class="w-fit h-7 px-3 text-[10px] cursor-pointer" onclick={() => toast.info("Add subject coming soon")}>
							<Plus size={12} class="me-1" />
							{m.exams_subjects_add()}
						</Button>
					{/if}
				</div>

			<!-- ═══ Schedule Tab ═══ -->
			{:else if activeTab === "schedule"}
				<div class="flex flex-col gap-4">
					{#if schedules.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Calendar size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم جدولة امتحانات بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAutoSchedule}>
								<Zap size={12} class="me-1" />
								{m.exams_actions_auto_schedule()}
							</Button>
						</div>
					{:else}
						<!-- Timeline Grid -->
						<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
							{#each schedules as sched (sched.id)}
								<div class="card-tactile bg-card p-4 flex flex-col gap-2">
									<div class="flex items-center justify-between">
										<span class="text-[10px] font-bold text-foreground">{sched.exam_date}</span>
										<span class="text-[9px] px-1.5 py-0.5 rounded {sched.period === 'morning' ? 'bg-sky-500/10 text-sky-600' : 'bg-violet-500/10 text-violet-600'}">
											{sched.period === 'morning' ? '☀️ صباحي' : '🌙 مسائي'}
										</span>
									</div>
									<div class="text-xs font-medium text-foreground">{sched.exam_subject_id}</div>
									<div class="text-[9px] text-muted-foreground font-mono">
										{sched.start_time} → {sched.end_time}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			<!-- ═══ Seating Tab ═══ -->
			{:else if activeTab === "seating"}
				<div class="flex flex-col gap-4">
					{#if seatingNumbers.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Hash size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم إصدار أرقام الجلوس بعد</p>
							<Button variant="default" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleGenerateSeating}>
								<Hash size={12} class="me-1" />
								{m.exams_actions_generate_seating()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead class="w-10">#</TableHead>
										<TableHead>رقم الجلوس</TableHead>
										<TableHead>الطالب</TableHead>
										<TableHead>الفصل</TableHead>
										<TableHead>المستوى</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each seatingNumbers as sn, i (sn.id)}
										<TableRow class="text-xs">
											<TableCell class="font-mono text-muted-foreground">{i + 1}</TableCell>
											<TableCell class="font-mono font-bold text-primary">{sn.seating_number}</TableCell>
											<TableCell>{sn.student_name_ar || sn.enrollment_id}</TableCell>
											<TableCell>{sn.class_id}</TableCell>
											<TableCell>{sn.level}</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
					{/if}
				</div>

			<!-- ═══ Halls Tab ═══ -->
			{:else if activeTab === "halls"}
				<div class="flex flex-col gap-4">
					{#if halls.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<Building2 size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم تحديد لجان بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAllocateHalls}>
								<Shuffle size={12} class="me-1" />
								{m.exams_actions_allocate_halls()}
							</Button>
						</div>
					{:else}
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
							{#each halls as hall (hall.id)}
								{@const utilPct = hall.capacity > 0 ? Math.round((hall.allocated_count / hall.capacity) * 100) : 0}
								<div class="card-tactile bg-card p-4 flex flex-col gap-3">
									<div class="flex items-center justify-between">
										<span class="text-xs font-bold text-foreground">{hall.hall_number}</span>
										<span class="text-[9px] font-mono px-1.5 py-0.5 rounded bg-muted/60 border border-border/40">
											{hall.allocated_count}/{hall.capacity}
										</span>
									</div>
									{#if hall.building}
										<span class="text-[9px] text-muted-foreground">{hall.building} - الطابق {hall.floor_number}</span>
									{/if}
									<!-- Utilization Bar -->
									<div class="h-1.5 w-full rounded-full bg-muted/40 overflow-hidden">
										<div
											class="h-full rounded-full transition-all duration-500 {utilPct > 90 ? 'bg-red-500' : utilPct > 70 ? 'bg-amber-500' : 'bg-emerald-500'}"
											style="width: {utilPct}%"
										></div>
									</div>
									<span class="text-[8px] text-muted-foreground">{utilPct}% ممتلئ</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			<!-- ═══ Proctors Tab ═══ -->
			{:else if activeTab === "proctors"}
				<div class="flex flex-col gap-4">
					{#if proctors.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<UserCheck size={24} class="text-muted-foreground" />
							<p class="text-xs text-muted-foreground">لم يتم تعيين مراقبين بعد</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => toast.info("Assign proctors coming soon")}>
								<UserCheck size={12} class="me-1" />
								{m.exams_actions_assign_proctors()}
							</Button>
						</div>
					{:else}
						<div class="card-tactile bg-card overflow-hidden">
							<Table>
								<TableHeader>
									<TableRow class="text-[10px]">
										<TableHead>المراقب</TableHead>
										<TableHead>اللجنة</TableHead>
										<TableHead>الدور</TableHead>
										<TableHead class="w-20">الحالة</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{#each proctors as p (p.id)}
										<TableRow class="text-xs">
											<TableCell class="font-medium">{p.teacher_id}</TableCell>
											<TableCell>{p.exam_hall_id}</TableCell>
											<TableCell>{p.role}</TableCell>
											<TableCell>
												{#if p.is_confirmed === 1}
													<CheckCircle2 size={14} class="text-emerald-500" />
												{:else}
													<span class="text-[9px] text-amber-500">قيد التأكيد</span>
												{/if}
											</TableCell>
										</TableRow>
									{/each}
								</TableBody>
							</Table>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Quick Actions Bar -->
		<div class="border-t border-border pt-4 flex flex-wrap items-center gap-2" in:fade={{ delay: 150, duration: 200 }}>
			<span class="text-[9px] text-muted-foreground font-bold uppercase tracking-wider me-2">إجراءات سريعة</span>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSeating}>
				<Hash size={10} /> أرقام الجلوس
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleGenerateSecrets}>
				<Lock size={10} /> الأرقام السرية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleAutoSchedule}>
				<Zap size={10} /> جدولة تلقائية
			</Button>
			<Button variant="outline" class="h-7 px-2.5 text-[9px] flex items-center gap-1 cursor-pointer" onclick={handleAllocateHalls}>
				<Shuffle size={10} /> توزيع اللجان
			</Button>
		</div>
	{/if}
</div>
