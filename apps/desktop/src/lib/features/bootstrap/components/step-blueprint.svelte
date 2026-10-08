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
  highlights: string[];
}

const blueprintsList: BlueprintOption[] = [
  {
    type: "k12_school",
    blueprint: BLUEPRINTS.k12_school,
    highlights: [
      m.operations_ui_blueprint_k12_highlight_grades(),
      m.operations_ui_blueprint_k12_highlight_guardian_portal(),
      m.operations_ui_blueprint_k12_highlight_bus_routing(),
      m.operations_ui_blueprint_k12_highlight_attendance_conduct(),
    ],
  },
  {
    type: "university",
    blueprint: BLUEPRINTS.university,
    highlights: [
      m.operations_ui_blueprint_university_highlight_credits(),
      m.operations_ui_blueprint_university_highlight_faculties(),
      m.operations_ui_blueprint_university_highlight_advising(),
      m.operations_ui_blueprint_university_highlight_registration(),
    ],
  },
  {
    type: "vocational_institute",
    blueprint: BLUEPRINTS.vocational_institute,
    highlights: [
      m.operations_ui_blueprint_vocational_highlight_tracks(),
      m.operations_ui_blueprint_vocational_highlight_cohorts(),
      m.operations_ui_blueprint_vocational_highlight_training_hours(),
      m.operations_ui_blueprint_vocational_highlight_invoicing(),
    ],
  },
  {
    type: "language_center",
    blueprint: BLUEPRINTS.language_center,
    highlights: [
      m.operations_ui_blueprint_language_highlight_placement(),
      m.operations_ui_blueprint_language_highlight_intensive_courses(),
      m.operations_ui_blueprint_language_highlight_flexible_schedules(),
      m.operations_ui_blueprint_language_highlight_attendance_certificates(),
    ],
  },
];
</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {m.operations_ui_blueprint_selection_description()}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each blueprintsList as item (item.type)}
      {@const isSelected = selectedType === item.type}

      <button
        type="button"
        aria-pressed={isSelected}
        onclick={() => (selectedType = item.type)}
        class="group w-full text-start focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
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
            {#each item.highlights as highlight (highlight)}
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
