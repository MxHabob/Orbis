import { goto } from "$app/navigation";
import { studentsStore, authStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";

export class StudentFormState {
  orgId: string;
  studentId?: string;
  isEditMode: boolean;

  // Student details
  nameAr = $state("");
  nameEn = $state("");
  nationalId = $state("");
  sex = $state("male");
  classId = $state("");
  studentNumber = $state("");
  email = $state("");
  phone = $state("");
  address = $state("");
  status = $state("active");

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

  // Date picker components
  birthDay = $state("");
  birthMonth = $state("");
  birthYear = $state("");

  get birthDate() {
    if (this.birthDay && this.birthMonth && this.birthYear) {
      return `${this.birthYear}-${this.birthMonth.padStart(2, "0")}-${this.birthDay.padStart(2, "0")}`;
    }
    return "";
  }

  // Guardians
  linkedGuardians = $state<Array<{
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
  }>>([
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

  parentSearchResults = $state<any[]>([]);
  activeSearchIndex = $state<number | null>(null);

  // Address and previous school
  motherName = $state("");
  motherPhone = $state("");
  motherJob = $state("");
  previousSchool = $state("");
  district = $state("");
  governorate = $state("");

  // Documents
  orgRequiredDocuments = $state<string[]>([]);
  submittedDocs = $state<string[]>([]);

  // Page level indicators
  isSaving = $state(false);
  orgName = $state("");

  // Printing templates
  availableTemplates = $state<any[]>([]);
  selectedTemplateId = $state("");

  // Finance details
  discountTemplates = $state<any[]>([]);
  feeTemplates = $state<any[]>([]);
  applyDiscount = $state(false);
  discountType = $state("template"); // template | custom
  selectedDiscountTemplateId = $state("");
  customDiscountAmount = $state<number | null>(null);
  selectedOptionalFees = $state<Record<string, { checked: boolean; amount: number }>>({});

  isDownloadingForm = $state(false);

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
    this.isEditMode = !!studentId;
  }

  async init() {
    try {
      // 1. Ensure studentsStore is loaded so we can find class details and student records
      if (studentsStore.list.length === 0 || studentsStore.classes.length === 0) {
        await studentsStore.load(this.orgId);
      }

      // 2. Fetch active fee templates and discount templates
      const [feeTpls, discountTpls] = await Promise.all([
        commands.finance.getActiveFeeTemplates(this.orgId),
        commands.finance.getActiveDiscountTemplates(this.orgId),
      ]);
      this.feeTemplates = feeTpls || [];
      this.discountTemplates = discountTpls || [];

      // 3. Pre-populate optional fees mapping so they can be bound in UI
      const feeMap: Record<string, { checked: boolean; amount: number }> = {};
      for (const fTpl of this.feeTemplates) {
        if (fTpl.is_mandatory === 0) {
          feeMap[String(fTpl.id)] = {
            checked: false,
            amount: Number(fTpl.default_amount) || 0,
          };
        }
      }
      this.selectedOptionalFees = feeMap;

      // 4. Load organization settings
      const org = await commands.users.getOrganization(this.orgId);
      if (org) {
        this.orgName = String(org.name || org.name_ar || "");
        if (org.settings) {
          try {
            const settings = JSON.parse(org.settings as string);
            const regConfig = settings.registration_config || {};

            this.orgRequiredDocuments = regConfig.required_documents || [];

            if (regConfig.required_fields) {
              this.requiredFields = { ...this.requiredFields, ...regConfig.required_fields };
            }
            if (regConfig.active_fields) {
              this.activeFields = { ...this.activeFields, ...regConfig.active_fields };
            }
            if (regConfig.prevent_duplicates) {
              this.preventDuplicates = {
                ...this.preventDuplicates,
                ...regConfig.prevent_duplicates,
              };
            }
          } catch (err) {
            console.error("Failed to parse settings JSON:", err);
          }
        }
      }

      // 5. Load document templates for printing
      try {
        const templatesList = await commands.documentTemplates.getTemplates(this.orgId, {
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

      // 6. In edit mode, load student profile, guardians, documents, and fees
      if (this.isEditMode && this.studentId) {
        const originalStudent = studentsStore.list.find((s) => s.id === this.studentId) as any;
        if (originalStudent) {
          this.nameAr = originalStudent.full_name || originalStudent.name_ar || "";
          this.nameEn = originalStudent.name_en || "";
          this.nationalId = originalStudent.national_id || "";
          
          const birth = originalStudent.birth_date ? String(originalStudent.birth_date).slice(0, 10) : "";
          if (birth) {
            const parts = birth.split("-");
            if (parts.length === 3) {
              this.birthYear = parts[0];
              this.birthMonth = String(parseInt(parts[1], 10));
              this.birthDay = String(parseInt(parts[2], 10));
            }
          }

          this.sex = originalStudent.sex || "male";
          this.classId = String(originalStudent.class_id || "");
          this.studentNumber = originalStudent.student_number || "";
          this.email = originalStudent.email || "";
          this.phone = originalStudent.phone_number || originalStudent.phone || "";
          this.address = originalStudent.address || "";
          this.registrationStatus = originalStudent.status || "active";

          // Discount details
          if (originalStudent.discount_template_id) {
            this.selectedDiscountTemplateId = String(originalStudent.discount_template_id);
            this.discountType = "template";
            this.applyDiscount = true;
          } else if (originalStudent.custom_discount_amount) {
            this.customDiscountAmount = Number(originalStudent.custom_discount_amount) || null;
            this.discountType = "custom";
            this.applyDiscount = true;
          }

          // Fetch student fees to check which optional fees are assigned
          try {
            const studentFees = await commands.finance.getStudentFees(this.orgId, this.studentId);
            for (const fee of studentFees as any[]) {
              const feeTemplateId = fee.fee_template_id as string;
              if (feeTemplateId && this.selectedOptionalFees[feeTemplateId]) {
                this.selectedOptionalFees[feeTemplateId].checked = true;
                this.selectedOptionalFees[feeTemplateId].amount = Number(fee.amount) || 0;
              }
            }
          } catch (feeErr) {
            console.error("Failed to load student assigned fees:", feeErr);
          }

          // Fetch guardians
          try {
            const guardiansList = await commands.guardians.getByStudent(originalStudent.student_id);
            const populatedGuardians = [];
            for (const link of guardiansList) {
              try {
                const u = await commands.users.getUser(String(link.guardian_id));
                if (u) {
                  populatedGuardians.push({
                    id: String(link.id),
                    user_id: String(link.guardian_id),
                    full_name: String(u.full_name || u.name_ar || ""),
                    phone: String(u.phone || ""),
                    email: String(u.email || ""),
                    national_id: String(u.national_id || ""),
                    relationship: String(link.relationship || ""),
                    relationship_ar: String(link.relationship_ar || ""),
                    is_primary: link.is_primary === 1,
                    is_emergency_contact: link.is_emergency_contact === 1,
                    is_payer: link.is_payer === 1,
                    payment_share_percent: Number(link.payment_share_percent) || 0,
                    occupation: String(link.occupation || ""),
                  });
                }
              } catch (err) {
                console.error("Failed to load guardian user details:", err);
              }
            }
            if (populatedGuardians.length > 0) {
              this.linkedGuardians = populatedGuardians;
            }
          } catch (guardErr) {
            console.error("Failed to load student guardians:", guardErr);
          }

          // Fetch documents
          try {
            const studentDocs = await commands.documents.getAll({
              orgId: this.orgId,
              ownerId: originalStudent.student_id,
            });
            const submitted = [];
            const requiredDocNames = this.orgRequiredDocuments.length > 0
              ? this.orgRequiredDocuments
              : ["birth_certificate", "national_id", "previous_record"];
            for (const rDoc of requiredDocNames) {
              const hasDoc = studentDocs.some((d: any) => {
                const title = String(d.title || d.name || "").toLowerCase();
                const rName = rDoc.toLowerCase();
                return title === rName || title.includes(rName) || rName.includes(title);
              });
              if (hasDoc) {
                submitted.push(rDoc);
              }
            }
            this.submittedDocs = submitted;
          } catch (docErr) {
            console.error("Failed to load student documents:", docErr);
          }
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
      const params = {
        nationalId: field === "national_id" ? value : undefined,
        email: field === "email" ? value : undefined,
        phone: field === "phone" ? value : undefined,
      };
      const duplicates = await commands.enrollments.checkDuplicate(this.orgId, params);
      if (duplicates && duplicates.length > 0) {
        const match = duplicates[0];
        this.duplicateWarnings = {
          ...this.duplicateWarnings,
          [field]: m.form_guardian_duplicate_warning({ name: match.full_name }),
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

  // Guardian management
  async searchParent(query: string, index: number) {
    if (query.trim().length < 3) {
      this.parentSearchResults = [];
      return;
    }
    this.activeSearchIndex = index;
    try {
      this.parentSearchResults = await commands.guardians.search(this.orgId, query);
    } catch (err) {
      console.error("Guardian search error:", err);
    }
  }

  selectExistingParent(parent: any, index: number) {
    this.linkedGuardians[index].user_id = parent.id;
    this.linkedGuardians[index].full_name = parent.full_name;
    this.linkedGuardians[index].phone = parent.phone || "";
    this.linkedGuardians[index].email = parent.email || "";
    this.linkedGuardians[index].national_id = parent.national_id || "";
    this.parentSearchResults = [];
    this.activeSearchIndex = null;
    toast.success(m.form_guardian_select_existing_success());
  }

  addGuardianField() {
    this.linkedGuardians = [
      ...this.linkedGuardians,
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

  removeGuardianField(index: number) {
    this.linkedGuardians = this.linkedGuardians.filter((_, i) => i !== index);
  }

  toggleDoc(docId: string) {
    if (this.submittedDocs.includes(docId)) {
      this.submittedDocs = this.submittedDocs.filter((d) => d !== docId);
    } else {
      this.submittedDocs = [...this.submittedDocs, docId];
    }
  }

  // Financial Live Calculations (Svelte 5 class getters refer to active states)
  get classLevel() {
    return studentsStore.classes.find((c) => String(c.id) === this.classId)?.level || "";
  }

  get selectedClassName() {
    return (
      studentsStore.classes.find((c) => String(c.id) === this.classId)?.name ||
      studentsStore.classes.find((c) => String(c.id) === this.classId)?.name_ar ||
      m.form_select_class()
    );
  }

  get mandatoryFeesList() {
    const level = this.classLevel;
    if (!level) return [];
    return this.feeTemplates.filter((t) => {
      if (t.is_mandatory === 1 && t.is_active === 1) {
        if (t.applicable_levels) {
          try {
            const rules = JSON.parse(t.applicable_levels);
            if (rules.levels && Array.isArray(rules.levels)) {
              return rules.levels.includes(level);
            } else if (rules.level) {
              return rules.level === level;
            }
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
    if (!this.applyDiscount) return 0;
    if (this.discountType === "template" && this.selectedDiscountTemplateId) {
      const tpl = this.discountTemplates.find((t) => t.id === this.selectedDiscountTemplateId);
      if (tpl) {
        const val = (tpl.value as number) || 0;
        if (String(tpl.calculation_method).toLowerCase() === "percentage") {
          return (this.mandatoryTotal * val) / 100;
        } else {
          return val;
        }
      }
    } else if (this.discountType === "custom" && this.customDiscountAmount) {
      return this.customDiscountAmount;
    }
    return 0;
  }

  get optionalFeesList() {
    return this.feeTemplates.filter((t) => t.is_mandatory === 0 && t.is_active === 1);
  }

  get optionalTotal() {
    let sum = 0;
    for (const key in this.selectedOptionalFees) {
      if (this.selectedOptionalFees[key].checked) {
        sum += this.selectedOptionalFees[key].amount || 0;
      }
    }
    return sum;
  }

  get netTotal() {
    return Math.max(0, this.mandatoryTotal + this.optionalTotal - this.discountTotal);
  }

  // Save operation
  async handleSave() {
    if (!this.nameAr.trim()) {
      toast.error(m.form_error_name_required());
      return;
    }
    if (!this.classId) {
      toast.error(m.form_error_class_required());
      return;
    }
    if (this.requiredFields.national_id && !this.nationalId.trim()) {
      toast.error(m.form_error_national_id_required());
      return;
    }
    if (this.requiredFields.phone && !this.phone.trim()) {
      toast.error(m.form_error_phone_required());
      return;
    }
    if (this.requiredFields.email && !this.email.trim()) {
      toast.error(m.form_error_email_required());
      return;
    }

    this.isSaving = true;
    const createdBy = authStore.user?.id || "";
    let targetStudentId = "";
    let targetEnrollmentId = this.studentId || "";

    try {
      // Verify control account
      const accounts = await commands.finance.getLedgerAccounts(this.orgId);
      const hasControlAccount = accounts.some((acc) => acc.code === "110301");
      if (!hasControlAccount) {
        toast.error(m.form_error_control_account_missing());
        this.isSaving = false;
        return;
      }

      const studentUserData = {
        full_name: this.nameAr.trim(),
        name_ar: this.nameAr.trim(),
        name_en: this.nameEn.trim() || null,
        email: this.email.trim() || null,
        phone: this.phone.trim() || null,
        sex: this.sex,
        date_of_birth: this.birthDate || null,
        national_id: this.nationalId.trim() || null,
        address: this.address.trim() || null,
        status: "active",
      };

      if (this.isEditMode) {
        const originalStudent = studentsStore.list.find((s) => s.id === this.studentId);
        if (!originalStudent) {
          throw new Error(m.form_error_enrollment_not_found());
        }
        targetStudentId = String(originalStudent.student_id);
        await commands.users.updateUser(targetStudentId, studentUserData);

        if (originalStudent.class_id !== this.classId) {
          await commands.enrollments.transfer(this.studentId!, {
            new_class_id: this.classId,
          });
        }

        // Update registration status transition
        if (originalStudent.status !== this.registrationStatus) {
          await commands.enrollments.transitionStatus(
            this.orgId,
            this.studentId!,
            this.registrationStatus
          );
        }

        toast.success(m.form_success_update());
      } else {
        const tempEmail = this.email.trim() || `stu.${Date.now()}@orbis.local`;
        const studentUser = await commands.users.createUser(this.orgId, createdBy, {
          email: tempEmail,
          password: "Orbis123456",
          full_name: this.nameAr.trim(),
          name_ar: this.nameAr.trim(),
          primary_role: "student",
          sex: this.sex,
          phone: this.phone.trim() || null,
          national_id: this.nationalId.trim() || null,
          date_of_birth: this.birthDate || null,
        });
        targetStudentId = studentUser.id;

        await commands.users.updateUser(studentUser.id, {
          name_en: this.nameEn.trim() || null,
          address: this.address.trim() || null,
        });

        const enrollmentRequest = {
          student_id: studentUser.id,
          class_id: this.classId,
          enrollment_type: "regular",
          discount_template_id:
            this.applyDiscount && this.discountType === "template"
              ? this.selectedDiscountTemplateId || null
              : null,
          custom_discount_amount:
            this.applyDiscount && this.discountType === "custom"
              ? this.customDiscountAmount || null
              : null,
          status: this.registrationStatus,
        };

        const res = await commands.enrollments.create(this.orgId, enrollmentRequest);
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
      for (const guardian of this.linkedGuardians) {
        if (!guardian.full_name.trim()) continue;
        let guardianUserId = guardian.user_id;

        if (!guardianUserId) {
          // Search if already exists
          const checkExisting = await commands.guardians.search(this.orgId, guardian.phone);
          if (checkExisting && checkExisting.length > 0) {
            guardianUserId = checkExisting[0].id;
          } else {
            // Create new parent user
            const guardEmail = guardian.email.trim() || `parent.${Date.now()}@orbis.local`;
            const guardUser = await commands.users.createUser(this.orgId, createdBy, {
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
        const existingLinks = await commands.guardians.getByStudent(targetStudentId);
        const matchedLink = existingLinks.find((l) => l.guardian_id === guardianUserId);

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
          await commands.guardians.updateLink(String(matchedLink.id), linkRequest);
        } else {
          await commands.guardians.createLink(linkRequest);
        }
      }

      // Assign optional/custom fees (only if status is active/approved)
      if (this.registrationStatus === "active" || this.registrationStatus === "approved") {
        const today = new Date().toISOString().slice(0, 10);
        for (const key in this.selectedOptionalFees) {
          if (this.selectedOptionalFees[key].checked) {
            try {
              await commands.finance.assignStudentFee(this.orgId, {
                enrollment_id: targetEnrollmentId,
                fee_template_id: key,
                amount: this.selectedOptionalFees[key].amount,
                due_date: today,
                notes: "رسوم إضافية مضافة عند التسجيل",
              });
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
          await commands.users.deleteUser(targetStudentId, createdBy);
        } catch (deleteErr) {
          console.error("Failed to rollback student user creation:", deleteErr);
        }
      }
      console.error("Save error:", e);
      toast.error(m.form_error_save({ error: e.message || String(e) }));
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
      const templateRecord = await commands.documentTemplates.getTemplate(
        this.orgId,
        this.selectedTemplateId
      );
      if (!templateRecord) {
        toast.error(`القالب '${this.selectedTemplateId}' غير موجود في قاعدة البيانات`);
        return;
      }
      const documentModel = JSON.parse(templateRecord.document_json as string);

      const printVariables = {
        orgName: this.orgName,
        student_name: this.nameAr,
        name_ar: this.nameAr,
        name_en: this.nameEn,
        birth_date: this.birthDate,
        sex: this.sex === "male" ? "ذكر" : "أنثى",
        sex_en: this.sex,
        class_name: this.selectedClassName,
        student_number: this.studentNumber,
        email: this.email,
        phone: this.phone,
        address: this.address,
        documents: this.submittedDocs.join(", "),
        current_date: new Date().toLocaleDateString("ar-YE"),
        mandatory_fees: this.mandatoryTotal.toString(),
        optional_fees: this.optionalTotal.toString(),
        discount_fees: this.discountTotal.toString(),
        net_fees: this.netTotal.toString(),
      };

      const pdfBytes = await commands.docforge.previewDocumentTemplate(
        JSON.stringify(documentModel),
        JSON.stringify(printVariables)
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
        link.download = `استمارة_${this.nameAr || "طالب"}.pdf`;
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
      this.isDownloadingForm = false;
    }
  }
}
