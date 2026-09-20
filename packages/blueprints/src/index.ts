/**
 * @module @orbis/blueprints
 * Precompiled, code-first Institution Blueprints for Orbis ERP/SIS.
 */

export type InstitutionType =
  | "k12_school"
  | "university"
  | "vocational_institute"
  | "language_center";

export type FieldType =
  | "text"
  | "number"
  | "email"
  | "phone"
  | "date"
  | "select"
  | "multiselect"
  | "boolean"
  | "textarea"
  | "file";

export interface SelectOption {
  value: string;
  labelEn: string;
  labelAr: string;
}

export interface FieldDefinition {
  name: string;
  labelEn: string;
  labelAr: string;
  type: FieldType;
  required: boolean;
  options?: SelectOption[];
  placeholderEn?: string;
  placeholderAr?: string;
  helpTextEn?: string;
  helpTextAr?: string;
  displayOrder: number;
  isCustom?: boolean;
}

export interface FormSection {
  id: string;
  titleEn: string;
  titleAr: string;
  descriptionEn?: string;
  descriptionAr?: string;
  fields: FieldDefinition[];
}

export interface ModuleConfig {
  id: string;
  enabled: boolean;
  requiredPermissions: string[];
}

export interface InstitutionBlueprint {
  type: InstitutionType;
  nameEn: string;
  nameAr: string;
  descriptionEn: string;
  descriptionAr: string;
  modules: Record<string, boolean>;
  studentRegistrationSections: FormSection[];
  studentProfileTabs: Array<{
    id: string;
    labelEn: string;
    labelAr: string;
    icon: string;
  }>;
}

// ═══════════════════════════════════════════════════════════════
// 1. K-12 School Blueprint (@orbis/blueprint-school)
// ═══════════════════════════════════════════════════════════════

