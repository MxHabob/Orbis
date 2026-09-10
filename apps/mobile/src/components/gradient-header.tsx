/**
 * @module @orbis/mobile/components/gradient-header
 *
 * Reusable animated gradient background.
 *
 * - Does not consume layout space.
 * - Can be anchored to any edge by the parent.
 * - Supports soft animated fluid blobs.
 * - Does not control StatusBar.
 */

import React, {
  useEffect,
} from 'react';

import {
  StyleSheet,
  type ColorValue,
  type StyleProp,
  type ViewStyle,
} from 'react-native';

import {
  LinearGradient,
} from 'expo-linear-gradient';

import Svg, {
  Defs,
  Ellipse,
  RadialGradient,
  Stop,
} from 'react-native-svg';

import Animated, {
  Easing,
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withTiming,
} from 'react-native-reanimated';


type GradientHeaderColors =
  readonly [
    ColorValue,
    ColorValue,
    ...ColorValue[],
  ];


type GradientHeaderLocations =
  readonly [
    number,
    number,
    ...number[],
  ];


type GradientHeaderProps = {
  colors: GradientHeaderColors;

  locations?: GradientHeaderLocations;

  height?: number;

  start?: {
    x: number;
    y: number;
  };

  end?: {
    x: number;
    y: number;
  };

  style?: StyleProp<ViewStyle>;

  animated?: boolean;

  duration?: number;

  anchor?: 'top' | 'bottom';

  blobPrimary?: string;

  blobSecondary?: string;

  blobTertiary?: string;
};


const AnimatedView =
  Animated.View;


function FluidBlob({
  size,
  opacity,
  duration,
  delay = 0,
  color,
  initialX = 0,
  initialY = 0,
}: {
  size: number;

  opacity: number;

  duration: number;

  delay?: number;

  color: string;

  initialX?: number;

  initialY?: number;
}) {
  const progress =
    useSharedValue(0);


  useEffect(() => {
    const timeout =
      setTimeout(() => {
        progress.value =
          withRepeat(
            withTiming(
              1,
              {
                duration,
                easing:
                  Easing.inOut(
                    Easing.ease,
                  ),
              },
            ),
            -1,
            true,
          );
      }, delay);


    return () => {
      clearTimeout(timeout);
    };
  }, [
    delay,
    duration,
    progress,
  ]);


  const animatedStyle =
    useAnimatedStyle(() => {
      const p =
        progress.value;


      const waveX =
        Math.sin(
          p * Math.PI * 2,
        );


      const waveY =
        Math.cos(
          p * Math.PI * 2,
        );


      return {
        transform: [
          {
            translateX:
              initialX +
              waveX * 35,
          },

          {
            translateY:
              initialY +
              waveY * 24,
          },

          {
            scale:
              1 +
              waveX * 0.06,
          },

          {
            rotate:
              `${waveX * 4}deg`,
          },
        ],
      };
    });


  const gradientId =
    `fluid-${size}-${delay}`;


  return (
    <AnimatedView
      pointerEvents="none"
      style={[
        styles.blob,

        {
          width: size,
          height:
            size * 0.72,
          opacity,
        },

        animatedStyle,
      ]}>

      <Svg
        width="100%"
        height="100%"
        viewBox="0 0 100 72">

        <Defs>

          <RadialGradient
            id={gradientId}
            cx="50%"
            cy="50%"
            rx="50%"
            ry="50%">

            <Stop
              offset="0%"
              stopColor={color}
              stopOpacity="0.75"
            />

            <Stop
              offset="45%"
              stopColor={color}
              stopOpacity="0.35"
            />

            <Stop
              offset="75%"
              stopColor={color}
              stopOpacity="0.10"
            />

            <Stop
              offset="100%"
              stopColor={color}
              stopOpacity="0"
            />

          </RadialGradient>

        </Defs>


        <Ellipse
          cx="50"
          cy="36"
          rx="50"
          ry="36"
          fill={`url(#${gradientId})`}
        />

      </Svg>

    </AnimatedView>
  );
}


export function GradientHeader({
  colors,

  locations,

  height = 500,

  start = {
    x: 0.5,
    y: 0,
  },

  end = {
    x: 0.5,
    y: 1,
  },

  style,

  animated = false,

  duration = 18000,

  anchor = 'top',

  blobPrimary = '#7C3AED',

  blobSecondary = '#A855F7',

  blobTertiary = '#C084FC',
}: GradientHeaderProps) {
  const resolvedLocations =
    locations && locations.length === colors.length
      ? locations
      : Array.from({ length: colors.length }, (_, index) =>
          colors.length === 1 ? 0 : index / (colors.length - 1),
        );

  return (
    <AnimatedView
      pointerEvents="none"
      style={[
        styles.container,

        {
          height,
          [anchor === 'top' ? 'top' : 'bottom']: 0,
        },

        style,
      ]}>

      <LinearGradient
        colors={colors}
        locations={resolvedLocations}
        start={start}
        end={end}
        style={
          StyleSheet.absoluteFill
        }
      />


      {animated && (
        <AnimatedView
          pointerEvents="none"
          style={
            StyleSheet.absoluteFill
          }>

          <FluidBlob
            size={430}
            opacity={0.20}
            duration={duration}
            color={blobPrimary}
            initialX={-80}
            initialY={40}
          />


          <FluidBlob
            size={360}
            opacity={0.14}
            duration={
              duration + 2600
            }
            delay={900}
            color={blobSecondary}
            initialX={100}
            initialY={150}
          />


          <FluidBlob
            size={300}
            opacity={0.10}
            duration={
              duration + 4200
            }
            delay={1800}
            color={blobTertiary}
            initialX={0}
            initialY={250}
          />

        </AnimatedView>
      )}

    </AnimatedView>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'absolute',

    left: 0,
    right: 0,

    width: '100%',

    overflow: 'hidden',

    zIndex: 0,
  },

  blob: {
    position: 'absolute',
  },
});