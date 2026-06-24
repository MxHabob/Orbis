<!--
  Interactive school onboarding page.
  Designed following the 2026 design language of Orbis, featuring premium glassmorphism, responsive sidebar layout, and bilingual (Arabic/English) interface.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { toast } from "svelte-sonner";
import { Button } from "@orbis/ui-svelte/button";
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@orbis/ui-svelte/card";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import { Badge } from "@orbis/ui-svelte/badge";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

import { 
  School, 
  Calendar, 
  Building, 
  GraduationCap, 
  Clock, 
  BookOpen, 
  ArrowRight, 
  ArrowLeft, 
  Check, 
  Plus, 
  Trash, 
  Loader2, 
  Sparkles, 
  Globe, 
  Coins, 
  PlusCircle, 
  X,
  Layers
} from "lucide-svelte";

// --- State variables using Svelte 5 Runes ---
let activeStep = $state(1);
let isLoading = $state(false);
let errorMessage = $state<string | null>(null);

// Get current locale for layout direction (RTL or LTR)
const currentLocale = $derived(getLocale());
const isRtl = $derived(currentLocale === "ar");

// Step 1: School General Info
let orgName = $state("Sanaa Model School");
let orgNameAr = $state("مدرسة صنعاء النموذجية");
let rootUnitName = $state("Sanaa Model School");
let rootUnitNameAr = $state("مدرسة صنعاء النموذجية");
let timezone = $state("Asia/Aden");
let locale = $state("ar");
let currency = $state("YER");

// Step 2: Academic Calendar (Academic Year + Semesters)
let academicYearName = $state("Academic Year 1447 Hijri");
let academicYearNameAr = $state("العام الدراسي 1447 هـ");
let startDate = $state("2026-06-21");
let endDate = $state("2027-02-21");

let semester1Name = $state("Semester 1");
let semester1NameAr = $state("الفصل الدراسي الأول");
let semester1Start = $state("2026-06-21");
let semester1End = $state("2026-10-21");

let semester2Name = $state("Semester 2");
let semester2NameAr = $state("الفصل الدراسي الثاني");
let semester2Start = $state("2026-10-22");
let semester2End = $state("2027-02-21");

// Step 3: Buildings & Rooms Setup
interface Room {
  id: string;
  name: string;
  name_ar: string;
  capacity: number;
  room_type: string;
  building: string;
  floor: number;
}
let rooms = $state<Room[]>([]);
let activeBuildingTab = $state("المبنى 1");
let newRoomName = $state("");
let newRoomNameAr = $state("");
let newRoomCapacity = $state(30);
let newRoomType = $state("classroom");
let newRoomFloor = $state(1);

const buildings = ["المبنى 1", "المبنى 2"];

function resetRoomsToDefault() {
  const list: Room[] = [];
  // Building 1 Classrooms
  for (let i = 1; i <= 10; i++) {
    list.push({
      id: `room-b1-f1-${i}`,
      name: `Classroom 1-0${i}`,
      name_ar: `فصل 1-0${i}`,
      capacity: 30,
      room_type: "classroom",
      building: "المبنى 1",
      floor: 1
    });
  }
  for (let i = 11; i <= 20; i++) {
    list.push({
      id: `room-b1-f2-${i}`,
      name: `Classroom 1-${i}`,
      name_ar: `فصل 1-${i}`,
      capacity: 30,
      room_type: "classroom",
      building: "المبنى 1",
      floor: 2
    });
  }
  // Building 1 Computer labs
  for (let i = 1; i <= 3; i++) {
    list.push({
      id: `room-b1-lab-comp-${i}`,
      name: `Computer Lab 1-${i}`,
      name_ar: `مختبر الحاسوب 1-${i}`,
      capacity: 25,
      room_type: "computer_lab",
      building: "المبنى 1",
      floor: 2
    });
  }
  // Building 1 Chemistry labs
  for (let i = 1; i <= 2; i++) {
    list.push({
      id: `room-b1-lab-chem-${i}`,
      name: `Chemistry Lab 1-${i}`,
      name_ar: `مختبر الكيمياء 1-${i}`,
      capacity: 25,
      room_type: "chemistry_lab",
      building: "المبنى 1",
      floor: 1
    });
  }
  // Building 2
  for (let floor = 1; floor <= 3; floor++) {
    for (let room = 1; room <= 7; room++) {
      const roomNum = `${floor}-0${room}`;
      list.push({
        id: `room-b2-f${floor}-${room}`,
        name: `Classroom 2-${roomNum}`,
        name_ar: `فصل 2-${roomNum}`,
        capacity: 30,
        room_type: "classroom",
        building: "المبنى 2",
        floor
      });
    }
  }
  rooms = list;
}

function addRoom() {
  if (!newRoomName || !newRoomNameAr) {
    toast.error(isRtl ? "يرجى ملء جميع الحقول المطلوبة للغرفة" : "Please fill in all required fields for the room");
    return;
  }
  const id = `room-${Date.now()}`;
  rooms.push({
    id,
    name: newRoomName,
    name_ar: newRoomNameAr,
    capacity: newRoomCapacity,
    room_type: newRoomType,
    building: activeBuildingTab,
    floor: newRoomFloor
  });
  rooms = [...rooms]; // Trigger Svelte reactivity
  newRoomName = "";
  newRoomNameAr = "";
  toast.success(isRtl ? "تمت إضافة الغرفة بنجاح" : "Room added successfully");
}

function removeRoom(id: string) {
  rooms = rooms.filter(r => r.id !== id);
  toast.success(isRtl ? "تم حذف الغرفة" : "Room deleted successfully");
}

// Step 4: Grade Levels & Classes
interface GradeLevel {
  level: string;
  name: string;
  name_ar: string;
  color: string;
  enabled: boolean;
  divisions: string[];
}
let gradeLevels = $state<GradeLevel[]>([
  { level: "kg1", name: "KG1", name_ar: "روضة أولى", color: "#f43f5e", enabled: true, divisions: ["أ"] },
  { level: "kg2", name: "KG2", name_ar: "روضة ثانية", color: "#ec4899", enabled: true, divisions: ["أ"] },
  { level: "grade1", name: "Grade 1", name_ar: "الصف الأول", color: "#d946ef", enabled: true, divisions: ["أ", "ب"] },
  { level: "grade2", name: "Grade 2", name_ar: "الصف الثاني", color: "#a855f7", enabled: true, divisions: ["أ", "ب"] },
  { level: "grade3", name: "Grade 3", name_ar: "الصف الثالث", color: "#6366f1", enabled: true, divisions: ["أ", "ب"] },
  { level: "grade4", name: "Grade 4", name_ar: "الصف الرابع", color: "#3b82f6", enabled: true, divisions: ["أ"] },
  { level: "grade5", name: "Grade 5", name_ar: "الصف الخامس", color: "#0ea5e9", enabled: true, divisions: ["أ"] },
  { level: "grade6", name: "Grade 6", name_ar: "الصف السادس", color: "#06b6d4", enabled: true, divisions: ["أ"] },
  { level: "grade7", name: "Grade 7", name_ar: "الصف السابع", color: "#14b8a6", enabled: true, divisions: ["أ"] },
  { level: "grade8", name: "Grade 8", name_ar: "الصف الثامن", color: "#10b981", enabled: true, divisions: ["أ"] },
  { level: "grade9", name: "Grade 9", name_ar: "الصف التاسع", color: "#22c55e", enabled: true, divisions: ["أ"] },
]);

let divisionInput = $state<Record<string, string>>({});

function addDivision(level: string) {
  const text = divisionInput[level]?.trim();
  if (!text) return;
  const gl = gradeLevels.find(g => g.level === level);
  if (gl && !gl.divisions.includes(text)) {
    gl.divisions.push(text);
    gradeLevels = [...gradeLevels];
    divisionInput[level] = "";
    toast.success(isRtl ? "تمت إضافة الشعبة" : "Division added successfully");
  }
}