export const SCHOOL_BLUEPRINT: InstitutionBlueprint = {
  type: "k12_school",
  nameEn: "K-12 School",
  nameAr: "مدرسة تعليم أساسي وثانوي (K-12)",
  descriptionEn: "Tailored for elementary, middle, and high schools with homeroom classes, primary guardian tracking, and grade levels.",
  descriptionAr: "مصمم للمدارس الابتدائية، المتوسطة، والثانوية مع دعم الفصول، أولياء الأمور، والمراحل الدراسية.",
  modules: {
    guardians: true,
    homeroom: true,
    transportation: true,
    assessments: true,
    attendance: true,
    behavior: true,
    curriculum: true,
    scheduling: true,
    finance: true,
    creditHours: false,
    academicAdvisor: false,
  },
  studentProfileTabs: [
    { id: "overview", labelEn: "Overview", labelAr: "نظرة عامة", icon: "User" },
    { id: "academics", labelEn: "Academics & Grades", labelAr: "الدرجات الأكاديمية", icon: "GraduationCap" },
    { id: "attendance", labelEn: "Attendance", labelAr: "الحضور والغياب", icon: "CalendarCheck" },
    { id: "behavior", labelEn: "Behavior & Conduct", labelAr: "السلوك والمواظبة", icon: "ShieldCheck" },
    { id: "guardians", labelEn: "Guardians & Family", labelAr: "أولياء الأمور", icon: "Users" },
    { id: "fees", labelEn: "Tuition & Fees", labelAr: "الرسوم والمستحقات", icon: "CreditCard" },
    { id: "documents", labelEn: "Documents & Files", labelAr: "المستندات والملفات", icon: "FileText" },
  ],
  studentRegistrationSections: [
    {
      id: "demographics",
      titleEn: "Student Demographics",
      titleAr: "بيانات الطالب الشخصية",
      fields: [
        { name: "firstName", labelEn: "First Name (EN)", labelAr: "الاسم الأول (إنجليزي)", type: "text", required: true, displayOrder: 1 },
        { name: "lastName", labelEn: "Last Name (EN)", labelAr: "اسم العائلة (إنجليزي)", type: "text", required: true, displayOrder: 2 },
        { name: "firstNameAr", labelEn: "First Name (AR)", labelAr: "الاسم الأول (عربي)", type: "text", required: true, displayOrder: 3 },
        { name: "lastNameAr", labelEn: "Last Name (AR)", labelAr: "اسم العائلة (عربي)", type: "text", required: true, displayOrder: 4 },
        {
          name: "gender",
          labelEn: "Gender",
          labelAr: "الجنس",
          type: "select",
          required: true,
          displayOrder: 5,
          options: [
            { value: "male", labelEn: "Male", labelAr: "ذكر" },
            { value: "female", labelEn: "Female", labelAr: "أنثى" },
          ],
        },
        { name: "birthDate", labelEn: "Date of Birth", labelAr: "تاريخ الميلاد", type: "date", required: true, displayOrder: 6 },
        { name: "nationalId", labelEn: "National ID / Civil ID", labelAr: "رقم الهوية الوطنية / الإقامة", type: "text", required: false, displayOrder: 7 },
        { name: "nationality", labelEn: "Nationality", labelAr: "الجنسية", type: "text", required: false, displayOrder: 8 },
      ],
    },
    {
      id: "academic_placement",
      titleEn: "Academic Placement",
      titleAr: "التعيين الأكاديمي والصف الدراسي",
      fields: [
        { name: "gradeLevel", labelEn: "Grade Level", labelAr: "المستوى / الصف الدراسي", type: "select", required: true, displayOrder: 1 },
        { name: "classId", labelEn: "Assigned Section / Class", labelAr: "الشعبة / الفصل الدراسي", type: "select", required: false, displayOrder: 2 },
        { name: "studentNumber", labelEn: "Student Registration Number", labelAr: "الرقم الأكاديمي / رقم القيد", type: "text", required: false, displayOrder: 3 },
      ],
    },
    {
      id: "guardian_contact",
      titleEn: "Primary Guardian Information",
      titleAr: "بيانات ولي الأمر الأساسي",
      fields: [
        { name: "guardianName", labelEn: "Guardian Full Name", labelAr: "اسم ولي الأمر كاملاً", type: "text", required: true, displayOrder: 1 },
        { name: "guardianPhone", labelEn: "Guardian Mobile Phone", labelAr: "رقم جوال ولي الأمر", type: "phone", required: true, displayOrder: 2 },
        { name: "guardianEmail", labelEn: "Guardian Email", labelAr: "البريد الإلكتروني لولي الأمر", type: "email", required: false, displayOrder: 3 },
        {
          name: "guardianRelationship",
          labelEn: "Relationship to Student",
          labelAr: "صلة القرابة",
          type: "select",
          required: true,
          displayOrder: 4,
          options: [
            { value: "father", labelEn: "Father", labelAr: "الأب" },
            { value: "mother", labelEn: "Mother", labelAr: "الأم" },
            { value: "legal_guardian", labelEn: "Legal Guardian", labelAr: "الوصي القانوني" },
            { value: "brother", labelEn: "Brother", labelAr: "الأخ" },
          ],
        },
      ],
    },
  ],
};

// ═══════════════════════════════════════════════════════════════
// 2. University / Higher Education Blueprint
// ═══════════════════════════════════════════════════════════════

