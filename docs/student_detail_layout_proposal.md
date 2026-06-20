# تصميم شاشات تفاصيل الطلاب بناءً على فلسفة GitButler

تعتمد شاشات **GitButler** على مفهوم **الإنتاجية العالية (High Density)** والتحكم اللامركزي في النوافذ (Resizable Panes) والاعتماد على لوحة المفاتيح والسرعة اللحظية. نقل هذه الفلسفة إلى شاشة تفاصيل الطالب (`[studentId]/+layout.svelte`) يعطينا تجربة عمل مدهشة واحترافية للغاية للإداريين والمعلمين.

---

## 1. تحليل تصميم شاشات GitButler وكيف نطبقه على الطلاب

تميز واجهات **GitButler** بثلاث ركائز أساسية:
1. **الأعمدة المتراصة (Stacked Columns / Lanes):** عدم التنقل الكامل بل فتح النوافذ بجوار بعضها البعض، مع المقدرة على تعديل قياساتها (Resizable).
2. **الكثافة العالية وتفادي المساحات الفارغة (High Density UI):** الخطوط الأنيقة والصغيرة، الأزرار اللمسية المستوية (Flat Tactile Buttons)، وتجاور البيانات المهمة بدلاً من إخفائها تحت قوائم منسدلة.
3. **التجاوب الدقيق للحواف (Tactile Sash Borders):** الحدود الفاصلة بمقدار `1px` التي تضيء تدريجياً بألوان برّاقة (Glow) عند الاقتراب منها بمؤشر الفأرة.

---

## 2. مقترح هيكل تفاصيل الطالب (`[studentId]/+layout.svelte`)

عندما يختار المستخدم طالباً أو يفتحه بكامل الشاشة، نقسم الشاشة داخلياً إلى عمودين باستخدام الهوية اللمسية لـ Orbis:

```
┌────────────────────────────────────────────────────────────────────────┐
│  ← Back to Directory  |  🎓 Ahmad Al-Otaibi (Grade 1-A)  |  [Status]   │
├──────────────────────────────────────┬─────────────────────────────────┤
│                                      │  Sub-Navigation:                │
│  [Student Passport Sidebar]          │  [ Grades ]  [ Fees ]  [ ... ]  │
│ ┌──────────────────────────────────┐ │ ├───────────────────────────────┤
│ │            [Avatar]              │ │ │                               │
│ │         Ahmad Al-Otaibi          │ │ │                               │
│ │         ID: #2026-9812           │ │ │     Active Tab Content        │
│ ├──────────────────────────────────┤ │ │                               │
│ │ • Parent: Khaled (050xxxx)       │ │ │     (Grades / Fees View)      │
│ │ • GPA: 94.2%  • Attendance: 98%  │ │ │                               │
│ │ • Unpaid balance: 1,200 SAR      │ │ │                               │
│ └──────────────────────────────────┘ │ │                               │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### تقسيم المهام (Split Pane Layout inside detail):
* **العمود الأيمن (في اللغة العربية - Passport Sidebar):**
  يكون ثابتاً على الجانب (عرض 320px). يعرض بطاقة تعريفية سريعة كـ (Passport) للطالب: الصورة الشخصية، بيانات الاتصال، معلومات ولي الأمر، إحصائيات سريعة (مثل: نسبة الغياب، المعدل التراكمي، الرصيد المستحق) بحيث تظل ظاهرة طوال تصفح المعلم للتبويبات المختلفة.
* **العمود الأيسر (Viewport Content):**
  يعرض شريط تبويبات علوي بتأثير انزلاق ناعم (Sliding Pill Bar). عند الانتقال بين (الدرجات، الرسوم، السلوك، الحضور)، يتم تحميل المحتوى المخصص داخل هذا الفضاء بشكل فوري ومحكم دون تجميد الواجهة.

---

## 3. الكود البرمجي المقترح لـ `+layout.svelte` لتفاصيل الطالب

إليك كود Svelte 5 المقترح للملف [+layout.svelte](file:///c:/Users/YZLPN3/Downloads/new/Orbis/apps/desktop-svelte/src/routes/[orgId]/students/[studentId]/+layout.svelte):

```html
<!-- apps/desktop-svelte/src/routes/[orgId]/students/[studentId]/+layout.svelte -->
<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { 
  ArrowRight, ArrowLeft, GraduationCap, 
  Wallet, UserCheck, ShieldAlert, PhoneCall 
} from "lucide-svelte";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);
const currentPath = $derived(page.url.pathname);

