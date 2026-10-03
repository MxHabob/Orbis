<!--
  @component StepBlueprint
  Step 2: Selection of Institution Blueprint.
  Spacious, elegant 4-model selection with design tokens and feature highlights.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { BLUEPRINTS, type InstitutionType, type InstitutionBlueprint } from "@orbis/blueprints";
import { Badge } from "@orbis/ui/badge";
import { Check } from "@orbis/icons";

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

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {m.ui_onboarding_blueprint_step_2_of_6()}
      </span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {m.ui_onboarding_blueprint_select_institutional_blueprint()}
    </h2>
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {isRTL
        ? "يتم تكييف الهيكل الأكاديمي، نظام التقييم، وتوزيع الفصول تلقائياً وفقاً للنموذج المختار."
        : "Evaluation policies, academic structures, and core operational rules will be calibrated to this model."}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each blueprintsList as item}
      {@const isSelected = selectedType === item.type}

      <button
        type="button"
        onclick={() => (selectedType = item.type)}
        class="text-start focus:outline-none w-full group"
      >
        <div
          class="h-full rounded-2xl border p-6 transition-all cursor-pointer relative flex flex-col justify-between {isSelected
            ? 'border-primary bg-primary/[0.04] shadow-md shadow-primary/10 ring-1 ring-primary/40'
            : 'border-border/70 hover:border-border bg-card shadow-xs'}"
        >
          <div class="space-y-3">
            <div class="flex items-start justify-between gap-3">
              <div>
                <h3 class="text-base font-bold text-foreground group-hover:text-primary transition-colors">
                  {isRTL ? item.blueprint.nameAr : item.blueprint.nameEn}
                </h3>
                <p class="text-xs text-muted-foreground mt-1 line-clamp-2 leading-relaxed">
                  {isRTL ? item.blueprint.descriptionAr : item.blueprint.descriptionEn}
                </p>
              </div>

              <div
                class="size-6 rounded-full flex items-center justify-center shrink-0 transition-colors {isSelected
                  ? 'bg-primary text-primary-foreground'
                  : 'border border-border/70 bg-muted/40 text-transparent'}"
              >
                <Check size={14} strokeWidth={3} />
              </div>
            </div>
          </div>

          <div class="flex flex-wrap gap-1.5 mt-5 pt-3 border-t border-border/40">
            {#each (isRTL ? item.highlightsAr : item.highlightsEn) as highlight}
              <Badge
                variant="outline"
                class="text-[11px] px-2 py-0.5 font-normal text-muted-foreground border-border/60 bg-muted/20"
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
