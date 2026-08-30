import type { UniversalFilter, FilterCondition } from "./types";
import { format, isAfter, isBefore, isSameDay, subDays, addDays, startOfWeek, endOfWeek, startOfMonth, endOfMonth } from "date-fns";

export function applyFilter<T extends Record<string, any>>(items: T[], filter: UniversalFilter | undefined | null): T[] {
  if (!filter) return items;

  return items.filter((item) => {
    // 1. Search term (applies across all string fields heuristically, or standard search behavior if needed)
    // Note: Most pages implement search separately, but if search is in UniversalFilter:
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

      switch (cond.operator) {
        case "eq": return val === target;
        case "neq": return val !== target;
        case "gt": return val > target;
        case "gte": return val >= target;
        case "lt": return val < target;
        case "lte": return val <= target;
        case "contains": return String(val).toLowerCase().includes(String(target).toLowerCase());
        case "not_contains": return !String(val).toLowerCase().includes(String(target).toLowerCase());
        case "starts_with": return String(val).toLowerCase().startsWith(String(target).toLowerCase());
        case "ends_with": return String(val).toLowerCase().endsWith(String(target).toLowerCase());
        case "in": return Array.isArray(target) && target.includes(val);
        case "not_in": return Array.isArray(target) && !target.includes(val);
        case "is_null": return val === null || val === undefined;
        case "is_not_null": return val !== null && val !== undefined;
        case "between": return Array.isArray(target) && val >= target[0] && val <= target[1];
        
        // Date operators
        case "date_before": return val ? isBefore(new Date(val), new Date(target as string)) : false;
        case "date_after": return val ? isAfter(new Date(val), new Date(target as string)) : false;
        case "date_between": return val && Array.isArray(target) ? (new Date(val) >= new Date(target[0]) && new Date(val) <= new Date(target[1])) : false;
        case "date_is_today": return val ? isSameDay(new Date(val), new Date()) : false;
        case "date_in_last": {
          if (!val) return false;
          const days = Number(target) || 0;
          return new Date(val) >= subDays(new Date(), days) && new Date(val) <= new Date();
        }
        case "date_is_this_week": {
          if (!val) return false;
          const now = new Date();
          return new Date(val) >= startOfWeek(now) && new Date(val) <= endOfWeek(now);
        }
        case "date_is_this_month": {
          if (!val) return false;
          const now = new Date();
          return new Date(val) >= startOfMonth(now) && new Date(val) <= endOfMonth(now);
        }
        
        // Boolean operators
        case "bool_true": return val === true || val === 1 || val === "true";
        case "bool_false": return val === false || val === 0 || val === "false";
        
        default: return true;
      }
    };

    const logic = filter.logic === "OR" ? "OR" : "AND";

    if (logic === "AND") {
      return filter.conditions.every((cond) => evaluateCondition(cond, item));
    } else {
      return filter.conditions.some((cond) => evaluateCondition(cond, item));
    }
  });
}
