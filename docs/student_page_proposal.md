# بنية وهندسة صفحة الطلاب (Students Workspace) لـ Orbis Desktop

يقدم هذا المستند مقترحاً وتصميماً متكاملاً لبناء صفحة الطلاب في تطبيق `apps/desktop-svelte` باستخدام إطار العمل **Svelte 5** ومنصة **Tauri**، متبعاً أفضل الممارسات البرمجية لعام **2026** المعتمدة لدى كبرى الشركات التقنية (مثل Linear و Vercel و GitButler).

---

## 1. فلسفة التصميم وتجربة المستخدم (UX/UI Philosophy)

بدلاً من التنقل التقليدي بين الصفحات (Full Page Navigation) الذي يُفقد المعلم أو الإداري تركيزه، سنعتمد تصميم **منطقة عمل ثلاثية الأعمدة (Split-Pane Workspace)** باستخدام مكون `MainViewport` المتاح بالفعل في المشروع.

```
┌────────────────────────────────────────────────────────────────────────┐
│                              AppHeader                                 │
├────────────────────────────────────────────────────────────────────────┤
│  Left Pane      │            Middle Pane               │  Right Pane   │
│  (Navigation)   │            (Main Grid)               │  (Inspector)  │
│ ┌─────────────┐ │ ┌──────────────────────────────────┐ │ ┌───────────┐ │
│ │ 📅 2025/26  │ │ │ 🔍 Search students...            │ │ │  Student  │ │
│ │ 🏫 Grades   │ │ ├──────────────────────────────────┤ │ │  Profile  │ │
│ │   - Grade 1 │ │ │ [x] Name     │ Class  │ Status   │ │ │ ┌───────┐ │ │
│ │   - Grade 2 │ │ │ [ ] Ahmad    │ 1-A    │ Active   │ │ │ │ Avatar  │ │ │
│ │ ⚙️ Filters   │ │ │ [ ] Fatima   │ 2-B    │ Active   │ │ │ └───────┘ │ │
│ │   - Active  │ │ │ [ ] John     │ 1-A    │ Withdr.. │ │ │ Details   │ │
│ │   - Left    │ │ └──────────────────────────────────┘ │ │ Tabs:     │ │
│ └─────────────┘ │                                      │ │ - Finance │ │
│                 │                                      │ │ - Grades  │ │
└─────────────────┴──────────────────────────────────────┴───────────────┘
```

### تقسيم الأعمدة (The 3-Pane Layout):
1. **العمود الأيسر (Left Pane - عرض 280px):**
   * اختيار العام الدراسي والدورة الحالية.
   * فلترة سريعة حسب المراحل والصفوف (Grades/Classes) مع إظهار عدد الطلاب الفوري بجانب كل صف.
   * تصفية سريعة بناءً على حالة القيد (نشط، منسحب، خريج).
2. **العمود الأوسط (Middle Pane - عرض مرن):**
   * شريط أدوات علوي (البحث المتقدم، أزرار الإجراءات السريعة كإضافة طالب أو الترقية الجماعية، وتصدير البيانات).
   * جدول بيانات ذكي (Premium Interactive Datagrid) فائق السرعة يدعم التمرير الافتراضي (Virtual Scrolling) لمعالجة آلاف السجلات.
3. **العمود الأيمن (Right Pane - عرض 380px):**
   * مفتش الطالب (Student Inspector): ينزلق أو يظهر بسلاسة عند اختيار أي طالب من الجدول.
   * يعرض نظرة عامة سريعة دون مغادرة الصفحة: البيانات الشخصية، الرسوم المالية المستحقة، السلوك، والدرجات، مع روابط للانتقال التفصيلي.

---

## 2. الهيكل الهرمي للمجلدات والملفات المقترح

تماشياً مع هيكلية SvelteKit والـ Desktop App، نقترح تنظيم الملفات كالتالي:

