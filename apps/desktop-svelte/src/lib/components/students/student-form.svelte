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
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
import { studentsStore, authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { 
  ArrowRight, Save, User, FileText, Sparkles, AlertCircle, CheckSquare, Square
} from "lucide-svelte";

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
  studentsStore.classes.find(c => String(c.id) === classId)?.name || 
  studentsStore.classes.find(c => String(c.id) === classId)?.name_ar || 
  "اختر الصف الدراسي"
);

// Document checklist
const docTypes = [
  { id: "birth_certificate", label: "شهادة الميلاد" },
  { id: "national_id", label: "البطاقة الشخصية" },
  { id: "grade_1", label: "شهادة أول" },
  { id: "grade_2", label: "شهادة ثاني" },
  { id: "grade_3", label: "شهادة ثالث" },
  { id: "grade_4", label: "شهادة رابع" },
  { id: "grade_5", label: "شهادة خامس" },
  { id: "grade_6", label: "شهادة سادس" },
  { id: "grade_7", label: "شهادة سابع" },
  { id: "grade_8", label: "شهادة ثامن" },
  { id: "grade_9", label: "شهادة تاسع" }
];
let submittedDocs = $state<string[]>([]);

let isSaving = $state(false);

onMount(async () => {
  if (studentsStore.classes.length === 0) {
    await studentsStore.load(orgId);
  }

  if (isEditMode && studentId) {
    await loadStudentData();
  }
});

