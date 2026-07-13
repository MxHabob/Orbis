import type { PeriodTemplateDef } from "../types";

export class CalendarInfoState {
  academicYearName = $state("Academic Year 1447 Hijri");
  academicYearNameAr = $state("العام الدراسي 1447 هـ");
  startDate = $state("2026-06-21");
  endDate = $state("2027-02-21");
  selectedTemplateId = $state("");
  periodTemplates = $state<PeriodTemplateDef[]>([]);

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
    if (data.academicYearNameAr) this.academicYearNameAr = data.academicYearNameAr;
    if (data.startDate) this.startDate = data.startDate;
    if (data.endDate) this.endDate = data.endDate;
    if (data.selectedTemplateId) this.selectedTemplateId = data.selectedTemplateId;
    if (data.periodTemplates) this.periodTemplates = data.periodTemplates;
  }
}
