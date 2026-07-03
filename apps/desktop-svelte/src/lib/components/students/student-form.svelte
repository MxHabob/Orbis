<!--
@component StudentForm
Redesigned student registration form matching the fields and layout of the registration paper form.
Uses S.C.R.A.P.S. design components with top-tier 2026 aesthetics, tabbed UX, optional fees checklist, and discounts.
Supports dynamic visibility config, duplicate student check, multi-guardian links, and registration status transitions.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import { Switch } from "@orbis/ui-svelte/switch";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui-svelte/card";
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
  Users,
  MapPin,
  GraduationCap,
  Calendar,
  Contact,
  CreditCard,
  Percent,
  Plus,
  Trash2,
  Search,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

let { studentId, orgId }: { studentId?: string; orgId: string } = $props();

const isEditMode = $derived(!!studentId);

// Core Student fields
let nameAr = $state("");
let nameEn = $state("");
let nationalId = $state("");
let birthDate = $state("");
let sex = $state("male"); // male / female
let classId = $state<any>("");
let studentNumber = $state("");
let email = $state("");
let phone = $state("");
let address = $state("");
let status = $state("active");

// Dynamic Config values
let requiredFields = $state<Record<string, boolean>>({});
let activeFields = $state<Record<string, boolean>>({});
let preventDuplicates = $state<Record<string, boolean>>({});
let registrationStatus = $state("active"); // draft, pending_review, active

// Duplicate warning states
let duplicateWarnings = $state<{
  national_id?: string;
  email?: string;
  phone?: string;
}>({});

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

// Many-to-Many Guardians List
let linkedGuardians = $state<
  Array<{
    id?: string;
    user_id?: string;
    full_name: string;
    phone: string;
    email: string;
    national_id: string;
    relationship: string;
    relationship_ar: string;
    is_primary: boolean;
    is_emergency_contact: boolean;
    is_payer: boolean;
    payment_share_percent: number;
    occupation: string;
  }>
>([
  {
    full_name: "",
    phone: "",
    email: "",
    national_id: "",
    relationship: "father",
    relationship_ar: "أب",
    is_primary: true,
    is_emergency_contact: true,
    is_payer: true,
    payment_share_percent: 100,
    occupation: "",
  },
]);

// Guardian autocomplete states
let parentSearchResults = $state<any[]>([]);
let activeSearchIndex = $state<number | null>(null);

async function searchParent(query: string, index: number) {
  if (query.trim().length < 3) {
    parentSearchResults = [];
    return;
  }
  activeSearchIndex = index;
  try {
    parentSearchResults = await commands.guardians.search(orgId, query);
  } catch (err) {
    console.error(err);
  }
}

function selectExistingParent(parent: any, index: number) {
  linkedGuardians[index].user_id = parent.id;
  linkedGuardians[index].full_name = parent.full_name;
  linkedGuardians[index].phone = parent.phone || "";
  linkedGuardians[index].email = parent.email || "";
  linkedGuardians[index].national_id = parent.national_id || "";
  parentSearchResults = [];
  activeSearchIndex = null;
  toast.success(m.form_guardian_select_existing_success());
}

function addGuardianField() {
  linkedGuardians = [
    ...linkedGuardians,
    {
      full_name: "",
      phone: "",
      email: "",
      national_id: "",
      relationship: "mother",
      relationship_ar: "أم",
      is_primary: false,
      is_emergency_contact: false,
      is_payer: false,
      payment_share_percent: 0,
      occupation: "",
    },
  ];
}

function removeGuardianField(index: number) {
  linkedGuardians = linkedGuardians.filter((_, i) => i !== index);
}

// Flat educational and geographical meta details
let motherName = $state("");
let motherPhone = $state("");
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
let orgRequiredDocuments = $state<string[]>([]);

const docTypes = $derived.by(() => {
  if (orgRequiredDocuments.length > 0) {
    return orgRequiredDocuments.map((name) => ({ id: name, label: name }));
  }
  // Fallback
  return [
    { id: "birth_certificate", label: m.form_doc_birth_certificate() },
    { id: "national_id", label: m.form_doc_national_id() },
    {
      id: "previous_record",
      label:
        getLocale() === "ar"
          ? "الشهادة الدراسية السابقة"
          : "Previous Academic Record",
    },
  ];
});
let submittedDocs = $state<string[]>([]);

let isSaving = $state(false);
let orgName = $state("");

// Print Template selection state
let availableTemplates = $state<any[]>([]);
let selectedTemplateId = $state("");

// Discount & Optional Fees State
let discountTemplates = $state<any[]>([]);
let feeTemplates = $state<any[]>([]);
let applyDiscount = $state(false);
let discountType = $state("template"); // template | custom
let selectedDiscountTemplateId = $state("");
let customDiscountAmount = $state<number | null>(null);
let selectedOptionalFees = $state<
  Record<string, { checked: boolean; amount: number }>
>({});

