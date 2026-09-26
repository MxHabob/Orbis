import { api } from "$lib/api";

export class FinancialInfoState {
  orgId: string;
  studentId?: string;

  feeTemplates = $state<any[]>([]);
  discountTemplates = $state<any[]>([]);
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
        api.listDiscountTemplates ? api.listDiscountTemplates().catch(() => []) : Promise.resolve([]),
      ]);

      this.feeTemplates = feeTpls || [];
      this.discountTemplates = discountTpls || [];

      const feeMap: Record<string, { checked: boolean; amount: number }> = {};
      for (const fTpl of this.feeTemplates) {
        if (fTpl.is_mandatory === 0 || fTpl.is_mandatory === false) {
          feeMap[String(fTpl.id)] = {
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
        const studentFees = await api.listStudentFees({
          enrollmentId: this.studentId,
        });

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
