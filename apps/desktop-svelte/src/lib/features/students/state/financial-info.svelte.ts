import { commands } from "$lib/core/ipc";

export class FinancialInfoState {
  orgId: string;
  studentId?: string;

  discountTemplates = $state<any[]>([]);
  feeTemplates = $state<any[]>([]);
  applyDiscount = $state(false);
  discountType = $state("template"); // template | custom
  selectedDiscountTemplateId = $state("");
  customDiscountAmount = $state<number | null>(null);
  selectedOptionalFees = $state<
    Record<string, { checked: boolean; amount: number }>
  >({});

  constructor(orgId: string, studentId?: string) {
    this.orgId = orgId;
    this.studentId = studentId;
  }

  async init() {
    const [feeTpls, discountTpls] = await Promise.all([
      commands.finance.listFeeTemplates(this.orgId),
      commands.finance.listDiscountTemplates(this.orgId),
    ]);
    this.feeTemplates = feeTpls || [];
    this.discountTemplates = discountTpls || [];

    const feeMap: Record<string, { checked: boolean; amount: number }> = {};
    for (const fTpl of this.feeTemplates) {
      if (fTpl.is_mandatory === 0) {
        feeMap[String(fTpl.id)] = {
          checked: false,
          amount: Number(fTpl.default_amount) || 0,
        };
      }
    }
    this.selectedOptionalFees = feeMap;
  }

  async populate(originalStudent: any) {
    if (!originalStudent) return;

    if (originalStudent.discount_template_id) {
      this.selectedDiscountTemplateId = String(
        originalStudent.discount_template_id,
      );
      this.discountType = "template";
      this.applyDiscount = true;
    } else if (originalStudent.custom_discount_amount) {
      this.customDiscountAmount =
        Number(originalStudent.custom_discount_amount) || null;
      this.discountType = "custom";
      this.applyDiscount = true;
    }

    if (this.studentId) {
      try {
        const studentFees = await commands.finance.listStudentFees(
          this.studentId,
        );
        for (const fee of studentFees as any[]) {
          const feeTemplateId = fee.fee_template_id as string;
          if (feeTemplateId && this.selectedOptionalFees[feeTemplateId]) {
            this.selectedOptionalFees[feeTemplateId].checked = true;
            this.selectedOptionalFees[feeTemplateId].amount =
              Number(fee.amount) || 0;
          }
        }
      } catch (feeErr) {
        console.error("Failed to load student assigned fees:", feeErr);
      }
    }
  }
}