onMount(async () => {
  if (studentsStore.classes.length === 0) {
    await studentsStore.load(orgId);
  }

  try {
    const org = await commands.users.getOrganization(orgId);
    orgName = org.name || "";

    // Parse settings
    if (org.settings) {
      try {
        const settings = JSON.parse(org.settings);
        const config = settings.registration_config || {};
        activeFields = config.active_fields || {};
        requiredFields = config.required_fields || {};
        preventDuplicates = config.prevent_duplicates || {};
        orgRequiredDocuments = config.required_documents || [];
      } catch (err) {
        console.error("Failed to parse settings JSON:", err);
      }
    }
  } catch (e) {
    console.error("Failed to load organization details:", e);
  }

  // Load print templates
  try {
    const tpls = await commands.documentTemplates.getTemplates(orgId);
    availableTemplates = tpls.filter(
      (t) =>
        t.entity_type === "user" ||
        t.entity_type === "enrollment" ||
        String(t.id).includes("registration"),
    );

    const defaultTpl =
      availableTemplates.find((t) => t.id === "t5-student-registration-form") ||
      availableTemplates.find((t) => t.is_default === 1) ||
      availableTemplates[0];

    if (defaultTpl) {
      selectedTemplateId = defaultTpl.id;
    } else {
      selectedTemplateId = "t5-student-registration-form";
    }
  } catch (e) {
    console.error("Failed to load print templates:", e);
    selectedTemplateId = "t5-student-registration-form";
  }

  // Load active discount templates and fee templates
  try {
    const [fTpls, dTpls] = await Promise.all([
      commands.finance.getFeeTemplates(orgId),
      commands.finance.getActiveDiscountTemplates(orgId),
    ]);
    feeTemplates = fTpls || [];
    discountTemplates = dTpls || [];

    // Pre-populate optional fees mapping
    const optionalFeesMap: Record<
      string,
      { checked: boolean; amount: number }
    > = {};
    feeTemplates.forEach((t) => {
      if (t.is_mandatory === 0 && t.is_active === 1) {
        optionalFeesMap[t.id] = { checked: false, amount: t.default_amount };
      }
    });
    selectedOptionalFees = optionalFeesMap;
  } catch (e) {
    console.error("Failed to load financial templates:", e);
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
      status = String(student.status || "active");

      // Fetch linked guardians
      const guardianLinks = await commands.guardians.getByStudent(
        student.student_id,
      );
      if (guardianLinks && guardianLinks.length > 0) {
        linkedGuardians = [];
        for (const link of guardianLinks) {
          const guardianUser = await commands.users.getUser(
            String(link.guardian_id),
          );
          if (guardianUser) {
            linkedGuardians.push({
              id: String(link.id),
              user_id: String(guardianUser.id),
              full_name: guardianUser.full_name,
              phone: guardianUser.phone || "",
              email: guardianUser.email || "",
              national_id: guardianUser.national_id || "",
              relationship: link.relationship,
              relationship_ar: link.relationship_ar || "أب",
              is_primary: link.is_primary === 1,
              is_emergency_contact: link.is_emergency_contact === 1,
              is_payer: link.is_payer === 1,
              payment_share_percent: link.payment_share_percent || 100,
              occupation: link.occupation || "",
            });
          }
        }
      }

      // Load existing student fees
      try {
        const fees = await commands.finance.getStudentFees(orgId, student.id);
        const optionalFeesMap = { ...selectedOptionalFees };
        fees.forEach((f) => {
          if (f.fee_template_id && optionalFeesMap[f.fee_template_id]) {
            optionalFeesMap[f.fee_template_id] = {
              checked: true,
              amount: f.original_amount,
            };
          }
        });
        selectedOptionalFees = optionalFeesMap;
      } catch (e) {
        console.error("Failed to load existing student fees:", e);
      }
    }
  } catch (e) {
    console.error("Error loading student edit data:", e);
    toast.error(m.form_error_load());
  }
}

// Live duplicate check triggers
async function triggerDuplicateCheck(
  field: "national_id" | "email" | "phone",
  value: string,
) {
  if (!value.trim() || !preventDuplicates[field]) return;
  try {
    const params = {
      nationalId: field === "national_id" ? value : undefined,
      email: field === "email" ? value : undefined,
      phone: field === "phone" ? value : undefined,
    };
    const duplicates = await commands.enrollments.checkDuplicate(orgId, params);
    if (duplicates && duplicates.length > 0) {
      const match = duplicates[0];
      duplicateWarnings = {
        ...duplicateWarnings,
        [field]: m.form_guardian_duplicate_warning({ name: match.full_name }),
      };
    } else {
      duplicateWarnings = {
        ...duplicateWarnings,
        [field]: undefined,
      };
    }
  } catch (err) {
    console.error(err);
  }
}

// ── Financial Live Calculations ─────────────────────────────────────
const classLevel = $derived(
  studentsStore.classes.find((c) => String(c.id) === classId)?.level || "",
);

const mandatoryFeesList = $derived.by(() => {
  if (!classLevel) return [];
  return feeTemplates.filter((t) => {
    if (t.is_mandatory === 1 && t.is_active === 1) {
      if (t.applicable_levels) {
        try {
          const rules = JSON.parse(t.applicable_levels);
          if (rules.levels && Array.isArray(rules.levels)) {
            return rules.levels.includes(classLevel);
          } else if (rules.level) {
            return rules.level === classLevel;
          }
        } catch {
          return false;
        }
      }
      return true;
    }
    return false;
  });
});

const mandatoryTotal = $derived(
  mandatoryFeesList.reduce((sum, t) => sum + t.default_amount, 0),
);

const discountTotal = $derived.by(() => {
  if (!applyDiscount) return 0;
  if (discountType === "template" && selectedDiscountTemplateId) {
    const tpl = discountTemplates.find(
      (t) => t.id === selectedDiscountTemplateId,
    );
    if (tpl) {
      const val = (tpl.value as number) || 0;
      if (String(tpl.calculation_method).toLowerCase() === "percentage") {
        return (mandatoryTotal * val) / 100;
      } else {
        return val;
      }
    }
  } else if (discountType === "custom" && customDiscountAmount) {
    return customDiscountAmount;
  }
  return 0;
});

const optionalFeesList = $derived(
  feeTemplates.filter((t) => t.is_mandatory === 0 && t.is_active === 1),
);

const optionalTotal = $derived.by(() => {
  let sum = 0;
  for (const key in selectedOptionalFees) {
    if (selectedOptionalFees[key].checked) {
      sum += selectedOptionalFees[key].amount || 0;
    }
  }
  return sum;
});