function removeDivision(level: string, div: string) {
  const gl = gradeLevels.find(g => g.level === level);
  if (gl) {
    gl.divisions = gl.divisions.filter(d => d !== div);
    gradeLevels = [...gradeLevels];
    toast.success(isRtl ? "تم حذف الشعبة" : "Division deleted successfully");
  }
}

// Step 5: Time Slots (Daily Periods)
interface TimeSlot {
  name: string;
  name_ar: string;
  start_time: string;
  end_time: string;
}
let timeSlots = $state<TimeSlot[]>([
  { name: "1st Period", name_ar: "الحصة الأولى", start_time: "08:00", end_time: "08:45" },
  { name: "2nd Period", name_ar: "الحصة الثانية", start_time: "08:45", end_time: "09:30" },
  { name: "3rd Period", name_ar: "الحصة الثالثة", start_time: "09:45", end_time: "10:30" },
  { name: "4th Period", name_ar: "الحصة الرابعة", start_time: "10:30", end_time: "11:15" },
  { name: "5th Period", name_ar: "الحصة الخامسة", start_time: "11:30", end_time: "12:15" },
]);

let newSlotName = $state("");
let newSlotNameAr = $state("");
let newSlotStart = $state("12:15");
let newSlotEnd = $state("13:00");

function addTimeSlot() {
  if (!newSlotName || !newSlotNameAr) {
    toast.error(isRtl ? "يرجى إدخال اسم الحصة" : "Please enter the period name");
    return;
  }
  timeSlots.push({
    name: newSlotName,
    name_ar: newSlotNameAr,
    start_time: newSlotStart,
    end_time: newSlotEnd
  });
  timeSlots = [...timeSlots];
  newSlotName = "";
  newSlotNameAr = "";
  toast.success(isRtl ? "تمت إضافة الحصة" : "Period added successfully");
}

function removeTimeSlot(idx: number) {
  timeSlots = timeSlots.filter((_, i) => i !== idx);
  toast.success(isRtl ? "تم حذف الحصة" : "Period deleted successfully");
}

// Step 6: Subjects Setup
interface Subject {
  code: string;
  name: string;
  name_ar: string;
  color: string;
  icon: string;
  sort_order: number;
  enabled: boolean;
}
let subjects = $state<Subject[]>([
  { code: "QUR", name: "Holy Quran", name_ar: "القرآن الكريم", color: "#10b981", icon: "book-open", sort_order: 1, enabled: true },
  { code: "ISL", name: "Islamic Studies", name_ar: "الدراسات الإسلامية", color: "#0d9488", icon: "bookmark", sort_order: 2, enabled: true },
  { code: "ARB", name: "Arabic Language", name_ar: "اللغة العربية", color: "#f59e0b", icon: "pen-tool", sort_order: 3, enabled: true },
  { code: "MTH", name: "Mathematics", name_ar: "الرياضيات", color: "#3b82f6", icon: "binary", sort_order: 4, enabled: true },
  { code: "SCI", name: "Science", name_ar: "العلوم", color: "#8b5cf6", icon: "flask-conical", sort_order: 5, enabled: true },
  { code: "ENG", name: "English Language", name_ar: "اللغة الإنجليزية", color: "#6366f1", icon: "languages", sort_order: 6, enabled: true },
  { code: "SOC", name: "Social Studies", name_ar: "الدراسات الاجتماعية", color: "#f97316", icon: "globe", sort_order: 7, enabled: true },
]);

let newSubCode = $state("");
let newSubName = $state("");
let newSubNameAr = $state("");
let newSubColor = $state("#f59e0b");

function addSubject() {
  if (!newSubCode || !newSubName || !newSubNameAr) {
    toast.error(isRtl ? "يرجى تعبئة كافة بيانات المادة الجديدة" : "Please fill in all the details for the new subject");
    return;
  }
  subjects.push({
    code: newSubCode.toUpperCase(),
    name: newSubName,
    name_ar: newSubNameAr,
    color: newSubColor,
    icon: "book",
    sort_order: subjects.length + 1,
    enabled: true
  });
  subjects = [...subjects];
  newSubCode = "";
  newSubName = "";
  newSubNameAr = "";
  toast.success(isRtl ? "تمت إضافة المادة الجديدة" : "New subject added successfully");
}

// Helpers for date cycle calculations
function generateMonthsForSemester(parentCycleId: string, namePrefix: string, namePrefixAr: string, startStr: string, endStr: string) {
  const start = new Date(startStr);
  const end = new Date(endStr);
  const totalDays = Math.ceil((end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24));
  const daysPerMonth = Math.floor(totalDays / 4);

  const months = [];
  const monthNamesAr = ["الأول", "الثاني", "الثالث", "الرابع (النهائي)"];
  const monthNamesEn = ["Month 1", "Month 2", "Month 3", "Month 4 (Final)"];

  let currentStart = new Date(start);
  for (let i = 0; i < 4; i++) {
    const mStart = new Date(currentStart);
    const mEnd = new Date(currentStart);
    if (i === 3) {
      mEnd.setTime(end.getTime());
    } else {
      mEnd.setDate(mEnd.getDate() + daysPerMonth - 1);
    }

    const start_date = mStart.toISOString().split("T")[0];
    const end_date = mEnd.toISOString().split("T")[0];

    months.push({
      id: `${parentCycleId}-m${i + 1}`,
      parent_id: parentCycleId,
      name: `${namePrefix} - ${monthNamesEn[i]}`,
      name_ar: `${namePrefixAr} - الشهر ${monthNamesAr[i]}`,
      start_date,
      end_date,
      cycle_type: "month",
      weight: i === 3 ? 30 : 20
    });

    currentStart = new Date(mEnd);
    currentStart.setDate(currentStart.getDate() + 1);
  }
  return months;
}

// Derived summary metrics
const totalCapacity = $derived(rooms.reduce((acc, curr) => acc + curr.capacity, 0));
const enabledStages = $derived(gradeLevels.filter(l => l.enabled).length);
const totalDivisions = $derived(gradeLevels.filter(l => l.enabled).reduce((acc, l) => acc + l.divisions.length, 0));
const enabledSubjects = $derived(subjects.filter(s => s.enabled).length);
const totalSlots = $derived(timeSlots.length);
const bld1Rooms = $derived(rooms.filter(r => r.building === 'المبنى 1').length);
const bld2Rooms = $derived(rooms.filter(r => r.building === 'المبنى 2').length);

