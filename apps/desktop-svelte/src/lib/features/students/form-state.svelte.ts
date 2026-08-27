import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

import { l } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";

import { PersonalInfoState } from "./state/personal-info.svelte";
import { GuardianState } from "./state/guardian-info.svelte";
import { FinancialInfoState } from "./state/financial-info.svelte";
import { DocumentInfoState } from "./state/document-info.svelte";

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
  preventDuplicates = $state<Record<string, boolean>>({});
  registrationStatus = $state("draft");

  // Warnings
  duplicateWarnings = $state<{
    national_id?: string;
    email?: string;
    phone?: string;
  }>({});

  // Class data for computations
  classes = $state<any[]>([]);

  // Page level indicators
  isSaving = $state(false);
  orgName = $state("");
  isDownloadingForm = $state(false);

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
        commands.organization.getOrganization(),
        this.financial.init(),
      ]);

      if (org) {
        const organization = org as any;
        this.orgName = String(organization.name || organization.name_ar || "");
        if (organization.settings) {
          try {
            const settings =
              typeof organization.settings === "string"
                ? JSON.parse(organization.settings)
                : (organization.settings || {});
            const reg = settings.registration || settings.registration_config || {};

            if (Array.isArray(reg.required_documents)) {
              this.document.setRequiredDocuments(reg.required_documents);
            }

            if (reg.active_fields) {
              if (Array.isArray(reg.active_fields)) {
                const actMap: Record<string, boolean> = {};
                for (const k of reg.active_fields) actMap[k] = true;
                this.activeFields = { ...this.activeFields, ...actMap };
              } else {
                this.activeFields = { ...this.activeFields, ...reg.active_fields };
              }
            }

            if (reg.required_fields) {
              if (Array.isArray(reg.required_fields)) {
                const reqMap: Record<string, boolean> = {};
                for (const k of reg.required_fields) reqMap[k] = true;
                this.requiredFields = { ...this.requiredFields, ...reqMap };
              } else {
                this.requiredFields = { ...this.requiredFields, ...reg.required_fields };
              }
            }

            if (reg.prevent_duplicates) {
              if (Array.isArray(reg.prevent_duplicates)) {
                const dupMap: Record<string, boolean> = {};
                for (const k of reg.prevent_duplicates) dupMap[k] = true;
                this.preventDuplicates = { ...this.preventDuplicates, ...dupMap };
              } else {
                this.preventDuplicates = {
                  ...this.preventDuplicates,
                  ...reg.prevent_duplicates,
                };
              }
            }
          } catch (err) {
            console.error("Failed to parse settings JSON:", err);
          }
        }
      }

      // Load document templates for printing
      try {
        const templatesList: any[] = [];
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
          originalStudent = await commands.academic.getEnrollment(this.studentId);
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

  // Live duplicate check triggers
  async triggerDuplicateCheck(
    field: "national_id" | "email" | "phone",
    value: string,
  ) {
    if (!value.trim() || !this.preventDuplicates[field]) return;
    try {
      const duplicates = await commands.iam.search(value);
      if (duplicates && duplicates.length > 0) {
        const match = duplicates[0];
        this.duplicateWarnings = {
          ...this.duplicateWarnings,
          [field]: m.common_form_guardian_duplicate_warning({
            name: match.full_name,
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
  }

  // Financial Computations proxy
  get classLevel() {
    return (
      this.classes.find((c) => String(c.id) === this.personal.classId)?.level ||
      ""
    );
  }

  get selectedClassName() {
    const cls = this.classes.find((c) => String(c.id) === this.personal.classId);
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
          const rules = JSON.parse(t.applicable_levels);
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
            const matchesClassId = classId ? targetLevels.includes(classId) : false;
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
    return this.mandatoryFeesList.reduce((sum, t) => sum + t.default_amount, 0);
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
      (t) => t.is_mandatory === 0 && t.is_active === 1,
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
    if (this.activeFields?.national_id && this.requiredFields?.national_id && !this.personal.nationalId.trim()) {
      toast.error(m.common_form_error_national_id_required());
      return;
    }
    if (this.activeFields?.email && this.requiredFields?.email && !this.personal.email.trim()) {
      toast.error(`${m.common_form_student_email()} مطلوب`);
      return;
    }
    if (this.activeFields?.phone && this.requiredFields?.phone && !this.personal.phone.trim()) {
      toast.error(`${m.common_form_student_phone()} مطلوب`);
      return;
    }
    if (this.activeFields?.birth_date && this.requiredFields?.birth_date && !this.personal.birthDate) {
      toast.error(`${m.common_bulk_col_dob()} مطلوب`);
      return;
    }
    if (this.activeFields?.sex && this.requiredFields?.sex && !this.personal.sex) {
      toast.error(`${m.common_form_sex()} مطلوب`);
      return;
    }
    if (this.activeFields?.address && this.requiredFields?.address && !this.personal.address.trim()) {
      toast.error(`${m.common_form_address()} مطلوب`);
      return;
    }
    if (this.activeFields?.previous_school && this.requiredFields?.previous_school && !this.personal.previousSchool.trim()) {
      toast.error(`${m.common_form_previous_school()} مطلوب`);
      return;
    }

    this.isSaving = true;
    let targetStudentId = "";
    let targetEnrollmentId = this.studentId || "";

    try {
      // Verify control account
      const accounts = await commands.finance.listAccounts();
      const hasControlAccount = accounts.some((acc) => acc.code === "110301");
      if (!hasControlAccount) {
        toast.error(m.common_form_error_control_account_missing());
        this.isSaving = false;
        return;
      }

      const studentUserData = {
        full_name: this.personal.nameAr.trim(),
        name_ar: this.personal.nameAr.trim(),
        name_en: this.personal.nameEn.trim() || null,
        email: this.personal.email.trim() || null,
        phone: this.personal.phone.trim() || null,
        sex: this.personal.sex,
        date_of_birth: this.personal.birthDate || null,
        national_id: this.personal.nationalId.trim() || null,
        address: this.personal.address.trim() || null,
        status: "active",
      };

      if (this.isEditMode) {
        const originalStudent = this.originalStudent;
        if (!originalStudent)
          throw new Error(m.common_form_error_enrollment_not_found());

        targetStudentId = String(originalStudent.student_id);
        await commands.iam.updateUser(targetStudentId, studentUserData);

        if (originalStudent.class_id !== this.personal.classId) {
          await commands.academic.transfer(this.studentId!, this.personal.classId);
        }

        if (originalStudent.status !== this.registrationStatus) {
          await commands.academic.withdraw(this.studentId!);
        }
        toast.success(m.common_form_success_update());
      } else {
        const firstGuardian = this.guardian.linkedGuardians[0];

        const payload = {
          name: this.personal.nameAr.trim() || this.personal.nameEn.trim() || "طالب جديد",
          email: this.personal.email.trim() || null,
          phone: this.personal.phone.trim() || null,
          gender: this.personal.sex || null,
          birth_date: this.personal.birthDate || null,
          national_id: this.personal.nationalId.trim() || null,
          address: this.personal.address.trim() || null,
          class_id: this.personal.classId || null,
          guardian_name: firstGuardian ? firstGuardian.full_name.trim() : null,
          guardian_phone: firstGuardian ? firstGuardian.phone.trim() : null,
          guardian_relationship: firstGuardian
            ? firstGuardian.relationship
            : null,
        };

        const res: any = await commands.enrollment_workflow.registerStudent(payload);
        targetStudentId = String(res.user_id);
        targetEnrollmentId = String(res.enrollment_id);

        toast.success(m.common_form_success_create());
      }

      // Save multi-guardian links
      let guardiansToProcess = this.guardian.linkedGuardians;
      if (!this.isEditMode && this.guardian.linkedGuardians.length > 0) {
        guardiansToProcess = this.guardian.linkedGuardians.slice(1);
      }

      for (const guardian of guardiansToProcess) {
        if (!guardian.full_name.trim()) continue;
        let guardianUserId = guardian.user_id;

        if (!guardianUserId) {
          const checkExisting = await commands.academic.search(guardian.phone);
          if (checkExisting && checkExisting.length > 0) {
            guardianUserId = checkExisting[0].id;
          } else {
            const guardEmail =
              guardian.email.trim() || `parent.${Date.now()}@orbis.local`;
            const guardUser = await commands.iam.createUser({
                email: guardEmail,
                full_name: guardian.full_name.trim(),
                primary_role: "parent",
                phone: guardian.phone.trim() || null,
              password_hash: null,
            });
            guardianUserId = guardUser.id;
          }
        } else {
          await commands.iam.updateUser(guardianUserId!, {
            full_name: guardian.full_name.trim(),
            phone: guardian.phone.trim() || null,
          });
        }

        const existingLinks = await commands.academic.listGuardiansByStudent(
          targetStudentId,
        );
        const matchedLink = existingLinks.find(
          (l) => l.guardian_id === guardianUserId,
        );

        const linkRequest = {
          student_id: targetStudentId,
          guardian_id: guardianUserId,
          relationship: guardian.relationship,
          relationship_ar: guardian.relationship_ar || guardian.relationship,
          is_primary: !!guardian.is_primary,
          is_emergency_contact: !!guardian.is_emergency_contact,
          is_authorized_pickup: false,
          occupation: guardian.occupation?.trim() || null,
          is_payer: !!guardian.is_payer,
          payment_share_percent: guardian.payment_share_percent,
          can_view_grades: true,
          can_view_attendance: true,
          can_view_behavior: true,
          can_view_finance: !!guardian.is_payer,
          can_view_schedule: true,
          can_receive_notifications: true,
        };

        if (matchedLink) {
          await commands.academic.updateGuardian(
            String(matchedLink.id),
            linkRequest,
          );
        } else {
          await commands.academic.createGuardian(linkRequest);
        }
      }

      // Assign optional/custom fees (only if status is active/approved)
      if (
        this.registrationStatus === "active" ||
        this.registrationStatus === "approved"
      ) {
        const today = new Date().toISOString().slice(0, 10);
        for (const key in this.financial.selectedOptionalFees) {
          if (this.financial.selectedOptionalFees[key].checked) {
            try {
              await commands.finance.assign({
                enrollment_id: targetEnrollmentId,
                fee_template_id: key,
                amount: this.financial.selectedOptionalFees[key].amount,
                due_date: today,
                notes: "رسوم إضافية مضافة عند التسجيل",
              } as any);
            } catch (feeErr) {
              console.error("Failed to assign optional fee:", key, feeErr);
            }
          }
        }
      }

      goto(localizeHref(`/${this.orgId}/students`));
    } catch (e: any) {
      if (targetStudentId && !this.isEditMode) {
        try {
          await commands.iam.deleteUser(targetStudentId);
        } catch (deleteErr) {
          console.error("Failed to rollback student user creation:", deleteErr);
        }
      }
      console.error("Save error:", e);
      toast.error(m.common_form_error_save({ error: e.message || String(e) }));
    } finally {
      this.isSaving = false;
    }
  }

  // Document print/download logic
  async handleDownloadRegistrationForm() {
    if (!this.selectedTemplateId) {
      toast.error("يرجى اختيار قالب الطباعة أولاً");
      return;
    }
    this.isDownloadingForm = true;
    try {
      const templateRecord = await commands.docforge.getTemplate(
        this.selectedTemplateId,
      );
      if (!templateRecord) {
        toast.error(
          `القالب '${this.selectedTemplateId}' غير موجود في قاعدة البيانات`,
        );
        return;
      }
      const documentModel = JSON.parse((templateRecord as any).document_json as string);

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
        current_date: new Date().toLocaleDateString("ar-YE"),
        mandatory_fees: this.mandatoryTotal.toString(),
        optional_fees: this.optionalTotal.toString(),
        discount_fees: this.discountTotal.toString(),
        net_fees: this.netTotal.toString(),
      };

      // const pdfBytes = await commands.docforge.previewDocumentTemplate(
      //   JSON.stringify(documentModel),
      //   JSON.stringify(printVariables),
      // );
      const pdfBytes: number[] = [];
      toast.error("Preview is temporarily disabled in the backend.");

      if (pdfBytes && pdfBytes.length > 0) {
        let binary = "";
        const len = pdfBytes.length;
        for (let i = 0; i < len; i++)
          binary += String.fromCharCode(pdfBytes[i]);
        const base64 = window.btoa(binary);
        const pdfUrl = `data:application/pdf;base64,${base64}`;

        const link = document.createElement("a");
        link.href = pdfUrl;
        link.download = `استمارة_${this.personal.nameAr || "طالب"}.pdf`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        toast.success("تم توليد وتحميل المستند بنجاح");
      } else {
        toast.error("Failed to generate PDF document");
      }
    } catch (e: any) {
      console.error("Failed to download registration form:", e);
      toast.error(
        `خطأ أثناء تحميل الاستمارة: ${(e as any)?.message || String(e)}`,
      );
    } finally {
      this.isDownloadingForm = false;
    }
  }
}
