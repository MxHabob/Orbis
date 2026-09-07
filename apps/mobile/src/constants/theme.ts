/**
 * @module @orbis/mobile/theme
 *
 * S.C.R.A.P.S. Design System Tokens for Orbis Mobile.
 * Synchronized with @orbis/ui (packages/ui/globals.css).
 */

import '@/global.css';
import { Platform } from 'react-native';

export const Colors = {
  light: {
    // Brand & Primary
    primary: '#7C3AED', // oklch(0.58 0.1 285)
    primaryForeground: '#FFFFFF',
    primaryLight: '#F5F3FF',
    primaryDark: '#6D28D9',

    // Brand Palette
    brandPurple: '#7C3AED',
    brandPurpleLight: '#8B5CF6',
    brandPurpleDark: '#6D28D9',
    brandLime: '#84CC16', // oklch(0.75 0.08 145)
    brandLimeDark: '#65A30D',
    brandPink: '#EC4899', // oklch(58.18% 0.23022 346.346)
    brandPinkLight: '#F472B6',
    brandIndigo: '#6366F1', // oklch(0.6 0.08 250)
    brandRose: '#F43F5E', // oklch(0.65 0.16 25)

    // Base & Surfaces
    background: '#FAFAFA', // oklch(0.99 0 0)
    foreground: '#18181B', // oklch(0.23 0 0)
    backgroundElement: '#F4F4F5',
    backgroundSelected: '#E4E4E7',

    card: '#FFFFFF', // oklch(1 0 0)
    cardForeground: '#18181B',
    cardElevated: '#FFFFFF',
    cardBorder: '#E4E4E7',

    // Neutrals
    secondary: '#F4F4F5', // oklch(0.97 0 0)
    secondaryForeground: '#18181B',
    muted: '#F4F4F5', // oklch(0.96 0 0)
    mutedForeground: '#71717A', // oklch(0.52 0 0)
    accent: '#F4F4F5', // oklch(0.965 0 0)
    accentForeground: '#18181B',
    accentLight: '#F5F3FF',

    text: '#18181B',
    textSecondary: '#71717A',
    textMuted: '#A1A1AA',

    // Inputs & Borders
    border: '#E4E4E7', // oklch(0.9 0 0)
    borderSubtle: '#F4F4F5',
    input: '#F4F4F5', // oklch(0.94 0 0)
    ring: '#7C3AED', // oklch(0.58 0.1 285)

    // States & Alerts
    destructive: '#EF4444', // oklch(0.62 0.18 25)
    destructiveForeground: '#FFFFFF',
    success: '#10B981',
    successLight: '#ECFDF5',
    warning: '#F59E0B',
    warningLight: '#FFFBEB',
    error: '#EF4444',
    errorLight: '#FEF2F2',
    info: '#06B6D4',
    infoLight: '#ECFEFF',

    // Tab Bar
    tabBarBackground: '#FFFFFFFA',
    tabBarBorder: '#E4E4E7',
    tabBarActive: '#7C3AED',
    tabBarInactive: '#71717A',
  },
  dark: {
    // Brand & Primary
    primary: '#A78BFA', // oklch(0.7 0.1 285)
    primaryForeground: '#FFFFFF',
    primaryLight: '#2E1065',
    primaryDark: '#C4B5FD',

    // Brand Palette
    brandPurple: '#A78BFA',
    brandPurpleLight: '#C4B5FD',
    brandPurpleDark: '#7C3AED',
    brandLime: '#A3E635', // oklch(0.78 0.08 145)
    brandLimeDark: '#65A30D',
    brandPink: '#F472B6',
    brandPinkLight: '#FB7185',
    brandIndigo: '#818CF8', // oklch(0.72 0.08 250)
    brandRose: '#FB7185', // oklch(0.68 0.16 25)

    // Base & Surfaces
    background: '#121214', // oklch(0.24 0 0)
    foreground: '#F4F4F5', // oklch(0.92 0 0)
    backgroundElement: '#1E1E22',
    backgroundSelected: '#27272A',

    card: '#18181B', // oklch(0.28 0 0)
    cardForeground: '#F4F4F5',
    cardElevated: '#202024',
    cardBorder: '#27272A',

    // Neutrals
    secondary: '#27272A', // oklch(0.32 0 0)
    secondaryForeground: '#F4F4F5',
    muted: '#27272A', // oklch(0.3 0 0)
    mutedForeground: '#A1A1AA', // oklch(0.7 0 0)
    accent: '#3F3F46', // oklch(0.34 0 0)
    accentForeground: '#F4F4F5',
    accentLight: '#2E1065',

    text: '#F4F4F5',
    textSecondary: '#A1A1AA',
    textMuted: '#71717A',

    // Inputs & Borders
    border: '#27272A', // oklch(0.38 0 0)
    borderSubtle: '#1E1E22',
    input: '#27272A', // oklch(0.34 0 0)
    ring: '#A78BFA', // oklch(0.7 0.1 285)

    // States & Alerts
    destructive: '#F87171', // oklch(0.66 0.18 25)
    destructiveForeground: '#FFFFFF',
    success: '#34D399',
    successLight: '#064E3B',
    warning: '#FBBF24',
    warningLight: '#78350F',
    error: '#F87171',
    errorLight: '#7F1D1D',
    info: '#22D3EE',
    infoLight: '#164E63',

    // Tab Bar
    tabBarBackground: '#121214FA',
    tabBarBorder: '#27272A',
    tabBarActive: '#A78BFA',
    tabBarInactive: '#71717A',
  },
} as const;

