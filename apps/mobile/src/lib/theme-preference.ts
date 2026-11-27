import { colorScheme } from "nativewind";
import { Appearance, Platform } from "react-native";
import {
  deleteSecureItem,
  getSecureItem,
  setSecureItem,
} from "@/lib/api/storage";

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
  colorScheme.set(preference);
  if (
    Platform.OS !== "web" &&
    typeof Appearance.setColorScheme === "function"
  ) {
    Appearance.setColorScheme(
      preference === "system" ? "unspecified" : preference,
    );
  }
}

export async function saveThemePreference(
  preference: ThemePreference,
): Promise<void> {
  if (preference === "system") {
    await deleteSecureItem(THEME_STORAGE_KEY);
  } else {
    await setSecureItem(THEME_STORAGE_KEY, preference);
  }
  applyThemePreference(preference);
}
