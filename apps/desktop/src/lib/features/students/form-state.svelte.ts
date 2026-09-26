import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { api, queryKeys } from "$lib/api";
import { toast } from "@orbis/ui";

import { l } from "$lib/core/i18n";
import { formatDate } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import { studentRegistrationSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

import { PersonalInfoState } from "./state/personal-info.svelte";
import { GuardianState } from "./state/guardian-info.svelte";
import { FinancialInfoState } from "./state/financial-info.svelte";
import { DocumentInfoState } from "./state/document-info.svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";

export class StudentFormState {
  orgId: string;
  studentId?: string;
  isEditMode = $state(false);
  personal: PersonalInfoState;
  guardian: GuardianState;
  financial: FinancialInfoState;
  document: DocumentInfoState;
  originalStudent: any = null;

  requiredFields = $state<Record<string, boolean>>({
    national_id: true,
    email: false,
    phone: true,
    birth_date: true,
    sex: true,
    address: false,
    previous_school: false,
    documents: false,
  });
  activeFields = $state<Record<string, boolean>>({
    national_id: true,
    email: true,
    phone: true,
    birth_date: true,
    sex: true,
    address: true,
    previous_school: true,
    documents: true,
  });
  preventDuplicates = $state<Record<string, boolean>>({
    national_id: true,
    phone: false,
    email: false,
  });
  registrationStatus = $state("draft");

  // Warnings for duplicate values
  duplicateWarnings = $state<{
    national_id?: string;
    email?: string;
    phone?: string;
  }>({});

  private duplicateCheckTimers: Record<string, ReturnType<typeof setTimeout>> = {};

  // Class data for computations
  classes = $state<any[]>([]);

  // Page level indicators
  isSaving = $state(false);
  orgName = $state("");
  isDownloadingForm = $state(false);
  registrationForm = new FormController({
    initialValues: {
      name: "",
      name_en: null as string | null,
      email: null as string | null,
      password: "",
      phone: null as string | null,
      gender: null as string | null,
      birth_date: null as string | null,
      nationality: null as string | null,
      national_id: null as string | null,
      address: null as string | null,
      class_id: "",
      guardian_name: null as string | null,
      guardian_phone: null as string | null,
      guardian_relationship: null as string | null,
    },
    schema: studentRegistrationSchema,
  });

  // Printing templates
  availableTemplates = $state<any[]>([]);
  selectedTemplateId = $state("");

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
    this.isEditMode = !!studentId;

    this.personal = new PersonalInfoState();
    this.guardian = new GuardianState(orgId, studentId);
    this.financial = new FinancialInfoState(orgId, studentId);
    this.document = new DocumentInfoState(orgId, studentId);
  }

  async init() {
    try {
      const [org] = await Promise.all([
        api.getCurrentOrg().catch(() => null),
        this.financial.init(),
        this.document.loadRequirements(),
      ]);

      if (org) {
        const organization = org as any;
        this.orgName = String(organization.name || organization.name_ar || "");

        // Parse organization registration configuration
        let settingsObj: any = {};
        try {
          settingsObj = typeof organization.settings === "string" 
            ? JSON.parse(organization.settings) 
            : organization.settings || {};
        } catch {
          settingsObj = {};
        }

        const reg = settingsObj.registration || settingsObj.registration_config || {};
        if (reg.form_fields_list && Array.isArray(reg.form_fields_list)) {
          const activeMap: Record<string, boolean> = {};
          const requiredMap: Record<string, boolean> = {};
          const duplicateMap: Record<string, boolean> = {};

          for (const field of reg.form_fields_list) {
            activeMap[field.key] = field.isActive !== false;
            requiredMap[field.key] = field.isActive !== false && field.isRequired === true;
            duplicateMap[field.key] = field.isActive !== false && field.preventDuplicate === true;
          }

          this.activeFields = { ...this.activeFields, ...activeMap };
          this.requiredFields = { ...this.requiredFields, ...requiredMap };
          this.preventDuplicates = { ...this.preventDuplicates, ...duplicateMap };
        }

        if (reg.required_documents_list && Array.isArray(reg.required_documents_list)) {
          this.document.setRequiredDocuments(reg.required_documents_list.map((d: any) => d.nameAr || d.nameEn));
        }
      }

      // Load document templates for printing
      try {
        const templatesList = await api
          .listTemplates({ entityType: "student" })
          .catch(() => []);
        this.availableTemplates = templatesList || [];
        if (this.availableTemplates.length > 0) {
          this.selectedTemplateId = this.availableTemplates[0].id;
        }
      } catch (err) {
        console.error("Failed to load document templates:", err);
      }

      // In edit mode, load student profile
      if (this.isEditMode && this.studentId) {
        let originalStudent: any = null;
        try {
          originalStudent = await api.getEnrollment(this.studentId);
        } catch (err) {
          console.error("Failed to load student by ID from backend:", err);
        }
        this.originalStudent = originalStudent;

        if (originalStudent) {
          this.registrationStatus = originalStudent.status || "active";
          this.personal.populate(originalStudent);
          await this.financial.populate(originalStudent);
          await this.guardian.populate();
          await this.document.populate(originalStudent);
        }
      }
    } catch (e: any) {
      console.error("Failed to initialize student form state:", e);
      toast.error(
        m.common_form_initialization_error({
          error: e.message || String(e),
        }),
      );
    }
  }

  // Live debounced duplicate check triggers using authoritative backend API
  triggerDuplicateCheck(
    field: "national_id" | "email" | "phone",
    value: string,
  ) {
    if (this.duplicateCheckTimers[field]) {
      clearTimeout(this.duplicateCheckTimers[field]);
    }

    const trimmed = (value || "").trim();
    if (!trimmed) {
      this.duplicateWarnings = {
        ...this.duplicateWarnings,
        [field]: undefined,
      };
      return;
    }

    this.duplicateCheckTimers[field] = setTimeout(async () => {
      try {
        const res = await api.checkDuplicate({
          [field]: trimmed,
          exclude_student_id: this.studentId,
        });

        if (res.has_duplicate && res.matches.length > 0) {
          const match = res.matches.find((m) => m.field === field) || res.matches[0];
          const studentInfo = match.student_number ? ` (${match.student_number})` : "";
          this.duplicateWarnings = {
            ...this.duplicateWarnings,
            [field]: l(
              `هذه القيمة مسجلة مسبقاً للشخص "${match.matched_person_name}"${studentInfo}`,
              `Already registered to "${match.matched_person_name}"${studentInfo}`,
            ),
          };
        } else {
          this.duplicateWarnings = {
            ...this.duplicateWarnings,
            [field]: undefined,
          };
        }
      } catch (err) {
        console.error("Duplicate check error:", err);
      }
    }, 300);
  }

  // Financial Computations proxy
  get classLevel() {
    return (
      this.classes.find((c) => String(c.id) === this.personal.classId)?.level ||
      ""
    );
  }

  get selectedClassName() {
    const cls = this.classes.find(
      (c) => String(c.id) === this.personal.classId,
    );
    if (!cls) return m.common_form_select_class();
    return l(cls.name_ar, cls.name) || String(cls.id);
  }

  get mandatoryFeesList() {
    const level = this.classLevel;
    const classId = this.personal.classId;
    return this.financial.feeTemplates.filter((t) => {
      const isMandatory = t.is_mandatory === 1 || t.is_mandatory === true;
      const isActive = t.is_active === 1 || t.is_active === true;
      if (!isMandatory || !isActive) return false;

      if (t.applicable_levels) {
        try {
          const rules = typeof t.applicable_levels === "string" ? JSON.parse(t.applicable_levels) : t.applicable_levels;
          let targetLevels: string[] = [];
          if (Array.isArray(rules)) {
            targetLevels = rules.map(String);
          } else if (rules.levels && Array.isArray(rules.levels)) {
            targetLevels = rules.levels.map(String);
          } else if (rules.level) {
            targetLevels = [String(rules.level)];
          }

          if (targetLevels.length > 0) {
            const matchesLevel = level ? targetLevels.includes(level) : false;
            const matchesClassId = classId
              ? targetLevels.includes(classId)
              : false;
            return matchesLevel || matchesClassId;
          }
        } catch {
          return false;
        }
      }
      return true;
    });
  }

  get mandatoryTotal() {
    return this.mandatoryFeesList.reduce(
      (sum, t) => sum + (Number(t.default_amount) || 0),
      0,
    );
  }

  get discountTotal() {
    if (!this.financial.applyDiscount) return 0;
    if (
      this.financial.discountType === "template" &&
      this.financial.selectedDiscountTemplateId
    ) {
      const tpl = this.financial.discountTemplates.find(
        (t) => t.id === this.financial.selectedDiscountTemplateId,
      );
      if (tpl) {
        const val = (tpl.value as number) || 0;
        if (String(tpl.calculation_method).toLowerCase() === "percentage") {
          return (this.mandatoryTotal * val) / 100;
        } else {
          return val;
        }
      }
    } else if (
      this.financial.discountType === "custom" &&
      this.financial.customDiscountAmount
    ) {
      return this.financial.customDiscountAmount;
    }
    return 0;
  }

  get optionalFeesList() {
    return this.financial.feeTemplates.filter(
      (t) => (t.is_mandatory === 0 || t.is_mandatory === false) && (t.is_active === 1 || t.is_active === true),
    );
  }

  get optionalTotal() {
    let sum = 0;
    for (const key in this.financial.selectedOptionalFees) {
      if (this.financial.selectedOptionalFees[key].checked) {
        sum += this.financial.selectedOptionalFees[key].amount || 0;
      }
    }
    return sum;
  }

  get netTotal() {
    return Math.max(
      0,
      this.mandatoryTotal + this.optionalTotal - this.discountTotal,
    );
  }

  // Save operation
  async handleSave(queryClient?: any) {
    if (!this.personal.nameAr.trim()) {
      toast.error(m.common_form_error_name_required());
      return;
    }
    if (!this.personal.classId) {
      toast.error(m.common_form_error_class_required());
      return;
    }

    // Check duplicate blockers
    if (
      this.duplicateWarnings.national_id ||
      this.duplicateWarnings.phone ||
      this.duplicateWarnings.email
    ) {
      toast.error(l("يرجى معالجة تنبيهات تكرار البيانات قبل المتابعة", "Please resolve duplicate warnings before proceeding"));
      return;
    }

    this.isSaving = true;
    let targetStudentId = "";
    let targetEnrollmentId = this.studentId || "";

    try {
      if (!this.isEditMode) {
        // 1. Register Student & initial enrollment atomically via StudentService
        const namePartsAr = (this.personal.nameAr || "").trim().split(/\s+/);
        const namePartsEn = (this.personal.nameEn || "").trim().split(/\s+/);

        const firstName = namePartsEn[0] || namePartsAr[0] || "طالب";
        const lastName = namePartsEn.length > 1 ? namePartsEn[namePartsEn.length - 1] : (namePartsAr.length > 1 ? namePartsAr[namePartsAr.length - 1] : "جديد");
        const middleName = namePartsEn.length > 2 ? namePartsEn.slice(1, -1).join(" ") : undefined;

        const firstNameAr = namePartsAr[0] || undefined;
        const lastNameAr = namePartsAr.length > 1 ? namePartsAr[namePartsAr.length - 1] : undefined;
        const middleNameAr = namePartsAr.length > 2 ? namePartsAr.slice(1, -1).join(" ") : undefined;

        const detail = await api.registerStudent({
          student_number: this.personal.studentNumber || `STU-${Date.now()}`,
          first_name: firstName,
          middle_name: middleName,
          last_name: lastName,
          first_name_ar: firstNameAr,
          middle_name_ar: middleNameAr,
          last_name_ar: lastNameAr,
          display_name: this.personal.nameAr.trim() || this.personal.nameEn.trim(),
          national_id: this.personal.nationalId || undefined,
          passport_number: this.personal.passportNumber || undefined,
          gender: this.personal.sex || "male",
          birth_date: this.personal.birthDate || undefined,
          place_of_birth: this.personal.placeOfBirth || undefined,
          nationality: this.personal.nationality || undefined,
          blood_type: this.personal.bloodType || undefined,
          medical_notes: this.personal.medicalNotes || undefined,
          phone: this.personal.phone || undefined,
          email: this.personal.email.trim() || undefined,
          initial_period_id: effectiveContext.currentPeriodId || this.orgId,
          initial_class_id: this.personal.classId || undefined,
          study_mode: "regular",
        });

        targetStudentId = detail.student.id;
        toast.success(m.common_form_success_create());
      } else {
        targetEnrollmentId = this.studentId!;
        toast.success(m.common_form_success_update());
      }

      // 2. Save / link guardians
      for (const guardian of this.guardian.linkedGuardians) {
        if (!guardian.full_name.trim()) continue;
        let guardianUserId = guardian.user_id;

        if (!guardianUserId) {
          const guardEmail =
            guardian.email.trim() || `parent.${Date.now()}@orbis.local`;
          const guardUser = await api.registerUser({
            email: guardEmail,
            full_name: guardian.full_name.trim(),
            role: "parent",
          });
          guardianUserId = guardUser.id;
        }

        if (targetStudentId) {
          await api
            .assignGuardian({
              student_id: targetStudentId,
              guardian_id: guardianUserId,
              relationship: guardian.relationship || "parent",
              is_primary: !!guardian.is_primary,
            })
            .catch((err) => console.warn("Assign guardian error:", err));
        }
      }

      // 3. Assign optional fees
      const today = new Date().toISOString().slice(0, 10);
      for (const key in this.financial.selectedOptionalFees) {
        if (this.financial.selectedOptionalFees[key].checked) {
          try {
            await api.generateStudentFee({
              student_id: targetStudentId,
              enrollment_id: targetEnrollmentId,
              fee_template_id: key,
              period_id: this.orgId,
              due_date: today,
            });
          } catch (feeErr) {
            console.error("Failed to assign optional fee:", key, feeErr);
          }
        }
      }

      if (queryClient) {
        queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      }

      goto(localizeHref(`/${this.orgId}/students`));
    } catch (e) {
      console.error("Save error:", e);
      toast.error(
        m.common_form_error_save({
          error: e instanceof Error ? e.message : String(e),
        }),
      );
    } finally {
      this.isSaving = false;
    }
  }

  // Document print/download logic
  async handleDownloadRegistrationForm() {
    if (!this.selectedTemplateId) {
      toast.error(l("يرجى اختيار قالب الطباعة أولاً", "Please select a print template first"));
      return;
    }
    this.isDownloadingForm = true;
    try {
      const templateRecord = await api.getTemplate(this.selectedTemplateId);
      if (!templateRecord) {
        toast.error(
          l(`القالب '${this.selectedTemplateId}' غير موجود في قاعدة البيانات`, `Template '${this.selectedTemplateId}' not found`),
        );
        return;
      }
      const docJson = templateRecord.document_json;

      const printVariables = {
        orgName: this.orgName,
        student_name: this.personal.nameAr,
        name_ar: this.personal.nameAr,
        name_en: this.personal.nameEn,
        birth_date: this.personal.birthDate,
        sex: this.personal.sex === "male" ? "ذكر" : "أنثى",
        sex_en: this.personal.sex,
        class_name: this.selectedClassName,
        student_number: this.personal.studentNumber,
        email: this.personal.email,
        phone: this.personal.phone,
        address: this.personal.address,
        documents: this.document.submittedDocs.join(", "),
        current_date: formatDate(new Date()),
        mandatory_fees: this.mandatoryTotal.toString(),
        optional_fees: this.optionalTotal.toString(),
        discount_fees: this.discountTotal.toString(),
        net_fees: this.netTotal.toString(),
      };

      const pdfBytes = await api.previewTemplate(docJson, printVariables, "ar");

      if (pdfBytes && pdfBytes.length > 0) {
        const blob = new Blob([pdfBytes as unknown as BlobPart], {
          type: "application/pdf",
        });
        const pdfUrl = URL.createObjectURL(blob);

        const link = document.createElement("a");
        link.href = pdfUrl;
        link.download = `استمارة_${this.personal.nameAr || "طالب"}.pdf`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(pdfUrl);

        toast.success(l("تم توليد وتحميل المستند بنجاح", "Document generated and downloaded successfully"));
      } else {
        toast.error(l("فشل توليد ملف PDF", "Failed to generate PDF document"));
      }
    } catch (e: any) {
      console.error("Failed to download registration form:", e);
      toast.error(
        `${l("خطأ أثناء تحميل الاستمارة:", "Error downloading form:")} ${(e as any)?.message || String(e)}`,
      );
    } finally {
      this.isDownloadingForm = false;
    }
  }
}
