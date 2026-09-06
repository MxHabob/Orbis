import type { UniversalFilter, FilterCondition } from "./types.js";
import { isAfter, isBefore, isSameDay, subDays, startOfWeek, endOfWeek, startOfMonth, endOfMonth } from "date-fns";

export function applyFilter<T extends Record<string, any>>(items: T[], filter: UniversalFilter | undefined | null): T[] {
  if (!filter) return items;

  return items.filter((item) => {
    // 1. Search term
    if (filter.search) {
      const q = filter.search.toLowerCase();
      const matchesSearch = Object.values(item).some(
        (val) => typeof val === "string" && val.toLowerCase().includes(q)
      );
      if (!matchesSearch) return false;
    }

    // 2. Conditions
    if (!filter.conditions || filter.conditions.length === 0) return true;

    const evaluateCondition = (cond: FilterCondition, data: T): boolean => {
      const val = data[cond.field];
      const target = cond.value;
      const op = String(cond.operator).toLowerCase();

      switch (op) {
        case "eq":
          return String(val).toLowerCase() === String(target ?? "").toLowerCase();
        case "ne":
        case "neq":
          return String(val).toLowerCase() !== String(target ?? "").toLowerCase();
        case "like":
        case "contains":
          return String(val ?? "").toLowerCase().includes(String(target ?? "").toLowerCase());
        case "notlike":
        case "not_contains":
          return !String(val ?? "").toLowerCase().includes(String(target ?? "").toLowerCase());
        case "starts_with":
          return String(val ?? "").toLowerCase().startsWith(String(target ?? "").toLowerCase());
        case "ends_with":
          return String(val ?? "").toLowerCase().endsWith(String(target ?? "").toLowerCase());
        
        case "isnull":
        case "is_null":
          return val === null || val === undefined || val === "";
        case "isnotnull":
        case "is_not_null":
          return val !== null && val !== undefined && val !== "";
        
        case "gt":
          return Number(val) > Number(target);
        case "gte":
          return Number(val) >= Number(target);
        case "lt":
          return Number(val) < Number(target);
        case "lte":
          return Number(val) <= Number(target);
        case "between": {
          const num = Number(val);
          const [min, max] = Array.isArray(target) ? target.map(Number) : [Number(target), Number(target)];
          return num >= (min ?? 0) && num <= (max ?? 0);
        }

        case "in": {
          const arr = Array.isArray(target) ? target : [target];
          return arr.some((t: any) => String(t).toLowerCase() === String(val).toLowerCase());
        }
        case "notin":
        case "not_in": {
          const arr = Array.isArray(target) ? target : [target];
          return !arr.some((t: any) => String(t).toLowerCase() === String(val).toLowerCase());
        }

        // Date Operators
        case "date_is_today":
          return val ? isSameDay(new Date(val), new Date()) : false;
        case "date_in_last": {
          if (!val) return false;
          const days = Number(target) || 0;
          return new Date(val) >= subDays(new Date(), days) && new Date(val) <= new Date();
        }
        case "date_is_this_week": {
          if (!val) return false;
          const d = new Date(val);
          return isAfter(d, startOfWeek(new Date())) && isBefore(d, endOfWeek(new Date()));
        }
        case "date_is_this_month": {
          if (!val) return false;
          const d = new Date(val);
          return isAfter(d, startOfMonth(new Date())) && isBefore(d, endOfMonth(new Date()));
        }
        case "date_before":
          return val && target ? isBefore(new Date(val), new Date(String(target))) : false;
        case "date_after":
          return val && target ? isAfter(new Date(val), new Date(String(target))) : false;
        case "date_between":
          return val && Array.isArray(target) && target.length >= 2
            ? (new Date(val) >= new Date(String(target[0])) && new Date(val) <= new Date(String(target[1])))
            : false;

        case "bool_true":
          return val === true || val === 1 || val === "true";
        case "bool_false":
          return val === false || val === 0 || val === "false";
        
        default:
          return true;
      }
    };

    const logic = filter.logic === "OR" ? "OR" : "AND";

    if (logic === "AND") {
      return filter.conditions.every((cond: FilterCondition) => evaluateCondition(cond, item));
    } else {
      return filter.conditions.some((cond: FilterCondition) => evaluateCondition(cond, item));
    }
  });
}
