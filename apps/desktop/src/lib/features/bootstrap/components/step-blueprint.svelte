<!--
  @component StepBlueprint
  Step 2: Selection of Institution Blueprint.
  Spacious, elegant 4-model selection with design tokens and feature highlights.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { BLUEPRINTS, type InstitutionType, type InstitutionBlueprint } from "@orbis/blueprints";
import { Check } from "@orbis/icons";
import { Card, CardContent } from "@orbis/ui/card";

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

<div class="mx-auto max-w-5xl space-y-5 py-2">
  <div class="text-start">
    <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">
      {m.operations_ui_blueprint_selection_description()}
    </p>
  </div>

  <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
    {#each blueprintsList as item, index (item.type)}
      {@const isSelected = selectedType === item.type}

      <button
        type="button"
        aria-pressed={isSelected}
        onclick={() => (selectedType = item.type)}
        class="group w-full rounded-xl text-start focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
      >
        <Card class="h-full">
          <CardContent class="h-full p-5">
          <div class="flex items-start gap-4">
            <span
              aria-hidden="true"
              class="flex size-10 shrink-0 items-center justify-center rounded-lg text-xs font-semibold tabular-nums {isSelected
                ? 'bg-primary text-primary-foreground'
                : 'bg-muted text-muted-foreground'}"
            >
              {String(index + 1).padStart(2, "0")}
            </span>

            <div class="min-w-0 flex-1">
              <div class="flex items-start justify-between gap-3">
                <h3 class="text-base font-semibold leading-6 text-foreground transition-colors group-hover:text-primary">
                  {isRTL ? item.blueprint.nameAr : item.blueprint.nameEn}
                </h3>
                <span
                  aria-hidden="true"
                  class="flex size-5 shrink-0 items-center justify-center rounded-full border transition-colors {isSelected
                    ? 'border-primary bg-primary text-primary-foreground'
                    : 'border-border text-transparent'}"
                >
                  <Check size={12} strokeWidth={3} />
                </span>
              </div>
              <p class="mt-1 line-clamp-2 text-xs leading-relaxed text-muted-foreground">
                {isRTL ? item.blueprint.descriptionAr : item.blueprint.descriptionEn}
              </p>
            </div>
          </div>

          <ul class="mt-4 grid grid-cols-2 gap-x-4 gap-y-2 border-t border-border/50 pt-3">
            {#each item.highlights as highlight, index (`${item.type}-${index}`)}
              <li class="flex min-w-0 items-start gap-2 text-xs leading-5 text-muted-foreground">
                <span class="mt-2 size-1 shrink-0 rounded-full {isSelected ? 'bg-primary' : 'bg-muted-foreground/60'}"></span>
                <span>{highlight}</span>
              </li>
            {/each}
          </ul>
          </CardContent>
        </Card>
      </button>
    {/each}
  </div>
</div>
