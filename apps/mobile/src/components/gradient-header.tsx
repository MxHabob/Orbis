import React, { useEffect } from 'react';
import {
  StyleSheet,
  type ColorValue,
  type StyleProp,
  type ViewStyle,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
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

type GradientHeaderColors = readonly [
  ColorValue,
  ColorValue,
  ...ColorValue[],
];

type GradientHeaderLocations = readonly [
  number,
  number,
  ...number[],
];

type GradientHeaderProps = {
  colors: GradientHeaderColors;
  locations?: GradientHeaderLocations;
  height?: number;
  start?: { x: number; y: number };
  end?: { x: number; y: number };
  style?: StyleProp<ViewStyle>;

  animated?: boolean;
  duration?: number;
};

const AnimatedView = Animated.View;

function FluidBlob({
  size,
  opacity,
  duration,
  delay = 0,
  color = '#7628F5',
  initialX = 0,
  initialY = 0,
}: {
  size: number;
  opacity: number;
  duration: number;
  delay?: number;
  color?: string;
  initialX?: number;
  initialY?: number;
}) {
  const progress = useSharedValue(0);

  useEffect(() => {
    const timeout = setTimeout(() => {
      progress.value = withRepeat(
        withTiming(1, {
          duration,
          easing: Easing.inOut(Easing.ease),
        }),
        -1,
        true,
      );
    }, delay);

    return () => {
      clearTimeout(timeout);
    };
  }, [delay, duration, progress]);

  const animatedStyle = useAnimatedStyle(() => {
    const p = progress.value;

    return {
      transform: [
        {
          translateX:
            initialX +
            Math.sin(p * Math.PI * 2) * 28,
        },
        {
          translateY:
            initialY +
            Math.cos(p * Math.PI * 2) * 18,
        },
        {
          scale:
            1 +
            Math.sin(p * Math.PI * 2) * 0.08,
        },
        {
          rotate: `${Math.sin(p * Math.PI * 2) * 5}deg`,
        },
      ],
    };
  });

  return (
    <AnimatedView
      pointerEvents="none"
      style={[
        styles.blob,
        {
          width: size,
          height: size * 0.72,
          opacity,
          borderRadius: size,
        },
        animatedStyle,
      ]}
    >
      <Svg
        width="100%"
        height="100%"
        viewBox="0 0 100 72"
      >
        <Defs>
          <RadialGradient
            id={`blob-${size}-${delay}`}
            cx="50%"
            cy="50%"
            rx="50%"
            ry="50%"
          >
            <Stop
              offset="0%"
              stopColor={color}
              stopOpacity="0.9"
            />
            <Stop
              offset="45%"
              stopColor={color}
              stopOpacity="0.45"
            />
            <Stop
              offset="75%"
              stopColor={color}
              stopOpacity="0.12"
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
          fill={`url(#blob-${size}-${delay})`}
        />
      </Svg>
    </AnimatedView>
  );
}

export function GradientHeader({
  colors,
  locations,
  height = 180,
  start = { x: 0.5, y: 0 },
  end = { x: 0.5, y: 1 },
  style,
  animated = false,
  duration = 16000,
}: GradientHeaderProps) {
  return (
    <AnimatedView
      style={[
        styles.container,
        { height },
        style,
      ]}
    >
      {/* Base gradient */}
      <LinearGradient
        colors={colors}
        locations={locations}
        start={start}
        end={end}
        style={StyleSheet.absoluteFill}
      />

      {animated && (
        <AnimatedView
          pointerEvents="none"
          style={StyleSheet.absoluteFill}
        >
          {/* Large fluid mass */}
          <FluidBlob
            size={360}
            opacity={0.52}
            duration={duration}
            initialX={-90}
            initialY={-45}
          />

          {/* Secondary fluid mass */}
          <FluidBlob
            size={300}
            opacity={0.38}
            duration={duration + 2500}
            delay={900}
            initialX={110}
            initialY={10}
            color="#893BF2"
          />

          {/* Small softer mass */}
          <FluidBlob
            size={250}
            opacity={0.28}
            duration={duration + 4200}
            delay={1800}
            initialX={20}
            initialY={65}
            color="#A75AF0"
          />
        </AnimatedView>
      )}
    </AnimatedView>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '100%',
    overflow: 'hidden',
  },

  blob: {
    position: 'absolute',
  },
});