```text
apps/desktop-svelte/src/routes/[orgId]/students/
├── +page.svelte                      # الصفحة الرئيسية (منطقة العمل ثلاثية الأعمدة)
├── +page.ts                          # تحميل البيانات الأولي من الـ IPC
├── [studentId]/                      # مجلد التفاصيل العميقة للطلاب
│   ├── +layout.svelte                # التصميم الهيكلي لتفاصيل طالب محدد (Tabs)
│   ├── attendance/                   # صفحة غياب وحضور الطالب المحدّد
│   │   └── +page.svelte
│   ├── behavior/                     # السلوك والملاحظات الذكية
│   │   └── +page.svelte
│   ├── fees/                         # المستحقات والمدفوعات
│   │   └── +page.svelte
│   └── grades/                       # درجات الطالب والاختبارات
│       └── +page.svelte
└── new/                              # (اختياري) نموذج إضافة طالب منفصل أو نستخدم الـ Sheet
```

ولتنظيم الكود وإعادة الاستخدام، نقترح وضع المكونات الخاصة بالطلاب داخل مجلد مستقل:
```text
apps/desktop-svelte/src/lib/components/students/
├── student-table.svelte              # جدول الطلاب الذكي (Datagrid)
├── student-sidebar-filters.svelte    # الفلاتر الجانبية (Left Panel)
├── student-inspector.svelte          # مفتش تفاصيل الطالب السريعة (Right Panel)
├── student-create-sheet.svelte       # لوح منزلق لإضافة طالب جديد
└── student-promote-modal.svelte      # نافذة الترقية الجماعية للطلاب
```

---

## 3. نمط إدارة الحالة ومزامنة البيانات لعام 2026

تعتمد كبرى الشركات على مبدأ **الاستجابة الفورية للمستند (Optimistic UI)** وسرعة الاستجابة اللحظية (Sub-100ms Interactions).

### أ. استخدام Svelte 5 Runes لإدارة الحالة المحلية
نستعمل الـ Runes الجديدة (`$state` و `$derived`) لإدارة البيانات بشكل حيوي:

```typescript
// src/lib/stores/students.svelte.ts
import { commands } from "$lib/services/ipc";
import type { Enrollment } from "$lib/services/ipc";

class StudentsStore {
  // الحالة الأساسية
  list = $state<Enrollment[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);
  selectedId = $state<string | null>(null);

  // الحالة المستنبتة (Derived State) لفلترة فورية في الواجهة
  searchQuery = $state("");
  selectedClassId = $state<string | null>(null);
  statusFilter = $state<string>("active");

  filteredList = $derived.by(() => {
    return this.list.filter(student => {
      const matchSearch = this.searchQuery ? 
        (student.full_name?.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
         student.student_number?.includes(this.searchQuery)) : true;
      const matchClass = this.selectedClassId ? student.class_id === this.selectedClassId : true;
      const matchStatus = this.statusFilter === "all" ? true : student.status === this.statusFilter;
      return matchSearch && matchClass && matchStatus;
    });
  });

  // جلب البيانات من الـ Rust Backend عبر Tauri IPC
  async load(orgId: string) {
    this.isLoading = true;
    this.error = null;
    try {
      this.list = await commands.enrollments.getAll(orgId);
    } catch (e: any) {
      this.error = e.message || String(e);
    } finally {
      this.isLoading = false;
    }
  }

  // تحديث متفائل (Optimistic Update) عند نقل طالب لصف آخر
  async transferStudent(enrollmentId: string, newClassId: string) {
    // 1. التحديث المحلي الفوري
    const previousList = [...this.list];
    const index = this.list.findIndex(s => s.id === enrollmentId);
    if (index !== -1) {
      this.list[index] = { ...this.list[index], class_id: newClassId };
    }

    try {
      // 2. إرسال الأمر للخلفية
      await commands.enrollments.transfer(enrollmentId, { new_class_id: newClassId });
    } catch (err) {
      // 3. التراجع في حال الفشل (Rollback)
      this.list = previousList;
      throw err;
    }
  }
}

export const studentsStore = new StudentsStore();
```

