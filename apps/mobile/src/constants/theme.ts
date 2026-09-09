/**
 * @module @orbis/mobile/theme
 *
 * S.C.R.A.P.S. Design System Tokens for Orbis Mobile.
 * Synchronized 100% with @orbis/ui (packages/ui/globals.css).
 */

import { Platform } from 'react-native';

export const Colors = {
  light: {
    // Brand & Primary — oklch(0.58 0.1 285)
    primary: '#7C3AED',
    primaryForeground: '#FFFFFF',
    primaryLight: '#F5F3FF',
    primaryDark: '#5B21B6',

    // Brand Palette
    brandPurple: '#7C3AED',
    brandPurpleLight: '#8B5CF6',
    brandPurpleDark: '#5B21B6',
    brandLime: '#84CC16', // oklch(0.75 0.08 145)
    brandLimeDark: '#4D7C0F',
    brandPink: '#EC4899', // oklch(58.18% 0.23022 346.346)
    brandPinkLight: '#F472B6',
    brandIndigo: '#6366F1', // oklch(0.6 0.08 250)
    brandRose: '#F43F5E', // oklch(0.65 0.16 25)

    // Base & Surfaces — oklch(0.99 0 0)
    background: '#FAFAFA',
    foreground: '#18181B', // oklch(0.23 0 0)
    backgroundElement: '#F4F4F5',
    backgroundSelected: '#E4E4E7',

    // Cards — oklch(1 0 0)
    card: '#FFFFFF',
    cardForeground: '#18181B',
    cardElevated: '#FFFFFF',
    cardBorder: '#E4E4E7',

    // Popover
    popover: '#FFFFFF',
    popoverForeground: '#18181B',

    // Neutrals — oklch(0.97 0 0)
    secondary: '#F4F4F5',
    secondaryForeground: '#18181B',
    muted: '#F4F4F5', // oklch(0.96 0 0)
    mutedForeground: '#71717A', // oklch(0.52 0 0)
    accent: '#F4F4F5', // oklch(0.965 0 0)
    accentForeground: '#18181B',
    accentLight: '#F5F3FF',

    text: '#18181B',
    textSecondary: '#71717A',
    textMuted: '#A1A1AA',

    // Inputs & Borders — oklch(0.9 0 0)
    border: '#E4E4E7',
    borderSubtle: '#F4F4F5',
    input: '#F4F4F5', // oklch(0.94 0 0)
    ring: '#7C3AED', // oklch(0.58 0.1 285)

    // States & Alerts — oklch(0.62 0.18 25)
    destructive: '#DC2626',
    destructiveForeground: '#FFFFFF',
    success: '#10B981',
    successLight: '#ECFDF5',
    warning: '#F59E0B',
    warningLight: '#FFFBEB',
    error: '#DC2626',
    errorLight: '#FEF2F2',
    info: '#06B6D4',
    infoLight: '#ECFEFF',

    // Tactile Shadow Colors
    btnShadowPrimary: '#5B21B6',
    btnShadowSecondary: '#D4D4D8',
    btnShadowDestructive: '#991B1B',
    btnShadowLime: '#4D7C0F',

    // Tab Bar
    tabBarBackground: '#FFFFFFF2',
    tabBarBorder: '#E4E4E7',
    tabBarActive: '#7C3AED',
    tabBarInactive: '#71717A',
  },
  dark: {
    // Brand & Primary — oklch(0.7 0.1 285)
    primary: '#9061F9',
    primaryForeground: '#FFFFFF',
    primaryLight: '#2E1065',
    primaryDark: '#7C3AED',

    // Brand Palette
    brandPurple: '#9061F9',
    brandPurpleLight: '#A78BFA',
    brandPurpleDark: '#6D28D9',
    brandLime: '#A3E635', // oklch(0.78 0.08 145)
    brandLimeDark: '#4D7C0F',
    brandPink: '#F472B6',
    brandPinkLight: '#FB7185',
    brandIndigo: '#818CF8', // oklch(0.72 0.08 250)
    brandRose: '#FB7185', // oklch(0.68 0.16 25)

    // Base & Surfaces — oklch(0.24 0 0)
    background: '#18181B',
    foreground: '#F4F4F5', // oklch(0.92 0 0)
    backgroundElement: '#27272A',
    backgroundSelected: '#3F3F46',

    // Cards — oklch(0.28 0 0)
    card: '#27272A',
    cardForeground: '#F4F4F5',
    cardElevated: '#27272A',
    cardBorder: '#3F3F46',

    // Popover — oklch(0.3 0 0)
    popover: '#27272A',
    popoverForeground: '#F4F4F5',

    // Neutrals — oklch(0.32 0 0)
    secondary: '#27272A',
    secondaryForeground: '#F4F4F5',
    muted: '#27272A', // oklch(0.3 0 0)
    mutedForeground: '#A1A1AA', // oklch(0.7 0 0)
    accent: '#3F3F46', // oklch(0.34 0 0)
    accentForeground: '#F4F4F5',
    accentLight: '#2E1065',

    text: '#F4F4F5',
    textSecondary: '#A1A1AA',
    textMuted: '#71717A',

    // Inputs & Borders — oklch(0.38 0 0)
    border: '#3F3F46',
    borderSubtle: '#27272A',
    input: '#27272A', // oklch(0.34 0 0)
    ring: '#9061F9', // oklch(0.7 0.1 285)

    // States & Alerts — oklch(0.66 0.18 25)
    destructive: '#EF4444',
    destructiveForeground: '#FFFFFF',
    success: '#34D399',
    successLight: '#064E3B',
    warning: '#FBBF24',
    warningLight: '#78350F',
    error: '#EF4444',
    errorLight: '#7F1D1D',
    info: '#22D3EE',
    infoLight: '#164E63',

    // Tactile Shadow Colors
    btnShadowPrimary: '#581C87',
    btnShadowSecondary: '#18181B',
    btnShadowDestructive: '#7F1D1D',
    btnShadowLime: '#365314',

    // Tab Bar
    tabBarBackground: '#18181BF2',
    tabBarBorder: '#27272A',
    tabBarActive: '#9061F9',
    tabBarInactive: '#71717A',
  },
} as const;

