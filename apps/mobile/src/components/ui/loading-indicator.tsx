/**
 * @module @orbis/mobile/components/ui/loading-indicator
 *
 * Material 3 Expressive indeterminate loading indicator synchronized with
 * `@orbis/ui`'s desktop LoadingIndicator (7-shape silhouette sequence + dual rotation).
 * Uses pure `useAnimatedStyle` transforms and opacity for 100% reliability across
 * iOS, Android Fabric, and Web without relying on native SVG path prop bridging.
 */

import React, { useEffect } from "react";
import { StyleSheet, View, type ViewStyle } from "react-native";
import Animated, {
  cancelAnimation,
  Easing,
  type SharedValue,
  useAnimatedStyle,
  useReducedMotion,
  useSharedValue,
  withRepeat,
  withSequence,
  withTiming,
} from "react-native-reanimated";
import Svg, { Path } from "react-native-svg";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

export type LoadingIndicatorSize = "sm" | "md" | "lg";

export interface LoadingIndicatorProps {
  size?: LoadingIndicatorSize | number;
  label?: string;
  color?: string;
  containerColor?: string;
  style?: ViewStyle;
}

const SIZE_MAP: Record<LoadingIndicatorSize, number> = {
  sm: 16,
  md: 48,
  lg: 64,
};

const POINT_COUNT = 72;

function silhouette(radius: (angle: number) => number): string {
  const points: [number, number][] = [];
  for (let i = 0; i < POINT_COUNT; i++) {
    const angle = (i * Math.PI) / 36;
    const distance = radius(angle);
    points.push([
      50 + distance * Math.cos(angle),
      50 + distance * Math.sin(angle),
    ]);
  }
  const segments = points.map(([x, y], index) => {
    const [nextX, nextY] = points[(index + 1) % points.length]!;
    return `Q ${x.toFixed(2)} ${y.toFixed(2)} ${((x + nextX) / 2).toFixed(2)} ${((y + nextY) / 2).toFixed(2)}`;
  });
  const [lastX, lastY] = points[points.length - 1]!;
  const [firstX, firstY] = points[0]!;
  return `M ${((lastX + firstX) / 2).toFixed(2)} ${((lastY + firstY) / 2).toFixed(2)} ${segments.join(" ")} Z`;
}

function oval(angle: number, width: number, height: number): number {
  return 1 / Math.hypot(Math.cos(angle) / width, Math.sin(angle) / height);
}

// Material 3 Expressive 7-shape sequence matching @orbis/ui LoadingIndicator
const SHAPE_PATHS: readonly string[] = [
  silhouette((angle) => 32 + 4.5 * Math.cos(angle * 10)), // softBurst
  silhouette((angle) => 34 + 3 * Math.sin(angle * 9)), // cookie9
  silhouette((angle) => {
    const sector = (Math.PI * 2) / 5;
    const offset = ((angle + Math.PI / 2) % sector) - sector / 2;
    return 29 / Math.cos(offset * 0.94);
  }), // pentagon
  silhouette((angle) => {
    const x = Math.abs(Math.cos(angle));
    const y = Math.abs(Math.sin(angle));
    return 18 * x < 20 * y
      ? 18 / y
      : 20 * x + Math.sqrt(Math.max(0, 18 ** 2 - (20 * y) ** 2));
  }), // pill
  silhouette((angle) => 34 + 3.5 * Math.cos(angle * 8)), // sunny
  silhouette((angle) => 31 - 6 * Math.cos(angle * 4)), // cookie4
  silhouette((angle) => oval(angle - Math.PI / 4, 38, 25)), // ovalPath
];

function ExpressiveShapeLayer({
  d,
  index,
  total,
  morph,
  color,
  reduceMotion,
}: {
  d: string;
  index: number;
  total: number;
  morph: SharedValue<number>;
  color: string;
  reduceMotion: boolean;
}) {
  const layerStyle = useAnimatedStyle(() => {
    if (reduceMotion) {
      return {
        opacity: index === 0 ? 1 : 0,
        transform: [{ scale: 1 }],
      };
    }

    const wrapped = ((morph.value % total) + total) % total;
    let dist = Math.abs(wrapped - index);
    if (dist > total / 2) dist = total - dist;

    // Crossfade smoothly between consecutive shapes
    const opacity = Math.max(0, 1 - dist);
    const scale = 0.88 + 0.12 * opacity;
    return {
      opacity,
      transform: [{ scale }],
    };
  });

  return (
    <Animated.View
      accessible={false}
      style={[StyleSheet.absoluteFill, layerStyle]}
    >
      <Svg width="100%" height="100%" viewBox="0 0 100 100">
        <Path d={d} fill={color} />
      </Svg>
    </Animated.View>
  );
}

export const LoadingIndicator = React.memo(function LoadingIndicator({
  size = "md",
  label,
  color,
  containerColor,
  style,
}: LoadingIndicatorProps) {
  const theme = useTheme();
  const { t } = useI18n();
  const reduceMotion = useReducedMotion();
  const rotation = useSharedValue(0);
  const turn = useSharedValue(90);
  const morph = useSharedValue(0);

  const dimension = typeof size === "number" ? size : SIZE_MAP[size];
  const shapeColor = color ?? theme.primaryForeground;
  const backgroundColor = containerColor ?? theme.primary;

  useEffect(() => {
    cancelAnimation(rotation);
    cancelAnimation(turn);
    cancelAnimation(morph);

    if (reduceMotion) {
      rotation.value = 0;
      turn.value = 0;
      morph.value = 0;
      return;
    }

    rotation.value = withRepeat(
      withTiming(360, { duration: 4666, easing: Easing.linear }),
      -1,
      false,
    );

    const turnStepDuration = 650;
    const turnEase = Easing.out(Easing.back(1.2));
    turn.value = withRepeat(
      withSequence(
        withTiming(180, { duration: turnStepDuration, easing: turnEase }),
        withTiming(270, { duration: turnStepDuration, easing: turnEase }),
        withTiming(360, { duration: turnStepDuration, easing: turnEase }),
        withTiming(450, { duration: turnStepDuration, easing: turnEase }),
      ),
      -1,
      false,
    );

    morph.value = withRepeat(
      withTiming(SHAPE_PATHS.length, { duration: 4550, easing: Easing.linear }),
      -1,
      false,
    );

    return () => {
      cancelAnimation(rotation);
      cancelAnimation(turn);
      cancelAnimation(morph);
    };
  }, [morph, reduceMotion, rotation, turn]);

  const shapeStyle = useAnimatedStyle(() => ({
    transform: [{ rotate: `${rotation.value + turn.value}deg` }],
  }));

  return (
    <View
      accessible
      accessibilityRole="progressbar"
      accessibilityLabel={label ?? t("common_loading")}
      accessibilityLiveRegion="polite"
      accessibilityState={{ busy: true }}
      className="items-center justify-center overflow-hidden rounded-full"
      style={[
        {
          width: dimension,
          height: dimension,
          backgroundColor,
        },
        style,
      ]}
    >
      <Animated.View
        accessible={false}
        style={[{ width: "100%", height: "100%" }, shapeStyle]}
      >
        {SHAPE_PATHS.map((d, index) => (
          <ExpressiveShapeLayer
            key={index}
            d={d}
            index={index}
            total={SHAPE_PATHS.length}
            morph={morph}
            color={shapeColor}
            reduceMotion={Boolean(reduceMotion)}
          />
        ))}
      </Animated.View>
    </View>
  );
});

LoadingIndicator.displayName = "LoadingIndicator";
