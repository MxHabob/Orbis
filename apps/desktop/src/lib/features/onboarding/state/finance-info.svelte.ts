export class FinanceInfoState {
  registrationFees = $state<Record<string, number>>({});

  serialize() {
    return {
      registrationFees: this.registrationFees,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.registrationFees) this.registrationFees = data.registrationFees;
  }
}
