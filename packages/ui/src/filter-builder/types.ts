export enum FilterOperator {
  Eq = "Eq",
  Ne = "Ne",
  Gt = "Gt",
  Gte = "Gte",
  Lt = "Lt",
  Lte = "Lte",
  Like = "Like",
  NotLike = "NotLike",
  In = "In",
  NotIn = "NotIn",
  IsNull = "IsNull",
  IsNotNull = "IsNotNull",
  StartsWith = "starts_with",
  EndsWith = "ends_with",
  Between = "between",
  BoolTrue = "bool_true",
  BoolFalse = "bool_false",
}

export type FilterOperatorType =
  | FilterOperator
  | "Eq"
  | "Ne"
  | "Gt"
  | "Gte"
  | "Lt"
  | "Lte"
  | "Like"
  | "NotLike"
  | "In"
  | "NotIn"
  | "IsNull"
  | "IsNotNull"
  | "eq"
  | "neq"
  | "gt"
  | "gte"
  | "lt"
  | "lte"
  | "contains"
  | "not_contains"
  | "starts_with"
  | "ends_with"
  | "in"
  | "not_in"
  | "is_null"
  | "is_not_null"
  | "between"
  | "date_before"
  | "date_after"
  | "date_between"
  | "date_is_today"
  | "date_in_last"
  | "date_is_this_week"
  | "date_is_this_month"
  | "bool_true"
  | "bool_false"
  | (string & {});

export type FilterValue =
  | string
  | number
  | boolean
  | Array<string | number | boolean>
  | undefined
  | null;

export interface FilterCondition {
  field: string;
  operator: FilterOperatorType;
  value?: FilterValue;
}

export interface UniversalFilter {
  logic: "AND" | "OR";
  conditions: FilterCondition[];
  search?: string;
  groups?: UniversalFilter[];
}

export interface FieldDefinition {
  id: string;
  label: string;
  type: "string" | "number" | "boolean" | "date" | "select";
  options?: { label: string; value: string | number }[];
}

export interface FilterTranslations {
  filter: string;
  advancedFilter: string;
  matchAll: string;
  matchAny: string;
  rule: string;
  group: string;
  removeGroup: string;
  noRules: string;
  addRule: string;
  addSubGroup: string;
  removeCondition: string;
  clearAll: string;
  applyFilter: string;
  selectField: string;
  selectValue: string;
  valuePlaceholder: string;
  notApplicable: string;
  operatorEqual: string;
  operatorNotEqual: string;
  operatorGreaterThan: string;
  operatorGreaterThanOrEqual: string;
  operatorLessThan: string;
  operatorLessThanOrEqual: string;
  operatorContains: string;
  operatorNotContains: string;
  operatorIn: string;
  operatorNotIn: string;
  operatorStartsWith: string;
  operatorEndsWith: string;
  operatorBetween: string;
  operatorIs: string;
  operatorIsEmpty: string;
  operatorIsNotEmpty: string;
  trueValue: string;
  falseValue: string;
}

export interface FilterBuilderProps {
  filter: UniversalFilter;
  fields: FieldDefinition[];
  isRoot?: boolean;
  onRemove?: () => void;
  translations?: FilterTranslations;
}

export const defaultFilterTranslations: FilterTranslations = {
  filter: "Filter",
  advancedFilter: "Advanced Filter",
  matchAll: "Match ALL of the following:",
  matchAny: "Match ANY of the following:",
  rule: "Rule",
  group: "Group",
  removeGroup: "Remove Group",
  noRules: "No rules defined in this group.",
  addRule: "Add Rule",
  addSubGroup: "Add Sub-Group",
  removeCondition: "Remove Condition",
  clearAll: "Clear All",
  applyFilter: "Apply Filter",
  selectField: "Select field",
  selectValue: "Select value",
  valuePlaceholder: "Value...",
  notApplicable: "N/A",
  operatorEqual: "=",
  operatorNotEqual: "≠",
  operatorGreaterThan: ">",
  operatorGreaterThanOrEqual: "≥",
  operatorLessThan: "<",
  operatorLessThanOrEqual: "≤",
  operatorContains: "Contains",
  operatorNotContains: "Not contains",
  operatorIn: "In",
  operatorNotIn: "Not in",
  operatorStartsWith: "Starts with",
  operatorEndsWith: "Ends with",
  operatorBetween: "Between",
  operatorIs: "Is",
  operatorIsEmpty: "Is empty",
  operatorIsNotEmpty: "Is not empty",
  trueValue: "True",
  falseValue: "False",
};

export const arabicFilterTranslations: FilterTranslations = {
  filter: "تصفية",
  advancedFilter: "تصفية متقدمة",
  matchAll: "مطابقة جميع الشروط التالية:",
  matchAny: "مطابقة أي شرط من التالي:",
  rule: "شرط",
  group: "مجموعة",
  removeGroup: "إزالة المجموعة",
  noRules: "لا توجد شروط في هذه المجموعة.",
  addRule: "إضافة شرط",
  addSubGroup: "إضافة مجموعة فرعية",
  removeCondition: "إزالة الشرط",
  clearAll: "مسح عوامل التصفية",
  applyFilter: "تطبيق التصفية",
  selectField: "اختر الحقل",
  selectValue: "اختر قيمة",
  valuePlaceholder: "القيمة...",
  notApplicable: "لا ينطبق",
  operatorEqual: "يساوي",
  operatorNotEqual: "لا يساوي",
  operatorGreaterThan: "أكبر من",
  operatorGreaterThanOrEqual: "أكبر من أو يساوي",
  operatorLessThan: "أقل من",
  operatorLessThanOrEqual: "أقل من أو يساوي",
  operatorContains: "يحتوي على",
  operatorNotContains: "لا يحتوي على",
  operatorIn: "ضمن القيم",
  operatorNotIn: "ليس ضمن القيم",
  operatorStartsWith: "يبدأ بـ",
  operatorEndsWith: "ينتهي بـ",
  operatorBetween: "بين",
  operatorIs: "هو",
  operatorIsEmpty: "فارغ",
  operatorIsNotEmpty: "غير فارغ",
  trueValue: "نعم",
  falseValue: "لا",
};
