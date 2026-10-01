<!--
  @component StepBlueprint
  Step 1: Selection of Institution Blueprint.
  Minimalist, elegant card selection with design tokens.
-->
<script lang="ts">
import { BLUEPRINTS, type InstitutionType, type InstitutionBlueprint } from "@orbis/blueprints";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";

let {
  selectedType = $bindable<InstitutionType>(),
  isRTL = false,
}: {
  selectedType: InstitutionType;
  isRTL?: boolean;
} = $props();

interface BlueprintOption {
  type: InstitutionType;
  blueprint: InstitutionBlueprint;
  highlightsEn: string[];
  highlightsAr: string[];
}

const blueprintsList: BlueprintOption[] = [
  {
    type: "k12_school",
    blueprint: BLUEPRINTS.k12_school,
    highlightsEn: ["Grade Levels & Homerooms", "Guardian Portals", "Student Bus Routing", "Attendance & Conduct"],
    highlightsAr: ["المراحل والفصول الدراسية", "بوابة أولياء الأمور", "حافلات النقل المدرسي", "المواظبة والسلوك"],
  },
  {
    type: "university",
    blueprint: BLUEPRINTS.university,
    highlightsEn: ["Credit Hours & GPA", "Faculties & Departments", "Academic Advising", "Semester Registrations"],
    highlightsAr: ["الساعات المعتمدة والمعدل", "الكليات والأقسام", "الإرشاد الأكاديمي", "التسجيل الفصلي"],
  },
  {
    type: "vocational_institute",
    blueprint: BLUEPRINTS.vocational_institute,
    highlightsEn: ["Certificate Tracks", "Cohort Batches", "Training Hours", "Modular Invoicing"],
    highlightsAr: ["المسارات المهنية", "الدفعات التدريبية", "ساعات التدريب العملي", "الفواتير المرنة"],
  },
  {
    type: "language_center",
    blueprint: BLUEPRINTS.language_center,
    highlightsEn: ["Placement Tests", "Intensive Courses", "Flexible Schedules", "Attendance Certifications"],
    highlightsAr: ["تحديد المستوى", "الدورات المكثفة", "جداول مرنة", "شهادات الحضور"],
  },
];
</script>

<div class="space-y-6 max-w-3xl mx-auto">
  <div class="space-y-1">
    <h2 class="text-lg font-semibold tracking-tight text-foreground">
      {isRTL ? "اختر النموذج الأنسب لمؤسستك" : "Select Institution Blueprint"}
    </h2>
    <p class="text-xs text-muted-foreground">
      {isRTL
        ? "سيتم تهيئة الهيكل الأكاديمي واللائحة التقييمية والصلاحيات تلقائياً بناءً على هذا الاختيار."
        : "Initial academic structures and evaluation policies will be pre-configured based on this model."}
    </p>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
    {#each blueprintsList as item}
      {@const isSelected = selectedType === item.type}
      
      <button
        type="button"
        onclick={() => (selectedType = item.type)}
        class="text-start focus:outline-none"
      >
        <Card class="h-full border transition-colors cursor-pointer {isSelected
          ? 'border-primary bg-primary/5 shadow-xs'
          : 'border-border/60 hover:border-border bg-card'}">
          <CardHeader class="p-4 pb-2 space-y-1">
            <div class="flex items-center justify-between gap-2">
              <CardTitle class="text-sm font-semibold text-foreground">
                {isRTL ? item.blueprint.nameAr : item.blueprint.nameEn}
              </CardTitle>
              {#if isSelected}
                <span class="size-2 rounded-full bg-primary shrink-0"></span>
              {/if}
            </div>
            <CardDescription class="text-xs text-muted-foreground line-clamp-2">
              {isRTL ? item.blueprint.descriptionAr : item.blueprint.descriptionEn}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-4 pt-2">
            <div class="flex flex-wrap gap-1 mt-2">
              {#each (isRTL ? item.highlightsAr : item.highlightsEn) as highlight}
                <Badge
                  variant="outline"
                  class="text-[10px] px-1.5 py-0 h-4 font-normal text-muted-foreground"
                >
                  {highlight}
                </Badge>
              {/each}
            </div>
          </CardContent>
        </Card>
      </button>
    {/each}
  </div>
</div>