// Save Onboarding configuration
async function handleCompleteSetup() {
  isLoading = true;
  errorMessage = null;

  try {
    const orgId = authStore.orgId;
    if (!orgId) {
      throw new Error(isRtl ? "لم يتم العثور على معرف المنظمة الفعال، يرجى تسجيل الدخول مجدداً" : "Active organization ID not found, please sign in again");
    }

    // 1. Prepare Cycles structure
    const cycleYearId = "cycle-1447";
    const cyclesList = [
      {
        id: cycleYearId,
        parent_id: null,
        name: academicYearName,
        name_ar: academicYearNameAr,
        start_date: startDate,
        end_date: endDate,
        cycle_type: "year",
        weight: null
      },
      {
        id: `${cycleYearId}-s1`,
        parent_id: cycleYearId,
        name: semester1Name,
        name_ar: semester1NameAr,
        start_date: semester1Start,
        end_date: semester1End,
        cycle_type: "term",
        weight: 50
      },
      {
        id: `${cycleYearId}-s2`,
        parent_id: cycleYearId,
        name: semester2Name,
        name_ar: semester2NameAr,
        start_date: semester2Start,
        end_date: semester2End,
        cycle_type: "term",
        weight: 50
      },
      ...generateMonthsForSemester(`${cycleYearId}-s1`, semester1Name, semester1NameAr, semester1Start, semester1End),
      ...generateMonthsForSemester(`${cycleYearId}-s2`, semester2Name, semester2NameAr, semester2Start, semester2End)
    ];

    // 2. Prepare classes structure
    const classesList = [];
    for (const gl of gradeLevels) {
      if (!gl.enabled) continue;
      for (const div of gl.divisions) {
        classesList.push({
          id: `class-${gl.level}-${div}`,
          name: `${gl.name} - ${div}`,
          name_ar: `${gl.name_ar} - شعبة ${div}`,
          level: gl.level,
          division: div,
          color: gl.color
        });
      }
    }

    // 3. Prepare subjects structure
    const activeSubjects = subjects
      .filter(s => s.enabled)
      .map((s, idx) => ({
        id: `subject-${s.code.toLowerCase()}`,
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        color: s.color,
        icon: s.icon,
        sort_order: idx + 1
      }));

    // 4. Root Unit ID
    const rootUnitId = "unit-sanaa-model";

    // 5. Config Payload
    const payload = {
      org_name: orgName,
      org_name_ar: orgNameAr,
      timezone,
      locale,
      currency,
      root_unit_id: rootUnitId,
      root_unit_name: rootUnitName,
      root_unit_name_ar: rootUnitNameAr,
      cycles: cyclesList,
      time_slots: timeSlots,
      rooms: rooms,
      subjects: activeSubjects,
      classes: classesList
    };

    console.log("Submitting Onboarding Setup Payload:", payload);
    
    // Call Tauri backend transactional seed API
    await commands.academics.initializeSchoolSetup(orgId, payload);

    authStore.markSetupComplete();

    toast.success(m.onboarding_toast_success ? m.onboarding_toast_success() : "تم تهيئة وتأسيس حساب المدرسة والمحاسبة المالية بنجاح!");
    
    // Redirect to home dashboard
    goto(localizeHref(`/${orgId}`));
  } catch (err: any) {
    console.error("Setup initialization failed:", err);
    errorMessage = err?.message || String(err) || (isRtl ? "حدث خطأ غير متوقع أثناء إعداد المدرسة" : "An unexpected error occurred during setup");
    toast.error(m.onboarding_toast_error ? m.onboarding_toast_error() : "فشل تأسيس المدرسة، يرجى مراجعة التفاصيل أدناه");
  } finally {
    isLoading = false;
  }
}

onMount(async () => {
  authStore.restore();
  if (!authStore.isAuthenticated) {
    goto(localizeHref("/login"), { replaceState: true });
    return;
  }
  await authStore.checkSetupStatus();
  if (authStore.isSetupComplete) {
    goto(localizeHref(`/${authStore.orgId}`), { replaceState: true });
    return;
  }
  resetRoomsToDefault();
});

// Sidebar Steps list
const steps = [
  { id: 1, label: m.onboarding_step_1_label ? m.onboarding_step_1_label() : "معلومات المدرسة", labelEn: "School Info", icon: School },
  { id: 2, label: m.onboarding_step_2_label ? m.onboarding_step_2_label() : "التقويم الدراسي", labelEn: "Academic Year", icon: Calendar },
  { id: 3, label: m.onboarding_step_3_label ? m.onboarding_step_3_label() : "المباني والقاعات", labelEn: "Buildings & Rooms", icon: Building },
  { id: 4, label: m.onboarding_step_4_label ? m.onboarding_step_4_label() : "الصفوف والشعب", labelEn: "Stages & Classes", icon: GraduationCap },
  { id: 5, label: m.onboarding_step_5_label ? m.onboarding_step_5_label() : "الحصص اليومية", labelEn: "Daily Schedule", icon: Clock },
  { id: 6, label: m.onboarding_step_6_label ? m.onboarding_step_6_label() : "المواد الدراسية", labelEn: "Subjects List", icon: BookOpen },
  { id: 7, label: m.onboarding_step_7_label ? m.onboarding_step_7_label() : "الملخص وتدشين النظام", labelEn: "Review & Launch", icon: Sparkles },
];
</script>

<svelte:head>
  <title>{m.onboarding_subtitle ? m.onboarding_subtitle() : "تهيئة نظام أوربيس المدرسي الذكي"} | Setup Wizard</title>
</svelte:head>

