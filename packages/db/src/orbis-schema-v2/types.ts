import {
  pgSchema,
  customType,
} from "drizzle-orm/pg-core";

// ============================================================
// CUSTOM TYPES
// ============================================================

export const moneyType = customType<{
  data: number;
  driverData: number;
  config: { precision?: number };
}>({
  dataType(config) {
    return `integer${config?.precision ? ` CHECK (VALUE >= 0 AND VALUE < ${Math.pow(10, config.precision + 2)})` : ""}`;
  },
});

export const scoreType = customType<{
  data: number;
  driverData: number;
}>({
  dataType() {
    return "smallint CHECK (VALUE >= 0 AND VALUE <= 1000)";
  },
});

export const percentType = customType<{
  data: number;
  driverData: number;
}>({
  dataType() {
    return "smallint CHECK (VALUE >= 0 AND VALUE <= 100)";
  },
});

export const hijriDateType = customType<{
  data: string;
  driverData: string;
}>({
  dataType() {
    return "varchar(10)";
  },
});

// ============================================================
// SCHEMA DECLARATION
// ============================================================

export const coreSchema = pgSchema("core");
export const academicSchema = pgSchema("academic");
export const financeSchema = pgSchema("finance");
export const auditSchema = pgSchema("audit");
