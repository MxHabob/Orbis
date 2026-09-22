<script lang="ts">
import { Clipboard, Home } from "@orbis/icons";
</script>
export interface Terminology {
  stage: string;
  stage_ar: string;
  classroom: string;
  classroom_ar: string;
  division: string;
  division_ar: string;
  subject: string;
  subject_ar: string;
  calendar: string;
  calendar_ar: string;
}

export interface DefaultGrade {
  level: string;
  name: string;
  name_ar: string;
  color: string;
  divisions: string[];
}

export interface DefaultTimeSlot {
  name: string;
  name_ar: string;
  start_time: string;
  end_time: string;
  is_break?: boolean;
  break_type?: "recess" | "lunch" | "prayer" | "custom";
  applicable_days?: number[];
}

export interface DefaultSubject {
  code: string;
  name: string;
  name_ar: string;
  color: string;
  icon: string;
  sort_order: number;
  enabled?: boolean;
}

export interface PolicySettings {
  requires_approval: boolean;
  allow_no_guardian: boolean;
  auto_assign_fees: boolean;
  allow_duplicate_national_id: boolean;
}

export interface InstitutionProfile {
  type: string;
  name: string;
  name_ar: string;
  description: string;
  description_ar: string;
  terminology: Terminology;
  stages: DefaultGrade[];
  timeSlots: DefaultTimeSlot[];
  subjects: DefaultSubject[];
  activeFields: Record<string, boolean>;
  requiredFields: Record<string, boolean>;
  preventDuplicates: Record<string, boolean>;
  requiredDocuments: string[];
  numberingPattern: string;
  policySettings?: PolicySettings;
}

