import { detectTimezone, detectLocale, detectCurrency } from "../utils";

export class ProfileInfoState {
  institutionType = $state("school");
  studentNumberPattern = $state("STU-{YEAR}-{SEQ}");
  activeFields = $state<Record<string, boolean>>({});
  requiredFields = $state<Record<string, boolean>>({});
  preventDuplicates = $state<Record<string, boolean>>({});
  requiredDocumentsList = $state<string[]>([]);

  orgName = $state("Sanaa Model School");
  orgNameAr = $state("مدرسة صنعاء النموذجية");
  rootUnitName = $state("Sanaa Model School");
  rootUnitNameAr = $state("مدرسة صنعاء النموذجية");
  timezone = $state(detectTimezone());
  locale = $state(detectLocale());
  currency = $state(detectCurrency(detectTimezone()));

  serialize() {
    return {
      institutionType: this.institutionType,
      studentNumberPattern: this.studentNumberPattern,
      activeFields: this.activeFields,
      requiredFields: this.requiredFields,
      preventDuplicates: this.preventDuplicates,
      requiredDocumentsList: this.requiredDocumentsList,
      orgName: this.orgName,
      orgNameAr: this.orgNameAr,
      rootUnitName: this.rootUnitName,
      rootUnitNameAr: this.rootUnitNameAr,
      timezone: this.timezone,
      locale: this.locale,
      currency: this.currency,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.institutionType) this.institutionType = data.institutionType;
    if (data.studentNumberPattern) this.studentNumberPattern = data.studentNumberPattern;
    if (data.activeFields) this.activeFields = data.activeFields;
    if (data.requiredFields) this.requiredFields = data.requiredFields;
    if (data.preventDuplicates) this.preventDuplicates = data.preventDuplicates;
    if (data.requiredDocumentsList) this.requiredDocumentsList = data.requiredDocumentsList;
    if (data.orgName) this.orgName = data.orgName;
    if (data.orgNameAr) this.orgNameAr = data.orgNameAr;
    if (data.rootUnitName) this.rootUnitName = data.rootUnitName;
    if (data.rootUnitNameAr) this.rootUnitNameAr = data.rootUnitNameAr;
    if (data.timezone) this.timezone = data.timezone;
    if (data.locale) this.locale = data.locale;
    if (data.currency) this.currency = data.currency;
  }
}
