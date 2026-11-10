import { useColorScheme } from "@/hooks/use-color-scheme";

export function useTheme() {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const useColor = (_variable: string, light: string, dark: string) =>
    isDark ? dark : light;

  return {
    primary: useColor("--primary", "#7471B3", "#9896DA"),
    overlay: useColor("--overlay", "rgb(0 0 0 / 10%)", "rgb(0 0 0 / 10%)"),
    primaryForeground: useColor(
      "--primary-foreground",
      "#FFFFFF",
      "#FFFFFF",
    ),
    primaryLight: useColor(
      "--primary-light",
      "#928FD4",
      "#B0AFF5",
    ),
    primaryDark: useColor(
      "--primary-dark",
      "#534F8D",
      "#7471B3",
    ),

    brandPurple: useColor(
      "--brand-purple",
      "#7471B3",
      "#9896DA",
    ),
    brandPurpleLight: useColor(
      "--brand-purple-light",
      "#928FD4",
      "#B0AFF5",
    ),
    brandPurpleDark: useColor(
      "--brand-purple-dark",
      "#534F8D",
      "#7471B3",
    ),
    brandLime: useColor(
      "--brand-lime",
      "#8EBC8F",
      "#98C598",
    ),
    brandPink: useColor(
      "--brand-pink",
      "#D11F96",
      "#D11F96",
    ),
    brandIndigo: useColor(
      "--brand-indigo",
      "#5B84AE",
      "#7EA9D5",
    ),
    brandRose: useColor(
      "--brand-rose",
      "#E0615C",
      "#EA6A64",
    ),

    background: useColor("--background", "#FCFCFC", "#111111"),
    foreground: useColor("--foreground", "#1D1D1D", "#E4E4E4"),
    backgroundElement: useColor("--background-element", "#F2F2F2", "#292929"),
    backgroundSelected: useColor("--background-selected", "#EBEBEB", "#333333"),

    card: useColor("--card", "#FFFFFF", "#292929"),
    cardForeground: useColor(
      "--card-foreground",
      "#1D1D1D",
      "#E4E4E4",
    ),
    cardElevated: useColor("--card-elevated", "#FFFFFF", "#333333"),
    cardBorder: useColor("--card-border", "#E4E4E4", "#3D3D3D"),

    popover: useColor("--popover", "#FFFFFF", "#2E2E2E"),
    popoverForeground: useColor(
      "--popover-foreground",
      "#1D1D1D",
      "#E4E4E4",
    ),
    secondary: useColor("--secondary", "#F5F5F5", "#333333"),
    secondaryForeground: useColor(
      "--secondary-foreground",
      "#1D1D1D",
      "#E4E4E4",
    ),
    muted: useColor("--muted", "#F2F2F2", "#2E2E2E"),
    mutedForeground: useColor(
      "--muted-foreground",
      "#696969",
      "#9E9E9E",
    ),
    accent: useColor("--accent", "#F3F3F3", "#383838"),
    accentForeground: useColor(
      "--accent-foreground",
      "#1D1D1D",
      "#E4E4E4",
    ),
    accentLight: useColor("--primary-light", "#928FD4", "#B0AFF5"),

    text: useColor("--foreground", "#1D1D1D", "#E4E4E4"),
    textSecondary: useColor(
      "--muted-foreground",
      "#696969",
      "#9E9E9E",
    ),
    textMuted: useColor(
      "--muted-foreground",
      "#696969",
      "#9E9E9E",
    ),

    border: useColor("--border", "#DEDEDE", "#424242"),
    borderSubtle: useColor("--border-subtle", "#E8E8E8", "#383838"),
    input: useColor("--input", "#EBEBEB", "#383838"),
    ring: useColor("--ring", "#7471B3", "#9896DA"),

    destructive: useColor(
      "--destructive",
      "#DE4E4B",
      "#EC5B57",
    ),
    destructiveForeground: useColor(
      "--destructive-foreground",
      "#FFFFFF",
      "#FFFFFF",
    ),
    success: useColor("--success", "#009A00", "#39C34B"),
    successLight: useColor("--success-light", "#D3F7D3", "#0D3811"),
    warning: useColor("--warning", "#CD8800", "#E4AC59"),
    warningLight: useColor("--warning-light", "#FBE8CE", "#3F301B"),
    error: useColor("--error", "#DE4E4B", "#EC5B57"),
    errorLight: useColor("--error-light", "#FFDCD7", "#4D2623"),
    info: useColor("--info", "#5B84AE", "#7EA9D5"),
    infoLight: useColor("--info-light", "#D8EEFF", "#1E2F41"),

    btnShadowPrimary: useColor("--btn-shadow-primary", "#32326D", "#26245E"),
    btnShadowSecondary: useColor(
      "--btn-shadow-secondary",
      "#AEAEAE",
      "#3A3A3A",
    ),
    btnShadowDestructive: useColor(
      "--btn-shadow-destructive",
      "#A10000",
      "#930000",
    ),
    btnShadowLime: useColor("--btn-shadow-lime", "#829D00", "#586F00"),

    tabBarBackground: useColor(
      "--tab-bar-background",
      "#FFFFFF",
      "#292929",
    ),
    tabBarBorder: useColor(
      "--tab-bar-border",
      "#DEDEDE",
      "#424242",
    ),
    tabBarActive: useColor(
      "--tab-bar-active",
      "#7471B3",
      "#9896DA",
    ),
    tabBarInactive: useColor(
      "--tab-bar-inactive",
      "#696969",
      "#9E9E9E",
    ),
    brandLimeForeground: useColor(
      "--brand-lime-foreground",
      "#0B290E",
      "#91E694",
    ),
  };
}