async function loadStudentData() {
  try {
    const student = studentsStore.list.find(s => s.id === studentId);
    if (student) {
      nameAr = String(student.full_name || student.name_ar || "");
      nameEn = String(student.name_en || "");
      birthDate = student.birth_date ? String(student.birth_date).slice(0, 10) : "";
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
      const guardianUser = await commands.users.getUser(String(link.guardian_id));
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
    toast.error("حدث خطأ أثناء تحميل بيانات الطالب للتعديل");
  }
}

function toggleDoc(docId: string) {
  if (submittedDocs.includes(docId)) {
    submittedDocs = submittedDocs.filter(d => d !== docId);
  } else {
    submittedDocs = [...submittedDocs, docId];
  }
}

async function handleSave() {
  if (!nameAr.trim()) {
    toast.error("يرجى إدخال اسم الطالب الكامل بالعربية");
    return;
  }
  if (!classId) {
    toast.error("يرجى اختيار الصف الدراسي");
    return;
  }
  if (!birthDate) {
    toast.error("يرجى إدخال تاريخ ميلاد الطالب");
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
      status: "active"
    };

    let targetStudentId = ""; // User ID
    let targetEnrollmentId = studentId || "";

    if (isEditMode) {
      const originalStudent = studentsStore.list.find(s => s.id === studentId);
      if (!originalStudent) {
        throw new Error("سجل قيد الطالب غير موجود");
      }
      targetStudentId = String(originalStudent.student_id);

      // Update student user details
      await commands.users.updateUser(targetStudentId, studentUserData);
      
      // If class has changed, invoke transfer command
      if (originalStudent.class_id !== classId) {
        await commands.enrollments.transfer(studentId!, { new_class_id: classId });
      }

      toast.success("تم تحديث بيانات الطالب بنجاح");
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
        date_of_birth: birthDate
      };

      const studentUser = await commands.users.createUser(orgId, createdBy, userRequest);
      targetStudentId = studentUser.id;

      // Update extra fields not supported in CreateUserRequest directly
      await commands.users.updateUser(studentUser.id, {
        name_en: nameEn.trim() || null,
        address: address.trim() || null
      });

      // Create new student enrollment
      const enrollmentRequest = {
        student_id: studentUser.id,
        class_id: classId,
        enrollment_type: "regular"
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
        address: studentUser.address
      };
      
      studentsStore.list = [mergedEnrollment, ...studentsStore.list];
      toast.success("تم تسجيل الطالب الجديد بنجاح");
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
        documents: submittedDocs
      });

      // 1. Create/Get Guardian User for Father
      const guardianLinks = isEditMode ? await commands.guardians.getByStudent(targetEnrollmentId) : [];
      
      if (guardianLinks && guardianLinks.length > 0) {
        const link = guardianLinks[0];
        // Update father user details
        await commands.users.updateUser(String(link.guardian_id), {
          full_name: guardianName.trim(),
          name_ar: guardianName.trim(),
          phone: guardianPhone.trim() || null
        });

        // Update link
        await commands.guardians.updateLink(String(link.id), {
          relationship: "father",
          occupation: fatherJob.trim() || null,
          notes: notesMetadata
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
          date_of_birth: null
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
          notes: notesMetadata
        });
      }
    }

    // Navigate back to directory
    goto(`/${orgId}/students`);
  } catch (e: any) {
    console.error("Save error:", e);
    toast.error("فشل الحفظ: " + (e.message || String(e)));
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background text-right" dir="rtl">
  <!-- Form Body Scroll Container -->
  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    <div class="w-full space-y-8 pb-12">
      
      <!-- Section 1: معلومات الطالب/ة -->
      <div class="bg-card/40 border border-border/80 rounded-xl p-6 space-y-6">
        <div class="flex items-center gap-2 border-b border-border pb-3">
          <User class="text-primary h-4.5 w-4.5" />
          <h2 class="text-sm font-bold text-foreground">معلومات الطالب/ة</h2>
        </div>

        <!-- Alert Note -->
        <div class="flex items-start gap-2.5 bg-primary/5 border border-primary/10 rounded-lg p-3 text-[11px] text-muted-foreground leading-relaxed">
          <AlertCircle class="text-primary h-4 w-4 shrink-0 mt-0.5" />
          <p>يرجى الانتباه على اسم الطالب وتاريخ الميلاد الصحيح للطالب لكي لا توجد عقبات في المستقبل.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
          <!-- الاسم الكامل -->
          <div class="flex flex-col gap-1.5 md:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="nameAr">الاسم الكامل لطالب/ة <span class="text-destructive">*</span></Label>
            <Input 
              id="nameAr"
              type="text" 
              placeholder="الاسم الثلاثي أو الرباعي كما هو مسجل في الهوية" 
              class="font-bold text-xs" 
              bind:value={nameAr}
            />
          </div>

          <!-- الاسم بالإنجليزية -->
          <div class="flex flex-col gap-1.5 md:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="nameEn">الاسم الكامل باللغة الإنجليزية (اختياري)</Label>
            <Input 
              id="nameEn"
              type="text" 
              placeholder="Full name in English" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={nameEn}
            />
          </div>

          <!-- تاريخ الميلاد -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="birthDate">تاريخ الميلاد <span class="text-destructive">*</span></Label>
            <Input 
              id="birthDate"
              type="date" 
              class="text-xs" 
              bind:value={birthDate}
            />
          </div>

          <!-- الجنس -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground">الجنس <span class="text-destructive">*</span></Label>
            <div class="flex gap-4 items-center h-9">
              <button 
                type="button"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'male' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground'}"
                onclick={() => sex = 'male'}
              >
                <span>ذكر</span>
              </button>
              <button 
                type="button"
                class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'female' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground'}"
                onclick={() => sex = 'female'}
              >
                <span>أنثى</span>
              </button>
            </div>
          </div>

          <!-- اسم الأم -->
          <div class="flex flex-col gap-1.5 md:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherName">اسم الأم الكامل</Label>
            <Input 
              id="motherName"
              type="text" 
              placeholder="اسم الأم الكامل" 
              class="text-xs" 
              bind:value={motherName}
            />
          </div>

          <!-- اسم الأب / ولي الأمر -->
          <div class="flex flex-col gap-1.5 md:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="guardianName">اسم الأب الكامل (ولي الأمر)</Label>
            <Input 
              id="guardianName"
              type="text" 
              placeholder="اسم الأب الكامل" 
              class="text-xs" 
              bind:value={guardianName}
            />
          </div>

          <!-- رقم هاتف الأب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="guardianPhone">رقم هاتف الأب</Label>
            <Input 
              id="guardianPhone"
              type="tel" 
              placeholder="رقم الهاتف" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={guardianPhone}
            />
          </div>

          <!-- رقم هاتف الأم -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherPhone">رقم هاتف الأم</Label>
            <Input 
              id="motherPhone"
              type="tel" 
              placeholder="رقم الهاتف" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={motherPhone}
            />
          </div>

          <!-- عمل الأب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="fatherJob">عمل الأب</Label>
            <Input 
              id="fatherJob"
              type="text" 
              placeholder="الوظيفة / المهنة" 
              class="text-xs" 
              bind:value={fatherJob}
            />
          </div>

          <!-- عمل الأم -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="motherJob">عمل الأم</Label>
            <Input 
              id="motherJob"
              type="text" 
              placeholder="الوظيفة / المهنة" 
              class="text-xs" 
              bind:value={motherJob}
            />
          </div>

          <!-- المدرسة السابقة -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="previousSchool">المدرسة السابقة</Label>
            <Input 
              id="previousSchool"
              type="text" 
              placeholder="اسم المدرسة السابقة" 
              class="text-xs" 
              bind:value={previousSchool}
            />
          </div>

          <!-- الصف الدراسي المطلوب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="classSelect">الصف الدراسي المطلوب قيده فيه <span class="text-destructive">*</span></Label>
            <Select bind:value={classId} type="single">
              <SelectTrigger class="w-full h-8 text-xs text-right flex items-center justify-between" id="classSelect">
                <span>{selectedClassName}</span>
              </SelectTrigger>
              <SelectContent class="text-right" align="end">
                {#each studentsStore.classes as cls (cls.id)}
                  <SelectItem value={String(cls.id)} class="text-xs text-right justify-start">
                    {cls.name || cls.name_ar}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <!-- المديرية -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="district">المديرية</Label>
            <Input 
              id="district"
              type="text" 
              placeholder="اسم المديرية" 
              class="text-xs" 
              bind:value={district}
            />
          </div>

          <!-- المحافظة -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="governorate">المحافظة</Label>
            <Input 
              id="governorate"
              type="text" 
              placeholder="اسم المحافظة" 
              class="text-xs" 
              bind:value={governorate}
            />
          </div>

          <!-- البريد الإلكتروني -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="email">البريد الإلكتروني للطالب (اختياري)</Label>
            <Input 
              id="email"
              type="email" 
              placeholder="student@example.com" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={email}
            />
          </div>

          <!-- الرقم الأكاديمي -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="studentNumber">الرقم الأكاديمي للطالب (إن وجد)</Label>
            <Input 
              id="studentNumber"
              type="text" 
              placeholder="رقم القيد الأكاديمي" 
              class="text-xs" 
              bind:value={studentNumber}
            />
          </div>

          <!-- هاتف الطالب -->
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="phone">رقم هاتف الطالب (إن وجد)</Label>
            <Input 
              id="phone"
              type="tel" 
              placeholder="رقم الهاتف الخاص بالطالب" 
              class="text-xs font-mono text-left" 
              dir="ltr"
              bind:value={phone}
            />
          </div>

          <!-- السكن / العنوان الحالي -->
          <div class="flex flex-col gap-1.5 md:col-span-2">
            <Label class="text-[10px] font-semibold text-muted-foreground" for="address">العنوان الحالي / السكن</Label>
            <Input 
              id="address"
              type="text" 
              placeholder="المحافظة - المديرية - الحارة / الشارع" 
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
          <h2 class="text-sm font-bold text-foreground">بيانات الوثائق المرفقة والمستلمة</h2>
        </div>

        <p class="text-[10px] text-muted-foreground">قم بتحديد الوثائق والشهادات الدراسية التي تم تسليمها من قبل الطالب لإلحاقها بملفه الأكاديمي:</p>

        <!-- Dynamic Grid Checklist matching the paper form layout -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
          {#each docTypes as doc (doc.id)}
            {@const isChecked = submittedDocs.includes(doc.id)}
            <button
              type="button"
              class="flex items-center justify-between p-3 border rounded-lg cursor-pointer text-right text-xs transition-all duration-150 {isChecked ? 'border-primary bg-primary/5 text-foreground' : 'border-border bg-card/25 text-muted-foreground'}"
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
            </button>
          {/each}
        </div>
      </div>

      <!-- Action Buttons Row at the bottom of the form -->
      <div class="flex items-center justify-end gap-3 pt-6 border-t border-border mt-8 shrink-0">
        <Button 
          variant="ghost" 
          class="btn-tactile text-xs h-9 px-5" 
          onclick={() => goto(`/${orgId}/students`)}
          disabled={isSaving}
        >
          إلغاء
        </Button>
        <Button 
          variant="lime" 
          class="btn-tactile-lime text-xs h-9 px-6 gap-1.5" 
          onclick={handleSave}
          disabled={isSaving}
        >
          <Save size={14} />
          {isSaving ? "جاري الحفظ..." : "حفظ البيانات"}
        </Button>
      </div>
      
    </div>
  </div>
</div>
