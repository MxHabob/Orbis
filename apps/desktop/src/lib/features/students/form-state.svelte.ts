import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import type { QueryClient } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  extractErrorMessage,
  type StudentDetailDto,
  type ClassDto,
  type TemplateDto,
} from "$lib/api";
import { toast } from "@orbis/ui";

import { formatDate, formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import { studentRegistrationSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

import { PersonalInfoState } from "./state/personal-info.svelte";
import { GuardianState } from "./state/guardian-info.svelte";
import { FinancialInfoState } from "./state/financial-info.svelte";
import { DocumentInfoState } from "./state/document-info.svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";
import { orgContext } from "$lib/stores/org-context.svelte";

export class StudentFormState {
  orgId: string;
  studentId?: string;
  isEditMode = $state(false);

  // Sub-states
  personal: PersonalInfoState;
  guardian: GuardianState;
  financial: FinancialInfoState;
  document: DocumentInfoState;
  originalStudent: StudentDetailDto | null = null;

  // Configuration from Org Settings
  currency = $state(orgContext.baseCurrency || "YER");
  studyMode = $state<"regular" | "part_time" | "distance_learning" | "homeschooled" | "auditor" | "visiting" | "exchange" | "special_needs">("regular");
  avatarUrl = $state<string | null>(null);

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

  // Class & Academic Data
  classes = $state<ClassDto[]>([]);

  // Page level indicators
  isSaving = $state(false);
  orgName = $state("");
  isDownloadingForm = $state(false);

  registrationForm = new FormController({
    initialValues: {
      name: "",
      name_en: null as string | null,
      email: null as string | null,
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

  // Printing templates & staged media files
  availableTemplates = $state<TemplateDto[]>([]);
  selectedTemplateId = $state("");
  avatarFile = $state<File | null>(null);

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
      const [org, classRes] = await Promise.all([
        api.getCurrentOrg().catch(() => null),
        api.listClasses().catch(() => []),
        this.financial.init(),
        this.document.loadRequirements(),
      ]);

      if (classRes && Array.isArray(classRes)) {
        this.classes = classRes;
      }

      if (org) {
        this.orgName = String(org.name || org.name_ar || "");
        const baseCurr = org.base_currency_code || orgContext.baseCurrency;
        if (baseCurr) {
          this.currency = baseCurr;
        }

        // Parse organization registration configuration
        let settingsObj: Record<string, unknown> = {};
        try {
          settingsObj = typeof org.settings === "string" 
            ? JSON.parse(org.settings) 
            : (org.settings as Record<string, unknown>) || {};
        } catch {
          settingsObj = {};
        }

        const reg = (settingsObj.registration || settingsObj.registration_config || {}) as Record<string, unknown>;
        const formFieldsList = (reg.form_fields_list || reg.formFieldsList) as Array<{ key: string; isActive?: boolean; isRequired?: boolean; preventDuplicate?: boolean }> | undefined;
        if (formFieldsList && Array.isArray(formFieldsList)) {
          const activeMap: Record<string, boolean> = {};
          const requiredMap: Record<string, boolean> = {};
          const duplicateMap: Record<string, boolean> = {};

          for (const field of formFieldsList) {
            activeMap[field.key] = field.isActive !== false;
            requiredMap[field.key] = field.isActive !== false && field.isRequired === true;
            duplicateMap[field.key] = field.isActive !== false && field.preventDuplicate === true;
          }

          this.activeFields = { ...this.activeFields, ...activeMap };
          this.requiredFields = { ...this.requiredFields, ...requiredMap };
          this.preventDuplicates = { ...this.preventDuplicates, ...duplicateMap };
        }

        const reqDocs = (reg.required_documents_list || reg.requiredDocumentsList) as Array<{ nameAr?: string; nameEn?: string; name?: string }> | undefined;
        if (reqDocs && Array.isArray(reqDocs)) {
          this.document.setRequiredDocuments(reqDocs.map((d) => d.nameAr || d.nameEn || d.name || ""));
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
        let originalStudent: StudentDetailDto | null = null;
        try {
          originalStudent = await api.getStudent(this.studentId);
        } catch (err) {
          console.error("Failed to load student by ID from backend:", err);
        }
        this.originalStudent = originalStudent;

        if (originalStudent) {
          this.registrationStatus = originalStudent.student?.status || "active";
          this.studyMode = (originalStudent.enrollment?.study_mode as typeof this.studyMode) || "regular";
          this.personal.populate(originalStudent);
          await this.financial.populate(originalStudent);
          await this.guardian.populate();
          await this.document.populate(originalStudent);
        }
      }
    } catch (e: unknown) {
      console.error("Failed to initialize student form state:", e);
      toast.error(
        m.common_form_initialization_error({
          error: extractErrorMessage(e),
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
          const match = res.matches.find((m: any) => m.field === field) || res.matches[0];
          const studentInfo = match.student_number ? ` (${match.student_number})` : "";
          this.duplicateWarnings = {
            ...this.duplicateWarnings,
            [field]: m.operations_ui_duplicate_person_value({
              name: match.matched_person_name,
              details: studentInfo,
            }),
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

  // Selected Class details & Capacity
  get selectedClass() {
    return this.classes.find((c) => String(c.id) === this.personal.classId) || null;
  }

  get classLevel() {
    return this.selectedClass?.level || "";
  }

  get selectedClassName() {
    const cls = this.selectedClass;
    if (!cls) return m.common_form_select_class();
    return cls.name || String(cls.id);
  }

  get mandatoryFeesList() {
    const level = this.classLevel;
    const classId = this.personal.classId;
    return this.financial.feeTemplates.filter((t) => {
      if (!t.is_mandatory) return false;

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
        const val = Number(tpl.value) || 0;
        if (tpl.discount_type === "percentage") {
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
    return this.financial.feeTemplates.filter((t) => !t.is_mandatory);
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
  async handleSave(queryClient?: QueryClient) {
    const displayName = (this.personal.nameAr || this.personal.nameEn || "").trim();
    if (!displayName) {
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
      toast.error(m.ui_copy_please_resolve_duplicate_warnings_before_p_d77b26cb());
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
          student_number: this.personal.studentNumber || undefined,
          first_name: firstName,
          middle_name: middleName,
          last_name: lastName,
          first_name_ar: firstNameAr,
          middle_name_ar: middleNameAr,
          last_name_ar: lastNameAr,
          display_name: displayName,
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
          initial_period_id: effectiveContext.currentPeriodId || undefined,
          initial_class_id: this.personal.classId || undefined,
          study_mode: this.studyMode,
        });

        targetStudentId = detail.student.id;
        if (detail.enrollment?.id) {
          targetEnrollmentId = detail.enrollment.id;
        }
        toast.success(m.common_form_success_create());
      } else {
        targetStudentId = this.originalStudent?.student?.id || this.studentId!;
        targetEnrollmentId = this.originalStudent?.enrollment?.id || "";

        const namePartsAr = (this.personal.nameAr || "").trim().split(/\s+/);
        const namePartsEn = (this.personal.nameEn || "").trim().split(/\s+/);

        const firstName = namePartsEn[0] || namePartsAr[0] || "طالب";
        const lastName = namePartsEn.length > 1 ? namePartsEn[namePartsEn.length - 1] : (namePartsAr.length > 1 ? namePartsAr[namePartsAr.length - 1] : "جديد");
        const middleName = namePartsEn.length > 2 ? namePartsEn.slice(1, -1).join(" ") : undefined;

        const firstNameAr = namePartsAr[0] || undefined;
        const lastNameAr = namePartsAr.length > 1 ? namePartsAr[namePartsAr.length - 1] : undefined;
        const middleNameAr = namePartsAr.length > 2 ? namePartsAr.slice(1, -1).join(" ") : undefined;

        await api.updateStudentProfile({
          student_id: targetStudentId,
          first_name: firstName,
          middle_name: middleName,
          last_name: lastName,
          first_name_ar: firstNameAr,
          middle_name_ar: middleNameAr,
          last_name_ar: lastNameAr,
          display_name: displayName,
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
          address: this.personal.address || undefined,
        });

        toast.success(m.common_form_success_update());
      }

      // 2. Link existing guardians
      for (const guardian of this.guardian.linkedGuardians) {
        if (!guardian.guardian_id || !targetStudentId) continue;

        await api
          .assignGuardian({
            student_id: targetStudentId,
            guardian_id: guardian.guardian_id,
            relationship: guardian.relationship || "other",
            relationship_ar: guardian.relationship_ar || "آخر",
            is_primary: !!guardian.is_primary,
            is_emergency_contact: !!guardian.is_emergency_contact,
            is_authorized_pickup: !!guardian.is_authorized_pickup,
            is_payer: !!guardian.is_payer,
            payment_share_percent: guardian.payment_share_percent ?? 0,
          })
          .catch((err) => console.warn("Assign guardian error:", err));
      }

      // 3. Assign optional fees
      const activePeriod = effectiveContext.currentPeriodId;
      const today = new Date().toISOString().slice(0, 10);
      if (activePeriod && targetStudentId && targetEnrollmentId) {
        for (const key in this.financial.selectedOptionalFees) {
          if (this.financial.selectedOptionalFees[key].checked) {
            try {
              await api.generateStudentFee({
                student_id: targetStudentId,
                enrollment_id: targetEnrollmentId,
                fee_template_id: key,
                period_id: activePeriod,
                due_date: today,
              });
            } catch (feeErr) {
              console.error("Failed to assign optional fee:", key, feeErr);
            }
          }
        }
      }
      // 4. Upload staged document attachments and staged avatar photo with concrete studentId
      if (targetStudentId) {
        await this.document.uploadPendingAttachments(targetStudentId);

        if (this.avatarFile) {
          try {
            await api.uploadDocument({
              name: this.avatarFile.name,
              mime_type: this.avatarFile.type || "image/jpeg",
              entity_type: "student",
              entity_id: targetStudentId,
              category: "avatar",
              visibility: "public",
              file_path: this.avatarFile.name,
              file_size_bytes: this.avatarFile.size,
            });
          } catch (avatarErr) {
            console.warn("Failed to upload staged avatar on student save:", avatarErr);
          }
        }
      }

      // 5. If saving as pending_review, automatically create an admission approval request
      if (this.registrationStatus === "pending_review" && targetStudentId) {
        try {
          await api.submitOverride({
            override_type: "admission_approval",
            risk_level: "medium",
            entity_type: "student",
            entity_id: targetStudentId,
            reason: m.operations_ui_student_enrollment_review_reason({
              name: displayName,
              className: this.selectedClassName,
            }),
            field_name: "registration_approval",
            new_value: "approved",
            old_value: "pending_review",
          });
        } catch (overrideErr) {
          console.warn("Auto-submit admission approval request error:", overrideErr);
        }
      }

      if (queryClient) {
        queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
        queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
        queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
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
      toast.error(m.ui_copy_please_select_a_print_template_first_42eed155());
      return;
    }
    this.isDownloadingForm = true;
    try {
      const templateRecord = await api.getTemplate(this.selectedTemplateId);
      if (!templateRecord) {
        toast.error(
          m.ui_template_not_found({ templateId: this.selectedTemplateId }),
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
        mandatory_fees: formatCurrency(this.mandatoryTotal, this.currency),
        optional_fees: formatCurrency(this.optionalTotal, this.currency),
        discount_fees: formatCurrency(this.discountTotal, this.currency),
        net_fees: formatCurrency(this.netTotal, this.currency),
      };

      const pdfBytes = await api.previewTemplate(docJson, printVariables, "ar");

      if (pdfBytes && pdfBytes.length > 0) {
        const blob = new Blob([pdfBytes as unknown as BlobPart], {
          type: "application/pdf",
        });
        const pdfUrl = URL.createObjectURL(blob);

        const link = document.createElement("a");
        link.href = pdfUrl;
        link.download = `${m.operations_ui_student_form_filename()}_${this.personal.nameAr || m.operations_ui_student_fallback()}.pdf`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(pdfUrl);

        toast.success(m.ui_copy_document_generated_and_downloaded_successf_456c90b7());
      } else {
        toast.error(m.ui_copy_failed_to_generate_pdf_document_b7b11e81());
      }
    } catch (e: unknown) {
      console.error("Failed to download registration form:", e);
      toast.error(
        `${m.ui_copy_error_downloading_form_c76eff53()} ${extractErrorMessage(e)}`,
      );
    } finally {
      this.isDownloadingForm = false;
    }
  }
}
