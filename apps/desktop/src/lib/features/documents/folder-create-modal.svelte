<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Folder, Sparkles, AcademicCap, Plus, Search, Check } from "@orbis/icons";
import { api, type FolderDto, type CreateFolderRequest, type CreateCurriculumFolderRequest, type StudentDetailDto } from "$lib/api";
import { createQuery } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui";

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
        toast.error(m.operations_ui_folder_name_required());
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
      toast.success(m.operations_ui_folder_create_success({ name: created.name }));
      oncreated(created);
      handleClose();
    } else if (activeTab === "curriculum") {
      if (!curriculumName.trim()) {
        toast.error(m.operations_ui_curriculum_name_required());
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
      toast.success(m.operations_ui_curriculum_folder_create_success({ name: created.name }));
      oncreated(created);
      handleClose();
    } else if (activeTab === "student") {
      if (!selectedStudent) {
        toast.error(m.operations_ui_student_required());
        return;
      }
      const sId = selectedStudent.student.id;
      const sName = selectedStudent.person.full_name || selectedStudent.person.full_name_ar || selectedStudent.student.student_number || m.operations_ui_student_fallback();
      const created = await api.getOrCreateStudentFolder(sId, sName);
      toast.success(m.operations_ui_student_folder_open_success({ name: created.name }));
      oncreated(created);
      handleClose();
    }
  } catch (err: any) {
    console.error("Failed to create folder:", err);
    toast.error(err?.message || m.operations_ui_folder_create_error());
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open onOpenChange={(value) => !value && resetForm()}>
  <DialogContent class="max-w-md gap-5 p-5 text-start sm:p-6">
    <DialogHeader class="gap-1">
      <DialogTitle class="flex items-center gap-2.5 text-base">
        <span class="flex size-9 items-center justify-center rounded-lg bg-muted text-foreground">
          {#if activeTab === "curriculum"}
            <Sparkles class="size-4" />
          {:else if activeTab === "student"}
            <AcademicCap class="size-4" />
          {:else}
            <Folder class="size-4" />
          {/if}
        </span>
        {#if activeTab === "curriculum"}
          {m.operations_ui_curriculum_folder_create_title()}
        {:else if activeTab === "student"}
          {m.operations_ui_student_folder_create_title()}
        {:else}
          {m.operations_ui_folder_create_title()}
        {/if}
      </DialogTitle>
      <DialogDescription class="text-xs leading-relaxed">
        {m.operations_ui_folder_create_description()}
      </DialogDescription>
    </DialogHeader>

    <div class="flex items-center gap-1 rounded-lg border border-border/60 bg-muted/30 p-1 text-xs">
        <button
          type="button"
          onclick={() => activeTab = "standard"}
          aria-pressed={activeTab === "standard"}
          class="flex-1 rounded-md px-2 py-2 transition-colors flex items-center justify-center gap-1.5 {activeTab === 'standard' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          <Folder class="size-3.5" />
          <span>{m.operations_ui_general_folder()}</span>
        </button>

        <button
          type="button"
          onclick={() => activeTab = "curriculum"}
          aria-pressed={activeTab === "curriculum"}
          class="flex-1 rounded-md px-2 py-2 transition-colors flex items-center justify-center gap-1.5 {activeTab === 'curriculum' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          <Sparkles class="size-3.5" />
          <span>{m.operations_ui_ai_curriculum_folder()}</span>
        </button>

        <button
          type="button"
          onclick={() => activeTab = "student"}
          aria-pressed={activeTab === "student"}
          class="flex-1 rounded-md px-2 py-2 transition-colors flex items-center justify-center gap-1.5 {activeTab === 'student' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          <AcademicCap class="size-3.5" />
          <span>{m.operations_ui_student_folder()}</span>
        </button>
    </div>

    <div class="space-y-4">
        {#if activeTab === "standard"}
          <div class="space-y-2">
            <label for="standard-folder-name" class="text-xs font-medium text-foreground">{m.operations_ui_folder_name()}</label>
            <Input
              id="standard-folder-name"
              bind:value={folderName}
              placeholder={m.operations_ui_folder_name_example()}
              class="h-10 w-full text-sm"
              autofocus
              onkeydown={(e) => e.key === "Enter" && handleCreate()}
            />
          </div>
        {:else if activeTab === "curriculum"}
          <div class="space-y-3">
            <div class="flex items-start gap-2 rounded-lg border border-border/60 bg-muted/30 p-3 text-xs text-muted-foreground">
              <Sparkles class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
              <div>
                <p class="font-medium text-foreground">{m.operations_ui_ai_integration()}</p>
                <p class="mt-0.5 text-[11px] leading-relaxed">
                  {m.operations_ui_ai_folder_indexing_description()}
                </p>
              </div>
            </div>

            <div class="space-y-1.5">
              <label for="curriculum-name" class="text-xs font-medium text-foreground">{m.operations_ui_curriculum_or_course_name()}</label>
              <Input
                id="curriculum-name"
                bind:value={curriculumName}
                placeholder={m.operations_ui_curriculum_name_example()}
                class="h-10 w-full text-sm"
                autofocus
              />
            </div>

            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1.5">
                <label for="curriculum-subject" class="text-xs font-medium text-foreground">{m.operations_ui_subject_optional()}</label>
                <Input
                  id="curriculum-subject"
                  bind:value={curriculumSubject}
                  placeholder={m.operations_ui_subject_example()}
                  class="w-full text-xs"
                />
              </div>

              <div class="space-y-1.5">
                <label for="curriculum-grade" class="text-xs font-medium text-foreground">{m.operations_ui_grade_optional()}</label>
                <Input
                  id="curriculum-grade"
                  bind:value={curriculumGrade}
                  placeholder={m.operations_ui_grade_example()}
                  class="w-full text-xs"
                />
              </div>
            </div>
          </div>
        {:else if activeTab === "student"}
          <div class="space-y-3">
            <div class="space-y-1.5">
              <label for="student-search-input" class="text-xs font-medium text-foreground">{m.operations_ui_search_student()}</label>
              <div class="relative">
                <Search class="absolute inset-s-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground pointer-events-none" />
                <Input
                  id="student-search-input"
                  bind:value={studentSearch}
                  placeholder={m.operations_ui_student_search_placeholder()}
                  class="h-10 w-full ps-9 text-sm"
                  autofocus
                />
              </div>
            </div>

            {#if studentList.length > 0}
              <div class="border border-border/60 rounded-xl overflow-hidden divide-y divide-border/40 max-h-48 overflow-y-auto">
                {#each studentList as s (s.student.id)}
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
              <p class="text-center text-xs text-muted-foreground py-3">{m.operations_ui_no_matching_results()}</p>
            {/if}
          </div>
        {/if}
      </div>

    <DialogFooter class="gap-2 border-t border-border/60 pt-4">
      <Button
        variant="outline"
        size="sm"
        onclick={handleClose}
        disabled={isSubmitting}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        size="sm"
        onclick={handleCreate}
        disabled={isSubmitting || (activeTab === "student" && !selectedStudent)}
        class="gap-1.5"
      >
        {#if isSubmitting}
          <LoadingIndicator size={14} class="size-3.5" />
          <span>{m.operations_ui_processing()}</span>
        {:else}
          <Plus class="size-4" />
          <span>{activeTab === "student" ? m.operations_ui_open_student_folder() : m.operations_ui_folder_create_title()}</span>
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