export const INSTITUTION_PROFILES: Record<string, InstitutionProfile> = {
  school: {
    type: "school",
    name: "School",
    name_ar: "مدرسة",
    description: "Primary, preparatory, or secondary school setup.",
    description_ar: "إعدادات مدرسة ابتدائية، أساسية، أو ثانوية.",
    terminology: {
      stage: "Grade",
      stage_ar: "الصف الدراسي",
      classroom: "Classroom",
      classroom_ar: "الفصل الدراسي",
      division: "Section",
      division_ar: "الشعبة",
      subject: "Subject",
      subject_ar: "المادة الدراسية",
      calendar: "Academic Year",
      calendar_ar: "العام الدراسي",
    },
    numberingPattern: "STU-{YEAR}-{SEQ}",
    activeFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      nationality: true,
      place_of_birth: true,
      blood_type: true,
      emergency_contact: true,
      medical_notes: true,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: false,
      phone: true,
      birth_date: true,
      sex: true,
      address: false,
      nationality: false,
      place_of_birth: false,
      blood_type: false,
      emergency_contact: true,
      medical_notes: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: false,
      phone: true,
    },
    policySettings: {
      requires_approval: false,
      allow_no_guardian: false,
      auto_assign_fees: true,
      allow_duplicate_national_id: false,
    },
    requiredDocuments: [
      "شهادة الميلاد (Birth Certificate)",
      "كرت اللقاحات (Vaccination Card)",
      "آخر شهادة دراسية (Previous Report Card)",
      "صورة بطاقة ولي الأمر (Guardian ID Clipboard)",
    ],
    stages: [
      {
        level: "kg1",
        name: "KG1",
        name_ar: "روضة أولى",
        color: "#f43f5e",
        divisions: ["A"],
      },
      {
        level: "kg2",
        name: "KG2",
        name_ar: "روضة ثانية",
        color: "#ec4899",
        divisions: ["A"],
      },
      {
        level: "grade1",
        name: "Grade 1",
        name_ar: "الصف الأول",
        color: "#d946ef",
        divisions: ["A", "B"],
      },
      {
        level: "grade2",
        name: "Grade 2",
        name_ar: "الصف الثاني",
        color: "#a855f7",
        divisions: ["A", "B"],
      },
      {
        level: "grade3",
        name: "Grade 3",
        name_ar: "الصف الثالث",
        color: "#6366f1",
        divisions: ["A", "B"],
      },
      {
        level: "grade4",
        name: "Grade 4",
        name_ar: "الصف الرابع",
        color: "#3b82f6",
        divisions: ["A"],
      },
      {
        level: "grade5",
        name: "Grade 5",
        name_ar: "الصف الخامس",
        color: "#0ea5e9",
        divisions: ["A"],
      },
      {
        level: "grade6",
        name: "Grade 6",
        name_ar: "الصف السادس",
        color: "#06b6d4",
        divisions: ["A"],
      },
      {
        level: "grade7",
        name: "Grade 7",
        name_ar: "الصف السابع",
        color: "#14b8a6",
        divisions: ["A"],
      },
      {
        level: "grade8",
        name: "Grade 8",
        name_ar: "الصف الثامن",
        color: "#10b981",
        divisions: ["A"],
      },
      {
        level: "grade9",
        name: "Grade 9",
        name_ar: "الصف التاسع",
        color: "#22c55e",
        divisions: ["A"],
      },
      {
        level: "grade10",
        name: "Grade 10 (1st Secondary)",
        name_ar: "الصف الأول ثانوي",
        color: "#16a34a",
        divisions: ["A"],
      },
      {
        level: "grade11",
        name: "Grade 11 (2nd Secondary)",
        name_ar: "الصف الثاني ثانوي",
        color: "#0284c7",
        divisions: ["A"],
      },
      {
        level: "grade12",
        name: "Grade 12 (3rd Secondary)",
        name_ar: "الصف الثالث ثانوي",
        color: "#7c3aed",
        divisions: ["A"],
      },
    ],
    timeSlots: [
      {
        name: "1st Period",
        name_ar: "الحصة الأولى",
        start_time: "08:00",
        end_time: "08:40",
      },
      {
        name: "2nd Period",
        name_ar: "الحصة الثانية",
        start_time: "08:40",
        end_time: "09:20",
      },
      {
        name: "3rd Period",
        name_ar: "الحصة الثالثة",
        start_time: "09:20",
        end_time: "10:00",
      },
      {
        name: "Recess",
        name_ar: "الفسحة / استراحة",
        start_time: "10:00",
        end_time: "10:20",
        is_break: true,
        break_type: "recess",
      },
      {
        name: "4th Period",
        name_ar: "الحصة الرابعة",
        start_time: "10:20",
        end_time: "11:00",
      },
      {
        name: "5th Period",
        name_ar: "الحصة الخامسة",
        start_time: "11:00",
        end_time: "11:40",
      },
      {
        name: "6th Period",
        name_ar: "الحصة السادسة",
        start_time: "11:40",
        end_time: "12:20",
      },
      {
        name: "7th Period",
        name_ar: "الحصة السابعة",
        start_time: "12:20",
        end_time: "13:00",
      },
    ],
    subjects: [
      {
        code: "QUR",
        name: "Holy Quran",
        name_ar: "القرآن الكريم",
        color: "#10b981",
        icon: "book-open",
        sort_order: 1,
      },
      {
        code: "ISL",
        name: "Islamic Studies",
        name_ar: "الدراسات الإسلامية",
        color: "#0d9488",
        icon: "bookmark",
        sort_order: 2,
      },
      {
        code: "ARB",
        name: "Arabic Language",
        name_ar: "اللغة العربية",
        color: "#f59e0b",
        icon: "pen-tool",
        sort_order: 3,
      },
      {
        code: "MTH",
        name: "Mathematics",
        name_ar: "الرياضيات",
        color: "#3b82f6",
        icon: "binary",
        sort_order: 4,
      },
      {
        code: "SCI",
        name: "Science",
        name_ar: "العلوم",
        color: "#8b5cf6",
        icon: "flask-conical",
        sort_order: 5,
      },
      {
        code: "ENG",
        name: "English Language",
        name_ar: "اللغة الإنجليزية",
        color: "#6366f1",
        icon: "languages",
        sort_order: 6,
      },
      {
        code: "SOC",
        name: "Social Studies",
        name_ar: "الدراسات الاجتماعية",
        color: "#f97316",
        icon: "globe",
        sort_order: 7,
      },
      {
        code: "ART",
        name: "Art",
        name_ar: "الفن",
        color: "#f43f5e",
        icon: "palette",
        sort_order: 8,
      },
      {
        code: "MUS",
        name: "Music",
        name_ar: "الموسيقى",
        color: "#8b5cf6",
        icon: "music",
        sort_order: 9,
      },
      {
        code: "PE",
        name: "Physical Education",
        name_ar: "الرياضة",
        color: "#f97316",
        icon: "activity",
        sort_order: 10,
      },
      {
        code: "COMP",
        name: "Computer Science",
        name_ar: "الحاسوب",
        color: "#06b6d4",
        icon: "monitor",
        sort_order: 11,
      },
      {
        code: "PHYS",
        name: "Physics",
        name_ar: "الفيزياء",
        color: "#0284c7",
        icon: "atom",
        sort_order: 12,
        enabled: false,
      },
      {
        code: "CHEM",
        name: "Chemistry",
        name_ar: "الكيمياء",
        color: "#8b5cf6",
        icon: "flask-conical",
        sort_order: 13,
        enabled: false,
      },
      {
        code: "BIO",
        name: "Biology",
        name_ar: "الأحياء",
        color: "#10b981",
        icon: "dna",
        sort_order: 14,
        enabled: false,
      },
      {
        code: "SOC_CIV",
        name: "Civics & Community Studies",
        name_ar: "دراسات المجتمع والتربية الوطنية",
        color: "#f97316",
        icon: "users",
        sort_order: 15,
        enabled: false,
      },
      {
        code: "HIST",
        name: "History",
        name_ar: "التاريخ",
        color: "#b45309",
        icon: "landmark",
        sort_order: 16,
        enabled: false,
      },
      {
        code: "GEOG",
        name: "Geography",
        name_ar: "الجغرافيا",
        color: "#0d9488",
        icon: "globe",
        sort_order: 17,
        enabled: false,
      },
      {
        code: "PSYCH",
        name: "Psychology",
        name_ar: "علم النفس",
        color: "#ec4899",
        icon: "brain",
        sort_order: 18,
        enabled: false,
      },
      {
        code: "PHIL",
        name: "Philosophy",
        name_ar: "الفلسفة",
        color: "#6366f1",
        icon: "feather",
        sort_order: 19,
        enabled: false,
      },
      {
        code: "LOGIC",
        name: "Logic",
        name_ar: "علم المنطق",
        color: "#e11d48",
        icon: "cpu",
        sort_order: 20,
        enabled: false,
      },
      {
        code: "CALC",
        name: "Calculus & Analysis",
        name_ar: "تفاضل وتكامل",
        color: "#2563eb",
        icon: "binary",
        sort_order: 21,
        enabled: false,
      },
      {
        code: "ALG_GEOM",
        name: "Algebra & Geometry",
        name_ar: "جبر وهندسة",
        color: "#4f46e5",
        icon: "shapes",
        sort_order: 22,
        enabled: false,
      },
    ],
  },
  university: {
    type: "university",
    name: "University",
    name_ar: "جامعة",
    description: "Higher education, college, or university setup.",
    description_ar: "إعدادات التعليم العالي، الكلية، أو الجامعة.",
    terminology: {
      stage: "Academic Level",
      stage_ar: "المستوى الأكاديمي",
      classroom: "Lecture Hall",
      classroom_ar: "قاعة المحاضرات",
      division: "Section",
      division_ar: "الشعبة",
      subject: "Course",
      subject_ar: "المساق الدراسي",
      calendar: "Academic Period",
      calendar_ar: "الدورة الأكاديمية",
    },
    numberingPattern: "UNI-{YEAR}-{STAGE}-{SEQ}",
    activeFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      nationality: true,
      passport_number: true,
      place_of_birth: true,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      nationality: true,
      passport_number: false,
      place_of_birth: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: true,
      phone: true,
    },
    policySettings: {
      requires_approval: true,
      allow_no_guardian: true,
      auto_assign_fees: true,
      allow_duplicate_national_id: false,
    },
    requiredDocuments: [
      "شهادة الثانوية العامة (High Home Certificate)",
      "صورة جواز السفر / الهوية (National ID / Passport copy)",
      "صور شخصية حديثة (Personal Photos)",
      "شهادة خلو من السوابق (Good Conduct Certificate)",
    ],
    stages: [
      {
        level: "level1",
        name: "Level 1 (Freshman)",
        name_ar: "المستوى الأول",
        color: "#3b82f6",
        divisions: ["A", "B"],
      },
      {
        level: "level2",
        name: "Level 2 (Sophomore)",
        name_ar: "المستوى الثاني",
        color: "#0ea5e9",
        divisions: ["A"],
      },
      {
        level: "level3",
        name: "Level 3 (Junior)",
        name_ar: "المستوى الثالث",
        color: "#10b981",
        divisions: ["A"],
      },
      {
        level: "level4",
        name: "Level 4 (Senior)",
        name_ar: "المستوى الرابع",
        color: "#14b8a6",
        divisions: ["A"],
      },
    ],
    timeSlots: [
      {
        name: "Lecture 1",
        name_ar: "المحاضرة الأولى",
        start_time: "08:00",
        end_time: "09:30",
      },
      {
        name: "Lecture 2",
        name_ar: "المحاضرة الثانية",
        start_time: "09:45",
        end_time: "11:15",
      },
      {
        name: "Lecture 3",
        name_ar: "المحاضرة الثالثة",
        start_time: "11:30",
        end_time: "13:00",
      },
      {
        name: "Lecture 4",
        name_ar: "المحاضرة الرابعة",
        start_time: "13:15",
        end_time: "14:45",
      },
    ],
    subjects: [
      {
        code: "CS101",
        name: "Intro to Computer Science",
        name_ar: "مقدمة في علوم الحاسوب",
        color: "#6366f1",
        icon: "laptop",
        sort_order: 1,
      },
      {
        code: "MATH101",
        name: "Calculus I",
        name_ar: "تفاضل وتكامل 1",
        color: "#3b82f6",
        icon: "binary",
        sort_order: 2,
      },
      {
        code: "PHYS101",
        name: "General Physics I",
        name_ar: "الفيزياء العامة 1",
        color: "#f59e0b",
        icon: "atom",
        sort_order: 3,
      },
      {
        code: "ENG101",
        name: "Academic English I",
        name_ar: "اللغة الإنجليزية الأكاديمية 1",
        color: "#10b981",
        icon: "languages",
        sort_order: 4,
      },
    ],
  },
  institute: {
    type: "institute",
    name: "Institute",
    name_ar: "معهد",
    description: "Language center, vocational institute, or technical school.",
    description_ar: "معهد لغات، معهد مهني، أو مدرسة فنية تقنية.",
    terminology: {
      stage: "Training Level",
      stage_ar: "المستوى التدريبي",
      classroom: "Training Room",
      classroom_ar: "قاعة التدريب",
      division: "Group",
      division_ar: "المجموعة",
      subject: "Module",
      subject_ar: "الوحدة التدريبية",
      calendar: "Training Year",
      calendar_ar: "العام التدريبي",
    },
    numberingPattern: "INST-{YEAR}-{SEQ}",
    activeFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      nationality: true,
      emergency_contact: true,
      previous_school: false,
    },
    requiredFields: {
      national_id: false,
      email: false,
      phone: true,
      birth_date: false,
      sex: true,
      address: false,
      nationality: false,
      emergency_contact: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: false,
      email: true,
      phone: true,
    },
    policySettings: {
      requires_approval: false,
      allow_no_guardian: true,
      auto_assign_fees: true,
      allow_duplicate_national_id: true,
    },
    requiredDocuments: [
      "صورة الهوية الوطنية (ID Clipboard)",
      "المؤهلات العلمية السابقة (Educational Qualifications)",
    ],
    stages: [
      {
        level: "beginner",
        name: "Beginner (A1)",
        name_ar: "مبتدئ (A1)",
        color: "#10b981",
        divisions: ["G1"],
      },
      {
        level: "elementary",
        name: "Elementary (A2)",
        name_ar: "ابتدائي (A2)",
        color: "#22c55e",
        divisions: ["G1"],
      },
      {
        level: "intermediate",
        name: "Intermediate (B1)",
        name_ar: "متوسط (B1)",
        color: "#f59e0b",
        divisions: ["G1"],
      },
      {
        level: "advanced",
        name: "Advanced (B2)",
        name_ar: "متقدم (B2)",
        color: "#ef4444",
        divisions: ["G1"],
      },
    ],
    timeSlots: [
      {
        name: "Morning Session",
        name_ar: "الفترة الصباحية",
        start_time: "09:00",
        end_time: "12:00",
      },
      {
        name: "Evening Session",
        name_ar: "الفترة المسائية",
        start_time: "16:00",
        end_time: "19:00",
      },
    ],
    subjects: [
      {
        code: "ENG-CONV",
        name: "English Conversation",
        name_ar: "المحادثة الإنجليزية",
        color: "#6366f1",
        icon: "message-square",
        sort_order: 1,
      },
      {
        code: "BUS-WRITE",
        name: "Business Writing",
        name_ar: "الكتابة التجارية",
        color: "#10b981",
        icon: "pen-tool",
        sort_order: 2,
      },
      {
        code: "IT-FUND",
        name: "IT Fundamentals",
        name_ar: "أساسيات تكنولوجيا المعلومات",
        color: "#3b82f6",
        icon: "cpu",
        sort_order: 3,
      },
    ],
  },
  training_center: {
    type: "training_center",
    name: "Training Center",
    name_ar: "مركز تدريب",
    description: "Short term courses, bootcamps, or professional development.",
    description_ar: "دورات قصيرة المدى، معسكرات برمجية، أو تطوير مهني.",
    terminology: {
      stage: "Course Level",
      stage_ar: "مستوى الدورة",
      classroom: "Lab",
      classroom_ar: "المختبر",
      division: "Cohort",
      division_ar: "الدفعة",
      subject: "Program",
      subject_ar: "البرنامج التدريبي",
      calendar: "Semester Period",
      calendar_ar: "دورة الفصول",
    },
    numberingPattern: "TR-{YEAR}-{SEQ}",
    activeFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: false,
      sex: true,
      address: false,
      previous_school: false,
    },
    requiredFields: {
      national_id: false,
      email: true,
      phone: true,
      birth_date: false,
      sex: false,
      address: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: false,
      email: true,
      phone: true,
    },
    policySettings: {
      requires_approval: false,
      allow_no_guardian: true,
      auto_assign_fees: false,
      allow_duplicate_national_id: true,
    },
    requiredDocuments: [
      "صورة الهوية الوطنية (ID Clipboard)",
      "موافقة جهة العمل (Employer Approval)",
      "السيرة الذاتية (Resume / CV)",
    ],
    stages: [
      {
        level: "short_courses",
        name: "Short Courses",
        name_ar: "دورات قصيرة",
        color: "#a855f7",
        divisions: ["C1"],
      },
      {
        level: "professional_certs",
        name: "Professional Certifications",
        name_ar: "شهادات مهنية",
        color: "#ec4899",
        divisions: ["C1"],
      },
      {
        level: "bootcamps",
        name: "Executive Bootcamps",
        name_ar: "معسكرات تدريبية",
        color: "#f43f5e",
        divisions: ["C1"],
      },
    ],
    timeSlots: [
      {
        name: "Morning Slot",
        name_ar: "الفترة الصباحية",
        start_time: "09:00",
        end_time: "13:00",
      },
      {
        name: "Afternoon Slot",
        name_ar: "الفترة المسائية",
        start_time: "14:00",
        end_time: "18:00",
      },
      {
        name: "Weekend Bootcamp",
        name_ar: "معسكر نهاية الأسبوع",
        start_time: "09:00",
        end_time: "17:00",
      },
    ],
    subjects: [
      {
        code: "PMP",
        name: "Project Management (PMP)",
        name_ar: "إدارة المشاريع الاحترافية",
        color: "#f59e0b",
        icon: "briefcase",
        sort_order: 1,
      },
      {
        code: "DS-BOOTCAMP",
        name: "Data Science Bootcamp",
        name_ar: "معسكر علوم البيانات",
        color: "#3b82f6",
        icon: "bar-chart-2",
        sort_order: 2,
      },
      {
        code: "DIGITAL-MKT",
        name: "Digital Marketing",
        name_ar: "التسويق الرقمي",
        color: "#10b981",
        icon: "trending-up",
        sort_order: 3,
      },
    ],
  },
  custom: {
    type: "custom",
    name: "Custom",
    name_ar: "مخصص",
    description: "Define a tailored, customized configuration.",
    description_ar: "تحديد تهيئة وتسميات مخصصة بالكامل للمؤسسة.",
    terminology: {
      stage: "Level",
      stage_ar: "المستوى",
      classroom: "Room",
      classroom_ar: "الغرفة/القاعة",
      division: "Section",
      division_ar: "الشعبة/المجموعة",
      subject: "Subject",
      subject_ar: "المادة/المساق",
      calendar: "Academic Period",
      calendar_ar: "الدورة الأكاديمية",
    },
    numberingPattern: "ORG-{YEAR}-{SEQ}",
    activeFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: false,
      phone: true,
      birth_date: true,
      sex: true,
      address: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: false,
      phone: true,
    },
    requiredDocuments: ["صورة الهوية الوطنية (ID Clipboard)"],
    stages: [
      {
        level: "level1",
        name: "Level 1",
        name_ar: "المستوى الأول",
        color: "#3b82f6",
        divisions: ["1"],
      },
    ],
    timeSlots: [
      {
        name: "Slot 1",
        name_ar: "الفترة الأولى",
        start_time: "08:00",
        end_time: "09:00",
      },
    ],
    subjects: [
      {
        code: "SUBJ1",
        name: "Subject 1",
        name_ar: "المادة الأولى",
        color: "#6366f1",
        icon: "book",
        sort_order: 1,
      },
    ],
  },
};