<div class="h-screen w-screen bg-[#07090e] text-slate-100 flex flex-col md:flex-row overflow-hidden font-sans antialiased" class:dir-rtl={isRtl} style="direction: {isRtl ? 'rtl' : 'ltr'};">
  <!-- Left/Right Side Wizard Steps Sidebar -->
  <aside class="w-full md:w-80 bg-slate-950/80 border-slate-800 flex flex-col justify-between shrink-0 p-6 z-10 backdrop-blur-xl relative" class:border-l={isRtl} class:border-r={!isRtl}>
    <div class="absolute inset-0 bg-gradient-to-b from-violet-500/5 to-cyan-500/5 pointer-events-none"></div>
    <div class="space-y-8 relative">
      <!-- Title & Branding -->
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-violet-600 to-cyan-500 flex items-center justify-center shadow-lg shadow-violet-500/20 text-white">
          <School class="w-5 h-5" />
        </div>
        <div class="text-start">
          <h2 class="font-bold text-lg leading-tight tracking-wide bg-gradient-to-r from-slate-100 to-slate-300 bg-clip-text text-transparent">
            {m.onboarding_title ? m.onboarding_title() : "تأسيس نظام المدرسة"}
          </h2>
          <span class="text-xs text-slate-400 font-mono tracking-widest uppercase">
            {m.onboarding_subtitle ? m.onboarding_subtitle() : "Orbis OS v2026"}
          </h2>
        </div>
      </div>

      <!-- Steps Progress -->
      <nav class="space-y-1">
        {#each steps as step}
          <button 
            onclick={() => !isLoading && (activeStep = step.id)}
            disabled={isLoading}
            class="w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all duration-300 group border text-start
              {activeStep === step.id 
                ? 'bg-violet-600/10 border-violet-500/30 text-violet-400 shadow-md shadow-violet-500/5' 
                : 'border-transparent text-slate-400 hover:bg-slate-900/50 hover:text-slate-200'}"
          >
            <!-- Step Circle Icon -->
            <div class="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-semibold transition-all duration-300 shrink-0
              {activeStep === step.id 
                ? 'bg-violet-500 text-white scale-105' 
                : activeStep > step.id 
                  ? 'bg-emerald-500/20 text-emerald-400'
                  : 'bg-slate-900 border border-slate-800 text-slate-400 group-hover:border-slate-700'}"
            >
              {#if activeStep > step.id}
                <Check class="w-4 h-4" />
              {:else}
                <step.icon class="w-4 h-4" />
              {/if}
            </div>

            <!-- Labels -->
            <div class="flex-1 min-w-0 text-start">
              <p class="text-sm font-medium transition-colors duration-200 {activeStep === step.id ? 'text-slate-100' : 'text-slate-400'}">{step.label}</p>
              <p class="text-xs text-slate-500 font-mono tracking-wider mt-0.5">{step.labelEn}</p>
            </div>
          </button>
        {/each}
      </nav>
    </div>

    <!-- Bottom Footer Brand info -->
    <div class="text-center pt-6 border-t border-slate-900 relative">
      <p class="text-xs text-slate-500 font-mono">Connected to Orbis Core v2.0</p>
      <p class="text-[10px] text-slate-600 mt-1">Sanaa Model School setup engine</p>
    </div>
  </aside>

  <!-- Content Panel -->
  <main class="flex-1 flex flex-col min-w-0 h-full relative bg-slate-950/40">
    <!-- Top Nav bar inside Panel -->
    <header class="h-16 border-b border-slate-800/80 px-8 flex items-center justify-between shrink-0 bg-slate-950/30 backdrop-blur-md">
      <div class="text-start">
        <h1 class="text-slate-100 font-medium text-lg flex items-center gap-2">
          <span>{steps[activeStep - 1].label}</span>
          <span class="text-slate-500 text-sm font-mono font-normal">/ {steps[activeStep - 1].labelEn}</span>
        </h1>
      </div>
      <div>
        <Badge variant="outline" class="border-slate-800 bg-slate-900/50 text-slate-400 text-[10px] font-mono px-3.5 py-1">
          {isRtl ? `${activeStep} من ${steps.length} خطوات` : `Step ${activeStep} of ${steps.length}`}
        </Badge>
      </div>
    </header>

    <!-- Wizard Steps Form Scroll Content -->
    <div class="flex-1 overflow-y-auto px-8 py-8 space-y-6 text-start">
      {#if errorMessage}
        <div class="p-4 rounded-xl border border-rose-500/20 bg-rose-500/5 text-rose-400 text-sm flex gap-3 items-start max-w-3xl">
          <div class="w-5 h-5 rounded-full bg-rose-500/10 flex items-center justify-center shrink-0 mt-0.5 text-rose-500">!</div>
          <div class="space-y-1">
            <h4 class="font-semibold text-rose-300">{isRtl ? "حدث خطأ أثناء الإعداد" : "Error during setup"}</h4>
            <p>{errorMessage}</p>
          </div>
        </div>
      {/if}

      <!-- Step 1 Content: General Info -->
      {#if activeStep === 1}
        <div class="max-w-3xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_school_info_title ? m.onboarding_school_info_title() : "معلومات المدرسة والمنشأة"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_school_info_desc ? m.onboarding_school_info_desc() : "حدد الهوية واللغة والعملة الأساسية للنظام المحاسبي والتعليمي."}</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <Label for="orgNameAr">{m.onboarding_school_name_ar ? m.onboarding_school_name_ar() : "اسم المدرسة الرسمي (بالعربية)"}</Label>
              <Input id="orgNameAr" bind:value={orgNameAr} placeholder="مثال: مدرسة صنعاء النموذجية" class="bg-slate-900/50 border-slate-800 focus:border-violet-500" />
            </div>

            <div class="space-y-2">
              <Label for="orgName">{m.onboarding_school_name_en ? m.onboarding_school_name_en() : "اسم المدرسة بالإنجليزية (English Name)"}</Label>
              <Input id="orgName" bind:value={orgName} placeholder="Sanaa Model School" class="bg-slate-900/50 border-slate-800 focus:border-violet-500 text-left font-sans" style="direction: ltr;" />
            </div>

            <div class="space-y-2">
              <Label for="rootUnitNameAr">{m.onboarding_root_unit_ar ? m.onboarding_root_unit_ar() : "اسم الوحدة التعليمية الجذرية (عربي)"}</Label>
              <Input id="rootUnitNameAr" bind:value={rootUnitNameAr} placeholder="مدرسة صنعاء النموذجية" class="bg-slate-900/50 border-slate-800 focus:border-violet-500" />
            </div>

            <div class="space-y-2">
              <Label for="rootUnitName">{m.onboarding_root_unit_en ? m.onboarding_root_unit_en() : "الاسم الجذري بالإنجليزية (English Root Unit)"}</Label>
              <Input id="rootUnitName" bind:value={rootUnitName} placeholder="Sanaa Model School" class="bg-slate-900/50 border-slate-800 focus:border-violet-500 text-left font-sans" style="direction: ltr;" />
            </div>

            <div class="space-y-2">
              <Label for="currency">{m.onboarding_currency ? m.onboarding_currency() : "العملة الأساسية للنظام المالي"}</Label>
              <div class="relative">
                <select id="currency" bind:value={currency} class="w-full h-10 px-3 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-violet-500">
                  <option value="YER">ريال يمني (YER)</option>
                  <option value="SAR">ريال سعودي (SAR)</option>
                  <option value="USD">دولار أمريكي (USD)</option>
                </select>
                <div class="absolute top-2.5 text-slate-500" class:left-3={!isRtl} class:right-3={isRtl}><Coins class="w-5 h-5" /></div>
              </div>
              <p class="text-[11px] text-slate-500">{m.onboarding_currency_hint ? m.onboarding_currency_hint() : "العملة التي ستعتمد في شجرة الحسابات وتقارير الميزانية والمستحقات."}</p>
            </div>

            <div class="space-y-2">
              <Label for="locale">{m.onboarding_locale ? m.onboarding_locale() : "لغة واجهة النظام الافتراضية"}</Label>
              <div class="relative">
                <select id="locale" bind:value={locale} class="w-full h-10 px-3 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-violet-500">
                  <option value="ar">العربية (Arabic)</option>
                  <option value="en">English (الإنجليزية)</option>
                </select>
                <div class="absolute top-2.5 text-slate-500" class:left-3={!isRtl} class:right-3={isRtl}><Globe class="w-5 h-5" /></div>
              </div>
            </div>

            <div class="space-y-2">
              <Label for="timezone">{m.onboarding_timezone ? m.onboarding_timezone() : "المنطقة الزمنية"}</Label>
              <select id="timezone" bind:value={timezone} class="w-full h-10 px-3 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-violet-500">
                <option value="Asia/Aden">توقيت صنعاء / عدن (GMT+3)</option>
                <option value="Asia/Riyadh">توقيت الرياض (GMT+3)</option>
                <option value="UTC">التوقيت العالمي الموحد (UTC)</option>
              </select>
            </div>
          </div>
        </div>
      {/if}

      <!-- Step 2 Content: Academic Calendar -->
      {#if activeStep === 2}
        <div class="max-w-3xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_calendar_title ? m.onboarding_calendar_title() : "التقويم والأعوام الدراسية"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_calendar_desc ? m.onboarding_calendar_desc() : "حدد فترات وتواريخ العام الدراسي والفصول وتوزيع الأشهر."}</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-slate-950/50 p-6 rounded-xl border border-slate-800/80">
            <div class="md:col-span-2">
              <h4 class="text-sm font-semibold text-violet-400 border-b border-slate-800 pb-2 mb-4">
                {m.onboarding_academic_year_section ? m.onboarding_academic_year_section() : "العام الدراسي الأساسي"}
              </h4>
            </div>

            <div class="space-y-2">
              <Label for="academicYearNameAr">{m.onboarding_academic_year_name_ar ? m.onboarding_academic_year_name_ar() : "اسم العام الدراسي (عربي)"}</Label>
              <Input id="academicYearNameAr" bind:value={academicYearNameAr} placeholder="العام الدراسي 1447 هـ" class="bg-slate-900/50 border-slate-800 focus:border-violet-500" />
            </div>

            <div class="space-y-2">
              <Label for="academicYearName">{m.onboarding_academic_year_name_en ? m.onboarding_academic_year_name_en() : "اسم العام الدراسي (English)"}</Label>
              <Input id="academicYearName" bind:value={academicYearName} placeholder="Academic Year 1447 Hijri" class="bg-slate-900/50 border-slate-800 focus:border-violet-500 text-left font-sans" style="direction: ltr;" />
            </div>

            <div class="space-y-2">
              <Label for="startDate">{m.onboarding_start_date ? m.onboarding_start_date() : "تاريخ بدء العام الدراسي"}</Label>
              <Input id="startDate" type="date" bind:value={startDate} class="bg-slate-900/50 border-slate-800 focus:border-violet-500" />
            </div>

            <div class="space-y-2">
              <Label for="endDate">{m.onboarding_end_date ? m.onboarding_end_date() : "تاريخ انتهاء العام الدراسي"}</Label>
              <Input id="endDate" type="date" bind:value={endDate} class="bg-slate-900/50 border-slate-800 focus:border-violet-500" />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-slate-950/50 p-6 rounded-xl border border-slate-800/80">
            <div class="md:col-span-2 flex items-center justify-between border-b border-slate-800 pb-2 mb-4">
              <h4 class="text-sm font-semibold text-violet-400">{m.onboarding_semester_split ? m.onboarding_semester_split() : "تقسيم الفصل الأول والفصل الثاني"}</h4>
              <span class="text-xs text-slate-500 font-mono">{m.onboarding_semester_weight_hint ? m.onboarding_semester_weight_hint() : "الوزن الافتراضي (50% لكل فصل)"}</span>
            </div>

            <div class="space-y-4">
              <div class="space-y-1">
                <h5 class="text-xs font-semibold text-slate-300">
                  {m.onboarding_semester_1 ? m.onboarding_semester_1() : "الفصل الدراسي الأول"}
                </h5>
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_name_ar ? m.onboarding_semester_name_ar() : "اسم الفصل (عربي)"}</Label>
                <Input bind:value={semester1NameAr} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_start ? m.onboarding_semester_start() : "تاريخ البدء"}</Label>
                <Input type="date" bind:value={semester1Start} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_end ? m.onboarding_semester_end() : "تاريخ الانتهاء"}</Label>
                <Input type="date" bind:value={semester1End} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
            </div>

            <div class="space-y-4 pr-6" class:border-r={isRtl} class:border-l={!isRtl} class:pl-6={!isRtl} class:border-slate-850={true}>
              <div class="space-y-1">
                <h5 class="text-xs font-semibold text-slate-300">
                  {m.onboarding_semester_2 ? m.onboarding_semester_2() : "الفصل الدراسي الثاني"}
                </h5>
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_name_ar ? m.onboarding_semester_name_ar() : "اسم الفصل (عربي)"}</Label>
                <Input bind:value={semester2NameAr} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_start ? m.onboarding_semester_start() : "تاريخ البدء"}</Label>
                <Input type="date" bind:value={semester2Start} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
              <div class="space-y-2">
                <Label class="text-xs">{m.onboarding_semester_end ? m.onboarding_semester_end() : "تاريخ الانتهاء"}</Label>
                <Input type="date" bind:value={semester2End} class="h-9 bg-slate-900 border-slate-800 focus:border-violet-500 text-xs" />
              </div>
            </div>

            <div class="md:col-span-2 pt-2 text-start">
              <p class="text-[11px] text-slate-500 leading-relaxed">
                {m.onboarding_calendar_months_hint ? m.onboarding_calendar_months_hint() : "💡 سيقوم المعالج تلقائياً بتوزيع وتقسيم أشهر التقييم (4 أشهر بكل فصل دراسي) مع تخصيص 20% لكل شهر تقييم عادي و 30% لامتحان منتصف/نهاية العام ليتوافق تماماً مع إعداد المدرسة الذكية."}
              </p>
            </div>
          </div>
        </div>
      {/if}

      <!-- Step 3 Content: Buildings & Rooms -->
      {#if activeStep === 3}
        <div class="max-w-5xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_buildings_title ? m.onboarding_buildings_title() : "توزيع الفصول والقاعات والمباني"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_buildings_desc ? m.onboarding_buildings_desc() : "حدد مباني المدرسة وأنشئ الفصول والمعامل والقاعات المتوفرة للتدريس."}</p>
          </div>

          <!-- Building Tabs -->
          <div class="flex gap-2 border-b border-slate-800 pb-px">
            {#each buildings as bld}
              <button 
                onclick={() => (activeBuildingTab = bld)}
                class="px-5 py-3 font-semibold text-sm border-b-2 transition-all duration-200 
                  {activeBuildingTab === bld 
                    ? 'border-violet-500 text-slate-200 bg-slate-900/20' 
                    : 'border-transparent text-slate-400 hover:text-slate-200'}"
              >
                {bld}
                <Badge variant="secondary" class="mr-2 ml-2 text-[10px] bg-slate-900 border-slate-800 text-slate-400">
                  {rooms.filter(r => r.building === bld).length} {m.onboarding_rooms_count_suffix ? m.onboarding_rooms_count_suffix() : "قاعات"}
                </Badge>
              </button>
            {/each}
          </div>

          <!-- Add Room Form for current Building -->
          <div class="p-6 bg-slate-900/30 rounded-xl border border-slate-800/80 grid grid-cols-1 md:grid-cols-5 gap-4 items-end">
            <div class="space-y-2 md:col-span-2">
              <Label class="text-xs">{m.onboarding_room_name_hint ? m.onboarding_room_name_hint() : "اسم القاعة (عربي / إنجليزي)"}</Label>
              <div class="grid grid-cols-2 gap-2">
                <Input bind:value={newRoomNameAr} placeholder="فصل 1-05" class="bg-slate-950 border-slate-800 text-xs h-9" />
                <Input bind:value={newRoomName} placeholder="Classroom 1-05" class="bg-slate-950 border-slate-800 text-xs h-9 text-left font-sans" style="direction: ltr;" />
              </div>
            </div>
            
            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_room_type ? m.onboarding_room_type() : "نوع القاعة"}</Label>
              <select bind:value={newRoomType} class="w-full h-9 px-3 bg-slate-955 border border-slate-800 rounded-lg text-xs text-slate-200">
                <option value="classroom">{m.onboarding_room_type_classroom ? m.onboarding_room_type_classroom() : "فصل دراسي (Classroom)"}</option>
                <option value="computer_lab">{m.onboarding_room_type_computer ? m.onboarding_room_type_computer() : "مختبر حاسوب (Computer Lab)"}</option>
                <option value="chemistry_lab">{m.onboarding_room_type_chemistry ? m.onboarding_room_type_chemistry() : "مختبر كيمياء (Chemistry Lab)"}</option>
                <option value="physics_lab">{m.onboarding_room_type_physics ? m.onboarding_room_type_physics() : "مختبر فيزياء (Physics Lab)"}</option>
                <option value="hall">{m.onboarding_room_type_hall ? m.onboarding_room_type_hall() : "قاعة أنشطة (Activity Hall)"}</option>
              </select>
            </div>

            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_room_floor_capacity ? m.onboarding_room_floor_capacity() : "الطابق / السعة الاستيعابية"}</Label>
              <div class="grid grid-cols-2 gap-2">
                <select bind:value={newRoomFloor} class="h-9 px-2 bg-slate-955 border border-slate-800 rounded-lg text-xs text-slate-200 font-sans">
                  <option value={1}>{m.onboarding_room_floor_n ? m.onboarding_room_floor_n({ n: 1 }) : "طابق 1"}</option>
                  <option value={2}>{m.onboarding_room_floor_n ? m.onboarding_room_floor_n({ n: 2 }) : "طابق 2"}</option>
                  <option value={3}>{m.onboarding_room_floor_n ? m.onboarding_room_floor_n({ n: 3 }) : "طابق 3"}</option>
                  <option value={4}>{m.onboarding_room_floor_n ? m.onboarding_room_floor_n({ n: 4 }) : "طابق 4"}</option>
                </select>
                <Input type="number" bind:value={newRoomCapacity} min="10" max="100" class="bg-slate-955 border-slate-800 text-xs h-9 font-sans" />
              </div>
            </div>

            <Button onclick={addRoom} class="w-full h-9 bg-violet-600 hover:bg-violet-700 text-xs flex gap-1.5 items-center justify-center">
              <Plus class="w-4 h-4" />
              <span>{m.onboarding_add_room_title ? m.onboarding_add_room_title() : "إضافة القاعة"}</span>
            </Button>
          </div>

          <!-- Rooms List for the Active Building -->
          <div class="space-y-3">
            <h4 class="text-xs font-semibold text-slate-400 uppercase tracking-wider">
              {m.onboarding_room_approved ? m.onboarding_room_approved({ building: activeBuildingTab }) : `قاعات ${activeBuildingTab} المعتمدة`}
            </h4>
            
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
              {#each rooms.filter(r => r.building === activeBuildingTab) as rm (rm.id)}
                <div class="bg-slate-900/20 border border-slate-800 rounded-xl p-4 flex justify-between items-start hover:border-slate-700 transition-all duration-200">
                  <div class="space-y-1">
                    <div class="flex items-center gap-2">
                      <span class="font-semibold text-slate-200 text-sm">{rm.name_ar}</span>
                      <span class="text-xs text-slate-500 font-mono">({rm.name})</span>
                    </div>
                    <div class="flex items-center gap-2 mt-2">
                      <Badge variant="outline" class="text-[10px] text-violet-400 border-violet-500/20 bg-violet-500/5">
                        {#if rm.room_type === 'classroom'}{m.onboarding_room_type_classroom ? m.onboarding_room_type_classroom() : "فصل دراسي"}
                        {:else if rm.room_type === 'computer_lab'}{m.onboarding_room_type_computer ? m.onboarding_room_type_computer() : "مختبر حاسوب"}
                        {:else if rm.room_type === 'chemistry_lab'}{m.onboarding_room_type_chemistry ? m.onboarding_room_type_chemistry() : "مختبر كيمياء"}
                        {:else if rm.room_type === 'physics_lab'}{m.onboarding_room_type_physics ? m.onboarding_room_type_physics() : "مختبر فيزياء"}
                        {:else}{m.onboarding_room_type_hall ? m.onboarding_room_type_hall() : "قاعة أنشطة"}
                        {/if}
                      </Badge>
                      <span class="text-xs text-slate-500">
                        {isRtl ? `الطابق: ${rm.floor}` : `Floor: ${rm.floor}`}
                      </span>
                      <span class="text-xs text-slate-500">
                        {rm.capacity} {m.onboarding_room_capacity_unit ? m.onboarding_room_capacity_unit() : "طالب"}
                      </span>
                    </div>
                  </div>

                  <button 
                    onclick={() => removeRoom(rm.id)} 
                    class="text-slate-500 hover:text-rose-400 p-1.5 hover:bg-rose-500/10 rounded-lg transition-colors"
                  >
                    <Trash class="w-4 h-4" />
                  </button>
                </div>
              {/each}
            </div>

            <div class="flex justify-end pt-2">
              <Button variant="ghost" onclick={resetRoomsToDefault} class="text-slate-500 hover:text-slate-300 text-xs">
                {m.onboarding_room_reset_defaults ? m.onboarding_room_reset_defaults() : "إعادة ضبط القاعات الافتراضية"}
              </Button>
            </div>
          </div>
        </div>
      {/if}

      <!-- Step 4 Content: Stages & Classes -->
      {#if activeStep === 4}
        <div class="max-w-4xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_stages_title ? m.onboarding_stages_title() : "المراحل والصفوف الدراسية (الشعب)"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_stages_desc ? m.onboarding_stages_desc() : "اختر الصفوف الدراسية المفعّلة في مدرستك وخصص الشعب (الأقسام) لكل صف."}</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each gradeLevels as gl, idx (gl.level)}
              <div 
                class="bg-slate-900/30 border rounded-xl p-5 space-y-4 transition-all duration-300
                  {gl.enabled ? 'border-slate-800 bg-slate-900/20' : 'border-slate-900/50 opacity-50 bg-slate-955/20'}"
              >
                <!-- Grade Header -->
                <div class="flex justify-between items-start">
                  <div class="flex gap-3 items-center">
                    <!-- Color Tag -->
                    <div class="w-4 h-4 rounded-full" style="background-color: {gl.color};"></div>
                    <div>
                      <h4 class="font-bold text-slate-200 text-sm">{gl.name_ar}</h4>
                      <p class="text-xs text-slate-500 font-mono">{gl.name}</p>
                    </div>
                  </div>

                  <div class="flex items-center gap-2">
                    <Label class="text-xs text-slate-400">{m.onboarding_stage_enable ? m.onboarding_stage_enable() : "تفعيل الصف"}</Label>
                    <input 
                      type="checkbox" 
                      bind:checked={gl.enabled} 
                      class="w-4 h-4 rounded border-slate-800 text-violet-600 focus:ring-violet-500 bg-slate-955" 
                    />
                  </div>
                </div>

                <!-- Divisions / Sections manager -->
                {#if gl.enabled}
                  <div class="space-y-3 pt-3 border-t border-slate-900">
                    <Label class="text-xs text-slate-400">{m.onboarding_stage_divisions ? m.onboarding_stage_divisions() : "شعب الفصل الدراسي المتاحة"}</Label>
                    
                    <div class="flex flex-wrap gap-1.5 min-h-[32px] p-2 bg-slate-955/40 rounded-lg border border-slate-900">
                      {#each gl.divisions as div}
                        <Badge variant="secondary" class="bg-slate-900 border-slate-800 text-slate-300 text-xs px-2.5 py-0.5 flex gap-1 items-center font-sans">
                          <span>
                            {m.onboarding_stage_division_unit ? m.onboarding_stage_division_unit({ name: div }) : `الشعبة ${div}`}
                          </span>
                          <button onclick={() => removeDivision(gl.level, div)} class="hover:text-rose-400 font-bold ml-0.5 mr-0.5 text-[10px]">&times;</button>
                        </Badge>
                      {/each}
                      
                      {#if gl.divisions.length === 0}
                        <span class="text-xs text-slate-600 self-center">
                          {m.onboarding_stage_no_divisions ? m.onboarding_stage_no_divisions() : "لا توجد شعب حالياً."}
                        </span>
                      {/if}
                    </div>

                    <!-- Quick Add Division Form -->
                    <div class="flex gap-2">
                      <Input 
                        placeholder={m.onboarding_stage_add_division_placeholder ? m.onboarding_stage_add_division_placeholder() : "إضافة شعبة (مثل: ج)"} 
                        bind:value={divisionInput[gl.level]} 
                        class="bg-slate-955 border-slate-800 text-xs h-8" 
                      />
                      <Button onclick={() => addDivision(gl.level)} size="sm" class="h-8 bg-violet-600 hover:bg-violet-700 text-xs shrink-0 px-3">
                        <Plus class="w-3.5 h-3.5" />
                      </Button>
                    </div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Step 5 Content: Daily Time Slots -->
      {#if activeStep === 5}
        <div class="max-w-3xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_schedule_title ? m.onboarding_schedule_title() : "الخطة الزمنية للحصص اليومية"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_schedule_desc ? m.onboarding_schedule_desc() : "حدد مواعيد الحصص اليومية وفترات الراحة (الفسحة) لتوزيع الجدول تلقائياً."}</p>
          </div>

          <!-- Add Period Form -->
          <div class="p-6 bg-slate-900/30 rounded-xl border border-slate-800/80 grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_schedule_name_ar ? m.onboarding_schedule_name_ar() : "اسم الحصة (عربي)"}</Label>
              <Input bind:value={newSlotNameAr} placeholder="الحصة السادسة" class="bg-slate-955 border-slate-800 text-xs h-9" />
            </div>

            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_schedule_name_en ? m.onboarding_schedule_name_en() : "اسم الحصة (English)"}</Label>
              <Input bind:value={newSlotName} placeholder="6th Period" class="bg-slate-955 border-slate-800 text-xs h-9 text-left font-sans" style="direction: ltr;" />
            </div>

            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_schedule_start_end ? m.onboarding_schedule_start_end() : "البدء / النهاية"}</Label>
              <div class="grid grid-cols-2 gap-2">
                <Input type="time" bind:value={newSlotStart} class="bg-slate-955 border-slate-800 text-xs h-9 font-sans" />
                <Input type="time" bind:value={newSlotEnd} class="bg-slate-955 border-slate-800 text-xs h-9 font-sans" />
              </div>
            </div>

            <Button onclick={addTimeSlot} class="w-full h-9 bg-violet-600 hover:bg-violet-700 text-xs flex gap-1.5 items-center justify-center">
              <Plus class="w-4 h-4" />
              <span>{m.onboarding_schedule_add_period ? m.onboarding_schedule_add_period() : "إضافة الفترة"}</span>
            </Button>
          </div>

          <!-- Time Slots table -->
          <div class="bg-slate-900/20 border border-slate-800 rounded-xl overflow-hidden">
            <table class="w-full border-collapse">
              <thead>
                <tr class="bg-slate-900/60 text-slate-400 border-b border-slate-800 text-xs font-semibold">
                  <th class="p-4 text-start w-12">{m.onboarding_schedule_col_num ? m.onboarding_schedule_col_num() : "م"}</th>
                  <th class="p-4 text-start">{m.onboarding_schedule_col_name ? m.onboarding_schedule_col_name() : "اسم الحصة (عربي / إنجليزي)"}</th>
                  <th class="p-4 text-center">{m.onboarding_schedule_col_start ? m.onboarding_schedule_col_start() : "وقت البدء"}</th>
                  <th class="p-4 text-center">{m.onboarding_schedule_col_end ? m.onboarding_schedule_col_end() : "وقت الانتهاء"}</th>
                  <th class="p-4 text-center">{m.onboarding_schedule_col_duration ? m.onboarding_schedule_col_duration() : "مدة الحصة"}</th>
                  <th class="p-4 text-center">{m.onboarding_schedule_col_actions ? m.onboarding_schedule_col_actions() : "إجراءات"}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-800/80 text-sm">
                {#each timeSlots as slot, idx}
                  <tr class="hover:bg-slate-900/10">
                    <td class="p-4 text-slate-400 font-mono text-xs">{idx + 1}</td>
                    <td class="p-4 text-start">
                      <div class="font-medium text-slate-200">{slot.name_ar}</div>
                      <div class="text-xs text-slate-500 font-mono">{slot.name}</div>
                    </td>
                    <td class="p-4 text-center font-mono text-xs text-slate-300">{slot.start_time}</td>
                    <td class="p-4 text-center font-mono text-xs text-slate-300">{slot.end_time}</td>
                    <td class="p-4 text-center">
                      {#const startParts = slot.start_time.split(':').map(Number)}
                      {#const endParts = slot.end_time.split(':').map(Number)}
                      {#const dur = (endParts[0]*60 + endParts[1]) - (startParts[0]*60 + startParts[1])}
                      <span class="text-xs bg-slate-900 text-slate-400 border border-slate-800 px-2 py-0.5 rounded font-sans">
                        {m.onboarding_schedule_duration_value ? m.onboarding_schedule_duration_value({ minutes: dur }) : `${dur} دقيقة`}
                      </span>
                    </td>
                    <td class="p-4 text-center">
                      <button 
                        onclick={() => removeTimeSlot(idx)} 
                        class="text-slate-500 hover:text-rose-400 p-1.5 hover:bg-rose-500/10 rounded-lg transition-colors"
                      >
                        <Trash class="w-4 h-4" />
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}

      <!-- Step 6 Content: Subjects -->
      {#if activeStep === 6}
        <div class="max-w-4xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_subjects_title ? m.onboarding_subjects_title() : "المواد الدراسية والمناهج"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_subjects_desc ? m.onboarding_subjects_desc() : "اختر وتكامل مع المواد الأساسية المعتمدة في مدرستك وخصص ألوانها أو أضف مواد إضافية."}</p>
          </div>

          <!-- Add Subject form -->
          <div class="p-6 bg-slate-900/30 rounded-xl border border-slate-800/80 grid grid-cols-1 md:grid-cols-5 gap-4 items-end">
            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_subject_code ? m.onboarding_subject_code() : "رمز المادة (رمز فريد)"}</Label>
              <Input bind:value={newSubCode} placeholder="PHY" class="bg-slate-955 border-slate-800 text-xs h-9 uppercase font-sans" />
            </div>

            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_subject_name_ar ? m.onboarding_subject_name_ar() : "اسم المادة (عربي)"}</Label>
              <Input bind:value={newSubNameAr} placeholder="الفيزياء" class="bg-slate-955 border-slate-800 text-xs h-9" />
            </div>

            <div class="space-y-2 font-sans text-start">
              <Label class="text-xs">{m.onboarding_subject_name_en ? m.onboarding_subject_name_en() : "اسم المادة (English)"}</Label>
              <Input bind:value={newSubName} placeholder="Physics" class="bg-slate-955 border-slate-800 text-xs h-9 text-left" style="direction: ltr;" />
            </div>

            <div class="space-y-2">
              <Label class="text-xs">{m.onboarding_subject_color ? m.onboarding_subject_color() : "لون المادة التعريفي"}</Label>
              <div class="flex gap-2 items-center">
                <input type="color" bind:value={newSubColor} class="w-9 h-9 border border-slate-800 rounded bg-slate-955 cursor-pointer" />
                <span class="text-xs font-mono text-slate-400">{newSubColor}</span>
              </div>
            </div>

            <Button onclick={addSubject} class="w-full h-9 bg-violet-600 hover:bg-violet-700 text-xs flex gap-1.5 items-center justify-center">
              <Plus class="w-4 h-4" />
              <span>{m.onboarding_subjects_add_subject ? m.onboarding_subjects_add_subject() : "إضافة المادة"}</span>
            </Button>
          </div>

          <!-- Subjects List Grid -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each subjects as sub, idx}
              <div 
                class="bg-slate-900/30 border rounded-xl p-4 space-y-3 transition-all duration-300
                  {sub.enabled ? 'border-slate-800 bg-slate-900/20' : 'border-transparent opacity-50 bg-slate-955/20'}"
              >
                <div class="flex justify-between items-start">
                  <div class="flex gap-3 items-center">
                    <!-- Color indicator -->
                    <div class="w-8 h-8 rounded-lg flex items-center justify-center" style="background-color: {sub.color}20; border: 1px solid {sub.color}40;">
                      <BookOpen class="w-4 h-4" style="color: {sub.color};" />
                    </div>
                    <div class="text-start">
                      <div class="font-bold text-slate-200 text-sm flex gap-1.5 items-center">
                        <span>{sub.name_ar}</span>
                        <Badge variant="outline" class="text-[9px] px-1 font-mono uppercase bg-slate-900 border-slate-800 text-slate-500">
                          {sub.code}
                        </Badge>
                      </div>
                      <p class="text-xs text-slate-500 font-mono mt-0.5">{sub.name}</p>
                    </div>
                  </div>

                  <input 
                    type="checkbox" 
                    bind:checked={sub.enabled} 
                    class="w-4 h-4 rounded border-slate-800 text-violet-600 focus:ring-violet-500 bg-slate-955" 
                  />
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Step 7 Content: Review & Launch -->
      {#if activeStep === 7}
        <div class="max-w-4xl space-y-6">
          <div class="space-y-2">
            <h3 class="text-slate-200 font-bold text-xl">{m.onboarding_review_title ? m.onboarding_review_title() : "مراجعة وتدشين النظام"}</h3>
            <p class="text-slate-400 text-sm">{m.onboarding_review_desc ? m.onboarding_review_desc() : "راجع الهيكل المدرسي النهائي قبل البدء بعمليات الإعداد الشاملة وقيد الحسابات المالية."}</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- School info card -->
            <Card class="bg-slate-900/20 border-slate-800">
              <CardHeader class="pb-2 text-start">
                <CardTitle class="text-sm font-semibold text-slate-400 flex gap-2 items-center">
                  <School class="w-4 h-4 text-violet-400" />
                  <span>{m.onboarding_review_school_info ? m.onboarding_review_school_info() : "معلومات المدرسة الأساسية"}</span>
                </CardTitle>
              </CardHeader>
              <CardContent class="space-y-2 text-sm">
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_school_name ? m.onboarding_review_school_name() : "اسم المدرسة الرسمي:"}</span><span class="font-medium text-slate-200">{orgNameAr}</span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_org_id ? m.onboarding_review_org_id() : "المعرف البرمجي:"}</span><span class="font-mono text-xs text-slate-400">{authStore.orgId}</span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_currency ? m.onboarding_review_currency() : "العملة الأساسية للنظام:"}</span><span class="font-medium text-emerald-400">{currency}</span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_locale_timezone ? m.onboarding_review_locale_timezone() : "اللغة / التوقيت:"}</span><span class="font-medium text-slate-200">{locale === 'ar' ? 'العربية' : 'English'} / {timezone}</span></div>
              </CardContent>
            </Card>

            <!-- Calendar card -->
            <Card class="bg-slate-900/20 border-slate-800">
              <CardHeader class="pb-2 text-start">
                <CardTitle class="text-sm font-semibold text-slate-400 flex gap-2 items-center">
                  <Calendar class="w-4 h-4 text-violet-400" />
                  <span>{m.onboarding_review_calendar ? m.onboarding_review_calendar() : "التقويم والفصول المعتمدة"}</span>
                </CardTitle>
              </CardHeader>
              <CardContent class="space-y-2 text-sm font-sans">
                <div class="flex justify-between"><span class="text-slate-500 font-medium">{m.onboarding_review_academic_year ? m.onboarding_review_academic_year() : "العام الدراسي:"}</span><span class="font-medium text-slate-200 font-sans">{academicYearNameAr}</span></div>
                <div class="flex justify-between"><span class="text-slate-500 font-medium">{m.onboarding_review_year_duration ? m.onboarding_review_year_duration() : "مدة العام:"}</span><span class="font-mono text-xs text-slate-400">{startDate} إلى {endDate}</span></div>
                <div class="flex justify-between"><span class="text-slate-500 font-medium">{m.onboarding_review_sem1 ? m.onboarding_review_sem1() : "الفصل الأول:"}</span><span class="font-mono text-xs text-slate-400">{semester1Start} إلى {semester1End}</span></div>
                <div class="flex justify-between"><span class="text-slate-500 font-medium">{m.onboarding_review_sem2 ? m.onboarding_review_sem2() : "الفصل الثاني:"}</span><span class="font-mono text-xs text-slate-400">{semester2Start} إلى {semester2End}</span></div>
              </CardContent>
            </Card>

            <!-- Buildings & Rooms card -->
            <Card class="bg-slate-900/20 border-slate-800">
              <CardHeader class="pb-2 text-start">
                <CardTitle class="text-sm font-semibold text-slate-400 flex gap-2 items-center">
                  <Building class="w-4 h-4 text-violet-400" />
                  <span>{m.onboarding_review_buildings ? m.onboarding_review_buildings() : "توزيع المنشآت والغرف"}</span>
                </CardTitle>
              </CardHeader>
              <CardContent class="space-y-2 text-sm">
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_bld1 ? m.onboarding_review_bld1() : "المبنى 1:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_bld_rooms_value ? m.onboarding_review_bld_rooms_value({ count: bld1Rooms }) : `${bld1Rooms} غرف ومختبرات`}
                </span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_bld2 ? m.onboarding_review_bld2() : "المبنى 2:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_bld_rooms_only ? m.onboarding_review_bld_rooms_only({ count: bld2Rooms }) : `${bld2Rooms} غرف`}
                </span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_capacity ? m.onboarding_review_capacity() : "إجمالي الطاقة الاستيعابية:"}</span><span class="font-medium text-violet-400">
                  {m.onboarding_review_capacity_value ? m.onboarding_review_capacity_value({ count: totalCapacity }) : `${totalCapacity} طالب`}
                </span></div>
              </CardContent>
            </Card>

            <!-- Stages & Subjects card -->
            <Card class="bg-slate-900/20 border-slate-800">
              <CardHeader class="pb-2 text-start">
                <CardTitle class="text-sm font-semibold text-slate-400 flex gap-2 items-center">
                  <GraduationCap class="w-4 h-4 text-violet-400" />
                  <span>{m.onboarding_review_stages ? m.onboarding_review_stages() : "الصفوف والمواد الدراسية"}</span>
                </CardTitle>
              </CardHeader>
              <CardContent class="space-y-2 text-sm">
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_stages_enabled ? m.onboarding_review_stages_enabled() : "الصفوف الدراسية المفعلة:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_stages_value ? m.onboarding_review_stages_value({ count: enabledStages }) : `${enabledStages} صفوف`}
                </span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_divisions_count ? m.onboarding_review_divisions_count() : "عدد الشعب الدراسي الكلية:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_divisions_value ? m.onboarding_review_divisions_value({ count: totalDivisions }) : `${totalDivisions} شعب`}
                </span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_subjects_count ? m.onboarding_review_subjects_count() : "المواد التعليمية المفعلة:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_subjects_value ? m.onboarding_review_subjects_value({ count: enabledSubjects }) : `${enabledSubjects} مواد`}
                </span></div>
                <div class="flex justify-between"><span class="text-slate-500">{m.onboarding_review_time_slots ? m.onboarding_review_time_slots() : "الحصص اليومية:"}</span><span class="font-medium text-slate-200">
                  {m.onboarding_review_slots_value ? m.onboarding_review_slots_value({ count: totalSlots }) : `${totalSlots} فترات`}
                </span></div>
              </CardContent>
            </Card>
          </div>

          <div class="p-5 rounded-xl border border-amber-500/20 bg-amber-500/5 text-amber-300 text-xs leading-relaxed max-w-4xl space-y-1 text-start">
            <h5 class="font-semibold text-amber-200">{m.onboarding_review_warn_title ? m.onboarding_review_warn_title() : "⚠️ تنبيه هام:"}</h5>
            <p>{m.onboarding_review_warn_desc ? m.onboarding_review_warn_desc() : "سيؤدي تدشين النظام إلى مسح أي هيكل مدرسي تجريبي قديم وإنشاء الحسابات المحاسبية والقيد التلقائي للعملات. يرجى الانتظار دون إغلاق التطبيق حتى تكتمل المعاملة تماماً."}</p>
          </div>

          <!-- Central Launch Button -->
          <div class="pt-6 flex justify-center">
            <Button 
              onclick={handleCompleteSetup} 
              disabled={isLoading}
              size="lg"
              class="px-10 py-6 text-base font-bold bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-700 hover:to-indigo-700 text-white rounded-xl shadow-xl shadow-violet-600/20 flex gap-3 items-center justify-center transition-all duration-300 hover:scale-105"
            >
              {#if isLoading}
                <Loader2 class="w-5 h-5 animate-spin" />
                <span>{m.onboarding_review_launching_btn ? m.onboarding_review_launching_btn() : "جاري تهيئة قاعدة البيانات والتأسيس المالي..."}</span>
              {:else}
                <Sparkles class="w-5 h-5 animate-pulse" />
                <span>{m.onboarding_review_launch_btn ? m.onboarding_review_launch_btn() : "إطلاق وتدشين النظام المدرسي"}</span>
              {/if}
            </Button>
          </div>
        </div>
      {/if}
    </div>

    <!-- Bottom Actions Control Bar -->
    <footer class="h-20 border-t border-slate-900 px-8 flex justify-between items-center shrink-0 bg-slate-950/80 backdrop-blur-md">
      <!-- Back button -->
      <Button 
        variant="outline" 
        onclick={() => activeStep > 1 && (activeStep -= 1)} 
        disabled={activeStep === 1 || isLoading}
        class="border-slate-800 text-slate-400 hover:text-slate-200 hover:bg-slate-900 flex gap-2 items-center"
      >
        <ArrowRight class="w-4 h-4" />
        <span>{m.onboarding_prev_btn ? m.onboarding_prev_btn() : "السابق"}</span>
      </Button>

      <!-- Next button -->
      {#if activeStep < 7}
        <Button 
          onclick={() => (activeStep += 1)} 
          disabled={isLoading}
          class="bg-violet-600 hover:bg-violet-700 text-white flex gap-2 items-center"
        >
          <span>{m.onboarding_next_btn ? m.onboarding_next_btn() : "التالي"}</span>
          <ArrowLeft class="w-4 h-4" />
        </Button>
      {/if}
    </footer>
  </main>
</div>

<style>
/* Smooth micro-animations and scroll hides */
:global(.dir-rtl) {
  direction: rtl;
}

/* Scroll area customization */
div::-webkit-scrollbar {
  width: 6px;
}
div::-webkit-scrollbar-thumb {
  background: rgba(148, 163, 184, 0.1);
  border-radius: 4px;
}
div::-webkit-scrollbar-thumb:hover {
  background: rgba(148, 163, 184, 0.2);
}
</style>