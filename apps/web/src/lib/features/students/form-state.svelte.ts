import { periodOverrides } from "$lib/shared/utils/period-override.svelte";
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
  type OrganizationSettingsDto,
} from "$lib/api";
import { toast } from "@orbis/ui";

import { formatDate, formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import type { DuplicateCheckRequest } from "@orbis/sdk";
import {
  createShowInFolderToastAction,
  downloadBlob,
} from "$lib/shared/utils/download";

import { PersonalInfoState } from "./state/personal-info.svelte";
import { GuardianState } from "./state/guardian-info.svelte";
import { splitPersonName } from "$lib/shared/utils/person-name";
import {
  ensureStudentRegistrationArtifacts,
  getOrCreateStudentArchiveFolder,
  provisionStudentLoginAccount,
} from "./registration-effects";
import { FinancialInfoState } from "./state/financial-info.svelte";
import { DocumentInfoState } from "./state/document-info.svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";
import { orgContext } from "$lib/stores/org-context.svelte";
import { authStore } from "$lib/stores/auth.svelte";
import { uiStore } from "$lib/stores/ui.svelte";
import { parseRegistrationFieldPolicies } from "./registration-fields";

interface AdmissionPolicyState {
  registrationEnabled: boolean;
  requiresApproval: boolean;
  allowWaitlist: boolean;
  allowNoGuardian: boolean;
  allowDuplicateNationalId: boolean;
  preventDuplicateFullName: boolean;
  autoAssignFees: boolean;
  allowLateEnrollment: boolean;
  siblingPriorityEnabled: boolean;
  maxClassCapacity: number;
}

const defaultAdmissionPolicy: AdmissionPolicyState = {
  registrationEnabled: true,
  requiresApproval: true,
  allowWaitlist: true,
  allowNoGuardian: false,
  allowDuplicateNationalId: false,
  preventDuplicateFullName: true,
  autoAssignFees: true,
  allowLateEnrollment: false,
  siblingPriorityEnabled: true,
  maxClassCapacity: 30,
};

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
  studyMode = $state<
    | "regular"
    | "part_time"
    | "distance_learning"
    | "homeschooled"
    | "auditor"
    | "visiting"
    | "exchange"
    | "special_needs"
  >("regular");
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
    nationality: true,
    place_of_birth: false,
    passport_number: false,
    blood_type: false,
    emergency_contact: true,
    medical_notes: false,
    special_needs_notes: false,
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
    nationality: true,
    place_of_birth: true,
    passport_number: true,
    blood_type: true,
    emergency_contact: true,
    medical_notes: true,
    special_needs_notes: true,
  });
  preventDuplicates = $state<Record<string, boolean>>({
    national_id: true,
    phone: false,
    email: false,
  });
  registrationStatus = $state("draft");
  createLoginAccount = $state(false);
  admissionPolicy = $state<AdmissionPolicyState>({ ...defaultAdmissionPolicy });
  isInitializing = $state(false);
  initializationFailed = $state(false);

  // Warnings for duplicate values
  duplicateWarnings = $state<{
    national_id?: string;
    email?: string;
    phone?: string;
    full_name?: string;
  }>({});

  private duplicateCheckTimers: Record<string, ReturnType<typeof setTimeout>> =
    {};

  // Class & Academic Data
  classes = $state<ClassDto[]>([]);

  // Page level indicators
  isSaving = $state(false);
  orgName = $state("");
  isDownloadingForm = $state(false);

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
    this.studyMode = uiStore.getPreference(
      { userId: authStore.userId, organizationId: orgId },
      "students.studyMode",
      "regular",
      (value): value is typeof this.studyMode =>
        value === "regular" ||
        value === "part_time" ||
        value === "distance_learning" ||
        value === "homeschooled" ||
        value === "auditor" ||
        value === "visiting" ||
        value === "exchange" ||
        value === "special_needs",
    );
  }

  private appliedClassDefault = false;

  applyClassOptions(
    classes: ClassDto[],
    contextualIds: readonly (string | null)[],
  ) {
    this.classes = classes;
    if (this.isEditMode || this.appliedClassDefault || classes.length === 0)
      return;
    this.appliedClassDefault = true;
    if (this.personal.classId) return;

    const rememberedClassId = uiStore.getPreference(
      { userId: authStore.userId, organizationId: this.orgId },
      "students.classId",
      "",
      (value): value is string => typeof value === "string",
    );
    const classIds = classes.map((item) => String(item.id));
    const defaultClassId = [
      ...contextualIds,
      rememberedClassId,
      classIds.length === 1 ? classIds[0] : "",
    ].find((id) => id !== null && classIds.includes(id));
    if (defaultClassId) this.personal.classId = defaultClassId;
    return defaultClassId;
  }

  async init() {
    this.isInitializing = true;
    this.initializationFailed = false;
    try {
      const [org, settings] = await Promise.all([
        api.getCurrentOrg(),
        api.getSettings(),
        this.financial.init(),
        this.document.loadRequirements(),
      ]);
      this.orgName = String(org.name || org.name_ar || "");
      const baseCurr = org.base_currency_code || orgContext.baseCurrency;
      if (baseCurr) {
        this.currency = baseCurr;
      }

      const policies = parseRegistrationFieldPolicies(settings);
      for (const [key, policy] of Object.entries(policies)) {
        this.activeFields[key] = policy.active;
        this.requiredFields[key] = policy.active && policy.required;
        this.preventDuplicates[key] = policy.active && policy.preventDuplicate;
      }

      this.admissionPolicy = this.parseAdmissionPolicy(settings);
      this.document.setRequiredDocuments(
        this.getConfiguredRequiredDocuments(org.settings),
      );

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
          this.studyMode =
            (originalStudent.enrollment?.study_mode as typeof this.studyMode) ||
            "regular";
          this.personal.populate(originalStudent);
          await this.financial.populate(originalStudent);
          await this.guardian.populate();
          await this.document.populate(originalStudent);
        }
      }
    } catch (e: unknown) {
      this.initializationFailed = true;
      console.error("Failed to initialize student form state:", e);
      toast.error(
        m.common_form_initialization_error({
          error: extractErrorMessage(e),
        }),
      );
    } finally {
      this.isInitializing = false;
    }
  }

  private parseAdmissionPolicy(
    settings: OrganizationSettingsDto,
  ): AdmissionPolicyState {
    const admissionPolicy = settings.admission?.policy;
    const enrollment = settings.enrollment;
    return {
      registrationEnabled:
        enrollment?.registration_enabled ??
        defaultAdmissionPolicy.registrationEnabled,
      requiresApproval:
        enrollment?.require_admin_approval ??
        admissionPolicy?.requires_approval ??
        defaultAdmissionPolicy.requiresApproval,
      allowWaitlist:
        enrollment?.allow_waitlist ??
        admissionPolicy?.allow_waitlist ??
        defaultAdmissionPolicy.allowWaitlist,
      allowNoGuardian:
        admissionPolicy?.allow_no_guardian ??
        defaultAdmissionPolicy.allowNoGuardian,
      allowDuplicateNationalId:
        admissionPolicy?.allow_duplicate_national_id ??
        defaultAdmissionPolicy.allowDuplicateNationalId,
      preventDuplicateFullName:
        (admissionPolicy as any)?.prevent_duplicate_full_name ??
        defaultAdmissionPolicy.preventDuplicateFullName,
      autoAssignFees:
        admissionPolicy?.auto_assign_fees ??
        defaultAdmissionPolicy.autoAssignFees,
      allowLateEnrollment:
        enrollment?.allow_late_enrollment ??
        admissionPolicy?.allow_late_enrollment ??
        defaultAdmissionPolicy.allowLateEnrollment,
      siblingPriorityEnabled:
        enrollment?.sibling_priority_enabled ??
        admissionPolicy?.sibling_priority_enabled ??
        defaultAdmissionPolicy.siblingPriorityEnabled,
      maxClassCapacity:
        enrollment?.max_class_size_default ??
        admissionPolicy?.max_class_capacity ??
        defaultAdmissionPolicy.maxClassCapacity,
    };
  }

  private getConfiguredRequiredDocuments(settings: unknown): string[] {
    let settingsObject: Record<string, unknown> = {};
    try {
      settingsObject =
        typeof settings === "string"
          ? (JSON.parse(settings) as Record<string, unknown>)
          : ((settings as Record<string, unknown>) ?? {});
    } catch (error) {
      console.error(
        "Failed to parse organization document requirements:",
        error,
      );
      return [];
    }

    const admission = (settingsObject.admission ??
      settingsObject.registration ??
      settingsObject.registration_config) as
      | Record<string, unknown>
      | undefined;
    const requirements =
      admission?.required_documents_list ?? admission?.requiredDocumentsList;
    if (!Array.isArray(requirements)) return [];

    return requirements
      .map((item: unknown) => {
        if (typeof item === "string") return item;
        if (!item || typeof item !== "object") return "";
        const document = item as Record<string, unknown>;
        return String(
          document.nameAr ?? document.nameEn ?? document.name ?? "",
        );
      })
      .filter(Boolean);
  }

  // Live debounced duplicate check triggers using authoritative backend API
  triggerDuplicateCheck(
    field: "national_id" | "email" | "phone" | "full_name",
    value: string,
  ) {
    if (this.duplicateCheckTimers[field]) {
      clearTimeout(this.duplicateCheckTimers[field]);
    }

    const trimmed = (value || "").trim();
    const hasName =
      !!this.personal.nameEn?.trim() || !!this.personal.nameAr?.trim();

    if (!trimmed && !(field === "full_name" && hasName)) {
      this.duplicateWarnings = {
        ...this.duplicateWarnings,
        [field]: undefined,
      };
      return;
    }

    this.duplicateCheckTimers[field] = setTimeout(async () => {
      try {
        const payload: DuplicateCheckRequest = {
          exclude_student_id: this.studentId,
          full_name:
            this.personal.nameEn?.trim() ||
            this.personal.nameAr?.trim() ||
            undefined,
          birth_date: this.personal.birthDate || undefined,
        };
        if (field !== "full_name") {
          payload[field] = trimmed;
        } else if (this.personal.nationalId?.trim()) {
          payload.national_id = this.personal.nationalId.trim();
        }
        if (this.personal.phone?.trim())
          payload.phone = this.personal.phone.trim();
        if (this.personal.email?.trim())
          payload.email = this.personal.email.trim();

        const res = await api.checkDuplicate(payload);

        if (res.has_duplicate && res.matches.length > 0) {
          const match =
            res.matches.find((m: any) => m.field === field) ||
            res.matches.find((m: any) => m.field === "full_name") ||
            res.matches[0];
          const studentInfo = match.student_number
            ? ` (${match.student_number})`
            : "";
          const message =
            match.field === "full_name"
              ? m.operations_ui_duplicate_person_value({
                  name: match.matched_person_name,
                  details: studentInfo,
                })
              : m.operations_ui_duplicate_person_value({
                  name: match.matched_person_name,
                  details: studentInfo,
                });
          this.duplicateWarnings = {
            ...this.duplicateWarnings,
            [field]: message,
            ...(match.field === "full_name" && field !== "full_name"
              ? { full_name: message }
              : {}),
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
    return (
      this.classes.find((c) => String(c.id) === this.personal.classId) || null
    );
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
          const rules =
            typeof t.applicable_levels === "string"
              ? JSON.parse(t.applicable_levels)
              : t.applicable_levels;
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
  getRequiredFieldIssue(
    saveAsDraft: boolean,
    includesField = (_id: string) => true,
  ) {
    if (
      includesField("nameAr") &&
      !this.personal.nameAr.trim() &&
      !this.personal.nameEn.trim()
    ) {
      return { id: "nameAr", message: m.common_form_error_name_required() };
    }
    if (saveAsDraft) return null;
    if (includesField("classSelect") && !this.personal.classId) {
      return {
        id: "classSelect",
        message: m.common_form_error_class_required(),
      };
    }

    const fields = [
      ["national_id", this.personal.nationalId, "nationalId"],
      ["passport_number", this.personal.passportNumber, "passportNumber"],
      ["nationality", this.personal.nationality, "nationality"],
      ["place_of_birth", this.personal.placeOfBirth, "placeOfBirth"],
      ["birth_date", this.personal.birthDate, "student-birth-day"],
      ["sex", this.personal.sex, "student-sex"],
      ["phone", this.personal.phone, "phone"],
      ["email", this.personal.email, "email"],
      ["blood_type", this.personal.bloodType, "bloodType"],
      ["emergency_contact", this.personal.emergencyContact, "emergencyContact"],
      ["medical_notes", this.personal.medicalNotes, "medicalNotes"],
      [
        "special_needs_notes",
        this.personal.specialNeedsNotes,
        "specialNeedsNotes",
      ],
      ["previous_school", this.personal.previousSchool, "previousSchool"],
      ["address", this.personal.address, "address"],
    ] as const;
    const missing = fields.find(
      ([key, value, id]) =>
        includesField(id) && this.requiredFields[key] && !value.trim(),
    );
    return missing
      ? { id: missing[2], message: m.common_form_error_required_fields() }
      : null;
  }

  private get primaryGuardian() {
    return (
      this.guardian.linkedGuardians.find((guardian) => guardian.is_primary) ??
      this.guardian.linkedGuardians[0]
    );
  }

  private get blockingDocumentRequirements() {
    return this.document.requirements.filter(
      (requirement) => requirement.blocks_enrollment,
    );
  }

  getAdmissionIssue(
    saveAsDraft: boolean,
    step?: "profile" | "contacts" | "enrollment" | "documents" | "review",
  ) {
    if (saveAsDraft) return null;

    if (!step || step === "profile") {
      const fields = [
        ["national_id", "nationalId"],
        ["phone", "phone"],
        ["email", "email"],
        ["full_name", "nameAr"],
      ] as const;
      const duplicate = fields.find(([key]) => this.duplicateWarnings[key]);
      if (duplicate) {
        return {
          id: duplicate[1],
          step: "profile" as const,
          message:
            m.ui_copy_please_resolve_duplicate_warnings_before_p_d77b26cb(),
        };
      }
    }

    if (this.isEditMode && this.originalStudent?.student?.status !== "inactive")
      return null;

    if (!step || step === "contacts") {
      const guardians = this.guardian.linkedGuardians;
      const primary = this.primaryGuardian;
      if (!this.admissionPolicy.allowNoGuardian && !primary?.full_name.trim()) {
        return {
          id: primary
            ? `guardian-name-${guardians.indexOf(primary)}`
            : "registration-contacts-heading",
          step: "contacts" as const,
          message: m.common_validation_guardian_name_required(),
        };
      }

      const unlinked = guardians.findIndex(
        (guardian) =>
          guardian !== primary &&
          !guardian.guardian_id &&
          (guardian.full_name.trim() || guardian.phone.trim()),
      );
      if (unlinked >= 0) {
        return {
          id: `guardian-name-${unlinked}`,
          step: "contacts" as const,
          message: m.student_registration_guardian_unlinked(),
        };
      }
    }

    if (!step || step === "documents") {
      const requirements = this.blockingDocumentRequirements;
      const codes = requirements.length
        ? requirements.map((requirement) => requirement.code)
        : this.document.orgRequiredDocuments;
      const fulfilled = new Set(
        this.document.fulfillments
          .filter((item) => item.is_fulfilled)
          .map((item) => item.requirement.code),
      );
      const missing = codes.find(
        (code) =>
          !fulfilled.has(code) &&
          !this.document.submittedDocs.includes(code) &&
          !this.document.attachments[code],
      );
      if (missing) {
        return {
          id: `doc_${missing}`,
          step: "documents" as const,
          message: m.student_registration_documents_pending(),
        };
      }
    }

    return null;
  }

  async handleSave(queryClient?: QueryClient, saveAsDraft = false) {
    const isCompletingDraft =
      this.isEditMode &&
      this.originalStudent?.student?.status === "inactive" &&
      !saveAsDraft;
    const displayName =
      this.personal.nameAr.trim() || this.personal.nameEn.trim();
    const issue = this.getRequiredFieldIssue(saveAsDraft);
    if (issue) {
      toast.error(issue.message);
      return;
    }

    const admissionIssue = this.getAdmissionIssue(saveAsDraft);
    if (admissionIssue) {
      toast.error(admissionIssue.message);
      return;
    }

    const primaryGuardian = this.primaryGuardian;
    const blockingRequirements = this.blockingDocumentRequirements;

    this.isSaving = true;
    let targetStudentId = "";
    let targetEnrollmentId = this.studentId || "";
    let registrationArtifactWarnings: string[] = [];
    let studentArchiveFolderId: string | undefined;

    try {
      if (!this.isEditMode) {
        const namePartsAr = splitPersonName(
          this.personal.nameAr || this.personal.nameEn,
        );
        const namePartsEn = splitPersonName(
          this.personal.nameEn || this.personal.nameAr,
        );

        const registrationJob = await api.startBulkRegisterJob(
          [
            {
              row_index: 0,
              registration: {
                student_number: this.personal.studentNumber || undefined,
                first_name: namePartsEn.firstName,
                middle_name: namePartsEn.middleName,
                last_name: namePartsEn.lastName,
                first_name_ar: namePartsAr.firstName,
                middle_name_ar: namePartsAr.middleName,
                last_name_ar: namePartsAr.lastName,
                display_name: displayName,
                national_id: this.activeFields.national_id
                  ? this.personal.nationalId || undefined
                  : undefined,
                passport_number: this.activeFields.passport_number
                  ? this.personal.passportNumber || undefined
                  : undefined,
                gender: this.activeFields.sex
                  ? this.personal.sex || undefined
                  : undefined,
                birth_date: this.activeFields.birth_date
                  ? this.personal.birthDate || undefined
                  : undefined,
                place_of_birth: this.activeFields.place_of_birth
                  ? this.personal.placeOfBirth || undefined
                  : undefined,
                nationality: this.activeFields.nationality
                  ? this.personal.nationality || undefined
                  : undefined,
                blood_type: this.activeFields.blood_type
                  ? this.personal.bloodType || undefined
                  : undefined,
                medical_notes: this.activeFields.medical_notes
                  ? this.personal.medicalNotes || undefined
                  : undefined,
                special_needs_notes: this.activeFields.special_needs_notes
                  ? this.personal.specialNeedsNotes || undefined
                  : undefined,
                phone: this.activeFields.phone
                  ? this.personal.phone || undefined
                  : undefined,
                email: this.activeFields.email
                  ? this.personal.email.trim() || undefined
                  : undefined,
                address: this.activeFields.address
                  ? this.personal.address || undefined
                  : undefined,
                initial_period_id: saveAsDraft
                  ? undefined
                  : effectiveContext.currentPeriodId || undefined,
                initial_section_id: saveAsDraft
                  ? undefined
                  : this.personal.classId || undefined,
                study_mode: this.studyMode,
                is_draft: saveAsDraft,
              },
              guardian_name: primaryGuardian?.full_name.trim() || undefined,
              guardian_phone: primaryGuardian?.phone.trim() || undefined,
              guardian_relationship: primaryGuardian?.relationship || undefined,
            },
          ],
          { atomic: true },
        );
        let registrationJobState = registrationJob;
        while (registrationJobState.status === "running") {
          await new Promise((resolve) => window.setTimeout(resolve, 250));
          const polled = await api.getBulkRegisterJob(registrationJob.id);
          if (!polled) break;
          registrationJobState = polled;
        }
        if (
          periodOverrides.open(
            this.orgId,
            registrationJobState.override_targets ?? [],
          )
        )
          return;
        if (registrationJobState.status === "failed") {
          throw new Error(
            registrationJobState.error || "Student registration failed.",
          );
        }
        const registrationResultItem = registrationJobState.result?.items[0];
        if (
          !registrationResultItem?.success ||
          !registrationResultItem.student_id
        ) {
          throw new Error(
            registrationResultItem?.error ||
              m.common_form_error_save({
                error: "Student registration failed.",
              }),
          );
        }
        targetStudentId = registrationResultItem.student_id;
        if (!saveAsDraft) {
          try {
            const detail = await api.getStudent(targetStudentId);
            targetEnrollmentId = detail.enrollment?.id || "";
          } catch (error) {
            console.error(
              "Failed to load the newly registered student:",
              error,
            );
            registrationArtifactWarnings.push(extractErrorMessage(error));
          }

          const registrationArtifacts =
            await ensureStudentRegistrationArtifacts(
              targetStudentId,
              displayName,
            );
          studentArchiveFolderId = registrationArtifacts.studentFolder?.id;
          registrationArtifactWarnings.push(...registrationArtifacts.warnings);
          if (registrationArtifacts.folderError) {
            registrationArtifactWarnings.push(
              m.students_archive_folder_failed({
                error: registrationArtifacts.folderError,
              }),
            );
          }
        } else {
          try {
            studentArchiveFolderId = (
              await getOrCreateStudentArchiveFolder(
                targetStudentId,
                displayName,
              )
            ).id;
          } catch (error) {
            registrationArtifactWarnings.push(
              m.students_archive_folder_failed({
                error: extractErrorMessage(error),
              }),
            );
          }
        }
        if (this.createLoginAccount && !saveAsDraft) {
          try {
            const account = await provisionStudentLoginAccount({
              studentId: targetStudentId,
              fullNameAr: this.personal.nameAr,
              fullNameEn: this.personal.nameEn,
              email: this.personal.email,
              phone: this.personal.phone,
              preferredUsername:
                this.personal.email ||
                this.personal.phone ||
                this.personal.studentNumber,
            });
            toast.success(
              m.students_login_account_created({ username: account.username }),
            );
          } catch (error) {
            registrationArtifactWarnings.push(
              m.students_login_account_failed({
                error: extractErrorMessage(error),
              }),
            );
          }
        }
        toast.success(
          saveAsDraft
            ? m.student_registration_draft_saved()
            : m.common_form_success_create(),
        );
      } else {
        targetStudentId = this.originalStudent?.student?.id || this.studentId!;
        targetEnrollmentId = this.originalStudent?.enrollment?.id || "";
        try {
          const studentFolder = await getOrCreateStudentArchiveFolder(
            targetStudentId,
            displayName,
          );
          studentArchiveFolderId = studentFolder.id;
        } catch (error) {
          registrationArtifactWarnings.push(
            m.students_archive_folder_failed({
              error: extractErrorMessage(error),
            }),
          );
        }

        const namePartsAr = splitPersonName(
          this.personal.nameAr || this.personal.nameEn,
        );
        const namePartsEn = splitPersonName(
          this.personal.nameEn || this.personal.nameAr,
        );

        await api.updateStudentProfile({
          student_id: targetStudentId,
          first_name: namePartsEn.firstName,
          middle_name: namePartsEn.middleName,
          last_name: namePartsEn.lastName,
          first_name_ar: namePartsAr.firstName,
          middle_name_ar: namePartsAr.middleName,
          last_name_ar: namePartsAr.lastName,
          display_name: displayName,
          national_id: this.activeFields.national_id
            ? this.personal.nationalId || undefined
            : undefined,
          passport_number: this.activeFields.passport_number
            ? this.personal.passportNumber || undefined
            : undefined,
          gender: this.activeFields.sex
            ? this.personal.sex || undefined
            : undefined,
          birth_date: this.activeFields.birth_date
            ? this.personal.birthDate || undefined
            : undefined,
          place_of_birth: this.activeFields.place_of_birth
            ? this.personal.placeOfBirth || undefined
            : undefined,
          nationality: this.activeFields.nationality
            ? this.personal.nationality || undefined
            : undefined,
          blood_type: this.activeFields.blood_type
            ? this.personal.bloodType || undefined
            : undefined,
          medical_notes: this.activeFields.medical_notes
            ? this.personal.medicalNotes || undefined
            : undefined,
          special_needs_notes: this.activeFields.special_needs_notes
            ? this.personal.specialNeedsNotes || undefined
            : undefined,
          phone: this.activeFields.phone
            ? this.personal.phone || undefined
            : undefined,
          email: this.activeFields.email
            ? this.personal.email.trim() || undefined
            : undefined,
          address: this.activeFields.address
            ? this.personal.address || undefined
            : undefined,
        });

        if (!saveAsDraft && !isCompletingDraft) {
          toast.success(m.common_form_success_update());
        }
      }

      // 2. Link existing guardians
      const additionalGuardians = this.isEditMode
        ? this.guardian.linkedGuardians
        : this.guardian.linkedGuardians.filter(
            (guardian) => guardian !== primaryGuardian,
          );
      for (const guardian of additionalGuardians) {
        if (!guardian.guardian_id || !targetStudentId) continue;

        try {
          await api.assignGuardian({
            student_id: targetStudentId,
            guardian_id: guardian.guardian_id,
            relationship: guardian.relationship || "other",
            relationship_ar: guardian.relationship_ar || "آخر",
            is_primary: !!guardian.is_primary,
            is_emergency_contact: !!guardian.is_emergency_contact,
            is_authorized_pickup: !!guardian.is_authorized_pickup,
            is_payer: !!guardian.is_payer,
            payment_share_percent: guardian.payment_share_percent ?? 0,
          });
        } catch (error) {
          console.error("Failed to assign guardian:", error);
          registrationArtifactWarnings.push(extractErrorMessage(error));
        }
      }

      // 3. Store staged files in the student's archive folder.
      if (targetStudentId) {
        if (!studentArchiveFolderId) {
          try {
            studentArchiveFolderId = (
              await getOrCreateStudentArchiveFolder(
                targetStudentId,
                displayName,
              )
            ).id;
          } catch (error) {
            registrationArtifactWarnings.push(
              m.students_archive_upload_skipped({
                error: extractErrorMessage(error),
              }),
            );
          }
        }

        if (studentArchiveFolderId) {
          const uploadWarnings = await this.document.uploadPendingAttachments(
            targetStudentId,
            studentArchiveFolderId,
          );
          registrationArtifactWarnings.push(...uploadWarnings);
        }

        if (
          isCompletingDraft &&
          blockingRequirements.some((requirement) => {
            const attachment = this.document.attachments[requirement.code];
            return attachment?.file && !attachment.documentId;
          })
        ) {
          throw new Error(m.student_registration_documents_pending());
        }

        if (this.avatarFile && studentArchiveFolderId) {
          try {
            const avatarDocument = await api.uploadFile({
              file: this.avatarFile,
              name: this.avatarFile.name,
              entity_type: "student",
              entity_id: targetStudentId,
              folder_id: studentArchiveFolderId,
              category: "avatar",
              asset_type: "image",
              visibility: "public",
            });
            const avatarUrl = avatarDocument.url || avatarDocument.file_path;
            if (avatarUrl) {
              await api.updateStudentProfile({
                student_id: targetStudentId,
                avatar_url: avatarUrl,
              });
              this.avatarUrl = avatarUrl;
            }
          } catch (avatarErr) {
            registrationArtifactWarnings.push(
              m.students_archive_upload_failed({
                fileName: this.avatarFile.name,
                error: extractErrorMessage(avatarErr),
              }),
            );
          }
        }
      }

      if (isCompletingDraft) {
        const enrollment = await api.enrollStudent({
          period_id: effectiveContext.currentPeriodId!,
          student_id: targetStudentId,
          section_id: this.personal.classId,
          student_number:
            this.personal.studentNumber ||
            this.originalStudent?.student?.student_number ||
            targetStudentId.slice(0, 8),
          study_mode: this.studyMode,
        });
        targetEnrollmentId = enrollment.id;
        await api.updateStudentStatus({
          student_id: targetStudentId,
          status: "active",
        });
        toast.success(m.student_registration_draft_completed());
      }

      // Assign optional fees after a draft has been enrolled.
      const activePeriod = effectiveContext.currentPeriodId;
      const today = new Date().toISOString().slice(0, 10);
      if (
        activePeriod &&
        targetStudentId &&
        targetEnrollmentId &&
        !saveAsDraft
      ) {
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

      if (queryClient) {
        queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
        queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
        queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      }

      if (registrationArtifactWarnings.length > 0) {
        toast.warning(registrationArtifactWarnings.join("؛ "));
      }

      const preferenceScope = {
        userId: authStore.userId,
        organizationId: this.orgId,
      };
      uiStore.setPreference(
        preferenceScope,
        "students.studyMode",
        this.studyMode,
      );
      if (this.personal.classId) {
        uiStore.setPreference(
          preferenceScope,
          "students.classId",
          this.personal.classId,
        );
      }

      goto(
        localizeHref(
          saveAsDraft
            ? `/${this.orgId}/students/${targetStudentId}/edit`
            : `/${this.orgId}/students`,
        ),
      );
    } catch (e) {
      if (periodOverrides.handleError(this.orgId, e)) return;
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
        sex:
          this.personal.sex === "male"
            ? "ذكر"
            : this.personal.sex === "female"
              ? "أنثى"
              : "",
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
        const openFile = downloadBlob(
          blob,
          `${m.operations_ui_student_form_filename()}_${this.personal.nameAr || m.operations_ui_student_fallback()}.pdf`,
        );
        toast.success(
          m.ui_copy_document_generated_and_downloaded_successf_456c90b7(),
          {
            action: openFile
              ? createShowInFolderToastAction(openFile)
              : undefined,
          },
        );
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
