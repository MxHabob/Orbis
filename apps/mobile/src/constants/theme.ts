/**
 * @module @orbis/mobile/theme
 *
 * S.C.R.A.P.S. Design System Tokens for Orbis Mobile.
 */

import { Platform } from 'react-native';


export const Colors = {
  // ───────────────────────────────────────────────────────────────────────
  // LIGHT
  // ───────────────────────────────────────────────────────────────────────

  light: {
    primary: '#7C3AED',
    primaryForeground: '#FFFFFF',
    primaryLight: '#F5F3FF',
    primaryDark: '#5B21B6',

    brandPurple: '#7C3AED',
    brandPurpleLight: '#8B5CF6',
    brandPurpleDark: '#5B21B6',

    brandLime: '#84CC16',
    brandLimeDark: '#4D7C0F',

    brandPink: '#EC4899',
    brandPinkLight: '#F472B6',

    brandIndigo: '#6366F1',

    brandRose: '#F43F5E',


    // ─────────────────────────────────────────────────────────────────────
    // Base
    // ─────────────────────────────────────────────────────────────────────

    background: '#FAFAFA',
    foreground: '#18181B',

    backgroundElement: '#F4F4F5',
    backgroundSelected: '#E4E4E7',


    // ─────────────────────────────────────────────────────────────────────
    // Cards
    // ─────────────────────────────────────────────────────────────────────

    card: '#FFFFFF',
    cardForeground: '#18181B',

    cardElevated: '#FFFFFF',
    cardBorder: '#E4E4E7',


    // ─────────────────────────────────────────────────────────────────────
    // Popovers
    // ─────────────────────────────────────────────────────────────────────

    popover: '#FFFFFF',
    popoverForeground: '#18181B',


    // ─────────────────────────────────────────────────────────────────────
    // Semantic surfaces
    // ─────────────────────────────────────────────────────────────────────

    secondary: '#F4F4F5',
    secondaryForeground: '#18181B',

    muted: '#F4F4F5',
    mutedForeground: '#71717A',

    accent: '#F4F4F5',
    accentForeground: '#18181B',
    accentLight: '#F5F3FF',


    // ─────────────────────────────────────────────────────────────────────
    // Text
    // ─────────────────────────────────────────────────────────────────────

    text: '#18181B',
    textSecondary: '#71717A',
    textMuted: '#A1A1AA',


    // ─────────────────────────────────────────────────────────────────────
    // Borders / Inputs
    // ─────────────────────────────────────────────────────────────────────

    border: '#E4E4E7',
    borderSubtle: '#F4F4F5',

    input: '#F4F4F5',

    ring: '#7C3AED',


    // ─────────────────────────────────────────────────────────────────────
    // Destructive
    // ─────────────────────────────────────────────────────────────────────

    destructive: '#DC2626',
    destructiveForeground: '#FFFFFF',


    // ─────────────────────────────────────────────────────────────────────
    // Success
    // ─────────────────────────────────────────────────────────────────────

    success: '#10B981',
    successLight: '#ECFDF5',


    // ─────────────────────────────────────────────────────────────────────
    // Warning
    // ─────────────────────────────────────────────────────────────────────

    warning: '#F59E0B',
    warningLight: '#FFFBEB',


    // ─────────────────────────────────────────────────────────────────────
    // Error
    // ─────────────────────────────────────────────────────────────────────

    error: '#DC2626',
    errorLight: '#FEF2F2',


    // ─────────────────────────────────────────────────────────────────────
    // Info
    // ─────────────────────────────────────────────────────────────────────

    info: '#06B6D4',
    infoLight: '#ECFEFF',


    // ─────────────────────────────────────────────────────────────────────
    // Button shadows
    // ─────────────────────────────────────────────────────────────────────

    btnShadowPrimary: '#5B21B6',
    btnShadowSecondary: '#D4D4D8',
    btnShadowDestructive: '#991B1B',
    btnShadowLime: '#4D7C0F',


    // ─────────────────────────────────────────────────────────────────────
    // Tab Bar
    // ─────────────────────────────────────────────────────────────────────

    tabBarBackground: '#FFFFFFF2',
    tabBarBorder: '#E4E4E7',
    tabBarActive: '#7C3AED',
    tabBarInactive: '#71717A',


    // ─────────────────────────────────────────────────────────────────────
    // Gradient Surface
    //
    // Direction:
    //
    //   FAFAFA
    //      ↓
    //   FAFAFA
    //      ↓
    //   F8F4FF
    //      ↓
    //   EDE3FF
    //      ↓
    //   D8B4FE
    //      ↓
    //   A855F7
    //      ↓
    //   7C3AED
    //
    // The top remains visually clean for a long distance.
    // The purple becomes progressively stronger toward the bottom.
    // ─────────────────────────────────────────────────────────────────────

gradientHeader: {
  colors: [
    '#FAFAFA',
    '#FAFAFA',
    '#FAFAFA',
    '#F8F4FF',
    '#EDE3FF',
    '#D8B4FE',
    '#A855F7',
    '#7C3AED',
  ] as const,

  blobPrimary: '#A855F7',

  blobSecondary: '#C084FC',

  blobTertiary: '#7C3AED',

  start: {
    x: 0.5,
    y: 0,
  },

  end: {
    x: 0.5,
    y: 1,
  },
},

homeGradient: {
  colors: [
    '#7C3AED',
    '#8B5CF6',
    '#A78BFA',
    '#C4B5FD',
    '#DDD6FE',
    '#F3EEFF',
    '#FAFAFA',
  ] as const,

  blobPrimary: '#6D28D9',
  blobSecondary: '#8B5CF6',
  blobTertiary: '#A78BFA',

  start: {
    x: 0.5,
    y: 0,
  },

  end: {
    x: 0.5,
    y: 1,
  },
},
    // ─────────────────────────────────────────────────────────────────────
    // Authentication Surface
    // ─────────────────────────────────────────────────────────────────────

    auth: {
      surfaceText: '#FFFFFF',
      surfaceTextSecondary:
        'rgba(255,255,255,0.68)',

      surfaceBorder:
        'rgba(255,255,255,0.32)',

      surfaceInput:
        'rgba(255,255,255,0.08)',

      primaryButton: '#FFFFFF',
      primaryButtonText: '#18181B',

      secondaryButton:
        'rgba(255,255,255,0.08)',

      secondaryButtonText:
        '#FFFFFF',

      errorBackground:
        'rgba(239,68,68,0.18)',

      errorBorder:
        'rgba(255,255,255,0.28)',

      errorText: '#FFFFFF',
    },
  },


  // ───────────────────────────────────────────────────────────────────────
  // DARK
  // ───────────────────────────────────────────────────────────────────────

  dark: {
    primary: '#9061F9',
    primaryForeground: '#FFFFFF',
    primaryLight: '#2E1065',
    primaryDark: '#7C3AED',

    brandPurple: '#9061F9',
    brandPurpleLight: '#A78BFA',
    brandPurpleDark: '#6D28D9',

    brandLime: '#A3E635',
    brandLimeDark: '#4D7C0F',

    brandPink: '#F472B6',
    brandPinkLight: '#FB7185',

    brandIndigo: '#818CF8',

    brandRose: '#FB7185',


    // ─────────────────────────────────────────────────────────────────────
    // Base
    // ─────────────────────────────────────────────────────────────────────

    background: '#18181B',
    foreground: '#F4F4F5',

    backgroundElement: '#27272A',
    backgroundSelected: '#3F3F46',


    // ─────────────────────────────────────────────────────────────────────
    // Cards
    // ─────────────────────────────────────────────────────────────────────

    card: '#27272A',
    cardForeground: '#F4F4F5',

    cardElevated: '#27272A',
    cardBorder: '#3F3F46',


    // ─────────────────────────────────────────────────────────────────────
    // Popovers
    // ─────────────────────────────────────────────────────────────────────

    popover: '#27272A',
    popoverForeground: '#F4F4F5',


    // ─────────────────────────────────────────────────────────────────────
    // Semantic surfaces
    // ─────────────────────────────────────────────────────────────────────

    secondary: '#27272A',
    secondaryForeground: '#F4F4F5',

    muted: '#27272A',
    mutedForeground: '#A1A1AA',

    accent: '#3F3F46',
    accentForeground: '#F4F4F5',
    accentLight: '#2E1065',


    // ─────────────────────────────────────────────────────────────────────
    // Text
    // ─────────────────────────────────────────────────────────────────────

    text: '#F4F4F5',
    textSecondary: '#A1A1AA',
    textMuted: '#71717A',


    // ─────────────────────────────────────────────────────────────────────
    // Borders / Inputs
    // ─────────────────────────────────────────────────────────────────────

    border: '#3F3F46',
    borderSubtle: '#27272A',

    input: '#27272A',

    ring: '#9061F9',


    // ─────────────────────────────────────────────────────────────────────
    // Destructive
    // ─────────────────────────────────────────────────────────────────────

    destructive: '#EF4444',
    destructiveForeground: '#FFFFFF',


    // ─────────────────────────────────────────────────────────────────────
    // Success
    // ─────────────────────────────────────────────────────────────────────

    success: '#34D399',
    successLight: '#064E3B',


    // ─────────────────────────────────────────────────────────────────────
    // Warning
    // ─────────────────────────────────────────────────────────────────────

    warning: '#FBBF24',
    warningLight: '#78350F',


    // ─────────────────────────────────────────────────────────────────────
    // Error
    // ─────────────────────────────────────────────────────────────────────

    error: '#EF4444',
    errorLight: '#7F1D1D',


    // ─────────────────────────────────────────────────────────────────────
    // Info
    // ─────────────────────────────────────────────────────────────────────

    info: '#22D3EE',
    infoLight: '#164E63',


    // ─────────────────────────────────────────────────────────────────────
    // Button shadows
    // ─────────────────────────────────────────────────────────────────────

    btnShadowPrimary: '#581C87',
    btnShadowSecondary: '#18181B',
    btnShadowDestructive: '#7F1D1D',
    btnShadowLime: '#365314',


    // ─────────────────────────────────────────────────────────────────────
    // Tab Bar
    // ─────────────────────────────────────────────────────────────────────

    tabBarBackground: '#18181BF2',
    tabBarBorder: '#27272A',
    tabBarActive: '#9061F9',
    tabBarInactive: '#71717A',


    // ─────────────────────────────────────────────────────────────────────
    // Gradient Surface
    //
    // Direction:
    //
    //   18181B
    //      ↓
    //   18181B
    //      ↓
    //   18181B
    //      ↓
    //   211A2E
    //      ↓
    //   2E1B45
    //      ↓
    //   4C1D95
    //      ↓
    //   6D28D9
    //      ↓
    //   7C3AED
    //
    // The upper area remains dark and clean.
    // Purple gradually emerges toward the bottom.
    // ─────────────────────────────────────────────────────────────────────

gradientHeader: {
  colors: [
    '#18181B',
    '#18181B',
    '#18181B',
    '#211A2E',
    '#2E1B45',
    '#4C1D95',
    '#6D28D9',
    '#7C3AED',
  ] as const,

  blobPrimary: '#8B5CF6',

  blobSecondary: '#A78BFA',

  blobTertiary: '#6366F1',

  start: {
    x: 0.5,
    y: 0,
  },

  end: {
    x: 0.5,
    y: 1,
  },
},
homeGradient: {
  colors: [
    '#7C3AED',
    '#6D28D9',
    '#5B21B6',
    '#4C1D95',
    '#32134F',
    '#21162B',
    '#18181B',
  ] as const,

  blobPrimary: '#A78BFA',
  blobSecondary: '#8B5CF6',
  blobTertiary: '#6366F1',

  start: {
    x: 0.5,
    y: 0,
  },

  end: {
    x: 0.5,
    y: 1,
  },
},


    // ─────────────────────────────────────────────────────────────────────
    // Authentication Surface
    // ─────────────────────────────────────────────────────────────────────

    auth: {
      surfaceText: '#FFFFFF',

      surfaceTextSecondary:
        'rgba(255,255,255,0.68)',

      surfaceBorder:
        'rgba(255,255,255,0.24)',

      surfaceInput:
        'rgba(255,255,255,0.06)',

      primaryButton: '#FFFFFF',
      primaryButtonText: '#18181B',

      secondaryButton:
        'rgba(255,255,255,0.08)',

      secondaryButtonText:
        '#FFFFFF',

      errorBackground:
        'rgba(239,68,68,0.20)',

      errorBorder:
        'rgba(255,255,255,0.20)',

      errorText: '#FFFFFF',
    },
  },
} as const;


// ─────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────

export type ThemeColors =
  typeof Colors.light;

export type ThemeColor =
  keyof ThemeColors;


// ─────────────────────────────────────────────────────────────────────────
// Spacing
// ─────────────────────────────────────────────────────────────────────────

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


// ─────────────────────────────────────────────────────────────────────────
// Radii
// ─────────────────────────────────────────────────────────────────────────

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


// ─────────────────────────────────────────────────────────────────────────
// Typography
// ─────────────────────────────────────────────────────────────────────────

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


// ─────────────────────────────────────────────────────────────────────────
// Fonts
// ─────────────────────────────────────────────────────────────────────────

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
    sans:
      'Rubik, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',

    mono:
      'JetBrains Mono, monospace',
  },
});