const netTotal = $derived(
  Math.max(0, mandatoryTotal + optionalTotal - discountTotal),
);

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
  if (requiredFields.national_id && !nationalId.trim()) {
    toast.error(
      getLocale() === "ar"
        ? "رقم الهوية الوطنية مطلوب"
        : "National ID is required",
    );
    return;
  }
  if (requiredFields.phone && !phone.trim()) {
    toast.error(
      getLocale() === "ar"
        ? "رقم هاتف الطالب مطلوب"
        : "Phone number is required",
    );
    return;
  }
  if (requiredFields.email && !email.trim()) {
    toast.error(
      getLocale() === "ar" ? "البريد الإلكتروني مطلوب" : "Email is required",
    );
    return;
  }

  isSaving = true;
  const createdBy = authStore.user?.id || "";
  let targetStudentId = "";
  let targetEnrollmentId = studentId || "";

  try {
    // Verify control account
    const accounts = await commands.finance.getLedgerAccounts(orgId);
    const hasControlAccount = accounts.some((acc) => acc.code === "110301");
    if (!hasControlAccount) {
      toast.error(m.form_error_control_account_missing());
      isSaving = false;
      return;
    }

    const studentUserData = {
      full_name: nameAr.trim(),
      name_ar: nameAr.trim(),
      name_en: nameEn.trim() || null,
      email: email.trim() || null,
      phone: phone.trim() || null,
      sex: sex,
      date_of_birth: birthDate || null,
      national_id: nationalId.trim() || null,
      address: address.trim() || null,
      status: "active",
    };

    if (isEditMode) {
      const originalStudent = studentsStore.list.find(
        (s) => s.id === studentId,
      );
      if (!originalStudent) {
        throw new Error(m.form_error_enrollment_not_found());
      }
      targetStudentId = String(originalStudent.student_id);
      await commands.users.updateUser(targetStudentId, studentUserData);

      if (originalStudent.class_id !== classId) {
        await commands.enrollments.transfer(studentId!, {
          new_class_id: classId,
        });
      }

      // Update registration status transition
      if (originalStudent.status !== registrationStatus) {
        await commands.enrollments.transitionStatus(
          orgId,
          studentId!,
          registrationStatus,
        );
      }

      toast.success(m.form_success_update());
    } else {
      const tempEmail = email.trim() || `stu.${Date.now()}@orbis.local`;
      const studentUser = await commands.users.createUser(orgId, createdBy, {
        email: tempEmail,
        password: "Orbis123456",
        full_name: nameAr.trim(),
        name_ar: nameAr.trim(),
        primary_role: "student",
        sex: sex,
        phone: phone.trim() || null,
        national_id: nationalId.trim() || null,
        date_of_birth: birthDate || null,
      });
      targetStudentId = studentUser.id;

      await commands.users.updateUser(studentUser.id, {
        name_en: nameEn.trim() || null,
        address: address.trim() || null,
      });

      const enrollmentRequest = {
        student_id: studentUser.id,
        class_id: classId,
        enrollment_type: "regular",
        discount_template_id:
          applyDiscount && discountType === "template"
            ? selectedDiscountTemplateId || null
            : null,
        custom_discount_amount:
          applyDiscount && discountType === "custom"
            ? customDiscountAmount || null
            : null,
        status: registrationStatus,
      };

      const res = await commands.enrollments.create(orgId, enrollmentRequest);
      targetEnrollmentId = String(res.id);

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

    // Save multi-guardian links
    for (const guardian of linkedGuardians) {
      if (!guardian.full_name.trim()) continue;
      let guardianUserId = guardian.user_id;

      if (!guardianUserId) {
        // Search if already exists
        const checkExisting = await commands.guardians.search(
          orgId,
          guardian.phone,
        );
        if (checkExisting && checkExisting.length > 0) {
          guardianUserId = checkExisting[0].id;
        } else {
          // Create new parent user
          const guardEmail =
            guardian.email.trim() || `parent.${Date.now()}@orbis.local`;
          const guardUser = await commands.users.createUser(orgId, createdBy, {
            email: guardEmail,
            password: "Orbis123456",
            full_name: guardian.full_name.trim(),
            name_ar: guardian.full_name.trim(),
            primary_role: "parent",
            phone: guardian.phone.trim() || null,
            sex: "male",
            national_id: guardian.national_id.trim() || null,
            date_of_birth: null,
          });
          guardianUserId = guardUser.id;
        }
      } else {
        // Update existing parent user
        await commands.users.updateUser(guardianUserId, {
          full_name: guardian.full_name.trim(),
          name_ar: guardian.full_name.trim(),
          phone: guardian.phone.trim() || null,
          national_id: guardian.national_id.trim() || null,
        });
      }

      // Check if link exists
      const existingLinks =
        await commands.guardians.getByStudent(targetStudentId);
      const matchedLink = existingLinks.find(
        (l) => l.guardian_id === guardianUserId,
      );

      const linkRequest = {
        student_id: targetStudentId,
        guardian_id: guardianUserId,
        relationship: guardian.relationship,
        relationship_ar: guardian.relationship_ar,
        is_primary: guardian.is_primary ? 1 : 0,
        is_emergency_contact: guardian.is_emergency_contact ? 1 : 0,
        occupation: guardian.occupation.trim() || null,
        is_payer: guardian.is_payer ? 1 : 0,
        payment_share_percent: guardian.payment_share_percent,
        can_view_grades: 1,
        can_view_attendance: 1,
        can_view_behavior: 1,
        can_view_finance: guardian.is_payer ? 1 : 0,
        can_view_schedule: 1,
        can_receive_notifications: 1,
      };

      if (matchedLink) {
        await commands.guardians.updateLink(
          String(matchedLink.id),
          linkRequest,
        );
      } else {
        await commands.guardians.createLink(linkRequest);
      }
    }

    // Assign optional/custom fees (only if status is active/approved)
    if (registrationStatus === "active" || registrationStatus === "approved") {
      const today = new Date().toISOString().slice(0, 10);
      for (const key in selectedOptionalFees) {
        if (selectedOptionalFees[key].checked) {
          try {
            await commands.finance.assignStudentFee(orgId, {
              enrollment_id: targetEnrollmentId,
              fee_template_id: key,
              amount: selectedOptionalFees[key].amount,
              due_date: today,
              notes: "رسوم إضافية مضافة عند التسجيل",
            });
          } catch (feeErr) {
            console.error("Failed to assign optional fee:", key, feeErr);
          }
        }
      }
    }

    goto(localizeHref(`/${orgId}/students`));
  } catch (e: any) {
    if (targetStudentId && !isEditMode) {
      try {
        await commands.users.deleteUser(targetStudentId, createdBy);
      } catch (deleteErr) {
        console.error("Failed to rollback student user creation:", deleteErr);
      }
    }
    console.error("Save error:", e);
    toast.error(m.form_error_save({ error: e.message || String(e) }));
  } finally {
    isSaving = false;
  }
}

let isDownloadingForm = $state(false);

