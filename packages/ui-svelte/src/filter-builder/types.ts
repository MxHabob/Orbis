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
