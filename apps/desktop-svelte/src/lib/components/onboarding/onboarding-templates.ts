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
}

export interface DefaultSubject {
  code: string;
  name: string;
  name_ar: string;
  color: string;
  icon: string;
  sort_order: number;
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
      mother_name: true,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: false,
      phone: true,
      birth_date: true,
      sex: true,
      address: false,
      mother_name: true,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: false,
      phone: true,
    },
    requiredDocuments: [
      "شهادة الميلاد (Birth Certificate)",
      "كرت اللقاحات (Vaccination Card)",
      "آخر شهادة دراسية (Previous Report Card)",
      "صورة بطاقة ولي الأمر (Guardian ID Copy)"
    ],
    stages: [
      { level: "kg1", name: "KG1", name_ar: "روضة أولى", color: "#f43f5e", divisions: ["أ"] },
      { level: "kg2", name: "KG2", name_ar: "روضة ثانية", color: "#ec4899", divisions: ["أ"] },
      { level: "grade1", name: "Grade 1", name_ar: "الصف الأول", color: "#d946ef", divisions: ["أ", "ب"] },
      { level: "grade2", name: "Grade 2", name_ar: "الصف الثاني", color: "#a855f7", divisions: ["أ", "ب"] },
      { level: "grade3", name: "Grade 3", name_ar: "الصف الثالث", color: "#6366f1", divisions: ["أ", "ب"] },
      { level: "grade4", name: "Grade 4", name_ar: "الصف الرابع", color: "#3b82f6", divisions: ["أ"] },
      { level: "grade5", name: "Grade 5", name_ar: "الصف الخامس", color: "#0ea5e9", divisions: ["أ"] },
      { level: "grade6", name: "Grade 6", name_ar: "الصف السادس", color: "#06b6d4", divisions: ["أ"] },
      { level: "grade7", name: "Grade 7", name_ar: "الصف السابع", color: "#14b8a6", divisions: ["أ"] },
      { level: "grade8", name: "Grade 8", name_ar: "الصف الثامن", color: "#10b981", divisions: ["أ"] },
      { level: "grade9", name: "Grade 9", name_ar: "الصف التاسع", color: "#22c55e", divisions: ["أ"] },
    ],
    timeSlots: [
      { name: "1st Period", name_ar: "الحصة الأولى", start_time: "08:00", end_time: "08:45" },
      { name: "2nd Period", name_ar: "الحصة الثانية", start_time: "08:45", end_time: "09:30" },
      { name: "3rd Period", name_ar: "الحصة الثالثة", start_time: "09:45", end_time: "10:30" },
      { name: "4th Period", name_ar: "الحصة الرابعة", start_time: "10:30", end_time: "11:15" },
      { name: "5th Period", name_ar: "الحصة الخامسة", start_time: "11:30", end_time: "12:15" },
    ],
    subjects: [
      { code: "QUR", name: "Holy Quran", name_ar: "القرآن الكريم", color: "#10b981", icon: "book-open", sort_order: 1 },
      { code: "ISL", name: "Islamic Studies", name_ar: "الدراسات الإسلامية", color: "#0d9488", icon: "bookmark", sort_order: 2 },
      { code: "ARB", name: "Arabic Language", name_ar: "اللغة العربية", color: "#f59e0b", icon: "pen-tool", sort_order: 3 },
      { code: "MTH", name: "Mathematics", name_ar: "الرياضيات", color: "#3b82f6", icon: "binary", sort_order: 4 },
      { code: "SCI", name: "Science", name_ar: "العلوم", color: "#8b5cf6", icon: "flask-conical", sort_order: 5 },
      { code: "ENG", name: "English Language", name_ar: "اللغة الإنجليزية", color: "#6366f1", icon: "languages", sort_order: 6 },
      { code: "SOC", name: "Social Studies", name_ar: "الدراسات الاجتماعية", color: "#f97316", icon: "globe", sort_order: 7 },
    ]
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
      calendar: "Academic Cycle",
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
      mother_name: false,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: true,
      phone: true,
      birth_date: true,
      sex: true,
      address: true,
      mother_name: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: true,
      phone: true,
    },
    requiredDocuments: [
      "شهادة الثانوية العامة (High School Certificate)",
      "صورة جواز السفر / الهوية (National ID / Passport copy)",
      "صور شخصية حديثة (Personal Photos)",
      "شهادة خلو من السوابق (Good Conduct Certificate)"
    ],
    stages: [
      { level: "level1", name: "Level 1 (Freshman)", name_ar: "المستوى الأول", color: "#3b82f6", divisions: ["A", "B"] },
      { level: "level2", name: "Level 2 (Sophomore)", name_ar: "المستوى الثاني", color: "#0ea5e9", divisions: ["A"] },
      { level: "level3", name: "Level 3 (Junior)", name_ar: "المستوى الثالث", color: "#10b981", divisions: ["A"] },
      { level: "level4", name: "Level 4 (Senior)", name_ar: "المستوى الرابع", color: "#14b8a6", divisions: ["A"] },
    ],
    timeSlots: [
      { name: "Lecture 1", name_ar: "المحاضرة الأولى", start_time: "08:00", end_time: "09:30" },
      { name: "Lecture 2", name_ar: "المحاضرة الثانية", start_time: "09:45", end_time: "11:15" },
      { name: "Lecture 3", name_ar: "المحاضرة الثالثة", start_time: "11:30", end_time: "13:00" },
      { name: "Lecture 4", name_ar: "المحاضرة الرابعة", start_time: "13:15", end_time: "14:45" },
    ],
    subjects: [
      { code: "CS101", name: "Intro to Computer Science", name_ar: "مقدمة في علوم الحاسوب", color: "#6366f1", icon: "laptop", sort_order: 1 },
      { code: "MATH101", name: "Calculus I", name_ar: "تفاضل وتكامل 1", color: "#3b82f6", icon: "binary", sort_order: 2 },
      { code: "PHYS101", name: "General Physics I", name_ar: "الفيزياء العامة 1", color: "#f59e0b", icon: "atom", sort_order: 3 },
      { code: "ENG101", name: "Academic English I", name_ar: "اللغة الإنجليزية الأكاديمية 1", color: "#10b981", icon: "languages", sort_order: 4 },
    ]
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
      mother_name: false,
      previous_school: false,
    },
    requiredFields: {
      national_id: false,
      email: false,
      phone: true,
      birth_date: false,
      sex: true,
      address: false,
      mother_name: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: false,
      email: true,
      phone: true,
    },
    requiredDocuments: [
      "صورة الهوية الوطنية (ID Copy)",
      "المؤهلات العلمية السابقة (Educational Qualifications)"
    ],
    stages: [
      { level: "beginner", name: "Beginner (A1)", name_ar: "مبتدئ (A1)", color: "#10b981", divisions: ["G1"] },
      { level: "elementary", name: "Elementary (A2)", name_ar: "ابتدائي (A2)", color: "#22c55e", divisions: ["G1"] },
      { level: "intermediate", name: "Intermediate (B1)", name_ar: "متوسط (B1)", color: "#f59e0b", divisions: ["G1"] },
      { level: "advanced", name: "Advanced (B2)", name_ar: "متقدم (B2)", color: "#ef4444", divisions: ["G1"] },
    ],
    timeSlots: [
      { name: "Morning Session", name_ar: "الفترة الصباحية", start_time: "09:00", end_time: "12:00" },
      { name: "Evening Session", name_ar: "الفترة المسائية", start_time: "16:00", end_time: "19:00" },
    ],
    subjects: [
      { code: "ENG-CONV", name: "English Conversation", name_ar: "المحادثة الإنجليزية", color: "#6366f1", icon: "message-square", sort_order: 1 },
      { code: "BUS-WRITE", name: "Business Writing", name_ar: "الكتابة التجارية", color: "#10b981", icon: "pen-tool", sort_order: 2 },
      { code: "IT-FUND", name: "IT Fundamentals", name_ar: "أساسيات تكنولوجيا المعلومات", color: "#3b82f6", icon: "cpu", sort_order: 3 },
    ]
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
      calendar: "Semester Cycle",
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
      mother_name: false,
      previous_school: false,
    },
    requiredFields: {
      national_id: false,
      email: true,
      phone: true,
      birth_date: false,
      sex: false,
      address: false,
      mother_name: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: false,
      email: true,
      phone: true,
    },
    requiredDocuments: [
      "صورة الهوية الوطنية (ID Copy)"
    ],
    stages: [
      { level: "short_courses", name: "Short Courses", name_ar: "دورات قصيرة", color: "#a855f7", divisions: ["C1"] },
      { level: "professional_certs", name: "Professional Certifications", name_ar: "شهادات مهنية", color: "#ec4899", divisions: ["C1"] },
      { level: "bootcamps", name: "Executive Bootcamps", name_ar: "معسكرات تدريبية", color: "#f43f5e", divisions: ["C1"] },
    ],
    timeSlots: [
      { name: "Morning Slot", name_ar: "الفترة الصباحية", start_time: "09:00", end_time: "13:00" },
      { name: "Afternoon Slot", name_ar: "الفترة المسائية", start_time: "14:00", end_time: "18:00" },
      { name: "Weekend Bootcamp", name_ar: "معسكر نهاية الأسبوع", start_time: "09:00", end_time: "17:00" },
    ],
    subjects: [
      { code: "PMP", name: "Project Management (PMP)", name_ar: "إدارة المشاريع الاحترافية", color: "#f59e0b", icon: "briefcase", sort_order: 1 },
      { code: "DS-BOOTCAMP", name: "Data Science Bootcamp", name_ar: "معسكر علوم البيانات", color: "#3b82f6", icon: "bar-chart-2", sort_order: 2 },
      { code: "DIGITAL-MKT", name: "Digital Marketing", name_ar: "التسويق الرقمي", color: "#10b981", icon: "trending-up", sort_order: 3 },
    ]
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
      calendar: "Academic Cycle",
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
      mother_name: true,
      previous_school: true,
    },
    requiredFields: {
      national_id: true,
      email: false,
      phone: true,
      birth_date: true,
      sex: true,
      address: false,
      mother_name: false,
      previous_school: false,
    },
    preventDuplicates: {
      national_id: true,
      email: false,
      phone: true,
    },
    requiredDocuments: [
      "صورة الهوية الوطنية (ID Copy)"
    ],
    stages: [
      { level: "level1", name: "Level 1", name_ar: "المستوى الأول", color: "#3b82f6", divisions: ["1"] },
    ],
    timeSlots: [
      { name: "Slot 1", name_ar: "الفترة الأولى", start_time: "08:00", end_time: "09:00" },
    ],
    subjects: [
      { code: "SUBJ1", name: "Subject 1", name_ar: "المادة الأولى", color: "#6366f1", icon: "book", sort_order: 1 },
    ]
  }
};