---

## 4. تطبيق عملي لصفحة الطلاب الرئيسية `+page.svelte`

إليك كيف يمكن صياغة المكون الرئيسي للطلاب جامعاً بين الـ `MainViewport` والـ `S.C.R.A.P.S.` Design System المعتمد في المشروع مع دعم اللغتين العربية والإنجليزية:

```html
<!-- apps/desktop-svelte/src/routes/[orgId]/students/+page.svelte -->
<script lang="ts">
import { onMount } from "svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Plus, Search, FilterX, Download, ArrowUpRight } from "lucide-svelte";
import { studentsStore } from "$lib/stores/students.svelte";
import StudentSidebarFilters from "$lib/components/students/student-sidebar-filters.svelte";
import StudentTable from "$lib/components/students/student-table.svelte";
import StudentInspector from "$lib/components/students/student-inspector.svelte";
import StudentCreateSheet from "$lib/components/students/student-create-sheet.svelte";

let { data } = $props();

// التحكم بفتح لوحة إضافة طالب جديدة
let isCreateOpen = $state(false);

onMount(() => {
  studentsStore.load(data.orgId);
});
</script>

<svelte:head>
	<title>الطلاب | Orbis</title>
</svelte:head>

<div class="h-full w-full flex flex-col overflow-hidden">
	<MainViewport
		name="students"
		leftWidth={{ default: 260, min: 200 }}
		rightWidth={{ default: 380, min: 300 }}
	>
		<!-- 1. العمود الأيسر: شجرة الفلاتر والصفوف -->
		{#snippet left()}
			<div class="h-full border-r border-border bg-card/40 p-4 flex flex-col gap-4 overflow-y-auto">
				<div class="flex items-center justify-between">
					<h3 class="font-semibold text-xs tracking-wider text-muted-foreground uppercase">التصفية والصفوف</h3>
					{#if studentsStore.selectedClassId || studentsStore.statusFilter !== 'active'}
						<Button 
							variant="ghost" 
							size="icon" 
							class="h-6 w-6 text-muted-foreground" 
							onclick={() => { 
								studentsStore.selectedClassId = null; 
								studentsStore.statusFilter = 'active'; 
							}}
						>
							<FilterX size={14} />
						</Button>
					{/if}
				</div>
				<StudentSidebarFilters />
			</div>
		{/snippet}

		<!-- 2. العمود الأوسط: الجدول والبحث والإجراءات -->
		{#snippet middle()}
			<div class="h-full flex flex-col p-6 overflow-hidden gap-4">
				<!-- شريط الأدوات العلوي -->
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div>
						<h1 class="text-2xl font-bold tracking-tight text-gradient-brand">دليل الطلاب</h1>
						<p class="text-xs text-muted-foreground">إدارة شؤون الطلاب، الفصول، والترقيات الأكاديمية.</p>
					</div>

					<div class="flex items-center gap-2">
						<Button variant="ghost" class="btn-tactile text-xs" onclick={() => console.log('Exporting...')}>
							<Download size={14} class="me-2" />
							تصدير Excel
						</Button>
						<Button variant="lime" class="btn-tactile-lime" onclick={() => isCreateOpen = true}>
							<Plus size={16} class="me-2" />
							طالب جديد
						</Button>
					</div>
				</div>

				<!-- البحث السريع والفلترة السريعة -->
				<div class="relative w-full max-w-sm">
					<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
					<input
						type="text"
						placeholder="البحث بالاسم، الرقم الأكاديمي..."
						class="input-sunken ps-9 text-xs"
						bind:value={studentsStore.searchQuery}
					/>
				</div>

				<!-- جدول الطلاب -->
				<div class="flex-1 min-h-0 bg-card rounded-xl border border-border shadow-sm overflow-hidden">
					{#if studentsStore.isLoading}
						<div class="h-full w-full flex items-center justify-center">
							<div class="skeleton w-full h-full"></div>
						</div>
					{:else if studentsStore.error}
						<div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-2">
							<p class="text-sm text-destructive">{studentsStore.error}</p>
							<Button variant="ghost" onclick={() => studentsStore.load(data.orgId)}>إعادة المحاولة</Button>
						</div>
					{:else}
						<StudentTable students={studentsStore.filteredList} />
					{/if}
				</div>
			</div>
		{#/snippet}

		<!-- 3. العمود الأيمن: مفتش الطالب الذكي (يظهر فقط عند الاختيار) -->
		{#snippet right()}
			{#if studentsStore.selectedId}
				<div class="h-full border-l border-border bg-card/60 flex flex-col overflow-hidden">
					<StudentInspector 
						studentId={studentsStore.selectedId} 
						onClose={() => studentsStore.selectedId = null} 
					/>
				</div>
			{:else}
				<div class="h-full flex items-center justify-center text-center p-6 text-muted-foreground bg-card/10">
					<div class="flex flex-col items-center gap-2">
						<ArrowUpRight size={24} class="text-muted-foreground/40 animate-pulse" />
						<p class="text-xs">اختر طالباً من الجدول لمعاينة ملفه السريع وإجراء العمليات الفورية.</p>
					</div>
				</div>
			{/if}
		{/snippet}
	</MainViewport>

	<!-- اللوح المنزلق لإضافة طالب جديد -->
	<StudentCreateSheet bind:open={isCreateOpen} orgId={data.orgId} />
</div>
```