export type ThemeColors = typeof Colors.light;
export type ThemeColor = keyof ThemeColors;

export const Fonts = Platform.select({
  ios: {
    sans: 'Rubik',
    serif: 'ui-serif',
    rounded: 'ui-rounded',
    mono: 'JetBrains Mono',
  },
  default: {
    sans: 'Rubik',
    serif: 'serif',
    rounded: 'normal',
    mono: 'JetBrains Mono',
  },
  web: {
    sans: 'var(--font-sans, "Rubik", system-ui, sans-serif)',
    serif: 'var(--font-serif)',
    rounded: 'var(--font-rounded)',
    mono: 'var(--font-mono, "JetBrains Mono", monospace)',
  },
});

export const Spacing = {
  half: 2,
  one: 4,
  two: 8,
  three: 12,
  four: 16,
  five: 24,
  six: 32,
  seven: 40,
  eight: 48,
} as const;

export const Radii = {
  sm: 4, // --radius-sm: calc(0.5rem - 4px)
  md: 6, // --radius-md: calc(0.5rem - 2px)
  lg: 8, // --radius-lg: 0.5rem (8px)
  xl: 12, // --radius-xl: calc(0.5rem + 4px)
  '2xl': 16, // --radius-2xl: calc(0.5rem + 8px)
  '3xl': 20, // --radius-3xl: calc(0.5rem + 12px)
  full: 9999,
} as const;

export const Typography = {
  h1: { fontSize: 26, fontWeight: '700' as const, lineHeight: 32, fontFamily: Fonts?.sans },
  h2: { fontSize: 20, fontWeight: '700' as const, lineHeight: 26, fontFamily: Fonts?.sans },
  h3: { fontSize: 17, fontWeight: '600' as const, lineHeight: 22, fontFamily: Fonts?.sans },
  body: { fontSize: 14, fontWeight: '400' as const, lineHeight: 20, fontFamily: Fonts?.sans },
  bodyBold: { fontSize: 14, fontWeight: '600' as const, lineHeight: 20, fontFamily: Fonts?.sans },
  caption: { fontSize: 12, fontWeight: '400' as const, lineHeight: 16, fontFamily: Fonts?.sans },
  captionBold: { fontSize: 12, fontWeight: '600' as const, lineHeight: 16, fontFamily: Fonts?.sans },
  small: { fontSize: 11, fontWeight: '500' as const, lineHeight: 14, fontFamily: Fonts?.sans },
} as const;

export const BottomTabInset = Platform.select({ ios: 64, android: 72 }) ?? 64;
export const MaxContentWidth = 800;
