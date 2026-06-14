
import {
  type AllTableSettings,
  mergeWithDefaults,
  TABLE_SETTINGS_COOKIE,
  type TableId,
  type TableSettings,
} from "./table-settings";

/**
 * Get initial table settings from the unified cookie
 * Falls back to defaults if cookie doesn't exist or is invalid
 */
function getCookie(name: string): string | null {
  if (typeof document === "undefined") return null;
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop()?.split(";").shift() || null;
  return null;
}

/**
 * Get initial table settings from the unified cookie
 * Falls back to defaults if cookie doesn't exist or is invalid
 */
export async function getInitialTableSettings(
  tableId: TableId,
): Promise<TableSettings> {
  const saved = getCookie(TABLE_SETTINGS_COOKIE);

  if (!saved) {
    return mergeWithDefaults(undefined, tableId);
  }

  try {
    const allSettings: AllTableSettings = JSON.parse(decodeURIComponent(saved));
    return mergeWithDefaults(allSettings[tableId], tableId);
  } catch {
    // Invalid JSON, return defaults
    return mergeWithDefaults(undefined, tableId);
  }
}
