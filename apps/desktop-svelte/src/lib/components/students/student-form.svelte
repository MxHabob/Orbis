<!--
@component StudentForm
Redesigned student registration form matching the fields and layout of the registration paper form.
Excludes the top paper header, conditions checklist, and signature footer.
Uses native ui-svelte design components.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { studentsStore, authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import {
  ArrowRight,
  Save,
  User,
  FileText,
  Sparkles,
  AlertCircle,
  CheckSquare,
  Square,
  Printer,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

const isEditMode = $derived(!!studentId);

// Core Student fields
let nameAr = $state("");
let nameEn = $state("");
let birthDate = $state("");
let sex = $state("male"); // male / female
let classId = $state<any>("");
let studentNumber = $state("");
let email = $state("");
let phone = $state("");
let address = $state("");

// Date picker options and reactive values
const currentYear = new Date().getFullYear();
const yearsList = Array.from({ length: 47 }, (_, i) => currentYear - i);
const daysList = Array.from({ length: 31 }, (_, i) => String(i + 1));
const monthsList = $derived([
  { value: "1", label: m.form_month_1() },
  { value: "2", label: m.form_month_2() },
  { value: "3", label: m.form_month_3() },
  { value: "4", label: m.form_month_4() },
  { value: "5", label: m.form_month_5() },
  { value: "6", label: m.form_month_6() },
  { value: "7", label: m.form_month_7() },
  { value: "8", label: m.form_month_8() },
  { value: "9", label: m.form_month_9() },
  { value: "10", label: m.form_month_10() },
  { value: "11", label: m.form_month_11() },
  { value: "12", label: m.form_month_12() },
]);

let birthDay = $state("");
let birthMonth = $state("");
let birthYear = $state("");

$effect(() => {
  if (birthDay && birthMonth && birthYear) {
    birthDate = `${birthYear}-${birthMonth.padStart(2, "0")}-${birthDay.padStart(2, "0")}`;
  } else {
    birthDate = "";
  }
});

// Parent & Location details
let motherName = $state("");
let guardianName = $state(""); // Father's Name
let guardianPhone = $state(""); // Father's Phone
let motherPhone = $state("");
let fatherJob = $state("");
let motherJob = $state("");
let previousSchool = $state("");
let district = $state("");
let governorate = $state("");

// Derived selected class name for UI Select component
const selectedClassName = $derived(
  studentsStore.classes.find((c) => String(c.id) === classId)?.name ||
    studentsStore.classes.find((c) => String(c.id) === classId)?.name_ar ||
    m.form_select_class(),
);

// Document checklist
const docTypes = $derived([
  { id: "birth_certificate", label: m.form_doc_birth_certificate() },
  { id: "national_id", label: m.form_doc_national_id() },
  { id: "grade_1", label: m.form_doc_grade_1() },
  { id: "grade_2", label: m.form_doc_grade_2() },
  { id: "grade_3", label: m.form_doc_grade_3() },
  { id: "grade_4", label: m.form_doc_grade_4() },
  { id: "grade_5", label: m.form_doc_grade_5() },
  { id: "grade_6", label: m.form_doc_grade_6() },
  { id: "grade_7", label: m.form_doc_grade_7() },
  { id: "grade_8", label: m.form_doc_grade_8() },
  { id: "grade_9", label: m.form_doc_grade_9() },
]);
let submittedDocs = $state<string[]>([]);

let isSaving = $state(false);

let orgName = $state("");

onMount(async () => {
  if (studentsStore.classes.length === 0) {
    await studentsStore.load(orgId);
  }

  try {
    const org = await commands.users.getOrganization(orgId);
    orgName = org.name || "";
  } catch (e) {
    console.error("Failed to load organization name:", e);
  }

  if (isEditMode && studentId) {
    await loadStudentData();
  }
});

async function loadStudentData() {
  try {
    const student = studentsStore.list.find((s) => s.id === studentId);
    if (student) {
      nameAr = String(student.full_name || student.name_ar || "");
      nameEn = String(student.name_en || "");
      birthDate = student.birth_date
        ? String(student.birth_date).slice(0, 10)
        : "";
      if (birthDate) {
        const parts = birthDate.split("-");
        if (parts.length === 3) {
          birthYear = parts[0];
          birthMonth = String(parseInt(parts[1], 10));
          birthDay = String(parseInt(parts[2], 10));
        }
      }
      sex = String(student.sex || "male");
      classId = String(student.class_id || "");
      studentNumber = String(student.student_number || "");
      email = String(student.email || "");
      phone = String(student.phone_number || "");
      address = String(student.address || "");
    }

    // Load guardian & extra fields
    const guardianLinks = await commands.guardians.getByStudent(studentId!);
    if (guardianLinks && guardianLinks.length > 0) {
      const link = guardianLinks[0];
      const guardianUser = await commands.users.getUser(
        String(link.guardian_id),
      );
      if (guardianUser) {
        guardianName = String(guardianUser.full_name || "");
        guardianPhone = String(guardianUser.phone || "");
      }

      fatherJob = String(link.occupation || "");

      if (link.notes) {
        try {
          const parsed = JSON.parse(String(link.notes));
          motherName = parsed.mother_name || "";
          motherPhone = parsed.mother_phone || "";
          motherJob = parsed.mother_job || "";
          previousSchool = parsed.previous_school || "";
          district = parsed.district || "";
          governorate = parsed.governorate || "";
          submittedDocs = parsed.documents || [];
        } catch {
          // Standard notes string
        }
      }
    }
  } catch (e) {
    console.error("Error loading student edit data:", e);
    toast.error(m.form_error_load());
  }
}

function toggleDoc(docId: string) {
  if (submittedDocs.includes(docId)) {
    submittedDocs = submittedDocs.filter((d) => d !== docId);
  } else {
    submittedDocs = [...submittedDocs, docId];
  }
}

async function handleSave() {
  if (!nameAr.trim()) {
    toast.error(m.form_error_name_required());
    return;
  }
  if (!classId) {
    toast.error(m.form_error_class_required());
    return;
  }
  if (!birthDate) {
    toast.error(m.form_error_birth_required());
    return;
  }

  isSaving = true;
  try {
    const createdBy = authStore.user?.id || "";

    // Data package to update the student user record
    const studentUserData = {
      full_name: nameAr.trim(),
      name_ar: nameAr.trim(),
      name_en: nameEn.trim() || null,
      email: email.trim() || null,
      phone: phone.trim() || null,
      sex: sex,
      date_of_birth: birthDate,
      address: address.trim() || null,
      status: "active",
    };

    let targetStudentId = ""; // User ID
    let targetEnrollmentId = studentId || "";

    if (isEditMode) {
      const originalStudent = studentsStore.list.find(
        (s) => s.id === studentId,
      );
      if (!originalStudent) {
        throw new Error(m.form_error_enrollment_not_found());
      }
      targetStudentId = String(originalStudent.student_id);

      // Update student user details
      await commands.users.updateUser(targetStudentId, studentUserData);

      // If class has changed, invoke transfer command
      if (originalStudent.class_id !== classId) {
        await commands.enrollments.transfer(studentId!, {
          new_class_id: classId,
        });
      }

      toast.success(m.form_success_update());
    } else {
      // Create new student user first
      const tempEmail = email.trim() || `stu.${Date.now()}@orbis.local`;
      const userRequest = {
        email: tempEmail,
        password: "Orbis123456", // Default temporary password
        full_name: nameAr.trim(),
        name_ar: nameAr.trim(),
        primary_role: "student",
        sex: sex,
        phone: phone.trim() || null,
        national_id: null,
        date_of_birth: birthDate,
      };

      const studentUser = await commands.users.createUser(
        orgId,
        createdBy,
        userRequest,
      );
      targetStudentId = studentUser.id;

      // Update extra fields not supported in CreateUserRequest directly
      await commands.users.updateUser(studentUser.id, {
        name_en: nameEn.trim() || null,
        address: address.trim() || null,
      });

      // Create new student enrollment
      const enrollmentRequest = {
        student_id: studentUser.id,
        class_id: classId,
        enrollment_type: "regular",
      };

      const res = await commands.enrollments.create(orgId, enrollmentRequest);
      targetEnrollmentId = String(res.id);

      // Merge details for optimistic UI updates in store list
      const mergedEnrollment = {
        ...res,
        student_id: studentUser.id,
        full_name: studentUser.full_name,
        name_ar: studentUser.name_ar,
        name_en: studentUser.name_en,
        email: studentUser.email,
        phone_number: studentUser.phone,
        avatar: studentUser.avatar,
        birth_date: studentUser.date_of_birth,
        address: studentUser.address,
      };

      studentsStore.list = [mergedEnrollment, ...studentsStore.list];
      toast.success(m.form_success_create());
    }

    // Save/Update Father & Mother details (via Guardian link)
    if (guardianName.trim() && targetStudentId) {
      const notesMetadata = JSON.stringify({
        mother_name: motherName.trim(),
        mother_phone: motherPhone.trim(),
        mother_job: motherJob.trim(),
        previous_school: previousSchool.trim(),
        district: district.trim(),
        governorate: governorate.trim(),
        documents: submittedDocs,
      });

      // 1. Create/Get Guardian User for Father
      const guardianLinks = isEditMode
        ? await commands.guardians.getByStudent(targetEnrollmentId)
        : [];

      if (guardianLinks && guardianLinks.length > 0) {
        const link = guardianLinks[0];
        // Update father user details
        await commands.users.updateUser(String(link.guardian_id), {
          full_name: guardianName.trim(),
          name_ar: guardianName.trim(),
          phone: guardianPhone.trim() || null,
        });

        // Update link
        await commands.guardians.updateLink(String(link.id), {
          relationship: "father",
          occupation: fatherJob.trim() || null,
          notes: notesMetadata,
        });
      } else {
        // Create new Father/Guardian User
        const guardEmail = `father.${Date.now()}@orbis.local`;
        const guardianUser = await commands.users.createUser(orgId, createdBy, {
          email: guardEmail,
          password: "Orbis123456",
          full_name: guardianName.trim(),
          name_ar: guardianName.trim(),
          primary_role: "parent",
          phone: guardianPhone.trim() || null,
          sex: "male",
          national_id: null,
          date_of_birth: null,
        });

        // Create link
        await commands.guardians.createLink({
          student_id: targetStudentId,
          guardian_id: guardianUser.id,
          relationship: "father",
          relationship_ar: "أب",
          is_primary: 1,
          is_emergency_contact: 1,
          occupation: fatherJob.trim() || null,
          notes: notesMetadata,
        });
      }
    }

    // Navigate back to directory
    goto(localizeHref(`/${orgId}/students`));
  } catch (e: any) {
    console.error("Save error:", e);
    toast.error(m.form_error_save({ error: e.message || String(e) }));
  } finally {
    isSaving = false;
  }
}

let isDownloadingForm = $state(false);

async function handleDownloadRegistrationForm() {
  isDownloadingForm = true;
  try {
    const templateRecord = await commands.documentTemplates.getTemplate(orgId, "t5-student-registration-form");
    if (!templateRecord) {
      toast.error("Template 't5-student-registration-form' not found in database");
      return;
    }
    const documentModel = JSON.parse(templateRecord.document_json);

    const pdfBytes = await commands.docforge.previewDocumentTemplate(
      JSON.stringify(documentModel),
      JSON.stringify({ orgName })
    );

    if (pdfBytes && pdfBytes.length > 0) {
      let binary = "";
      const len = pdfBytes.length;
      for (let i = 0; i < len; i++) {
        binary += String.fromCharCode(pdfBytes[i]);
      }
      const base64 = window.btoa(binary);
      const pdfUrl = `data:application/pdf;base64,${base64}`;

      const link = document.createElement("a");
      link.href = pdfUrl;
      link.download = `استمارة_تسجيل_طالب.pdf`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      
      toast.success("تم توليد وتحميل استمارة التسجيل بنجاح");
    } else {
      toast.error("Failed to generate PDF document");
    }
  } catch (e: any) {
    console.error("Failed to download registration form:", e);
    const msg = e?.message || String(e);
    toast.error(`خطأ أثناء تحميل الاستمارة: ${msg}`);
  } finally {
    isDownloadingForm = false;
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background text-start" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <!-- Form Body Scroll Container -->
  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <div class="w-full space-y-8 pb-6">
      
      <!-- Section 1: معلومات الطالب/ة -->
      <div class="bg-card/40 border border-border/80 rounded-xl p-6 space-y-6">
        <div class="flex items-center gap-2 border-b border-border pb-3">
          <User class="text-primary h-4.5 w-4.5" />
          <h2 class="text-sm font-bold text-foreground">{m.form_student_info()}</h2>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-5">
          <div class="flex flex-col gap-1.5 sm:col-span-2 md:col-span-3 lg:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="nameAr">{m.form_full_name_ar()} <span class="text-destructive">{m.form_required()}</span></Label>
            <Input 
              id="nameAr"
              type="text" 
              placeholder={m.form_full_name_ar_placeholder()} 
              class="font-bold text-xs" 
              bind:value={nameAr}
              autocomplete="off"
            />
          </div>

          <!-- الاسم بالإنجليزية -->
          <div class="flex flex-col gap-1.5 sm:col-span-2 md:col-span-3 lg:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="nameEn">{m.form_full_name_en()}</Label>
            <Input 
              id="nameEn"
              type="text" 
              placeholder={m.form_full_name_en_placeholder()} 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={nameEn}
              autocomplete="off"
            />
          </div>

          <!-- تاريخ الميلاد -->
          <div class="flex flex-col gap-1.5 sm:col-span-2 lg:col-span-1">
            <Label class="text-[10px] font-semibold text-muted-foreground">{m.form_birth_date()} <span class="text-destructive">{m.form_required()}</span></Label>
            <div class="grid grid-cols-3 gap-1.5" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
              <!-- اليوم -->
              <Select bind:value={birthDay} type="single">
                <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input/60 px-2" id="birthDaySelect">
                  <span class="truncate">{birthDay || m.form_day()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each daysList as dy}
                    <SelectItem value={dy} class="text-xs text-start justify-start">
                      {dy}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <!-- الشهر -->
              <Select bind:value={birthMonth} type="single">
                <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input/60 px-2" id="birthMonthSelect">
                  <span class="truncate">{monthsList.find(m => m.value === birthMonth)?.label || m.form_month()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each monthsList as mo}
                    <SelectItem value={mo.value} class="text-xs text-start justify-start">
                      {mo.label}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <!-- السنة -->
              <Select bind:value={birthYear} type="single">
                <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input/60 px-2" id="birthYearSelect">
                  <span class="truncate">{birthYear || m.form_year()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each yearsList as yr}
                    <SelectItem value={String(yr)} class="text-xs text-start justify-start">
                      {yr}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- الجنس -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">{m.form_sex()} <span class="text-destructive">{m.form_required()}</span></Label>
            <div class="flex gap-2 items-center h-8">
              <Button
                variant="ghost"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'male' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground'}"
                onclick={() => sex = 'male'}
              >
                <span>{m.form_male()}</span>
              </Button>
              <Button 
                variant="ghost"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'female' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground'}"
                onclick={() => sex = 'female'}
              >
                <span>{m.form_female()}</span>
              </Button>
            </div>
          </div>

          <!-- الصف الدراسي المطلوب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="classSelect">{m.form_target_class()} <span class="text-destructive">{m.form_required()}</span></Label>
            <Select bind:value={classId} type="single">
              <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input/60" id="classSelect">
                <span>{selectedClassName}</span>
              </SelectTrigger>
              <SelectContent class="text-start" align="end">
                {#each studentsStore.classes as cls (cls.id)}
                  <SelectItem value={String(cls.id)} class="text-xs text-start justify-start">
                    {cls.name || cls.name_ar}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <!-- الرقم الأكاديمي -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="studentNumber">{m.form_student_number()}</Label>
            <Input 
              id="studentNumber"
              type="text" 
              placeholder={m.form_student_number_placeholder()} 
              class="text-xs" 
              bind:value={studentNumber}
            />
          </div>

          <!-- اسم الأب / ولي الأمر -->
          <div class="flex flex-col gap-1.5 sm:col-span-2 md:col-span-2 lg:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="guardianName">{m.form_father_name()}</Label>
            <Input 
              id="guardianName"
              type="text" 
              placeholder={m.form_father_name_placeholder()} 
              class="text-xs font-semibold" 
              bind:value={guardianName}
            />
          </div>

          <!-- رقم هاتف الأب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="guardianPhone">{m.form_father_phone()}</Label>
            <Input 
              id="guardianPhone"
              type="tel" 
              placeholder={m.form_phone_placeholder()} 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={guardianPhone}
            />
          </div>

          <!-- عمل الأب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="fatherJob">{m.form_father_job()}</Label>
            <Input 
              id="fatherJob"
              type="text" 
              placeholder={m.form_job_placeholder()} 
              class="text-xs" 
              bind:value={fatherJob}
            />
          </div>

          <!-- اسم الأم -->
          <div class="flex flex-col gap-1.5 sm:col-span-2 md:col-span-2 lg:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherName">{m.form_mother_name()}</Label>
            <Input 
              id="motherName"
              type="text" 
              placeholder={m.form_mother_name_placeholder()} 
              class="text-xs" 
              bind:value={motherName}
            />
          </div>

          <!-- رقم هاتف الأم -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherPhone">{m.form_mother_phone()}</Label>
            <Input 
              id="motherPhone"
              type="tel" 
              placeholder={m.form_phone_placeholder()} 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={motherPhone}
            />
          </div>

          <!-- عمل الأم -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherJob">{m.form_mother_job()}</Label>
            <Input 
              id="motherJob"
              type="text" 
              placeholder={m.form_job_placeholder()} 
              class="text-xs" 
              bind:value={motherJob}
            />
          </div>

          <!-- المدرسة السابقة -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="previousSchool">{m.form_previous_school()}</Label>
            <Input 
              id="previousSchool"
              type="text" 
              placeholder={m.form_previous_school_placeholder()} 
              class="text-xs" 
              bind:value={previousSchool}
            />
          </div>

          <!-- المديرية -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="district">{m.form_district()}</Label>
            <Input 
              id="district"
              type="text" 
              placeholder={m.form_district_placeholder()} 
              class="text-xs" 
              bind:value={district}
            />
          </div>

          <!-- المحافظة -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="governorate">{m.form_governorate()}</Label>
            <Input 
              id="governorate"
              type="text" 
              placeholder={m.form_governorate_placeholder()} 
              class="text-xs" 
              bind:value={governorate}
            />
          </div>

          <!-- هاتف الطالب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="phone">{m.form_student_phone()}</Label>
            <Input 
              id="phone"
              type="tel" 
              placeholder={m.form_student_phone_placeholder()} 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={phone}
            />
          </div>

          <!-- البريد الإلكتروني -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="email">{m.form_student_email()}</Label>
            <Input 
              id="email"
              type="email" 
              placeholder="student@example.com" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={email}
            />
          </div>

          <!-- السكن / العنوان الحالي -->
          <div class="flex flex-col gap-1.5 sm:col-span-2 md:col-span-2 lg:col-span-3">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="address">{m.form_address()}</Label>
            <Input 
              id="address"
              type="text" 
              placeholder={m.form_address_placeholder()} 
              class="text-xs" 
              bind:value={address}
            />
          </div>
        </div>
      </div>

      <!-- Section 2: بيانات الوثائق -->
      <div class="bg-card/40 border border-border/80 rounded-xl p-6 space-y-6">
        <div class="flex items-center gap-2 border-b border-border pb-3">
          <FileText class="text-primary h-4.5 w-4.5" />
          <h2 class="text-sm font-bold text-foreground">{m.form_documents_title()}</h2>
        </div>

        <p class="text-[10px] text-muted-foreground">{m.form_documents_hint()}</p>

        <!-- Dynamic Grid Checklist matching the paper form layout -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
          {#each docTypes as doc (doc.id)}
            {@const isChecked = submittedDocs.includes(doc.id)}
            <Button
              variant="ghost"
              class="flex items-center justify-between p-3 h-auto border rounded-lg cursor-pointer text-start text-xs transition-all duration-150 {isChecked ? 'border-primary bg-primary/5 text-foreground' : 'border-border bg-card/25 text-muted-foreground'}"
              onclick={() => toggleDoc(doc.id)}
            >
              <span class="font-medium">{doc.label}</span>
              <span class="text-primary flex items-center justify-center">
                {#if isChecked}
                  <CheckSquare size={16} />
                {:else}
                  <Square size={16} class="opacity-50" />
                {/if}
              </span>
            </Button>
          {/each}
        </div>
      </div>
      
    </div>
  </div>

  <!-- Sticky/Fixed Footer Action Buttons Row at the bottom of the page -->
  <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-border bg-card/10 shrink-0 bg-background/95 backdrop-blur-xs">
    <Button 
      variant="outline" 
      class="text-xs h-9 px-5 gap-1.5 border-primary/40 text-primary hover:bg-primary/5 hover:text-primary cursor-pointer" 
      onclick={handleDownloadRegistrationForm}
      disabled={isSaving || isDownloadingForm}
    >
      <Printer size={14} class={isDownloadingForm ? 'animate-spin' : ''} />
      {isDownloadingForm ? m.form_printing_registration_form() : m.form_print_registration_form()}
    </Button>
    <Button 
      variant="outline" 
      class="text-xs h-9 px-5 cursor-pointer" 
      onclick={() => goto(localizeHref(`/${orgId}/students`))}
      disabled={isSaving}
    >
      {m.form_cancel()}
    </Button>
    <Button 
      variant="lime" 
      class="text-xs h-9 px-6 gap-1.5" 
      onclick={handleSave}
      disabled={isSaving}
    >
      <Save size={14} />
      {isSaving ? m.form_saving() : m.form_save()}
    </Button>
  </div>
</div>

