<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Folder, Sparkles, AcademicCap, Plus, XMark, Search, Check } from "@orbis/icons";
import { api, type FolderDto, type CreateFolderRequest, type CreateCurriculumFolderRequest, type StudentDetailDto } from "$lib/api";
import { createQuery } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui/toast";

let {
  open = $bindable(false),
  parentId = null,
  academicYearId = null,
  termId = null,
  oncreated,
}: {
  open: boolean;
  parentId?: string | null;
  academicYearId?: string | null;
  termId?: string | null;
  oncreated: (folder: FolderDto) => void;
} = $props();

type FolderTab = "standard" | "curriculum" | "student";
let activeTab = $state<FolderTab>("standard");

// Standard Folder State
let folderName = $state("");
let isSubmitting = $state(false);

// Curriculum Folder State
let curriculumName = $state("");
let curriculumGrade = $state("");
let curriculumSubject = $state("");

// Student Search State
let studentSearch = $state("");
let selectedStudent = $state<StudentDetailDto | null>(null);

const studentSearchResults = createQuery(() => ({
  queryKey: ["folder-modal-student-search", studentSearch.trim()],
  queryFn: () => api.listStudents({ search: studentSearch.trim(), pageSize: 5 }),
  enabled: activeTab === "student" && studentSearch.trim().length >= 2,
  staleTime: 30 * 1000,
}));

const studentList = $derived(studentSearchResults.data?.items || []);

function resetForm() {
  folderName = "";
  curriculumName = "";
  curriculumGrade = "";
  curriculumSubject = "";
  studentSearch = "";
  selectedStudent = null;
  isSubmitting = false;
}

function handleClose() {
  open = false;
  resetForm();
}