---

## 5. ميزات متقدمة لعام 2026 (Enterprise Patterns)

لجعل تجربة الاستخدام تضاهي المنتجات الفاخرة للشركات التقنية العملاقة، يجب تضمين ما يلي:

### أ. لوحة تحكم سريعة (Command Palette / Command Menu)
* تفعيل اختصار لوحة المفاتيح `Ctrl + K` أو `Cmd + K` لفتح لوحة الأوامر السريعة.
* تمكين المعلم من كتابة "نقل أحمد" أو "ترقية الصف الأول" لتوجيهه مباشرة للعملية أو تنفيذها فوراً.

### ب. جدول بيانات شبيه بـ Excel (Excel Grid Workspace)
* بدلاً من النقر على "تعديل" ثم فتح صفحة جديدة، يتمكن المعلم من النقر المزدوج على حقل الهاتف أو الصف لتعديله فوراً داخل الجدول مع حفظ تلقائي (Auto-save) ومزامنة بالخلفية.
* دعم التنقل بالأسهم (`ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`) للانتقال الفوري بين الخلايا.

### ج. الذكاء الاصطناعي التوليدي المدمج (Copilot Summary)
* استدعاء أمر الـ AI المتاح بالخلفية `behavior.submitAI` لتلخيص سلوك الطالب، إما لإنشاء تقرير فوري للوالدين أو لتقديم توصيات للمعلمين في مجلس الانضباط.

### د. دعم الاتجاهين والتوطين الكامل (i18n / RTL Support)
* الاستفادة من `@inlang/paraglide-js` المدمج بالمشروع لدعم التبديل السلس بين اللغة العربية والإنجليزية وتغيير اتجاه الصفحة بالكامل دون أي خلل في التصميم التفاعلي.

---

### خطة العمل المقترحة للتطبيق (Next Steps)
1. **إنشاء الـ Store**: كتابة ملف `src/lib/stores/students.svelte.ts` للتحكم بالحالة ومزامنتها مع الـ Rust IPC.
2. **بناء المكونات الفرعية**: تطوير `student-table.svelte` و `student-inspector.svelte` بالاعتماد على مكتبة `@orbis/ui-svelte`.
3. **تطبيق الصفحة الرئيسية**: تفعيل ملف `src/routes/[orgId]/students/+page.svelte` وربطه بالـ Layouts الحالية.