export const UNIVERSITY_BLUEPRINT: InstitutionBlueprint = {
  type: "university",
  nameEn: "University / Higher Education",
  nameAr: "جامعة / كلية تعليم عالي",
  descriptionEn: "Tailored for universities and colleges with major/department tracking, credit hours, cumulative GPA, and advisors.",
  descriptionAr: "مصمم للجامعات والكليات مع دعم التخصصات، الساعات المعتمدة، المعدل التراكمي، والإرشاد الأكاديمي.",
  modules: {
    guardians: false,
    homeroom: false,
    transportation: false,
    assessments: true,
    attendance: true,
    behavior: false,
    curriculum: true,
    scheduling: true,
    finance: true,
    creditHours: true,
    academicAdvisor: true,
  },
  studentProfileTabs: [
    { id: "overview", labelEn: "Student Record", labelAr: "السجل الجامعي", icon: "User" },
    { id: "transcript", labelEn: "Academic Transcript & GPA", labelAr: "السجل الأكاديمي والمعدل", icon: "GraduationCap" },
    { id: "schedule", labelEn: "Course Schedule", labelAr: "الجدول الدراسي", icon: "CalendarCheck" },
    { id: "advising", labelEn: "Academic Advising", labelAr: "الإرشاد الأكاديمي", icon: "Compass" },
    { id: "fees", labelEn: "Financial Account", labelAr: "الحساب المالي والأقساط", icon: "CreditCard" },
    { id: "documents", labelEn: "Documents & Archive", labelAr: "المستندات والأرشيف", icon: "FileText" },
  ],
  studentRegistrationSections: [
    {
      id: "demographics",
      titleEn: "Personal & Demographic Information",
      titleAr: "البيانات الشخصية والأكاديمية",
      fields: [
        { name: "firstName", labelEn: "First Name (EN)", labelAr: "الاسم الأول (إنجليزي)", type: "text", required: true, displayOrder: 1 },
        { name: "lastName", labelEn: "Last Name (EN)", labelAr: "اسم العائلة (إنجليزي)", type: "text", required: true, displayOrder: 2 },
        { name: "firstNameAr", labelEn: "First Name (AR)", labelAr: "الاسم الأول (عربي)", type: "text", required: false, displayOrder: 3 },
        { name: "lastNameAr", labelEn: "Last Name (AR)", labelAr: "اسم العائلة (عربي)", type: "text", required: false, displayOrder: 4 },
        { name: "nationalId", labelEn: "National ID / Passport No.", labelAr: "رقم الهوية / جواز السفر", type: "text", required: true, displayOrder: 5 },
        { name: "email", labelEn: "University Email", labelAr: "البريد الجامعي", type: "email", required: true, displayOrder: 6 },
        { name: "phone", labelEn: "Mobile Phone", labelAr: "رقم الجوال", type: "phone", required: true, displayOrder: 7 },
      ],
    },
    {
      id: "program_enrollment",
      titleEn: "Faculty & Degree Program",
      titleAr: "الكلية والتخصص الأكاديمي",
      fields: [
        { name: "facultyId", labelEn: "Faculty / College", labelAr: "الكلية", type: "select", required: true, displayOrder: 1 },
        { name: "majorId", labelEn: "Major / Program", labelAr: "التخصص الأكاديمي", type: "select", required: true, displayOrder: 2 },
        { name: "degreeType", labelEn: "Degree Level", labelAr: "الدرجة العلمية", type: "select", required: true, displayOrder: 3, options: [
          { value: "bachelor", labelEn: "Bachelor's Degree", labelAr: "بكالوريوس" },
          { value: "master", labelEn: "Master's Degree", labelAr: "ماجستير" },
          { value: "phd", labelEn: "Doctorate (Ph.D.)", labelAr: "دكتوراه" },
          { value: "diploma", labelEn: "Associate Diploma", labelAr: "دبلوم عالي" },
        ]},
      ],
    },
  ],
};

// ═══════════════════════════════════════════════════════════════
// 3. Vocational & Training Institute Blueprint
// ═══════════════════════════════════════════════════════════════

