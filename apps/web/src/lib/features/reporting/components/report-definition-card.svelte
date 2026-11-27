<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import type { ReportDefinitionDto } from "$lib/api/types";
import { Card, CardContent } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { DocumentPage, Play } from "@orbis/icons";
import { getReportDomainIcon } from "../report-catalog";

let {
  definition,
  onGenerate,
  onManageTemplate,
}: {
  definition: ReportDefinitionDto;
  onGenerate: (def: ReportDefinitionDto) => void;
  onManageTemplate: (def: ReportDefinitionDto) => void;
} = $props();

const IconComponent = $derived(
  getReportDomainIcon(definition.domain, definition.category),
);
const title = $derived(
  l(definition.name_ar, definition.name) || definition.name,
);
const description = $derived(
  l(definition.description_ar, definition.description) ||
    m.operations_ui_report_default_description(),
);
const formatsLabel = $derived(
  (definition.supported_formats?.length
    ? definition.supported_formats
    : ["PDF"]
  ).join(" · "),
);
</script>

<Card class="flex h-full flex-col justify-between transition-colors duration-200 hover:border-border">
  <CardContent class="flex flex-1 flex-col justify-between gap-4 p-5 text-start">
    <div class="flex flex-col gap-3">
      <div class="flex items-start justify-between gap-3">
        <div class="flex size-9 shrink-0 items-center justify-center rounded-md border border-border/60 bg-muted/50 text-foreground/80">
          <IconComponent size={16} />
        </div>
        <span class="font-mono text-[10px] font-medium uppercase tracking-wider text-muted-foreground">
          {formatsLabel}
        </span>
      </div>

      <div class="flex flex-col gap-1">
        <h3 class="text-sm font-semibold leading-snug text-foreground line-clamp-1">
          {title}
        </h3>
        <p class="text-xs leading-relaxed text-muted-foreground line-clamp-2">
          {description}
        </p>
      </div>
    </div>

    <div class="flex items-center justify-between gap-2 border-t border-border/50 pt-3">
      <Button
        type="button"
        variant="ghost"
        size="sm"
        onclick={() => onManageTemplate(definition)}
        class="h-8 px-2 text-xs text-muted-foreground hover:text-foreground gap-1.5"
      >
        <DocumentPage size={13} />
        <span>{m.operations_ui_report_template_manage()}</span>
      </Button>

      <Button
        type="button"
        variant="outline"
        size="sm"
        onclick={() => onGenerate(definition)}
        class="h-8 px-3 text-xs font-medium gap-1.5"
      >
        <Play size={12} />
        <span>{m.operations_ui_generate_report()}</span>
      </Button>
    </div>
  </CardContent>
</Card>
