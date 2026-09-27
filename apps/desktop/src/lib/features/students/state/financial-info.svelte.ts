import {
  api,
  type FeeTemplateDto,
  type StudentFeeDto,
  type StudentDetailDto,
  type DiscountTemplateDto,
} from "$lib/api";

export class FinancialInfoState {
  orgId: string;
  studentId?: string;

  feeTemplates = $state<FeeTemplateDto[]>([]);
  discountTemplates = $state<DiscountTemplateDto[]>([]);
  applyDiscount = $state(false);
  discountType = $state<"template" | "custom">("template");
  selectedDiscountTemplateId = $state("");
  customDiscountAmount = $state<number | null>(null);
  selectedOptionalFees = $state<
    Record<string, { checked: boolean; amount: number }>
  >({});
  isLoading = $state(false);

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
  }

  async init() {
    this.isLoading = true;
    try {
      const [feeTpls, discountTpls] = await Promise.all([
        api.listFeeTemplates().catch(() => []),
        api.listDiscountTemplates().catch(() => []),
      ]);
      this.feeTemplates = feeTpls || [];
      this.discountTemplates = discountTpls || [];

      const feeMap: Record<string, { checked: boolean; amount: number }> = {};
      for (const fTpl of this.feeTemplates) {
        if (!fTpl.is_mandatory) {
          feeMap[fTpl.id] = {
            checked: false,
            amount: Number(fTpl.default_amount) || 0,
          };
        }
      }
      this.selectedOptionalFees = feeMap;
    } catch (err) {
      console.error("Failed to initialize financial info state:", err);
    } finally {
      this.isLoading = false;
    }
  }

  async populate(originalStudent: StudentDetailDto | null | undefined) {
    if (!originalStudent) return;

    if (this.studentId) {
      try {
        const studentFees: StudentFeeDto[] = await api.listStudentFees({
          student_id: this.studentId,
        });

        for (const fee of studentFees) {
          const feeTemplateId = fee.fee_template_id;
          if (feeTemplateId && this.selectedOptionalFees[feeTemplateId]) {
            this.selectedOptionalFees[feeTemplateId].checked = true;
            this.selectedOptionalFees[feeTemplateId].amount =
              Number(fee.net_amount || fee.original_amount) || 0;
          }
        }
      } catch (feeErr) {
        console.error("Failed to load student assigned fees:", feeErr);
      }
    }
  }
}
