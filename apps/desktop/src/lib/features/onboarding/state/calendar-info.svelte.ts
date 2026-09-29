import type { PeriodTemplateDef } from "../types";
import { api } from "$lib/api";

// Generate dynamic defaults based on current date
function getDefaultStartDate(): string {
  const now = new Date();
  // If we're past August, use this year's September; otherwise use last year's
  const year = now.getMonth() >= 7 ? now.getFullYear() : now.getFullYear();
  // Default academic year starts September 1st
  return `${year}-09-01`;
}

function getDefaultEndDate(): string {
  const start = new Date(getDefaultStartDate());
  // Academic year is typically ~10 months (June 30)
  const endYear = start.getFullYear() + 1;
  return `${endYear}-06-30`;
}

function getDefaultYearName(): string {
  const start = new Date(getDefaultStartDate());
  const end = new Date(getDefaultEndDate());
  return `Academic Year ${start.getFullYear()}-${end.getFullYear()}`;
}

function getDefaultYearNameAr(): string {
  const start = new Date(getDefaultStartDate());
  const end = new Date(getDefaultEndDate());
  return `العام الدراسي ${start.getFullYear()}-${end.getFullYear()}`;
}

export class CalendarInfoState {
  academicYearName = $state(getDefaultYearName());
  academicYearNameAr = $state(getDefaultYearNameAr());
  startDate = $state(getDefaultStartDate());
  endDate = $state(getDefaultEndDate());
  selectedTemplateId = $state<string>("");
  periodTemplates = $state<PeriodTemplateDef[]>([]);
  isLoadingTemplates = $state(false);

  constructor() {
    this.loadPeriodTemplates();
  }

  async loadPeriodTemplates() {
    this.isLoadingTemplates = true;
    try {
      const templates = await api.listPeriodTemplates();
      if (Array.isArray(templates) && templates.length > 0) {
        this.periodTemplates = templates.map((t) => ({
          id: t.id,
          name: t.name,
          name_ar: t.name_ar || t.name,
          structure: (t.structure || []) as any,
        }));
        if (!this.selectedTemplateId || !this.periodTemplates.some((t) => t.id === this.selectedTemplateId)) {
          const defaultTpl = (templates as any).find((t: any) => t.is_default) || this.periodTemplates[0];
          this.selectedTemplateId = defaultTpl?.id || this.periodTemplates[0]?.id || "";
        }
      }
    } catch (e) {
      console.warn("Failed to load period templates from bootstrap:", e);
    } finally {
      this.isLoadingTemplates = false;
    }
  }

  serialize() {
    return {
      academicYearName: this.academicYearName,
      academicYearNameAr: this.academicYearNameAr,
      startDate: this.startDate,
      endDate: this.endDate,
      selectedTemplateId: this.selectedTemplateId,
      periodTemplates: this.periodTemplates,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.academicYearName) this.academicYearName = data.academicYearName;
    if (data.academicYearNameAr)
      this.academicYearNameAr = data.academicYearNameAr;
    if (data.startDate) this.startDate = data.startDate;
    if (data.endDate) this.endDate = data.endDate;
    if (data.selectedTemplateId)
      this.selectedTemplateId = data.selectedTemplateId;
    if (
      data.periodTemplates &&
      Array.isArray(data.periodTemplates) &&
      data.periodTemplates.length > 0 &&
      data.periodTemplates.some((t: any) => t.structure)
    ) {
      this.periodTemplates = data.periodTemplates;
    } else {
      this.loadPeriodTemplates();
    }
  }
}
