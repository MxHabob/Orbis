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

export type FilterValue = string | number | boolean | Array<string | number>;

export interface FilterCondition {
    field: string;
    operator: FilterOperator;
    value?: FilterValue;
}

export interface UniversalFilter {
    logic: "AND" | "OR";
    conditions: FilterCondition[];
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
    applyFilter: "Apply Filter"
};