export const DEFAULT_PERIOD_TEMPLATES: import("./types").PeriodTemplateDef[] = [
  {
    id: "template-two-semesters",
    name: "Two Semesters (Fall & Spring - 50% / 50%)",
    name_ar: "نظام الفصلين الدراسيين (الترم الأول والثاني - 50% لكل فصل)",
    structure: [
      {
        name: "First Semester",
        name_ar: "الفصل الدراسي الأول",
        type: "semester",
        weight: 50,
      },
      {
        name: "Second Semester",
        name_ar: "الفصل الدراسي الثاني",
        type: "semester",
        weight: 50,
      },
    ],
  },
  {
    id: "template-three-terms",
    name: "Three Terms / Trimesters (33.3% each)",
    name_ar: "نظام الفصول الثلاثية (ثلاثة فصول دراسية متساوية)",
    structure: [
      {
        name: "First Term",
        name_ar: "الفصل الدراسي الأول",
        type: "term",
        weight: 33.33,
      },
      {
        name: "Second Term",
        name_ar: "الفصل الدراسي الثاني",
        type: "term",
        weight: 33.33,
      },
      {
        name: "Third Term",
        name_ar: "الفصل الدراسي الثالث",
        type: "term",
        weight: 33.34,
      },
    ],
  },
  {
    id: "template-four-quarters",
    name: "Four Quarters (25% each)",
    name_ar: "نظام الأرباع السنوية (أربعة أرباع - 25% لكل ربع)",
    structure: [
      {
        name: "First Quarter",
        name_ar: "الربع الأول",
        type: "quarter",
        weight: 25,
      },
      {
        name: "Second Quarter",
        name_ar: "الربع الثاني",
        type: "quarter",
        weight: 25,
      },
      {
        name: "Third Quarter",
        name_ar: "الربع الثالث",
        type: "quarter",
        weight: 25,
      },
      {
        name: "Fourth Quarter",
        name_ar: "الربع الرابع",
        type: "quarter",
        weight: 25,
      },
    ],
  },
  {
    id: "template-semesters-monthly",
    name: "Two Semesters with Monthly Assessment",
    name_ar: "فصلين دراسيين مع تقييمات واختبارات شهرية",
    structure: [
      {
        name: "First Semester",
        name_ar: "الفصل الدراسي الأول",
        type: "semester",
        weight: 50,
        children: [
          {
            name: "Month 1 Assessment",
            name_ar: "تقييم الشهر الأول",
            type: "month",
            weight: 33.33,
          },
          {
            name: "Month 2 Assessment",
            name_ar: "تقييم الشهر الثاني",
            type: "month",
            weight: 33.33,
          },
          {
            name: "Semester 1 Final Exam",
            name_ar: "امتحان نهاية الفصل الأول",
            type: "exam",
            weight: 33.34,
          },
        ],
      },
      {
        name: "Second Semester",
        name_ar: "الفصل الدراسي الثاني",
        type: "semester",
        weight: 50,
        children: [
          {
            name: "Month 1 Assessment",
            name_ar: "تقييم الشهر الأول",
            type: "month",
            weight: 33.33,
          },
          {
            name: "Month 2 Assessment",
            name_ar: "تقييم الشهر الثاني",
            type: "month",
            weight: 33.33,
          },
          {
            name: "Semester 2 Final Exam",
            name_ar: "امتحان نهاية الفصل الثاني",
            type: "exam",
            weight: 33.34,
          },
        ],
      },
    ],
  },
  {
    id: "template-university-summer",
    name: "Two Semesters + Summer Term",
    name_ar: "فصلين دراسيين + فصل صيفي (جامعي ومعاهد)",
    structure: [
      {
        name: "Fall Semester",
        name_ar: "الفصل الدراسي الأول (الخريف)",
        type: "semester",
        weight: 45,
      },
      {
        name: "Spring Semester",
        name_ar: "الفصل الدراسي الثاني (الربيع)",
        type: "semester",
        weight: 45,
      },
      {
        name: "Summer Term",
        name_ar: "الفصل الصيفي",
        type: "term",
        weight: 10,
      },
    ],
  },
  {
    id: "template-full-year",
    name: "Full Academic Year (Single Continuous Period)",
    name_ar: "عام دراسي سنوي متصل (بدون تقسيم فصول)",
    structure: [
      {
        name: "Full Academic Year",
        name_ar: "العام الدراسي الكامل",
        type: "year",
        weight: 100,
      },
    ],
  },
];
