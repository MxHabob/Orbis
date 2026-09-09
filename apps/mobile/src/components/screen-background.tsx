import React, {
  type ReactNode,
} from 'react';

import {
  StyleSheet,
  View,
  useWindowDimensions,
} from 'react-native';

import {
  GradientHeader,
} from '@/components/gradient-header';

import {
  useTheme,
} from '@/hooks/use-theme';


interface ScreenBackgroundProps {
  children: ReactNode;

  /**
   * Percentage of the screen occupied by the gradient.
   *
   * Default: 52%
   */
  gradientHeight?: number;
}


export function ScreenBackground({
  children,
  gradientHeight = 0.52,
}: ScreenBackgroundProps) {

  const {
    height,
  } = useWindowDimensions();

  const theme =
    useTheme();


  return (
    <View
      style={[
        styles.container,

        {
          backgroundColor:
            theme.background,
        },
      ]}>

      {/* ─────────────────────────────────────────────────────────────
          FULL SCREEN BACKGROUND

          This is attached directly to the screen root.

          Therefore:

          left  = screen left
          right = screen right
          width = screen width

          It is NOT affected by Header padding/margins.
        ───────────────────────────────────────────────────────────── */}

      <GradientHeader
        height={
          height *
          gradientHeight
        }

        colors={
          theme.homeGradient.colors
        }

        locations={[
          0,
          0.14,
          0.30,
          0.46,
          0.62,
          0.78,
          1,
        ]}

        start={{
          x: 0.5,
          y: 0,
        }}

        end={{
          x: 0.5,
          y: 1,
        }}

        blobPrimary={
          theme.homeGradient.blobPrimary
        }

        blobSecondary={
          theme.homeGradient.blobSecondary
        }

        blobTertiary={
          theme.homeGradient.blobTertiary
        }

        animated

        duration={
          18000
        }

        style={
          styles.gradient
        }
      />


      {/* ─────────────────────────────────────────────────────────────
          Screen Content
        ───────────────────────────────────────────────────────────── */}

      <View
        style={
          styles.content
        }>

        {children}

      </View>

    </View>
  );
}


const styles = StyleSheet.create({

  container: {
    flex: 1,

    width:
      '100%',

    position:
      'relative',
  },


  gradient: {
    position:
      'absolute',

    top:
      0,

    left:
      0,

    right:
      0,

    width:
      '100%',

    zIndex:
      0,
  },


  content: {
    flex: 1,

    width:
      '100%',

    zIndex:
      1,
  },
});