async function handleDownloadRegistrationForm() {
  if (!selectedTemplateId) {
    toast.error("يرجى اختيار قالب الطباعة أولاً");
    return;
  }
  isDownloadingForm = true;
  try {
    const templateRecord = await commands.documentTemplates.getTemplate(
      orgId,
      selectedTemplateId,
    );
    if (!templateRecord) {
      toast.error(`القالب '${selectedTemplateId}' غير موجود في قاعدة البيانات`);
      return;
    }
    const documentModel = JSON.parse(templateRecord.document_json);

    const printVariables = {
      orgName,
      student_name: nameAr,
      name_ar: nameAr,
      name_en: nameEn,
      birth_date: birthDate,
      sex: sex === "male" ? "ذكر" : "أنثى",
      sex_en: sex,
      class_name: selectedClassName,
      student_number: studentNumber,
      email: email,
      phone: phone,
      address: address,
      documents: submittedDocs.join(", "),
      current_date: new Date().toLocaleDateString("ar-YE"),
      mandatory_fees: mandatoryTotal.toString(),
      optional_fees: optionalTotal.toString(),
      discount_fees: discountTotal.toString(),
      net_fees: netTotal.toString(),
    };

    const pdfBytes = await commands.docforge.previewDocumentTemplate(
      JSON.stringify(documentModel),
      JSON.stringify(printVariables),
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
      link.download = `استمارة_${nameAr || "طالب"}.pdf`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      toast.success("تم توليد وتحميل المستند بنجاح");
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

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-foreground text-start" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    
    <!-- Page Header -->
    <div class="flex items-center justify-between gap-4 border-b border-border/60 pb-5">
      <div class="flex flex-col gap-1 text-start">
        <h1 class="text-xl font-bold tracking-tight text-foreground">
          {isEditMode ? m.form_edit_student() : m.form_new_student()}
        </h1>
        <p class="text-xs text-muted-foreground">
          {m.form_student_desc()}
        </p>
      </div>
      
      <!-- Badges indicating the current state -->
      <div class="flex items-center gap-2 flex-wrap">
        {#if isEditMode}
          <Badge variant={status === 'active' ? 'lime' : 'secondary'} class="text-[10px] font-semibold tracking-wider uppercase">
            {status === 'active' ? m.form_active_status() : status}
          </Badge>
        {:else}
          <Badge variant="outline" class="text-[10px] font-semibold text-primary border-primary/30 bg-primary/5 uppercase">
            {m.form_draft_status()}
          </Badge>
        {/if}
      </div>
    </div>

    <!-- Tabs Interface -->
    <Tabs value="profile" class="w-full">
      <TabsList class="grid grid-cols-5 max-w-3xl mx-auto mb-8 bg-muted/20 p-1 rounded-xl border border-border/40">
        <TabsTrigger value="profile" class="text-xs font-semibold gap-2 py-2 cursor-pointer">
          <User class="h-4 w-4" />
          <span class="hidden sm:inline">{m.form_tab_profile()}</span>
        </TabsTrigger>
        <TabsTrigger value="family" class="text-xs font-semibold gap-2 py-2 cursor-pointer">
          <Users class="h-4 w-4" />
          <span class="hidden sm:inline">{m.form_tab_family()}</span>
        </TabsTrigger>
        <TabsTrigger value="location" class="text-xs font-semibold gap-2 py-2 cursor-pointer">
          <MapPin class="h-4 w-4" />
          <span class="hidden sm:inline">{m.form_tab_location()}</span>
        </TabsTrigger>
        <TabsTrigger value="financials" class="text-xs font-semibold gap-2 py-2 cursor-pointer">
          <CreditCard class="h-4 w-4" />
          <span class="hidden sm:inline">{m.form_tab_financials()}</span>
        </TabsTrigger>
        <TabsTrigger value="documents" class="text-xs font-semibold gap-2 py-2 cursor-pointer">
          <FileText class="h-4 w-4" />
          <span class="hidden sm:inline">{m.form_tab_documents()}</span>
        </TabsTrigger>
      </TabsList>

      <!-- TAB 1: Student Profile -->
      <TabsContent value="profile">
        <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
          <CardHeader class="border-b border-border/60 pb-4">
            <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
              <User class="text-primary h-4.5 w-4.5" />
              <span>{m.form_student_info()}</span>
            </CardTitle>
            <CardDescription class="text-[11px] text-muted-foreground">
              البيانات الشخصية والأساسية المطلوبة لطالب/ة
            </CardDescription>
          </CardHeader>
          <CardContent class="p-6 space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              
              <!-- الاسم بالكامل (العربية) -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="nameAr">
                  {m.form_full_name_ar()} <span class="text-destructive">{m.form_required()}</span>
                </Label>
                <Input 
                  id="nameAr"
                  type="text" 
                  placeholder={m.form_full_name_ar_placeholder()} 
                  class="font-bold text-xs border border-input" 
                  bind:value={nameAr}
                  autocomplete="off"
                />
              </div>

              <!-- الاسم بالكامل (الإنجليزية) -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="nameEn">
                  {m.form_full_name_en()}
                </Label>
                <Input 
                  id="nameEn"
                  type="text" 
                  placeholder={m.form_full_name_en_placeholder()} 
                  class="text-xs font-mono text-left border border-input" 
                  dir="ltr"
                  bind:value={nameEn}
                  autocomplete="off"
                />
              </div>

              <!-- رقم الهوية الوطنية / جواز السفر -->
              {#if activeFields.national_id}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground" for="nationalId">
                    رقم الهوية الوطنية / جواز السفر {#if requiredFields.national_id}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <Input 
                    id="nationalId"
                    type="text" 
                    placeholder="1000234900" 
                    class="text-xs font-mono border border-input" 
                    bind:value={nationalId}
                    onblur={() => triggerDuplicateCheck("national_id", nationalId)}
                  />
                  {#if duplicateWarnings.national_id}
                    <span class="text-[10px] text-destructive font-semibold flex items-center gap-1">
                      <AlertCircle size={10} />
                      {duplicateWarnings.national_id}
                    </span>
                  {/if}
                </div>
              {/if}

              <!-- تاريخ الميلاد -->
              {#if activeFields.birth_date}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground">
                    {m.form_birth_date()} {#if requiredFields.birth_date}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <div class="grid grid-cols-3 gap-2" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
                    <!-- اليوم -->
                    <Select bind:value={birthDay} type="single">
                      <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input px-2" id="birthDaySelect">
                        <span class="truncate">{birthDay || m.form_day()}</span>
                      </SelectTrigger>
                      <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                        {#each daysList as dy}
                          <SelectItem value={dy} class="text-xs text-start justify-start cursor-pointer">
                            {dy}
                          </SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>

                    <!-- الشهر -->
                    <Select bind:value={birthMonth} type="single">
                      <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input px-2" id="birthMonthSelect">
                        <span class="truncate">{monthsList.find(m => m.value === birthMonth)?.label || m.form_month()}</span>
                      </SelectTrigger>
                      <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                        {#each monthsList as mo}
                          <SelectItem value={mo.value} class="text-xs text-start justify-start cursor-pointer">
                            {mo.label}
                          </SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>

                    <!-- السنة -->
                    <Select bind:value={birthYear} type="single">
                      <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input px-2" id="birthYearSelect">
                        <span class="truncate">{birthYear || m.form_year()}</span>
                      </SelectTrigger>
                      <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                        {#each yearsList as yr}
                          <SelectItem value={String(yr)} class="text-xs text-start justify-start cursor-pointer">
                            {yr}
                          </SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              {/if}

              <!-- الجنس -->
              {#if activeFields.sex}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground">
                    {m.form_sex()} {#if requiredFields.sex}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <div class="flex gap-2 items-center h-8">
                    <Button
                      variant="ghost"
                      class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'male' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                      onclick={() => sex = 'male'}
                    >
                      <span>{m.form_male()}</span>
                    </Button>
                    <Button 
                      variant="ghost"
                      class="flex-1 flex items-center justify-center gap-2 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {sex === 'female' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                      onclick={() => sex = 'female'}
                    >
                      <span>{m.form_female()}</span>
                    </Button>
                  </div>
                </div>
              {/if}

              <!-- الصف الدراسي المطلوب -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="classSelect">
                  {m.form_target_class()} <span class="text-destructive">{m.form_required()}</span>
                </Label>
                <Select bind:value={classId} type="single">
                  <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input" id="classSelect">
                    <span>{selectedClassName}</span>
                  </SelectTrigger>
                  <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                    {#each studentsStore.classes as cls (cls.id)}
                      <SelectItem value={String(cls.id)} class="text-xs text-start justify-start cursor-pointer">
                        {cls.name || cls.name_ar}
                      </SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>

              <!-- الرقم الأكاديمي -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="studentNumber">
                  {m.form_student_number()}
                </Label>
                <Input 
                  id="studentNumber"
                  type="text" 
                  placeholder={m.form_student_number_placeholder()} 
                  class="text-xs border border-input" 
                  disabled={!isEditMode}
                  bind:value={studentNumber}
                />
              </div>

              <!-- هاتف الطالب -->
              {#if activeFields.phone}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground" for="phone">
                    {m.form_student_phone()} {#if requiredFields.phone}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <Input 
                    id="phone"
                    type="tel" 
                    placeholder={m.form_student_phone_placeholder()} 
                    class="text-xs font-mono text-left border border-input" 
                    dir="ltr"
                    bind:value={phone}
                    onblur={() => triggerDuplicateCheck("phone", phone)}
                  />
                  {#if duplicateWarnings.phone}
                    <span class="text-[10px] text-destructive font-semibold flex items-center gap-1">
                      <AlertCircle size={10} />
                      {duplicateWarnings.phone}
                    </span>
                  {/if}
                </div>
              {/if}

              <!-- البريد الإلكتروني -->
              {#if activeFields.email}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground" for="email">
                    {m.form_student_email()} {#if requiredFields.email}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <Input 
                    id="email"
                    type="email" 
                    placeholder="student@example.com" 
                    class="text-xs font-mono text-left border border-input" 
                    dir="ltr"
                    bind:value={email}
                    onblur={() => triggerDuplicateCheck("email", email)}
                  />
                  {#if duplicateWarnings.email}
                    <span class="text-[10px] text-destructive font-semibold flex items-center gap-1">
                      <AlertCircle size={10} />
                      {duplicateWarnings.email}
                    </span>
                  {/if}
                </div>
              {/if}
            </div>
          </CardContent>
        </Card>
      </TabsContent>

      <!-- TAB 2: Family & Guardians -->
      <TabsContent value="family">
        <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
          <CardHeader class="border-b border-border/60 pb-4 flex flex-row items-center justify-between">
            <div class="flex flex-col gap-1">
              <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
                <Users class="text-primary h-4.5 w-4.5" />
                <span>{m.form_guardian_title()}</span>
              </CardTitle>
              <CardDescription class="text-[11px] text-muted-foreground">
                {m.form_guardian_desc()}
              </CardDescription>
            </div>
            <Button type="button" variant="outline" size="sm" onclick={addGuardianField}>
              <Plus size={14} class="me-1.5" />
              {m.form_guardian_add()}
            </Button>
          </CardHeader>
          <CardContent class="p-6 space-y-6">
            {#each linkedGuardians as guardian, index}
              <div class="p-4 border border-border/80 rounded-xl bg-card/10 space-y-4 text-start relative">
                
                <div class="flex items-center justify-between border-b border-border/60 pb-2">
                  <span class="text-xs font-bold text-primary">{m.form_guardian_number()} {index + 1}</span>
                  {#if index > 0}
                    <Button type="button" variant="ghost" class="text-destructive hover:bg-destructive/10 h-7 px-2" onclick={() => removeGuardianField(index)}>
                      <Trash2 size={13} class="me-1.5" />
                      {m.form_guardian_remove()}
                    </Button>
                  {/if}
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <!-- البحث عن ولي أمر موجود -->
                  <div class="flex flex-col gap-1.5 md:col-span-2 relative">
                    <Label class="text-[11px] font-semibold text-muted-foreground">{m.form_guardian_search_label()}</Label>
                    <div class="relative w-full">
                      <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
                      <Input 
                        placeholder={m.form_guardian_search_placeholder()} 
                        class="h-8 ps-9 text-xs border border-input"
                        oninput={(e) => searchParent((e.target as HTMLInputElement).value, index)}
                      />
                    </div>

                    <!-- Search dropdown results -->
                    {#if activeSearchIndex === index && parentSearchResults.length > 0}
                      <div class="absolute top-full left-0 right-0 z-50 mt-1 bg-popover border border-border rounded-lg shadow-xl max-h-48 overflow-y-auto p-1">
                        {#each parentSearchResults as parent}
                          <button 
                            type="button"
                            class="w-full text-start p-2 hover:bg-accent/40 rounded text-xs flex justify-between items-center text-foreground font-medium cursor-pointer"
                            onclick={() => selectExistingParent(parent, index)}
                          >
                            <span>{parent.full_name} ({parent.phone || 'بلا هاتف'})</span>
                            <Badge variant="secondary" class="text-[9px]">ولي أمر مسجل</Badge>
                          </button>
                        {/each}
                      </div>
                    {/if}
                  </div>

                  <!-- الاسم -->
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[11px] font-semibold text-muted-foreground">{m.form_guardian_name_label()} <span class="text-destructive">{m.form_required()}</span></Label>
                    <Input bind:value={guardian.full_name} class="h-8 text-xs border border-input" placeholder="اسم ولي الأمر" />
                  </div>

                  <!-- رقم الهاتف -->
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[11px] font-semibold text-muted-foreground">{m.form_guardian_phone_label()} <span class="text-destructive">{m.form_required()}</span></Label>
                    <Input bind:value={guardian.phone} class="h-8 text-xs font-mono border border-input text-left" dir="ltr" placeholder="777000000" />
                  </div>

                  <!-- صلة القرابة -->
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[11px] font-semibold text-muted-foreground">{m.form_guardian_relationship_label()}</Label>
                    <Select bind:value={guardian.relationship} type="single">
                      <SelectTrigger class="h-8 text-xs flex justify-between items-center border border-input">
                        <span>{guardian.relationship === "father" ? "أب" : guardian.relationship === "mother" ? "أم" : guardian.relationship === "uncle" ? "عم" : "آخر"}</span>
                      </SelectTrigger>
                      <SelectContent class="text-start">
                        <SelectItem value="father" onclick={() => guardian.relationship_ar = "أب"}>أب</SelectItem>
                        <SelectItem value="mother" onclick={() => guardian.relationship_ar = "أم"}>أم</SelectItem>
                        <SelectItem value="uncle" onclick={() => guardian.relationship_ar = "عم"}>عم</SelectItem>
                        <SelectItem value="other" onclick={() => guardian.relationship_ar = "قريب/آخر"}>آخر</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <!-- الهوية الوطنية لولي الأمر -->
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-[11px] font-semibold text-muted-foreground">{m.form_guardian_national_id_label()}</Label>
                    <Input bind:value={guardian.national_id} class="h-8 text-xs font-mono border border-input" placeholder="رقم الهوية" />
                  </div>

                  <!-- خيارات الصلاحية والمسؤولية -->
                  <div class="md:col-span-2 grid grid-cols-2 sm:grid-cols-4 gap-4 pt-2">
                    <div class="flex items-center gap-2">
                      <Checkbox id={`primary_${index}`} checked={guardian.is_primary} onCheckedChange={(val) => guardian.is_primary = !!val} />
                      <Label for={`primary_${index}`} class="text-xs cursor-pointer">{m.form_guardian_role_primary()}</Label>
                    </div>

                    <div class="flex items-center gap-2">
                      <Checkbox id={`emergency_${index}`} checked={guardian.is_emergency_contact} onCheckedChange={(val) => guardian.is_emergency_contact = !!val} />
                      <Label for={`emergency_${index}`} class="text-xs cursor-pointer">{m.form_guardian_role_emergency()}</Label>
                    </div>

                    <div class="flex items-center gap-2">
                      <Checkbox id={`payer_${index}`} checked={guardian.is_payer} onCheckedChange={(val) => guardian.is_payer = !!val} />
                      <Label for={`payer_${index}`} class="text-xs cursor-pointer">{m.form_guardian_role_payer()}</Label>
                    </div>

                    {#if guardian.is_payer}
                      <div class="flex items-center gap-1.5">
                        <Label class="text-[10px] text-muted-foreground">{m.form_guardian_payment_share()}</Label>
                        <Input type="number" min="0" max="100" class="h-7 w-16 text-center font-bold px-1 text-xs border border-input" bind:value={guardian.payment_share_percent} />
                        <span class="text-[10px] text-muted-foreground">%</span>
                      </div>
                    {/if}
                  </div>

                </div>
              </div>
            {/each}
          </CardContent>
        </Card>
      </TabsContent>

      <!-- TAB 3: Location & Education -->
      <TabsContent value="location">
        <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
          <CardHeader class="border-b border-border/60 pb-4">
            <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
              <MapPin class="text-primary h-4.5 w-4.5" />
              <span>العنوان والسكن والدراسة السابقة</span>
            </CardTitle>
            <CardDescription class="text-[11px] text-muted-foreground">
              تفاصيل النطاق الجغرافي للأسرة والخلفية الدراسية السابقة
            </CardDescription>
          </CardHeader>
          <CardContent class="p-6 space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- المدرسة السابقة -->
              {#if activeFields.previous_school}
                <div class="flex flex-col gap-2 text-start">
                  <Label class="text-[11px] font-semibold text-muted-foreground" for="previousSchool">
                    {m.form_previous_school()}
                  </Label>
                  <Input 
                    id="previousSchool"
                    type="text" 
                    placeholder={m.form_previous_school_placeholder()} 
                    class="text-xs border border-input" 
                    bind:value={previousSchool}
                  />
                </div>
              {/if}

              <!-- المحافظة -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="governorate">
                  {m.form_governorate()}
                </Label>
                <Input 
                  id="governorate"
                  type="text" 
                  placeholder={m.form_governorate_placeholder()} 
                  class="text-xs border border-input" 
                  bind:value={governorate}
                />
              </div>

              <!-- المديرية -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-[11px] font-semibold text-muted-foreground" for="district">
                  {m.form_district()}
                </Label>
                <Input 
                  id="district"
                  type="text" 
                  placeholder={m.form_district_placeholder()} 
                  class="text-xs border border-input" 
                  bind:value={district}
                />
              </div>

              <!-- السكن / العنوان الحالي -->
              {#if activeFields.address}
                <div class="flex flex-col gap-2 text-start md:col-span-2">
                  <Label class="text-[11px] font-semibold text-muted-foreground" for="address">
                    {m.form_address()} {#if requiredFields.address}<span class="text-destructive">{m.form_required()}</span>{/if}
                  </Label>
                  <Input 
                    id="address"
                    type="text" 
                    placeholder={m.form_address_placeholder()} 
                    class="text-xs border border-input" 
                    bind:value={address}
                  />
                </div>
              {/if}
            </div>
          </CardContent>
        </Card>
      </TabsContent>

      <!-- TAB 4: Financials & Discounts -->
      <TabsContent value="financials">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          
          <!-- Financial Form Configuration Panel -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Discounts & Grants section -->
            <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
              <CardHeader class="border-b border-border/60 pb-4">
                <CardTitle class="text-sm font-bold flex items-center justify-between text-foreground">
                  <span class="flex items-center gap-2">
                    <Percent class="text-primary h-4.5 w-4.5" />
                    <span>{m.form_discount_section()}</span>
                  </span>
                  <!-- Switch to apply discount -->
                  <div class="flex items-center gap-2 cursor-pointer">
                    <Label class="text-[10px] font-semibold text-muted-foreground cursor-pointer" for="applyDiscountSwitch">
                      {m.form_apply_discount()}
                    </Label>
                    <Switch id="applyDiscountSwitch" bind:checked={applyDiscount} />
                  </div>
                </CardTitle>
                <CardDescription class="text-[11px] text-muted-foreground">
                  تطبيق منح دراسية أو خصومات مخصصة للطلاب (أقارب الموظفين، إخوة، أيتام، إلخ)
                </CardDescription>
              </CardHeader>
              
              {#if applyDiscount}
                <CardContent class="p-6 space-y-4">
                  <div class="flex gap-4 items-center">
                    <Button
                      variant="ghost"
                      class="flex-1 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {discountType === 'template' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                      onclick={() => discountType = 'template'}
                    >
                      <span>{m.form_discount_type_template()}</span>
                    </Button>
                    <Button 
                      variant="ghost"
                      class="flex-1 border rounded-lg text-xs h-8 cursor-pointer transition-all duration-150 {discountType === 'custom' ? 'border-primary bg-primary/5 font-semibold text-primary' : 'border-border bg-card/20 text-muted-foreground hover:bg-muted/30'}"
                      onclick={() => discountType = 'custom'}
                    >
                      <span>{m.form_discount_type_custom()}</span>
                    </Button>
                  </div>

                  {#if discountType === 'template'}
                    <div class="flex flex-col gap-2 text-start">
                      <Label class="text-[11px] font-semibold text-muted-foreground">اختر قالب الخصم</Label>
                      <Select bind:value={selectedDiscountTemplateId} type="single">
                        <SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input">
                          <span class="truncate">
                            {discountTemplates.find(d => d.id === selectedDiscountTemplateId)?.name || "قالب الخصم"}
                          </span>
                        </SelectTrigger>
                        <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                          {#each discountTemplates as tpl}
                            <SelectItem value={tpl.id} class="text-xs text-start cursor-pointer justify-start">
                              {tpl.name} ({tpl.value}{tpl.calculation_method === 'percentage' ? '%' : ' YER'})
                            </SelectItem>
                          {/each}
                        </SelectContent>
                      </Select>
                    </div>
                  {:else}
                    <div class="flex flex-col gap-2 text-start">
                      <Label class="text-[11px] font-semibold text-muted-foreground" for="customDiscount">{m.form_discount_custom_amount()}</Label>
                      <Input 
                        id="customDiscount"
                        type="number"
                        min="0"
                        placeholder="أدخل مبلغ الخصم المباشر بالريال اليمني"
                        class="text-xs border border-input"
                        bind:value={customDiscountAmount}
                      />
                    </div>
                  {/if}
                </CardContent>
              {/if}
            </Card>

            <!-- Optional Custom Fees list -->
            <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
              <CardHeader class="border-b border-border/60 pb-4">
                <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
                  <CreditCard class="text-primary h-4.5 w-4.5" />
                  <span>{m.form_optional_fees_section()}</span>
                </CardTitle>
                <CardDescription class="text-[11px] text-muted-foreground">
                  اختر رسوم الخدمات المضافة والمنتجات التي ترغب في تسجيلها على الطالب حالاً
                </CardDescription>
              </CardHeader>
              <CardContent class="p-6 space-y-4">
                {#if optionalFeesList.length === 0}
                  <p class="text-xs text-muted-foreground text-center py-4">
                    لا توجد رسوم إضافية مسجلة كـ "اختيارية" في النظام حالياً.
                  </p>
                {:else}
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    {#each optionalFeesList as fTpl}
                      {@const isChecked = selectedOptionalFees[fTpl.id]?.checked || false}
                      <div class="flex items-center justify-between p-3 border border-border/80 rounded-xl bg-card/20 hover:bg-card/40 transition-all duration-150 gap-4">
                        <div class="flex items-start gap-2.5 text-start">
                          <Checkbox 
                            id={`fee_${fTpl.id}`} 
                            checked={isChecked}
                            onCheckedChange={(checked) => {
                              selectedOptionalFees[fTpl.id].checked = !!checked;
                            }}
                          />
                          <div class="flex flex-col gap-0.5">
                            <Label for={`fee_${fTpl.id}`} class="text-xs font-bold text-foreground cursor-pointer">
                              {fTpl.name_ar || fTpl.name}
                            </Label>
                            <span class="text-[10px] text-muted-foreground capitalize">
                              {fTpl.category}
                            </span>
                          </div>
                        </div>

                        <!-- Amount override field -->
                        <div class="w-24 shrink-0 flex items-center gap-1">
                          <Input 
                            type="number"
                            disabled={!isChecked}
                            class="text-xs h-7 text-center font-bold px-1 border border-input"
                            bind:value={selectedOptionalFees[fTpl.id].amount}
                          />
                          <span class="text-[10px] text-muted-foreground">ر.ي</span>
                        </div>
                      </div>
                    {/each}
                  </div>
                {/if}
              </CardContent>
            </Card>
          </div>

          <!-- Live Receipt Summary Panel (Fintech 2026 Style) -->
          <div class="space-y-6">
            <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md sticky top-6">
              <CardHeader class="border-b border-border/60 pb-4">
                <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
                  <Sparkles class="text-primary h-4.5 w-4.5" />
                  <span>{m.form_summary_section()}</span>
                </CardTitle>
                <CardDescription class="text-[11px] text-muted-foreground">
                  تفاصيل الفاتورة والقيم المالية المحسوبة ديناميكياً
                </CardDescription>
              </CardHeader>
              <CardContent class="p-6 space-y-4 text-start">
                
                <!-- Mandatory Fees Total -->
                <div class="flex justify-between items-center text-xs">
                  <span class="text-muted-foreground">{m.form_summary_mandatory()}:</span>
                  <span class="font-bold text-foreground">{mandatoryTotal.toLocaleString()} ريال</span>
                </div>

                <!-- Selected Optional Fees Total -->
                <div class="flex justify-between items-center text-xs">
                  <span class="text-muted-foreground">{m.form_summary_optional()}:</span>
                  <span class="font-bold text-foreground">+{optionalTotal.toLocaleString()} ريال</span>
                </div>

                <!-- Discounts Total -->
                <div class="flex justify-between items-center text-xs">
                  <span class="text-muted-foreground">{m.form_summary_discount()}:</span>
                  <span class="font-bold text-destructive">-{discountTotal.toLocaleString()} ريال</span>
                </div>

                <div class="py-1">
                  <Separator class="bg-border/60" />
                </div>

                <!-- Final Net Fees Required -->
                <div class="flex justify-between items-center">
                  <span class="text-sm font-bold text-foreground">{m.form_summary_net()}:</span>
                  <span class="text-base font-extrabold text-primary">{netTotal.toLocaleString()} ريال يمني</span>
                </div>

                <!-- Fee breakdown visualizer -->
                {#if mandatoryFeesList.length > 0}
                  <div class="mt-4 p-3 rounded-xl bg-muted/20 border border-border/60 space-y-1.5">
                    <span class="text-[10px] font-bold text-muted-foreground block border-b border-border/40 pb-1">
                      تفاصيل الرسوم الأساسية المعتمدة للصف:
                    </span>
                    {#each mandatoryFeesList as feeTpl}
                      <div class="flex justify-between items-center text-[10px]">
                        <span class="text-muted-foreground truncate max-w-[150px]">
                          {feeTpl.name_ar || feeTpl.name}
                        </span>
                        <span class="font-mono text-foreground">{feeTpl.default_amount.toLocaleString()} ر.ي</span>
                      </div>
                    {/each}
                  </div>
                {/if}
              </CardContent>
            </Card>
          </div>
        </div>
      </TabsContent>

      <!-- TAB 5: Documents Status -->
      <TabsContent value="documents">
        <Card class="bg-card/50 border border-border/80 rounded-2xl shadow-xl backdrop-blur-md">
          <CardHeader class="border-b border-border/60 pb-4">
            <CardTitle class="text-sm font-bold flex items-center gap-2 text-foreground">
              <FileText class="text-primary h-4.5 w-4.5" />
              <span>{m.form_documents_title()}</span>
            </CardTitle>
            <CardDescription class="text-[11px] text-muted-foreground">
              {m.form_documents_hint()}
            </CardDescription>
          </CardHeader>
          <CardContent class="p-6 space-y-6">
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
              {#each docTypes as doc (doc.id)}
                {@const isChecked = submittedDocs.includes(doc.id)}
                <Button
                  variant="ghost"
                  class="flex items-center justify-between p-3 h-auto border rounded-xl cursor-pointer text-start text-xs transition-all duration-150 {isChecked ? 'border-primary bg-primary/5 text-foreground font-semibold' : 'border-border bg-card/10 text-muted-foreground hover:bg-muted/30'}"
                  onclick={() => toggleDoc(doc.id)}
                >
                  <span>{doc.label}</span>
                  <span class="text-primary flex items-center justify-center">
                    {#if isChecked}
                      <CheckSquare size={16} />
                    {:else}
                      <Square size={16} class="opacity-40" />
                    {/if}
                  </span>
                </Button>
              {/each}
            </div>
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>

    <!-- Footer Action Bar with Template Selection, Status Choice, and Save/Cancel Actions -->
    <div class="flex flex-col sm:flex-row items-center justify-between gap-4 mt-6 p-4 border border-border/80 bg-card/30 rounded-2xl backdrop-blur-md">
      
      <!-- Template Selector (Left side) -->
      <div class="flex items-center gap-2 w-full sm:w-auto">
        <Label class="text-[10px] font-semibold text-muted-foreground shrink-0 hidden md:inline">
          {m.form_select_template()}:
        </Label>
        <Select bind:value={selectedTemplateId} type="single">
          <SelectTrigger class="w-full sm:w-56 h-9 text-xs flex items-center justify-between border border-input" id="templateSelect">
            <span class="truncate">
              {availableTemplates.find(t => t.id === selectedTemplateId)?.name_ar || 
               availableTemplates.find(t => t.id === selectedTemplateId)?.name || 
               "استمارة تسجيل طالب"}
            </span>
          </SelectTrigger>
          <SelectContent class="text-start max-h-48 overflow-y-auto" align="start">
            {#each availableTemplates as tpl}
              <SelectItem value={tpl.id} class="text-xs cursor-pointer text-start justify-start">
                {tpl.name_ar || tpl.name}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Action buttons & Lifecycle State Selection -->
      <div class="flex items-center justify-end gap-3 w-full sm:w-auto shrink-0 flex-nowrap">
        
        <!-- Print preview -->
        <Button 
          variant="outline" 
          class="text-xs h-9 px-4 gap-1.5 border-primary/30 text-primary hover:bg-primary/5 hover:text-primary cursor-pointer font-semibold shrink-0" 
          onclick={handleDownloadRegistrationForm}
          disabled={isSaving || isDownloadingForm}
        >
          <Printer size={14} class={isDownloadingForm ? 'animate-spin' : ''} />
          <span class="hidden xs:inline">
            {isDownloadingForm ? m.form_printing_registration_form() : m.form_print_registration_form()}
          </span>
        </Button>

        <!-- Lifecycle selection -->
        {#if !isEditMode}
          <Select bind:value={registrationStatus} type="single">
            <SelectTrigger class="h-9 text-xs border border-input min-w-36">
              <span>
                {registrationStatus === "draft" ? m.form_save_draft() : 
                 registrationStatus === "pending_review" ? m.form_submit_review() : 
                 m.form_direct_enroll()}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="draft">{m.form_save_draft()}</SelectItem>
              <SelectItem value="pending_review">{m.form_submit_review()}</SelectItem>
              <SelectItem value="active">{m.form_direct_enroll()}</SelectItem>
            </SelectContent>
          </Select>
        {:else}
          <Select bind:value={registrationStatus} type="single">
            <SelectTrigger class="h-9 text-xs border border-input min-w-36">
              <span>
                {registrationStatus === "active" ? m.form_status_active() :
                 registrationStatus === "pending_review" ? m.form_status_pending_review() :
                 registrationStatus === "draft" ? m.form_status_draft() :
                 registrationStatus === "withdrawn" ? m.form_status_withdrawn() :
                 registrationStatus === "graduated" ? m.form_status_graduated() :
                 registrationStatus}
              </span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="draft">{m.form_status_draft()}</SelectItem>
              <SelectItem value="pending_review">{m.form_status_pending_review()}</SelectItem>
              <SelectItem value="approved">{m.form_status_approved()}</SelectItem>
              <SelectItem value="active">{m.form_status_active()}</SelectItem>
              <SelectItem value="withdrawn">{m.form_status_withdrawn()}</SelectItem>
              <SelectItem value="graduated">{m.form_status_graduated()}</SelectItem>
            </SelectContent>
          </Select>
        {/if}

        <Button 
          variant="outline" 
          class="text-xs h-9 px-4 cursor-pointer font-semibold shrink-0" 
          onclick={() => goto(localizeHref(`/${orgId}/students`))}
          disabled={isSaving}
        >
          {m.form_cancel()}
        </Button>
        
        <Button 
          variant="lime" 
          class="text-xs h-9 px-5 gap-1.5 cursor-pointer font-bold shrink-0" 
          onclick={handleSave}
          disabled={isSaving}
        >
          <Save size={14} />
          {isSaving ? m.form_saving() : m.form_save()}
        </Button>
      </div>
    </div>

  </div>
</div>
