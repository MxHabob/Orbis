/**
 * @module @orbis/mobile/components/ui/loading-indicator
 *
 * Google-inspired radial loading indicator for mobile surfaces.
 */

import React, { useEffect } from "react";
import { View, type ViewStyle } from "react-native";
import Animated, {
  cancelAnimation,
  Easing,
  useAnimatedStyle,
  useReducedMotion,
  useSharedValue,
  withRepeat,
  withSequence,
  withTiming,
} from "react-native-reanimated";
import Svg, { Polygon } from "react-native-svg";
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
  md: 32,
  lg: 64,
};

const STAR_POINTS =
  "50,24.6 61.4,7.3 62.7,28 81.2,18.8 72,37.3 92.7,38.6 75.4,50 92.7,61.4 72,62.7 81.2,81.2 62.7,72 61.4,92.7 50,75.4 38.6,92.7 37.3,72 18.8,81.2 28,62.7 7.3,61.4 24.6,50 7.3,38.6 28,37.3 18.8,18.8 37.3,28 38.6,7.3";

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
  const shapeScale = useSharedValue(0.72);
  const dimension = typeof size === "number" ? size : SIZE_MAP[size];
  const shapeColor = color ?? theme.primaryForeground;
  const backgroundColor = containerColor ?? theme.primary;

  useEffect(() => {
    cancelAnimation(rotation);
    cancelAnimation(shapeScale);

    if (reduceMotion) {
      rotation.value = 0;
      shapeScale.value = 1;
      return;
    }

    rotation.value = withRepeat(
      withTiming(360, { duration: 1350, easing: Easing.linear }),
      -1,
      false,
    );
    shapeScale.value = withRepeat(
      withSequence(
        withTiming(1.12, {
          duration: 900,
          easing: Easing.inOut(Easing.ease),
        }),
        withTiming(0.72, {
          duration: 900,
          easing: Easing.inOut(Easing.ease),
        }),
      ),
      -1,
      false,
    );

    return () => {
      cancelAnimation(rotation);
      cancelAnimation(shapeScale);
    };
  }, [reduceMotion, rotation, shapeScale]);

  const shapeStyle = useAnimatedStyle(() => ({
    opacity: 0.78 + ((shapeScale.value - 0.72) / 0.4) * 0.22,
    transform: [
      { rotate: `${rotation.value}deg` },
      { scale: shapeScale.value },
    ],
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
        style={[
          {
            width: "54%",
            height: "54%",
          },
          shapeStyle,
        ]}
      >
        <Svg width="100%" height="100%" viewBox="0 0 100 100">
          <Polygon points={STAR_POINTS} fill={shapeColor} />
        </Svg>
      </Animated.View>
    </View>
  );
});

LoadingIndicator.displayName = "LoadingIndicator";