export type ThemeColors = typeof Colors.light;
export type ThemeColor = keyof ThemeColors;

export const Spacing = {
  half: 2,
  one: 4,
  two: 8,
  three: 12,
  four: 16,
  five: 20,
  six: 24,
  eight: 32,
  ten: 40,
  twelve: 48,
  sixteen: 64,
} as const;

export const Radii = {
  none: 0,
  xs: 2,
  sm: 4,
  md: 6,
  lg: 8,
  xl: 12,
  '2xl': 16,
  '3xl': 20,
  full: 9999,
} as const;

export const Typography = {
  h1: {
    fontSize: 24,
    lineHeight: 30,
    fontWeight: '800' as const,
  },
  h2: {
    fontSize: 18,
    lineHeight: 24,
    fontWeight: '700' as const,
  },
  h3: {
    fontSize: 16,
    lineHeight: 22,
    fontWeight: '600' as const,
  },
  body: {
    fontSize: 14,
    lineHeight: 20,
    fontWeight: '400' as const,
  },
  bodyBold: {
    fontSize: 14,
    lineHeight: 20,
    fontWeight: '600' as const,
  },
  caption: {
    fontSize: 12,
    lineHeight: 16,
    fontWeight: '400' as const,
  },
  captionBold: {
    fontSize: 12,
    lineHeight: 16,
    fontWeight: '600' as const,
  },
  small: {
    fontSize: 11,
    lineHeight: 14,
    fontWeight: '500' as const,
  },
} as const;

export const Fonts = Platform.select({
  ios: {
    sans: 'System',
    mono: 'Courier',
  },
  android: {
    sans: 'sans-serif',
    mono: 'monospace',
  },
  default: {
    sans: 'Rubik, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',
    mono: 'JetBrains Mono, monospace',
  },
});


