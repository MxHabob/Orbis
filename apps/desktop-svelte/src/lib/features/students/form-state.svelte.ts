import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";

import { localizeHref } from "$lib/paraglide/runtime";

import { PersonalInfoState } from "./state/personal-info.svelte";
import { GuardianState } from "./state/guardian-info.svelte";
import { FinancialInfoState } from "./state/financial-info.svelte";
import { DocumentInfoState } from "./state/document-info.svelte";

export class StudentFormState {
  orgId: string;
  studentId?: string;
  isEditMode: boolean;
  classes = $state<any[]>([]);
  originalStudent = $state<any>(null);

  // Sub-states
  personal: PersonalInfoState;
  guardian: GuardianState;
  financial: FinancialInfoState;
  document: DocumentInfoState;

  // Dynamic config fields
  requiredFields = $state<Record<string, boolean>>({});
  activeFields = $state<Record<string, boolean>>({});
  preventDuplicates = $state<Record<string, boolean>>({});
  registrationStatus = $state("draft");

  // Warnings
  duplicateWarnings = $state<{
    national_id?: string;
    email?: string;
    phone?: string;
  }>({});

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
      await this.financial.init();

      // Load organization settings
      const org = await commands.organization.getOrganization(this.orgId);
      if (org) {
        this.orgName = String(org.name || org.name_ar || "");
        if (org.settings) {
          try {
            const settings = JSON.parse(org.settings as string);
            const regConfig = settings.registration_config || {};

            this.document.setRequiredDocuments(regConfig.required_documents || []);

            if (regConfig.required_fields) {
              this.requiredFields = { ...this.requiredFields, ...regConfig.required_fields };
            }
            if (regConfig.active_fields) {
              this.activeFields = { ...this.activeFields, ...regConfig.active_fields };
            }
            if (regConfig.prevent_duplicates) {
              this.preventDuplicates = { ...this.preventDuplicates, ...regConfig.prevent_duplicates };
            }
          } catch (err) {
            console.error("Failed to parse settings JSON:", err);
          }
        }
      }

