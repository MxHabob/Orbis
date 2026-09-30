<!--
  @component StepBlueprint
  Step 1: Selection of the Institution Type / Blueprint.
-->
<script lang="ts">
import { BLUEPRINTS, type InstitutionType, type InstitutionBlueprint } from "@orbis/blueprints";
import { AcademicCap, BookOpen, BuildingLibrary, BuildingOffice, Check, Sparkles } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";

let {
  selectedType = $bindable<InstitutionType>(),
  isRTL = false,
}: {
  selectedType: InstitutionType;
  isRTL?: boolean;
} = $props();

interface BlueprintCardOption {
  type: InstitutionType;
  icon: any;
  blueprint: InstitutionBlueprint;
  highlightsEn: string[];
  highlightsAr: string[];
}

const blueprintsList: BlueprintCardOption[] = [
  {
    type: "k12_school",
    icon: AcademicCap,
    blueprint: BLUEPRINTS.k12_school,
    highlightsEn: ["Homeroom & Grade Levels", "Guardian Portals", "Student Bus Routing", "Conduct & Attendance"],
    highlightsAr: ["الفصول والمراحل الدراسية", "بوابة أولياء الأمور", "مسارات النقل المدرسي", "السلوك والمواظبة"],
  },
  {
    type: "university",
    icon: BuildingLibrary,
    blueprint: BLUEPRINTS.university,
    highlightsEn: ["Credit Hours & GPA", "Faculties & Majors", "Academic Advising", "Semester Registrations"],
    highlightsAr: ["الساعات المعتمدة والمعدل", "الكليات والتخصصات", "الإرشاد الأكاديمي", "التسجيل الفصلي"],
  },
  {
    type: "vocational_institute",
    icon: BuildingOffice,
    blueprint: BLUEPRINTS.vocational_institute,
    highlightsEn: ["Certificate Tracks", "Cohort Batches", "Practical Training Hours", "Modular Invoicing"],
    highlightsAr: ["المسارات المهنية المعتمدة", "الدفعات التدريبية", "ساعات التدريب العملي", "الفواتير المرنة"],
  },
  {
    type: "language_center",
    icon: BookOpen,
    blueprint: BLUEPRINTS.language_center,
    highlightsEn: ["Level Placements", "Intensive Workshops", "Flexible Timetables", "Attendance Certifications"],
    highlightsAr: ["اختبارات تحديد المستوى", "الدورات المكثفة", "جداول مرنة", "شهادات الحضور"],
  },
];
</script>

<div class="space-y-6">
  <div class="text-center max-w-xl mx-auto space-y-2">
    <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">
      <Sparkles size={14} />
      <span>{isRTL ? "الخطوة الأولى" : "Step 1 of 4"}</span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isRTL ? "اختر النموذج الأنسب لمؤسستك التعليمية" : "Select Your Institution Blueprint"}
    </h2>
    <p class="text-sm text-muted-foreground">
      {isRTL
        ? "سيقوم أوربيس بتهيئة الهيكل الأكاديمي، الصلاحيات، والوحدات تلقائياً بناءً على هذا الاختيار."
        : "Orbis will tailor academic structures, workflows, and modules based on this selection."}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-w-4xl mx-auto">
    {#each blueprintsList as item}
      {@const isSelected = selectedType === item.type}
      {@const Icon = item.icon}
      
      <button
        type="button"
        onclick={() => (selectedType = item.type)}
        class="relative flex flex-col text-start p-5 rounded-2xl border-2 transition-all duration-200 group focus:outline-none focus-visible:ring-2 focus-visible:ring-primary {isSelected
          ? 'border-primary bg-primary/5 shadow-md shadow-primary/10 ring-1 ring-primary'
          : 'border-border bg-card/50 hover:bg-card hover:border-border/80 hover:shadow-sm'}"
      >
        <!-- Selection Check Badge -->
        <div class="absolute top-4 {isRTL ? 'left-4' : 'right-4'}">
          <div
            class="size-6 rounded-full flex items-center justify-center transition-all {isSelected
              ? 'bg-primary text-primary-foreground scale-100'
              : 'border-2 border-muted-foreground/30 text-transparent scale-90 group-hover:border-muted-foreground/60'}"
          >
            <Check size={14} strokeWidth={3} />
          </div>
        </div>

        <!-- Icon & Title -->
        <div class="flex items-start gap-3.5 mb-3">
          <div
            class="size-11 rounded-xl flex items-center justify-center shrink-0 transition-colors {isSelected
              ? 'bg-primary text-primary-foreground shadow-sm shadow-primary/20'
              : 'bg-muted text-foreground/80 group-hover:bg-primary/10 group-hover:text-primary'}"
          >
            <Icon size={22} />
          </div>
          <div class="pe-8">
            <h3 class="font-semibold text-base text-foreground group-hover:text-primary transition-colors">
              {isRTL ? item.blueprint.nameAr : item.blueprint.nameEn}
            </h3>
            <span class="text-xs text-muted-foreground font-mono">
              {item.type}
            </span>
          </div>
        </div>

        <!-- Description -->
        <p class="text-xs text-muted-foreground line-clamp-2 mb-4 leading-relaxed">
          {isRTL ? item.blueprint.descriptionAr : item.blueprint.descriptionEn}
        </p>

        <!-- Feature Highlights -->
        <div class="mt-auto pt-3 border-t border-border/60">
          <div class="text-[11px] font-medium text-muted-foreground mb-2">
            {isRTL ? "الميزات المهيأة مسبقاً:" : "Preconfigured Capabilities:"}
          </div>
          <div class="flex flex-wrap gap-1.5">
            {#each (isRTL ? item.highlightsAr : item.highlightsEn) as highlight}
              <Badge
                variant={isSelected ? "default" : "secondary"}
                class="text-[10px] px-2 py-0.5 font-normal {isSelected ? 'bg-primary/15 text-primary hover:bg-primary/20 border-primary/20' : ''}"
              >
                {highlight}
              </Badge>
            {/each}
          </div>
        </div>
      </button>
    {/each}
  </div>
</div>