// بيانات وهمية مؤقتاً لمحاكاة الطالب (سيتم جلبها عبر IPC)
const student = $derived({
  id: studentId,
  name_ar: "أحمد بن خالد العتيبي",
  name_en: "Ahmad Khaled Al-Otaibi",
  avatar: null,
  class_name: "الصف الأول - أ",
  gpa: "94.8%",
  attendance: "98.2%",
  balance: "1,200 ر.س",
  guardian: "خالد العتيبي (أب)",
  guardian_phone: "+966 50 123 4567"
});

// التبويبات المتوافقة مع هيكل المجلدات
const tabs = $derived([
  { id: "grades", label: "الدرجات والتقييمات", icon: GraduationCap, href: `/${orgId}/students/${studentId}/grades` },
  { id: "fees", label: "الشؤون المالية", icon: Wallet, href: `/${orgId}/students/${studentId}/fees` },
  { id: "attendance", label: "الحضور والغياب", icon: UserCheck, href: `/${orgId}/students/${studentId}/attendance` },
  { id: "behavior", label: "السلوك والانضباط", icon: ShieldAlert, href: `/${orgId}/students/${studentId}/behavior` }
]);

function isActive(tabHref: string) {
  return currentPath === tabHref;
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden">
	<!-- شريط الأدوات العلوي / التنقل الخلفي -->
	<div class="h-14 border-b border-border flex items-center justify-between px-6 bg-card/20 shrink-0">
		<div class="flex items-center gap-4">
			<Button 
				href="/{orgId}/students" 
				variant="ghost" 
				size="icon" 
				class="h-8 w-8 hover:bg-accent"
			>
				<!-- توافق السهم مع الاتجاه العربي -->
				<ArrowRight class="h-4 w-4" />
			</Button>
			<div class="h-4 w-px bg-border"></div>
			<div class="flex items-center gap-2">
				<span class="text-sm font-semibold">{student.name_ar}</span>
				<span class="badge-brand-lime text-[10px] py-0.5 px-2">{student.class_name}</span>
			</div>
		</div>

		<!-- إجراءات سريعة فورية للاتصال أو الإرسال -->
		<div class="flex items-center gap-2">
			<Button variant="ghost" class="btn-tactile text-xs">
				<PhoneCall size={14} class="me-2" />
				الاتصال بولي الأمر
			</Button>
		</div>
	</div>

	<!-- منطقة العمل الأساسية المقسمة -->
	<div class="flex-1 flex overflow-hidden">
		<!-- العمود الجانبي: جواز سفر الطالب (Passport Sidebar) -->
		<div class="w-80 border-e border-border bg-card/30 flex flex-col p-5 gap-6 overflow-y-auto shrink-0 select-none">
			<!-- الصورة والاسم الأساسي -->
			<div class="flex flex-col items-center text-center gap-3">
				<Avatar class="h-20 w-20 ring-4 ring-primary/10 glow-primary">
					<AvatarImage src={student.avatar ?? undefined} alt={student.name_ar} />
					<AvatarFallback class="bg-primary/5 text-primary text-xl font-bold">أ</AvatarFallback>
				</Avatar>
				<div>
					<h2 class="text-base font-bold">{student.name_ar}</h2>
					<p class="text-xs text-muted-foreground font-mono mt-0.5">{student.name_en}</p>
				</div>
			</div>

			<div class="h-px bg-border"></div>

			<!-- الإحصائيات السريعة (Metrics) -->
			<div class="grid grid-cols-3 gap-2 text-center">
				<div class="bg-card border border-border p-2 rounded-lg">
					<p class="text-[10px] text-muted-foreground mb-0.5">المعدل</p>
					<p class="text-xs font-bold text-gradient-brand">{student.gpa}</p>
				</div>
				<div class="bg-card border border-border p-2 rounded-lg">
					<p class="text-[10px] text-muted-foreground mb-0.5">الحضور</p>
					<p class="text-xs font-bold text-emerald-500">{student.attendance}</p>
				</div>
				<div class="bg-card border border-border p-2 rounded-lg">
					<p class="text-[10px] text-muted-foreground mb-0.5">الرسوم</p>
					<p class="text-xs font-bold text-destructive">{student.balance}</p>
				</div>
			</div>

			<!-- البيانات الشخصية وتفاصيل الاتصال -->
			<div class="flex flex-col gap-4 text-xs">
				<div class="space-y-1">
					<span class="text-muted-foreground text-[10px]">ولي الأمر:</span>
					<p class="font-medium text-foreground">{student.guardian}</p>
				</div>
				<div class="space-y-1">
					<span class="text-muted-foreground text-[10px]">رقم هاتف ولي الأمر:</span>
					<p class="font-medium text-foreground font-mono">{student.guardian_phone}</p>
				</div>
				<div class="space-y-1">
					<span class="text-muted-foreground text-[10px]">الرقم الأكاديمي:</span>
					<p class="font-medium text-foreground font-mono">{student.id}</p>
				</div>
			</div>
		</div>

		<!-- منطقة عرض التبويبات الفعالة -->
		<div class="flex-1 flex flex-col overflow-hidden bg-background">
			<!-- شريط تبويبات انزلاقي (Sentry-style pill navbar) -->
			<div class="h-12 border-b border-border flex items-center px-6 bg-card/10 gap-1 shrink-0">
				{#each tabs as tab (tab.id)}
					<Button
						href={tab.href}
						variant={isActive(tab.href) ? 'default' : 'ghost'}
						class="text-xs font-semibold px-4 py-1.5 h-auto transition-all duration-200"
						class:btn-tactile-primary={isActive(tab.href)}
						class:btn-tactile-ghost={!isActive(tab.href)}
					>
						<tab.icon size={13} class="me-2" />
						{tab.label}
					</Button>
				{/each}
			</div>

			<!-- محتوى التبويب الفرعي الممرر -->
			<div class="flex-1 overflow-y-auto p-6">
				{@render children()}
			</div>
		</div>
	</div>
</div>
```

---

## 4. الميزات المتقدمة المحاكية لـ GitButler في شاشات الطلاب

1. **التحكم بمقاس الـ Passport Sidebar:**
   يمكننا بسهولة استخدام مكون `Resizer` الذي قمنا بتحليله لجعل عمود جواز السفر (Passport Sidebar) قابلاً للسحب لليمين واليسار مع حفظ تفضيل العرض الخاص بالمستخدم محلياً في `localStorage`.
2. **الانتقال السريع بلوحة المفاتيح (Shortcuts):**
   * الضغط على `Alt + 1` يفتح الدرجات، `Alt + 2` للمالية، `Alt + 3` للحضور، و `Alt + 4` للسلوك.
   * زر `Esc` يعود فوراً إلى دليل الطلاب الرئيسي.
3. **تحديث فوري وبأثر رجعي:**
   * عند إدخال علامة اختبار جديدة أو حضور في صفحة الحضور، يظهر وميض خفيف (Pulse / Glow Effect) حول قيم المتركس (النسب الإجمالية للمعدل والحضور) في الجانب ليعكس الحسابات الجديدة فوراً وبشكل متناسق مع التغيير الحاصل.