      // Load document templates for printing
      try {
        const templatesList = await commands.platform.listDocumentTemplates(this.orgId, {
          entity_type: "student",
          status: "published",
        });
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
          originalStudent = await commands.academic.getEnrollment(this.orgId, this.studentId);
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
      toast.error("فشل تهيئة نموذج الطالب: " + (e.message || String(e)));
    }
  }

  // Live duplicate check triggers
  async triggerDuplicateCheck(field: "national_id" | "email" | "phone", value: string) {
    if (!value.trim() || !this.preventDuplicates[field]) return;
    try {
      const duplicates = await commands.iam.searchUsers(value);
      if (duplicates && duplicates.length > 0) {
        const match = duplicates[0];
        this.duplicateWarnings = {
          ...this.duplicateWarnings,
          [field]: m.common_form_guardian_duplicate_warning({ name: match.full_name }),
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
    return this.classes.find((c) => String(c.id) === this.personal.classId)?.level || "";
  }

  get selectedClassName() {
    return (
      this.classes.find((c) => String(c.id) === this.personal.classId)?.name ||
      this.classes.find((c) => String(c.id) === this.personal.classId)?.name_ar ||
      m.common_form_select_class()
    );
  }

  get mandatoryFeesList() {
    const level = this.classLevel;
    if (!level) return [];
    return this.financial.feeTemplates.filter((t) => {
      if (t.is_mandatory === 0 && t.is_active === 1) {
        if (t.applicable_levels) {
          try {
            const rules = JSON.parse(t.applicable_levels);
            if (rules.levels && Array.isArray(rules.levels)) return rules.levels.includes(level);
            else if (rules.level) return rules.level === level;
          } catch {
            return false;
          }
        }
        return true;
      }
      return false;
    });
  }

  get mandatoryTotal() {
    return this.mandatoryFeesList.reduce((sum, t) => sum + t.default_amount, 0);
  }

  get discountTotal() {
    if (!this.financial.applyDiscount) return 0;
    if (this.financial.discountType === "template" && this.financial.selectedDiscountTemplateId) {
      const tpl = this.financial.discountTemplates.find(t => t.id === this.financial.selectedDiscountTemplateId);
      if (tpl) {
        const val = (tpl.value as number) || 0;
        if (String(tpl.calculation_method).toLowerCase() === "percentage") {
          return (this.mandatoryTotal * val) / 100;
        } else {
          return val;
        }
      }
    } else if (this.financial.discountType === "custom" && this.financial.customDiscountAmount) {
      return this.financial.customDiscountAmount;
    }
    return 0;
  }

  get optionalFeesList() {
    return this.financial.feeTemplates.filter(t => t.is_mandatory === 0 && t.is_active === 1);
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
    return Math.max(0, this.mandatoryTotal + this.optionalTotal - this.discountTotal);
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
    if (this.requiredFields.national_id && !this.personal.nationalId.trim()) {
      toast.error(m.common_form_error_national_id_required());
      return;
    }

    this.isSaving = true;
    let targetStudentId = "";
    let targetEnrollmentId = this.studentId || "";

    try {
      // Verify control account
      const accounts = await commands.finance.listAccounts(this.orgId);
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
        if (!originalStudent) throw new Error(m.common_form_error_enrollment_not_found());

        targetStudentId = String(originalStudent.student_id);
        await commands.iam.updateUser(targetStudentId, studentUserData);

        if (originalStudent.class_id !== this.personal.classId) {
          await commands.academic.transferEnrollment(this.orgId, this.studentId!, this.personal.classId);
        }

        if (originalStudent.status !== this.registrationStatus) {
          await commands.platform.transitionEnrollmentStatus(this.orgId, this.studentId!, this.registrationStatus);
        }
        toast.success(m.common_form_success_update());
      } else {
        const firstGuardian = this.guardian.linkedGuardians[0];
        
        const res = await commands.organization.registerStudent(this.orgId, {
          name: this.personal.nameAr.trim(),
          email: this.personal.email.trim() || null,
          phone: this.personal.phone.trim() || null,
          gender: this.personal.sex,
          birth_date: this.personal.birthDate || null,
          national_id: this.personal.nationalId.trim() || null,
          address: this.personal.address.trim() || null,
          class_id: this.personal.classId,
          guardian_name: firstGuardian ? firstGuardian.full_name.trim() : null,
          guardian_phone: firstGuardian ? firstGuardian.phone.trim() : null,
          guardian_relationship: firstGuardian ? firstGuardian.relationship : null,
        });
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
          const checkExisting = await commands.academic.searchGuardians(this.orgId, guardian.phone);
          if (checkExisting && checkExisting.length > 0) {
            guardianUserId = checkExisting[0].id;
          } else {
            const guardEmail = guardian.email.trim() || `parent.${Date.now()}@orbis.local`;
            const guardUser = await commands.iam.createUser(this.orgId, "system", {
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
          await commands.iam.updateUser(guardianUserId!, {
            full_name: guardian.full_name.trim(),
            name_ar: guardian.full_name.trim(),
            phone: guardian.phone.trim() || null,
            national_id: guardian.national_id.trim() || null,
          });
        }

        const existingLinks = await commands.academic.listGuardiansByStudent(this.orgId, targetStudentId);
        const matchedLink = existingLinks.find((l) => l.guardian_id === guardianUserId);

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
          await commands.platform.updateGuardianLink(String(matchedLink.id), linkRequest);
        } else {
          await commands.platform.createGuardianLink(linkRequest);
        }
      }

      // Assign optional/custom fees (only if status is active/approved)
      if (this.registrationStatus === "active" || this.registrationStatus === "approved") {
        const today = new Date().toISOString().slice(0, 10);
        for (const key in this.financial.selectedOptionalFees) {
          if (this.financial.selectedOptionalFees[key].checked) {
            try {
              await commands.finance.assignStudentFee({
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
      const templateRecord = await commands.platform.getDocumentTemplate(this.orgId, this.selectedTemplateId);
      if (!templateRecord) {
        toast.error(`القالب '${this.selectedTemplateId}' غير موجود في قاعدة البيانات`);
        return;
      }
      const documentModel = JSON.parse(templateRecord.document_json as string);

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

      const pdfBytes = await commands.docforge.previewDocumentTemplate(
        JSON.stringify(documentModel),
        JSON.stringify(printVariables),
      );

      if (pdfBytes && pdfBytes.length > 0) {
        let binary = "";
        const len = pdfBytes.length;
        for (let i = 0; i < len; i++) binary += String.fromCharCode(pdfBytes[i]);
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
      toast.error(`خطأ أثناء تحميل الاستمارة: ${(e as any)?.message || String(e)}`);
    } finally {
      this.isDownloadingForm = false;
    }
  }
}
