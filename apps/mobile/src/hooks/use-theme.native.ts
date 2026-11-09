import { useColorScheme } from "@/hooks/use-color-scheme";
import { useNativeVariable } from "react-native-css/native";

function useColorToken(
  variable: string,
  light: string,
  dark: string,
  isDark: boolean,
): string {
  const value = useNativeVariable(variable);
  return typeof value === "string" ? value : isDark ? dark : light;
}

export function useTheme() {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const useColor = (variable: string, light: string, dark: string) =>
    useColorToken(variable, light, dark, isDark);

  return {
    primary: useColor("--primary", "oklch(0.58 0.1 285)", "oklch(0.7 0.1 285)"),
    primaryForeground: useColor(
      "--primary-foreground",
      "oklch(1 0 0)",
      "oklch(1 0 0)",
    ),
    primaryLight: useColor(
      "--primary-light",
      "oklch(0.965 0 0)",
      "oklch(0.34 0 0)",
    ),
    primaryDark: useColor(
      "--primary-dark",
      "oklch(0.46 0.1 285)",
      "oklch(0.58 0.1 285)",
    ),

    brandPurple: useColor(
      "--brand-purple",
      "oklch(0.58 0.1 285)",
      "oklch(0.7 0.1 285)",
    ),
    brandPurpleLight: useColor(
      "--brand-purple-light",
      "oklch(0.68 0.1 285)",
      "oklch(0.78 0.1 285)",
    ),
    brandPurpleDark: useColor(
      "--brand-purple-dark",
      "oklch(0.46 0.1 285)",
      "oklch(0.58 0.1 285)",
    ),
    brandLime: useColor(
      "--brand-lime",
      "oklch(0.75 0.08 145)",
      "oklch(0.78 0.08 145)",
    ),
    brandPink: useColor(
      "--brand-pink",
      "oklch(58.18% 0.23022 346.346)",
      "oklch(58.18% 0.23022 346.346)",
    ),
    brandIndigo: useColor(
      "--brand-indigo",
      "oklch(0.6 0.08 250)",
      "oklch(0.72 0.08 250)",
    ),
    brandRose: useColor(
      "--brand-rose",
      "oklch(0.65 0.16 25)",
      "oklch(0.68 0.16 25)",
    ),

    background: useColor("--background", "oklch(0.99 0 0)", "oklch(0.24 0 0)"),
    foreground: useColor("--foreground", "oklch(0.23 0 0)", "oklch(0.92 0 0)"),
    backgroundElement: useColor(
      "--background-element",
      "oklch(0.97 0 0)",
      "oklch(0.32 0 0)",
    ),
    backgroundSelected: useColor(
      "--background-selected",
      "oklch(0.965 0 0)",
      "oklch(0.34 0 0)",
    ),

    card: useColor("--card", "oklch(1 0 0)", "oklch(0.28 0 0)"),
    cardForeground: useColor(
      "--card-foreground",
      "oklch(0.23 0 0)",
      "oklch(0.92 0 0)",
    ),
    cardElevated: useColor(
      "--card-elevated",
      "oklch(1 0 0)",
      "oklch(0.28 0 0)",
    ),
    cardBorder: useColor("--card-border", "oklch(0.9 0 0)", "oklch(0.38 0 0)"),

    popover: useColor("--popover", "oklch(1 0 0)", "oklch(0.3 0 0)"),
    popoverForeground: useColor(
      "--popover-foreground",
      "oklch(0.23 0 0)",
      "oklch(0.92 0 0)",
    ),
    secondary: useColor("--secondary", "oklch(0.97 0 0)", "oklch(0.32 0 0)"),
    secondaryForeground: useColor(
      "--secondary-foreground",
      "oklch(0.23 0 0)",
      "oklch(0.92 0 0)",
    ),
    muted: useColor("--muted", "oklch(0.96 0 0)", "oklch(0.3 0 0)"),
    mutedForeground: useColor(
      "--muted-foreground",
      "oklch(0.52 0 0)",
      "oklch(0.7 0 0)",
    ),
    accent: useColor("--accent", "oklch(0.965 0 0)", "oklch(0.34 0 0)"),
    accentForeground: useColor(
      "--accent-foreground",
      "oklch(0.23 0 0)",
      "oklch(0.92 0 0)",
    ),
    accentLight: useColor(
      "--primary-light",
      "oklch(0.965 0 0)",
      "oklch(0.34 0 0)",
    ),

    text: useColor("--foreground", "oklch(0.23 0 0)", "oklch(0.92 0 0)"),
    textSecondary: useColor(
      "--muted-foreground",
      "oklch(0.52 0 0)",
      "oklch(0.7 0 0)",
    ),
    textMuted: useColor(
      "--muted-foreground",
      "oklch(0.52 0 0)",
      "oklch(0.7 0 0)",
    ),

    border: useColor("--border", "oklch(0.9 0 0)", "oklch(0.38 0 0)"),
    borderSubtle: useColor(
      "--border-subtle",
      "oklch(0.97 0 0)",
      "oklch(0.32 0 0)",
    ),
    input: useColor("--input", "oklch(0.94 0 0)", "oklch(0.34 0 0)"),
    ring: useColor("--ring", "oklch(0.58 0.1 285)", "oklch(0.7 0.1 285)"),

    destructive: useColor(
      "--destructive",
      "oklch(0.62 0.18 25)",
      "oklch(0.66 0.18 25)",
    ),
    destructiveForeground: useColor(
      "--destructive-foreground",
      "oklch(1 0 0)",
      "oklch(1 0 0)",
    ),
    success: useColor(
      "--success",
      "oklch(0.74 0.08 145)",
      "oklch(0.76 0.08 145)",
    ),
    successLight: useColor(
      "--success-light",
      "oklch(0.97 0 0)",
      "oklch(0.32 0 0)",
    ),
    warning: useColor("--warning", "oklch(0.82 0.1 85)", "oklch(0.84 0.1 85)"),
    warningLight: useColor(
      "--warning-light",
      "oklch(0.97 0 0)",
      "oklch(0.32 0 0)",
    ),
    error: useColor("--error", "oklch(0.62 0.18 25)", "oklch(0.66 0.18 25)"),
    errorLight: useColor("--error-light", "oklch(0.97 0 0)", "oklch(0.32 0 0)"),
    info: useColor("--info", "oklch(0.68 0.08 250)", "oklch(0.72 0.08 250)"),
    infoLight: useColor("--info-light", "oklch(0.97 0 0)", "oklch(0.32 0 0)"),

    btnShadowPrimary: useColor(
      "--btn-shadow-primary",
      "oklch(0.46 0.1 285)",
      "oklch(0.58 0.1 285)",
    ),
    btnShadowSecondary: useColor(
      "--btn-shadow-secondary",
      "oklch(0.9 0 0)",
      "oklch(0.38 0 0)",
    ),
    btnShadowDestructive: useColor(
      "--btn-shadow-destructive",
      "oklch(0.62 0.18 25)",
      "oklch(0.66 0.18 25)",
    ),
    btnShadowLime: useColor(
      "--btn-shadow-lime",
      "oklch(0.62 0.08 145)",
      "oklch(0.66 0.08 145)",
    ),

    tabBarBackground: useColor(
      "--tab-bar-background",
      "oklch(1 0 0)",
      "oklch(0.28 0 0)",
    ),
    tabBarBorder: useColor(
      "--tab-bar-border",
      "oklch(0.9 0 0)",
      "oklch(0.38 0 0)",
    ),
    tabBarActive: useColor(
      "--tab-bar-active",
      "oklch(0.58 0.1 285)",
      "oklch(0.7 0.1 285)",
    ),
    tabBarInactive: useColor(
      "--tab-bar-inactive",
      "oklch(0.52 0 0)",
      "oklch(0.7 0 0)",
    ),
  };
}
