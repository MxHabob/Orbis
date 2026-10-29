import { Appearance } from "react-native";
import { getSecureItem, setSecureItem } from "@/lib/api/storage";

const THEME_STORAGE_KEY = "orbis_app_theme";

export type ThemePreference = "system" | "light" | "dark";

export function isThemePreference(
  value: string | null,
): value is ThemePreference {
  return value === "system" || value === "light" || value === "dark";
}

export async function loadThemePreference(): Promise<ThemePreference> {
  const storedValue = await getSecureItem(THEME_STORAGE_KEY);
  return isThemePreference(storedValue) ? storedValue : "system";
}

export function applyThemePreference(preference: ThemePreference) {
  Appearance.setColorScheme(
    preference === "system" ? "unspecified" : preference,
  );
}

export async function saveThemePreference(
  preference: ThemePreference,
): Promise<void> {
  await setSecureItem(THEME_STORAGE_KEY, preference);
  applyThemePreference(preference);
}