async function handleCreate() {
  if (isSubmitting) return;

  try {
    isSubmitting = true;

    if (activeTab === "standard") {
      if (!folderName.trim()) {
        toast.error("يرجى إدخال اسم المجلد");
        return;
      }
      const req: CreateFolderRequest = {
        name: folderName.trim(),
        parent_id: parentId ?? undefined,
        folder_type: "custom",
        academic_year_id: academicYearId ?? undefined,
        term_id: termId ?? undefined,
      };
      const created = await api.createFolder(req);
      toast.success(`تم إنشاء المجلد "${created.name}" بنجاح`);
      oncreated(created);
      handleClose();
    } else if (activeTab === "curriculum") {
      if (!curriculumName.trim()) {
        toast.error("يرجى إدخال اسم المنهج الدراسي");
        return;
      }
      const req: CreateCurriculumFolderRequest = {
        name: curriculumName.trim(),
        academic_year_id: academicYearId ?? undefined,
        term_id: termId ?? undefined,
        metadata: {
          grade: curriculumGrade.trim() || undefined,
          subject: curriculumSubject.trim() || undefined,
          knowledge_base: true,
          ai_agent_accessible: true,
        },
      };
      const created = await api.getOrCreateCurriculumFolder(req);
      toast.success(`تم إنشاء مجلد المنهج "${created.name}" وتهيئته للذكاء الاصطناعي`);
      oncreated(created);
      handleClose();
    } else if (activeTab === "student") {
      if (!selectedStudent) {
        toast.error("يرجى اختيار الطالب");
        return;
      }
      const sId = selectedStudent.student.id;
      const sName = selectedStudent.person.full_name || selectedStudent.person.full_name_ar || selectedStudent.student.student_number || "طالب";
      const created = await api.getOrCreateStudentFolder(sId, sName);
      toast.success(`تم فتح مجلد الطالب "${created.name}"`);
      oncreated(created);
      handleClose();
    }
  } catch (err: any) {
    console.error("Failed to create folder:", err);
    toast.error(err?.message || "فشل في إنشاء المجلد");
  } finally {
    isSubmitting = false;
  }
}
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/80 backdrop-blur-sm animate-in fade-in duration-150">
    <div
      class="relative w-full max-w-lg rounded-2xl border border-border bg-card p-6 shadow-2xl animate-in zoom-in-95 duration-150 text-start"
      role="dialog"
      aria-modal="true"
    >
      <!-- Close Button -->
      <button
        type="button"
        onclick={handleClose}
        class="absolute top-4 left-4 p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 transition-colors"
      >
        <XMark class="w-4 h-4" />
      </button>

      <!-- Modal Header -->
      <div class="mb-5">
        <h3 class="text-lg font-bold text-foreground flex items-center gap-2">
          {#if activeTab === "curriculum"}
            <Sparkles class="w-5 h-5 text-emerald-500" />
            <span>إنشاء مجلد منهج دراسي (قاعدة معرفية للذكاء الاصطناعي)</span>
          {:else if activeTab === "student"}
            <AcademicCap class="w-5 h-5 text-indigo-500" />
            <span>فتح / إنشاء مجلد مخصص لطالب</span>
          {:else}
            <Folder class="w-5 h-5 text-amber-500" />
            <span>إنشاء مجلد جديد</span>
          {/if}
        </h3>
        <p class="text-xs text-muted-foreground mt-1">
          قم بتنظيم الملفات والمناهج لتسهيل البحث وتمكين المساعد الذكي من قراءتها.
        </p>
      </div>

      <!-- Tab Switcher -->
      <div class="flex items-center gap-1.5 p-1 rounded-xl bg-muted/60 mb-5 text-xs font-medium">
        <button
          type="button"
          onclick={() => activeTab = "standard"}
          class="flex-1 py-1.5 rounded-lg transition-all flex items-center justify-center gap-1.5 {activeTab === 'standard' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
        >
          <Folder class="w-3.5 h-3.5 text-amber-500" />
          <span>مجلد عام</span>
        </button>

        <button
          type="button"
          onclick={() => activeTab = "curriculum"}
          class="flex-1 py-1.5 rounded-lg transition-all flex items-center justify-center gap-1.5 {activeTab === 'curriculum' ? 'bg-background text-emerald-600 dark:text-emerald-400 font-semibold shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
        >
          <Sparkles class="w-3.5 h-3.5 text-emerald-500" />
          <span>منهج دراسي AI</span>
        </button>

        <button
          type="button"
          onclick={() => activeTab = "student"}
          class="flex-1 py-1.5 rounded-lg transition-all flex items-center justify-center gap-1.5 {activeTab === 'student' ? 'bg-background text-indigo-600 dark:text-indigo-400 font-semibold shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
        >
          <AcademicCap class="w-3.5 h-3.5 text-indigo-500" />
          <span>مجلد طالب</span>
        </button>
      </div>

      <!-- Form Contents based on Tab -->
      <div class="space-y-4">
        {#if activeTab === "standard"}
          <div class="space-y-1.5">
            <label for="standard-folder-name" class="text-xs font-medium text-foreground">اسم المجلد</label>
            <Input
              id="standard-folder-name"
              bind:value={folderName}
              placeholder="مثال: التقارير الإدارية، الشهادات، الفعاليات..."
              class="w-full text-xs"
              autofocus
              onkeydown={(e) => e.key === "Enter" && handleCreate()}
            />
          </div>
        {:else if activeTab === "curriculum"}
          <div class="space-y-3">
            <div class="p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-xs text-emerald-800 dark:text-emerald-300 flex items-start gap-2">
              <Sparkles class="w-4 h-4 shrink-0 mt-0.5" />
              <div>
                <p class="font-semibold">تكامل الذكاء الاصطناعي:</p>
                <p class="text-[11px] opacity-90 mt-0.5">
                  الملفات المرفوعة داخل هذا المجلد سيتم فهرستها تلقائياً كمعرفة أساسية لوكيل الذكاء الاصطناعي للإجابة وتلخيص المناهج بدقة تامة.
                </p>
              </div>
            </div>

            <div class="space-y-1.5">
              <label for="curriculum-name" class="text-xs font-medium text-foreground">اسم المنهج أو المقرر</label>
              <Input
                id="curriculum-name"
                bind:value={curriculumName}
                placeholder="مثال: الرياضيات - الصف العاشر - الفصل الأول"
                class="w-full text-xs"
                autofocus
              />
            </div>

            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1.5">
                <label for="curriculum-subject" class="text-xs font-medium text-foreground">المادة (اختياري)</label>
                <Input
                  id="curriculum-subject"
                  bind:value={curriculumSubject}
                  placeholder="مثال: الرياضيات"
                  class="w-full text-xs"
                />
              </div>

              <div class="space-y-1.5">
                <label for="curriculum-grade" class="text-xs font-medium text-foreground">الصف الدراسي (اختياري)</label>
                <Input
                  id="curriculum-grade"
                  bind:value={curriculumGrade}
                  placeholder="مثال: الصف العاشر"
                  class="w-full text-xs"
                />
              </div>
            </div>
          </div>
        {:else if activeTab === "student"}
          <div class="space-y-3">
            <div class="space-y-1.5">
              <label for="student-search-input" class="text-xs font-medium text-foreground">ابحث عن الطالب</label>
              <div class="relative">
                <Search class="w-4 h-4 absolute right-3 top-2.5 text-muted-foreground pointer-events-none" />
                <Input
                  id="student-search-input"
                  bind:value={studentSearch}
                  placeholder="اكتب اسم الطالب أو الرقم الأكاديمي..."
                  class="w-full pr-9 text-xs"
                  autofocus
                />
              </div>
            </div>

            {#if studentList.length > 0}
              <div class="border border-border/60 rounded-xl overflow-hidden divide-y divide-border/40 max-h-48 overflow-y-auto">
                {#each studentList as s}
                  {@const isSelected = selectedStudent?.student?.id === s.student.id}
                  <button
                    type="button"
                    onclick={() => selectedStudent = s}
                    class="w-full p-2.5 text-start text-xs flex items-center justify-between transition-colors {isSelected ? 'bg-primary/10 text-primary font-medium' : 'hover:bg-muted/50 text-foreground'}"
                  >
                    <div class="truncate">
                      <p class="font-medium truncate">{s.person.full_name || s.person.full_name_ar}</p>
                      <p class="text-[10px] text-muted-foreground font-mono mt-0.5">#{s.student.student_number || s.student.id.slice(0, 8)}</p>
                    </div>
                    {#if isSelected}
                      <Check class="w-4 h-4 text-primary shrink-0" />
                    {/if}
                  </button>
                {/each}
              </div>
            {:else if studentSearch.trim().length >= 2 && !studentSearchResults.isLoading}
              <p class="text-center text-xs text-muted-foreground py-3">لا توجد نتائج مطابقة</p>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer Buttons -->
      <div class="flex items-center justify-end gap-2 mt-6 pt-4 border-t border-border/40">
        <Button variant="ghost" size="sm" onclick={handleClose} disabled={isSubmitting}>
          إلغاء
        </Button>
        <Button
          size="sm"
          onclick={handleCreate}
          disabled={isSubmitting || (activeTab === "student" && !selectedStudent)}
          class="gap-1.5"
        >
          {#if isSubmitting}
            <span class="w-3.5 h-3.5 border-2 border-current border-t-transparent rounded-full animate-spin"></span>
            <span>جاري المعالجة...</span>
          {:else}
            <Plus class="w-4 h-4" />
            <span>{activeTab === "student" ? "فتح مجلد الطالب" : "إنشاء المجلد"}</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}