export const INSTITUTE_BLUEPRINT: InstitutionBlueprint = {
  type: "vocational_institute",
  nameEn: "Vocational & Training Institute",
  nameAr: "معهد مهني / مركز تدريب وتأهيل",
  descriptionEn: "Tailored for training centers and institutes with certificate tracks, training cohorts, and flexible modules.",
  descriptionAr: "مصمم للمعاهد ومراكز التدريب المهني مع دعم المسارات التدريبية، الدفعات، والشهادات المعتمدة.",
  modules: {
    guardians: false,
    homeroom: false,
    transportation: false,
    assessments: true,
    attendance: true,
    behavior: false,
    curriculum: true,
    scheduling: true,
    finance: true,
    creditHours: false,
    academicAdvisor: false,
  },
  studentProfileTabs: [
    { id: "overview", labelEn: "Trainee Profile", labelAr: "ملف المتدرب", icon: "User" },
    { id: "track", labelEn: "Training Track & Progress", labelAr: "المسار التدريبي والتقدم", icon: "Award" },
    { id: "attendance", labelEn: "Attendance & Hours", labelAr: "ساعات الحضور", icon: "Clock" },
    { id: "fees", labelEn: "Invoices & Payments", labelAr: "الرسوم والدفعات", icon: "CreditCard" },
    { id: "certificates", labelEn: "Certificates Issued", labelAr: "الشهادات الصادرة", icon: "FileCheck" },
  ],
  studentRegistrationSections: [
    {
      id: "trainee_demographics",
      titleEn: "Trainee Information",
      titleAr: "بيانات المتدرب",
      fields: [
        { name: "fullName", labelEn: "Full Name", labelAr: "الاسم الكامل", type: "text", required: true, displayOrder: 1 },
        { name: "fullNameAr", labelEn: "Full Name (AR)", labelAr: "الاسم الكامل بالعربية للشهادة", type: "text", required: true, displayOrder: 2 },
        { name: "phone", labelEn: "Mobile Phone (WhatsApp)", labelAr: "رقم الجوال (واتساب)", type: "phone", required: true, displayOrder: 3 },
        { name: "email", labelEn: "Email Address", labelAr: "البريد الإلكتروني", type: "email", required: true, displayOrder: 4 },
        { name: "nationalId", labelEn: "ID / Residence No.", labelAr: "رقم الهوية / الإقامة", type: "text", required: true, displayOrder: 5 },
      ],
    },
    {
      id: "course_track",
      titleEn: "Course & Cohort Selection",
      titleAr: "المسار والدفعة التدريبية",
      fields: [
        { name: "programId", labelEn: "Training Program / Track", labelAr: "البرنامج التدريبي", type: "select", required: true, displayOrder: 1 },
        { name: "cohortBatch", labelEn: "Cohort Batch", labelAr: "الدفعة / المجموعة", type: "select", required: true, displayOrder: 2 },
      ],
    },
  ],
};

// ═══════════════════════════════════════════════════════════════
// Registry & Helper Functions
// ═══════════════════════════════════════════════════════════════

export const BLUEPRINTS: Record<InstitutionType, InstitutionBlueprint> = {
  k12_school: SCHOOL_BLUEPRINT,
  university: UNIVERSITY_BLUEPRINT,
  vocational_institute: INSTITUTE_BLUEPRINT,
  language_center: INSTITUTE_BLUEPRINT,
};

/**
 * Resolves the appropriate precompiled Blueprint for a given organization type.
 */
export function getBlueprint(type?: string | null): InstitutionBlueprint {
  if (type && type in BLUEPRINTS) {
    return BLUEPRINTS[type as InstitutionType];
  }
  return SCHOOL_BLUEPRINT;
}

/**
 * Dynamically merges precompiled Blueprint definitions with organization-specific custom fields and settings.
 */
export function mergeBlueprintWithOrgSettings(
  blueprint: InstitutionBlueprint,
  customFields?: FieldDefinition[],
  disabledModules?: string[],
): InstitutionBlueprint {
  const mergedModules = { ...blueprint.modules };
  if (disabledModules) {
    for (const mod of disabledModules) {
      mergedModules[mod] = false;
    }
  }

  const sections = blueprint.studentRegistrationSections.map((s) => ({
    ...s,
    fields: [...s.fields],
  }));

  if (customFields && customFields.length > 0) {
    sections.push({
      id: "custom_fields",
      titleEn: "Additional Information",
      titleAr: "معلومات إضافية مخصصة",
      fields: customFields.map((f, i) => ({
        ...f,
        isCustom: true,
        displayOrder: 100 + i,
      })),
    });
  }

  return {
    ...blueprint,
    modules: mergedModules,
    studentRegistrationSections: sections,
  };
}
