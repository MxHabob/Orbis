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
  | "bool_false";

export type FilterValue =
  | string
  | number
  | boolean
  | Array<string | number>
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
};
